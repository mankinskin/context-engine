# Ticket Serve API Contract v0.1

Status: draft for review
Scope: v1 ticket-viewer backend integration

## Authentication
- Required on all `/api/*` endpoints.
- Header: `Authorization: Bearer <token>`.
- 401 when header missing/invalid.
- 403 when token valid but unauthorized for requested workspace (reserved for future policy).

## Common response metadata
- `request_id`: string
- `workspace`: string (when applicable)
- `ts`: RFC3339 timestamp

## Error envelope
```json
{
  "code": "auth.invalid_token",
  "message": "Bearer token is invalid",
  "request_id": "...",
  "details": {}
}
```

## Endpoints

### GET /healthz
- Auth: no
- Response:
```json
{
  "status": "ok",
  "service": "ticket-serve"
}
```

### GET /api/workspaces
- Auth: yes
- Response:
```json
{
  "request_id": "...",
  "items": [
    { "name": "default", "active": true }
  ]
}
```

### GET /api/tickets
Query params:
- `workspace` (required)
- `state` (optional)
- `query` (optional)
- `limit` (optional, default 100, max 1000)
- `cursor` (optional)

Response:
```json
{
  "request_id": "...",
  "workspace": "default",
  "items": [
    {
      "id": "uuid",
      "type": "tracker-improvement",
      "title": "...",
      "state": "in-progress",
      "updated_at": "2026-03-21T00:00:00Z",
      "fields": {}
    }
  ],
  "next_cursor": null
}
```

### GET /api/tickets/{id}
Query params:
- `workspace` (required)

Response:
```json
{
  "request_id": "...",
  "workspace": "default",
  "ticket": {
    "id": "uuid",
    "created_at": "...",
    "fields": {}
  }
}
```

### GET /api/edges
Query params:
- `workspace` (required)
- `kind` (optional: `depends_on|blocks|linked|all`, default `all`)

Response:
```json
{
  "request_id": "...",
  "workspace": "default",
  "items": [
    { "from": "uuid", "to": "uuid", "kind": "depends_on" }
  ]
}
```

### GET /api/graph/subgraph
Query params:
- `workspace` (required)
- `root` (required)
- `direction` (optional: `in|out|both`, default `both`)
- `edge_kind` (optional: `depends_on|blocks|all`, default `all`)
- `depth` (optional, default 2)
- `limit_nodes` (optional, default 500)
- `limit_edges` (optional, default 2000)
- `cursor` (optional)

Response:
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

### GET /api/stream (SSE)
Query params:
- `workspace` (required)

Auth required.
Event payload contract is frozen in ticket `09a32876-665c-476c-9587-8dcb3acd6e6a`.

## HTTP status mapping
- 200 OK: success
- 400 Bad Request: malformed parameters
- 401 Unauthorized: missing/invalid bearer
- 403 Forbidden: auth valid but not permitted
- 404 Not Found: ticket/workspace/resource missing
- 409 Conflict: optimistic concurrency conflict
- 429 Too Many Requests: server throttle/backpressure
- 500 Internal Server Error: unexpected failure
