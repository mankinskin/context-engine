# Impl: Ticket editor — ticket-api CRUD, SSE live updates, 3D dependency graph as Bevy entities

## Problem

The context-editor needs full ticket editing capabilities integrated into the 3D world, connecting to ticket-api for CRUD operations, displaying dependency graphs as 3D node networks (Bevy entities), and receiving live updates via SSE.

## Scope

### Backend Integration (`src/editor/tickets/api.rs`)
- HTTP client for ticket-api endpoints (via viewer-api/ticket-http router):
  - `GET /api/tickets?workspace=...` — list tickets
  - `GET /api/tickets/{id}` — ticket detail
  - `GET /api/tickets/{id}/description` — markdown description
  - `POST /api/tickets` — create ticket
  - `PUT /api/tickets/{id}` — update ticket (state, fields)
  - `GET /api/edges?workspace=...` — dependency edges
  - `GET /api/graph/subgraph?root=...&depth=...` — subgraph BFS
  - `GET /api/stream?workspace=...` — SSE event stream

### SSE Live Updates (`src/editor/tickets/sse.rs`)
- EventSource connection to ticket-api SSE stream
- Handle events: TicketCreated, TicketUpdated, TicketDeleted, EdgeCreated
- Auto-reconnect on disconnect
- Updates reflected in both ticket list panel and 3D dependency graph (Bevy entities)

### Ticket List Panel (`src/editor/tickets/list.rs`)
- Dioxus component: tickets grouped by state (new, in-refinement, ready, etc.)
- Search filter, state filter, sort options (title, updated, created)
- Each ticket as clickable list item with state badge color
- Glass panel background (via Taffy-Bevy bridge)

### Ticket Detail Panel (`src/editor/tickets/detail.rs`)
- Tabbed view: Description (markdown) | Fields (structured) | Edit
- Markdown rendering (pulldown-cmark or lightweight parser)
- Field editing: state transitions, priority, type, component
- Uses `set_text_content` for all user-supplied text (XSS prevention)

### 3D Dependency Graph (`src/editor/tickets/graph3d.rs`)
- Ticket nodes as **Bevy entities**: glass cube meshes + `TicketNode` component
- State color coding via `StandardMaterial` (new=#4a9eff, etc.)
- Dependency edges as 3D line entities connecting node entities
- Force-directed layout calculated in a Bevy system
- Click node → Rapier ray-cast (T7) identifies entity → open ticket detail
- Graph entities updated live when SSE events arrive (spawn/despawn/update entities)

### Workspace Selector
- Dropdown for switching ticket workspaces
- Workspace change despawns graph entities + reloads ticket list + reconnects SSE

## Integration Points
- **ticket-api**: all CRUD + graph query operations
- **viewer-api**: server config, SSE helpers, error envelope patterns
- **Bevy ECS**: graph nodes/edges as entities with Transform + physics
- **T3 (glass)**: ticket panels use liquid glass shader
- **T6 (scene)**: dependency graph rendered in 3D world space (Bevy scene)
- **T7 (physics)**: ray-cast picking via `RapierContext::cast_ray`
- **T9 (Taffy-Bevy bridge)**: ticket list/detail panels positioned via Taffy layout

## Files to Create
| File | Purpose |
|------|---------|
| `src/editor/tickets/mod.rs` | Ticket editor module |
| `src/editor/tickets/api.rs` | ticket-api HTTP client |
| `src/editor/tickets/sse.rs` | SSE live update consumer |
| `src/editor/tickets/list.rs` | Ticket list panel component |
| `src/editor/tickets/detail.rs` | Ticket detail/edit panel |
| `src/editor/tickets/graph3d.rs` | 3D dependency graph (Bevy entities) |

## Acceptance Criteria
1. Ticket list loads from ticket-api and displays grouped by state
2. Search/filter/sort operate correctly
3. Ticket detail shows markdown description + structured fields
4. State transitions work via PUT endpoint with valid state machine transitions
5. SSE updates create/update/delete ticket entities in real time without page reload
6. 3D dependency graph renders as Bevy entities with force-directed layout and state colors
7. Click ticket node in graph → Rapier ray-cast → opens detail panel
8. Workspace switch despawns old entities and loads new data correctly
