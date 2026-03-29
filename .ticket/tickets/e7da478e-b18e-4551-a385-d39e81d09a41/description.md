# Plan: context-editor — Unified GPU-Accelerated 3D World Editor

## Problem

The current viewer ecosystem (log-viewer, doc-viewer, ticket-viewer) consists of separate tools with duplicated frontend infrastructure. While Leptos ports and shared viewer-api extraction are underway, the architecture remains fundamentally 2D DOM-based with WebGPU as an overlay layer.

This plan defines a next-generation **context-editor** tool that unifies all viewer/editor functionality into a single GPU-accelerated 3D world with physically accurate rendering, interactive UI elements integrated into 3D space, and full editor capabilities for tickets, documentation, code files, and the context graph.

## Architecture

### Technology Stack
- **Game Engine / ECS Runtime**: Bevy (ECS for entity management, render graph orchestration, plugin system)
- **Scene Representation**: Sparse Voxel Octree (SVO) stored as GPU storage buffer — replaces traditional mesh-based rendering
- **Rendering**: Ray Marching through SVO + analytical SDF for UI — single unified shader pipeline
- **UI Logic**: Dioxus (multi-renderer architecture, strict UI/rendering separation)
- **Layout Engine**: Taffy (pixel-precise CSS-like layout computed in Rust)
- **Physics**: bevy_rapier3d (rigid body dynamics, collision against SVO-derived geometry)
- **Build System**: Trunk (Rust → WASM compilation + asset bundling)
- **Backend APIs**: context-api (workspace/search/insert/read), ticket-api (CRUD/graph/SSE)

### Core Rendering Pipeline: SVO Ray Marching + SDF UI

The rendering pipeline unifies the 3D world and UI into a single ray marching pass:

1. **Dioxus** defines UI structure → abstract component tree
2. **Taffy** computes pixel-precise bounding boxes from the tree
3. **Bevy** receives layout data as ECS resources; orchestrates the render graph
4. **Ray Marching shader**: For each pixel, cast a ray that:
   a. Queries analytical **UI SDFs** (glass panels from Taffy layout) — on hit, applies Snell's law refraction and continues
   b. Traverses the **Sparse Voxel Octree** for world geometry — on hit, computes lighting + shadows
   c. Computes **SDF soft shadows** and **ambient occlusion** from distance field data
5. **DOM overlay** provides text rendering, accessibility, and click targets

### Why SVO + Ray Marching?
- **Unified physics**: UI glass panels and 3D world exist in the same mathematical space — glass casts real shadows onto voxels, light refracts through glass physically
- **$O(\log n)$ traversal**: Octree reduces per-pixel cost vs blind ray marching; LOD is automatic (coarse voxels at distance)
- **Minimal CPU-GPU transfer**: WASM manages octree topology; GPU does all rendering. Dirty-region updates mean only changed nodes are uploaded
- **SDF soft shadows**: Free from the distance field — no shadow maps needed
- **Ambient occlusion**: A few extra samples from the octree yield realistic contact shadows

### Why Bevy?
- ECS architecture manages entities (particles, UI elements, world objects, lights) as components
- Render graph orchestrates custom passes: SVO traversal, particle compute, UI overlay
- `bevy_rapier3d` physics plugin provides collision against SVO-derived geometry
- Bevy uses Taffy internally for `bevy_ui`, creating natural synergy with Dioxus → Taffy → Bevy data flow
- Asset management, hot-reloading, and diagnostics built in

### Key Design Decisions
- **SVO over meshes**: Voxel octree enables LOD, efficient ray marching, and unified SDF lighting. Traditional meshes would require separate rasterization and shadow map passes.
- **Ray marching over rasterization**: Single shader handles world + UI + lighting + shadows. No multi-pass compositing needed for glass refraction.
- **Dioxus over Leptos**: Dioxus has native WGPU renderer (Blitz project), strict UI/rendering separation
- **Taffy for layout**: Bridges Dioxus component tree to Bevy resources; layout data projected as 3D box SDFs into ray marching space
- **Analytical SDF UI**: Glass panels are mathematical SDFs in the ray marching loop, not rasterized geometry. Refraction uses Snell's law for physical correctness.

## Existing Infrastructure to Build On
- `viewer-api` — shared HTTP server, tracing, CORS, auth, SSE, dev proxy
- `viewer-api-leptos` — shared Leptos components (TreeView, ResizeHandle, SidebarShell)
- Existing WGSL shaders — particles (4 types), background, hypergraph, scene3d, noise, palette
- `context-api` — workspace manager, search, insert, read, log parser, tracing capture
- `ticket-api` — ticket store (redb + Tantivy), state machine, dependency graph, watcher

## Phases

### Phase 1 — Foundation
- Crate scaffold (Dioxus + Bevy + Taffy + Trunk + SVO data structures)
- Bevy app setup with custom render graph, DOM-GPU bridge

### Phase 2 — Core GPU Rendering
- SVO scene renderer with ray marching, SDF shadows, ambient occlusion, LOD
- Liquid Glass as analytical SDFs in ray marching loop (refraction via Snell's law)
- Particle system with GPU-side SVO collision
- Color theme/palette system (Bevy resource + GPU uniforms)

### Phase 3 — 3D World
- SVO world management: voxel manipulation, dirty-region GPU upload, LOD streaming
- Physics via bevy_rapier3d against SVO-derived collision geometry
- Character controls with SVO-aware collision

### Phase 4 — UI Framework
- Dioxus-Taffy-Bevy bridge: UI layout → 3D box SDFs in ray marching space
- 3D-integrated UI elements (glass panel SDFs, floating HUD)
- Parameter manipulation UI

### Phase 5 — Editor Tools
- Ticket editor (ticket-api CRUD, SSE, dependency graph as Bevy entities)
- Documentation editor (markdown, doc-viewer API)
- Context graph editor (context-api, hypergraph visualization)
- Code file viewer (syntax highlighting)
- World editor (SDF brush voxel manipulation, terrain, lighting)

## Acceptance Criteria

1. All 16 sub-tickets created with clear scope, acceptance criteria, and dependency edges
2. Phase ordering enforced via depends_on edges
3. SVO ray marching architecture specified in all rendering/scene tickets
4. Bevy ECS architecture specified in all GPU/rendering/physics tickets
5. Backend integration (context-api, ticket-api) validated in editor tool tickets
6. Existing shader/GPU infrastructure reuse documented in relevant tickets
