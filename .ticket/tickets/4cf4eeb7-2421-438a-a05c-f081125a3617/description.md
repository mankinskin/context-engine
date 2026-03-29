# Character: First-Person Camera Controller with SVO-Derived Rapier Collision

## Problem

The user navigates the 3D world via a first-person character controller. Physics (gravity, ground detection, collision response) is handled by bevy_rapier3d, with collision geometry derived from the SVO by T7's Rapier bridge. The character is a **kinematic rigid body** — not a dynamic body — so movement is code-driven with physics constraints.

## Architecture: Kinematic Character on SVO Terrain

### Character ECS Setup

```rust
#[derive(Component)]
pub struct CharacterController {
    pub move_speed: f32,       // units per second
    pub sprint_multiplier: f32,
    pub jump_impulse: f32,
    pub mouse_sensitivity: f32,
    pub is_grounded: bool,
    pub vertical_velocity: f32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub controller: CharacterController,
    pub rigid_body: RigidBody,        // Rapier: KinematicPositionBased
    pub collider: Collider,           // Rapier: capsule
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
```

### Movement System

```rust
fn character_movement_system(
    input: Res<InputState>,
    mut characters: Query<(&mut CharacterController, &mut Transform)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (mut ctrl, mut transform) in characters.iter_mut() {
        // Horizontal movement
        let forward = transform.forward();
        let right = transform.right();
        let mut move_dir = Vec3::ZERO;
        if input.w { move_dir += forward; }
        if input.s { move_dir -= forward; }
        if input.d { move_dir += right; }
        if input.a { move_dir -= right; }
        move_dir.y = 0.0;
        if move_dir.length_squared() > 0.0 { move_dir = move_dir.normalize(); }

        let speed = ctrl.move_speed * if input.shift { ctrl.sprint_multiplier } else { 1.0 };

        // Vertical (gravity + jump)
        if ctrl.is_grounded && input.space {
            ctrl.vertical_velocity = ctrl.jump_impulse;
        }
        ctrl.vertical_velocity -= 9.81 * time.delta_seconds();

        // Desired displacement
        let displacement = move_dir * speed * time.delta_seconds()
            + Vec3::Y * ctrl.vertical_velocity * time.delta_seconds();

        // Rapier kinematic move (slides along SVO-derived colliders)
        let result = rapier_context.move_shape(
            transform.translation,
            &Collider::capsule_y(0.4, 0.3),
            displacement,
            QueryFilter::default(),
            0.01, // skin width
        );

        transform.translation += result.effective_translation;
        ctrl.is_grounded = result.grounded;
        if ctrl.is_grounded && ctrl.vertical_velocity < 0.0 {
            ctrl.vertical_velocity = 0.0;
        }
    }
}
```

### Camera System

The camera is attached to the character and controlled by mouse look:

```rust
fn camera_look_system(
    mut mouse_motion: EventReader<MouseMotion>,
    mut characters: Query<(&CharacterController, &mut Transform)>,
    mut camera: Query<&mut Camera3D>,
) {
    let total_delta: Vec2 = mouse_motion.read().map(|e| e.delta).sum();
    for (ctrl, mut transform) in characters.iter_mut() {
        // Yaw (horizontal) rotates the character
        transform.rotate_y(-total_delta.x * ctrl.mouse_sensitivity);
    }
    // Pitch (vertical) rotates the camera component only
    for mut cam in camera.iter_mut() {
        cam.pitch = (cam.pitch - total_delta.y * 0.002).clamp(-1.4, 1.4);
    }
}
```

### Input Handling (WASM)

Keyboard and mouse events captured via `web-sys` and fed into a Bevy `InputState` resource:

```rust
#[derive(Resource, Default)]
pub struct InputState {
    pub w: bool, pub a: bool, pub s: bool, pub d: bool,
    pub space: bool, pub shift: bool,
    pub mouse_locked: bool,
}
```

Pointer lock is requested on canvas click for FPS-style mouse control.

### Collision with SVO World

The character doesn't query the SVO directly. Instead:
1. T7 builds Rapier compound colliders from SVO chunks
2. Rapier's `move_shape` handles sliding, stepping, and ground detection
3. The character walks on voxel surfaces seamlessly

This means voxel edits (T16) immediately affect character collision after the Rapier chunk rebuild.

## Scope

### Rust: Character Controller (`src/character/`)
- `CharacterController` component
- `CharacterBundle`
- `character_movement_system` (kinematic move with Rapier)
- `camera_look_system` (mouse look)
- `InputState` resource
- Input capture from web-sys (keyboard + mouse events)

## Dependencies
- T7 (physics): Rapier plugin + SVO-derived chunk colliders
- T6 (3D scene): Camera3D component for rendering perspective
- T2 (WebGPU init): Canvas and window setup for input capture

## Acceptance Criteria
1. WASD moves the character horizontally in the world
2. Mouse look rotates the camera (yaw + pitch, pitch clamped)
3. Character falls under gravity and lands on SVO voxel surface
4. Jump (space) propels character upward, then gravity pulls back
5. Character slides along walls (Rapier kinematic collision response)
6. Sprint (shift) increases movement speed
7. Pointer lock activates on canvas click, ESC releases
