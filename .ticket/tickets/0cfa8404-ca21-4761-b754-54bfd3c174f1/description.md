# Arch: context-editor crate scaffold with Dioxus, Bevy, Taffy, and Trunk

## Problem

The context-editor needs a new crate with the correct dependency stack (Dioxus for UI, Bevy for ECS + rendering, Taffy for layout computation) and a Trunk-based build pipeline that compiles Rust to WASM and bundles assets for browser deployment.

## Architecture Decision: Bevy as Runtime

Bevy serves as the core runtime / game engine:
- **ECS world**: All renderable entities (glass panels, particles, world objects, lights) are Bevy entities with components
- **Render graph**: Bevy's render graph manages multi-pass rendering (scene → glass → particles → UI overlay)
- **wgpu access**: Custom render passes (Liquid Glass, particles) slot into Bevy's render graph using its wgpu abstraction
- **Systems**: Per-frame logic (physics update, layout sync, input processing) runs as Bevy systems
- **Resources**: Shared state (theme palette, mouse position, layout rects) stored as Bevy resources

Dioxus handles the DOM-side only: text rendering, events, accessibility. It does NOT own the GPU or render loop — Bevy does.

## Scope

### Crate Structure
```
tools/context-editor/
├── Cargo.toml            # Dependencies: bevy, dioxus, taffy, wasm-bindgen, web-sys, bytemuck
├── Trunk.toml            # Trunk build config (WASM target, asset pipelines)
├── index.html            # Entry HTML with GPU canvas + Dioxus root div
├── src/
│   ├── lib.rs            # WASM entrypoint, Bevy App::build + Dioxus mount
│   ├── app.rs            # Root Dioxus component (DOM overlay)
│   ├── bevy_app.rs       # Bevy App setup: plugins, startup systems, render graph config
│   ├── gpu/
│   │   └── mod.rs        # Custom Bevy render passes (glass, particles)
│   ├── ecs/
│   │   └── mod.rs        # ECS components and systems
│   ├── ui/
│   │   └── mod.rs        # UI component module (Dioxus hooks + Taffy bridge)
│   └── editor/
│       └── mod.rs        # Editor tools module stub
├── shaders/              # WGSL shader files
└── static/               # Static assets
```

### HTML Layout
```html
<div id="main">
  <canvas id="gpu-canvas" style="position: absolute; z-index: 0;"></canvas>
  <div id="dioxus-root" style="position: absolute; z-index: 1;"></div>
</div>
```

### Cargo.toml Key Dependencies
- `bevy` with WebGPU features (`bevy_render`, `bevy_core_pipeline`, `bevy_asset`, `bevy_ecs`)
- `bevy_rapier3d` (physics plugin — added in T7, declared early for workspace compatibility)
- `dioxus` with `web` feature (v0.4+)
- `taffy` (v0.3+)
- `wasm-bindgen`, `wasm-bindgen-futures`
- `web-sys` with WebGPU feature flags (Gpu, GpuDevice, GpuCanvas, etc.)
- `bytemuck` with `derive` feature (zero-copy GPU data upload)
- `js-sys`

### Bevy App Skeleton (`src/bevy_app.rs`)
```rust
pub fn build_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin { ... }))
       .add_plugins(RapierPhysicsPlugin::<NoUserData>::default()) // T7
       .init_resource::<ThemePalette>()   // T5
       .init_resource::<LayoutRects>()    // T9
       .init_resource::<GlobalUniforms>() // T2
       .add_systems(Update, (
           sync_layout_system,    // T9: Taffy → Bevy
           upload_uniforms,       // T2: resources → GPU
           glass_render_system,   // T3: liquid glass pass
           particle_system,       // T4: compute + render
       ));
    app
}
```

### Build Config
- Trunk.toml configured for wasm32-unknown-unknown target
- Release profile with LTO enabled (`opt-level = "s"`)
- Workspace Cargo.toml updated to include context-editor as member

## Files to Create
| File | Purpose |
|------|---------|
| `tools/context-editor/Cargo.toml` | Crate manifest (Bevy + Dioxus + Taffy) |
| `tools/context-editor/Trunk.toml` | Trunk build config |
| `tools/context-editor/index.html` | WASM entry HTML |
| `tools/context-editor/src/lib.rs` | WASM mount: Bevy app + Dioxus DOM |
| `tools/context-editor/src/app.rs` | Root Dioxus component (DOM overlay) |
| `tools/context-editor/src/bevy_app.rs` | Bevy App builder with plugins + systems |
| `tools/context-editor/src/gpu/mod.rs` | Custom Bevy render passes |
| `tools/context-editor/src/ecs/mod.rs` | ECS components + systems |
| `tools/context-editor/src/ui/mod.rs` | UI module stub |
| `tools/context-editor/src/editor/mod.rs` | Editor module stub |

## Files to Modify
| File | Change |
|------|--------|
| Workspace `Cargo.toml` | Add `tools/context-editor` to members |

## Acceptance Criteria
1. `trunk build` produces a working WASM bundle with Bevy + Dioxus initialized
2. `trunk serve` launches in browser with Bevy render loop running on canvas
3. Dioxus component renders "Hello context-editor" text over Bevy canvas
4. Bevy ECS world initializes with at least one startup system confirmed via console log
5. All dependencies resolve and compile for wasm32-unknown-unknown target
6. No console errors in browser developer tools
