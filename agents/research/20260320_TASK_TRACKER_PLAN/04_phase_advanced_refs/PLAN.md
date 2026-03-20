# Phase 3 — Advanced References and Graph Views

**Status:** BLOCKED (requires Phases 1 + 1.5 + 2 complete)

Global progress tracking: `../EXECUTION_CHECKLIST.md`.
Checkboxes in this file are phase-scope deliverable gates.

## Objective

Expose the dependency graph as a first-class query surface. Add cross-ticket
validation overlays, graph traversal commands, and optional visualisation output.

## Problem/Solution/Reference Baseline

1. Problem: deep dependency structures in parallel execution hide blockers and create merge queue chaos.
Solution: graph-native operations, blocker overlays, critical-path computation, and merge-aware scheduling.
Reference: dependency/ready semantics from both beads projects.

2. Problem: agents can unknowingly work on conflicting tickets.
Solution: graph + lease overlays expose active work conflicts and conflict domains.
Reference: Phase 1.5 lease protocol; claim/lease semantics from `delightful-ai/beads-rs`.

## Deliverables

- [ ] `ticket deps <id> --depth <n>` — walk the dependency graph to depth N
- [ ] `ticket blocked-by <id>` — list all tickets blocking a given ticket
- [ ] `ticket blocking <id>` — list all tickets that this ticket blocks
- [ ] `ticket critical-path` — compute longest blocking chain (for scheduling)
- [ ] `ticket validate-graph` — detect cycles, dangling refs, orphaned edges
- [ ] `ticket export-graph --format dot|mermaid|json` — export full dependency graph
- [ ] `ticket board` — terminal-renderable board view grouped by workflow state
- [ ] Validation overlay: surface blocking dependencies and active leases inline in `ticket get`
- [ ] Merge queue helper: `ticket merge-queue next` respects dependency + conflict order
- [ ] HTTP visualization endpoints:
  `GET /api/tickets/graph`, `GET /api/tickets/board`, `GET /api/tickets/critical-path`
- [ ] Auto-generation hooks: emit updated graph/board artifacts on dependency/state changes

## Graph Traversal (redb-backed)

The `EDGES` table from Phase 1 is the source. Traversal algorithms:

```
BFS  — shortest path / reachability queries
DFS  — cycle detection
Topo sort — critical path, scheduling order
```

All operate against in-memory copies of the edge table loaded per-query
(acceptable at <10 000 ticket scale; revisit with a dedicated graph store if scale grows).

## Critical Path Algorithm

- Assign each ticket an estimated effort weight (from `ticket.toml`).
- Run longest-path on the DAG (topological order + relaxation).
- Output: ordered list of tickets with cumulative cost, earliest start, latest start.

## Validation Overlay

On `ticket get <id>`:
```
2a7c59cc-9ab6-4ad4-89cc-4ab9ec7d8f55  Add login page  [in-progress]

  ⚠ Blocked by:
    f07fd470-7d84-4f3d-9f2e-0f9df173f8cb  Design auth flow  [open]
    3181df83-94b7-488d-82aa-616844d83dae  Write JWT lib     [review]

  👷 Active work leases:
    in-progress by agent/refiner-03 (expires 2026-03-20T18:24:00Z)

  -> Earliest unblocked start: when both blockers reach terminal done state
```

## Export Formats

| Format | Command flag | Use case |
|--------|-------------|---------|
| Graphviz DOT | `--format dot` | Render with `dot -Tsvg` |
| Mermaid | `--format mermaid` | Embed in Markdown |
| JSON adjacency list | `--format json` | Machine consumption |

## Key Questions Consumed Here

| Interview Q | Graph impact |
|-------------|-------------|
| Q4 — Dependency edge types | Which edge kinds appear in the graph |
| Q3 — State machine | What "blocked" means in the graph context |
| Q5 — Required fields | Whether `estimated_effort` exists for critical path |
| Q6 — Per-ticket lock | Graph/lease overlays must expose who is actively working |

## Risks

- Large graphs with many edge types make DOT output hard to read; may need layout hints.
- `critical-path` requires `estimated_effort` field to be filled; define fallback behaviour
  when it is absent (assume effort = 1, or skip from critical path calculation).

## TODO

- TODO: Decide whether `ticket export-graph` defaults to stdout or writes a file.
- TODO: Evaluate whether `petgraph` is worth adding as a dependency for graph algorithms,
  or whether bespoke BFS/DFS on the redb edge table is sufficient.
- TODO: Design `ticket board` terminal layout (kanban columns vs. flat list with colour).
- TODO: Define endpoint caching and artifact regeneration policy for large graphs.
