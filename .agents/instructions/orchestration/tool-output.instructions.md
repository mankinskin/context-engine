---
description: "Use when handling tool result output, managing command spills, or working with compact-terminal MCP. Covers output reduction and spill-file inspection."
---

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

**compact-terminal-mcp pattern** (when available as MCP tool):
1. `run("cargo test -p crate")` → gets spilled if long; use `spill_file` path.
2. `read_spill(spill_file, grep="FAILED")` → find failing test line numbers.
3. `read_spill(spill_file, start=N, end=M)` → read specific failure details.
4. Fix the issue; re-run only the targeted test.

Rules:
- When a command produces truncated output, inspect the transient file via bounded read before replaying the full command.
- Do not re-run long commands just to see more output — use the stored output file first.
- Keep test log queries targeted: search for the specific error string, not the full log.
