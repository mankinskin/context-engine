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
9. [20260320_USE_CASE_COW_SANDBOXED_SWARM_EXECUTION.md](20260320_USE_CASE_COW_SANDBOXED_SWARM_EXECUTION.md)
10. [20260320_USE_CASE_AUTOMATED_GRAPH_ARTIFACTS.md](20260320_USE_CASE_AUTOMATED_GRAPH_ARTIFACTS.md)
11. [20260320_USE_CASE_SWARM_MESSENGER_DIGESTS.md](20260320_USE_CASE_SWARM_MESSENGER_DIGESTS.md)

## Reading Order

1. Swarm refinement pipeline
2. Conflict avoidance and leases
3. Deep dependency graph execution
4. Branch boundary implementation
5. State transitions and archival
6. Parallel merge queue synchronization
7. Orphan discovery and recovery
8. Agent handoff and context continuity
9. COW sandboxed swarm execution
10. Automated graph and board artifacts
11. Swarm messenger digests

## Common Invariants Across All Scenarios

- Ticket identity is UUID and immutable.
- Ticket folders are distributed in workspace scan roots.
- Global index is derived from filesystem state plus reconciliation metadata.
- Per-ticket lock is required for all mutating operations.
- Git-backed history is append-only (revert creates new commit).
- Search index is derived and rebuildable via `ticket scan --reindex`.

## Problem/Solution/Reference Summary

1. Problem: multi-agent contention and convergence under heavy parallel work.
Solution: deterministic reconciliation and lease-aware scheduling patterns in our own architecture.
Reference: concepts borrowed from `delightful-ai/beads-rs`.

2. Problem: operator and agent usability across human CLI and machine protocol workflows.
Solution: ergonomic CLI for humans, explicit JSON command protocol for agents, and shared workflow semantics across both.
Reference: patterns borrowed from `Dicklesworthstone/beads_rust`.

3. Problem: neither upstream model is a direct fit for distributed ticket folders with workflow-defined schemas.
Solution: use upstream as pattern libraries only; implement core storage and contracts natively in this plan.
Reference: both projects.
