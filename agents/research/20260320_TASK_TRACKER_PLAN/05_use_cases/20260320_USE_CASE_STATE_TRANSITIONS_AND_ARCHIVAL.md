# Use Case: State Transitions, Closure, and Archival

## Goal

Handle lifecycle progression from active work to closure and long-term archival without losing traceability.

## Preconditions

- State machines are configurable by ticket type.
- Close and archive policies are defined in workflow config.
- Search supports archived scope filtering.

## Scenario

1. Ticket reaches implementation completion on branch.
2. Validation checks pass (tests, review, dependency closure).
3. Ticket moves to `done` with closure metadata (`closed_at`, `closed_by`, `resolution`).
4. Retention policy runs after N days and archives ticket content.
5. Archive process compacts large artifacts, keeps manifest and audit links hot.
6. Archived tickets remain queryable and can be reopened via policy-gated transitions.

## Data Flows

- Active index segment -> archive index segment transition.
- Filesystem: move from active path to archive path or mark-in-place with archive flag.
- Git history remains intact and linked.

## Concurrency Rules

- Archival job skips tickets with active lease or open blockers.
- Reopen requires lock and revalidation against current schema.
- Archive migration is chunked and resumable.

## Failure Modes

- Archive path unavailable: fallback to mark-in-place and retry queue.
- Schema drift on reopen: enter `needs-migration` state instead of forced reopen.
- Missing attachment during archive: record integrity error and halt ticket archive.

## Success Metrics

- Archive throughput and failure rate.
- Query latency impact from active vs archived partitioning.
- Reopen success ratio after archival.
