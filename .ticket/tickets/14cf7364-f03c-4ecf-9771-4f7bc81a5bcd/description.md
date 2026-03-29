# Rendering: Procedural Gaussian Splatting from SVO with EWA Projection and Tiled Forward+ Rasterization

> **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
> Implementation work happens in the children; this ticket tracks overall completion.
>
> **Sub-tickets:**
> - **T6a** Procedural Gaussian Generation from SVO — `f0ac6e8b-4e12-4765-9a9a-6b3e107f6779`
> - **T6b** EWA Splatting Projection — `5070c6b3-a37a-47fa-8dcf-69f805c1a2d2`
> - **T6c** GPU Radix Sort — `cf71418d-038b-4fc1-879d-0a302b681f84`
> - **T6d** Tiled Forward+ Rasterizer — `194ade77-6922-4be8-8c5b-4423173abcf6`
>
> This ticket is done when all four sub-tickets are closed.

---

## Problem

The context-editor world must be rendered with photorealistic quality at 120 FPS. The Sparse Voxel Octree (SVO) provides structure and physics. **Procedural Gaussian Splatting** generates the visual representation on-the-fly from the octree — no pre-baked point clouds needed. Gaussians are projected via EWA, sorted with GPU radix sort, and composited via tiled forward+ rasterization.

## Architecture: Dual-Layer Rendering

### Layer Separation

| Layer | Data | Purpose |
|-------|------|---------|
| **SVO (structure)** | `OctreeNode[]` in storage buffer | Physics collision, voxel editing, octree queries |
| **Gaussians (visual)** | Generated per-frame from SVO | Photorealistic rendering with soft edges, SH lighting, transparency |

The SVO is the single source of truth. Gaussians are ephemeral — regenerated every frame. This means:
- No Gaussian storage on disk (saves GB of VRAM)
- Voxel edits instantly produce new visuals
- LOD is automatic: coarse octree level → large fuzzy Gaussian, leaf level → many small sharp Gaussians

### Phase 1: Gaussian Generation (Compute Shader)

For each occupied voxel in the SVO, emit one or more Gaussians:

```wgsl
struct GaussianData {
    position: vec3f,
    opacity: f32,
    covariance: array<f32, 6>,  // upper-triangle Σ (3×3 symmetric)
    sh_coeffs: array<f32, 48>,  // 16 SH coefficients × 3 channels (degree 3)
}

@compute @workgroup_size(256)
fn generate_gaussians(@builtin(global_invocation_id) id: vec3u) {
    let node_idx = id.x;
    if node_idx >= total_nodes { return; }
    let node = octree[node_idx];
    let child_mask = node.child_pointer & 0xFFu;
    if child_mask != 0u { return; } // skip non-leaf (children handle themselves)

    let pos = node_position(node_idx);
    let size = voxel_size_at_depth(node_idx);
    let color = unpack_color(node.color_data);
    let roughness = unpack_roughness(node.color_data);
    let metallic = unpack_metallic(node.color_data);

    // LOD: distance from camera determines Gaussian sharpness
    let cam_dist = length(pos - camera.position.xyz);
    let lod_factor = clamp(cam_dist / lod_scale, 0.5, 4.0);

    let gi = atomicAdd(&gaussian_count, 1u);
    gaussians[gi] = GaussianData(
        pos,
        1.0,  // full opacity for solid voxels
        isotropic_covariance(size * lod_factor), // larger = fuzzier at distance
        material_to_sh(color, roughness, metallic),
    );
}
```

### Phase 2: EWA Projection (Compute Shader)

Project each 3D Gaussian's covariance matrix to 2D screen-space via EWA:

$$\Sigma' = J \cdot W \cdot \Sigma \cdot W^T \cdot J^T$$

- $W$: view matrix (3×3 rotation part)
- $J$: Jacobian of perspective projection
- $\Sigma$: 3D covariance (from generator)
- $\Sigma'$: 2D covariance for screen-space ellipse

```wgsl
@compute @workgroup_size(256)
fn ewa_project(@builtin(global_invocation_id) id: vec3u) {
    let idx = id.x;
    if idx >= gaussian_count_val { return; }
    let g = gaussians[idx];

    let pos_view = (view_mat * vec4f(g.position, 1.0)).xyz;
    if pos_view.z <= 0.001 { return; } // behind camera

    // Jacobian of perspective projection
    let inv_z = 1.0 / pos_view.z;
    let inv_z2 = inv_z * inv_z;
    let J = mat3x2f(
        focal_x * inv_z, 0.0,
        0.0, focal_y * inv_z,
        -focal_x * pos_view.x * inv_z2, -focal_y * pos_view.y * inv_z2
    );

    // Transform 3D covariance: W * Σ * Wᵀ
    let W = mat3x3f(view_mat[0].xyz, view_mat[1].xyz, view_mat[2].xyz);
    let cov3d = unpack_cov3d(g.covariance);
    let T = W * cov3d * transpose(W);

    // EWA projection: J * T * Jᵀ → 2×2 matrix
    var cov2d = J * T * transpose(J);

    // Low-pass filter (antialiasing): add 0.3 pixel² blur
    cov2d[0][0] += 0.3;
    cov2d[1][1] += 0.3;

    // Invert for fragment shader (V = Σ'⁻¹)
    let det = cov2d[0][0] * cov2d[1][1] - cov2d[0][1] * cov2d[0][1];
    let inv_det = 1.0 / det;

    // SH evaluation: view-dependent color
    let view_dir = normalize(g.position - camera.position.xyz);
    let color = evaluate_sh(g.sh_coeffs, view_dir);

    // Screen-space center
    let screen = vec2f(
        (pos_view.x * focal_x / pos_view.z + 0.5) * resolution.x,
        (pos_view.y * focal_y / pos_view.z + 0.5) * resolution.y,
    );

    projected[idx] = ProjectedGaussian(
        screen,
        vec3f(cov2d[1][1] * inv_det, cov2d[0][0] * inv_det, -cov2d[0][1] * inv_det), // V
        pos_view.z, // depth
        color,
        g.opacity,
    );

    // Sort key: tile_id (20 bits) | depth (12 bits)
    let tile_x = u32(screen.x) / TILE_SIZE;
    let tile_y = u32(screen.y) / TILE_SIZE;
    let tile_id = tile_y * grid_width + tile_x;
    let depth_quantized = u32(clamp(pos_view.z / max_depth * 4095.0, 0.0, 4095.0));
    sort_keys[idx] = (tile_id << 12u) | depth_quantized;
    sort_values[idx] = idx;
}
```

### Phase 3: GPU Radix Sort

Sort by composite key `tile_id | depth` using 8 passes of 4-bit radix sort:

```
For each 4-bit digit (8 passes for 32-bit key):
  1. Histogram: count occurrences of each digit (0-15) per workgroup
  2. Prefix Sum (Blelloch scan): compute global offsets
  3. Scatter: write each element to its new sorted position
```

```wgsl
var<workgroup> local_histogram: array<atomic<u32>, 16>;

@compute @workgroup_size(256)
fn radix_histogram(@builtin(global_invocation_id) id: vec3u) {
    let entry = sort_keys[id.x];
    let digit = (entry >> current_bit_shift) & 0xFu;
    atomicAdd(&local_histogram[digit], 1u);
    workgroupBarrier();
    if id.x % 256u == 0u {
        // Write local histogram to global
        for (var d = 0u; d < 16u; d++) {
            atomicAdd(&global_histograms[d + workgroup_id * 16u], atomicLoad(&local_histogram[d]));
        }
    }
}

@compute @workgroup_size(256)
fn radix_scatter(@builtin(global_invocation_id) id: vec3u) {
    let key = sort_keys[id.x];
    let digit = (key >> current_bit_shift) & 0xFu;
    let dest = offsets[digit] + local_offset;
    sorted_keys[dest] = key;
    sorted_values[dest] = sort_values[id.x];
}
```

**Performance**: GPU radix sort handles 1M elements in < 1ms on modern hardware. Data stays in VRAM for all 8 passes — no CPU round-trip.

### Phase 4: Tile Binning

After sorting, scan the sorted keys to find per-tile boundaries:

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

### Phase 5: Tiled Forward+ Rasterization (Fragment Shader)

```wgsl
@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    let tile_x = u32(in.coords.x) / TILE_SIZE;
    let tile_y = u32(in.coords.y) / TILE_SIZE;
    let tile_idx = tile_y * grid_width + tile_x;
    let tile = tile_data[tile_idx];

    var final_color = vec4f(0.0);
    var remaining_alpha = 1.0;

    // Front-to-back blending
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

        if remaining_alpha < 0.01 { break; } // EARLY-OUT: pixel saturated
    }

    return final_color;
}
```

### Spherical Harmonics Color Evaluation

Each Gaussian stores SH coefficients (degree 3, 16 coefficients per RGB channel = 48 floats). The EWA projection pass evaluates them for the current view direction:

```wgsl
fn evaluate_sh(sh: array<f32, 48>, dir: vec3f) -> vec3f {
    // Band 0 (constant)
    var color = vec3f(sh[0], sh[1], sh[2]) * 0.28209;
    // Band 1 (linear)
    color += vec3f(sh[3], sh[4], sh[5]) * 0.48860 * dir.y;
    color += vec3f(sh[6], sh[7], sh[8]) * 0.48860 * dir.z;
    color += vec3f(sh[9], sh[10], sh[11]) * 0.48860 * dir.x;
    // Band 2 (quadratic) + Band 3 (cubic) — additional terms...
    return max(color, vec3f(0.0));
}
```

**Effect**: Materials look different from different angles — glossy highlights shift with camera, metals reflect environment color.

### LOD: Automatic from Octree Depth

The Gaussian generator controls LOD naturally:
- **Near camera**: traverse to leaf voxels → many small, sharp Gaussians (high detail)
- **Mid distance**: stop 1-2 levels early → fewer, larger, fuzzier Gaussians
- **Far distance**: stop 3+ levels early → chunky silhouettes with soft edges

LOD scaling in the generator:
```wgsl
let lod_factor = clamp(cam_dist / lod_scale, 0.5, 4.0);
// Covariance scaled by lod_factor → larger Gaussian at distance
```

### Performance Targets

| Metric | Target | Method |
|--------|--------|--------|
| Gaussian generation | < 1ms | 1 compute dispatch per occupied voxel |
| EWA projection | < 0.5ms | Per-Gaussian compute, embarrassingly parallel |
| Radix sort (1M Gaussians) | < 1ms | 8 × 4-bit passes, data stays in VRAM |
| Tile binning | < 0.2ms | Single pass over sorted array |
| Tiled rasterization (1080p) | < 5ms | 16×16 tiles, early-out at α < 0.01 |
| Tiled rasterization (4K) | < 8ms | Same with 4× more tiles |
| Total frame (1M Gaussians) | < 8ms | Leaves room for glass + particles |

## Scope

### WGSL Shaders
| Shader | Purpose |
|--------|---------|
| `gaussian_gen.wgsl` | SVO → GaussianData[] with SH coefficients |
| `ewa_project.wgsl` | 3D→2D covariance projection + SH evaluation + sort key |
| `radix_sort.wgsl` | Histogram, prefix-sum (Blelloch scan), scatter |
| `tiled_render.wgsl` | Tile binning + tiled forward+ fragment rasterizer |

### Rust: Bevy Render Nodes
| Node | Type | Role |
|------|------|------|
| `GaussianGenNode` | Compute | Dispatch SVO → Gaussian generation |
| `EwaProjectNode` | Compute | Dispatch EWA projection + sort key build |
| `RadixSortNode` | Compute | 8-pass radix sort dispatch |
| `TileBinNode` | Compute | Build per-tile offset/count |
| `TiledRasterNode` | Fragment | Full-screen tiled Gaussian compositing |

### Rust: ECS Systems
| System | Schedule | Role |
|--------|----------|------|
| `camera_uniform_system` | `Update` | Extract camera matrices → `CameraUniforms` |
| `light_uniform_system` | `Update` | Extract lights → `GlobalUniforms` |
| `svo_upload_system` | `PostUpdate` | Dirty regions → BACK buffer `write_buffer` |
| `double_buffer_swap` | `PostUpdate` | Swap front/back after upload |

## Dependencies
- T1 (crate scaffold): Project structure, svo/ and splat/ modules
- T2 (render init): Render graph, double-buffered SVO buffers, splat buffers

## Acceptance Criteria
1. Procedural Gaussians generated from SVO — no pre-baked point cloud stored
2. EWA projection produces anti-aliased 2D ellipses (no aliasing flicker)
3. SH evaluation shows view-dependent color shifts when camera orbits
4. Radix sort correctly orders Gaussians by tile + depth
5. Tiled rasterizer composites front-to-back with early-out at saturated pixels
6. LOD visible: distant voxels → large fuzzy Gaussians, near → small sharp ones
7. Frame time < 10ms at 1080p for a 256³ voxel world (~1M Gaussians)
8. Early-out optimization measurably reduces fragment work (compare with/without)
9. No Bevy PbrBundle or Camera3dBundle — all rendering is custom Gaussian pipeline
