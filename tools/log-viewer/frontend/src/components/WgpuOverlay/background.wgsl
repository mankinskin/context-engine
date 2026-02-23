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

// ---- GPU cursor rendering ---------------------------------------------------

// Signed distance to a line segment (a → b), returns distance from point p
fn sd_segment(p: vec2f, a: vec2f, b: vec2f) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

// Arrow cursor SDF — proper pointer shape with shaft notch
fn cursor_arrow_sdf(p: vec2f) -> f32 {
    // 7-vertex polygon:  tip → right diagonal → notch-in → shaft-bottom-right
    //                   → shaft-bottom-left → notch-left → left edge
    let v0 = vec2f(0.0,  0.0);    // tip
    let v1 = vec2f(16.0, 16.8);   // right diagonal end
    let v2 = vec2f(6.8,  14.0);   // notch inward
    let v3 = vec2f(11.0, 24.0);   // shaft bottom-right
    let v4 = vec2f(5.5,  24.0);   // shaft bottom-left
    let v5 = vec2f(4.0,  16.0);   // notch left
    let v6 = vec2f(0.0,  22.0);   // left edge bottom

    // Winding-number polygon SDF (7 edges)
    var d = dot(p - v0, p - v0);
    var s = 1.0;

    // Helper: per-edge distance + winding update
    // Edge v0→v6
    var e = v6 - v0; var w = p - v0;
    var b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    var c0 = w.y >= 0.0; var c1 = e.y * w.x > e.x * w.y; var c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    // Edge v6→v5
    e = v5 - v6; w = p - v6;
    b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    c0 = w.y >= 0.0; c1 = e.y * w.x > e.x * w.y; c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    // Edge v5→v4
    e = v4 - v5; w = p - v5;
    b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    c0 = w.y >= 0.0; c1 = e.y * w.x > e.x * w.y; c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    // Edge v4→v3
    e = v3 - v4; w = p - v4;
    b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    c0 = w.y >= 0.0; c1 = e.y * w.x > e.x * w.y; c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    // Edge v3→v2
    e = v2 - v3; w = p - v3;
    b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    c0 = w.y >= 0.0; c1 = e.y * w.x > e.x * w.y; c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    // Edge v2→v1
    e = v1 - v2; w = p - v2;
    b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    c0 = w.y >= 0.0; c1 = e.y * w.x > e.x * w.y; c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    // Edge v1→v0
    e = v0 - v1; w = p - v1;
    b2 = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
    d = min(d, dot(b2, b2));
    c0 = w.y >= 0.0; c1 = e.y * w.x > e.x * w.y; c2 = w.y >= e.y;
    if ((c0 && c1 && !c2) || (!c0 && !c1 && c2)) { s *= -1.0; }

    return s * sqrt(d);
}

// ---- procedural texture helpers for cursors ---------------------------------

// Voronoi cell noise — returns (cell_distance, cell_id) for metallic grain
fn voronoi(p: vec2f) -> vec2f {
    let ip = floor(p);
    let fp = fract(p);
    var min_d = 8.0;
    var cell_id = 0.0;
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let neighbor = vec2f(f32(i), f32(j));
            let point = vec2f(hash2(ip + neighbor + vec2f(0.0, 0.0)),
                              hash2(ip + neighbor + vec2f(17.3, 31.7)));
            let diff = neighbor + point - fp;
            let dist = dot(diff, diff);
            if (dist < min_d) {
                min_d = dist;
                cell_id = hash2(ip + neighbor + vec2f(53.1, 97.3));
            }
        }
    }
    return vec2f(sqrt(min_d), cell_id);
}

// High-detail FBM with more octaves for texturing
fn fbm5(p_in: vec2f) -> f32 {
    var val  = 0.0;
    var amp  = 0.5;
    var freq = 1.0;
    var p    = p_in;
    for (var i = 0; i < 5; i++) {
        val  += amp * smooth_noise(p * freq);
        amp  *= 0.5;
        freq *= 2.1;
        // Rotate each octave slightly for less axis-alignment
        p = vec2f(p.x * 0.866 - p.y * 0.5, p.x * 0.5 + p.y * 0.866);
    }
    return val;
}

// ---- Metal cursor — forged dark iron with hammer marks, rust, patina --------

fn cursor_metal(px: vec2f, mouse: vec2f, t: f32) -> vec4f {
    let local = px - mouse;

    // Expanded bounding box for shadow
    if (local.x < -4.0 || local.x > 22.0 || local.y < -4.0 || local.y > 30.0) {
        return vec4f(0.0);
    }

    let sdf = cursor_arrow_sdf(local);

    // Anti-aliased edge (sub-pixel smooth)
    let aa = 1.0 - smoothstep(-1.2, 0.6, sdf);
    if (aa < 0.001) { return vec4f(0.0); }

    let uv = local / vec2f(16.0, 24.0);

    // ── Surface normal with height-field detail ──
    // Base dome curvature (convex shield shape)
    let dome = vec2f((uv.x - 0.4) * 0.5, (uv.y - 0.45) * 0.3);

    // Hammer-strike dents — large low-frequency deformations
    let dent1 = smooth_noise(local * 0.35 + vec2f(42.0, 17.0));
    let dent2 = smooth_noise(local * 0.22 + vec2f(88.0, 53.0));
    let dent_h = (dent1 * 0.6 + dent2 * 0.4) * 0.15;

    // Forged grain — directional, running along the cursor axis
    let grain_angle = 0.15; // slight rotation
    let grain_p = vec2f(
        local.x * cos(grain_angle) - local.y * sin(grain_angle),
        local.x * sin(grain_angle) + local.y * cos(grain_angle)
    );
    let grain = smooth_noise(vec2f(grain_p.x * 6.0, grain_p.y * 0.8 + 200.0)) * 0.06;

    // Fine micro-scratches (high freq, anisotropic)
    let scratch1 = smooth_noise(vec2f(local.x * 12.0, local.y * 1.5 + 500.0)) * 0.02;
    let scratch2 = smooth_noise(vec2f(local.x * 1.8 + 300.0, local.y * 10.0)) * 0.015;

    // Compute normal from height field via central differences
    let eps = 0.5;
    let h_center = dent_h + grain + scratch1 + scratch2;
    let h_right  = smooth_noise((local + vec2f(eps, 0.0)) * 0.35 + vec2f(42.0, 17.0)) * 0.09
                 + smooth_noise(vec2f((grain_p.x + eps) * 6.0, grain_p.y * 0.8 + 200.0)) * 0.06;
    let h_up     = smooth_noise((local + vec2f(0.0, eps)) * 0.35 + vec2f(42.0, 17.0)) * 0.09
                 + smooth_noise(vec2f(grain_p.x * 6.0, (grain_p.y + eps) * 0.8 + 200.0)) * 0.06;

    let normal = normalize(vec3f(
        dome.x + (h_center - h_right) * 3.0,
        dome.y + (h_center - h_up) * 3.0,
        1.0
    ));

    // ── Material: dark forged iron with rust and patina ──
    // Base: dark gunmetal
    let base_iron = vec3f(0.32, 0.30, 0.28);

    // Voronoi crystalline grain structure (like real metal microstructure)
    let vor = voronoi(local * 0.8 + vec2f(73.0, 11.0));
    let crystal_tint = mix(vec3f(0.30, 0.28, 0.26), vec3f(0.36, 0.34, 0.30), vor.y);

    // Rust patches — warm orange-brown, mostly in concavities
    let rust_mask = smoothstep(0.35, 0.65, fbm5(local * 0.3 + vec2f(15.0, 27.0)));
    let rust_detail = fbm5(local * 0.9 + vec2f(50.0, 80.0));
    let rust_col = mix(vec3f(0.35, 0.18, 0.08), vec3f(0.50, 0.25, 0.10), rust_detail);
    let rust_amount = rust_mask * smoothstep(0.3, 0.7, dent_h / 0.15) * 0.45;

    // Blue-black patina in protected areas
    let patina_mask = smoothstep(0.6, 0.4, fbm5(local * 0.25 + vec2f(200.0, 150.0)));
    let patina_col = vec3f(0.15, 0.18, 0.25);
    let patina_amount = patina_mask * 0.3 * (1.0 - rust_amount * 2.0);

    // Combine base material
    var metal_col = mix(crystal_tint, base_iron, grain * 8.0);
    metal_col = mix(metal_col, rust_col, rust_amount);
    metal_col = mix(metal_col, patina_col, patina_amount);

    // Brushed highlight streaks
    let brush_streak = pow(smooth_noise(vec2f(grain_p.x * 15.0, grain_p.y * 0.4 + 400.0)), 3.0) * 0.12;
    metal_col = metal_col + vec3f(brush_streak);

    // ── Lighting: PBR-ish with two lights ──
    let view = vec3f(0.0, 0.0, 1.0);

    // Key light: warm upper-left
    let light1 = normalize(vec3f(-0.5, -0.8, 1.0));
    let diff1 = max(dot(normal, light1), 0.0);
    let half1 = normalize(light1 + view);
    // Roughness varies: rust is rough, polished metal is sharp
    let roughness = mix(0.3, 0.9, rust_amount + patina_amount * 0.5);
    let spec_power = mix(80.0, 8.0, roughness);
    let spec1 = pow(max(dot(normal, half1), 0.0), spec_power) * mix(1.2, 0.15, roughness);

    // Fill light: cool from lower-right
    let light2 = normalize(vec3f(0.6, 0.3, 0.8));
    let diff2 = max(dot(normal, light2), 0.0) * 0.3;
    let half2 = normalize(light2 + view);
    let spec2 = pow(max(dot(normal, half2), 0.0), spec_power * 0.5) * mix(0.4, 0.05, roughness);

    // Ambient occlusion from SDF (edges darker)
    let ao = smoothstep(0.0, 5.0, -sdf) * 0.3 + 0.7;

    // Fresnel rim
    let fresnel = pow(1.0 - max(dot(normal, view), 0.0), 4.0);
    let rim_col = vec3f(0.4, 0.42, 0.5) * fresnel * 0.25;

    let ambient = vec3f(0.06, 0.055, 0.05);
    var col = metal_col * (ambient + (diff1 * vec3f(1.0, 0.95, 0.85) + diff2 * vec3f(0.7, 0.8, 1.0)) * ao)
            + vec3f(spec1) * vec3f(1.0, 0.95, 0.88) * (1.0 - rust_amount)
            + vec3f(spec2) * vec3f(0.7, 0.8, 1.0) * (1.0 - rust_amount)
            + rim_col;

    // Subtle heat shimmer near the tip (this IS a cinder theme)
    let tip_glow = exp(-length(local) * 0.15) * 0.08;
    col = col + vec3f(tip_glow * 0.8, tip_glow * 0.3, tip_glow * 0.05);

    // ── Dark forged border (bevelled edge) ──
    let bevel = smoothstep(0.5, -1.5, sdf);
    let bevel_light = max(dot(normalize(vec3f(-sign(sdf) * 0.5, -sign(sdf) * 0.3, 1.0)), light1), 0.0);
    col = mix(vec3f(0.05, 0.04, 0.03), col, bevel);
    col = col + vec3f(bevel_light * 0.1) * (1.0 - bevel);

    // ── Drop shadow ──
    let shadow_sdf = cursor_arrow_sdf(local - vec2f(2.0, 2.5));
    let shadow = (1.0 - smoothstep(-3.0, 2.0, shadow_sdf)) * 0.45;

    let shadow_result = vec4f(0.0, 0.0, 0.0, shadow);
    let cursor_result = vec4f(col, aa);
    let out_a = cursor_result.a + shadow_result.a * (1.0 - cursor_result.a);
    let out_rgb = (cursor_result.rgb * cursor_result.a + shadow_result.rgb * shadow_result.a * (1.0 - cursor_result.a)) / max(out_a, 0.001);
    return vec4f(out_rgb, out_a);
}

// ---- Glass cursor — crystal with internal fractures, caustics, dispersion ---

fn cursor_glass(px: vec2f, mouse: vec2f, t: f32) -> vec4f {
    let local = px - mouse;

    if (local.x < -6.0 || local.x > 24.0 || local.y < -6.0 || local.y > 34.0) {
        return vec4f(0.0);
    }

    let sdf = cursor_arrow_sdf(local);

    let aa = 1.0 - smoothstep(-1.2, 0.6, sdf);
    if (aa < 0.001) { return vec4f(0.0); }

    let uv = local / vec2f(16.0, 24.0);

    // ── Glass surface normals: thick convex lens ──
    let dome_strength = 0.7;
    let dome_x = (uv.x - 0.4) * dome_strength;
    let dome_y = (uv.y - 0.45) * dome_strength * 0.7;

    // Wavy imperfections in glass surface (hand-blown look)
    let wave1 = smooth_noise(local * 0.6 + vec2f(t * 0.08, 7.0)) * 0.12;
    let wave2 = smooth_noise(local * 1.3 + vec2f(13.0, t * 0.06)) * 0.06;
    let wave3 = smooth_noise(local * 2.5 + vec2f(t * 0.04, 22.0)) * 0.03;

    let eps = 0.4;
    let h_c = wave1 + wave2 + wave3;
    let h_r = smooth_noise((local + vec2f(eps, 0.0)) * 0.6 + vec2f(t * 0.08, 7.0)) * 0.12
            + smooth_noise((local + vec2f(eps, 0.0)) * 1.3 + vec2f(13.0, t * 0.06)) * 0.06;
    let h_u = smooth_noise((local + vec2f(0.0, eps)) * 0.6 + vec2f(t * 0.08, 7.0)) * 0.12
            + smooth_noise((local + vec2f(0.0, eps)) * 1.3 + vec2f(13.0, t * 0.06)) * 0.06;

    let normal = normalize(vec3f(
        dome_x + (h_c - h_r) * 2.5,
        dome_y + (h_c - h_u) * 2.5,
        1.0
    ));

    // ── Internal structure: fractures, bubbles, inclusions ──
    // Voronoi fracture pattern (like cracked ice / crystal structure)
    let fracture_vor = voronoi(local * 0.5 + vec2f(100.0, 200.0));
    let fracture_lines = smoothstep(0.12, 0.08, fracture_vor.x) * 0.3;
    let fracture_tint = hash2(vec2f(fracture_vor.y * 100.0, 33.0));

    // Air bubbles (small bright spots scattered inside)
    let bubble_vor = voronoi(local * 1.5 + vec2f(300.0, 400.0));
    let bubbles = smoothstep(0.08, 0.04, bubble_vor.x) * 0.5;

    // Deep internal caustic pattern (light bending inside the glass)
    let internal_caustic = fbm5(local * 0.15 + normal.xy * 3.0 + vec2f(t * 0.12, -t * 0.08));

    // ── Refraction: chromatic aberration (R/G/B refract differently) ──
    let refract_base = 10.0;
    let r_offset = normal.xy * (refract_base * 1.05);
    let g_offset = normal.xy * (refract_base * 1.00);
    let b_offset = normal.xy * (refract_base * 0.95);

    // Sample "background" at three offset positions (simulated via noise)
    let bg_scale = 0.008;
    let bg_r = smooth_noise((px + r_offset) * bg_scale + vec2f(t * 0.03, 0.0)) * 0.12 + 0.04;
    let bg_g = smooth_noise((px + g_offset) * bg_scale + vec2f(0.0, t * 0.03)) * 0.13 + 0.045;
    let bg_b = smooth_noise((px + b_offset) * bg_scale + vec2f(t * 0.02, t * 0.02)) * 0.14 + 0.05;
    var refracted = vec3f(bg_r, bg_g, bg_b);

    // Tint by glass body colour (very slight blue-green)
    let glass_tint = vec3f(0.85, 0.92, 0.95);
    refracted = refracted * glass_tint;

    // Add internal structures
    refracted = refracted + vec3f(fracture_lines * 0.7, fracture_lines * 0.8, fracture_lines);
    refracted = refracted + vec3f(bubbles * 0.8, bubbles * 0.9, bubbles);
    refracted = refracted + vec3f(internal_caustic * 0.04, internal_caustic * 0.05, internal_caustic * 0.06);

    // ── Fresnel: Schlick's approximation ──
    let view = vec3f(0.0, 0.0, 1.0);
    let n_dot_v = max(dot(normal, view), 0.0);
    let f0 = 0.04;  // glass IOR ~1.5
    let fresnel = f0 + (1.0 - f0) * pow(1.0 - n_dot_v, 5.0);

    // ── Reflection: environment approximation (multi-layer) ──
    let refl_dir = reflect(-view, normal);
    let refl_uv1 = refl_dir.xy * 5.0 + vec2f(t * 0.06, -t * 0.04);
    let refl_uv2 = refl_dir.xy * 12.0 + vec2f(-t * 0.03, t * 0.07);
    let refl1 = smooth_noise(refl_uv1) * 0.25 + 0.08;
    let refl2 = smooth_noise(refl_uv2) * 0.1;
    let reflection = vec3f(refl1 + refl2) * vec3f(0.9, 0.95, 1.0);

    // ── Specular highlights: two lights ──
    // Key: sharp point light upper-left
    let light1 = normalize(vec3f(-0.4, -0.7, 1.0));
    let half1 = normalize(light1 + view);
    let spec1 = pow(max(dot(normal, half1), 0.0), 128.0) * 1.5;

    // Fill: softer warm light from right
    let light2 = normalize(vec3f(0.7, -0.2, 0.9));
    let half2 = normalize(light2 + view);
    let spec2 = pow(max(dot(normal, half2), 0.0), 64.0) * 0.4;

    // ── Edge caustics: rainbow dispersion along borders ──
    let edge_d = abs(sdf);
    let edge_bright = smoothstep(3.5, 0.0, edge_d);

    // Travelling rainbow wave along the perimeter
    let perim_t = atan2(local.y - 12.0, local.x - 6.0); // angle around center
    let rainbow_phase = perim_t * 2.0 + t * 0.8 + sdf * 0.5;
    let caustic_r = sin(rainbow_phase) * 0.5 + 0.5;
    let caustic_g = sin(rainbow_phase + 2.094) * 0.5 + 0.5;
    let caustic_b = sin(rainbow_phase + 4.189) * 0.5 + 0.5;
    let caustic = vec3f(caustic_r, caustic_g, caustic_b) * edge_bright * 0.4;

    // Secondary: internal total-internal-reflection caustic bands
    let tir_bands = pow(sin(sdf * 1.2 + t * 0.3) * 0.5 + 0.5, 4.0) * edge_bright * 0.2;

    // ── Compose ──
    var col = mix(refracted, reflection, fresnel)
            + vec3f(spec1) * vec3f(1.0, 0.98, 0.95)
            + vec3f(spec2) * vec3f(1.0, 0.95, 0.85)
            + caustic
            + vec3f(tir_bands * 0.5, tir_bands * 0.7, tir_bands);

    // Glass alpha: mostly transparent body, opaque at edges (Fresnel)
    let body_alpha = 0.18 + fresnel * 0.55;

    // Bright crisp edge highlight (like polished glass bevels catching light)
    let edge_highlight = smoothstep(1.2, 0.0, edge_d) * 0.7;
    let edge_shadow_inner = smoothstep(0.0, 2.5, edge_d) * smoothstep(4.0, 2.5, edge_d) * 0.15;
    col = col + vec3f(edge_highlight * 0.9, edge_highlight * 0.95, edge_highlight);
    col = col - vec3f(edge_shadow_inner * 0.3);

    // ── Drop shadow (soft, slightly coloured by caustics) ──
    let shadow_sdf = cursor_arrow_sdf(local - vec2f(2.0, 3.0));
    let shadow_base = (1.0 - smoothstep(-4.0, 3.0, shadow_sdf)) * 0.25;
    // Caustic light leaking into shadow
    let shadow_caustic_phase = shadow_sdf * 0.6 + t * 0.4;
    let sc_r = sin(shadow_caustic_phase) * 0.5 + 0.5;
    let sc_g = sin(shadow_caustic_phase + 2.094) * 0.5 + 0.5;
    let sc_b = sin(shadow_caustic_phase + 4.189) * 0.5 + 0.5;
    let shadow_caustic_bright = smoothstep(3.0, 0.0, abs(shadow_sdf + 1.0)) * 0.12;
    let shadow_col_rgb = vec3f(sc_r, sc_g, sc_b) * shadow_caustic_bright;

    let cursor_a = clamp(aa * (body_alpha + edge_highlight * 0.3), 0.0, 1.0);
    let shadow_result = vec4f(shadow_col_rgb, shadow_base);
    let cursor_result = vec4f(col, cursor_a);
    let out_a = cursor_result.a + shadow_result.a * (1.0 - cursor_result.a);
    let out_rgb = (cursor_result.rgb * cursor_result.a + shadow_result.rgb * shadow_result.a * (1.0 - cursor_result.a)) / max(out_a, 0.001);
    return vec4f(out_rgb, out_a);
}

// Dispatch cursor rendering based on style uniform
fn gpu_cursor(px: vec2f, mouse: vec2f, style: f32, t: f32) -> vec4f {
    if (style < 0.5) { return vec4f(0.0); }       // 0 = default (no GPU cursor)
    if (style < 1.5) { return cursor_metal(px, mouse, t); } // 1 = metal
    return cursor_glass(px, mouse, t);                       // 2 = glass
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
    let cs = max(u.cinder_size, 0.01);
    let margin = 4.0 * cs;
    let inside_x = px.x >= ex - margin && px.x < ex + ew + margin;
    let inside_y = px.y >= ey - margin && px.y < ey + eh + margin;
    if !(inside_x && inside_y) { return vec4f(0.0); }

    if !(inside_x && inside_y) { return vec4f(0.0); }

    let dist = edge_dist(px, ex, ey, ew, eh);
    let pt   = perimeter_t(px, ex, ey, ew, eh);

    if dist > 7.0 * cs { return vec4f(0.0); }

    // Crackling ember wave — irregular, like smouldering cracks
    let crack_n = smooth_noise(vec2f(pt * 40.0, t * 1.5));
    let crack   = pow(crack_n, 3.0);

    // Slow pulsing heat
    let pulse = 0.6 + 0.4 * sin(t * 1.5 + pt * 6.28);

    // Ember colour: deep orange → dull red, with vine-green flickers
    let ember_core = palette.cinder_ember.rgb;
    let ember_edge = palette.cinder_gold.rgb;
    let vine_flick = palette.cinder_vine.rgb;
    var ember_rgb = mix(ember_edge, ember_core, crack * pulse);
    let vine_f = smoothstep(0.7, 0.9, smooth_noise(vec2f(pt * 20.0 + 5.0, t * 0.5)));
    ember_rgb = mix(ember_rgb, vine_flick, vine_f * 0.4);

    // Border glow profile
    let glow = smoothstep(0.0, 0.5 * cs, dist) * smoothstep(6.0 * cs, 1.0 * cs, dist);

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

    // --- Configurable smoke parameters from uniforms --------------------------
    let s_intensity  = u.smoke_intensity;       // 0.0–1.0
    let s_speed      = u.smoke_speed;           // 0.0–5.0
    let s_warm_scale = u.smoke_warm_scale;      // 0.0–2.0 (warm layers 1+4)
    let s_cool_scale = u.smoke_cool_scale;      // 0.0–2.0 (cool layer 2)
    let s_fine_scale = u.smoke_fine_scale;      // 0.0–2.0 (fine layer 3)
    let s_grain_i    = u.grain_intensity;       // 0.0–1.0 brightness
    let s_grain_c    = mix(0.5, 2.0, u.grain_coarseness);  // freq scale 0.5–2.0
    let s_grain_sz   = mix(1.0, 8.0, u.grain_size);         // pixel block 1–8
    let s_vignette   = u.vignette_str;          // 0.0–1.0
    let s_underglow  = u.underglow_str;         // 0.0–1.0

    // Speed is baked into the base time so ALL time-dependent effects scale uniformly
    let t = u.time * 0.35 * s_speed;

    // --- Smoky dark background with varied palette and vignette ---------------
    // Downsample coordinates for a chunky/pixelated feel
    let pixel_size = 4.0;
    let ds_px = floor(raw_px / pixel_size) * pixel_size;
    let ds_uv = ds_px / vec2f(u.width, u.height);

    // Vignette — darken edges, bright centre (scaled by vignette_str)
    let vig_d = length((raw_uv - 0.5) * vec2f(1.2, 1.0));
    let vignette = 1.0 - smoothstep(0.3, 1.1, vig_d) * 0.55 * s_vignette;

    // Grain pixel-block downsampling (s_grain_sz controls block size)
    let grain_px = floor(ds_px / s_grain_sz) * s_grain_sz;

    // Animated coarse noise — drifting (speed already baked into t)
    let drift = vec2f(t * 0.12, t * 0.06);
    var grain_sum = 0.0;
    if (s_grain_i > 0.0) {
        let n_fine   = smooth_noise((grain_px + drift * 40.0) * 0.025 * s_grain_c) * 0.028 * s_grain_i;
        let n_coarse = smooth_noise((grain_px + drift * 20.0) * 0.006 * s_grain_c + vec2f(7.3, 2.1)) * 0.018 * s_grain_i;
        let n_grain  = hash2(grain_px * 0.37 * s_grain_c + vec2f(floor(u.time * 8.0 * s_speed))) * 0.015 * s_grain_i;
        grain_sum = n_fine + n_coarse + n_grain;
    }

    // Varied base palette — subtle colour variation across the screen
    let palette_t = smooth_noise(ds_px * 0.003 + drift * 5.0);
    let cool_tone = palette.smoke_cool.rgb;
    let warm_tone = palette.smoke_warm.rgb;
    let mid_tone  = palette.smoke_moss.rgb;
    var base_col = mix(cool_tone, warm_tone, smoothstep(0.3, 0.7, palette_t));
    base_col = mix(base_col, mid_tone, smoothstep(0.5, 0.8, smooth_noise(ds_px * 0.005 + vec2f(3.0, -t * 0.02))) * 0.5);
    var bg = base_col + vec3f(grain_sum);

    // --- Layered animated smoke wisps (skip fbm when smoke disabled) --------
    if (s_intensity > 0.0) {
        // Per-layer UV scales control the visible "size" of each color group.
        // Speed is already embedded in t.
        // Layer 1: large slow rolling smoke (warm)
        let smoke1_uv = ds_uv * (2.0 * s_warm_scale) + vec2f(t * 0.06, t * 0.025);
        let smoke1 = fbm(smoke1_uv) * 0.05 * s_intensity;

        // Layer 2: medium tendrils drifting opposite direction (cool)
        let smoke2_uv = ds_uv * (4.0 * s_cool_scale) + vec2f(-t * 0.09, t * 0.05);
        let smoke2 = fbm(smoke2_uv) * 0.03 * s_intensity;

        // Layer 3: fine fast wisps — curling upward (warm fine)
        let smoke3_uv = ds_uv * (7.0 * s_fine_scale) + vec2f(sin(t * 0.3) * 0.5, -t * 0.12);
        let smoke3 = fbm(smoke3_uv) * 0.018 * s_intensity;

        // Layer 4: very slow deep background churn (warm)
        let smoke4_uv = ds_uv * (1.2 * s_warm_scale) + vec2f(t * 0.015, -t * 0.01);
        let smoke4 = fbm(smoke4_uv) * 0.035 * s_intensity;

        // Composite smoke with slight colour tinting per layer
        bg = bg + vec3f(smoke1 + smoke4) * vec3f(0.85, 0.80, 0.75);  // warm base smoke
        bg = bg + vec3f(smoke2) * vec3f(0.6, 0.7, 0.85);              // cool mid wisps
        bg = bg + vec3f(smoke3) * vec3f(0.9, 0.85, 0.7);              // warm fine wisps
    }

    // Faint animated grain shimmer (skip noise when grain disabled)
    if (s_grain_i > 0.0) {
        let grain_hi = smooth_noise((grain_px + drift * 60.0) * 0.12 * s_grain_c) * 0.012 * s_grain_i;
        bg = bg + vec3f(grain_hi * 0.6, grain_hi * 0.55, grain_hi * 0.5);
    }

    // Dim warm underglow from bottom edge (skip when off)
    if (s_underglow > 0.001) {
        let underglow = smoothstep(1.0, 0.4, raw_uv.y) * 0.015 * s_underglow;
        bg = bg + vec3f(0.5, 0.18, 0.05) * underglow;
    }

    // Apply vignette
    bg = bg * vignette;

    // --- Scene elements ------------------------------------------------------
    let scene = sample_scene(raw_px);
    var color = bg * (1.0 - scene.a) + scene.rgb;

    // --- Atmospheric CRT effects (independently controlled) ------------------
    let sh_i = u.crt_scanlines_h;  // horizontal scanlines
    let sv_i = u.crt_scanlines_v;  // vertical scanlines
    let es_i = u.crt_edge_shadow;  // edge/border shadow
    let fl_i = u.crt_flicker;      // torch flicker

    let any_crt = max(max(sh_i, sv_i), max(es_i, fl_i));
    if (any_crt > 0.001) {
        // Horizontal scanlines + horizontal component of pixel grid
        let scanline = mix(1.0, crt_scanlines(raw_px.y), sh_i);
        // Vertical scanlines + vertical component of pixel grid
        let vline    = mix(1.0, crt_vertical_lines(raw_px.x), sv_i);
        // Pixel grid: blend of both axes — only visible where both have intensity
        let grid_i = min(sh_i, sv_i);
        let pgrid  = mix(1.0, crt_pixel_grid(raw_px), grid_i);

        let edge     = mix(1.0, crt_edge_shadow(raw_uv), es_i);
        let torch_flicker = 1.0 - fl_i * (0.03 - 0.03 * sin(t * 3.0 + raw_uv.x * 2.0));

        color = color * scanline * vline * pgrid * edge * torch_flicker;
    }

    // --- Custom GPU cursor (after CRT, drawn last) --------------------------
    let cursor_col = gpu_cursor(raw_px, vec2f(u.mouse_x, u.mouse_y), u.cursor_style, u.time);
    color = mix(color, cursor_col.rgb, cursor_col.a);

    return vec4f(clamp(color, vec3f(0.0), vec3f(1.0)), 1.0);
}
