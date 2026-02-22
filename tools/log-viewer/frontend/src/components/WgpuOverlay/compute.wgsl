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

// ---- metal spark physics (at mouse cursor, continuous while hovering) --------

fn update_metal_spark(idx: u32) {
    var p  = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);

    p.life -= dt;

    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count) {
            park_dead(idx);
            return;
        }

        let seed = idx * 7919u + u32(u.time * 5000.0);

        // Spawn at mouse cursor position with wider random scatter
        // (particles trail behind the cursor)
        let scatter = vec2f(
            (rand_f(seed) - 0.5) * 30.0,
            (rand_f(seed + 1u) - 0.5) * 30.0,
        );
        p.pos = vec2f(u.mouse_x, u.mouse_y) + scatter;

        // Gentle radial drift outward from mouse
        let angle = rand_f(seed + 2u) * 6.2832;
        let since_hover = u.time - u.hover_start_time;
        // Mild burst on impact, very light continuous trickle
        let burst_mult = select(0.2, 0.5, since_hover < BURST_WINDOW);
        let speed = (20.0 + rand_f(seed + 3u) * 60.0) * burst_mult;
        p.vel = vec2f(cos(angle), sin(angle)) * speed;

        p.max_life = 0.3 + rand_f(seed + 5u) * 0.6;
        p.life     = p.max_life;
        p.hue      = rand_f(seed + 6u) * 0.12;
        p.size     = 0.4 + rand_f(seed + 7u) * 1.0;
        p.kind     = PK_METAL_SPARK;
        p.spawn_t  = u.time;
    } else {
        // Low drag — particles linger and trail behind
        p.vel = p.vel * (1.0 - 2.5 * dt);
        p.vel.y = p.vel.y + 60.0 * dt;
        p.pos = p.pos + p.vel * dt;
    }

    particles[idx] = p;
}

// ---- ember / ash physics (continuous rising embers) -------------------------

fn update_ember(idx: u32) {
    var p  = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);

    p.life -= dt;

    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count) {
            park_dead(idx);
            return;
        }

        let ei   = u32(hover_idx);
        let seed = idx * 7919u + u32(u.time * 1000.0);

        p.pos = spawn_on_perimeter(ei, seed);

        let normal = outward_normal(p.pos, ei);
        let speed  = 10.0 + rand_f(seed + 3u) * 25.0;
        p.vel = normal * speed * 0.5 + vec2f(0.0, -20.0 - rand_f(seed + 4u) * 15.0);

        p.max_life = 1.0 + rand_f(seed + 5u) * 1.5;
        p.life     = p.max_life;

        let r = rand_f(seed + 6u);
        if r < 0.80 {
            p.hue = rand_f(seed + 8u) * 0.12;
        } else {
            p.hue = 0.25 + rand_f(seed + 8u) * 0.15;
        }

        p.size    = 0.4 + rand_f(seed + 7u) * 1.0;
        p.kind    = PK_EMBER;
        p.spawn_t = u.time;
    } else {
        let drift = sin(u.time * 2.0 + f32(idx) * 0.3) * 8.0;
        p.vel = p.vel * (1.0 - 1.5 * dt) + vec2f(drift * dt, -25.0 * dt);
        p.pos = p.pos + p.vel * dt;
    }

    particles[idx] = p;
}

// ---- angelic beam physics (pixel-thin vertical rays from selected/opened) ---

fn update_god_ray(idx: u32) {
    var p  = particles[idx];
    let dt = u.delta_time;
    let sel_idx = i32(u.selected_elem);

    p.life -= dt;

    if p.life <= 0.0 {
        if sel_idx < 0 || sel_idx >= i32(u.element_count) {
            park_dead(idx);
            return;
        }

        let ei   = u32(sel_idx);
        let seed = idx * 7919u + u32(u.time * 800.0);

        // Spawn on element perimeter
        p.pos = spawn_on_perimeter(ei, seed);

        // Gentle upward rise
        p.vel = vec2f(
            (rand_f(seed + 2u) - 0.5) * 2.0,
            -10.0 - rand_f(seed + 3u) * 8.0,
        );

        p.max_life = 2.0 + rand_f(seed + 4u) * 2.0;
        p.life     = p.max_life;
        p.hue      = 0.08 + rand_f(seed + 5u) * 0.06;
        p.size     = 0.6 + rand_f(seed + 6u) * 1.0;   // wider beam, still thin
        p.kind     = PK_GOD_RAY;
        p.spawn_t  = u.time;
    } else {
        let sway = sin(u.time * 1.5 + f32(idx) * 0.7) * 1.5;
        p.vel.x = p.vel.x * (1.0 - 0.5 * dt) + sway * dt;
        p.vel.y = p.vel.y * (1.0 - 0.2 * dt);
        p.pos = p.pos + p.vel * dt;
    }

    particles[idx] = p;
}

// ---- angelic glitter physics (around hovered element border) ----------------

fn update_glitter(idx: u32) {
    var p  = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);

    p.life -= dt;

    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count) {
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
        p.vel = tangent * tang_dir * (4.0 + rand_f(seed + 3u) * 10.0)
              + norm * (0.5 + rand_f(seed + 4u) * 2.5);

        p.max_life = 0.8 + rand_f(seed + 5u) * 1.5;
        p.life     = p.max_life;
        p.hue      = rand_f(seed + 6u);
        p.size     = 0.6 + rand_f(seed + 7u) * 1.2;   // slightly larger — 0.6-1.8 px
        p.kind     = PK_GLITTER;
        p.spawn_t  = u.time;
    } else {
        // Slow drift with sparkle-like sway, stays near border
        let sway = sin(u.time * 4.0 + f32(idx) * 1.3) * 4.0;
        p.vel = p.vel * (1.0 - 3.0 * dt) + vec2f(sway * dt, -1.5 * dt);
        p.pos = p.pos + p.vel * dt;
    }

    particles[idx] = p;
}

// ---- compute entry point ----------------------------------------------------

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) gid: vec3u) {
    let idx   = gid.x;
    let total = arrayLength(&particles);
    if idx >= total { return; }

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
