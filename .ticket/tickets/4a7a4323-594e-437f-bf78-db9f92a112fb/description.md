# VFX: GPU Particle System with SVO Collision and Gaussian Rendering

## Problem

Ambient particles (motes, sparks, data wisps) must collide with SVO geometry on the GPU and be rendered as tiny Gaussians in the tiled forward+ pipeline — not as billboards or point SDFs.

## Architecture: Compute SVO Collision + Gaussian Particles

### Particle Lifecycle (GPU-Driven)

```
CPU (Bevy ECS):                    GPU (Compute + Tiled Render):
┌─────────────────┐               ┌──────────────────────┐
│ Emitter system   │──uniforms──→│ emit_particles.wgsl   │
│ (spawn params)   │              │ (initialize new)      │
└─────────────────┘               └──────────┬───────────┘
                                             │
                                  ┌──────────▼───────────┐
                                  │ sim_particles.wgsl    │
                                  │ • integrate velocity  │
                                  │ • query_svo(pos)      │
                                  │ • sdf_glass(pos)      │
                                  │ • bounce / kill       │
                                  └──────────┬───────────┘
                                             │
                                  ┌──────────▼───────────┐
                                  │ particle_to_gaussian  │
                                  │ (emit tiny Gaussians  │
                                  │  per live particle)   │
                                  └──────────┬───────────┘
                                             │
                                  ┌──────────▼───────────┐
                                  │ Tiled Rasterizer      │
                                  │ (particles appear as  │
                                  │  soft glowing splats) │
                                  └──────────────────────┘
```

### SVO Collision (Compute Shader)

```wgsl
@compute @workgroup_size(256)
fn sim_particles(@builtin(global_invocation_id) id: vec3u) {
    let idx = id.x;
    if idx >= sim_params.count { return; }
    var p = particles[idx];
    if p.life <= 0.0 { return; }

    p.vel += sim_params.gravity * sim_params.dt;
    let new_pos = p.pos + p.vel * sim_params.dt;

    // SVO collision
    let svo_dist = query_svo_distance(new_pos);
    if svo_dist < p.size {
        let normal = svo_gradient_normal(new_pos);
        p.vel = reflect(p.vel, normal) * sim_params.restitution;
        p.pos = new_pos + normal * (p.size - svo_dist);
    } else {
        p.pos = new_pos;
    }

    // Glass SDF collision
    for (var g = 0u; g < sim_params.glass_count; g++) {
        let glass_dist = sdf_rounded_box(p.pos, glass_panels[g]);
        if glass_dist < p.size {
            let glass_normal = glass_sdf_normal(p.pos, glass_panels[g]);
            p.vel = reflect(p.vel, glass_normal) * 0.5;
            p.pos += glass_normal * (p.size - glass_dist);
        }
    }

    p.life -= sim_params.dt;
    particles[idx] = p;
}
```

### Particle → Gaussian Emission

Each live particle emits a tiny isotropic Gaussian into the splat buffer:

```wgsl
@compute @workgroup_size(256)
fn particle_to_gaussian(@builtin(global_invocation_id) id: vec3u) {
    let idx = id.x;
    if idx >= sim_params.count { return; }
    let p = particles[idx];
    if p.life <= 0.0 { return; }

    let gi = atomicAdd(&gaussian_count, 1u);
    gaussians[gi] = GaussianData(
        p.pos,
        p.life / sim_params.max_life, // opacity fades with life
        isotropic_covariance(p.size),  // uniform sphere
        emissive_sh(unpack_color(p.color)), // constant SH (no view-dependence for particles)
    );
}
```

Particles rendered as Gaussians automatically benefit from:
- Tiled sorting (correct depth ordering with world Gaussians)
- EWA projection (smooth anti-aliased circles on screen)
- Glass refraction (particles behind glass appear distorted)

### Bevy ECS

```rust
#[derive(Component)]
pub struct ParticleEmitter {
    pub rate: f32,
    pub lifetime: f32,
    pub speed_range: (f32, f32),
    pub size_range: (f32, f32),
    pub color: Color,
    pub gravity_scale: f32,
    pub restitution: f32,
}
```

### Performance

| Metric | Target |
|--------|--------|
| Compute dispatch (10K particles) | < 2ms |
| Particle → Gaussian emission | < 0.2ms (atomic append) |
| SVO query per particle | ~10 octree levels |
| Particles integrate seamlessly into tiled sort | No extra sort pass needed |

## Dependencies
- T6 (3D scene): `query_svo_distance` shared WGSL function, SVO storage buffer
- T3 (liquid glass): `sdf_rounded_box` for glass collision
- T2 (render init): Gaussian buffer + atomic counter + tiled pipeline

## Acceptance Criteria
1. Particles emit from a point emitter and fall under gravity
2. Particles bounce off SVO voxels (visible collision response)
3. Particles bounce off glass panel SDFs
4. Particles rendered as soft glowing Gaussians (not billboards or points)
5. Particles correctly depth-sorted among world Gaussians
6. Particles behind glass appear refracted
7. Compute shader completes in < 2ms for 10K particles
8. Dead particles (life ≤ 0) are recycled; their Gaussians are not emitted
