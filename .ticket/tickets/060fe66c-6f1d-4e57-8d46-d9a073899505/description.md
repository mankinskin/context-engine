## Problem
There is currently no way to replace the running child process in place. A future watcher (T6) needs a supervisor that can kill/respawn the child, fail in-flight requests safely, and never let a bad respawn kill the whole proxy. Part of epic 25780944; depends on T3 (shadow-copy exec, since respawn re-copies from P to a fresh shadow path).

## Approach
Introduce a supervisor component owning the child process handle and in-flight request table (request id -> pending response channel). Expose `swap_child()`: quiesce new requests, wait for in-flight to either complete or be synthesized as errors after a bounded grace period, kill the current child, re-copy P to a new shadow path, respawn, resume routing. On respawn failure, do NOT propagate a process exit — retry with exponential backoff, and fall back to serving from the last-known-good shadow copy while retries continue.

## Acceptance criteria
- [ ] Supervisor tracks in-flight JSON-RPC requests by id
- [ ] `swap_child()` kills the current child and respawns from a freshly re-copied shadow binary
- [ ] Requests in flight when the child dies receive synthesized JSON-RPC error responses (proper `error` object, correlated `id`) instead of hanging
- [ ] On respawn failure, the proxy process does not exit; it retries with backoff (bounded max interval) and continues serving errors for new requests during the outage
- [ ] Last-known-good shadow copy is retained and used as fallback if respawn of the new binary fails validation (e.g. process exits immediately)
- [ ] Unit/integration test simulating a fake child binary crash mid-request confirms synthesized error, no proxy exit, and successful recovery on next swap
- [ ] No public MCP tool exposed for triggering swap (that remains internal, per rejected-design constraint; trigger comes from T6's watcher)

## Files touched
- memory-api/tools/mcp/mcp-toolmon/src/supervisor.rs (new)
- memory-api/tools/mcp/mcp-toolmon/src/main.rs
- memory-api/tools/mcp/mcp-toolmon/src/shadow.rs
- memory-api/tools/mcp/mcp-toolmon/tests/ (supervisor swap/failure tests)