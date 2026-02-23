// compute.wgsl — multi-effect particle physics simulation
//
// Concatenated after: types.wgsl + noise.wgsl
// Four particle types, partitioned by index:
//   [0, SPARK_END)               — metal sparks (at mouse position while hovering)
//   [SPARK_END, EMBER_END)       — flying embers / ash (continuous rise from hovered)
//   [EMBER_END, RAY_END)         — angelic beams (pixel-thin vertical from hovered)
//   [RAY_END, GLITTER_END)       — angelic glitter (around selected element)

// ---- bindings (compute pass) ------------------------------------------------

@group(0) @binding(0) var<uniform>             u         : Uniforms;
@group(0) @binding(1) var<storage, read>       elems     : array<ElemRect>;
@group(0) @binding(2) var<storage, read_write> particles : array<Particle>;

// ---- constants --------------------------------------------------------------

const BURST_WINDOW : f32 = 0.25;   // seconds — initial intense burst

// ---- helpers ----------------------------------------------------------------

fn spawn_on_perimeter(elem_idx: u32, seed: u32) -> vec2f {
    let e  = elems[elem_idx];
    let ex = e.rect.x;
    let ey = e.rect.y;
    let ew = e.rect.z;
    let eh = e.rect.w;
    let perim = 2.0 * (ew + eh);
    let t = rand_f(seed) * perim;

    if t < ew        { return vec2f(ex + t, ey); }
    if t < ew + eh   { return vec2f(ex + ew, ey + (t - ew)); }
    if t < 2.0*ew+eh { return vec2f(ex + ew - (t - ew - eh), ey + eh); }
    return vec2f(ex, ey + eh - (t - 2.0 * ew - eh));
}

fn outward_normal(pos: vec2f, elem_idx: u32) -> vec2f {
    let e  = elems[elem_idx];
    let cx = e.rect.x + e.rect.z * 0.5;
    let cy = e.rect.y + e.rect.w * 0.5;
    return normalize(pos - vec2f(cx, cy) + vec2f(0.001, 0.001));
}

fn park_dead(idx: u32) {
    var p = particles[idx];
    p.pos  = vec2f(-9999.0);
    p.vel  = vec2f(0.0);
    p.life = 0.0;
    p.size = 0.0;
    particles[idx] = p;
}

// Returns the kind of the currently hovered element, or -1.0 if none.
fn hovered_elem_kind() -> f32 {
    let hi = i32(u.hover_elem);
    if hi < 0 || hi >= i32(u.element_count) { return -1.0; }
    return elems[u32(hi)].kind;
}

// Returns true if this hover-based effect should spawn on the hovered element.
// Regular elements (kind < 8) allow all effects.
// Preview containers (kind 8-11) only allow their matching effect.
fn hover_allows(fx_kind: f32) -> bool {
    let hk = hovered_elem_kind();
    if hk < 0.0 { return false; }       // nothing hovered
    if hk < 7.5 { return true; }        // regular element — all effects
    return abs(hk - fx_kind) < 0.5;     // preview — must match
}

// ---- metal spark physics (at mouse cursor, continuous while hovering) --------

fn update_metal_spark(idx: u32) {
    // Speed == 0 means sparks are disabled — buffer already zeroed by CPU
    if u.spark_speed <= 0.0 { return; }

    var p  = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);
    let spd = u.spark_speed;

    // Respect spark count limit — park excess sparks
    let spark_frac = select(1.0, u.spark_count, u.spark_count > 0.0);
    let max_sparks = u32(f32(SPARK_END) * clamp(spark_frac, 0.0, 2.0));
    if idx >= max_sparks {
        park_dead(idx);
        return;
    }

    p.life -= dt * spd;

    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count)
           || !hover_allows(KIND_FX_SPARK) {
            park_dead(idx);
            return;
        }

        let seed = idx * 7919u + u32(u.time * 5000.0);

        // Spawn at mouse cursor with random radial offset
        let base_angle = rand_f(seed) * 6.2832;
        let scatter_r  = 5.0 + rand_f(seed + 1u) * 35.0;
        let scatter = vec2f(cos(base_angle), sin(base_angle)) * scatter_r;
        p.pos = vec2f(u.mouse_x, u.mouse_y) + scatter;

        // Velocity points outward from cursor, with ±25° angular spread
        let spread = (rand_f(seed + 2u) - 0.5) * 0.87;  // ±25°
        let ca = cos(base_angle + spread);
        let sa = sin(base_angle + spread);
        let since_hover = u.time - u.hover_start_time;
        let burst_mult = select(0.5, 1.2, since_hover < BURST_WINDOW);
        let speed = (40.0 + rand_f(seed + 3u) * 100.0) * burst_mult * spd;
        p.vel = vec2f(ca, sa) * speed;

        p.max_life = 0.5 + rand_f(seed + 5u) * 0.8;
        p.life     = p.max_life;
        p.hue      = rand_f(seed + 6u) * 0.12;
        p.size     = (1.0 + rand_f(seed + 7u) * 2.0) * max(u.spark_size, 0.01);
        p.kind     = PK_METAL_SPARK;
        p.spawn_t  = u.time;
    } else {
        // Moderate drag — particles trail behind with gravity
        p.vel = p.vel * (1.0 - 2.0 * dt * spd);
        p.vel.y = p.vel.y + 80.0 * dt * spd;
        p.pos = p.pos + p.vel * dt * spd;
    }

    particles[idx] = p;
}

// ---- ember / ash physics (continuous rising embers) -------------------------

fn update_ember(idx: u32) {
    // Speed == 0 means embers are disabled — buffer already zeroed by CPU
    if u.ember_speed <= 0.0 { return; }

    var p  = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);
    let spd = u.ember_speed;

    // Respect ember count limit
    let ember_frac = select(1.0, u.ember_count, u.ember_count > 0.0);
    let max_embers = u32(f32(EMBER_END - SPARK_END) * clamp(ember_frac, 0.0, 2.0));
    if (idx - SPARK_END) >= max_embers {
        park_dead(idx);
        return;
    }

    p.life -= dt * spd;

    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count)
           || !hover_allows(KIND_FX_EMBER) {
            park_dead(idx);
            return;
        }

        let ei   = u32(hover_idx);
        let seed = idx * 7919u + u32(u.time * 1000.0);

        p.pos = spawn_on_perimeter(ei, seed);

        let normal = outward_normal(p.pos, ei);
        let speed  = 10.0 + rand_f(seed + 3u) * 25.0;
        p.vel = (normal * speed * 0.5 + vec2f(0.0, -20.0 - rand_f(seed + 4u) * 15.0)) * spd;

        p.max_life = 1.0 + rand_f(seed + 5u) * 1.5;
        p.life     = p.max_life;

        let r = rand_f(seed + 6u);
        if r < 0.80 {
            p.hue = rand_f(seed + 8u) * 0.12;
        } else {
            p.hue = 0.25 + rand_f(seed + 8u) * 0.15;
        }

        p.size    = (0.4 + rand_f(seed + 7u) * 1.0) * max(u.ember_size, 0.01);
        p.kind    = PK_EMBER;
        p.spawn_t = u.time;
    } else {
        let drift = sin(u.time * 2.0 + f32(idx) * 0.3) * 8.0;
        p.vel = p.vel * (1.0 - 1.5 * dt * spd) + vec2f(drift * dt * spd, -25.0 * dt * spd);
        p.pos = p.pos + p.vel * dt * spd;
    }

    particles[idx] = p;
}

// ---- angelic beam physics (pixel-thin vertical rays from selected/opened) ---

fn update_god_ray(idx: u32) {
    // Speed == 0 means beams are disabled — buffer already zeroed by CPU
    if u.beam_speed <= 0.0 { return; }

    var p  = particles[idx];
    let dt = u.delta_time;
    let spd = u.beam_speed;

    // Beam source: normally selected_elem, or hovered beam-preview container
    var beam_src = i32(u.selected_elem);
    let hover_idx = i32(u.hover_elem);
    if hover_idx >= 0 && hover_idx < i32(u.element_count) {
        let hk = elems[u32(hover_idx)].kind;
        if abs(hk - KIND_FX_BEAM) < 0.5 {
            beam_src = hover_idx;
        }
    }

    // Respect beam count limit — park excess beams
    let max_beams = u32(u.beam_count);
    if max_beams > 0u && (idx - EMBER_END) >= max_beams {
        park_dead(idx);
        return;
    }

    p.life -= dt * spd;

    if p.life <= 0.0 {
        if beam_src < 0 || beam_src >= i32(u.element_count) {
            park_dead(idx);
            return;
        }

        let ei   = u32(beam_src);
        let seed = idx * 7919u + u32(u.time * 800.0);

        // Spawn on the element perimeter (beams rise upward from any border)
        p.pos = spawn_on_perimeter(ei, seed);

        // Rise upward only — drift distance scaled by beam_drift setting
        let drift_scale = select(1.0, u.beam_drift, u.beam_drift > 0.0);
        p.vel = vec2f(
            (rand_f(seed + 2u) - 0.5) * 2.0,
            (-12.0 - rand_f(seed + 3u) * 10.0) * drift_scale,
        ) * spd;

        p.max_life = 2.0 + rand_f(seed + 4u) * 2.0;
        p.life     = p.max_life;
        p.hue      = 0.08 + rand_f(seed + 5u) * 0.06;
        p.size     = 0.6 + rand_f(seed + 6u) * 1.0;   // wider beam, still thin
        p.kind     = PK_GOD_RAY;
        p.spawn_t  = u.time;
    } else {
        let sway = sin(u.time * 1.5 + f32(idx) * 0.7) * 1.5;
        p.vel.x = p.vel.x * (1.0 - 0.5 * dt * spd) + sway * dt * spd;
        p.vel.y = p.vel.y * (1.0 - 0.2 * dt * spd);
        p.pos = p.pos + p.vel * dt * spd;
    }

    particles[idx] = p;
}

// ---- angelic glitter physics (around hovered element border) ----------------

fn update_glitter(idx: u32) {
    // Speed == 0 means glitter is disabled — buffer already zeroed by CPU
    if u.glitter_speed <= 0.0 { return; }

    var p  = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);
    let spd = u.glitter_speed;

    // Respect glitter count limit
    let glitter_frac = select(1.0, u.glitter_count, u.glitter_count > 0.0);
    let max_glitter = u32(f32(GLITTER_END - RAY_END) * clamp(glitter_frac, 0.0, 2.0));
    if (idx - RAY_END) >= max_glitter {
        park_dead(idx);
        return;
    }

    p.life -= dt * spd;

    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count)
           || !hover_allows(KIND_FX_GLITTER) {
            park_dead(idx);
            return;
        }

        let ei   = u32(hover_idx);
        let seed = idx * 7919u + u32(u.time * 1200.0);

        // Spawn on the hovered element perimeter
        p.pos = spawn_on_perimeter(ei, seed);

        // Mostly tangential drift along border with tiny outward float
        let norm    = outward_normal(p.pos, ei);
        let tangent = vec2f(-norm.y, norm.x);
        let tang_dir = select(-1.0, 1.0, rand_f(seed + 2u) > 0.5);
        p.vel = (tangent * tang_dir * (4.0 + rand_f(seed + 3u) * 10.0)
              + norm * (0.5 + rand_f(seed + 4u) * 2.5)) * spd;

        p.max_life = 0.8 + rand_f(seed + 5u) * 1.5;
        p.life     = p.max_life;
        p.hue      = rand_f(seed + 6u);
        p.size     = (0.6 + rand_f(seed + 7u) * 1.2) * max(u.glitter_size, 0.01);   // slightly larger — 0.6-1.8 px
        p.kind     = PK_GLITTER;
        p.spawn_t  = u.time;
    } else {
        // Slow drift with sparkle-like sway, stays near border
        let sway = sin(u.time * 4.0 + f32(idx) * 1.3) * 4.0;
        p.vel = p.vel * (1.0 - 3.0 * dt * spd) + vec2f(sway * dt * spd, -1.5 * dt * spd);
        p.pos = p.pos + p.vel * dt * spd;
    }

    particles[idx] = p;
}

// ---- compute entry point ----------------------------------------------------

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) gid: vec3u) {
    let idx   = gid.x;
    let total = arrayLength(&particles);
    if idx >= total { return; }

    // Shift live particles by scroll delta so they track world-space positions.
    // The scroll delta is computed on the CPU as the negated change in
    // scrollTop/scrollLeft of the scrollable container, matching the direction
    // elements move on screen when the user scrolls.
    let sd = vec2f(u.scroll_dx, u.scroll_dy);
    if sd.x != 0.0 || sd.y != 0.0 {
        var p = particles[idx];
        if p.life > 0.0 {
            p.pos = p.pos + sd;
            particles[idx] = p;
        }
    }

    if idx < SPARK_END {
        update_metal_spark(idx);
    } else if idx < EMBER_END {
        update_ember(idx);
    } else if idx < RAY_END {
        update_god_ray(idx);
    } else {
        update_glitter(idx);
    }
}
