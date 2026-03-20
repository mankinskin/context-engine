# Use Cases — Parallel Agent Ticket Operations

This folder captures concrete operational scenarios for a distributed filesystem ticket system with a global index, per-ticket locks, git-backed diff history, and FTS+metadata query.

## Scenario List

1. [20260320_USE_CASE_SWARM_REFINEMENT_PIPELINE.md](20260320_USE_CASE_SWARM_REFINEMENT_PIPELINE.md)
2. [20260320_USE_CASE_DEEP_DEPENDENCY_GRAPH_EXECUTION.md](20260320_USE_CASE_DEEP_DEPENDENCY_GRAPH_EXECUTION.md)
3. [20260320_USE_CASE_BRANCH_BOUNDARY_IMPLEMENTATION.md](20260320_USE_CASE_BRANCH_BOUNDARY_IMPLEMENTATION.md)
4. [20260320_USE_CASE_CONFLICT_AVOIDANCE_AND_LEASES.md](20260320_USE_CASE_CONFLICT_AVOIDANCE_AND_LEASES.md)
5. [20260320_USE_CASE_STATE_TRANSITIONS_AND_ARCHIVAL.md](20260320_USE_CASE_STATE_TRANSITIONS_AND_ARCHIVAL.md)
6. [20260320_USE_CASE_PARALLEL_MERGE_QUEUE_SYNCHRONIZATION.md](20260320_USE_CASE_PARALLEL_MERGE_QUEUE_SYNCHRONIZATION.md)
7. [20260320_USE_CASE_ORPHAN_DISCOVERY_AND_RECOVERY.md](20260320_USE_CASE_ORPHAN_DISCOVERY_AND_RECOVERY.md)
8. [20260320_USE_CASE_AGENT_HANDOFF_AND_CONTEXT_CONTINUITY.md](20260320_USE_CASE_AGENT_HANDOFF_AND_CONTEXT_CONTINUITY.md)

## Reading Order

1. Swarm refinement pipeline
2. Conflict avoidance and leases
3. Deep dependency graph execution
4. Branch boundary implementation
5. State transitions and archival
6. Parallel merge queue synchronization
7. Orphan discovery and recovery
8. Agent handoff and context continuity

## Common Invariants Across All Scenarios

- Ticket identity is UUID and immutable.
- Ticket folders are distributed in workspace scan roots.
- Global index is derived from filesystem state plus reconciliation metadata.
- Per-ticket lock is required for all mutating operations.
- Git-backed history is append-only (revert creates new commit).
- Search index is derived and rebuildable via `ticket scan --reindex`.
