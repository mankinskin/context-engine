## Problem
When a child is respawned (T4), it needs a fresh MCP `initialize` handshake to function, but the client already completed its own handshake and must not see a second one. Part of epic 25780944; depends on T4 (supervisor owns the swap point where replay is injected).

## Approach
Cache the client's original `initialize` request params verbatim, plus the `notifications/initialized` notification, the first time they are observed. On every respawn (triggered by supervisor `swap_child()`), replay both into the new child before resuming normal routing. Suppress forwarding the new child's `initialize` response to the client (client already has its own). Compare the new child's reported capabilities against the cached original; log any divergence (do not fail the swap on divergence alone).

## Acceptance criteria
- [ ] Original client `initialize` params and `notifications/initialized` are cached on first observation
- [ ] On every respawn, cached `initialize` is sent to the new child and `notifications/initialized` follows, before any queued/new tool calls are routed to it
- [ ] The new child's `initialize` response is never forwarded to the client
- [ ] Capability divergence (new child's declared capabilities differ from the original handshake) is logged with enough detail to diagnose, but does not abort the swap
- [ ] Unit/integration test: after a simulated swap, the client sees no second `initialize` response and the new child receives correct `initialize`/`initialized` before any tool call
- [ ] Unit test: capability mismatch produces a log entry (or captured warning) without failing the swap

## Files touched
- memory-api/tools/mcp/mcp-toolmon/src/supervisor.rs
- memory-api/tools/mcp/mcp-toolmon/src/handshake.rs (new)
- memory-api/tools/mcp/mcp-toolmon/src/proxy.rs
- memory-api/tools/mcp/mcp-toolmon/tests/ (handshake replay tests)