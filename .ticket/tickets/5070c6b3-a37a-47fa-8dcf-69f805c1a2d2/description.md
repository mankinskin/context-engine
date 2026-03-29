# EWA Projection: 3D→2D Covariance, SH Evaluation, and Sort Key Construction

## Problem

The second stage of the Gaussian pipeline: project each 3D Gaussian's covariance to a 2D screen-space ellipse via EWA splatting, evaluate Spherical Harmonics for view-dependent color, and construct composite sort keys for the radix sort.

## Scope

### EWA Math

$$\Sigma' = J \cdot W \cdot \Sigma \cdot W^T \cdot J^T$$

- $W$: view matrix (3×3 rotation)
- $J$: Jacobian of perspective projection
- $\Sigma$: 3D covariance from generator
- $\Sigma'$: 2D covariance → screen-space ellipse

### Compute Shader: ewa_project.wgsl

```wgsl
@compute @workgroup_size(256)
fn ewa_project(@builtin(global_invocation_id) id: vec3u) {
    let idx = id.x;
    if idx >= gaussian_count_val { return; }
    let g = gaussians[idx];

    let pos_view = (view_mat * vec4f(g.position, 1.0)).xyz;
    if pos_view.z <= 0.001 { return; } // behind camera

    let inv_z = 1.0 / pos_view.z;
    let J = mat3x2f(
        focal_x * inv_z, 0.0,
        0.0, focal_y * inv_z,
        -focal_x * pos_view.x * inv_z * inv_z, -focal_y * pos_view.y * inv_z * inv_z
    );

    let W = mat3x3f(view_mat[0].xyz, view_mat[1].xyz, view_mat[2].xyz);
    let cov3d = unpack_cov3d(g.covariance);
    let T = W * cov3d * transpose(W);
    var cov2d = J * T * transpose(J);

    // Low-pass anti-aliasing filter: +0.3 pixel²
    cov2d[0][0] += 0.3;
    cov2d[1][1] += 0.3;

    let det = cov2d[0][0] * cov2d[1][1] - cov2d[0][1] * cov2d[0][1];
    let inv_det = 1.0 / det;

    // SH color evaluation
    let view_dir = normalize(g.position - camera.position.xyz);
    let color = evaluate_sh(g.sh_coeffs, view_dir);

    let screen = vec2f(
        (pos_view.x * focal_x / pos_view.z + 0.5) * resolution.x,
        (pos_view.y * focal_y / pos_view.z + 0.5) * resolution.y,
    );

    projected[idx] = ProjectedGaussian(
        screen,
        vec3f(cov2d[1][1] * inv_det, cov2d[0][0] * inv_det, -cov2d[0][1] * inv_det),
        pos_view.z, color, g.opacity,
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

### SH Evaluation Function

```wgsl
fn evaluate_sh(sh: array<f32, 48>, dir: vec3f) -> vec3f {
    var color = vec3f(sh[0], sh[1], sh[2]) * 0.28209;
    color += vec3f(sh[3], sh[4], sh[5]) * 0.48860 * dir.y;
    color += vec3f(sh[6], sh[7], sh[8]) * 0.48860 * dir.z;
    color += vec3f(sh[9], sh[10], sh[11]) * 0.48860 * dir.x;
    // Band 2 + Band 3 terms...
    return max(color, vec3f(0.0));
}
```

## Dependencies
- T6a (Gaussian generation): GaussianData[] input
- T2a (GPU buffer infra): projected[], sort_keys[], sort_values[]

## Acceptance Criteria
1. EWA projection produces 2D inverse covariance for screen-space ellipses
2. Low-pass filter (+0.3px²) prevents aliasing flicker
3. SH evaluation shows view-dependent color shift when camera orbits
4. Sort keys encode tile_id (20 bits) and depth (12 bits) correctly
5. Behind-camera Gaussians are culled
6. Projection completes in < 0.5ms for 1M Gaussians
