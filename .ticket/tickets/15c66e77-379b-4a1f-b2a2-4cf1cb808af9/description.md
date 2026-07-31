## Problem
The whole feature's value is that a live reload is invisible to the client and the Windows lock problem is actually gone. Needs end-to-end proof, not just unit tests of individual parts. Part of epic 25780944; depends on T6 (full pipeline: shadow-copy + supervisor + handshake replay + watcher).

## Approach
Build a fake child MCP server binary (or reuse an existing test fixture) driven by an integration test harness that: starts mcp-toolmon in front of it, completes a real `initialize` handshake, issues tool calls, then swaps the underlying binary file for a second build of the fake server (different behavior/version marker), and asserts the client observes no connection drop, no orphaned/hung request, and that calls issued after the swap are served by the NEW binary (e.g. by a version marker in its response). Add a Windows-specific test asserting the canonical path P is not locked (can be renamed/overwritten) while mcp-toolmon is running against it.

## Acceptance criteria
- [ ] Integration test spins up mcp-toolmon fronting a fake/controllable child MCP server
- [ ] Test performs a real handshake, then triggers a binary swap (either via the watcher path or by directly invoking swap for determinism)
- [ ] Assertion: client connection is never dropped during swap
- [ ] Assertion: a request in flight at the moment of swap either completes normally or receives a synthesized JSON-RPC error, never hangs
- [ ] Assertion: a tool call issued after the swap completes is served by the new binary (version marker check)
- [ ] Windows-specific test: while mcp-toolmon runs, the canonical path P can be overwritten/replaced (simulating `cargo install --force`) without an OS-level lock error
- [ ] Tests wired into the crate's normal `cargo test` run (no manual-only steps) and pass in CI/local Windows environment
- [ ] Any manual verification steps that can't be automated are documented in the ticket status summary with rationale

## Files touched
- memory-api/tools/mcp/mcp-toolmon/tests/integration_reload.rs (new)
- memory-api/tools/mcp/mcp-toolmon/tests/fixtures/ (new fake child server, if needed)
- memory-api/tools/mcp/mcp-toolmon/tests/windows_lock.rs (new, cfg(windows))