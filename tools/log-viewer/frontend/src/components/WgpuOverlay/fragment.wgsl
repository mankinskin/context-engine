// fragment.wgsl — thin glowing borders + 3D hover displacement + CRT effect
//
// Bindings
//   binding 0 (uniform)          : Uniforms  — time, viewport, element count, mouse
//   binding 1 (read-only-storage): ElemRect[] — DOM element rects uploaded each frame
//
// Features:
//   - Thin, bright border glows per element kind
//   - 3D displacement on hover (mouse acts as light, elements "lift" near cursor)
//   - CRT post-process: barrel distortion, fine scanlines, RGB fringe, vignette

// ---- uniforms / storage ---------------------------------------------------

struct Uniforms {
    time          : f32,
    width         : f32,
    height        : f32,
    element_count : f32,
    mouse_x       : f32,
    mouse_y       : f32,
    _pad0         : f32,
    _pad1         : f32,
}

struct ElemRect {
    rect : vec4f,   // x, y, w, h  (screen-space pixels, y=0 at top-left)
    hue  : f32,
    kind : f32,
    _p1  : f32,
    _p2  : f32,
}

@group(0) @binding(0) var<uniform>       u     : Uniforms;
@group(0) @binding(1) var<storage, read> elems : array<ElemRect>;

// ---- colour helpers -------------------------------------------------------

fn hue_to_rgb(h: f32) -> vec3f {
    let h6 = fract(h) * 6.0;
    let r  = abs(h6 - 3.0) - 1.0;
    let g  = 2.0 - abs(h6 - 2.0);
    let b  = 2.0 - abs(h6 - 4.0);
    return clamp(vec3f(r, g, b), vec3f(0.0), vec3f(1.0));
}

// ---- noise (aurora background) --------------------------------------------

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
    for (var i = 0; i < 5; i++) {
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

// ---- 3D hover displacement ------------------------------------------------

// Returns how much to brighten/shift an element based on mouse proximity.
// mouse acts as a "light source" — side facing cursor gets brighter.
fn hover_factor(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32) -> vec3f {
    let mouse = vec2f(u.mouse_x, u.mouse_y);
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let to_mouse = mouse - center;
    let mouse_dist = length(to_mouse);

    // Proximity: 1.0 when mouse is on element, fades over 200px
    let proximity = smoothstep(250.0, 0.0, mouse_dist);
    if proximity < 0.001 { return vec3f(0.0, 0.0, 0.0); }

    // Light direction from element center toward mouse
    let light_dir = select(vec2f(0.0, -1.0), normalize(to_mouse), mouse_dist > 1.0);

    // Pixel position relative to element center (normalised -1..1)
    let rel = (px - center) / vec2f(ew * 0.5, eh * 0.5);

    // Highlight: pixels on the side facing the mouse get brighter
    let facing = dot(normalize(rel), light_dir);
    let highlight = max(0.0, facing) * proximity;

    // Shadow offset: pixels on the opposite side get a shadow
    let shadow_offset = proximity * 3.0;  // pixels of displacement

    // Return: x = brightness boost, y = shadow, z = proximity
    return vec3f(highlight, shadow_offset, proximity);
}

// ---- thin border glows per kind -------------------------------------------

// Kind 0: structural — thin rainbow border
fn glow_structural(dist: f32, pt: f32, hue: f32, t: f32, hover: vec3f) -> vec4f {
    let glow  = smoothstep(0.0, 1.0, dist) * smoothstep(6.0 + hover.z * 4.0, 1.0, dist);
    let h     = fract(hue + pt * 2.0 + t * 0.12);
    let rgb   = hue_to_rgb(h);
    let pulse = 0.7 + 0.3 * sin(t * 2.0 + hue * 6.28);
    let alpha = glow * (0.6 + hover.x * 0.5) * pulse;
    return vec4f(rgb * alpha, alpha);
}

// Kind 1: error — thin red/magenta pulsing border
fn glow_error(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let glow  = smoothstep(0.0, 0.5, dist) * smoothstep(8.0 + hover.z * 6.0, 1.0, dist);
    let pulse = 0.5 + 0.5 * sin(t * 5.0);
    let h     = fract(0.97 + 0.06 * sin(t * 3.0 + pt * 12.56));
    let rgb   = hue_to_rgb(h);
    let alpha = glow * (0.75 + hover.x * 0.4) * pulse;
    return vec4f(rgb * alpha, alpha);
}

// Kind 2: warn — thin amber/yellow travelling wave
fn glow_warn(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let glow = smoothstep(0.0, 1.0, dist) * smoothstep(7.0 + hover.z * 5.0, 1.0, dist);
    let wave = 0.4 + 0.6 * (0.5 + 0.5 * sin(pt * 18.85 - t * 4.0));
    let h    = fract(0.1 + 0.04 * sin(pt * 6.28 + t * 2.0));
    let rgb  = hue_to_rgb(h);
    let alpha = glow * (0.65 + hover.x * 0.4) * wave;
    return vec4f(rgb * alpha, alpha);
}

// Kind 3: info — thin cyan/blue breathing border
fn glow_info(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let glow    = smoothstep(0.0, 1.0, dist) * smoothstep(6.0 + hover.z * 4.0, 1.0, dist);
    let breathe = 0.5 + 0.5 * sin(t * 1.5);
    let h       = fract(0.52 + 0.08 * sin(pt * 6.28 + t * 1.0));
    let rgb     = hue_to_rgb(h);
    let alpha   = glow * (0.6 + hover.x * 0.4) * breathe;
    return vec4f(rgb * alpha, alpha);
}

// Kind 4: debug/trace — thin soft green/teal border
fn glow_debug(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let glow  = smoothstep(0.0, 1.0, dist) * smoothstep(5.0 + hover.z * 3.0, 1.0, dist);
    let pulse = 0.6 + 0.4 * sin(t * 0.9);
    let h     = fract(0.38 + 0.05 * sin(pt * 6.28 + t * 0.5));
    let rgb   = hue_to_rgb(h);
    let alpha = glow * (0.35 + hover.x * 0.4) * pulse;
    return vec4f(rgb * alpha, alpha);
}

// Kind 5: span-highlighted — thin neon rainbow sweep
fn glow_span_hl(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let glow = smoothstep(0.0, 0.5, dist) * smoothstep(7.0 + hover.z * 5.0, 1.0, dist);
    let h    = fract(pt * 3.0 + t * 0.5);
    let rgb  = hue_to_rgb(h);
    let sweep = 0.5 + 0.5 * sin(pt * 6.28 - t * 3.0);
    let alpha = glow * (0.5 + 0.5 * sweep) * (0.7 + hover.x * 0.4);
    return vec4f(rgb * alpha, alpha);
}

// Kind 6: selected — thin white-gold focus ring
fn glow_selected(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let ring  = smoothstep(0.0, 0.5, dist) * smoothstep(4.0 + hover.z * 4.0, 1.0, dist);
    let halo  = smoothstep(0.0, 2.0, dist) * smoothstep(10.0 + hover.z * 6.0, 4.0, dist);
    let pulse = 0.75 + 0.25 * sin(t * 2.5);
    let h     = fract(0.12 + 0.03 * sin(pt * 12.56 + t * 2.0));
    let rgb   = hue_to_rgb(h) * 0.3 + vec3f(1.0, 0.92, 0.6) * 0.7;
    let alpha = (ring * 0.8 + halo * 0.2) * pulse * (1.0 + hover.x * 0.3);
    return vec4f(rgb * alpha, alpha);
}

// Kind 7: panic — thin strobing red alarm border
fn glow_panic(dist: f32, pt: f32, t: f32, hover: vec3f) -> vec4f {
    let glow   = smoothstep(0.0, 0.5, dist) * smoothstep(10.0 + hover.z * 6.0, 1.0, dist);
    let strobe = 0.3 + 0.7 * abs(sin(t * 7.0));
    let flash  = smoothstep(0.92, 1.0, sin(t * 1.5));
    let h      = fract(0.0 + 0.04 * flash);
    let rgb    = hue_to_rgb(h);
    let alpha  = glow * (0.7 + 0.3 * flash + hover.x * 0.3) * strobe;
    return vec4f(rgb * alpha, alpha);
}

// ---- Kind 8: graph node — 3D-shaded rounded rectangle --------------------
//
// Renders the node body with a gradient fill, 3D lighting from the mouse,
// rounded corners, inner highlight, and a glowing border.
// Uses:
//   hue   = level-based colour
//   _p1   = node_type (0=event, 1=span_enter, 2=span_exit)

// Rounded-rectangle SDF: returns distance to the rounded-rect boundary
// (negative = inside, positive = outside).
fn rounded_rect_sdf(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32, radius: f32) -> f32 {
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let half   = vec2f(ew * 0.5, eh * 0.5);
    let rel    = abs(px - center) - half + vec2f(radius);
    return length(max(rel, vec2f(0.0))) + min(max(rel.x, rel.y), 0.0) - radius;
}

fn graph_node(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32,
              hue: f32, node_type: f32, t: f32, hover: vec3f) -> vec4f {
    let radius = 10.0;
    let sdf = rounded_rect_sdf(px, ex, ey, ew, eh, radius);

    // Outside the rounded rect — nothing
    if sdf > 3.0 { return vec4f(0.0); }

    // Anti-aliased edge mask (1.0 inside, 0.0 outside, smooth at boundary)
    let body_mask = smoothstep(1.0, -0.5, sdf);

    // Normalised position within the node
    let nx = (px.x - ex) / ew;  // 0..1
    let ny = (px.y - ey) / eh;  // 0..1

    // ---- base gradient fill -----------------------------------------------
    // Vertical gradient: lighter at top, darker at bottom
    let base_rgb = hue_to_rgb(hue);
    let top_rgb    = base_rgb * 0.9 + vec3f(0.25);   // lighter
    let bottom_rgb = base_rgb * 0.45;                 // darker
    var fill_rgb   = mix(top_rgb, bottom_rgb, ny);

    // ---- 3D mouse-based lighting ------------------------------------------
    let mouse = vec2f(u.mouse_x, u.mouse_y);
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let to_mouse = normalize(mouse - center + vec2f(0.001));
    // Fake surface normal based on position within node
    let normal = vec2f((nx - 0.5) * 0.3, (ny - 0.5) * 0.5);
    // Diffuse lighting
    let diffuse = max(0.0, dot(normalize(normal + vec2f(0.0, -0.3)), to_mouse));
    // Specular highlight
    let spec_pos = vec2f(nx - 0.5, ny - 0.3);
    let spec = pow(max(0.0, 1.0 - length(spec_pos - to_mouse * 0.2) * 2.0), 8.0);
    // Apply intensity based on proximity
    let light_boost = hover.z * 0.6;
    fill_rgb = fill_rgb + fill_rgb * diffuse * (0.15 + light_boost);
    fill_rgb = fill_rgb + vec3f(1.0) * spec * (0.08 + light_boost * 0.3);

    // ---- inner highlight (top edge gleam) ----------------------------------
    let top_gleam = smoothstep(0.15, 0.0, ny) * 0.3;
    fill_rgb = fill_rgb + vec3f(1.0) * top_gleam;

    // ---- inner shadow (bottom edge) ----------------------------------------
    let bot_shadow = smoothstep(0.85, 1.0, ny) * 0.15;
    fill_rgb = fill_rgb * (1.0 - bot_shadow);

    // ---- node type indicators ---------------------------------------------
    let ntype = u32(node_type);
    if ntype == 1u {
        // Span enter: green left accent bar
        let bar = smoothstep(3.0, 0.0, px.x - ex) * smoothstep(-1.0, 0.0, sdf);
        fill_rgb = mix(fill_rgb, vec3f(0.2, 0.9, 0.3), bar * 0.8);
    } else if ntype == 2u {
        // Span exit: red right accent bar
        let bar = smoothstep(3.0, 0.0, (ex + ew) - px.x) * smoothstep(-1.0, 0.0, sdf);
        fill_rgb = mix(fill_rgb, vec3f(0.9, 0.2, 0.2), bar * 0.8);
    }

    // ---- glowing border ring -----------------------------------------------
    let border_glow = smoothstep(-2.0, 0.0, sdf) * smoothstep(2.0, 0.5, sdf);
    let border_rgb  = hue_to_rgb(fract(hue + t * 0.05)) * 1.2;
    let border_boost = 1.0 + hover.z * 0.8;
    fill_rgb = fill_rgb + border_rgb * border_glow * 0.5 * border_boost;

    // ---- outer glow (extends slightly beyond the body) --------------------
    let outer = smoothstep(3.0, 0.0, sdf) * (1.0 - body_mask);
    let outer_rgb = base_rgb * 0.8;
    let outer_alpha = outer * 0.3 * (1.0 + hover.z * 0.6);

    // ---- shadow beneath (3D depth illusion) --------------------------------
    let shadow_sdf = rounded_rect_sdf(px - vec2f(-2.0, 3.0 + hover.z * 4.0), ex, ey, ew, eh, radius);
    let shadow_mask = smoothstep(0.0, 8.0, -shadow_sdf) * (1.0 - body_mask);
    let shadow_alpha = shadow_mask * 0.2 * (0.5 + hover.z * 0.5);

    // ---- composit -----------------------------------------------------------
    let body_alpha = body_mask * 0.75;  // semi-transparent so Cytoscape labels show
    let total_rgb = fill_rgb * body_alpha
                  + outer_rgb * outer_alpha
                  + vec3f(0.0) * shadow_alpha;
    let total_a = body_alpha + outer_alpha + shadow_alpha;

    return vec4f(total_rgb, total_a);
}

// ---- CRT post-processing -------------------------------------------------

// Barrel distortion: warp UV outward from center to simulate curved CRT glass
fn barrel_distort(uv_in: vec2f) -> vec2f {
    let c     = uv_in - 0.5;                    // center-relative
    let r2    = dot(c, c);                       // squared radius
    let k     = 0.15;                            // distortion strength
    let warped = c * (1.0 + k * r2);
    return warped + 0.5;
}

// CRT scanlines: darken every other pixel row (based on original screen coords)
fn crt_scanlines(py: f32) -> f32 {
    // Fine scanlines: alternating rows with slight darkening
    let line = 0.85 + 0.15 * sin(py * 3.14159);
    return line;
}

// RGB sub-pixel fringe (chromatic aberration near edges)
fn rgb_fringe(uv: vec2f) -> vec3f {
    let c  = uv - 0.5;
    let r2 = dot(c, c);
    // Offset increases toward edges
    let shift = r2 * 0.006;
    return vec3f(shift, 0.0, -shift);  // R shifts out, B shifts in
}

// CRT vignette: darken corners of the screen
fn crt_vignette(uv: vec2f) -> f32 {
    let c  = uv - 0.5;
    let r2 = dot(c, c);
    return smoothstep(0.55, 0.2, r2);
}

// ---- main fragment --------------------------------------------------------

// Sample the scene at a given pixel position (aurora + element glows)
fn sample_scene(px: vec2f) -> vec4f {
    let uv = px / vec2f(u.width, u.height);
    let t  = u.time * 0.35;

    // Aurora background
    var p  = uv * 3.5 + vec2f(t * 0.25, 0.0);
    let n1 = fbm(p);
    let n2 = fbm(p + vec2f(0.0, t * 0.08) + vec2f(n1 * 1.8));
    let n3 = fbm(p + vec2f(n2 * 1.4, 0.0) - vec2f(0.0, t * 0.06));
    let band      = smoothstep(0.25, 0.80, n3) * (1.0 - uv.y * 0.9);
    let intensity = band * 0.18;
    let c1        = vec3f(0.10, 0.42, 0.50);
    let c2        = vec3f(0.18, 0.35, 0.58);
    let c3        = vec3f(0.38, 0.25, 0.52);
    let aurora_rgb = mix(mix(c1, c2, n2), c3, n1 * 0.6);
    var out = vec4f(aurora_rgb * intensity, intensity * 0.55);

    // Per-element thin border glows with 3D hover
    let count = u32(u.element_count);
    for (var i = 0u; i < count; i++) {
        let e  = elems[i];
        let r  = e.rect;
        let ex = r.x;
        let ey = r.y;
        let ew = r.z;
        let eh = r.w;

        // 3D hover displacement: shift sample point away from mouse
        let hover = hover_factor(px, ex, ey, ew, eh);
        let kind = u32(e.kind);

        // Graph nodes use the rounded-rect SDF approach (no displacement)
        if kind == 8u {
            let contrib = graph_node(px, ex, ey, ew, eh, e.hue, e._p1, u.time, hover);
            out = out + contrib;
            continue;
        }

        let offset = hover.y * normalize(px - vec2f(u.mouse_x, u.mouse_y) + vec2f(0.001));
        let sample_px = px - offset;

        if sample_px.x >= ex && sample_px.x < ex + ew && sample_px.y >= ey && sample_px.y < ey + eh {
            let dist = edge_dist(sample_px, ex, ey, ew, eh);
            let pt   = perimeter_t(sample_px, ex, ey, ew, eh);

            var contrib = vec4f(0.0);
            switch kind {
                case 0u { contrib = glow_structural(dist, pt, e.hue, u.time, hover); }
                case 1u { contrib = glow_error(dist, pt, u.time, hover); }
                case 2u { contrib = glow_warn(dist, pt, u.time, hover); }
                case 3u { contrib = glow_info(dist, pt, u.time, hover); }
                case 4u { contrib = glow_debug(dist, pt, u.time, hover); }
                case 5u { contrib = glow_span_hl(dist, pt, u.time, hover); }
                case 6u { contrib = glow_selected(dist, pt, u.time, hover); }
                case 7u { contrib = glow_panic(dist, pt, u.time, hover); }
                default { contrib = glow_structural(dist, pt, e.hue, u.time, hover); }
            }

            // Hover shadow: faint dark offset on the opposite side
            let shadow_a = hover.z * 0.15 * smoothstep(4.0, 0.0, dist);
            contrib = contrib + vec4f(0.0, 0.0, 0.0, shadow_a);

            out = out + contrib;
        }
    }

    return out;
}

@fragment
fn fs_main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let raw_px = pos.xy;
    let raw_uv = raw_px / vec2f(u.width, u.height);

    let px = raw_px;

    // --- RGB chromatic aberration ------------------------------------------
    let fringe = rgb_fringe(raw_uv);
    let px_r = (raw_uv + vec2f(fringe.x, 0.0)) * vec2f(u.width, u.height);
    let px_g = px;
    let px_b = (raw_uv + vec2f(fringe.z, 0.0)) * vec2f(u.width, u.height);

    let scene_r = sample_scene(px_r);
    let scene_g = sample_scene(px_g);
    let scene_b = sample_scene(px_b);

    // Merge RGB channels with chromatic split
    var color = vec4f(scene_r.r, scene_g.g, scene_b.b,
                      max(max(scene_r.a, scene_g.a), scene_b.a));

    // --- CRT fine scanlines ------------------------------------------------
    let scanline_dim = crt_scanlines(raw_px.y);
    color = vec4f(color.rgb * scanline_dim, color.a);

    // --- CRT vignette ------------------------------------------------------
    let vig = crt_vignette(raw_uv);
    color = vec4f(color.rgb * vig, color.a * vig);

    // --- Slight phosphor glow (softens the look) ---------------------------
    let phosphor = 0.97 + 0.03 * sin(raw_px.x * 6.28 * 0.333);
    color = vec4f(color.rgb * phosphor, color.a);

    // Canvas is behind HTML — force fully opaque so there are no holes
    let final_color = clamp(color, vec4f(0.0), vec4f(1.0));
    return vec4f(final_color.rgb, 1.0);
}
