# Plan: context-editor — Unified GPU-Accelerated 3D World Editor

## Problem

The current viewer ecosystem (log-viewer, doc-viewer, ticket-viewer) consists of separate tools with duplicated frontend infrastructure. While Leptos ports and shared viewer-api extraction are underway, the architecture remains fundamentally 2D DOM-based with WebGPU as an overlay layer.

This plan defines a next-generation **context-editor** tool that unifies all viewer/editor functionality into a single GPU-accelerated 3D world with physically accurate rendering, interactive UI elements integrated into 3D space, and full editor capabilities for tickets, documentation, code files, and the context graph.

## Architecture

### Technology Stack
- **Game Engine / ECS Runtime**: Bevy (with `bevy_webgpu` for browser targets, ECS for entity management, built-in render graph)
- **UI Logic**: Dioxus (multi-renderer architecture, strict UI/rendering separation)
- **Layout Engine**: Taffy (pixel-precise CSS-like layout computed in Rust — also used internally by Bevy's `bevy_ui`)
- **GPU Rendering**: wgpu (used by Bevy internally; custom render passes for Liquid Glass and SDF effects)
- **Physics**: bevy_rapier3d (Rapier3D integrated as a Bevy plugin for rigid body dynamics and collision)
- **Build System**: Trunk (Rust → WASM compilation + asset bundling)
- **Backend APIs**: context-api (workspace/search/insert/read), ticket-api (CRUD/graph/SSE)

### Core Pipeline: Dioxus → Taffy → Bevy → GPU
1. **Dioxus** defines UI structure → abstract component tree
2. **Taffy** computes pixel-precise bounding boxes from the tree
3. **Bevy** receives layout data as ECS resources/components; orchestrates the render graph
4. **wgpu** (via Bevy) executes custom render passes: scene → glass → particles → UI overlay
5. **DOM overlay** provides text rendering, accessibility, and click targets

### Why Bevy?
- Native WebGPU support via wgpu (runs identical code in browser and desktop)
- ECS architecture: entities (glass panels, particles, world objects, UI elements) managed as components, systems run per-frame
- Built-in render graph with extensible render passes — custom Liquid Glass and particle passes slot in naturally
- `bevy_rapier3d` plugin provides physics without custom collision code
- Bevy uses Taffy internally for `bevy_ui`, creating natural synergy with the Dioxus → Taffy → Bevy data flow
- Asset management, scene serialization, and hot-reloading built in

### Key Design Decisions
- **Dioxus over Leptos**: Dioxus has native WGPU renderer (Blitz project), strict UI/rendering separation — complements Bevy as the GPU runtime
- **Bevy as rendering runtime**: Dioxus handles DOM-side logic; Bevy owns the render loop, ECS world, and GPU resources (not raw wgpu init)
- **Taffy for layout**: Bridges Dioxus component tree to Bevy entities — layout data streams to Bevy resources which the glass render pass reads
- **SDF-based UI**: Signed Distance Fields for pixel-perfect rounded corners, anti-aliasing, and dynamic shape animation without CSS
- **Direct GPU buffer streaming**: Mouse/interaction events bypass Dioxus, write directly to Bevy resources which upload to GPU uniform buffers

## Existing Infrastructure to Build On
- `viewer-api` — shared HTTP server, tracing, CORS, auth, SSE, dev proxy
- `viewer-api-leptos` — shared Leptos components (TreeView, ResizeHandle, SidebarShell)
- Existing WGSL shaders — particles (4 types), background, hypergraph, scene3d, noise, palette
- `context-api` — workspace manager, search, insert, read, log parser, tracing capture
- `ticket-api` — ticket store (redb + Tantivy), state machine, dependency graph, watcher

## Phases

### Phase 1 — Foundation
- Crate scaffold (Dioxus + Bevy + Taffy + Trunk)
- Bevy app setup with WebGPU renderer, render loop, DOM-GPU bridge

### Phase 2 — Core GPU Rendering
- Liquid Glass shader system (custom Bevy render pass: SDF, refraction, chromatic aberration)
- Particle system (Bevy system + compute shader, instanced rendering)
- Color theme/palette system (Bevy resource + GPU uniforms, live switching)

### Phase 3 — 3D World
- 3D scene renderer (Bevy camera, lighting, PBR materials, multi-pass render graph)
- Physics simulation via bevy_rapier3d + world environment
- Character controls (Bevy system: first/third person, collision via rapier)

### Phase 4 — UI Framework
- Dioxus-Taffy-Bevy layout bridge (DOM → Taffy → Bevy ECS → GPU bounding boxes)
- 3D-integrated UI elements (glass panel entities, floating HUD)
- Parameter manipulation UI

### Phase 5 — Editor Tools
- Ticket editor (ticket-api CRUD, SSE, dependency graph as Bevy entities)
- Documentation editor (markdown, doc-viewer API)
- Context graph editor (context-api, hypergraph as Bevy entities)
- Code file viewer (syntax highlighting)
- World editor (Bevy entity placement, transforms, scene serialization)

## Acceptance Criteria

1. All 16 sub-tickets created with clear scope, acceptance criteria, and dependency edges
2. Phase ordering enforced via depends_on edges
3. Bevy ECS architecture specified in all GPU/rendering/physics tickets
4. Backend integration (context-api, ticket-api) validated in editor tool tickets
5. Existing shader/GPU infrastructure reuse documented in relevant tickets
