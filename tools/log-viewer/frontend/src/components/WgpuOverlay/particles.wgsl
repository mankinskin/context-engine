// particles.wgsl — multi-type instanced particle rendering
//
// Concatenated after: types.wgsl + noise.wgsl
// Four particle types rendered in one instanced draw call:
//   PK_METAL_SPARK (0) — tiny pixel-size metallic dot (at mouse cursor)
//   PK_EMBER       (1) — tiny pixel-size warm ember/ash glow (continuous)
//   PK_GOD_RAY     (2) — pixel-thin tall vertical angelic beam (continuous)
//   PK_GLITTER     (3) — tiny angelic twinkle around selected element border

// ---- bindings (render pass — read-only) -------------------------------------

@group(0) @binding(0) var<uniform>       u         : Uniforms;
@group(0) @binding(1) var<storage, read> elems     : array<ElemRect>;
@group(0) @binding(2) var<storage, read> particles : array<Particle>;

// ---- interpolated data between VS and FS ------------------------------------

struct ParticleVarying {
    @builtin(position)                    clip_pos : vec4f,
    @location(0)                          local_uv : vec2f,   // [-1..1] in oriented quad space
    @location(1) @interpolate(flat)       pidx     : u32,
    @location(2) @interpolate(flat)       pkind    : u32,
    @location(3) @interpolate(flat)       aspect   : f32,     // elongation ratio
}

// ---- vertex shader ----------------------------------------------------------

@vertex
fn vs_particle(
    @builtin(vertex_index)   vid : u32,
    @builtin(instance_index) iid : u32,
) -> ParticleVarying {
    var out: ParticleVarying;
    out.pidx  = iid;
    out.pkind = 0u;
    out.aspect = 1.0;

    let p = particles[iid];
    let kind = u32(p.kind);
    out.pkind = kind;

    // Dead → degenerate quad (off-screen)
    if p.life <= 0.0 {
        out.clip_pos = vec4f(-2.0, -2.0, 0.0, 1.0);
        out.local_uv = vec2f(0.0);
        return out;
    }

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

    var world_pos: vec2f;

    if kind == 0u {
        // ---- METAL SPARK: tiny pixel-size dot ----
        let radius = p.size * 1.5;
        world_pos = p.pos + corner * radius;

    } else if kind == 1u {
        // ---- EMBER / ASH: tiny pixel-size dot ----
        let radius = p.size * 1.2;
        world_pos = p.pos + corner * radius;

    } else if kind == 2u {
        // ---- ANGELIC BEAM: pixel-thin tall vertical line ----
        let half_w = p.size * 2.0;         // wider quad for AA margin
        let half_h = p.size * 35.0;        // taller vertical
        out.aspect = half_h / max(half_w, 0.1);
        world_pos = p.pos + vec2f(corner.x * half_w, corner.y * half_h);

    } else {
        // ---- GLITTER: slightly larger sparkle ----
        let radius = p.size * 1.8;
        world_pos = p.pos + corner * radius;
    }

    let clip_x = world_pos.x / u.width  *  2.0 - 1.0;
    let clip_y = world_pos.y / u.height * -2.0 + 1.0;
    out.clip_pos = vec4f(clip_x, clip_y, 0.0, 1.0);

    return out;
}

// ---- fragment shader --------------------------------------------------------

@fragment
fn fs_particle(in: ParticleVarying) -> @location(0) vec4f {
    let p = particles[in.pidx];
    let t_life = p.life / p.max_life;
    let kind = in.pkind;

    // Compute fwidth in uniform control flow (before branching on kind)
    let fw = fwidth(in.local_uv.x);

    if kind == 0u {
        return shade_metal_spark(in, p, t_life);
    } else if kind == 1u {
        return shade_ember(in, p, t_life);
    } else if kind == 2u {
        return shade_angel_beam(in, p, t_life, fw);
    } else {
        return shade_glitter(in, p, t_life);
    }
}

// ---- metal spark fragment (subtle trailing dot) ----------------------------

fn shade_metal_spark(in: ParticleVarying, p: Particle, t_life: f32) -> vec4f {
    let d = length(in.local_uv);

    // Soft-edged tiny dot — subtle
    let dot_mask = smoothstep(1.0, 0.2, d);
    let bright = t_life * t_life * dot_mask * 0.8;

    if bright < 0.005 { discard; }

    // Hot metal colouring: white-yellow core fading to ember
    let ember = cinder_rgb(p.hue);
    let hot_core = vec3f(1.4, 1.1, 0.6);
    let steel    = vec3f(0.6, 0.6, 0.7);
    let core_t = smoothstep(0.5, 0.0, d);
    var spark_col = mix(ember * 1.3, hot_core, core_t * 0.8);
    spark_col = mix(spark_col, steel, smoothstep(0.3, 0.8, d) * 0.3);

    let col = spark_col * bright;
    let a   = min(bright * 0.85, 1.0);
    return vec4f(col * a, a);
}

// ---- ember / ash fragment ---------------------------------------------------

fn shade_ember(in: ParticleVarying, p: Particle, t_life: f32) -> vec4f {
    let d = length(in.local_uv);

    let glow = exp(-d * d * 2.5);
    let bright = t_life * glow;

    if bright < 0.005 { discard; }

    let ember_col = cinder_rgb(p.hue);
    let hot = vec3f(1.2, 0.9, 0.4);
    let base = mix(ember_col * 1.5, hot, smoothstep(0.5, 0.0, d));

    let col = base * bright;
    let a   = min(bright * 0.7, 1.0);
    return vec4f(col * a, a);
}

// ---- angelic shard fragment (pointed crystalline beam) ----------------------

fn shade_angel_beam(in: ParticleVarying, p: Particle, t_life: f32, fw: f32) -> vec4f {
    let dx = in.local_uv.x;          // horizontal (thin axis)
    let dy = in.local_uv.y;          // vertical   (tall axis)

    // Double-pointed shard: widest at centre, pointed at both ends
    // t goes 0 at bottom, 1 at top; mid = distance from centre (0..0.5)
    let t   = (dy + 1.0) * 0.5;
    let mid = abs(t - 0.5) * 2.0;                          // 0 at centre, 1 at tips
    let shard_width = (1.0 - mid * mid) * 0.18;            // very thin diamond envelope

    // Horizontal mask with AA — sharp edge narrowing toward tips
    let hx = abs(dx) / max(shard_width, 0.005);
    let edge = smoothstep(1.0 + fw * 2.0, 1.0 - fw * 2.0, hx);

    // Heavy bright center ridge — concentrated core
    let core = exp(-dx * dx / max(shard_width * shard_width * 0.1, 0.0005));
    let h_falloff = edge * (0.25 + 0.75 * core);

    // Vertical fade — brightest at centre, pointed ends vanish
    let v_fade = (1.0 - mid * mid);

    let bright = h_falloff * v_fade * t_life * 1.6;

    if bright < 0.003 { discard; }

    // Golden-white angelic colour — hot center, warm edge
    let center_col = vec3f(1.6, 1.5, 1.3);
    let edge_col   = vec3f(1.1, 0.85, 0.4);
    var ray_col = mix(edge_col, center_col, core * 0.95);

    let col = ray_col * bright * 0.6;
    let a   = min(bright * 0.4, 1.0);

    return vec4f(col * a, a);
}

// ---- angelic glitter fragment (pixel-size twinkle around selected) -----------

fn shade_glitter(in: ParticleVarying, p: Particle, t_life: f32) -> vec4f {
    let d = length(in.local_uv);

    // Slightly softer dot — more visible
    let dot_mask = smoothstep(1.0, 0.15, d);

    // Rapid twinkle — celestial sparkle quality, brighter baseline
    let twinkle = 0.6 + 0.4 * sin(u.time * 12.0 + f32(in.pidx) * 7.3);
    let bright = t_life * dot_mask * twinkle * 1.4;

    if bright < 0.008 { discard; }

    // Angelic golden-white with cool blue variation
    let warm = vec3f(1.3, 1.15, 0.85);
    let cool = vec3f(0.85, 0.90, 1.25);
    let phase = fract(p.hue * 3.7 + u.time * 0.5);
    let glitter_col = mix(warm, cool, smoothstep(0.3, 0.7, phase));

    let col = glitter_col * bright;
    let a   = min(bright * 0.9, 1.0);

    return vec4f(col * a, a);
}
