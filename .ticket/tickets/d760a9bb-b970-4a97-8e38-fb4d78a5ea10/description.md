# Goal
Make benchmark and Rust end-to-end performance runs emit structured tracing so timing regressions can be diagnosed beyond final wall-clock numbers.

# Scope
- Instrument representative Criterion benches and Rust perf/E2E harnesses such as `ticket-api` perf tests and shared benchmark entry points.
- Add lightweight tracing/session setup that can emit benchmark/test-run ids, fixture ids, and phase-level timing summaries without distorting the benchmark signal.
- Align with the existing profiling/logging track rather than inventing a new output format.

# Acceptance criteria
- Representative benches and Rust perf harnesses emit structured spans/events with run ids and fixture context.
- Slow-path evidence can be correlated with internal phase timings and surrounding store/kernel spans.
- The tracing setup is optional or bounded enough not to invalidate benchmark usefulness.