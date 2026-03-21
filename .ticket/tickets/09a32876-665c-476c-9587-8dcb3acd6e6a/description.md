# Design: SSE event schema freeze for ticket graph updates

## Objective
Freeze the SSE payload contract so backend and frontend can implement independently.

## Envelope
- `id`: monotonic stream event id (string)
- `event`: event type
- `data`: JSON payload
- `ts`: server timestamp
- `workspace`: workspace name

## Event types
- `ticket.upsert`
- `ticket.delete`
- `edge.upsert`
- `edge.delete`
- `ticket.conflict`
- `snapshot.ready`
- `diagnostic.warning`

## Payload schema (high level)
- `ticket.upsert`: `{ id, fields, updated_at, state }`
- `ticket.delete`: `{ id, deleted_at }`
- `edge.upsert`: `{ from, to, kind }`
- `edge.delete`: `{ from, to, kind }`
- `ticket.conflict`: `{ id, expected_rev, observed_rev, operation }`
- `snapshot.ready`: `{ snapshot_id, node_count, edge_count }`

## Semantics
- Best-effort live only in v1 (no replay).
- Per-workspace ordering preserved by single workspace stream loop.
- Duplicate delivery tolerance required client-side.

## Draft artifacts
- Schema draft: `assets/design/sse-schema-v1.md`
- Contract tests draft: `assets/design/sse-contract-tests-v0.1.md`

## Checklist
- [x] Event list approved
- [x] Payload schemas approved
- [x] Ordering/duplication semantics approved
- [x] Error/diagnostic events approved
- [x] Contract tests identified
