# UI: Dioxus-Taffy Layout Bridge to 3D SDF Projection

## Problem

Dioxus manages the UI component tree (text, events, accessibility) and Taffy computes pixel-accurate flexbox layout. These 2D layout rectangles must be projected into **3D box SDFs** that the ray marching shader evaluates as glass panels. The bridge converts Taffy's `(x, y, width, height)` into `(center, half_size)` in world space for the GPU.

## Architecture: Layout → 3D SDF Pipeline

```
Dioxus Component Tree
        │
        ▼
   Taffy Flexbox Engine
   (pixel rects: x, y, w, h)
        │
        ▼
   LayoutRects Resource (Bevy)
   (per-panel: Rect + depth layer)
        │
        ▼
   Layout-to-3D System
   (project rect → world-space box SDF)
        │
        ▼
   GlassPanelBuffer (GPU uniform)
   (center, half_size, ior, tint)
        │
        ▼
   Ray March Shader
   (evaluate box SDFs in march loop)
```

### Screen-Space to World-Space Projection

UI panels are placed in 3D space at a configurable distance from the camera. Each 2D rect becomes a 3D box SDF:

```rust
fn layout_to_3d_system(
    layout_rects: Res<LayoutRects>,
    camera: Query<&Camera3D>,
    mut glass_buffer: ResMut<GlassPanelBuffer>,
) {
    let cam = camera.single();
    glass_buffer.panels.clear();

    for panel in layout_rects.panels.iter() {
        // Convert pixel rect to normalized device coordinates
        let ndc_x = (panel.rect.x + panel.rect.width * 0.5) / viewport_width * 2.0 - 1.0;
        let ndc_y = 1.0 - (panel.rect.y + panel.rect.height * 0.5) / viewport_height * 2.0;

        // Project to world space at panel's depth
        let depth = panel.depth_layer as f32 * PANEL_DEPTH_SPACING;
        let world_center = cam.ndc_to_world(Vec3::new(ndc_x, ndc_y, depth));

        // Scale pixel dimensions to world units
        let world_half_w = (panel.rect.width / viewport_width) * cam.frustum_width_at(depth) * 0.5;
        let world_half_h = (panel.rect.height / viewport_height) * cam.frustum_height_at(depth) * 0.5;
        let world_half_d = PANEL_THICKNESS * 0.5;

        glass_buffer.panels.push(GlassPanelGpu {
            center: world_center,
            half_size: Vec3::new(world_half_w, world_half_h, world_half_d),
            corner_radius: panel.corner_radius_world,
            ior: panel.ior,
            tint: panel.tint,
            blur_roughness: panel.blur_roughness,
        });
    }
}
```

### Depth Layering

Panels at different z-depths create a parallax effect when the camera moves:
- **Layer 0**: HUD elements (closest to camera, always visible)
- **Layer 1**: Primary panels (ticket editor, doc viewer)
- **Layer 2**: Secondary panels (search results, settings)
- **Layer 3+**: Background panels (context, overview)

Depth layers are configurable per-component and affect both visual stacking AND ray march evaluation order (closer panels refract first).

### Taffy Integration

```rust
#[derive(Resource)]
pub struct LayoutEngine {
    pub taffy: Taffy,
    pub node_map: HashMap<DioxusElementId, NodeId>,
}

#[derive(Resource)]
pub struct LayoutRects {
    pub panels: Vec<PanelLayout>,
}

pub struct PanelLayout {
    pub rect: PixelRect,          // from Taffy
    pub depth_layer: u32,         // z-layer index
    pub corner_radius_world: f32, // in world units
    pub ior: f32,
    pub tint: Vec4,
    pub blur_roughness: f32,
    pub element_id: DioxusElementId,
}
```

### Dioxus ↔ Bevy Communication

Dioxus runs in the browser's JS event loop. Bevy runs in the WASM render loop. Communication uses shared resources:

```rust
// Dioxus side: update layout tree
fn on_layout_change(element_id: DioxusElementId, style: TaffyStyle) {
    // Post message to shared channel
    LAYOUT_CHANNEL.send(LayoutUpdate { element_id, style });
}

// Bevy system: drain layout updates from Dioxus
fn drain_dioxus_updates_system(
    mut layout: ResMut<LayoutEngine>,
    mut layout_rects: ResMut<LayoutRects>,
) {
    while let Some(update) = LAYOUT_CHANNEL.try_recv() {
        layout.taffy.set_style(layout.node_map[&update.element_id], update.style);
    }
    // Recompute layout
    layout.taffy.compute_layout(root, available_space);
    // Extract rects
    layout_rects.update_from_taffy(&layout);
}
```

### Hit Testing (3D → 2D)

When the user clicks in the 3D viewport, we need to determine which UI panel (if any) was hit:

1. Cast ray from mouse position through camera
2. Evaluate each glass panel SDF along the ray
3. If a panel is hit, map the 3D hit point back to 2D panel-local coordinates
4. Dispatch the click event to the corresponding Dioxus element

```rust
fn hit_test_system(
    mouse: Res<MouseState>,
    camera: Query<&Camera3D>,
    layout_rects: Res<LayoutRects>,
    mut dioxus_events: EventWriter<DioxusClickEvent>,
) {
    let ray = camera.single().screen_to_ray(mouse.position);
    for panel in layout_rects.panels.iter() {
        if let Some(hit) = ray_box_intersection(ray, panel.world_bounds()) {
            let local_uv = panel.world_to_local_uv(hit.point);
            dioxus_events.send(DioxusClickEvent {
                element_id: panel.element_id,
                local_position: local_uv,
            });
            break; // first hit (closest panel)
        }
    }
}
```

## Scope

### Rust: Layout Bridge (`src/ui/bridge.rs`)
- `LayoutEngine` resource (Taffy instance + node map)
- `LayoutRects` resource (computed panel layouts)
- `drain_dioxus_updates_system` (Dioxus → Taffy → LayoutRects)
- `layout_to_3d_system` (LayoutRects → GlassPanelBuffer)

### Rust: Hit Testing (`src/ui/hit_test.rs`)
- `hit_test_system` (ray cast → panel hit → Dioxus event)
- `world_to_local_uv()` for mapping 3D hit to 2D panel coords

### Rust: Communication Channel (`src/ui/channel.rs`)
- Thread-safe channel between Dioxus event loop and Bevy systems

## Dependencies
- T1 (scaffold): Taffy and Dioxus dependencies
- T3 (liquid glass): GlassPanelBuffer + GlassPanelGpu struct
- T6 (3D scene): Camera3D component and ray generation

## Acceptance Criteria
1. Dioxus style changes propagate through Taffy to 3D panel positions within one frame
2. Panel positions in ray-marched scene match the Dioxus layout (visually aligned)
3. Depth layering produces visible parallax when camera orbits
4. Clicking on a glass panel dispatches the event to the correct Dioxus element
5. Clicking on empty space (no panel) does NOT trigger any Dioxus event
6. Resizing the viewport re-computes all panel world positions correctly
7. Adding/removing a Dioxus component adds/removes the corresponding glass SDF
