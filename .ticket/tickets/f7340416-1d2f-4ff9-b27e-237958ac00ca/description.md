# World: Physics, Voxel Manipulation, and SVO-Rapier Bridge

## Problem

The context-editor world simulation needs physics (gravity, collisions, character movement) and voxel manipulation (add/remove/paint voxels). Physics is handled by bevy_rapier3d, but the world geometry lives in the SVO — so Rapier must derive its collision shapes from the octree, and voxel edits must flow through to both the SVO GPU buffer and Rapier's collision world.

## Architecture: SVO ↔ Rapier Bridge

### Two-Layer Collision

| Layer | Purpose | Data Source |
|-------|---------|-------------|
| **SVO (GPU)** | Particle collision, visual contact | Octree storage buffer |
| **Rapier (CPU)** | Character physics, rigid body dynamics | Collision shapes derived from SVO |

The GPU doesn't use Rapier. Rapier doesn't read the GPU buffer. Both are derived from the authoritative `VoxelWorld` resource on the CPU side.

### Voxel Manipulation API

```rust
impl VoxelWorld {
    /// Set a single voxel. Marks affected region as dirty.
    pub fn set_voxel(&mut self, pos: IVec3, material: VoxelMaterial) {
        let (node_idx, depth) = self.descend_to(pos);
        self.nodes[node_idx].color_data = material.pack();
        self.mark_dirty(node_idx, depth);
    }

    /// Set voxels in a sphere (SDF brush). Returns count of modified voxels.
    pub fn apply_sdf_brush(&mut self, center: Vec3, radius: f32, material: VoxelMaterial) -> u32 {
        let mut count = 0;
        for pos in self.iter_positions_in_sphere(center, radius) {
            self.set_voxel(pos, material);
            count += 1;
        }
        count
    }

    /// Remove voxels in a sphere.
    pub fn carve_sdf_brush(&mut self, center: Vec3, radius: f32) -> u32 {
        let mut count = 0;
        for pos in self.iter_positions_in_sphere(center, radius) {
            self.remove_voxel(pos);
            count += 1;
        }
        count
    }

    /// Dirty-region tracking: byte offset ranges in the node array
    fn mark_dirty(&mut self, node_idx: usize, _depth: u32) {
        let byte_start = node_idx * std::mem::size_of::<OctreeNode>();
        let byte_end = byte_start + std::mem::size_of::<OctreeNode>();
        self.dirty_ranges.push((byte_start, byte_end));
    }

    /// Merge overlapping dirty ranges and return consolidated list
    pub fn take_dirty_ranges(&mut self) -> Vec<(usize, usize)> {
        self.dirty_ranges.sort_by_key(|r| r.0);
        let merged = merge_ranges(&self.dirty_ranges);
        self.dirty_ranges.clear();
        merged
    }
}
```

### GPU Upload: Dirty-Region Partial Buffer Write

Only changed octree regions are uploaded each frame:

```rust
fn svo_upload_system(
    voxel_world: ResMut<VoxelWorld>,
    svo_buffer: Res<SvoBuffer>,
    render_queue: Res<RenderQueue>,
) {
    let ranges = voxel_world.take_dirty_ranges();
    let node_bytes = bytemuck::cast_slice(&voxel_world.nodes);
    for (start, end) in ranges {
        render_queue.write_buffer(
            &svo_buffer.buffer,
            start as u64,           // byte offset into GPU buffer
            &node_bytes[start..end], // only the dirty slice
        );
    }
}
```

**Performance target**: < 1ms for typical edits (painting a 5-radius sphere ≈ ~500 nodes ≈ 4KB upload).

### SVO → Rapier Collision Extraction

Rapier needs CPU-side collision shapes. We extract these from the SVO as **compound colliders** per chunk:

```rust
#[derive(Component)]
pub struct VoxelChunkCollider {
    pub chunk_pos: IVec3,        // chunk grid position
    pub collider: Collider,       // Rapier compound collider
    pub dirty: bool,
}

/// Rebuild Rapier collider for a chunk from SVO data
fn rebuild_chunk_collider(
    voxel_world: &VoxelWorld,
    chunk_pos: IVec3,
    chunk_size: u32,
) -> Collider {
    let mut shapes = Vec::new();
    for pos in voxel_world.iter_occupied_in_chunk(chunk_pos, chunk_size) {
        // Greedy meshing: merge adjacent voxels into larger boxes
        let half = Vec3::splat(0.5 * voxel_world.voxel_size());
        shapes.push((
            Isometry::translation(pos.x as f32, pos.y as f32, pos.z as f32),
            SharedShape::cuboid(half.x, half.y, half.z),
        ));
    }
    Collider::compound(shapes)
}
```

### Chunk-Based Dirty Tracking

The world is divided into chunks (e.g., 16³ voxels each). When voxels change:
1. Mark the chunk as dirty in `VoxelWorld`
2. `svo_upload_system`: pushes dirty octree bytes to GPU (every frame, < 1ms)
3. `rapier_rebuild_system`: rebuilds Rapier colliders for dirty chunks (amortized, can spread over frames)

```rust
fn rapier_rebuild_system(
    voxel_world: Res<VoxelWorld>,
    mut chunks: Query<(&mut VoxelChunkCollider, &mut Collider)>,
) {
    for (mut chunk, mut collider) in chunks.iter_mut() {
        if chunk.dirty {
            *collider = rebuild_chunk_collider(&voxel_world, chunk.chunk_pos, CHUNK_SIZE);
            chunk.dirty = false;
        }
    }
}
```

### Bevy ECS Systems

| System | Schedule | Role |
|--------|----------|------|
| `voxel_edit_system` | `Update` | Process edit commands → `VoxelWorld::set_voxel/apply_sdf_brush` |
| `svo_upload_system` | `PostUpdate` | Dirty regions → GPU `write_buffer` |
| `rapier_rebuild_system` | `PostUpdate` | Dirty chunks → rebuild Rapier colliders |
| `physics_step` | `FixedUpdate` | bevy_rapier3d internal step |

### World Simulation

bevy_rapier3d handles:
- Gravity (configurable, default -9.81 on Y)
- Character controller (kinematic body, see T8)
- Rigid body dynamics for loose objects (future: floating data crystals, physics-driven UI debris)
- Collision events (particle hit, character ground detection)

Rapier configuration:
```rust
app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
   .insert_resource(RapierConfiguration {
       gravity: Vec3::new(0.0, -9.81, 0.0),
       ..default()
   });
```

## Scope

### Rust: VoxelWorld API (`src/svo/`)
- `set_voxel()`, `remove_voxel()`, `apply_sdf_brush()`, `carve_sdf_brush()`
- Dirty-range tracking and merging
- Position iteration (sphere, chunk, AABB)

### Rust: GPU Upload (`src/svo/upload.rs`)
- `svo_upload_system` with partial `write_buffer`
- Buffer resize when octree grows

### Rust: Rapier Bridge (`src/physics/`)
- `VoxelChunkCollider` component
- `rebuild_chunk_collider()` with greedy box merging
- `rapier_rebuild_system` (amortized per-frame)
- Rapier plugin configuration

### Rust: World Sim (`src/physics/`)
- Gravity and physics step configuration
- Collision event reader for gameplay logic

## Dependencies
- T1 (scaffold): VoxelWorld resource and svo/ module
- T6 (3D scene): SVO storage buffer for upload target
- T8 (character): Character controller uses Rapier colliders built here

## Acceptance Criteria
1. `set_voxel()` modifies the octree and marks the region dirty
2. `svo_upload_system` uploads only dirty byte ranges (verify with GPU debug)
3. Dirty upload completes in < 1ms for a 5-radius sphere edit
4. `apply_sdf_brush()` adds voxels in a sphere and all appear in the ray-marched view
5. `carve_sdf_brush()` removes voxels in a sphere (hole visible in scene)
6. Rapier chunk colliders are rebuilt when voxels change
7. Character (if present) stands on voxel surfaces via Rapier collision
8. A rigid body dropped into the scene falls and lands on voxel geometry
