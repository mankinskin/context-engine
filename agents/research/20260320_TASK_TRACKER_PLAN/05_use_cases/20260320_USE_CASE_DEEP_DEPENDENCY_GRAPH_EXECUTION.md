# Use Case: Deep Dependency Graph Execution

## Goal

Support execution planning over deep and wide dependency structures while avoiding deadlocks, hidden blockers, and conflicting parallel work.

## Preconditions

- Multiple edge types are configured (for example `blocks`, `relates_to`, `child_of`).
- Edge constraints define which edge kinds are acyclic-enforced.
- Query layer can resolve transitive dependency relationships.

## Scenario

1. Program agent creates an epic with 20+ child tickets and multiple cross-links.
2. Graph analysis computes depth, critical path, and blocker sets.
3. Scheduler agent opens only tickets with all hard blockers resolved.
4. Worker agents pull from `state:ready depth<=k blocked_by:0` queues.
5. As tickets complete, dependent tickets are automatically re-evaluated and promoted.
6. If a cycle is introduced, write is rejected and diagnostics suggest minimal cut edges.

## Data Flows

- Edge table: `(from_uuid, to_uuid, kind)`.
- Derived graph projections: `in_degree`, `out_degree`, `criticality_score`, `blocked_by_count`.
- Search/query: mixed text + graph predicates.

## Concurrency Rules

- Edge updates require lock on source ticket and short index lock.
- Scheduler updates are idempotent and retry-safe.
- Promotion operations use optimistic checks on dependency counters.

## Failure Modes

- High fan-out updates causing index churn: coalesce updates in short debounce windows.
- Cross-root moves invalidating edge paths: UUID remains stable, path remapped in index.
- Partial graph rebuild after crash: `ticket scan` reconstructs full dependency projection.

## Success Metrics

- Blocked-work percentage over time.
- Number of cycle rejections prevented before merge.
- Queue throughput for ready tickets.
