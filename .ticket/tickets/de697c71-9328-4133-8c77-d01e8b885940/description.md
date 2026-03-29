# Config: Runtime Parameters for SVO, Ray Marching, and Visual Tuning

## Problem

The context-editor needs a runtime parameter system for tuning SVO rendering, physics, particles, and UI without recompilation. Parameters must be accessible as Bevy resources and pushable to GPU uniforms.

## Architecture: Bevy Resource Parameters → GPU Uniforms

### Parameter Groups

```rust
#[derive(Resource)]
pub struct SvoParams {
    pub max_depth: u32,          // octree traversal depth (default: 10)
    pub world_size: f32,         // root cube edge length (default: 256.0)
    pub lod_scale: f32,          // LOD distance scaling factor (default: 5.0)
    pub voxel_resolution: f32,   // finest voxel edge length (default: 0.25)
}

#[derive(Resource)]
pub struct RayMarchParams {
    pub max_steps: u32,          // per-ray step limit (default: 128)
    pub max_distance: f32,       // ray termination distance (default: 500.0)
    pub hit_threshold: f32,      // SDF hit epsilon (default: 0.001)
    pub shadow_softness: f32,    // soft shadow factor (default: 8.0)
    pub ao_samples: u32,         // ambient occlusion sample count (default: 4)
    pub ao_radius: f32,          // AO sampling radius (default: 2.0)
}

#[derive(Resource)]
pub struct GlassParams {
    pub default_ior: f32,        // index of refraction (default: 1.5)
    pub max_panels: u32,         // max glass SDFs per frame (default: 16)
    pub max_refraction_bounces: u32, // stacked glass limit (default: 4)
}

#[derive(Resource)]
pub struct ParticleParams {
    pub max_particles: u32,      // buffer capacity (default: 10_000)
    pub gravity: Vec3,           // particle gravity (default: (0, -2, 0))
    pub restitution: f32,        // bounce factor (default: 0.6)
}

#[derive(Resource)]
pub struct PhysicsParams {
    pub gravity: Vec3,           // Rapier world gravity (default: (0, -9.81, 0))
    pub chunk_size: u32,         // Rapier chunk dimensions (default: 16)
}
```

### Uniform Packing

Parameters flow to GPU via `GlobalUniforms`:

```rust
fn pack_global_uniforms(
    svo: &SvoParams,
    rm: &RayMarchParams,
    glass: &GlassParams,
    time: f32,
) -> GlobalUniforms {
    GlobalUniforms {
        world_size: svo.world_size,
        max_depth: svo.max_depth,
        time,
        lod_scale: svo.lod_scale,
        max_steps: rm.max_steps,
        max_distance: rm.max_distance,
        hit_threshold: rm.hit_threshold,
        shadow_softness: rm.shadow_softness,
        ao_samples: rm.ao_samples,
        default_ior: glass.default_ior,
        ..Default::default()
    }
}
```

### Runtime Modification

Parameters can be changed at runtime via:
1. Debug UI panel (Dioxus component with sliders)
2. Bevy system that reads from a config file
3. Direct ECS mutation from other systems

Changes take effect next frame (uniform re-packed in `upload_uniforms_system`).

### Bevy Registration

```rust
app.insert_resource(SvoParams::default())
   .insert_resource(RayMarchParams::default())
   .insert_resource(GlassParams::default())
   .insert_resource(ParticleParams::default())
   .insert_resource(PhysicsParams::default())
   .add_systems(Update, upload_uniforms_system);
```

## Scope

### Rust: Parameter Resources (`src/params.rs`)
- `SvoParams`, `RayMarchParams`, `GlassParams`, `ParticleParams`, `PhysicsParams`
- Default constructors with documented values
- `pack_global_uniforms()` function

### Rust: Upload System
- `upload_uniforms_system` (reads all param resources → packs → writes GPU uniform buffer)

### Rust: Debug UI (optional stretch)
- Dioxus component with parameter sliders
- Two-way binding: slider change → resource mutation → GPU uniform update

## Dependencies
- T1 (scaffold): Bevy App for resource registration
- T6 (3D scene): GlobalUniforms struct consumed by ray march shader
- T2 (WebGPU init): Uniform buffer infrastructure

## Acceptance Criteria
1. All parameter resources exist with documented defaults
2. Changing `SvoParams::max_depth` at runtime changes LOD quality visibly
3. Changing `RayMarchParams::max_steps` affects rendering fidelity and performance
4. Changing `GlassParams::default_ior` changes glass refraction appearance
5. Parameters propagate to GPU within one frame of mutation
6. System handles missing/default resources gracefully
