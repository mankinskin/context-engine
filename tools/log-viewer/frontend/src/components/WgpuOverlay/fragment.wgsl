// fragment.wgsl — aurora bg + static shadows + hover sparks + CRT
//
// Architecture:
//   Canvas is BEHIND HTML (z-index -1, opaque).  HTML backgrounds are
//   transparent so the aurora shows through.
//
// Bindings (render pass):
//   binding 0 (uniform)          : Uniforms
//   binding 1 (read-only-storage): ElemRect[]   — DOM element rects
//   binding 2 (read-only-storage): Particle[]   — spark particles (written by compute)

struct Uniforms {
    time          : f32,
    width         : f32,
    height        : f32,
    element_count : f32,
    mouse_x       : f32,
    mouse_y       : f32,
    delta_time    : f32,
    hover_elem    : f32,
}

struct ElemRect {
    rect : vec4f,
    hue  : f32,
    kind : f32,
    _p1  : f32,
    _p2  : f32,
}

struct Particle {
    pos      : vec2f,
    vel      : vec2f,
    life     : f32,
    max_life : f32,
    hue      : f32,
    size     : f32,
}

@group(0) @binding(0) var<uniform>       u         : Uniforms;
@group(0) @binding(1) var<storage, read> elems     : array<ElemRect>;
@group(0) @binding(2) var<storage, read> particles : array<Particle>;

// ---- colour helpers -------------------------------------------------------

fn hue_to_rgb(h: f32) -> vec3f {
    let h6 = fract(h) * 6.0;
    let r  = abs(h6 - 3.0) - 1.0;
    let g  = 2.0 - abs(h6 - 2.0);
    let b  = 2.0 - abs(h6 - 4.0);
    return clamp(vec3f(r, g, b), vec3f(0.0), vec3f(1.0));
}

// ---- noise (aurora) -------------------------------------------------------

fn hash2(p: vec2f) -> f32 {
    return fract(sin(dot(p, vec2f(127.1, 311.7))) * 43758.5453);
}

fn smooth_noise(p: vec2f) -> f32 {
    let i  = floor(p);
    let f  = fract(p);
    let uv = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash2(i),                   hash2(i + vec2f(1.0, 0.0)), uv.x),
        mix(hash2(i + vec2f(0.0, 1.0)), hash2(i + vec2f(1.0, 1.0)), uv.x),
        uv.y
    );
}

fn fbm(p_in: vec2f) -> f32 {
    var val  = 0.0;
    var amp  = 0.5;
    var freq = 1.0;
    var p    = p_in;
    for (var i = 0; i < 3; i++) {
        val  += amp * smooth_noise(p * freq);
        amp  *= 0.5;
        freq *= 2.0;
    }
    return val;
}

// ---- edge helpers ---------------------------------------------------------

fn edge_dist(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32) -> f32 {
    let dx = min(px.x - ex, ex + ew - px.x);
    let dy = min(px.y - ey, ey + eh - px.y);
    return min(dx, dy);
}

fn perimeter_t(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32) -> f32 {
    let perim = 2.0 * (ew + eh);
    let lx = px.x - ex;
    let ly = px.y - ey;
    if ly < lx && ly < (eh - ly) && ly < (ew - lx) { return lx / perim; }
    if (ew - lx) < ly && (ew - lx) < (eh - ly) { return (ew + ly) / perim; }
    if (eh - ly) < lx && (eh - ly) < (ew - lx) { return (ew + eh + (ew - lx)) / perim; }
    return (2.0 * ew + eh + (eh - ly)) / perim;
}

// ---- rounded-rect SDF ----------------------------------------------------

fn rounded_rect_sdf(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32, radius: f32) -> f32 {
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let half   = vec2f(ew * 0.5, eh * 0.5);
    let rel    = abs(px - center) - half + vec2f(radius);
    return length(max(rel, vec2f(0.0))) + min(max(rel.x, rel.y), 0.0) - radius;
}

// ---- hover proximity ------------------------------------------------------

fn hover_proximity(ex: f32, ey: f32, ew: f32, eh: f32) -> f32 {
    let mouse  = vec2f(u.mouse_x, u.mouse_y);
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let dist   = length(mouse - center);
    return smoothstep(max(ew, eh) * 0.8, 0.0, dist);
}

// ---- graph node (kind 8) -------------------------------------------------

fn graph_node(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32,
              hue: f32, node_type: f32, t: f32, prox: f32) -> vec4f {
    let radius = 10.0;
    let sdf = rounded_rect_sdf(px, ex, ey, ew, eh, radius);
    if sdf > 3.0 { return vec4f(0.0); }

    let body_mask = smoothstep(1.0, -0.5, sdf);
    let nx = (px.x - ex) / ew;
    let ny = (px.y - ey) / eh;

    let base_rgb   = hue_to_rgb(hue);
    let top_rgb    = base_rgb * 0.9 + vec3f(0.25);
    let bottom_rgb = base_rgb * 0.45;
    var fill_rgb   = mix(top_rgb, bottom_rgb, ny);

    // Mouse lighting
    let mouse  = vec2f(u.mouse_x, u.mouse_y);
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let to_mouse = normalize(mouse - center + vec2f(0.001));
    let normal   = vec2f((nx - 0.5) * 0.3, (ny - 0.5) * 0.5);
    let diffuse  = max(0.0, dot(normalize(normal + vec2f(0.0, -0.3)), to_mouse));
    let spec_pos = vec2f(nx - 0.5, ny - 0.3);
    let spec     = pow(max(0.0, 1.0 - length(spec_pos - to_mouse * 0.2) * 2.0), 8.0);
    let lb       = prox * 0.6;
    fill_rgb = fill_rgb + fill_rgb * diffuse * (0.15 + lb);
    fill_rgb = fill_rgb + vec3f(1.0) * spec * (0.08 + lb * 0.3);

    // Top gleam + bottom shadow
    fill_rgb = fill_rgb + vec3f(1.0) * smoothstep(0.15, 0.0, ny) * 0.3;
    fill_rgb = fill_rgb * (1.0 - smoothstep(0.85, 1.0, ny) * 0.15);

    // Node type accent bars
    let ntype = u32(node_type);
    if ntype == 1u {
        let bar = smoothstep(3.0, 0.0, px.x - ex) * smoothstep(-1.0, 0.0, sdf);
        fill_rgb = mix(fill_rgb, vec3f(0.2, 0.9, 0.3), bar * 0.8);
    } else if ntype == 2u {
        let bar = smoothstep(3.0, 0.0, (ex + ew) - px.x) * smoothstep(-1.0, 0.0, sdf);
        fill_rgb = mix(fill_rgb, vec3f(0.9, 0.2, 0.2), bar * 0.8);
    }

    // Border
    let border_glow = smoothstep(-2.0, 0.0, sdf) * smoothstep(2.0, 0.5, sdf);
    let border_rgb  = hue_to_rgb(fract(hue + t * 0.05)) * 1.2;
    fill_rgb = fill_rgb + border_rgb * border_glow * 0.5 * (1.0 + prox * 0.8);

    // Outer glow + shadow
    let outer       = smoothstep(3.0, 0.0, sdf) * (1.0 - body_mask);
    let outer_alpha = outer * 0.3 * (1.0 + prox * 0.6);
    let shadow_sdf  = rounded_rect_sdf(px - vec2f(-2.0, 3.0 + prox * 4.0), ex, ey, ew, eh, radius);
    let shadow_mask = smoothstep(0.0, 8.0, -shadow_sdf) * (1.0 - body_mask);
    let shadow_alpha = shadow_mask * 0.2 * (0.5 + prox * 0.5);

    let body_alpha = body_mask * 0.75;
    let total_rgb  = fill_rgb * body_alpha + base_rgb * 0.8 * outer_alpha;
    let total_a    = body_alpha + outer_alpha + shadow_alpha;

    return vec4f(total_rgb, total_a);
}

// ---- CRT post-processing -------------------------------------------------

fn crt_scanlines(py: f32) -> f32 {
    return 0.85 + 0.15 * sin(py * 3.14159);
}

fn crt_vertical_lines(px_x: f32) -> f32 {
    return 0.90 + 0.10 * sin(px_x * 3.14159 * 0.6667);
}

fn crt_edge_shadow(uv: vec2f) -> f32 {
    let d_left   = uv.x;
    let d_right  = 1.0 - uv.x;
    let d_top    = uv.y;
    let d_bottom = 1.0 - uv.y;
    let d = min(min(d_left, d_right), min(d_top, d_bottom));
    return smoothstep(0.0, 0.008, d);
}

// rgb_fringe removed — was causing 3× sample_scene evaluation

// Particle rendering moved to instanced quads (vs_particle / fs_particle)
// — eliminates O(pixels × particles) per-pixel loop

// ---- static thin shadow for non-hovered elements --------------------------

fn static_shadow(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32) -> f32 {
    let inside_x = px.x >= ex && px.x < ex + ew;
    let inside_y = px.y >= ey && px.y < ey + eh;
    if !(inside_x && inside_y) { return 0.0; }

    let dist = edge_dist(px, ex, ey, ew, eh);
    return smoothstep(2.0, 0.0, dist) * 0.12;
}

// ---- animated hover border ------------------------------------------------

fn hover_border(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32,
                hue: f32, t: f32, prox: f32) -> vec4f {
    let inside_x = px.x >= ex - 3.0 && px.x < ex + ew + 3.0;
    let inside_y = px.y >= ey - 3.0 && px.y < ey + eh + 3.0;
    if !(inside_x && inside_y) { return vec4f(0.0); }

    let dist = edge_dist(px, ex, ey, ew, eh);
    let pt   = perimeter_t(px, ex, ey, ew, eh);

    if dist > 6.0 { return vec4f(0.0); }

    // Animated colour wave
    let wave_phase = pt * 12.56 - t * 4.0;
    let wave = 0.5 + 0.5 * sin(wave_phase);

    // Sparkle bursts along the perimeter
    let sparkle_phase = pt * 50.0 - t * 8.0;
    let sparkle = pow(max(0.0, sin(sparkle_phase)), 16.0);

    // Hue shifts along perimeter
    let h = fract(hue + pt * 0.5 + t * 0.1);
    let rgb = hue_to_rgb(h);

    // Border glow profile
    let glow = smoothstep(0.0, 0.5, dist) * smoothstep(5.0, 1.0, dist);

    // Impact pulse
    let impact = prox * prox * 0.6;

    let brightness = (wave * 0.6 + sparkle * 0.8 + impact) * prox;
    let final_rgb = rgb * glow * brightness * 1.5;
    let final_a   = glow * brightness * 0.9;

    return vec4f(final_rgb, final_a);
}

// ---- main scene -----------------------------------------------------------

fn sample_scene(px: vec2f) -> vec4f {
    var out = vec4f(0.0);
    let count = u32(u.element_count);
    let hover_idx = i32(u.hover_elem);

    for (var i = 0u; i < count; i++) {
        let e  = elems[i];
        let ex = e.rect.x;
        let ey = e.rect.y;
        let ew = e.rect.z;
        let eh = e.rect.w;
        let kind = u32(e.kind);

        if kind == 8u {
            let prox = hover_proximity(ex, ey, ew, eh);
            let contrib = graph_node(px, ex, ey, ew, eh, e.hue, e._p1, u.time, prox);
            out = out + contrib;
            continue;
        }

        let is_hovered = i32(i) == hover_idx;

        if is_hovered {
            let prox = hover_proximity(ex, ey, ew, eh);
            let border = hover_border(px, ex, ey, ew, eh, e.hue, u.time, prox);
            out = out + border;
        } else {
            let shadow = static_shadow(px, ex, ey, ew, eh);
            out = out + vec4f(0.0, 0.0, 0.0, shadow);
        }
    }

    return out;
}

// ---- fragment entry -------------------------------------------------------

@fragment
fn fs_main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let raw_px = pos.xy;
    let raw_uv = raw_px / vec2f(u.width, u.height);
    let t = u.time * 0.35;

    // --- Aurora background (soft hue-shifting) -----------------------------
    var p  = raw_uv * 3.5 + vec2f(t * 0.25, 0.0);
    let n1 = fbm(p);
    let n2 = fbm(p + vec2f(0.0, t * 0.08) + vec2f(n1 * 1.8));
    let n3 = fbm(p + vec2f(n2 * 1.4, 0.0) - vec2f(0.0, t * 0.06));
    let band      = smoothstep(0.25, 0.80, n3) * (1.0 - raw_uv.y * 0.9);
    let intensity = band * 0.12;
    let c1        = vec3f(0.10, 0.42, 0.50);
    let c2        = vec3f(0.18, 0.35, 0.58);
    let c3        = vec3f(0.38, 0.25, 0.52);
    let aurora_rgb = mix(mix(c1, c2, n2), c3, n1 * 0.6);
    var bg = aurora_rgb * intensity;

    let base_dark = vec3f(0.10, 0.10, 0.11);
    bg = base_dark + bg;

    // --- Scene elements (single evaluation) --------------------------------
    let scene = sample_scene(raw_px);

    // Composite scene over aurora background
    var color = bg * (1.0 - scene.a) + scene.rgb;

    // --- CRT effects -------------------------------------------------------
    let scanline = crt_scanlines(raw_px.y);
    let vline    = crt_vertical_lines(raw_px.x);
    let edge     = crt_edge_shadow(raw_uv);
    let phosphor = 0.97 + 0.03 * sin(raw_px.x * 6.28 * 0.333);
    let crt_dim  = scanline * vline * edge * phosphor;

    color = color * crt_dim;

    return vec4f(clamp(color, vec3f(0.0), vec3f(1.0)), 1.0);
}

// ---- instanced particle rendering ----------------------------------------
//
// Each particle is rendered as a small quad (6 vertices, instanced).
// This replaces the O(pixels × particles) per-pixel loop with
// O(particles × quad_area) — many orders of magnitude cheaper.

struct ParticleVarying {
    @builtin(position) clip_pos : vec4f,
    @location(0)       local_uv : vec2f,
    @location(1) @interpolate(flat) pidx : u32,
}

@vertex
fn vs_particle(
    @builtin(vertex_index)   vid : u32,
    @builtin(instance_index) iid : u32,
) -> ParticleVarying {
    var out: ParticleVarying;
    out.pidx = iid;

    let p = particles[iid];

    // Dead or invisible → degenerate quad (off-screen)
    if p.life <= 0.0 {
        out.clip_pos = vec4f(-2.0, -2.0, 0.0, 1.0);
        out.local_uv = vec2f(0.0);
        return out;
    }

    let radius = p.size * 4.0;

    // 6-vertex quad (two triangles)
    var corner: vec2f;
    switch vid {
        case 0u: { corner = vec2f(-1.0, -1.0); }
        case 1u: { corner = vec2f( 1.0, -1.0); }
        case 2u: { corner = vec2f(-1.0,  1.0); }
        case 3u: { corner = vec2f( 1.0, -1.0); }
        case 4u: { corner = vec2f( 1.0,  1.0); }
        default: { corner = vec2f(-1.0,  1.0); }
    }
    out.local_uv = corner;

    let world_pos = p.pos + corner * radius;
    let clip_x = world_pos.x / u.width  *  2.0 - 1.0;
    let clip_y = world_pos.y / u.height * -2.0 + 1.0;
    out.clip_pos = vec4f(clip_x, clip_y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_particle(in: ParticleVarying) -> @location(0) vec4f {
    let p = particles[in.pidx];

    let d_norm = length(in.local_uv);              // 0 at centre, 1 at quad edge
    let glow   = exp(-d_norm * d_norm * 8.0);      // Gaussian falloff
    let t_life = p.life / p.max_life;
    let bright = t_life * t_life * glow;

    if bright < 0.002 { discard; }

    let halo_rgb = hue_to_rgb(p.hue) * 1.5;
    let core     = smoothstep(0.125, 0.0, d_norm);  // hot white centre
    let col      = mix(halo_rgb, vec3f(1.5, 1.4, 1.2), core) * bright;
    let a        = min(bright * 0.8, 1.0);

    return vec4f(col * a, a);                       // premultiplied alpha
}