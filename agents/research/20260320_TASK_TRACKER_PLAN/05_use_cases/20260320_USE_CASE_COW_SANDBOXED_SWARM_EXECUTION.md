# Use Case — COW Sandboxed Swarm Execution (Optional)

> **⚠️ DEFERRED** — Executor and sandbox integration is parked until post-dogfooding.
> See [DEFERRED_EXECUTOR.md](../DEFERRED_EXECUTOR.md) for rationale and reactivation criteria.
> The lease protocol (Phase 1.5) covers claim/heartbeat/expiry without executor coupling.

## Scenario

A coordinator claims 40 ready tickets for refinement and implementation. The workspace runs mixed execution backends:

- Default: local process execution for broad compatibility
- Optional: Zeroboot COW sandbox execution on Linux/KVM hosts

The scheduler assigns each claimed task to an executor based on required capabilities and host support.

## Problem

Parallel agent execution can interfere through shared process state and host-level side effects, especially during high-concurrency swarms.

## Solution

Use a pluggable executor model tied to lease lifecycle:

1. Coordinator creates lease (`ticket claim`) with required capabilities.
2. Scheduler resolves executor:
   - Zeroboot when host supports Linux/KVM and task is compatible.
   - Local executor fallback otherwise.
3. Lease metadata records `executor_backend`, `sandbox_id` (if any), and timing.
4. Heartbeats renew leases while task runs.
5. Completion or failure closes lease and emits execution diagnostics.

## Reference

- `zerobootdev/zeroboot` for fast COW VM sandbox spawning patterns.
- Adopt as optional executor only; keep ticket source of truth unchanged.

## Commands / API Expectations

- `ticket claim --id <uuid> --json` returns executor assignment metadata.
- `ticket lease status --id <uuid> --json` includes backend and heartbeat freshness.
- `ticket unclaim --id <uuid> --reason <text>` always tears down sandbox/process resources.

## Acceptance Signals

- Mixed backend operation is deterministic: same capability inputs choose same backend.
- Zeroboot-unavailable hosts continue processing via local fallback without blocking queue.
- Lease expiry triggers cleanup and re-queue behavior for unfinished work.
- Graph/merge queue views surface active backend and lease holder where relevant.

## Failure Handling

- Sandbox spawn failure: record diagnostic, retry once on same backend, then fallback to local.
- Heartbeat timeout: mark lease stale, terminate sandbox/process, re-queue ticket.
- Capability mismatch: reject assignment early with clear machine-readable error.
