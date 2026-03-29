# Impl: Character controller — Bevy system with first/third person camera and Rapier collision

## Problem

The context-editor needs character-based navigation through the 3D world, supporting first-person and third-person camera modes with WASD movement, mouse look, jump, and collision with world geometry. Implemented as **Bevy ECS systems** using **bevy_rapier3d** for physics.

## Architecture: Bevy ECS Character

The character is a **Bevy entity** with Rapier physics components:
```rust
commands.spawn((
    CharacterController,
    RigidBody::KinematicPositionBased,
    Collider::capsule_y(0.5, 0.3),  // standing capsule
    KinematicCharacterController::default(),
    TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
));
```

Rapier's `KinematicCharacterController` handles:
- Ground detection (is the character standing on something?)
- Slope handling and step climbing
- Collision response with static/dynamic bodies
- No need for custom raycast-based ground detection

## Scope

### Character System (`src/world/character.rs`)
- Bevy system: reads input state, computes movement vector, applies via `KinematicCharacterController`
- WASD movement (relative to camera facing direction)
- Jump with gravity (Rapier handles downward acceleration)
- Sprint (shift key modifier)
- Ground detection via `KinematicCharacterControllerOutput::grounded`

### Camera Modes (Bevy Systems)
- **First-person**: camera at character entity eye height, mouselook controls view direction
- **Third-person**: camera orbits behind character at configurable distance
- Toggle between modes (keyboard shortcut, e.g. F5)
- Smooth transition when switching modes (Bevy system interpolates camera Transform)

### Pointer Lock
- Request pointer lock on canvas click (via web-sys `request_pointer_lock`)
- Release on Escape
- Raw mouse delta for smooth look movement (no cursor visible)

### Input System (`src/input/mod.rs`)
- Bevy resource: `InputState { keys_pressed: HashSet<KeyCode>, mouse_delta: Vec2, mouse_buttons: ... }`
- Event listeners on window (web-sys: keydown, keyup, mousemove, mousedown, mouseup)
- Input consumed once per frame in a Bevy system (batching)
- Or: use `bevy_input` built-in if available on wasm32 target

### ECS Components
```rust
#[derive(Component)]
struct CharacterController;

#[derive(Component)]
struct CameraMode {
    mode: CameraModeType,     // FirstPerson | ThirdPerson
    orbit_distance: f32,       // third-person distance
    pitch: f32,
    yaw: f32,
}

#[derive(Resource)]
struct InputState {
    keys: HashSet<KeyCode>,
    mouse_delta: Vec2,
    sprint: bool,
}
```

### Integration
- Character position = Bevy entity `Transform` — camera system (T6) reads it
- Character collision uses bevy_rapier3d `KinematicCharacterController` (T7)
- Character can interact with world objects via `RapierContext::cast_ray` (T7)

## Files to Create
| File | Purpose |
|------|---------|
| `src/world/character.rs` | Character controller Bevy system |
| `src/input/mod.rs` | Input state resource + web-sys listeners |

## Acceptance Criteria
1. WASD moves character relative to camera facing direction
2. Mouse look rotates view smoothly with pointer lock active
3. Jump launches character upward, Rapier gravity brings them down
4. Character collides with ground plane via `KinematicCharacterController` (does not fall through)
5. First-person and third-person modes toggle with visual transition
6. Sprint (shift) increases movement speed
7. Escape releases pointer lock and shows cursor
8. Character entity queryable by other systems via `With<CharacterController>` filter
