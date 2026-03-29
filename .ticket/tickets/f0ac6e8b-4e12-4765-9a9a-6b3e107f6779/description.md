# Gaussian Generation: SVO → GaussianData Compute Shader with LOD and SH

## Problem

The first stage of the Gaussian splatting pipeline: a compute shader traverses occupied SVO leaf voxels and emits GaussianData structs with position, isotropic covariance (size), opacity, and Spherical Harmonic coefficients derived from the voxel's material.

## Scope

### GaussianData Struct

```wgsl
struct GaussianData {
    position: vec3f,
    opacity: f32,
    covariance: array<f32, 6>,  // upper-triangle Σ (3×3 symmetric)
    sh_coeffs: array<f32, 48>,  // 16 SH coefficients × 3 channels (degree 3)
}
```

### Compute Shader: gaussian_gen.wgsl

```wgsl
@compute @workgroup_size(256)
fn generate_gaussians(@builtin(global_invocation_id) id: vec3u) {
    let node_idx = id.x;
    if node_idx >= total_nodes { return; }
    let node = octree[node_idx];
    let child_mask = node.child_pointer & 0xFFu;
    if child_mask != 0u { return; } // skip non-leaf

    let pos = node_position(node_idx);
    let size = voxel_size_at_depth(node_idx);
    let color = unpack_color(node.color_data);
    let roughness = unpack_roughness(node.color_data);
    let metallic = unpack_metallic(node.color_data);

    // LOD: camera distance → Gaussian sharpness
    let cam_dist = length(pos - camera.position.xyz);
    let lod_factor = clamp(cam_dist / lod_scale, 0.5, 4.0);

    let gi = atomicAdd(&gaussian_count, 1u);
    gaussians[gi] = GaussianData(
        pos, 1.0,
        isotropic_covariance(size * lod_factor),
        material_to_sh(color, roughness, metallic),
    );
}
```

### material_to_sh Function

```wgsl
fn material_to_sh(color: vec3f, roughness: f32, metallic: f32) -> array<f32, 48> {
    var sh: array<f32, 48>;
    sh[0] = color.r * 0.282; sh[1] = color.g * 0.282; sh[2] = color.b * 0.282;
    let spec_scale = (1.0 - roughness) * 0.5;
    // Band 1-3: view-dependent highlights scaled by spec_scale
    return sh;
}
```

### LOD Strategy

- Near camera: leaf voxels → many small sharp Gaussians
- Mid distance: stop 1-2 levels early → fewer, larger, fuzzier Gaussians
- Far distance: stop 3+ levels early → chunky soft silhouettes

### Bevy Render Node

```rust
pub struct GaussianGenNode;
impl Node for GaussianGenNode {
    fn run(&self, ...) {
        // Reset gaussian_count atomic to 0
        // Dispatch compute: ceil(total_nodes / 256) workgroups
    }
}
```

## Dependencies
- T2a (GPU buffer infra): SplatBuffers.gaussians, gaussian_count atomic
- T2b (render graph): GaussianGenNode slot in render graph

## Acceptance Criteria
1. Compute shader emits GaussianData for each occupied leaf voxel
2. LOD visible: distant voxels → large fuzzy Gaussians, near → small sharp
3. material_to_sh produces correct SH for diffuse/glossy/metallic materials
4. Atomic counter tracks Gaussian count accurately
5. Non-leaf nodes (with children) are skipped
6. Generation completes in < 1ms for a 256³ voxel world
