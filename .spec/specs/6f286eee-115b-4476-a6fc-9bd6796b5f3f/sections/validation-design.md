Every child ticket (CH1–CH12) carries a four-part validation matrix:
1. **Fast check** — quickest signal (e.g. `cargo check -p <crate>`, `cargo fmt --check`).
2. **Primary automated gate** — the authoritative test (`cargo test -p <crate>`,
   integration test, or Playwright e2e).
3. **Manual / browser evidence** — required for any server-interface or frontend
   change: external Chromium-family browser at a documented resolution, plus
   Playwright screenshots for transient UI (diff preview, mode-switch, live stream).
4. **Failure log path** — where debugging evidence lands, primarily
   `target/test-logs/` for tracing-based tests.

Cross-cutting validation properties the suite must prove:
- **Mode continuity**: converting an interactive session to a loop preserves
  session identity and history (integration test).
- **Reconnect continuity**: a client can disconnect and reconnect without losing
  authoritative session state (server integration + e2e).
- **Multi-observer fanout**: two observers of one session receive identical
  event streams (server integration).
- **Policy enforcement**: sandbox/budget gates block disallowed commands and
  over-budget loops, with an audit record (unit + integration).
- **Recovery**: a killed process resumes a loop from checkpoint (integration).

Tracing setup for tests uses `init_test_tracing!(&graph)` where a graph context
applies; otherwise standard `tracing-subscriber` test init, with logs routed to
`target/test-logs/`.
