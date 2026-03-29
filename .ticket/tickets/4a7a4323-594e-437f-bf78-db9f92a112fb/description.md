# VFX: GPU Particle System with SVO Collision

## Problem

The context-editor needs an ambient particle system (motes, sparks, data wisps) that interact with the voxel world. Particles must **collide with SVO geometry on the GPU** via a compute shader that queries the octree, and bounce off both voxels and glass panel SDFs.

## Architecture: Compute Shader SVO Collision

### Particle Lifecycle (GPU-Driven)

All particle state lives in GPU storage buffers. The CPU only configures emitters and uploads SVO data — the GPU handles simulation, collision, and rendering.

```
CPU (Bevy ECS):                    GPU (Compute + Render):
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
                                  │ ray_march.wgsl        │
                                  │ (particles as emissive│
                                  │  point SDFs in ray)   │
                                  └──────────────────────┘
```

### SVO Collision in Compute Shader

```wgsl
@group(0) @binding(0) var<storage, read> octree: array<OctreeNode>;
@group(0) @binding(1) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(2) var<uniform> sim_params: SimParams;
@group(0) @binding(3) var<storage, read> glass_panels: array<GlassPanel>;

struct Particle {
    pos: vec3f,
    vel: vec3f,
    life: f32,
    color: u32,    // packed RGBA
    size: f32,
}

@compute @workgroup_size(256)
fn sim_particles(@builtin(global_invocation_id) id: vec3u) {
    let idx = id.x;
    if idx >= sim_params.count { return; }

    var p = particles[idx];
    if p.life <= 0.0 { return; }

    // Integrate
    p.vel += sim_params.gravity * sim_params.dt;
    let new_pos = p.pos + p.vel * sim_params.dt;

    // SVO collision: query octree at new position
    let svo_dist = query_svo_distance(new_pos);
    if svo_dist < p.size {
        // Compute voxel surface normal from gradient
        let normal = svo_gradient_normal(new_pos);
        // Bounce
        p.vel = reflect(p.vel, normal) * sim_params.restitution;
        // Push out of surface
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

### `query_svo_distance`: Shared Octree Query

Both the ray marcher (T6) and particle compute shader share the same octree query function:

```wgsl
// Returns approximate distance to nearest occupied voxel
fn query_svo_distance(pos: vec3f) -> f32 {
    var node_idx = 0u;  // root
    var node_size = globals.world_size;
    var node_center = vec3f(0.0);

    for (var depth = 0u; depth < globals.max_depth; depth++) {
        let node = octree[node_idx];
        let child_mask = node.child_pointer & 0xFFu;
        if child_mask == 0u { return node_size; }  // empty node → distance = node size

        // Determine which octant `pos` falls in
        let octant = octant_index(pos, node_center);
        if (child_mask & (1u << octant)) == 0u {
            return node_size * 0.5;  // this octant is empty
        }

        // Descend
        let first_child = node.child_pointer >> 8u;
        let child_offset = countOneBits(child_mask & ((1u << octant) - 1u));
        node_idx = first_child + child_offset;
        node_size *= 0.5;
        node_center = child_center(node_center, node_size, octant);
    }
    return 0.0; // inside a leaf voxel
}
```

### Particle Rendering in Ray March

Particles are NOT rasterized as billboards. They are tiny emissive SDFs evaluated in the ray marching loop:

```wgsl
// In march_ray(), check particle proximity
fn nearest_particle_sdf(p: vec3f) -> ParticleHit {
    // Use spatial hash or brute force (for < 1000 particles)
    var best = ParticleHit(999.0, vec4f(0.0));
    for (var i = 0u; i < particle_count; i++) {
        let d = length(p - particles[i].pos) - particles[i].size;
        if d < best.dist {
            best = ParticleHit(d, unpack_color(particles[i].color));
        }
    }
    return best;
}
```

For large particle counts (>1000), a GPU spatial hash or BVH stored in a storage buffer would replace the brute-force loop.

### Bevy ECS: Emitter Management

```rust
#[derive(Component)]
pub struct ParticleEmitter {
    pub rate: f32,         // particles per second
    pub lifetime: f32,     // seconds
    pub speed_range: (f32, f32),
    pub size_range: (f32, f32),
    pub color: Color,
    pub gravity_scale: f32,
    pub restitution: f32,  // bounce factor
}

// System: update emitter uniforms
fn emitter_system(
    query: Query<(&Transform, &ParticleEmitter)>,
    mut sim_params: ResMut<ParticleSimParams>,
) {
    // Pack emitter data into GPU uniform
}
```

### Performance Targets

| Metric | Target |
|--------|--------|
| Compute dispatch (1024 particles) | < 0.5ms |
| Compute dispatch (10K particles) | < 2ms |
| SVO query per particle | ~10 octree levels = ~10 memory reads |
| Particle SDF eval in ray march | < 1ms (bounded by particle count) |

## Scope

### WGSL Shaders
- `particles_compute.wgsl`: emit + simulate + SVO collide + glass SDF collide
- `ray_march.wgsl` additions: `nearest_particle_sdf()` for rendering particles as emissive SDFs

### Rust: ECS
- `ParticleEmitter` component
- `ParticleSimParams` resource (gravity, dt, counts)
- `ParticleBuffer` resource (GPU storage buffer handle)
- `emitter_system` (Update)
- `particle_buffer_management_system` (resize buffer when particle count changes)

### Rust: Render
- Compute pipeline for particle simulation
- Bind group sharing octree + glass panel buffers
- Dispatch as pre-pass before ray march node

## Dependencies
- T6 (3D scene): `query_svo_distance` function in WGSL + octree storage buffer
- T3 (liquid glass): `sdf_rounded_box` function for glass collision
- T1 (scaffold): Compute shader infrastructure

## Acceptance Criteria
1. Particles emit from a point emitter and fall under gravity
2. Particles bounce off SVO voxels (visible collision response)
3. Particles bounce off glass panel SDFs
4. Particles rendered as tiny glowing points in the ray-marched scene (not billboards)
5. Particle compute shader completes in < 2ms for 10K particles
6. Dead particles (life ≤ 0) are recycled by the emitter
7. Emitter parameters (rate, speed, color) controllable from Bevy ECS
