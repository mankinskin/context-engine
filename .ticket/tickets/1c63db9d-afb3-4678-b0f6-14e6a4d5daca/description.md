# CH4 — MCP integration + per-session tool routing envelope

WS3. Answers research Q6 (routing MCP instances to the right session).

## Scope
- `rmcp` client: standard JSON-RPC stdio handshake at startup; register external tools.
- Tool invocation envelope carrying per-session routing metadata (session id +
  tool-call id) so concurrent sessions never cross-route.
- Audit record per tool call: name, args, exit, artifact paths, correlated by
  session + tool-call id.

## Acceptance criteria
- Two concurrent sessions route tool calls to their own MCP instances (test-isolated).
- Every tool call produces a correlated audit record.

## Dependencies
- depends_on CH2. Spec: unified-operator-interface AC 4.

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo check -p agent-core` |
| Primary gate | `cargo test -p agent-core` — handshake/registration (mock), per-session routing isolation, audit shape |
| Manual/browser | Not applicable |
| Failure logs | `target/test-logs/` |
