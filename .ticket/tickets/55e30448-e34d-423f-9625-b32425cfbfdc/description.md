# UI: 3D World-Space Panel System as Analytical SDFs

## Problem

Some UI elements (floating labels, context menus, data readouts) must exist as **world-space panels** — positioned in 3D space relative to world objects rather than screen-space. These panels are analytical SDFs in the unified ray marching loop, sharing the glass infrastructure from T3 but with world-anchored transforms.

## Architecture: World-Space SDFs

### Screen-Space vs World-Space Panels

| Property | Screen-Space (T9) | World-Space (T10) |
|----------|-------------------|-------------------|
| Position | Derived from Taffy layout | Attached to ECS entity transform |
| Movement | Fixed relative to camera | Fixed in world, moves with camera |
| Depth | Layer-based (0, 1, 2...) | True 3D position |
| Parallax | Simulated via layers | Real perspective |
| Use cases | Main UI, menus, editors | Labels, tooltips, node annotations |

### ECS: World-Space Panel Component

```rust
#[derive(Component)]
pub struct WorldPanel {
    pub size: Vec2,           // panel dimensions in world units
    pub corner_radius: f32,
    pub ior: f32,
    pub tint: Color,
    pub blur_roughness: f32,
    pub text_content: String, // rendered as texture or Dioxus overlay
    pub billboard: bool,       // if true, always faces camera
}

#[derive(Bundle)]
pub struct WorldPanelBundle {
    pub panel: WorldPanel,
    pub glass: GlassPanel,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
```

### Billboard System

World panels with `billboard: true` always face the camera:

```rust
fn billboard_system(
    camera: Query<&GlobalTransform, With<Camera3D>>,
    mut panels: Query<(&WorldPanel, &mut Transform), Without<Camera3D>>,
) {
    let cam_pos = camera.single().translation();
    for (panel, mut transform) in panels.iter_mut() {
        if panel.billboard {
            transform.look_at(cam_pos, Vec3::Y);
        }
    }
}
```

### World Panel → Glass SDF

World panels feed into the same `GlassPanelBuffer` as screen-space panels:

```rust
fn world_panel_to_glass_system(
    panels: Query<(&WorldPanel, &GlobalTransform)>,
    mut glass_buffer: ResMut<GlassPanelBuffer>,
) {
    for (panel, transform) in panels.iter() {
        glass_buffer.panels.push(GlassPanelGpu {
            center: transform.translation(),
            half_size: Vec3::new(panel.size.x * 0.5, panel.size.y * 0.5, PANEL_THICKNESS * 0.5),
            corner_radius: panel.corner_radius,
            ior: panel.ior,
            tint: panel.tint.as_linear_rgba_f32().into(),
            blur_roughness: panel.blur_roughness,
        });
    }
}
```

### Anchoring to World Objects

World panels can be anchored to other entities (e.g., a ticket node, a character, a data point):

```rust
#[derive(Component)]
pub struct AnchoredTo {
    pub entity: Entity,
    pub offset: Vec3, // offset from anchor entity
}

fn anchor_system(
    anchors: Query<(&AnchoredTo, &mut Transform), With<WorldPanel>>,
    targets: Query<&GlobalTransform, Without<WorldPanel>>,
) {
    for (anchor, mut transform) in anchors.iter() {
        if let Ok(target) = targets.get(anchor.entity) {
            transform.translation = target.translation() + anchor.offset;
        }
    }
}
```

### Text Rendering on World Panels

Two approaches for text on world-space panels:

1. **CPU-rasterized texture**: Render text to a texture on CPU, bind as overlay in shader
2. **SDF text in ray march**: Evaluate glyph SDFs in the shader (expensive but resolution-independent)

For v1, use approach 1: pre-rasterize text to a small texture, then sample it when the ray hits the panel SDF. The panel surface shows the texture instead of pure glass tint.

### Interaction with World Panels

World panels that are interactive (buttons, links) use the same hit-testing system as T9, but with true 3D ray-panel intersection rather than screen-space projection:

```rust
fn world_panel_hit_test(
    ray: Ray,
    panel_transform: &GlobalTransform,
    panel: &WorldPanel,
) -> Option<Vec2> {
    // Ray-OBB intersection using panel transform
    let local_ray = transform_ray_to_local(ray, panel_transform);
    let hit = ray_aabb_intersection(local_ray, panel.local_bounds())?;
    let uv = Vec2::new(
        (hit.point.x / panel.size.x) + 0.5,
        (hit.point.y / panel.size.y) + 0.5,
    );
    Some(uv)
}
```

## Scope

### Rust: World Panel ECS (`src/ui/world_panel.rs`)
- `WorldPanel` component
- `WorldPanelBundle`
- `AnchoredTo` component
- `billboard_system`
- `anchor_system`
- `world_panel_to_glass_system`

### Rust: World Panel Interaction (`src/ui/world_hit.rs`)
- `world_panel_hit_test()` with ray-OBB intersection
- Integration with Dioxus event dispatch

### Rust: Text Rendering (`src/ui/world_text.rs`)
- CPU text rasterizer (basic: single font, white-on-transparent)
- Texture bind for ray march panel surface

## Dependencies
- T3 (liquid glass): GlassPanelBuffer and GlassPanelGpu struct
- T6 (3D scene): Camera3D, ray generation, render pipeline
- T9 (bridge): GlassPanelBuffer is shared — world panels append to same buffer

## Acceptance Criteria
1. A `WorldPanel` entity placed at a fixed 3D position is visible in the ray-marched scene
2. Billboard panels rotate to face the camera as it orbits
3. Anchored panels follow their target entity when it moves
4. World panels appear as glass SDFs with refraction and tint
5. Text content is visible on the panel surface
6. Clicking on a world panel returns the correct UV coordinates
7. World panels and screen-space panels coexist in the same GlassPanelBuffer
