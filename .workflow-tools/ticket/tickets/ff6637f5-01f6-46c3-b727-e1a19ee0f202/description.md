# Goal

Define how benchmarks and profiling runs use tracing logs and journal metadata without mixing timing data into deterministic replay state.

## Scope

- Align with existing benchmarking/profiling spec `c598ddb2`.
- Add run ids, p50/p95 summaries, fixture profile, operation kind, and timing spans to log/session metadata.
- Link benchmark evidence to tickets/specs and optional operation journals.
- Define retention and high-volume trace guidance for profiling runs.

## Placement decision

Keep this benchmark/profiling coordination ticket in the root `context-engine` workspace as the lowest common ancestor. It links lower-crate timing and journal work across `memory-api`, `log-api`, and `context-stack`, so it should not be moved into a single child crate.

## Specialized lower-crate tickets linked from here

- memory-api: `6c859ac3`, `3041d7e3`, `35cd05c1`
- log-api: `d3349747`, `aa94d02e`
- context-stack: `1dffcf23`

## Instrumentation delta plan (near-term)

1. Decompose integration hot path timings
- Split integration timing into: manifest parse, index upsert, edge writes, description read, search upsert.
- Emit both span events and structured timing map fields for each sub-phase.

2. Decompose workflow-facts recompute timings
- Split workflow recompute into: dependency edge fetch, dependency ticket fetch, unresolved count calc, workflow-facts write.
- Record per-phase elapsed times and ticket counts to support scaling analysis.

3. Add correlation and run metadata
- Standardize required metadata fields: `run_id`, `operation_id`, `fixture_profile`, `change_count`, `reindex_mode`, `workspace_scope`.
- Ensure all benchmark/e2e evidence can be linked to ticket/spec/journal artifacts.

4. Add breadth and depth evidence shapes
- Add distribution summaries (`p50/p95/p99`) in addition to single-run timings.
- Add cross-fixture matrix (small/medium/large and incremental 1/10/100+ deltas).

5. Preserve deterministic replay boundaries
- Keep timing/profile values out of replay state and rollback-critical journal payloads.
- Store profiling evidence in log/session metadata and dedicated benchmark artifacts.

## Current evidence baseline

- Existing evidence already isolates dominant paths: integration and workflow recompute dominate, scan-root discovery is small.
- Open/init profiling plus scan phase maps are implemented and validated in ticket-api e2e/tests/benches.
- Move journals already carry phase timing maps; this should be aligned with the broader phase taxonomy.

## Missing profiling infrastructure for deeper/broader inspection

- No persistent first-class profile record model queryable alongside logs/journals.
- No standardized percentile or regression-threshold reporting contract.
- Limited cross-layer correlation from transport spans to journal/log/profile entities.
- No canonical dashboard-style aggregation over repeated runs and fixture cohorts.

## Acceptance criteria

- Benchmark/profile evidence can be linked from tickets/specs through log-api metadata.
- Timing spans are searchable by operation/run id.
- Replayable journal state remains deterministic and timing-free.

## Acceptance criteria addendum (execution-ready)

- Bench/e2e outputs include sub-phase timings for integration and workflow recompute paths.
- Benchmark runs emit standardized metadata (`run_id`, `fixture_profile`, `change_count`, `reindex_mode`) and can be queried by these keys.
- Evidence includes distribution summaries (`p50/p95/p99`) and explicit fixture-scale comparisons.
- Profile evidence linkage to ticket/spec/journal references is documented and test-validated.

## Validation checklist

- [x] Report includes distribution metrics (`p50`, `p95`, `p99`) for each target operation.
- [x] Report includes standardized run metadata: `run_id`, `fixture_profile`, `change_count`, `reindex_mode`, `workspace_scope`.
- [x] Evidence links resolve to ticket/spec/journal artifacts for at least one full benchmark run.
- [x] Replay payload remains deterministic and excludes profiling-only timing fields.