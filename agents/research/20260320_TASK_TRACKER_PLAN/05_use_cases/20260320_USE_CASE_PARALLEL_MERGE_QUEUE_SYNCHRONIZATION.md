# Use Case: Parallel Merge Queue Synchronization

## Goal

Coordinate many in-flight ticket branches so merges happen in a safe order that preserves dependency and integration correctness.

## Preconditions

- Ticket dependency graph is current.
- Branch metadata is tracked per ticket.
- CI status is ingestible into ticket metadata.

## Scenario

1. Multiple agent-owned tickets are in `merge-ready`.
2. Merge scheduler ranks them by dependency order and conflict likelihood.
3. Queue manager enforces that blockers merge before dependents.
4. Each candidate is rebased or merge-tested against moving main.
5. On successful merge, dependent tickets are re-evaluated and promoted.
6. On merge failure, ticket moves to `merge-conflict` with diagnostics and suggested owners.

## Data Flows

- Index fields: `ci_status`, `merge_queue_position`, `conflict_risk`, `depends_on_open`.
- Git metadata: branch ahead/behind, merge base, conflict files.
- Search/query for triage: `state:merge-conflict` with module predicates.

## Concurrency Rules

- One queue decision writer at a time (queue lock).
- Ticket updates still use per-ticket lock.
- Queue processing loop is deterministic and replayable from logs.

## Failure Modes

- Stale dependency projection causing wrong queue order: force graph refresh before dequeue.
- CI flakiness: require configurable retry threshold before demotion.
- Queue starvation: age-based priority boost.

## Success Metrics

- Merge queue throughput.
- Conflict rate before vs after dependency-aware scheduling.
- Lead time from merge-ready to merged.
