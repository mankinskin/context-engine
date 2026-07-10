# CH5 — Sandboxed command execution with policy gates

WS3. Implements D3 isolation boundary.

## Scope
- Command execution through a pluggable sandbox trait; default backend Docker via
  `bollard`, scoped to the per-session working directory, deny-by-default outside it.
- Policy gate evaluated before execution; blocked commands produce an audit record.
- Backend is swappable (stricter backend later) without session-model changes.

## Acceptance criteria
- Disallowed commands are blocked with an audit record.
- Commands cannot escape the per-session working dir under the default backend.

## Dependencies
- depends_on CH4. Spec: unified-operator-interface D3, AC 4.

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo check -p agent-core` |
| Primary gate | `cargo test -p agent-core` — policy allow/deny, working-dir scoping, audit on denial (Docker-gated; skip w/ reason if daemon absent) |
| Manual/browser | Not applicable |
| Failure logs | `target/test-logs/` |
