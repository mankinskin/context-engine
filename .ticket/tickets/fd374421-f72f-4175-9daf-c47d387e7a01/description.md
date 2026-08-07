`compact-terminal-mcp` hangs forever on every request, returning no output at all, including for a trivial `echo alive`. Reproduced repeatedly on 2026-08-07 after rebuilding the binary from source, so staleness is ruled out. Sibling MCP servers proxied through the same `mcp-toolmon` middleware respond normally, so session routing is not the cause.

## Root cause

In `memory-api/crates/compact-terminal-api/src/execute.rs`:

- Line 38-40: the child is spawned with `Command::new("sh")`, and `stdout`/`stderr` are set to `Stdio::piped()`, but `stdin` is never configured. The child therefore inherits the server process's stdin.
- The MCP server uses a stdio transport, so its stdin carries MCP protocol traffic. A spawned shell that reads stdin consumes or blocks that same stream, wedging the server.
- Line 74: the timeout path returns `RunResult::TimedOut` without killing the child. The orphaned shell keeps holding stdin, so every subsequent request hangs as well. This matches the observed progression exactly: the first call timed out, and all later calls hung with zero output.

## Acceptance criteria

1. The spawned child never inherits the server's stdin; stdin is explicitly set to null (or an empty pipe that is closed immediately).
2. The timeout path kills the child process rather than leaking it, and reaps it.
3. A regression test asserts that a command which reads from stdin (for example `cat`) terminates rather than hanging, and does not disturb the server.
4. A regression test asserts that a command exceeding its timeout leaves no surviving child process.
5. After the fix, a trivial `echo alive` through the MCP surface returns promptly, and a subsequent call also returns promptly (proving no orphan wedged the stream).