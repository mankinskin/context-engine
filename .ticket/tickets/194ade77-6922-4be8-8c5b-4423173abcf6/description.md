# Tiled Forward+ Rasterizer: Tile Binning + Fragment Compositing

## Problem

The final rendering stage: bin sorted Gaussians into 16×16 pixel tiles, then composite them per-pixel with front-to-back alpha blending in a fragment shader. This is where all Gaussians become visible pixels.

## Scope

### Phase 1: Tile Binning (Compute)

Scan sorted keys to find per-tile boundaries:

```wgsl
struct TileData {
    offset: u32,
    count: u32,
}

@compute @workgroup_size(256)
fn build_tiles(@builtin(global_invocation_id) id: vec3u) {
    let idx = id.x;
    let tile_id = sorted_keys[idx] >> 12u;
    let prev_tile = select(0xFFFFFFFFu, sorted_keys[idx - 1u] >> 12u, idx > 0u);
    if tile_id != prev_tile {
        tile_data[tile_id].offset = idx;
    }
    let next_tile = select(0xFFFFFFFFu, sorted_keys[idx + 1u] >> 12u, idx < total_count - 1u);
    if tile_id != next_tile {
        tile_data[tile_id].count = idx - tile_data[tile_id].offset + 1u;
    }
}
```

### Phase 2: Fragment Rasterizer

```wgsl
@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    let tile_x = u32(in.coords.x) / TILE_SIZE;
    let tile_y = u32(in.coords.y) / TILE_SIZE;
    let tile_idx = tile_y * grid_width + tile_x;
    let tile = tile_data[tile_idx];

    var final_color = vec4f(0.0);
    var remaining_alpha = 1.0;

    for (var i = 0u; i < tile.count; i++) {
        let inst = sorted_instances[tile.offset + i];
        let g = projected[inst.gaussian_id];

        let d = in.coords.xy - g.center_screen;
        let power = -0.5 * (d.x * d.x * g.cov2d_inv.x + d.y * d.y * g.cov2d_inv.y + 2.0 * d.x * d.y * g.cov2d_inv.z);
        if power > 0.0 { continue; }

        let alpha = min(0.99, g.opacity * exp(power));
        if alpha < 1.0 / 255.0 { continue; }

        let weight = alpha * remaining_alpha;
        final_color += vec4f(g.color * weight, weight);
        remaining_alpha *= (1.0 - alpha);

        if remaining_alpha < 0.01 { break; } // EARLY-OUT
    }

    return final_color;
}
```

### Early-Out Optimization

When `remaining_alpha < 0.01`, the pixel is saturated — no further Gaussians can contribute visible color. This saves significant fragment work in dense scenes.

### Integration Points

- Glass pre-loop (T3a) inserts before the Gaussian loop, shifting tile lookup
- Fullscreen triangle with no vertex geometry — all work in fragment shader

## Dependencies
- T6c (GPU radix sort): sorted_keys[], sorted_instances[] input
- T2a (GPU buffer infra): tile_data[], projected[] buffers
- T2b (render graph): TileBinNode + TiledRasterNode slots

## Acceptance Criteria
1. Tile binning produces correct per-tile offset/count from sorted data
2. Fragment shader composites Gaussians front-to-back correctly
3. Early-out measurably reduces fragment work (compare with/without)
4. Visual output matches reference: soft splats with correct depth ordering
5. Frame time < 5ms at 1080p, < 8ms at 4K for 1M Gaussians
6. Empty tiles (no Gaussians) render background color
