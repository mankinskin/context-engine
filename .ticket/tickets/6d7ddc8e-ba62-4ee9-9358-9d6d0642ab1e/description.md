# Impl: World editor — Bevy entity placement, transform gizmos, scene serialization

## Problem

The context-editor needs a world editing mode where users can place, move, rotate, and scale 3D objects in the scene, adjust terrain, configure lighting, and save/load world layouts. All objects are **Bevy entities** with Transform and physics components.

## Scope

### Object Placement (`src/editor/world/placement.rs`)
- Toolbar for selecting primitive types (cube, sphere, cylinder, plane)
- Click to place: Rapier ray-cast (T7) → spawn Bevy entity at intersection point
  ```rust
  commands.spawn((
      PbrBundle { mesh, material, transform, ..default() },
      RigidBody::Dynamic,
      Collider::cuboid(0.5, 0.5, 0.5),
      Placed,  // marker component
  ));
  ```
- Ghost preview entity before placement (translucent material)
- Undo/redo stack for entity operations (spawn/despawn/transform change)

### Transform Gizmo (`src/editor/world/gizmo.rs`)
- Select entity by clicking (Rapier ray-cast → entity ID)
- 3-axis translation gizmo (drag arrows) — Bevy entities with custom materials
- Rotation gizmo (drag rings) — stretch goal
- Scale handles — stretch goal
- Gizmo entities track selected entity's `Transform`

### Terrain Editing (`src/editor/world/terrain.rs`)
- Ground plane entity with material from theme
- Grid density adjustment
- Heightmap painting — stretch goal (would use `Collider::heightfield`)

### Lighting Editor (`src/editor/world/lighting.rs`)
- Drag sun direction: move `DirectionalLight` entity indicator
- Add/remove/position `PointLight` entities
- Light color/intensity sliders (integrated with T11 parameter panel)
- Light gizmo entities visible in editor mode

### World Persistence (`src/editor/world/persistence.rs`)
- Serialize Bevy world state to JSON: entity transforms, mesh types, lights, camera
- Bevy's `DynamicScene` or custom serialization for placed entities
- Save to localStorage
- Export/import world files
- Load default world on first launch

## Integration Points
- **Bevy ECS**: all objects are entities with Transform, Mesh, Material, RigidBody, Collider
- **T7 (physics/rapier)**: placed objects have Rapier rigid bodies and colliders, ray-cast for selection
- **T6 (scene)**: objects rendered by Bevy's PBR pipeline
- **T8 (character)**: character navigates among placed entities
- **T11 (params)**: lighting and physics sliders in parameter panel
- **T3 (glass)**: editor toolbar and panels use glass shader

## Files to Create
| File | Purpose |
|------|---------|
| `src/editor/world/mod.rs` | World editor module |
| `src/editor/world/placement.rs` | Entity placement + toolbar |
| `src/editor/world/gizmo.rs` | Transform gizmo entities |
| `src/editor/world/terrain.rs` | Terrain editing |
| `src/editor/world/lighting.rs` | Light entity placement + editing |
| `src/editor/world/persistence.rs` | Bevy scene save/load |

## Acceptance Criteria
1. Click toolbar primitive → click ground → Bevy entity spawned at location
2. Ghost preview entity visible before placement commit
3. Select entity → translation gizmo appears → drag moves entity
4. Sun direction draggable → `DirectionalLight` updates in real time
5. Point light entities addable and positionable in scene
6. World state serializes Bevy entities and loads correctly on refresh
7. Undo reverses last entity operation
8. Export produces valid JSON world file
