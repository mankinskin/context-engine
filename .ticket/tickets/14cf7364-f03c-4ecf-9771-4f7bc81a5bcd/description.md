# Impl: 3D scene renderer — Bevy camera, PBR lighting, depth buffer, multi-pass render graph

## Problem

The context-editor needs a full 3D scene rendering pipeline. Instead of building this from scratch on raw wgpu, we use **Bevy's built-in rendering**: camera entities, PBR materials, directional + point lights, depth buffer, and extend the render graph with custom passes for glass and particles.

## Architecture: Bevy's Render Graph

Bevy provides the 3D scene pipeline out of the box:
- `Camera3dBundle` for perspective camera with built-in depth buffer
- `DirectionalLightBundle`, `PointLightBundle` for lighting
- `PbrBundle` for meshes with PBR materials (StandardMaterial)
- Built-in shadow mapping, ambient occlusion (optional)
- Multi-pass render graph extended with custom nodes: Scene → **Glass** → **Particles** → Present

Benefits over raw wgpu:
- No manual MVP matrix management — Bevy's `Transform` + `GlobalTransform` handle this
- No manual depth buffer creation/resize — Bevy handles it
- No manual render-to-texture for glass input — Bevy's render graph captures scene output

## Scope

### Camera System (Bevy Entity)
- `Camera3dBundle` with orbit behavior: yaw, pitch, distance
- Bevy system: mouse drag to orbit, scroll wheel to zoom, middle-click to pan
- Smooth interpolation (lerp) for camera transitions
- Camera entity updated via Bevy `Query<&mut Transform, With<MainCamera>>`

### Lighting (Bevy Entities)
- `DirectionalLightBundle` (sun) with configurable direction, color, intensity
- Up to 4 `PointLightBundle` entities
- Ambient light via `AmbientLight` resource
- PBR shading handled by Bevy's built-in pipeline (no custom Blinn-Phong needed)
- Shadow mapping via Bevy's built-in shadow pass

### Ground Plane + Scene Objects
- Ground plane: `PbrBundle` with `Plane` mesh + grid material
- Anti-aliased grid lines via custom shader or Bevy's wireframe plugin
- Sky gradient from palette colors (custom background shader or Bevy skybox)

### Multi-Pass Render Graph
1. **Bevy Scene pass** (built-in): Render 3D world with PBR + shadows → output texture
2. **Glass pass** (custom node): Read scene texture, apply liquid glass refraction (T3)
3. **Particle pass** (custom node): Render particles with additive blending (T4)
4. **Present**: Composite final image to swap chain surface

### Render-to-Texture
- Bevy's render graph captures scene pass output as a texture handle
- Glass render node reads this texture for refraction sampling
- Automatically recreated on resize by Bevy

## Reuse from Existing Code
- Port camera orbit math from `log-viewer/frontend-leptos/src/gpu/math3d.rs`
- Port scene3d.wgsl grid rendering from `log-viewer/frontend/src/components/Scene3D/scene3d.wgsl`
- Port palette-driven sky gradient from existing shader code

## Files to Create
| File | Purpose |
|------|---------|
| `src/gpu/camera.rs` | Orbit camera system (Bevy system) |
| `src/gpu/scene.rs` | Scene setup: spawn ground, lights, configure render graph |
| `shaders/grid.wgsl` | Anti-aliased grid material (if not using Bevy wireframe) |

## Acceptance Criteria
1. Orbit camera responds to mouse drag (orbit), scroll (zoom), middle-click (pan)
2. PBR shading produces visible specular highlights on 3D objects
3. Ground plane grid renders with anti-aliased lines
4. Depth buffer correctly occludes objects (no z-fighting)
5. Bevy render graph captures scene output for glass refraction input (T3)
6. Multi-pass pipeline renders scene → glass → particles in correct order
7. Shadows visible from directional light (Bevy built-in shadow pass)
8. Camera position stored as Bevy entity `Transform` — queryable by other systems
