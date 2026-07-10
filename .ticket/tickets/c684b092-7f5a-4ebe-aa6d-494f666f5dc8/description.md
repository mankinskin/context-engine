# CH2 — Core loop state machine + unified session/mode model

WS2 core. The single session model that makes chat and loop two modes of one thing.

## Scope
- Explicit ReAct state machine phases: prompt -> tool -> observe -> decide -> act.
- One `Session` type with `Interactive` and `Autonomous` modes; promotion and
  demotion transitions preserve session identity and history.
- Supervised, isolated `tokio` runtime for the loop so UI event drops cannot lock
  core file-writing logic.
- Loop throttling primitive (hook points defined; budgets wired in CH3).

## Acceptance criteria
- Promoting an interactive session to a loop keeps the same session id + history.
- Pausing a loop, taking an interactive turn, and resuming works.
- State transitions are total and test-asserted.

## Dependencies
- depends_on CH1 (shared contracts). Spec: unified-operator-interface AC 1,5.

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo check -p agent-core` |
| Primary gate | `cargo test -p agent-core` — transitions, mode promotion identity/history, pause/resume |
| Manual/browser | Not applicable |
| Failure logs | `target/test-logs/` |
