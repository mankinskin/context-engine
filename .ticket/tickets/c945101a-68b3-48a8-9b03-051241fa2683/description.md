# Impl: Context graph editor — context-api integration, hypergraph as Bevy 3D entities

## Problem

The context-editor needs a context graph visualization and editing system that connects to context-api for workspace operations (atoms, sequences, search) and renders the hypergraph as **interactive 3D Bevy entities**.

## Scope

### Backend Integration (`src/editor/graph/api.rs`)
- HTTP client for context-api endpoints (via context-http):
  - `POST /api/execute` — execute context-api commands (add_atoms, insert_sequence, search, read)
  - `GET /api/workspaces` — list workspaces
  - `POST /api/workspaces` — create workspace

### Workspace Manager (`src/editor/graph/workspace.rs`)
- Workspace selector dropdown
- Create new workspace dialog
- Display workspace stats (atom count, vertex count, pattern count)

### Hypergraph 3D View (`src/editor/graph/hypergraph3d.rs`)
- Atoms as **Bevy entities**: sphere mesh + `AtomNode` component + `Transform`
- Edges as **Bevy entities**: tube/line mesh + `GraphEdge` component connecting atom entities
- Force-directed layout in 3D space as a Bevy system
- Color coding: atoms by type, edges by search path status (via `StandardMaterial`)
- Camera orbit around graph center
- Click node → Rapier ray-cast identifies entity → show atom detail

### Search Visualization (`src/editor/graph/search_viz.rs`)
- Execute search queries against context-api
- Highlight matching paths: update `StandardMaterial` on matching atom/edge entities
- Animate search progression through the graph (Bevy system steps through results)
- Display search results in a results panel

### Atom/Sequence Editor (`src/editor/graph/atom_editor.rs`)
- Add atoms to workspace
- Insert sequences (text → character atoms + edges)
- Delete atoms
- View atom details (value, connections, patterns)
- Operations spawn/despawn Bevy entities in the 3D graph

## Integration Points
- **context-api**: workspace manager, search, insert, read operations
- **Bevy ECS**: graph nodes/edges as entities with Transform, materials, physics
- **T6 (scene)**: hypergraph rendered in Bevy 3D scene
- **T7 (physics)**: ray-cast picking via `RapierContext::cast_ray`
- **T4 (particles)**: search result highlight with `ParticleEmitter` bursts
- **T3 (glass)**: editor panels use glass shader
- **T9 (Taffy-Bevy bridge)**: panel layout

## Reuse from Existing Code
- Port hypergraph billboard rendering approach from existing `hypergraph.wgsl`
- Port node/edge instance packing from `hypergraph_gpu.rs`
- Port palette color encoding from existing node coloring scheme

## Files to Create
| File | Purpose |
|------|---------|
| `src/editor/graph/mod.rs` | Context graph editor module |
| `src/editor/graph/api.rs` | context-api HTTP client |
| `src/editor/graph/workspace.rs` | Workspace management |
| `src/editor/graph/hypergraph3d.rs` | 3D hypergraph as Bevy entities |
| `src/editor/graph/search_viz.rs` | Search result visualization |
| `src/editor/graph/atom_editor.rs` | Atom/sequence CRUD |

## Acceptance Criteria
1. Workspace list loads from context-api
2. Atoms render as Bevy entities with sphere meshes and force-directed layout
3. Edges render as Bevy entities connecting atom nodes
4. Search query highlights matching paths with distinct material color
5. Add atom / insert sequence creates data via context-api and spawns Bevy entities
6. Click atom → Rapier ray-cast → detail panel with connections
7. Graph entities auto-update after insert/delete operations
