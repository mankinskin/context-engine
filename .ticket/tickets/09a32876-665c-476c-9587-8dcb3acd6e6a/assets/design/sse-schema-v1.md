# SSE Schema v1 (Frozen Draft)

Status: draft for review

## Envelope
Each SSE event uses:
- `id`: monotonic opaque id (string)
- `event`: event name
- `data`: JSON payload stringified

Data payload common fields:
- `request_id` string (optional for system events)
- `workspace` string
- `ts` RFC3339 timestamp

## Event names
- `ticket.upsert`
- `ticket.delete`
- `edge.upsert`
- `edge.delete`
- `ticket.conflict`
- `snapshot.ready`
- `diagnostic.warning`

## Payload schemas

### ticket.upsert
```json
{
  "workspace": "default",
  "ts": "2026-03-21T00:00:00Z",
  "ticket": {
    "id": "uuid",
    "state": "in-progress",
    "updated_at": "2026-03-21T00:00:00Z",
    "fields": {}
  }
}
```

### ticket.delete
```json
{
  "workspace": "default",
  "ts": "2026-03-21T00:00:00Z",
  "id": "uuid",
  "deleted_at": "2026-03-21T00:00:00Z"
}
```

### edge.upsert / edge.delete
```json
{
  "workspace": "default",
  "ts": "2026-03-21T00:00:00Z",
  "edge": {
    "from": "uuid",
    "to": "uuid",
    "kind": "depends_on"
  }
}
```

### ticket.conflict
```json
{
  "workspace": "default",
  "ts": "2026-03-21T00:00:00Z",
  "id": "uuid",
  "operation": "update",
  "expected_rev": 5,
  "observed_rev": 7
}
```

### snapshot.ready
```json
{
  "workspace": "default",
  "ts": "2026-03-21T00:00:00Z",
  "snapshot_id": "uuid",
  "node_count": 120,
  "edge_count": 310
}
```

### diagnostic.warning
```json
{
  "workspace": "default",
  "ts": "2026-03-21T00:00:00Z",
  "code": "hook.emit_failed",
  "message": "Failed to deliver hook event; reconcile fallback engaged"
}
```

## Semantics
- Best-effort live stream; no replay in v1.
- Client must tolerate duplicates.
- Ordering is guaranteed per-workspace by server stream loop.
