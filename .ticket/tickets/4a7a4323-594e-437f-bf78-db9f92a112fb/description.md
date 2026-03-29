# Impl: Particle system — Bevy ECS compute shader simulation + instanced rendering

## Problem

The context-editor needs a GPU-driven particle system for ambient effects, interactive feedback, and atmospheric world-building. Particles are managed as **Bevy entities/resources** with simulation via compute shaders and rendering via instanced draw calls in a custom Bevy render graph node.

## Architecture: Bevy ECS Particle Management

Particles live in Bevy's ECS:
- **`ParticleEmitter` component**: attached to entities that spawn particles (world objects, UI events)
- **`ParticleConfig` resource**: global particle settings (max count, type weights, global forces)
- **`ParticleRenderNode`**: custom Bevy render graph node dispatching compute + draw
- Bevy systems handle emitter logic (spawn, burst); GPU compute handles per-particle simulation

## Scope

### Compute Shader (`shaders/particles_compute.wgsl`)
- 640+ particles across multiple types (metal sparks, embers, god rays, glitter)
- Per-particle state: position, velocity, acceleration, lifetime, type, color
- Per-frame simulation: integrate velocity, apply gravity/wind, decay lifetime
- Emitter logic: continuous emission from world positions, burst on events
- View-aware: particles operate in world-space (3D scenes) or screen-space (UI)

### Render Shader (`shaders/particles_render.wgsl`)
- Instanced billboard quads (camera-facing)
- Per-type visual style: velocity-aligned streaks (sparks), tiny glows (embers), tall beams (god rays), point twinkles (glitter)
- Alpha blending with additive mode for glow effects
- Size modulation by lifetime (fade-out)

### Bevy Integration (`src/gpu/particles.rs`)
- `ParticlePlugin`: Bevy plugin registering systems + render graph node
- `ParticleEmitter` component: `{ position: Vec3, particle_type: ParticleType, rate: f32, burst_count: u32 }`
- `emit_burst` system: triggers particle bursts from emitter entities
- `ParticleRenderNode`: custom Bevy render graph node
  - Dispatches compute shader per frame (workgroup size 64)
  - Draws instanced quads after scene + glass passes
- Storage buffer (read-write) managed as Bevy render world resource

### ECS Components
```rust
#[derive(Component)]
struct ParticleEmitter {
    particle_type: ParticleType,
    rate: f32,           // particles per second
    burst_count: u32,    // burst size on trigger
    active: bool,
}

#[derive(Resource)]
struct ParticleConfig {
    max_particles: u32,
    gravity: Vec3,
    wind: Vec3,
    global_speed_scale: f32,
}
```

## Reuse from Existing Code
- Port particle types and compute logic from `log-viewer/frontend/src/components/WgpuOverlay/compute.wgsl`
- Port particle rendering from `log-viewer/frontend/src/components/WgpuOverlay/particles.wgsl`
- Reuse noise functions from `noise.wgsl`
- Reuse palette integration pattern from existing overlay

## Files to Create
| File | Purpose |
|------|---------|
| `shaders/particles_compute.wgsl` | GPU particle simulation |
| `shaders/particles_render.wgsl` | Instanced particle rendering |
| `src/gpu/particles.rs` | `ParticlePlugin` (Bevy plugin: systems, components, render node) |

## Acceptance Criteria
1. Compute shader simulates 640+ particles per frame without frame drops
2. At least 4 particle types render with distinct visual styles
3. `ParticleEmitter` entities spawn particles at their `Transform` positions
4. Burst emission triggered by Bevy events (click, selection)
5. Particles fade and die based on lifetime
6. Additive blending produces correct glow effects
7. Particles render correctly in both screen-space and world-space modes
8. Particle render node correctly positioned in Bevy render graph (after glass pass)
