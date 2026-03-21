# Subgraph API Semantics v0.1

Status: draft for review

## Endpoint
`GET /api/graph/subgraph`

## Query parameters
- `workspace` required
- `root` required (ticket id)
- `direction` optional: `in|out|both` (default `both`)
- `edge_kind` optional: `depends_on|blocks|all` (default `all`)
- `depth` optional int (default 2, max 8)
- `limit_nodes` optional int (default 500, max 5000)
- `limit_edges` optional int (default 2000, max 20000)
- `cursor` optional opaque token

## Traversal semantics
- Deterministic BFS layering by depth.
- Node ordering stable by `(depth, ticket_id)`.
- Edge ordering stable by `(from, to, kind)`.

## Pagination
- `next_cursor` contains workspace generation + traversal checkpoint.
- Cursor invalid if workspace generation changes.
- Invalid cursor returns `409 graph.cursor_invalid` and restart hint.

## Response
```json
{
  "request_id": "...",
  "workspace": "default",
  "nodes": [],
  "edges": [],
  "truncated": false,
  "next_cursor": null,
  "stats": {
    "nodes_returned": 0,
    "edges_returned": 0,
    "max_depth_reached": 0
  }
}
```

## Performance limits
- Query timeout budget: 2s default.
- On timeout, return partial graph with `truncated=true` and cursor.
- Enforce fan-out guardrail to prevent pathological expansions.
