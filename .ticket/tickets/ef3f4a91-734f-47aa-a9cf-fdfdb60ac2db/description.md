# [profiling] Performance profiling & benchmark matrix (tracker)

Parent tracker for adding performance profiling and benchmarking across the
context-engine architecture. Motivated by slow 3-D graph loading and non-smooth
node rendering in the viewer-api-based viewers.

## Phase 1 — DONE (implemented, compiles, validation pending browser run)

Implemented in the `memory-viewers/viewer-api` submodule
(`viewer-api/frontend/dioxus`):

- `profile-browser` cargo feature: `profile-browser = ["tracing/release_max_level_trace"]`.
- Zero-cost `profile_scope!` macro in `src/profiling.rs` (active only under
  `cfg(all(target_arch = "wasm32", feature = "profile-browser"))`).
- Instrumented per-frame hot path: `graph3d::render_frame` (`src/graph3d/render.rs`).
- `tracing-wasm` timeline marks via `WASMLayerConfigBuilder` +
  `set_report_logs_in_timings(true)` (`src/tracing_setup/mod.rs`).
- Playwright trace-capture helper `withBrowserTrace` + categories
  (`blink.user_timing` mandatory) in `e2e/shared/profiling.ts`.
- `graph3d-profiling-suite.ts` captures `chrome-profile.json`.
- WASM micro-benchmarks `tests/graph3d_bench.rs`
  (`wasm_bindgen_test_configure!(run_in_browser)` + `performance.now()`).

Validated: `cargo check --target wasm32-unknown-unknown --features profile-browser`
and native `cargo check` both pass with 0 errors.

## Child slices

1. Validate browser profiling pipeline (trace capture + wasm benches) — high.
2. Native Criterion benchmark matrix for context-* + ticket/spec APIs — medium.
3. CLI/HTTP/MCP end-to-end test matrix (ticket + spec surfaces) — medium.
4. CLI/HTTP/MCP throughput/latency benchmarks — low.
5. Testing + benchmark matrix index doc and run commands — low.

## Notes / blockers

- The `viewer-api/.ticket` store is currently missing its
  `tickets.db` (only `-wal`/`-shm` present) so this tracker lives in the root
  `.ticket` store. If viewer-api-scoped tickets are desired later, repair that
  store first (`ticket scan --force` after restoring `tickets.db`).