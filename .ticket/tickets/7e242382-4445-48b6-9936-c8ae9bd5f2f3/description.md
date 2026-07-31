## Problem
Nothing currently detects that the canonical child binary P has been rebuilt. Part of epic 25780944; depends on T4 (swap target) and T5 (handshake replay must run on every triggered swap).

## Approach
Add a poller that periodically checks P's (mtime, size) and, on suspected change, a content hash, debounced so a half-written file (mid `cargo install`) is never grabbed (e.g. require the (mtime,size) to be stable across two consecutive polls before treating it as changed). On confirmed change, invoke the supervisor's `swap_child()` (T4), which triggers handshake replay (T5). After a successful swap, emit `notifications/tools/list_changed` to the client. Add `TOOLMON_RELOAD` (default on/off switch) and `TOOLMON_POLL_MS` (poll interval) env vars.

## Acceptance criteria
- [ ] Watcher polls P's mtime+size (and hash on suspected change) at `TOOLMON_POLL_MS` interval (documented default)
- [ ] Debounce logic prevents triggering a swap on a partially-written binary (stability check across consecutive polls)
- [ ] `TOOLMON_RELOAD=0` (or equivalent falsy value) disables the watcher entirely; default is enabled
- [ ] Confirmed binary change triggers exactly one `swap_child()` call per change (no duplicate/rapid-fire swaps)
- [ ] `notifications/tools/list_changed` is sent to the client after a successful swap
- [ ] No new MCP tool is added to `tools/list` for triggering reload (explicitly rejected in design)
- [ ] Unit test: simulated binary write (including a slow/partial write) triggers exactly one debounced swap, not zero, not many
- [ ] Unit test: `TOOLMON_RELOAD` disabled means no watcher thread/polling runs

## Files touched
- memory-api/tools/mcp/mcp-toolmon/src/watcher.rs (new)
- memory-api/tools/mcp/mcp-toolmon/src/supervisor.rs
- memory-api/tools/mcp/mcp-toolmon/src/main.rs
- memory-api/tools/mcp/mcp-toolmon/src/proxy.rs
- memory-api/tools/mcp/mcp-toolmon/tests/ (watcher debounce tests)