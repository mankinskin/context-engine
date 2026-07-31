---
description: "Use when handling tool result output, managing command spills, or working with the compact-terminal MCP or CLI transports. Covers output reduction and spill-file inspection."
applyTo: "**/*.sh,**/*.ps1"
---

## Default Agent Tool Suite

Running commands is the **execute** category of the default agent tool suite
(read / execute / edit / filesystem / search). Its implementation is
`compact-terminal-mcp`, and it is the default path for **every** agent,
including delegated sub-agents:

- MCP (preferred): `compact-terminal-mcp` — `run`, `read_spill`. Reachable
  through the `'compact-terminal-mcp/*'` wildcard in
  `.agents/agents/*.agent.md`.
- CLI fallback: the `compact-terminal` binary
  (`memory-api/tools/cli/compact-terminal-cli`) — `run`, `read-spill`. Use this
  when the MCP surface is unavailable; it shares the same `compact-terminal-api`
  core, so bounding and spill behavior are identical to the MCP path.
- Shell fallback: the `rtk` proxy, which filters output at the shell level.
- Follow-up inspection: the **read** category (`peek`), covered in
  [file-inspection.instructions.md](file-inspection.instructions.md).

Use this suite instead of raw unbounded terminal capture.

## Tool Result Guarding

Before the model reasons over tool output, reduce it to the smallest useful form.

Rules:
- Keep commands, test runs, and searches in a normalized tuple: scope, command, result, blocker, pointer.
- Use grep, bounded reads, and targeted extraction before exposing raw output to the model.
- When a tool emits a large structured payload, extract the needed fields first and discard the rest.
- Do not pass duplicated tool arguments or repeated lifecycle wrappers forward as context.

Compact extraction pattern:

```text
artifact -> bounded search -> extracted finding -> prompt summary
```

## Compact Terminal Expectations

The `rtk` proxy and the compact-terminal MCP tool (`memory-api/tools/mcp/compact-terminal-mcp`) truncate long outputs automatically. Long outputs are stored in a transient file and can be inspected via bounded search/read tools:

```bash
# Long output: rtk returns a summary + transient file path
rtk cargo test -p context-read  # summary inline; full log in target/test-logs/

# Follow up on a specific failure
peek target/test-logs/<file> --grep "FAILED" --window 10
peek target/test-logs/<file> --start N --end M
```

**compact-terminal-mcp pattern** (registered in `.vscode/mcp.json` and
`.github/mcp.json`; use it as the default for long-running commands):
1. `run("cargo test -p crate")` → gets spilled if long; use `spill_file` path.
2. `read_spill(spill_file, grep="FAILED")` → find failing test line numbers.
3. `read_spill(spill_file, start=N, end=M)` → read specific failure details.
4. Fix the issue; re-run only the targeted test.

**compact-terminal-cli fallback** (same steps, when MCP is unreachable):

```bash
# 1. Run bounded; long output spills to a transient file
compact-terminal run "cargo test -p context-read"

# 2. Locate failures by line number
compact-terminal read-spill <spill_file> --grep "FAILED"

# 3. Read the specific failure
compact-terminal read-spill <spill_file> --start N --end M
```

`--grep` matches a literal substring, not a regex — pass `FAILED`, not `^FAILED$`.

Rules:
- When a command produces truncated output, inspect the transient file via bounded read before replaying the full command.
- Do not re-run long commands just to see more output — use the stored output file first.
- Keep test log queries targeted: search for the specific error string, not the full log.
