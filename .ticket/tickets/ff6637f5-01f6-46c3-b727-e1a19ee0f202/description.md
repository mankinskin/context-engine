# Goal

Define how benchmarks and profiling runs use tracing logs and journal metadata without mixing timing data into deterministic replay state.

## Scope

- Align with existing benchmarking/profiling spec `c598ddb2`.
- Add run ids, p50/p95 summaries, fixture profile, operation kind, and timing spans to log/session metadata.
- Link benchmark evidence to tickets/specs and optional operation journals.
- Define retention and high-volume trace guidance for profiling runs.

## Acceptance criteria

- Benchmark/profile evidence can be linked from tickets/specs through log-api metadata.
- Timing spans are searchable by operation/run id.
- Replayable journal state remains deterministic and timing-free.