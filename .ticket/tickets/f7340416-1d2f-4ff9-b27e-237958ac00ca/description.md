# Impl: Physics simulation via bevy_rapier3d + world environment system

## Problem

The context-editor needs a physical simulation layer for realistic object interactions and a world environment system. Instead of implementing custom AABB collision from scratch, we use **bevy_rapier3d** — the standard Bevy plugin for Rapier3D physics.

## Architecture: bevy_rapier3d Plugin

Rapier3D provides production-grade physics:
- Rigid body dynamics (position, velocity, forces, mass)
- Collision shapes (box, sphere, capsule, trimesh, heightfield)
- Joints and constraints
- Continuous collision detection
- Fixed timestep integration (decoupled from render frame rate by default)

Integration via `RapierPhysicsPlugin`:
```rust
app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
   .add_plugins(RapierDebugRenderPlugin::default()); // debug wireframes
```

Physics bodies are **Bevy entities** with Rapier components:
```rust
commands.spawn((
    RigidBody::Dynamic,
    Collider::cuboid(0.5, 0.5, 0.5),
    Restitution::coefficient(0.7),
    PbrBundle { mesh, material, transform, ..default() },
));
```

## Scope

### Physics via bevy_rapier3d (`src/world/physics.rs`)
- `RapierPhysicsPlugin` registered in Bevy app (T1)
- Rigid body types: `Dynamic` (affected by forces), `Fixed` (static), `KinematicPositionBased` (character)
- Collider shapes: `Collider::cuboid`, `Collider::ball`, `Collider::capsule`, `Collider::halfspace`
- Gravity: configurable via `RapierConfiguration` resource
- Damping: `Damping { linear_damping, angular_damping }` component
- Collision events: `CollisionEvent` Bevy event for gameplay logic
- Debug render: `RapierDebugRenderPlugin` for wireframe collision shape visualization

### World Environment (`src/world/environment.rs`)
- Terrain: ground plane as `Fixed` rigid body with `Collider::halfspace`
- Heightmap terrain (future): `Collider::heightfield` from Rapier
- Fog: distance-based fog with palette-derived color (shader uniform)
- Atmosphere: sky gradient (horizon to zenith) from theme
- Day/night cycle: sun `DirectionalLight` entity position animates over time
- Wind: `ExternalForce` applied to dynamic bodies + fed to particle compute shader

### Environment Shader Integration
- Fog computed in scene3d fragment: `mix(object_color, fog_color, fog_factor)`
- Sky gradient rendered as Bevy background or fullscreen quad shader
- Wind direction fed to particle compute shader as uniform from `WindConfig` resource

### World Objects (`src/world/objects.rs`)
- Primitive meshes: cube, sphere, cylinder, plane — spawned as Bevy entities with `PbrBundle` + `RigidBody` + `Collider`
- Instanced rendering for repeated objects (Bevy handles instancing automatically)
- Per-object `Transform`: position, rotation, scale
- Object picking: `bevy_rapier3d` ray-cast (`RapierContext::cast_ray`) from mouse through camera

### ECS Components
```rust
#[derive(Resource)]
struct WindConfig {
    direction: Vec3,
    strength: f32,
}

#[derive(Resource)]
struct EnvironmentConfig {
    fog_density: f32,
    fog_color: Color,
    day_cycle_speed: f32,
    sun_angle: f32,
}
```

## Files to Create
| File | Purpose |
|------|---------|
| `src/world/mod.rs` | World module |
| `src/world/physics.rs` | bevy_rapier3d setup + physics helpers |
| `src/world/environment.rs` | Terrain, fog, atmosphere, day/night |
| `src/world/objects.rs` | Primitive mesh spawning with physics bodies |
| `shaders/environment.wgsl` | Sky gradient + fog |

## Acceptance Criteria
1. Objects dropped into the scene fall under gravity and land on the ground plane (Rapier physics)
2. Rapier colliders prevent objects from passing through each other
3. Fog fades distant objects to palette-derived fog color
4. Sky gradient transitions from horizon to zenith using theme colors
5. Wind parameter visibly affects particle drift direction
6. Physics runs at Rapier's fixed timestep independent of render FPS
7. `RapierContext::cast_ray` correctly identifies clicked object (ray-cast picking)
8. Debug render plugin shows collision shape wireframes when enabled
