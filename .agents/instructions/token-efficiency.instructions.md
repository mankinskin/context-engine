---
description: "Use when editing agent workflow guidance, CLI output handling, or file inspection patterns. Covers compact-by-default output, TOON vs JSON, bounded inspection, and differential patching."
applyTo: "**"
---

## Token-Efficient Agent Workflow

These rules apply during every session to keep token consumption bounded and debuggability intact.

### Compact-by-Default Output

All CLI commands that support it should produce **compact output by default**. Verbose or full output is on-demand only.

| Situation | Preferred form | Verbose fallback |
|---|---|---|
| Machine-readable output | `--toon` | `--json` |
| Structured readable output | `--toon` or default | `--json --verbose` |
| Human scanning | default (no flag) | `--verbose` or `--json` |

Rules:
- Prefer `rtk <cmd>` over bare `<cmd>` — rtk filters/compresses output automatically.
- When a CLI supports `--toon`, prefer `--toon` over `--json` for compact machine-readable output.
- Never request `--json` output and then discard most of it; use a targeted filter (jq, toon-rust) instead.
- For ticket CLI: default text output is already compact; use `--json` only when you need to extract fields with jq or pipe to another tool.

### TOON vs JSON

- **TOON** (`--toon`): compact, binary-ish encoding. Prefer for data exchange between tools in the same pipeline.
  Use `toon-format` / `toon-rust` crates for encode/decode instead of hand-rolled text transforms.
- **JSON** (`--json`): verbose but universally parseable. Use when piping to external tools (jq, Python, etc.) or when debugging schema issues.
- **Do not** request JSON output when a plain-text or TOON representation would suffice — JSON adds 40–80% token overhead for the same data.

### Bounded File Inspection

Never pull an entire file when only a targeted slice is needed.

Preferred pattern:
1. Use `repo_map.toon` (`repo_map.toon` at the repository root) for structural orientation before opening any file.
2. Use an interface skeleton view before reading full source when available.
3. Open a bounded line window with explicit start/end coordinates.
4. Only escalate to a full-file read when the bounded window is genuinely insufficient.

Use the `peek` CLI tool (`tools/cli/peek-cli`) for bounded reads from the terminal:

```bash
# Step 1: learn file size
peek path/to/file.rs --count

# Step 2: locate the target (returns matching line numbers)
peek path/to/file.rs --grep "fn my_function"

# Step 3: read a tight window
peek path/to/file.rs --start 42 --end 80

# Step 4: show context around a pattern match
peek path/to/file.rs --grep "fn my_function" --window 15

# Escape hatch (explicit, token-expensive)
peek path/to/file.rs --all
```

When the required line coordinates are unknown, use `grep_search`, `semantic_search`, or `peek --grep` to locate the target region first, then read a bounded window around it.

Full-file reads should become the exception. The `--all` flag is intentionally named to make the cost visible in command history.

### Differential Patching

When editing files, always use the narrowest applicable edit operation:

- `replace_string_in_file` with 3–5 lines of context — preferred for surgical changes.
- `multi_replace_string_in_file` — batch multiple independent replacements in one call.
- Only use `create_file` when creating a new file from scratch.
- **Never** read a full file and rewrite it wholesale to make a small change.

### Compact Terminal Expectations

The `rtk` proxy and the compact-terminal MCP tool (`tools/mcp/compact-terminal-mcp`) truncate long outputs automatically. Long outputs are stored in a transient file and can be inspected via bounded search/read tools:

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

### Structural Awareness Before Exploration

Before running exploratory searches or broad file reads, consult compact structural sources first:

1. **`repo_map.toon`** — compact workspace map at the repository root. Read this first for directory/crate layout.
  Refresh with `cargo run -p peek-cli -- . --repo-map --output repo_map.toon` when crates or agent files change.
2. **Interface skeletons** — stripped function/type signatures without bodies (when available).
3. **`CHEAT_SHEET.md`** — API patterns, common gotchas.
4. **Crate `README.md`** and `HIGH_LEVEL_GUIDE.md` — design context.

Only fall back to broad `semantic_search` or exploratory file listing when the compact sources are insufficient.

### Pre-flight Write Validation

The `tools/agent-hooks/preflight-write.sh` hook runs automatically as a `PreToolUse` hook before file-write operations (`create_file`, `replace_string_in_file`, `multi_replace_string_in_file`). It:

- Runs `cargo check` for `.rs` files (nearest Cargo.toml).
- Runs `python3 -m py_compile` for `.py` files.
- Runs `bash -n` for `.sh` files.
- Parses TOML for `.toml` files (advisory, non-blocking).
- Runs `tsc --noEmit` for `.ts`/`.tsx` files (advisory, non-blocking).

**If a check fails (blocking):** The write is rejected with a diagnostic. Fix the syntax error before the tool call is retried.

**If a checker is unavailable:** A warning is emitted but the write is allowed. Record the missing checker gap in the ticket/spec status summary.

**Bypass:** Add `--no-verify` to the git commit or set `SKIP_PREFLIGHT=1` in the environment when the check is a false positive. Document why in the commit message.

### Fallback Escalation

When compact tooling is unavailable or insufficient:
1. Note the limitation in the ticket/spec status summary.
2. Use the next-best available tool (e.g., bounded grep instead of full file read).
3. Do not silently fall back to full-file pulls — record the gap explicitly so the tooling can be improved.
