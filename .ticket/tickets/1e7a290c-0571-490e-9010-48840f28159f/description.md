# Impl: Liquid Glass shader — custom Bevy render pass with SDF, refraction, chromatic aberration

## Problem

The context-editor needs a physically-inspired glass rendering system where UI panels appear as floating glass surfaces that refract the background (3D scene, video, or image) with realistic edge glow and chromatic aberration. This is implemented as a **custom Bevy render graph node** that reads scene output and glass element positions from ECS resources.

## Architecture: Bevy Render Pass

The Liquid Glass shader runs as a **post-process render graph node** in Bevy's pipeline:
1. Bevy's scene pass renders the 3D world to an intermediate texture
2. The glass render node reads this texture + `GlassElement` storage buffer (from Taffy/Dioxus bridge)
3. It outputs the final composited image with refracted glass panels
4. Bevy's render graph wires: Scene → Glass → Particles → Present

The glass pipeline is a Bevy `RenderNode` implementation, not a standalone wgpu pipeline.

## Scope

### SDF Rounded Rectangle Function
```wgsl
fn sd_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32
```
- Computes pixel-precise distance to rounded rectangle boundary
- Used as mask for glass panels (anti-aliased via smoothstep)
- Replaces CSS `border-radius` with GPU-computed shapes

### Refraction Shader
- Sample background texture at offset UV coordinates to simulate light bending through glass
- Distortion strength varies by distance to panel center (lens effect)
- Mouse-interactive: cursor proximity increases local distortion (liquid warp)
- Background texture = scene pass output (managed by Bevy render graph)

### Chromatic Aberration
- Sample R, G, B channels at slightly different UV offsets
- Creates realistic color fringing at glass edges
- Intensity controllable via uniform

### Edge Glow (Rim Light)
- Smoothstep-based edge detection from SDF distance
- Simulates glass thickness/edge highlight
- Color derived from theme palette (Bevy `ThemePalette` resource)

### Bevy Integration (`src/gpu/glass_pipeline.rs`)
- `GlassRenderNode`: implements `bevy::render::render_graph::Node`
- Reads `LayoutRects` resource for glass element positions/sizes
- Reads scene pass output texture as input binding
- Reads `GlobalUniforms` for mouse position, time
- Manages glass render pipeline (vertex/fragment shader, bind groups)

### Shader File: `shaders/liquid_glass.wgsl`
- Reads `GlassElement` array from storage buffer
- Reads `GlobalUniforms` for mouse position, time, viewport
- Reads background texture (from Bevy scene pass render-to-texture)
- Single-pass: loops over all glass elements per pixel
- Outputs: refracted color + rim light + chromatic aberration

### Integration
- Render graph node registered in Bevy app setup (T1)
- Glass elements populated from Dioxus-Taffy-Bevy bridge (T9)
- Background texture from Bevy 3D scene pass (T6) or solid color fallback

## Reuse from Existing Code
- Port particle-shading.wgsl patterns (color manipulation, smoothstep techniques)
- Reuse palette.wgsl uniform structure for theme colors

## Files to Create
| File | Purpose |
|------|---------|
| `shaders/liquid_glass.wgsl` | SDF + refraction + chromatic aberration |
| `src/gpu/glass_pipeline.rs` | Bevy `RenderNode` + pipeline setup |

## Acceptance Criteria
1. SDF renders pixel-perfect rounded rectangles (no aliasing artifacts)
2. Background refraction visible when glass panel overlaps scene content
3. Chromatic aberration produces visible RGB color fringing at panel edges
4. Mouse proximity causes local "liquid warp" distortion within glass boundaries
5. Edge glow visible as subtle highlight along glass panel borders
6. Multiple glass panels render correctly in single pass
7. `corner_radius` and `intensity` animatable via Bevy resource updates
8. Glass render node correctly wired in Bevy render graph (scene → glass → particles)
