# Impl: Dioxus-Taffy-Bevy layout bridge — DOM component tree to Bevy ECS to GPU bounding boxes

## Problem

The context-editor needs a bridge that translates Dioxus component trees into Taffy layout nodes, computes pixel-precise bounding boxes in Rust, and streams those positions into **Bevy ECS resources** which Bevy systems then upload to GPU storage buffers for the glass shader and 3D-integrated UI.

## Architecture: Dioxus → Taffy → Bevy → GPU

The full data flow as specified in the architecture:
1. **Dioxus**: defines UI structure (buttons, panels, text) as a component tree
2. **Taffy**: computes exact pixel positions (bounding boxes) from the tree — this happens in Rust, not the browser layout engine
3. **Bevy resources**: layout rects written to `LayoutRects` resource (shared state accessible by any Bevy system)
4. **Bevy system** (`upload_layout_system`): reads `LayoutRects`, writes to GPU storage buffer via `queue.write_buffer`
5. **Glass shader** (T3): reads storage buffer, renders refraction at those positions

Bevy's internal `bevy_ui` also uses Taffy, creating natural compatibility. However our use case is different: we use Taffy to position Dioxus DOM elements and their corresponding GPU glass panels.

## Scope

### Taffy Layout Engine (`src/ui/layout.rs`)
- Create Taffy tree mirroring Dioxus component hierarchy
- Each UI panel/card/window maps to a Taffy node with CSS-like style (size, padding, margin, flex)
- `compute_layout()` → pixel-precise bounding boxes for all nodes
- Recompute only on container resize or UI tree mutation

### Bevy Resource Bridge (`src/ui/bevy_bridge.rs`)
- `LayoutRects` Bevy resource: `Vec<GlassElement>` + dirty flag
- After Taffy computes layout, results are written into this resource
- Communication channel: `Arc<Mutex<>>` or Bevy event system between Dioxus side and Bevy side
- Bevy system reads `LayoutRects` and uploads to GPU storage buffer when dirty

### Layout Rect Tracking (`src/ui/tracker.rs`)
- `LayoutTracker`: maintains mapping of component ID → Taffy node → `LayoutRects` index
- `register_panel(id, style) -> TrackedPanel`
- `update_style(id, new_style)` — triggers relayout
- `remove_panel(id)` — removes from Taffy tree + Bevy resource
- Dirty flag: only upload changed elements per frame

### Dioxus Integration
- Custom Dioxus hook: `use_glass_panel(style) -> GlassPanelRef`
- Returns a DOM node ref for the transparent overlay + registers Taffy node
- Panel GlassElement auto-updates when parent container resizes
- Hook cleanup: unregisters panel on component unmount

## Files to Create
| File | Purpose |
|------|---------|
| `src/ui/layout.rs` | Taffy layout computation |
| `src/ui/bevy_bridge.rs` | Taffy → Bevy `LayoutRects` resource sync |
| `src/ui/tracker.rs` | Layout rect tracking + dirty management |
| `src/ui/hooks.rs` | Dioxus hooks (use_glass_panel) |

## Acceptance Criteria
1. Dioxus components mapped to Taffy nodes compute correct pixel positions
2. Layout rects written to `LayoutRects` Bevy resource match expected positions
3. Bevy system uploads `LayoutRects` to GPU storage buffer correctly
4. Container resize triggers relayout → Bevy resource update → GPU buffer update
5. Dirty tracking prevents redundant GPU uploads (only changed elements uploaded)
6. Component unmount removes element from Bevy resource (no ghost panels)
7. At least 20 simultaneous panels compute layout without frame drops
8. Bounding boxes match visible DOM element positions within 1px tolerance
