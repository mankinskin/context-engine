# Phase 4 — Advanced References and Graph Views

**Status:** BLOCKED (requires Phases 1 + 2 + 3 complete)

## Objective

Expose the dependency graph as a first-class query surface. Add cross-ticket
validation overlays, graph traversal commands, and optional visualisation output.

## Deliverables

- [ ] `task deps <id> --depth <n>` — walk the dependency graph to depth N
- [ ] `task blocked-by <id>` — list all tickets blocking a given ticket
- [ ] `task blocking <id>` — list all tickets that this ticket blocks
- [ ] `task critical-path` — compute longest blocking chain (for scheduling)
- [ ] `task validate-graph` — detect cycles, dangling refs, orphaned edges
- [ ] `task export-dot` — emit Graphviz DOT for the full dependency graph
- [ ] `task board` — terminal-renderable board view grouped by status
- [ ] Validation overlay: surface blocking dependencies inline in `task get` output

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

On `task get <id>`:
```
TCK-00042  Add login page      [in-progress]

  ⚠ Blocked by:
    TCK-00038  Design auth flow  [open]       ← not started yet
    TCK-00039  Write JWT lib     [review]     ← almost done

  → Earliest unblocked start: when TCK-00038 AND TCK-00039 reach [done]
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

## Risks

- Large graphs with many edge types make DOT output hard to read; may need layout hints.
- `critical-path` requires `estimated_effort` field to be filled; define fallback behaviour
  when it is absent (assume effort = 1, or skip from critical path calculation).

## TODO

- TODO: Decide whether `task export-dot` pipes to stdout or writes a file.
- TODO: Evaluate whether `petgraph` is worth adding as a dependency for graph algorithms,
  or whether bespoke BFS/DFS on the redb edge table is sufficient.
- TODO: Design `task board` terminal layout (kanban columns vs. flat list with colour).
