# Hook Emission Contract v0.1

Status: draft for review

## Trigger matrix
- Ticket create/update/delete
- Edge link/unlink
- Validation and release protocol transitions
- Lease claim/unclaim events

## Event payload
```json
{
  "event_id": "uuid",
  "workspace": "default",
  "op": "ticket.update",
  "entity_type": "ticket",
  "entity_id": "uuid",
  "rev": 12,
  "ts": "2026-03-21T00:00:00Z"
}
```

## Delivery semantics
- At-least-once delivery.
- Producer must be non-blocking for primary mutation path.
- Consumer deduplicates by `event_id`.

## Ordering
- Per-workspace FIFO queue.
- Cross-workspace ordering is undefined.

## Fallback reconciliation
- Periodic reconcile compares storage revision watermark vs emitted watermark.
- On gap detection:
  - emit `snapshot.ready`
  - emit batched upsert events to restore client convergence

## Failure behavior
- Hook emission failures increment metric counters.
- Mutation write still succeeds.
- Emit `diagnostic.warning` SSE event where possible.

## Observability metrics
- `hook_emit_success_total`
- `hook_emit_failure_total`
- `hook_queue_depth`
- `reconcile_runs_total`
- `reconcile_gap_events_total`
