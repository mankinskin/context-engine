# Rendering: SVO Ray Marching Scene Renderer

## Problem

The context-editor world must render a 3D voxel environment with lighting, shadows, and ambient occlusion — all from a **Sparse Voxel Octree (SVO)** traversed via ray marching in a single unified fragment shader. This replaces traditional mesh-based rasterization entirely.

## Architecture: SVO Ray Marching

### Why Not Meshes?

Traditional mesh rendering (PbrBundle, Camera3dBundle, shadow maps) requires:
- Separate geometry passes per object
- Shadow map renders from each light
- No natural LOD for dense environments
- Separate post-process for glass/transparency

SVO ray marching gives us:
- **Single pass**: One full-screen fragment shader does everything
- **Automatic LOD**: Stop octree traversal at coarse depth for distant voxels
- **Free soft shadows**: SDF-style shadow rays through octree distance data
- **Unified transparency**: Glass SDFs integrate into the same ray march loop
- **O(log n) traversal**: Stackless octree descent with bitmask child pointers

### GPU Data Layout

```wgsl
struct OctreeNode {
    child_pointer: u32,  // bits 0-7: child mask, bits 8-31: first child index
    color_data: u32,     // packed RGBA (8 bits each)
}

@group(0) @binding(0) var<storage, read> octree: array<OctreeNode>;
@group(0) @binding(1) var<uniform> camera: CameraUniforms;
@group(0) @binding(2) var<uniform> globals: GlobalUniforms;
```

### Ray Marching Pipeline

```
1. Generate ray from pixel → camera inverse-VP matrix
2. Enter SVO bounding box (ray-AABB intersection)
3. Stackless octree traversal:
   a. Descend to deepest non-empty child along ray
   b. If leaf: record hit (color, normal from gradient, depth)
   c. If empty: advance ray to next sibling via DDA
   d. Repeat until hit or ray exits SVO
4. At hit point:
   a. Compute normal from neighboring voxel occupancy
   b. Shadow ray: march from hit toward each light through octree
      - Accumulate occlusion factor (soft shadows from near-miss distance)
   c. Ambient occlusion: sample 4-8 directions in octree for contact darkness
   d. Final color = albedo × (direct_light × shadow + ambient × AO)
5. If ray hits UI SDF before SVO (glass panel):
   a. Evaluate glass SDF, apply Snell's refraction
   b. Continue ray marching into SVO behind glass
   c. Blend glass tint over SVO result
```

### Bevy Integration

This is a **custom render node** in Bevy's render graph, NOT Bevy's built-in PBR pipeline:

```rust
use bevy::render::render_graph::{Node, RenderGraph};

pub struct RayMarchNode;

impl Node for RayMarchNode {
    fn run(&self, _graph: &mut RenderGraphContext, render_context: &mut RenderContext, world: &World) -> Result<(), NodeRunError> {
        // 1. Bind octree storage buffer
        // 2. Bind camera + global uniforms
        // 3. Bind UI SDF uniforms (panel positions, IOR)
        // 4. Draw full-screen triangle
        // 5. Fragment shader does all ray marching
        Ok(())
    }
}
```

Registered in the render graph after `MainPass3d`:
```rust
render_graph.add_node("ray_march", RayMarchNode);
render_graph.add_node_edge(MAIN_PASS_3D, "ray_march");
```

### Bevy ECS Systems

| System | Schedule | Role |
|--------|----------|------|
| `camera_uniform_system` | `Update` | Extract camera matrices → `CameraUniforms` resource |
| `light_uniform_system` | `Update` | Extract light positions/colors → `GlobalUniforms` |
| `svo_upload_system` | `PostUpdate` | Upload dirty octree regions to GPU storage buffer |
| `ui_sdf_uniform_system` | `PostUpdate` | Pack UI panel SDFs into uniform buffer |

### LOD System

LOD is automatic from the octree structure:
- **Near camera** (< 10 voxel units): traverse to max depth → full detail
- **Mid distance** (10-50): stop 1-2 levels early → 2×-4× larger voxels
- **Far distance** (> 50): stop 3+ levels early → chunky silhouette

The ray marcher decides traversal depth per-ray based on `ray_t` (distance from camera):
```wgsl
let max_depth = clamp(u32(globals.max_depth) - u32(ray_t / lod_scale), 2u, globals.max_depth);
```

### Lighting Model

No shadow maps. All lighting is computed in the ray marching shader:

1. **Diffuse**: Lambert from voxel normal (computed from occupancy gradient)
2. **Soft shadows**: For each light, march a shadow ray through the octree. Track closest-miss distance — smoothstep gives penumbra width.
3. **Ambient occlusion**: At hit point, sample 4-8 octree queries in hemisphere. Ratio of occupied samples gives AO factor.
4. **Specular**: Optional Blinn-Phong from voxel material roughness (stored in `color_data` upper bits)

```wgsl
fn soft_shadow(origin: vec3f, light_dir: vec3f, max_dist: f32) -> f32 {
    var t = 0.01;
    var shadow = 1.0;
    for (var i = 0u; i < 64u; i++) {
        let p = origin + light_dir * t;
        let d = query_svo_distance(p);
        if d < 0.001 { return 0.0; }
        shadow = min(shadow, 8.0 * d / t);
        t += d;
        if t > max_dist { break; }
    }
    return clamp(shadow, 0.0, 1.0);
}
```

### Performance Targets

| Metric | Target | Method |
|--------|--------|--------|
| Ray march time (1080p) | < 8ms | LOD + early termination + bitmask skip |
| Ray march time (4K) | < 10ms | Same + reduced max steps at distance |
| SVO buffer size | < 64MB | 8 bytes/node, ~8M nodes for rich world |
| Octree upload (dirty) | < 1ms | Partial `queue.write_buffer` with byte offset |

## Scope

### WGSL Shader: `ray_march.wgsl`
- Full-screen ray generation from inverse camera matrix
- Stackless octree traversal with bitmask child pointers
- Voxel hit detection and normal computation
- Soft shadow marching
- Ambient occlusion sampling
- UI SDF evaluation and Snell's law refraction (shared with T3)
- LOD depth adjustment per ray
- Final compositing: SVO color + glass tint + lighting

### Rust: Bevy Render Integration
- `RayMarchNode` implementing `bevy::render::render_graph::Node`
- `RayMarchPipeline` (bind group layouts, shader handle, pipeline cache)
- Camera uniform extraction system
- Light uniform extraction system
- SVO storage buffer management (create, resize, partial upload)

### Rust: ECS Components
- `Camera3D` component (position, rotation, FOV — NOT Bevy's Camera3dBundle)
- `PointLight` / `DirectionalLight` components
- `SvoBuffer` resource (GPU buffer handle + size)

## Files to Create/Edit
| File | Purpose |
|------|---------|
| `shaders/ray_march.wgsl` | Unified SVO + SDF ray marching shader |
| `src/gpu/ray_march.rs` | RayMarchNode, pipeline, bind groups |
| `src/gpu/svo_buffer.rs` | GPU buffer create/upload/resize |
| `src/ecs/camera.rs` | Camera3D component + uniform extraction |
| `src/ecs/lighting.rs` | Light components + uniform extraction |
| `src/gpu/mod.rs` | Render graph registration |

## Dependencies
- T1 (crate scaffold): Project structure and svo/ module must exist
- T2 (WebGPU/Bevy init): Bevy render graph must be running

## Acceptance Criteria
1. Full-screen ray marching shader renders a test SVO (e.g., a cube)
2. Camera orbit controls update `CameraUniforms` and scene re-renders
3. At least one point light with soft shadows visible on voxel surface
4. Ambient occlusion darkens concave voxel regions
5. LOD visibly reduces detail at distance (compare near vs far voxels)
6. Frame time < 10ms at 1080p for a 256³ voxel world
7. Octree partial upload works: modify one voxel, only dirty region re-uploaded
8. No Bevy PbrBundle or Camera3dBundle used — all rendering is custom ray marching
