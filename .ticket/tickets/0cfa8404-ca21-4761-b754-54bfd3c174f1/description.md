# Arch: context-editor crate scaffold with Dioxus, Bevy, Taffy, SVO, and Trunk

## Problem

The context-editor needs a new crate with the correct dependency stack (Dioxus for UI, Bevy for ECS + render graph, Taffy for layout, SVO data structures for voxel world) and a Trunk-based build pipeline that compiles Rust to WASM and bundles assets for browser deployment.

## Architecture Decision: Bevy + SVO

Bevy serves as the ECS runtime and render graph orchestrator. The 3D world is represented as a **Sparse Voxel Octree (SVO)** stored in a GPU storage buffer, rendered via ray marching — not traditional mesh rasterization.

- **ECS world**: Entities include lights, particle emitters, UI panels, character — NOT mesh bundles for world geometry
- **SVO**: World geometry lives in a flat `Vec<OctreeNode>` uploaded to GPU. Bevy systems manage octree topology and dirty-region uploads.
- **Render graph**: Custom ray marching pass traverses SVO + analytical UI SDFs in a single unified shader
- **Dioxus**: Handles DOM-side only (text, events, accessibility)

## Scope

### Crate Structure
```
tools/context-editor/
├── Cargo.toml
├── Trunk.toml
├── index.html            # GPU canvas + Dioxus root div
├── src/
│   ├── lib.rs            # WASM entrypoint, Bevy App + Dioxus mount
│   ├── app.rs            # Root Dioxus component (DOM overlay)
│   ├── bevy_app.rs       # Bevy App: plugins, systems, render graph
│   ├── svo/
│   │   ├── mod.rs        # SVO module
│   │   ├── octree.rs     # OctreeNode, VoxelWorld, traversal
│   │   └── upload.rs     # Dirty-region GPU buffer upload
│   ├── gpu/
│   │   ├── mod.rs        # Custom Bevy render passes
│   │   └── ray_march.rs  # Ray marching render node
│   ├── ecs/
│   │   └── mod.rs        # ECS components and systems
│   ├── ui/
│   │   └── mod.rs        # UI module (Dioxus hooks + Taffy bridge)
│   └── editor/
│       └── mod.rs        # Editor tools module stub
├── shaders/
│   ├── ray_march.wgsl    # Unified SVO traversal + SDF UI + lighting
│   └── particles_compute.wgsl
└── static/
```

### SVO Data Structures (`src/svo/`)
```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OctreeNode {
    /// Bit 0-7: child mask (which of 8 children exist)
    /// Bit 8-31: index to first child in global buffer
    pub child_pointer: u32,
    /// Packed RGBA color + roughness/metallic
    pub color_data: u32,
}

#[derive(Resource)]
pub struct VoxelWorld {
    pub nodes: Vec<OctreeNode>,
    pub root_index: u32,
    pub max_depth: u32,
    pub dirty_ranges: Vec<(usize, usize)>, // byte offset ranges to upload
}
```

### Cargo.toml Key Dependencies
- `bevy` with render features (`bevy_render`, `bevy_core_pipeline`, `bevy_ecs`, `bevy_asset`)
- `bevy_rapier3d` (physics plugin)
- `dioxus` with `web` feature (v0.4+)
- `taffy` (v0.3+)
- `wasm-bindgen`, `wasm-bindgen-futures`
- `web-sys` with WebGPU feature flags
- `bytemuck` with `derive` feature (zero-copy SVO upload)
- `js-sys`

### Bevy App Skeleton (`src/bevy_app.rs`)
```rust
pub fn build_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin { ... }))
       .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
       .init_resource::<VoxelWorld>()
       .init_resource::<ThemePalette>()
       .init_resource::<LayoutRects>()
       .init_resource::<GlobalUniforms>()
       .add_systems(Update, (
           svo_dirty_upload_system,   // upload changed octree regions
           sync_layout_system,        // Taffy → Bevy → SDF uniforms
           upload_uniforms_system,    // resources → GPU
           particle_emitter_system,   // manage emitters
       ));
    // Custom render graph: ray_march_node, particle_render_node
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
| `tools/context-editor/Cargo.toml` | Crate manifest |
| `tools/context-editor/Trunk.toml` | Trunk build config |
| `tools/context-editor/index.html` | WASM entry HTML |
| `tools/context-editor/src/lib.rs` | WASM mount: Bevy + Dioxus |
| `tools/context-editor/src/app.rs` | Root Dioxus component |
| `tools/context-editor/src/bevy_app.rs` | Bevy App builder |
| `tools/context-editor/src/svo/mod.rs` | SVO module |
| `tools/context-editor/src/svo/octree.rs` | OctreeNode, VoxelWorld |
| `tools/context-editor/src/svo/upload.rs` | Dirty-region GPU upload |
| `tools/context-editor/src/gpu/mod.rs` | Custom render passes |
| `tools/context-editor/src/gpu/ray_march.rs` | Ray marching render node |
| `tools/context-editor/src/ecs/mod.rs` | ECS components + systems |
| `tools/context-editor/src/ui/mod.rs` | UI module stub |
| `tools/context-editor/src/editor/mod.rs` | Editor module stub |

## Acceptance Criteria
1. `trunk build` produces a working WASM bundle with Bevy + Dioxus initialized
2. `trunk serve` launches in browser with Bevy render loop running on canvas
3. `VoxelWorld` resource initialized with a root octree node
4. Dioxus component renders "Hello context-editor" text over Bevy canvas
5. Bevy ECS world starts with SVO upload system registered
6. All dependencies resolve and compile for wasm32-unknown-unknown target
7. No console errors in browser developer tools
