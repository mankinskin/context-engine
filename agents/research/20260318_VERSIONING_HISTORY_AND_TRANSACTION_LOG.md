# Status: TODO

# Versioning, History, and Transaction Logs

## Problem

The tool needs safe incremental refinement with auditable history and rollback.

## Core Design Patterns

### Pattern 1: Append-only event log + projections
- Every mutation emits an immutable event.
- Read models (current ticket state, dependency graph, search index) are projections.
- Supports replay, audit, and forensic debugging.

### Pattern 2: Snapshot + event tail
- Periodic snapshots reduce replay cost.
- Keep event tail since last snapshot.

### Pattern 3: Git-backed file history
- Use git-style object history for ticket folders.
- Optional integration with `git2` for in-process history operations.

Sources:
- https://docs.rs/git2/latest/git2/
- https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain

## Event Envelope Sketch

```json
{
  "event_id": "evt_...",
  "ticket_id": "TCK-2026-0001",
  "kind": "checkbox_toggled",
  "ts": "2026-03-18T12:00:00Z",
  "actor": "user-or-agent",
  "payload": { "path": "checklist.items[3]", "new": true },
  "base_version": 42,
  "new_version": 43
}
```

## Transaction Boundaries

- Multi-step updates should be grouped as a single logical transaction.
- Event record should include all sub-mutations or reference a bundle.
- Rollback options:
  - compensating event
  - snapshot restore
  - git reset-like branch restore (if git-backed mode enabled)

## Conflict Handling

- Single-writer per ticket lock (simple)
- Or optimistic version check (`base_version` must match)
- For distributed/multi-device: merge policy required (future phase)

## TODO

- TODO: Decide canonical version key (monotonic int vs hash-based IDs).
- TODO: Define snapshot cadence policy.
- TODO: Define rollback API semantics and user-facing CLI/HTTP commands.
- TODO: Define event schema evolution strategy.
