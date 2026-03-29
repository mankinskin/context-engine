# Tools: Voxel World Editor with SDF Brushes

## Problem

The context-editor needs world-building tools for painting, carving, and sculpting the SVO voxel terrain. Users should be able to modify the 3D environment in real-time using SDF brushes that operate on the octree, with changes immediately visible in the ray-marched scene and reflected in Rapier physics.

## Architecture: SDF Brush Tools on SVO

### Brush Types

```rust
#[derive(Clone)]
pub enum BrushTool {
    /// Place voxels in a sphere
    Paint { radius: f32, material: VoxelMaterial },
    /// Remove voxels in a sphere
    Carve { radius: f32 },
    /// Smooth voxel boundaries (average neighboring occupancy)
    Smooth { radius: f32, strength: f32 },
    /// Flatten to a plane
    Flatten { radius: f32, plane_y: f32 },
}
```

### Brush Application Pipeline

```
Mouse click/drag in 3D viewport
        │
        ▼
Ray cast from mouse → hit point on SVO surface
        │
        ▼
Apply brush at hit point:
  Paint  → voxel_world.apply_sdf_brush(center, radius, material)
  Carve  → voxel_world.carve_sdf_brush(center, radius)
  Smooth → average neighbor occupancy in radius
  Flatten → set all voxels above plane_y in radius to empty
        │
        ▼
SVO dirty regions marked automatically
        │
        ▼
svo_upload_system: partial GPU buffer write (< 1ms)
        │
        ▼
rapier_rebuild_system: rebuild affected chunk colliders
        │
        ▼
Next frame: ray marching shows updated voxels + physics updated
```

### Brush System

```rust
#[derive(Resource)]
pub struct ActiveBrush {
    pub tool: BrushTool,
    pub is_active: bool,  // mouse pressed
}

fn brush_system(
    mouse: Res<MouseState>,
    active_brush: Res<ActiveBrush>,
    camera: Query<&Camera3D>,
    mut voxel_world: ResMut<VoxelWorld>,
) {
    if !active_brush.is_active || !mouse.left_pressed { return; }

    // Ray cast to find hit point on SVO
    let ray = camera.single().screen_to_ray(mouse.position);
    let Some(hit) = ray_cast_svo(&voxel_world, ray) else { return; };

    match &active_brush.tool {
        BrushTool::Paint { radius, material } => {
            voxel_world.apply_sdf_brush(hit.point, *radius, material.clone());
        }
        BrushTool::Carve { radius } => {
            voxel_world.carve_sdf_brush(hit.point, *radius);
        }
        BrushTool::Smooth { radius, strength } => {
            smooth_voxels(&mut voxel_world, hit.point, *radius, *strength);
        }
        BrushTool::Flatten { radius, plane_y } => {
            flatten_voxels(&mut voxel_world, hit.point, *radius, *plane_y);
        }
    }
}
```

### CPU Ray Cast for Hit Detection

The brush needs to know WHERE the user is pointing. A CPU-side ray cast through the SVO finds the first occupied voxel:

```rust
fn ray_cast_svo(voxel_world: &VoxelWorld, ray: Ray) -> Option<VoxelHit> {
    // DDA traversal through octree on CPU
    // Similar to GPU ray march but on CPU for interaction
    let mut t = 0.0;
    for _ in 0..256 {
        let p = ray.origin + ray.direction * t;
        let dist = voxel_world.query_distance(p);
        if dist < 0.01 {
            let normal = voxel_world.compute_normal(p);
            return Some(VoxelHit { point: p, normal, distance: t });
        }
        t += dist.max(0.1);
        if t > 500.0 { break; }
    }
    None
}
```

### Real-Time Performance

| Operation | Target | Notes |
|-----------|--------|-------|
| Brush apply (radius 5) | < 0.5ms | ~500 voxels modified |
| SVO dirty upload | < 1ms | Partial buffer write |
| Rapier chunk rebuild | < 2ms | Only affected chunks |
| Total edit-to-visible | < 1 frame | Same frame feedback |

### Brush UI

Dioxus toolbar component (screen-space glass panel) with:
- Brush type selector (paint, carve, smooth, flatten)
- Radius slider
- Material/color picker (from ThemePalette)
- Undo/redo stack for brush operations

### Undo System

```rust
#[derive(Resource)]
pub struct BrushHistory {
    pub undo_stack: Vec<BrushAction>,
    pub redo_stack: Vec<BrushAction>,
}

pub struct BrushAction {
    pub affected_nodes: Vec<(usize, OctreeNode)>, // index + old node value
}

// Undo: restore saved node values, mark dirty
fn undo_brush(history: &mut BrushHistory, voxel_world: &mut VoxelWorld) {
    if let Some(action) = history.undo_stack.pop() {
        let mut redo_nodes = Vec::new();
        for (idx, old_node) in &action.affected_nodes {
            redo_nodes.push((*idx, voxel_world.nodes[*idx]));
            voxel_world.nodes[*idx] = *old_node;
            voxel_world.mark_dirty_node(*idx);
        }
        history.redo_stack.push(BrushAction { affected_nodes: redo_nodes });
    }
}
```

## Scope

### Rust: Brush Tools (`src/editor/brush.rs`)
- `BrushTool` enum (Paint, Carve, Smooth, Flatten)
- `ActiveBrush` resource
- `brush_system` (ray cast → apply → dirty)
- `ray_cast_svo()` (CPU-side octree traversal)

### Rust: Undo System (`src/editor/history.rs`)
- `BrushHistory` resource
- `BrushAction` struct
- `undo_brush` / `redo_brush` functions

### Dioxus: Brush Toolbar (`src/ui/brush_toolbar.rs`)
- Tool selector (paint/carve/smooth/flatten)
- Radius slider
- Material picker
- Undo/redo buttons

## Dependencies
- T7 (physics): `VoxelWorld::apply_sdf_brush` and `carve_sdf_brush` APIs
- T6 (3D scene): Ray marching renders the modified voxels
- T9 (bridge): Brush toolbar as screen-space glass panel
- T5 (theme): Material colors from palette

## Acceptance Criteria
1. Paint brush adds voxels at mouse-pointed SVO surface
2. Carve brush removes voxels, creating visible holes
3. Smooth brush softens sharp voxel edges
4. Flatten brush levels terrain to a plane
5. Changes visible in the same frame (no delay)
6. Rapier physics updated: character can walk on painted terrain
7. Undo reverses the last brush stroke
8. Redo re-applies the undone stroke
9. Brush radius adjustable via UI slider
10. Material color selectable from palette
