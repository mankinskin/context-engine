# Impl: Bevy render loop, WebGPU initialization, and DOM-GPU synchronization bridge

## Problem

The context-editor needs Bevy as the rendering runtime — owning the WebGPU device, render loop, and GPU resource lifecycle. A DOM-GPU bridge synchronizes browser element positions to Bevy ECS resources which are then uploaded to GPU storage buffers.

## Architecture: Bevy Owns the Render Loop

Unlike raw wgpu initialization, **Bevy manages the GPU lifecycle**:
- Bevy's `DefaultPlugins` + `RenderPlugin` initialize wgpu device, adapter, surface
- Bevy's schedule drives the render loop (not a manual `requestAnimationFrame` closure)
- GPU resources (buffers, textures, pipelines) are managed as Bevy render world resources
- Custom render passes (glass, particles) add nodes to Bevy's render graph

The DOM-GPU bridge is the only component that touches web-sys directly — it feeds data into Bevy resources which Bevy systems then upload to GPU.

## Scope

### Bevy WebGPU Setup (`src/gpu/init.rs`)
- Configure Bevy `WindowPlugin` to attach to existing `#gpu-canvas` element
- Set `PresentMode::Fifo` for V-Sync
- Enable `HighPerformance` power preference via Bevy's `WgpuSettings`
- Canvas auto-resize with HiDPI scaling on window resize (Bevy handles this natively)
- Graceful fallback message when WebGPU is unavailable

### Bevy Render Graph Extension (`src/gpu/render_graph.rs`)
- Add custom render graph nodes for: Glass pass, Particle pass, UI overlay pass
- Scene pass → Glass pass → Particle pass → Present (multi-pass pipeline)
- Render-to-texture for glass refraction (intermediate texture matching viewport)

### DOM-GPU Bridge (`src/gpu/bridge.rs`)
- `ResizeObserver` (via web-sys) watches tracked DOM elements
- On resize: extract `getBoundingClientRect()`, normalize to NDC (0.0–1.0)
- Pack into `GlassElement` struct and write to `LayoutRects` Bevy resource
- A Bevy system (`upload_layout_system`) reads `LayoutRects` and writes to GPU storage buffer

### ECS Resources and Components
```rust
// Bevy resource: layout data from Dioxus/Taffy → uploaded to GPU each frame
#[derive(Resource, Default)]
struct LayoutRects {
    elements: Vec<GlassElement>,
    dirty: bool,
}

// Bevy resource: global uniforms (mouse, time, viewport)
#[derive(Resource)]
struct GlobalUniforms {
    mouse_pos: [f32; 2],
    time: f32,
    viewport_size: [f32; 2],
}

// GPU-uploadable structs
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct GlassElement {
    pos: [f32; 2],
    size: [f32; 2],
    intensity: f32,
    _padding: [f32; 3],
}
```

### Mouse Tracking System
- `mousemove` event listener (web-sys) writes mouse position to `GlobalUniforms` resource
- Bypasses Dioxus entirely — direct web-sys → Bevy resource → GPU uniform path
- A Bevy system uploads `GlobalUniforms` to the GPU buffer once per frame

## Reuse from Existing Code
- Port `math3d.rs` from `log-viewer/frontend-leptos/src/gpu/math3d.rs` (Vec3, Mat4 ops)
- Reuse `OverlayContext` callback registration pattern from `log-viewer/frontend-leptos/src/gpu/overlay.rs`

## Files to Create
| File | Purpose |
|------|---------|
| `src/gpu/init.rs` | Bevy WebGPU/window plugin config |
| `src/gpu/render_graph.rs` | Custom render graph nodes |
| `src/gpu/bridge.rs` | DOM element tracking + ResizeObserver → Bevy resource |
| `src/gpu/types.rs` | GPU data structures (GlassElement, GlobalUniforms) |
| `src/gpu/math3d.rs` | 3D math library (ported from log-viewer) |

## Acceptance Criteria
1. Bevy initializes WebGPU successfully in Chrome/Edge (visible clear color on canvas)
2. Bevy render loop runs at monitor refresh rate (measured via Bevy diagnostics)
3. ResizeObserver tracks at least one DOM element and writes its bounding box to `LayoutRects` resource
4. Canvas auto-resizes with HiDPI scaling on window resize
5. Mouse position streams to `GlobalUniforms` resource at frame rate without Dioxus re-render
6. Custom render graph nodes registered (glass, particles) — runs no-op passes without error
7. Graceful fallback message when WebGPU is unavailable
