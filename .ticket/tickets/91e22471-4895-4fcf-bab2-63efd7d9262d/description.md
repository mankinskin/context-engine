# Plan: Ticket Viewer + Ticket HTTP Server + Shared Viewer Infrastructure

## Goal
Create a dedicated ticket-viewer (derived from doc-viewer structure), add HTTP server mode to `ticket` for live updates, and render dependency/state topology using the hypergraph display approach from log-viewer. Reuse and centralize shared server/viewer logic in `viewer-api`.

## Requested direction
- Reuse doc-viewer file tree + file display interactions for ticket browsing.
- Reuse log-viewer graph presentation patterns for dependency topology.
- Add `ticket` HTTP server mode with watch/reconcile updates.
- Drive frontend graph from live ticket state + edge changes.
- Start with baseline active-state styling and incrementally refine.

## Scope proposal
- In scope:
  - New `tools/ticket-viewer/` crate and frontend shell.
  - `ticket serve` (or equivalent) HTTP mode in context-tasks tool.
  - Streaming/refresh API for ticket manifests + edges + active state.
  - Shared endpoint wiring/middleware/utilities in `tools/viewer-api/`.
  - Hypergraph rendering of dependencies and active states.
- Out of scope (first increment):
  - Advanced timeline playback.
  - Deep workflow analytics and forecasting.
  - Complex role-based authorization model.

## Interview Round 1 (critical questions)
1. Product boundary
- Is `ticket-viewer` a standalone app under `tools/`, or a mode/route inside an existing viewer server?
- Should it ship with both local dev server and static build target in v1?

2. Runtime and serving model
- Should `ticket` tool host the HTTP API directly, while `ticket-viewer` hosts static assets only?
- Preferred transport for updates in v1: polling, SSE, or WebSocket?

3. Live-update semantics
- What freshness target is acceptable (e.g., <500ms, <2s, best-effort)?
- Should updates be eventual and coalesced, or strict per-event ordering?

4. Data contract
- Canonical graph node identity: ticket UUID only, or UUID + workspace scope?
- Which edge kinds are first-class in graph v1 (`depends_on`, `blocks`, `linked`)?
- Required state fields for node styling in v1 (e.g., `state`, `validation_status`, lease status)?

5. UX and interaction priorities
- Must-have interactions in v1: expand/collapse, filter by state, search, click-to-open ticket files, pin subgraph?
- Should file tree open `description.md` and `ticket.toml` side-by-side or tabbed?

6. Reuse architecture
- Which parts from doc-viewer are mandated for reuse: route layout, tree component, markdown/file renderer, API contract?
- Which parts from log-viewer hypergraph are reusable as-is vs intentionally forked?

7. Multi-workspace behavior
- Single active workspace only, or switch among workspaces from UI?
- Should graph include cross-workspace links if they exist?

8. Change detection source of truth
- Should server watch filesystem directly, rely on ticket command hooks, or both?
- How should deletions/renames be represented in live graph updates?

9. Performance and scale envelope
- Expected scale in v1 (ticket count, edge count, update rate)?
- Is graph virtualization required immediately?

10. Security and exposure
- Is HTTP mode localhost-only by default?
- Any auth/token requirement for non-local environments in v1?

11. Incremental styling plan
- Define minimum state color/style mapping for v1.
- Which states need distinct visual prominence from day one (`in-progress`, `blocked`, `review`, `validating`)?

12. Definition of done for parent ticket
- What exact acceptance criteria must be met before opening child implementation tickets?

## Proposed child tracks after interview refinement
- Track A: `ticket serve` API + watcher stream pipeline.
- Track B: `ticket-viewer` shell + file tree/file display reuse.
- Track C: hypergraph integration for dependencies + state styling.
- Track D: viewer-api extraction/unification.
- Track E: integration tests and sample workspace scenarios.

## Risks to clarify early
- Divergent frontend patterns between doc-viewer/log-viewer causing costly merges.
- Event model mismatch between ticket storage updates and graph UI expectations.
- Premature over-generalization in viewer-api.

## Interview Round 2 final decisions (canonical)
- Auth: static bearer token from env/config (v1).
- SSE: best-effort live only, no replay buffer (v1).
- Watch/reconcile source: hybrid, with command hooks as primary.
- Concurrency: optimistic version checks with explicit conflict events.
- Graph layout: deterministic DAG layering by dependency depth.
- Large-graph UX baseline: virtualization, clustering/collapsing, debounced filter/search, server-side subgraph queries.
- File panel default: open `description.md` first; `ticket.toml` as tab switch.
- Workspace switching: preserve per-workspace UI state.
- API shape: no endpoint versioning in v1.
- Parent done gate: design + API contract + wireframes + child tickets.

## Review handoff
- Design-to-implementation mapping checklist: `assets/design/design-review-checklist-v0.1.md`
