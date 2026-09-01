---
description: "Use when running Criterion benchmarks or adding performance measurements. Covers benchmark commands and fixture details."
---

## Criterion Benchmarks

The BFS graph query pipeline is benchmarked in `crates/ticket-api/benches/graph_ops.rs`.
Run with: `cargo bench --bench graph_ops -p ticket-api`

| Benchmark | What it measures |
|---|---|
| `phase1_list_all_edges` | ReDB edge table scan (~630 edges) |
| `phase2_bfs_in_memory` | Pure in-memory BFS, no DB |
| `phase3_get_indexed_many` | Batch metadata fetch (1 ReDB transaction, 39 nodes) |
| `phase3_get_indexed_one_by_one` | Per-node fetch baseline (39 separate transactions) |
| `pipeline_full` | All 3 phases end-to-end |
| `pipeline_concurrent/{2,4,8,16,32}` | N threads barrier-synchronized |

The fixture builds 360 tickets + ~630 edges once per process (via `OnceLock`).

When adding a new storage-layer optimization, add a matching Criterion benchmark that shows the before/after comparison.

## Timeout Discipline for Benchmark Runs

Before starting any `cargo bench` invocation (including `--test` smoke mode), estimate its expected wall time from the scenario count and sample/measurement settings (e.g. `sample_size × measurement_time × scenario_count`), and set a hard timeout at that estimate plus a modest buffer. Use an explicit `timeout` on the run, or background it and poll on a schedule bounded by that same budget. Never wait unboundedly on a benchmark process — see [tool-output.instructions.md](../orchestration/tool-output.instructions.md#long-running-process-ownership) for the general long-running-process rules this specializes.

When a run exceeds its budgeted timeout:
- Stop waiting on it (kill or detach) instead of continuing to poll indefinitely.
- Register whatever evidence it produced up to that point (partial scenario results, log tail) as the validation record, and name exactly which scenarios were not reached.
- Treat the remaining coverage gap as a follow-up, not a reason to silently re-launch the same exhaustive run hoping it finishes faster.

`--test` (fast smoke) mode is a correctness proxy, not a substitute for an acceptance criterion that literally requires Criterion statistical output. State that distinction explicitly in the validation evidence rather than treating smoke-mode success as sufficient on its own.
