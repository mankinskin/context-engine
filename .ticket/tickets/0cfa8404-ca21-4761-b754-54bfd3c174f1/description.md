# Arch: context-editor crate scaffold with Dioxus, Bevy, SVO, Gaussian Splatting, and Trunk

## Problem

The context-editor needs a new crate with the correct dependency stack and module layout to support the full rendering pipeline: SVO for structure/physics, procedural Gaussian splatting for visuals, tiled forward+ compositing, double-buffered GPU sync, and Trunk-based WASM build.

## Architecture Overview

Bevy is the ECS runtime. World geometry lives in a Sparse Voxel Octree uploaded to GPU storage buffers. Visual rendering generates Gaussians from voxels on-the-fly via compute shaders, sorts them with GPU radix sort, and rasterizes via a tiled forward+ fragment pass. Dioxus handles DOM-side UI. Double buffering decouples CPU writes from GPU reads.

## Scope

### Crate Structure
```
tools/context-editor/
├── Cargo.toml
├── Trunk.toml
├── index.html               # GPU canvas + Dioxus root div
├── src/
│   ├── lib.rs                # WASM entrypoint, Bevy App + Dioxus mount
│   ├── app.rs                # Root Dioxus component (DOM overlay)
│   ├── bevy_app.rs           # Bevy App: plugins, systems, render graph
│   ├── svo/
│   │   ├── mod.rs            # SVO module
│   │   ├── octree.rs         # OctreeNode, VoxelWorld, traversal
│   │   └── upload.rs         # Dirty-region GPU buffer upload
│   ├── splat/
│   │   ├── mod.rs            # Gaussian splatting module
│   │   ├── gaussian.rs       # GaussianData struct, SH coefficients
│   │   ├── generator.rs      # SVO-to-Gaussian compute pass logic
│   │   ├── ewa.rs            # EWA projection (Σ' = J·W·Σ·Wᵀ·Jᵀ)
│   │   ├── sort.rs           # GPU radix sort (histogram, scan, scatter)
│   │   └── tiled.rs          # Tiled rasterizer (tile binning, fragment)
│   ├── gpu/
│   │   ├── mod.rs            # Render graph registration
│   │   ├── buffers.rs        # DoubleBuffered<T>, SvoBuffer, SplatBuffer
│   │   ├── bind_groups.rs    # Bind group layouts & double-buffered groups
│   │   └── pipeline.rs       # Compute + render pipeline cache
│   ├── ecs/
│   │   └── mod.rs            # ECS components and systems
│   ├── ui/
│   │   └── mod.rs            # UI module (Dioxus hooks + Taffy bridge)│   ├── server/
│   │   └── mod.rs            # SpacetimeDB module tables, reducers, hooks (T17)
│   ├── net/
│   │   └── mod.rs            # Multiplayer networking, spatial subscriptions (T18)
│   ├── worldgen/
│   │   └── mod.rs            # Procedural noise world generation (T19)
│   ├── combat/
│   │   └── mod.rs            # Combat system, weapon SDFs (T22)
│   ├── skills/
│   │   └── mod.rs            # Skill/magic system, spell SDFs (T23)
│   ├── inventory/
│   │   └── mod.rs            # Voxel inventory, mini-SVO items (T21)
│   ├── llm/
│   │   └── mod.rs            # LLM text-to-voxel/shader integration (T24)│   └── editor/
│       └── mod.rs            # Editor tools module stub
├── shaders/
│   ├── gaussian_gen.wgsl     # SVO → Gaussian emission compute
│   ├── ewa_project.wgsl      # 3D→2D covariance projection
│   ├── radix_sort.wgsl       # Histogram, prefix-sum, scatter
│   ├── tiled_render.wgsl     # Tile-based Gaussian rasterizer + glass
│   └── particles_compute.wgsl
└── static/
```

### Core Data Structures

```rust
// SVO (structure + physics)
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OctreeNode {
    pub child_pointer: u32,  // bits 0-7: child mask, 8-31: first child index
    pub color_data: u32,     // packed RGBA + roughness/metallic
}

#[derive(Resource)]
pub struct VoxelWorld {
    pub nodes: Vec<OctreeNode>,
    pub root_index: u32,
    pub max_depth: u32,
    pub dirty_ranges: Vec<(usize, usize)>,
}

// Gaussian (visual representation, generated on GPU)
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GaussianData {
    pub position: [f32; 3],       // world-space center
    pub opacity: f32,
    pub covariance: [f32; 6],     // upper-triangle of 3×3 Σ matrix
    pub sh_coeffs: [f32; 48],     // 16 SH coefficients × RGB (degree 3)
}

// Projected 2D Gaussian (output of EWA compute pass)
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProjectedGaussian {
    pub center_screen: [f32; 2],
    pub cov2d_inv: [f32; 3],     // inverse 2×2 covariance (symmetric)
    pub depth: f32,
    pub color: [f32; 3],          // SH-evaluated color for current view
    pub opacity: f32,
}

// Double buffering
pub struct DoubleBuffered {
    pub front: wgpu::Buffer,
    pub back: wgpu::Buffer,
    pub current_is_front: bool,
}

impl DoubleBuffered {
    pub fn write_target(&self) -> &wgpu::Buffer {
        if self.current_is_front { &self.back } else { &self.front }
    }
    pub fn read_source(&self) -> &wgpu::Buffer {
        if self.current_is_front { &self.front } else { &self.back }
    }
    pub fn swap(&mut self) { self.current_is_front = !self.current_is_front; }
}
```

### Cargo.toml Key Dependencies
- `bevy` with render features (`bevy_render`, `bevy_core_pipeline`, `bevy_ecs`, `bevy_asset`)
- `bevy_rapier3d` (physics plugin)
- `dioxus` with `web` feature (v0.4+)
- `taffy` (v0.3+)
- `wasm-bindgen`, `wasm-bindgen-futures`
- `web-sys` with WebGPU feature flags
- `bytemuck` with `derive` feature (zero-copy SVO + Gaussian upload)
- `js-sys`
- `spacetimedb-sdk` (SpacetimeDB Rust client — used by T17/T18 for multiplayer backend)
- `naga` (WGSL validation for LLM-generated shaders — T24)
- `noise` (procedural noise functions for world generation — T19)

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
       .init_resource::<SplatParams>()
       .add_systems(Update, (
           svo_dirty_upload_system,      // upload changed octree regions to BACK buffer
           sync_layout_system,           // Taffy → Bevy → SDF uniforms
           upload_uniforms_system,       // resources → GPU
           particle_emitter_system,      // manage emitters
           double_buffer_swap_system,    // swap front/back after upload
       ));
    // Custom render graph: gaussian_gen → ewa_project → radix_sort → tiled_render
    app
}
```

## Files to Create
| File | Purpose |
|------|---------|
| `tools/context-editor/Cargo.toml` | Crate manifest |
| `tools/context-editor/Trunk.toml` | Trunk build config |
| `tools/context-editor/index.html` | WASM entry HTML |
| `tools/context-editor/src/lib.rs` | WASM mount: Bevy + Dioxus |
| `tools/context-editor/src/svo/` | SVO data structures + upload |
| `tools/context-editor/src/splat/` | Gaussian generation, EWA, radix sort, tiled rasterizer |
| `tools/context-editor/src/gpu/` | Double-buffered resources, bind groups, pipelines |
| `tools/context-editor/src/ecs/` | ECS components + systems |
| `tools/context-editor/src/ui/` | Dioxus + Taffy bridge |
| `tools/context-editor/src/editor/` | Editor tools stub |

## Acceptance Criteria
1. `trunk build` produces a working WASM bundle with Bevy + Dioxus initialized
2. `trunk serve` launches in browser with Bevy render loop on canvas
3. `VoxelWorld` resource initialized with a root octree node
4. `DoubleBuffered` SVO buffer created with front/back pair
5. `GaussianData` struct correctly sized at expected byte count (bytemuck)
6. All splatting modules (`splat/`) compile for wasm32-unknown-unknown
7. Dioxus component renders overlay text over Bevy canvas
8. No console errors in browser developer tools
