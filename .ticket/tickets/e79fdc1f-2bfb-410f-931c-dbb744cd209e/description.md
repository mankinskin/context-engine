# Design: server-side subgraph query API and pagination semantics

## Objective
Define scalable subgraph query semantics for large ticket dependency graphs.

## Query parameters
- `workspace` (required)
- `root` ticket id (required)
- `direction` in|out|both (default both)
- `edge_kind` depends_on|blocks|all (default all)
- `depth` integer (default 2)
- `limit_nodes` integer (default 500)
- `limit_edges` integer (default 2000)
- `cursor` opaque token (optional)

## Response shape
- `nodes`: array
- `edges`: array
- `truncated`: boolean
- `next_cursor`: optional string
- `stats`: `{ nodes_returned, edges_returned, max_depth_reached }`

## Pagination semantics
- Stable cursor based on deterministic traversal order.
- Cursor invalidation if workspace generation changes materially.
- Client can restart from root when cursor invalid.

## Performance constraints
- Query timeout budget with partial result signaling.
- Guardrail on traversal fan-out to prevent pathological expansions.

## Checklist
- [ ] Parameter contract approved
- [ ] Cursor semantics approved
- [ ] Truncation and timeout behavior approved
- [ ] Complexity guardrails approved
- [ ] Contract tests identified
