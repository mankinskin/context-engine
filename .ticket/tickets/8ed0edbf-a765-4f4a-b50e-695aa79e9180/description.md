# CH6 — Axum session lifecycle + websocket broadcast fanout

WS4. The streaming control plane both UIs consume.

## Scope
- Axum `State` atomic lifecycle machine: create/start/pause/resume/stop, no
  multi-thread write deadlocks across async transitions.
- Websocket endpoints stream `LifecycleEvent` + incremental output.
- `tokio::sync::broadcast` fanout so multiple observers of one session get identical frames.
- CORS for local native dev server <-> browser WASM client.

## Acceptance criteria
- Two observers of one session receive identical event streams.
- Lifecycle transitions are atomic and test-asserted; no deadlock under concurrent control.

## Dependencies
- depends_on CH2 (and CH1 contracts). Spec: unified-operator-interface AC 3.

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo check -p agent-server` |
| Primary gate | `cargo test -p agent-server` — lifecycle transitions, multi-observer identical fanout, concurrency/no-deadlock |
| Manual/browser | Deferred to CH11 (server exercised via clients) |
| Failure logs | `target/test-logs/` |
