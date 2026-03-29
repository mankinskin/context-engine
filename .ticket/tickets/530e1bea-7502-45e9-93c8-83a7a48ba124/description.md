# Rendering: Bevy Render Graph and SVO Buffer Initialization

## Problem

Bevy's render loop must be running in WASM with a custom render graph that hosts the SVO ray marching pass, particle compute pass, and glass SDF evaluation. This ticket sets up the GPU infrastructure: canvas acquisition, Bevy renderer initialization, storage buffer creation for the SVO, and render graph wiring.

## Architecture: Bevy Custom Render Graph for SVO

### Render Graph Structure

```
┌─────────────────────────────────────┐
│         Bevy Render Graph           │
│                                     │
│  ┌─────────────┐                    │
│  │ Camera Extract│ (built-in)       │
│  └──────┬──────┘                    │
│         │                           │
│  ┌──────▼──────────┐               │
│  │ Particle Compute │ (custom)      │
│  │ (sim_particles)  │               │
│  └──────┬──────────┘               │
│         │                           │
│  ┌──────▼──────────┐               │
│  │ Ray March Pass   │ (custom)      │
│  │ (SVO + SDFs +    │               │
│  │  lighting +      │               │
│  │  particles)      │               │
│  └──────┬──────────┘               │
│         │                           │
│  ┌──────▼──────────┐               │
│  │ UI Overlay       │ (custom)      │
│  │ (Dioxus text/    │               │
│  │  cursor overlay) │               │
│  └─────────────────┘               │
└─────────────────────────────────────┘
```

### Canvas and WebGPU Setup

Bevy handles WebGPU device creation internally via its `WgpuPlugin`. We configure it to use the HTML canvas:

```rust
app.add_plugins(DefaultPlugins.set(WindowPlugin {
    primary_window: Some(Window {
        canvas: Some("#bevy-canvas".to_string()),
        fit_canvas_to_parent: true,
        prevent_default_event_handling: true,
        ..default()
    }),
    ..default()
}));
```

### SVO Storage Buffer Creation

The octree storage buffer is created during Bevy's render world setup:

```rust
#[derive(Resource)]
pub struct SvoBuffer {
    pub buffer: Buffer,
    pub capacity_nodes: usize,
}

impl SvoBuffer {
    pub fn new(device: &RenderDevice, initial_capacity: usize) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("svo_octree"),
            size: (initial_capacity * std::mem::size_of::<OctreeNode>()) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { buffer, capacity_nodes: initial_capacity }
    }

    pub fn resize_if_needed(&mut self, device: &RenderDevice, needed: usize) {
        if needed > self.capacity_nodes {
            let new_cap = needed.next_power_of_two();
            self.buffer = device.create_buffer(&BufferDescriptor {
                label: Some("svo_octree"),
                size: (new_cap * std::mem::size_of::<OctreeNode>()) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.capacity_nodes = new_cap;
        }
    }
}
```

### Bind Group Layouts

All custom render passes share a common bind group layout:

```rust
// Group 0: SVO data (shared across ray march + particle compute)
@group(0) @binding(0) var<storage, read> octree: array<OctreeNode>;
@group(0) @binding(1) var<uniform> camera: CameraUniforms;
@group(0) @binding(2) var<uniform> globals: GlobalUniforms;

// Group 1: UI glass panels
@group(1) @binding(0) var<storage, read> glass_panels: array<GlassPanel>;
@group(1) @binding(1) var<uniform> glass_count: u32;

// Group 2: Particles
@group(2) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(2) @binding(1) var<uniform> sim_params: SimParams;
```

### Uniform Buffers

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniforms {
    pub view_proj: Mat4,
    pub inv_view_proj: Mat4,
    pub camera_pos: Vec4,  // w = FOV
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalUniforms {
    pub world_size: f32,
    pub max_depth: u32,
    pub time: f32,
    pub lod_scale: f32,
    pub light_count: u32,
    pub _pad: [u32; 3],
    pub lights: [LightData; MAX_LIGHTS],
}
```

### Bevy Plugin Structure

```rust
pub struct ContextEditorRenderPlugin;

impl Plugin for ContextEditorRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SvoBuffer>()
           .init_resource::<GlassPanelBuffer>()
           .init_resource::<ParticleBuffer>()
           .init_resource::<CameraUniforms>()
           .init_resource::<GlobalUniforms>();

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_render_graph_node::<ParticleComputeNode>("particle_compute")
            .add_render_graph_node::<RayMarchNode>("ray_march")
            .add_render_graph_edge("particle_compute", "ray_march");
    }
}
```

## Scope

### Rust: Render Plugin (`src/gpu/plugin.rs`)
- `ContextEditorRenderPlugin` (registers all render resources and graph nodes)
- Bind group layout definitions
- Pipeline cache entries

### Rust: Buffer Management (`src/gpu/buffers.rs`)
- `SvoBuffer` resource (create, resize, partial write)
- `GlassPanelBuffer` resource (uniform buffer for glass SDFs)
- `ParticleBuffer` resource (storage buffer for particles)
- `CameraUniforms`, `GlobalUniforms` resources

### Rust: Render Graph (`src/gpu/graph.rs`)
- Node registration and edge wiring
- Render graph node ordering

### HTML/Config
- `index.html` canvas element with `id="bevy-canvas"`
- Bevy WindowPlugin configuration for WASM

## Dependencies
- T1 (scaffold): Bevy App skeleton and Cargo.toml dependencies

## Acceptance Criteria
1. Bevy render loop runs in WASM, drawing to the HTML canvas
2. SVO storage buffer created with configurable initial capacity
3. Render graph executes: particle compute → ray march → overlay
4. Camera and global uniform buffers update every frame
5. Canvas resizes with the browser window (responsive)
6. No WebGPU validation errors in browser console
7. `SvoBuffer::resize_if_needed` correctly recreates the buffer when capacity exceeded
