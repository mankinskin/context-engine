# World Generation: Procedural Noise SVO, Delta Persistence & Resource Regrowth

## Problem

The open world needs an initial terrain generated procedurally from noise functions. SpacetimeDB stores only player-made modifications (deltas) against the deterministic base terrain. This means: same seed → same world on every client, with deltas layered on top. Resource regrowth runs as a server-side scheduled reducer.

## Architecture: Seed-Deterministic Base + Delta Overlay

### Generation Pipeline

```
World Seed (u64, stored in SpacetimeDB config table)
    ↓
Noise stack: Simplex3D terrain + Worley caves + FBM mountains
    ↓
Per-chunk SVO generation (deterministic, runs on client and server)
    ↓
Apply deltas from VoxelDelta table on top
    ↓
Final local VoxelWorld for rendering
```

### Noise Composition

```rust
pub struct WorldGenerator {
    pub seed: u64,
    pub terrain_noise: Simplex3D,
    pub cave_noise: Worley3D,
    pub mountain_fbm: FractalBrownianMotion,
    pub biome_noise: Simplex2D,
}

impl WorldGenerator {
    pub fn generate_chunk(&self, cx: i32, cy: i32, cz: i32) -> Vec<OctreeNode> {
        let mut nodes = Vec::new();
        for lx in 0..CHUNK_SIZE {
            for ly in 0..CHUNK_SIZE {
                for lz in 0..CHUNK_SIZE {
                    let (wx, wy, wz) = chunk_to_world(cx, cy, cz, lx, ly, lz);
                    let density = self.sample_density(wx, wy, wz);
                    if density > 0.0 {
                        let color = self.sample_material(wx, wy, wz, density);
                        nodes.push(OctreeNode::leaf(color));
                    }
                }
            }
        }
        build_octree_from_leaves(&nodes)
    }

    fn sample_density(&self, x: f32, y: f32, z: f32) -> f32 {
        let terrain = self.terrain_noise.sample(x * 0.01, y * 0.02, z * 0.01);
        let caves = self.cave_noise.sample(x * 0.05, y * 0.05, z * 0.05);
        let mountains = self.mountain_fbm.sample(x * 0.005, z * 0.005) * 80.0;

        let base_height = 64.0 + terrain * 20.0 + mountains;
        let height_density = base_height - y;

        // Carve caves
        let cave_carve = if caves > 0.7 { -1.0 } else { 0.0 };
        height_density + cave_carve
    }

    fn sample_material(&self, x: f32, y: f32, z: f32, density: f32) -> u32 {
        let biome = self.biome_noise.sample(x * 0.002, z * 0.002);
        match biome {
            b if b < -0.3 => MATERIAL_SNOW,
            b if b < 0.2 => MATERIAL_GRASS,
            b if b < 0.6 => MATERIAL_SAND,
            _ => MATERIAL_STONE,
        }
    }
}
```

### Delta Persistence Strategy

- **Base world**: Never stored in DB. Regenerated from seed deterministically.
- **Deltas**: Every player modification becomes a `VoxelDelta` row in SpacetimeDB.
- **Chunk loading**: When client subscribes to a chunk, it: (1) generates base from seed, (2) applies all deltas for that chunk from DB.
- **Storage efficiency**: A 1000×1000×100 world with 1% modification rate stores ~1M delta rows instead of ~100M voxel rows.

### Resource Regrowth (Server Tick)

```rust
#[spacetimedb::reducer(repeat = 60_000ms)]  // every 60 seconds
pub fn regrowth_tick(ctx: &ReducerContext) {
    // Find deltas older than REGROWTH_THRESHOLD ticks
    // For each: check if the original voxel should regrow (trees, ores)
    // If yes: delete the delta (base terrain reasserts)
    // Clients see the voxel "reappear" via subscription update
}
```

### Biome System

```rust
pub enum Biome {
    Forest { tree_density: f32, grass_color: u32 },
    Desert { sand_depth: f32, cactus_chance: f32 },
    Mountain { snow_line: f32, ore_density: f32 },
    Cave { crystal_chance: f32, lava_pockets: bool },
    Ocean { depth: f32, coral_density: f32 },
}
```

Biomes affect:
- Material palette (fed into ThemePalette / SH coefficients for Gaussians)
- Structure placement (trees, rocks, ore veins as pre-computed SVO patches)
- Regrowth rules (forests regrow fast, deserts don't)

### Structure Placement

Pre-defined SVO patches (trees, boulders, ruins) are stamp-placed during generation based on noise-driven probability. These are part of the deterministic base — not stored as deltas.

```rust
pub struct StructureTemplate {
    pub voxels: Vec<(IVec3, u32)>,  // relative positions + colors
    pub anchor: IVec3,               // placement origin
}
```

## Dependencies
- T17 (SpacetimeDB module): World seed table, VoxelDelta table, regrowth tick reducer
- T7a (VoxelWorld API): `set_voxel()` for applying generated terrain + deltas into local octree

## Acceptance Criteria
1. Same seed produces identical terrain on any client (byte-for-byte deterministic)
2. Chunk generation runs in <5ms per 16³ chunk on WASM
3. Deltas layer correctly on top of base terrain
4. Biome boundaries produce smooth material transitions (no hard edges)
5. Structure templates (trees, rocks) place correctly at noise-driven positions
6. Regrowth reducer deletes old deltas, causing base terrain to reassert
7. Storage: 0 rows needed for unexplored base terrain
8. Cave systems carve through terrain without floating artifacts
