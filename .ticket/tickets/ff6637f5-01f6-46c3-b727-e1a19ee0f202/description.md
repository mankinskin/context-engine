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

## Acceptance criteria

- Benchmark/profile evidence can be linked from tickets/specs through log-api metadata.
- Timing spans are searchable by operation/run id.
- Replayable journal state remains deterministic and timing-free.