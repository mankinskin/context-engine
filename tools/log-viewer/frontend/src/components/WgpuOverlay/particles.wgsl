// particles.wgsl — multi-type instanced particle rendering
//
// Concatenated after: palette.wgsl + types.wgsl + noise.wgsl + particle-shading.wgsl
// Four particle types rendered in one instanced draw call:
//   PK_METAL_SPARK (0) — tiny pixel-size metallic dot (at mouse cursor)
//   PK_EMBER       (1) — tiny pixel-size warm ember/ash glow (continuous)
//   PK_GOD_RAY     (2) — pixel-thin tall vertical angelic beam (continuous)
//   PK_GLITTER     (3) — tiny angelic twinkle around selected element border
//
// Fragment colouring uses shared functions from particle-shading.wgsl
// (shade_spark_fx, shade_ember_fx, shade_beam_fx, shade_glitter_fx).

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
        // Offset upward so the bottom tip sits at the spawn point
        let half_w = p.size * 2.0;         // wider quad for AA margin
        let bh = select(35.0, u.beam_height, u.beam_height > 0.0);
        let half_h = p.size * bh;          // configurable height
        out.aspect = half_h / max(half_w, 0.1);
        world_pos = p.pos + vec2f(corner.x * half_w, corner.y * half_h - half_h);

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

// ---- fragment shader (delegates to shared shade functions) -------------------

@fragment
fn fs_particle(in: ParticleVarying) -> @location(0) vec4f {
    let p = particles[in.pidx];
    let t_life = p.life / p.max_life;
    let kind = in.pkind;

    // Compute fwidth in uniform control flow (before branching on kind)
    let fw = fwidth(in.local_uv.x);

    if kind == 0u {
        return shade_spark_fx(in.local_uv, t_life, p.hue);
    } else if kind == 1u {
        return shade_ember_fx(in.local_uv, t_life, p.hue);
    } else if kind == 2u {
        return shade_beam_fx(in.local_uv, t_life, fw * 2.0);
    } else {
        return shade_glitter_fx(in.local_uv, t_life, p.hue, u.time, f32(in.pidx));
    }
}
