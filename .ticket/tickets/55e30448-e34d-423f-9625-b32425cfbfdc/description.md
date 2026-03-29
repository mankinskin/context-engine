# Impl: 3D-integrated UI elements — Bevy entities with glass panels, floating HUD, interaction overlays

## Problem

The context-editor needs UI elements that live within the 3D world: floating glass panels attached to world positions, head-up display (HUD) elements anchored to the camera, and interaction overlays (tooltips, context menus) that respect 3D depth. These are implemented as **Bevy entities** with Transform components, rendered by the glass shader pass.

## Architecture: UI as Bevy Entities

World-space UI panels are Bevy entities:
```rust
commands.spawn((
    WorldPanel,
    GlassPanelMarker,
    Transform::from_xyz(5.0, 2.0, 0.0),
    Billboard,          // always face camera
    PanelContent { ... },
));
```

A Bevy system projects world-space panel positions to screen-space, writes them to `LayoutRects`, and the glass shader renders refraction at those positions. DOM text overlay is positioned using the same screen-space coordinates.

## Scope

### World-Space UI Panels (`src/ui/world_panel.rs`)
- Glass panels as Bevy entities with `Transform` at 3D world coordinates
- `Billboard` component: Bevy system rotates entity to face camera each frame
- World-to-screen projection: Bevy `Camera::world_to_viewport` converts 3D → 2D
- Depth-based occlusion: panels behind objects are dimmed or hidden (depth test)
- Scale attenuation: panels shrink with distance (perspective scaling)

### HUD Elements (`src/ui/hud.rs`)
- Screen-space anchored elements (top bar, minimap, status indicators)
- Not affected by camera movement — positions fixed in screen coordinates
- Rendered as glass panels at fixed screen positions via `LayoutRects`
- Glass shader applies refraction against 3D scene background

### Context Menu (`src/ui/context_menu.rs`)
- Right-click on any UI element or world object
- Spawns a Bevy entity (or updates `LayoutRects` directly) at click position
- Menu items rendered as DOM text over glass background
- Dismiss on click outside or Escape

### Tooltip System (`src/ui/tooltip.rs`)
- Hover over UI elements or world objects to show info
- Positioned near cursor with collision avoidance (stays on screen)
- Fade-in/fade-out animation via glass intensity in Bevy resource

### DOM Text Integration
- All text rendered as DOM elements positioned over GPU glass panels
- Uses `set_text_content` (not innerHTML) for XSS safety
- Font rendering handled by browser (crisp, accessible, SEO-compatible)
- DOM elements receive pointer events; glass panels are `pointer-events: none`

## Files to Create
| File | Purpose |
|------|---------|
| `src/ui/world_panel.rs` | World-positioned Bevy entities with glass panels |
| `src/ui/hud.rs` | Screen-anchored HUD elements |
| `src/ui/context_menu.rs` | Right-click context menu |
| `src/ui/tooltip.rs` | Hover tooltip system |

## Acceptance Criteria
1. Glass panels float at specified 3D world coordinates (Bevy entity Transform)
2. Billboard panels always face the camera regardless of view angle
3. Panels behind scene objects are correctly depth-occluded
4. HUD elements remain stable during camera movement
5. Context menu appears at click position with glass background
6. Tooltip fades in on hover, fades out on mouse leave
7. All text is selectable/accessible DOM content (not GPU-rendered)
8. `Camera::world_to_viewport` correctly projects panel positions to screen space
