// background.wgsl — fullscreen scene rendering (Dark Souls cinder theme)
//
// Concatenated after: types.wgsl + noise.wgsl
// Contains: fullscreen quad VS, scene element rendering, smoky background,
//           CRT post-processing, fragment entry point.
//
// Canvas sits BEHIND HTML (z-index -1, opaque).  HTML backgrounds are
// transparent so the dark texture shows through.

// ---- bindings (render pass — read-only) -------------------------------------

@group(0) @binding(0) var<uniform>       u         : Uniforms;
@group(0) @binding(1) var<storage, read> elems     : array<ElemRect>;
@group(0) @binding(2) var<storage, read> particles : array<Particle>;

// ---- fullscreen quad vertex shader ------------------------------------------

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4f {
    var pos = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f( 1.0, -1.0), vec2f(-1.0,  1.0),
        vec2f( 1.0, -1.0), vec2f( 1.0,  1.0), vec2f(-1.0,  1.0),
    );
    return vec4f(pos[vi], 0.0, 1.0);
}

// ---- edge helpers -----------------------------------------------------------

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
    if (ew - lx) < ly && (ew - lx) < (eh - ly)     { return (ew + ly) / perim; }
    if (eh - ly) < lx && (eh - ly) < (ew - lx)      { return (ew + eh + (ew - lx)) / perim; }
    return (2.0 * ew + eh + (eh - ly)) / perim;
}

// ---- rounded-rect SDF ------------------------------------------------------

fn rounded_rect_sdf(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32, radius: f32) -> f32 {
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let half   = vec2f(ew * 0.5, eh * 0.5);
    let rel    = abs(px - center) - half + vec2f(radius);
    return length(max(rel, vec2f(0.0))) + min(max(rel.x, rel.y), 0.0) - radius;
}

// ---- hover proximity --------------------------------------------------------

fn hover_proximity(ex: f32, ey: f32, ew: f32, eh: f32) -> f32 {
    let mouse  = vec2f(u.mouse_x, u.mouse_y);
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let dist   = length(mouse - center);
    return smoothstep(max(ew, eh) * 0.8, 0.0, dist);
}

// ---- graph node (kind 8) — dark stone / iron slab ---------------------------

fn graph_node(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32,
              hue: f32, node_type: f32, t: f32, prox: f32) -> vec4f {
    let radius = 6.0;
    let sdf = rounded_rect_sdf(px, ex, ey, ew, eh, radius);
    if sdf > 4.0 { return vec4f(0.0); }

    let body_mask = smoothstep(0.5, -0.5, sdf);
    let nx = (px.x - ex) / ew;
    let ny = (px.y - ey) / eh;

    // Stone material: dark grey with subtle noise grain
    let stone_noise = smooth_noise(px * 0.15) * 0.08;
    let stone_base  = vec3f(0.16, 0.15, 0.14) + vec3f(stone_noise);
    let stone_top   = stone_base + vec3f(0.06, 0.05, 0.04);
    let stone_bot   = stone_base * 0.7;
    var fill_rgb    = mix(stone_top, stone_bot, ny);

    // Subtle vine veins
    let vine_n = smooth_noise(px * 0.08 + vec2f(3.7, 1.2));
    let vine_streak = smoothstep(0.48, 0.52, vine_n) * 0.3;
    fill_rgb = mix(fill_rgb, vec3f(0.12, 0.30, 0.10), vine_streak);

    // Mouse-based torch lighting
    let mouse  = vec2f(u.mouse_x, u.mouse_y);
    let center = vec2f(ex + ew * 0.5, ey + eh * 0.5);
    let to_mouse = normalize(mouse - center + vec2f(0.001));
    let normal   = vec2f((nx - 0.5) * 0.3, (ny - 0.5) * 0.5);
    let diffuse  = max(0.0, dot(normalize(normal + vec2f(0.0, -0.3)), to_mouse));
    let torch_col = vec3f(0.9, 0.5, 0.15);
    let lb = prox * 0.7;
    fill_rgb = fill_rgb + torch_col * diffuse * (0.10 + lb * 0.4);

    // Specular — dull metal sheen
    let spec_pos = vec2f(nx - 0.5, ny - 0.3);
    let spec = pow(max(0.0, 1.0 - length(spec_pos - to_mouse * 0.2) * 2.5), 12.0);
    fill_rgb = fill_rgb + vec3f(0.6, 0.5, 0.3) * spec * (0.05 + lb * 0.15);

    // Chiselled top edge gleam
    fill_rgb = fill_rgb + vec3f(0.5, 0.45, 0.35) * smoothstep(0.12, 0.0, ny) * 0.15;
    fill_rgb = fill_rgb * (1.0 - smoothstep(0.88, 1.0, ny) * 0.2);

    // Node type accent — vine (enter) or blood (exit)
    let ntype = u32(node_type);
    if ntype == 1u {
        let bar = smoothstep(3.0, 0.0, px.x - ex) * smoothstep(-1.0, 0.0, sdf);
        fill_rgb = mix(fill_rgb, vec3f(0.15, 0.40, 0.10), bar * 0.7);
    } else if ntype == 2u {
        let bar = smoothstep(3.0, 0.0, (ex + ew) - px.x) * smoothstep(-1.0, 0.0, sdf);
        fill_rgb = mix(fill_rgb, vec3f(0.55, 0.08, 0.05), bar * 0.7);
    }

    // Iron border with ember glow on hover
    let border_band = smoothstep(-1.5, 0.0, sdf) * smoothstep(2.0, 0.5, sdf);
    let ember_pulse = 0.5 + 0.5 * sin(t * 2.0 + nx * 6.28);
    let border_rgb  = mix(vec3f(0.10, 0.09, 0.08), vec3f(0.7, 0.25, 0.05), prox * ember_pulse * 0.6);
    fill_rgb = fill_rgb + border_rgb * border_band * 0.5;

    // Deep shadow beneath
    let outer       = smoothstep(4.0, 0.0, sdf) * (1.0 - body_mask);
    let outer_alpha = outer * 0.15;
    let shadow_sdf  = rounded_rect_sdf(px - vec2f(-1.0, 4.0 + prox * 3.0), ex, ey, ew, eh, radius);
    let shadow_mask = smoothstep(0.0, 10.0, -shadow_sdf) * (1.0 - body_mask);
    let shadow_alpha = shadow_mask * 0.35 * (0.6 + prox * 0.4);

    let body_alpha = body_mask * 0.80;
    let total_rgb  = fill_rgb * body_alpha + vec3f(0.08, 0.06, 0.04) * outer_alpha;
    let total_a    = body_alpha + outer_alpha + shadow_alpha;

    return vec4f(total_rgb, total_a);
}

// ---- CRT post-processing ---------------------------------------------------

fn crt_scanlines(py: f32) -> f32 {
    return 0.82 + 0.18 * sin(py * 3.14159);
}

fn crt_vertical_lines(px_x: f32) -> f32 {
    return 0.88 + 0.12 * sin(px_x * 3.14159 * 0.6667);
}

// Pixel-grid opacity effect — screen-door pattern (no colour shift)
fn crt_pixel_grid(px: vec2f) -> f32 {
    let cell = 3.0;
    // Horizontal gap between pixel cells
    let gx = smoothstep(0.0, 0.6, px.x % cell)
           * smoothstep(cell, cell - 0.6, px.x % cell);
    // Vertical gap between pixel cells
    let gy = smoothstep(0.0, 0.6, px.y % cell)
           * smoothstep(cell, cell - 0.6, px.y % cell);
    // Mix: mostly opaque, subtle grid darkening
    return mix(1.0, gx * gy, 0.22);
}

fn crt_edge_shadow(uv: vec2f) -> f32 {
    let d_left   = uv.x;
    let d_right  = 1.0 - uv.x;
    let d_top    = uv.y;
    let d_bottom = 1.0 - uv.y;
    let d = min(min(d_left, d_right), min(d_top, d_bottom));
    return smoothstep(0.0, 0.04, d) * (0.7 + 0.3 * smoothstep(0.0, 0.15, d));
}

// ---- static thin shadow for non-hovered elements ----------------------------

fn static_shadow(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32) -> f32 {
    let inside_x = px.x >= ex && px.x < ex + ew;
    let inside_y = px.y >= ey && px.y < ey + eh;
    if !(inside_x && inside_y) { return 0.0; }
    let dist = edge_dist(px, ex, ey, ew, eh);
    return smoothstep(3.0, 0.0, dist) * 0.20;
}

// ---- ember hover border — smouldering cracks along edges --------------------

fn hover_border(px: vec2f, ex: f32, ey: f32, ew: f32, eh: f32,
                hue: f32, t: f32, prox: f32) -> vec4f {
    let inside_x = px.x >= ex - 4.0 && px.x < ex + ew + 4.0;
    let inside_y = px.y >= ey - 4.0 && px.y < ey + eh + 4.0;
    if !(inside_x && inside_y) { return vec4f(0.0); }

    if !(inside_x && inside_y) { return vec4f(0.0); }

    let dist = edge_dist(px, ex, ey, ew, eh);
    let pt   = perimeter_t(px, ex, ey, ew, eh);

    if dist > 7.0 { return vec4f(0.0); }

    // Crackling ember wave — irregular, like smouldering cracks
    let crack_n = smooth_noise(vec2f(pt * 40.0, t * 1.5));
    let crack   = pow(crack_n, 3.0);

    // Slow pulsing heat
    let pulse = 0.6 + 0.4 * sin(t * 1.5 + pt * 6.28);

    // Ember colour: deep orange → dull red, with vine-green flickers
    let ember_core = vec3f(0.90, 0.35, 0.06);
    let ember_edge = vec3f(0.50, 0.12, 0.03);
    let vine_flick = vec3f(0.15, 0.40, 0.08);
    var ember_rgb = mix(ember_edge, ember_core, crack * pulse);
    let vine_f = smoothstep(0.7, 0.9, smooth_noise(vec2f(pt * 20.0 + 5.0, t * 0.5)));
    ember_rgb = mix(ember_rgb, vine_flick, vine_f * 0.4);

    // Border glow profile
    let glow = smoothstep(0.0, 0.5, dist) * smoothstep(6.0, 1.0, dist);

    // Impact: bonfire flare
    let impact = prox * prox * 0.5;

    let brightness = (crack * 0.7 + pulse * 0.3 + impact) * prox;
    let final_rgb = ember_rgb * glow * brightness * 1.2;
    let final_a   = glow * brightness * 0.85;

    return vec4f(final_rgb, final_a);
}

// ---- main scene compositing -------------------------------------------------

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

// ---- fragment entry point ---------------------------------------------------

@fragment
fn fs_main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let raw_px = pos.xy;
    let raw_uv = raw_px / vec2f(u.width, u.height);
    let t = u.time * 0.35;

    // --- Smoky dark background with varied palette and vignette ---------------
    // Downsample coordinates for a chunky/pixelated feel
    let pixel_size = 4.0;
    let ds_px = floor(raw_px / pixel_size) * pixel_size;
    let ds_uv = ds_px / vec2f(u.width, u.height);

    // Vignette — darken edges, bright centre
    let vig_d = length((raw_uv - 0.5) * vec2f(1.2, 1.0));
    let vignette = 1.0 - smoothstep(0.3, 1.1, vig_d) * 0.55;

    // Animated coarse noise — drifting slowly
    let drift = vec2f(t * 0.12, t * 0.06);
    let n_fine   = smooth_noise((ds_px + drift * 40.0) * 0.025) * 0.028;
    let n_coarse = smooth_noise((ds_px + drift * 20.0) * 0.006 + vec2f(7.3, 2.1)) * 0.018;
    let n_grain  = hash2(ds_px * 0.37 + vec2f(floor(u.time * 8.0))) * 0.015;

    // Varied base palette — subtle colour variation across the screen
    let palette_t = smooth_noise(ds_px * 0.003 + drift * 5.0);
    let cool_tone = vec3f(0.03, 0.035, 0.05);   // blue-grey
    let warm_tone = vec3f(0.055, 0.035, 0.025);  // brown-amber
    let mid_tone  = vec3f(0.035, 0.04, 0.035);   // mossy
    var base_col = mix(cool_tone, warm_tone, smoothstep(0.3, 0.7, palette_t));
    base_col = mix(base_col, mid_tone, smoothstep(0.5, 0.8, smooth_noise(ds_px * 0.005 + vec2f(3.0, -t * 0.02))) * 0.5);
    var bg = base_col + vec3f(n_fine + n_coarse + n_grain);

    // --- Layered animated smoke wisps ----------------------------------------
    // Layer 1: large slow rolling smoke
    let smoke1_uv = ds_uv * 2.0 + vec2f(t * 0.06, t * 0.025);
    let smoke1 = fbm(smoke1_uv) * 0.05;

    // Layer 2: medium tendrils drifting opposite direction
    let smoke2_uv = ds_uv * 4.0 + vec2f(-t * 0.09, t * 0.05);
    let smoke2 = fbm(smoke2_uv) * 0.03;

    // Layer 3: fine fast wisps — curling upward
    let smoke3_uv = ds_uv * 7.0 + vec2f(sin(t * 0.3) * 0.5, -t * 0.12);
    let smoke3 = fbm(smoke3_uv) * 0.018;

    // Layer 4: very slow deep background churn
    let smoke4_uv = ds_uv * 1.2 + vec2f(t * 0.015, -t * 0.01);
    let smoke4 = fbm(smoke4_uv) * 0.035;

    // Composite smoke with slight colour tinting per layer
    bg = bg + vec3f(smoke1 + smoke4) * vec3f(0.85, 0.80, 0.75);  // warm base smoke
    bg = bg + vec3f(smoke2) * vec3f(0.6, 0.7, 0.85);              // cool mid wisps
    bg = bg + vec3f(smoke3) * vec3f(0.9, 0.85, 0.7);              // warm fine wisps

    // Faint animated grain shimmer
    let grain_hi = smooth_noise((ds_px + drift * 60.0) * 0.12) * 0.012;
    bg = bg + vec3f(grain_hi * 0.6, grain_hi * 0.55, grain_hi * 0.5);

    // Dim warm underglow from bottom edge
    let underglow = smoothstep(1.0, 0.4, raw_uv.y) * 0.015;
    bg = bg + vec3f(0.5, 0.18, 0.05) * underglow;

    // Apply vignette
    bg = bg * vignette;

    // --- Scene elements ------------------------------------------------------
    let scene = sample_scene(raw_px);
    var color = bg * (1.0 - scene.a) + scene.rgb;

    // --- Atmospheric CRT effects ---------------------------------------------
    let scanline = crt_scanlines(raw_px.y);
    let vline    = crt_vertical_lines(raw_px.x);
    let edge     = crt_edge_shadow(raw_uv);
    let torch_flicker = 0.97 + 0.03 * sin(t * 3.0 + raw_uv.x * 2.0);
    let pixel_grid = crt_pixel_grid(raw_px);
    let crt_dim  = scanline * vline * edge * torch_flicker;

    color = color * crt_dim * pixel_grid;

    return vec4f(clamp(color, vec3f(0.0), vec3f(1.0)), 1.0);
}
