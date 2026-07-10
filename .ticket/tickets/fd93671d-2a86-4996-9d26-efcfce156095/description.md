# CH10 — Reliability/recovery: checkpointing + reconnect semantics

WS6. Implements D2 persistence and reconnect continuity.

## Scope
- Append-only NDJSON per-session event log + periodic checkpoints referencing last
  applied event offset (D2).
- Reconnect: a client reattaches without losing authoritative session state.
- Watchdog: stuck / over-budget loops pause with a recorded reason.
- Structured tracing with per-session + per-tool-call correlation ids.
- Health endpoint.

## Acceptance criteria
- A killed process resumes a loop from the last checkpoint.
- A client disconnect + reconnect restores authoritative state (no lost/duplicated events).
- Watchdog pauses a no-progress/over-budget loop with an audit reason.

## Dependencies
- depends_on CH6 (needs lifecycle + broadcast). Spec: unified-operator-interface AC 2,5.

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo check -p agent-server -p agent-core` |
| Primary gate | `cargo test` — checkpoint resume, reconnect continuity, watchdog trip |
| Manual/browser | Reconnect exercised in browser during CH11 e2e |
| Failure logs | `target/test-logs/` |
