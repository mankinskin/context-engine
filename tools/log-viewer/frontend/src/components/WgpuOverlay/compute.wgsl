// compute.wgsl — spark particle simulation
//
// Each particle has: position (xy), velocity (xy), life, max_life, hue, size
// The compute shader updates all particles in parallel each frame.
// Dead particles are respawned at the hovered element's border.

struct Uniforms {
    time          : f32,
    width         : f32,
    height        : f32,
    element_count : f32,
    mouse_x       : f32,
    mouse_y       : f32,
    delta_time    : f32,
    hover_elem    : f32,   // index of hovered element (-1 if none)
}

struct ElemRect {
    rect : vec4f,   // x, y, w, h
    hue  : f32,
    kind : f32,
    _p1  : f32,
    _p2  : f32,
}

// Per-particle state: [px, py, vx, vy, life, max_life, hue, size]
struct Particle {
    pos      : vec2f,
    vel      : vec2f,
    life     : f32,
    max_life : f32,
    hue      : f32,
    size     : f32,
}

@group(0) @binding(0) var<uniform>            u         : Uniforms;
@group(0) @binding(1) var<storage, read>      elems     : array<ElemRect>;
@group(0) @binding(2) var<storage, read_write> particles : array<Particle>;

// ---- pseudorandom ---------------------------------------------------------

fn pcg_hash(input: u32) -> u32 {
    var state = input * 747796405u + 2891336453u;
    let word  = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand_f(seed: u32) -> f32 {
    return f32(pcg_hash(seed)) / 4294967295.0;
}

fn rand2(seed: u32) -> vec2f {
    return vec2f(rand_f(seed), rand_f(seed + 1u));
}

// ---- spawn a particle on the perimeter of a rect --------------------------

fn spawn_on_perimeter(elem_idx: u32, seed: u32) -> vec2f {
    let e = elems[elem_idx];
    let ex = e.rect.x;
    let ey = e.rect.y;
    let ew = e.rect.z;
    let eh = e.rect.w;
    let perim = 2.0 * (ew + eh);
    let t = rand_f(seed) * perim;

    if t < ew {
        return vec2f(ex + t, ey);                // top edge
    } else if t < ew + eh {
        return vec2f(ex + ew, ey + (t - ew));    // right edge
    } else if t < 2.0 * ew + eh {
        return vec2f(ex + ew - (t - ew - eh), ey + eh); // bottom edge
    }
    return vec2f(ex, ey + eh - (t - 2.0 * ew - eh));    // left edge
}

fn outward_normal(pos: vec2f, elem_idx: u32) -> vec2f {
    let e = elems[elem_idx];
    let cx = e.rect.x + e.rect.z * 0.5;
    let cy = e.rect.y + e.rect.w * 0.5;
    return normalize(pos - vec2f(cx, cy) + vec2f(0.001, 0.001));
}

// ---- compute main ---------------------------------------------------------

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) gid: vec3u) {
    let idx = gid.x;
    let total = arrayLength(&particles);
    if idx >= total { return; }

    var p = particles[idx];
    let dt = u.delta_time;
    let hover_idx = i32(u.hover_elem);

    // --- age particle ------------------------------------------------------
    p.life -= dt;

    // --- respawn dead particles at hovered element -------------------------
    if p.life <= 0.0 {
        if hover_idx < 0 || hover_idx >= i32(u.element_count) {
            // No hovered element — park offscreen
            p.pos = vec2f(-9999.0, -9999.0);
            p.vel = vec2f(0.0);
            p.life = 0.0;
            p.size = 0.0;
            particles[idx] = p;
            return;
        }

        let ei = u32(hover_idx);
        let seed = idx * 7919u + u32(u.time * 1000.0);

        // Spawn on perimeter
        p.pos = spawn_on_perimeter(ei, seed);

        // Outward velocity + random tangent for sparkle spread
        let normal = outward_normal(p.pos, ei);
        let tangent = vec2f(-normal.y, normal.x);
        let speed = 30.0 + rand_f(seed + 3u) * 80.0;
        let tang_speed = (rand_f(seed + 4u) - 0.5) * 60.0;
        p.vel = normal * speed + tangent * tang_speed;

        // Lifetime
        p.max_life = 0.3 + rand_f(seed + 5u) * 0.7;
        p.life = p.max_life;

        // Colour: element hue + random variation
        let elem_hue = elems[ei].hue;
        p.hue = fract(elem_hue + (rand_f(seed + 6u) - 0.5) * 0.2);

        // Size
        p.size = 1.0 + rand_f(seed + 7u) * 2.5;
    } else {
        // --- integrate motion ------------------------------------------------
        // Slight gravity downward + drag
        p.vel = p.vel * (1.0 - 2.0 * dt) + vec2f(0.0, 15.0 * dt);
        p.pos = p.pos + p.vel * dt;
    }

    particles[idx] = p;
}
