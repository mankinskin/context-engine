# CH1 — Workspace + crate scaffolding + shared protocol contracts

Foundation slice (WS1). Establishes crate boundaries (D1) and the shared
protocol contract that every other crate depends on.

## Scope
- Create top-level `agent-harness/` and add members `agent-shared`, `agent-core`,
  `agent-server`, `agent-tui`, `agent-web/frontend/dioxus` to root `Cargo.toml`.
- In `agent-shared`: define tagged `serde(tag="type", content="payload")` enums for
  lifecycle events and the unified session/mode model; add an explicit schema
  version and a compatibility/migration note.
- Ensure `agent-shared` compiles for both native and `wasm32-unknown-unknown`.

## Acceptance criteria
- Workspace builds clean; members resolve.
- Lifecycle event + session enums are versioned and serde round-trip stable.
- No provider/server logic leaks into `agent-shared`.

## Dependencies
- Spec: agent-harness/unified-operator-interface (D1). No ticket dependencies (foundation).

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo fmt --check` + `cargo check -p agent-shared` (native + `--target wasm32-unknown-unknown`) |
| Primary gate | `cargo test -p agent-shared` (serde round-trip + version-tag tests) |
| Manual/browser | Not applicable |
| Failure logs | `target/test-logs/` |
