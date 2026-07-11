---
description: "Use when editing agent workflow guidance, CLI output handling, or file inspection patterns. Covers compact-by-default output, TOON vs JSON, bounded inspection, and differential patching."
applyTo: "AGENTS.md,.github/copilot-instructions.md,.agents/instructions/*.instructions.md,.agents/prompts/*.prompt.md,.agents/agents/*.agent.md,tools/cli/peek-cli/**,tools/mcp/compact-terminal-mcp/**"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=4135e465-dc19-4966-892c-b232e062346b slug=context-engine/instructions/token-efficiency/l1 -->

## Token-Efficient Agent Workflow

These rules apply during every session to keep token consumption bounded and debuggability intact.

The primary objective is to reduce what reaches the model API in the first place. Post-hoc transcript capture is diagnostic only; it does not make the current request cheaper.

### Model Cost Awareness & Routing

Token cost is a function of *which model* does the work, not only how much context it sees. Be model-cost-aware in every session: reserve expensive, high-capability models for work that genuinely needs them, and delegate routine or bulk work to smaller, cheaper models.

This matters most in sessions driven by a large, expensive model. Treat that model as an active **router**: it plans and reasons at a high level, then dispatches routine subtasks to cheaper models via `runSubagent` (passing an explicit cheaper `model`) instead of spending premium tokens itself.

**Tiered model ladder (smartness vs cost):**

| Tier | Use for | Avoid for |
|---|---|---|
| High-capability (most expensive) | Large-scope planning, cross-cutting architecture, high-level reasoning, review of dense content or a single critical artifact, final synthesis/decisions | Command batches, bulk output summarization, mechanical edits, wide file sweeps |
| Mid-tier (balanced) | Focused implementation on a known slice, targeted debugging, moderate multi-file edits | Long research sweeps that a cheap model can pre-digest |
| Cheap/fast (least expensive) | Running and summarizing command/tool-call batches, condensing large tool outputs, summarizing many large files or artifacts, mechanical extraction, first-pass research triage | Final architectural decisions, subtle correctness review of dense artifacts |

**Delegation rules:**
- In a large-model session, delegate to a cheaper subagent model when the subtask is: a batch of command or tool calls, summarization of large or numerous tool outputs, or research/summarization across many large files or artifacts.
- Give the subagent a self-contained prompt (it does not inherit session context) and pin the intended cheaper `model` explicitly on `runSubagent`.
- Ask the subagent to return only the distilled finding — scope, result, blocker, pointer — not raw output. The expensive model reasons over the summary, not the bulk.
- Escalate a subtask back up a tier only when the cheaper model's result is insufficient, and record why.
- Reserve the high-capability tier for planning, high-level reasoning, and review of dense content or individual artifacts.

**Inspection before delegation or premium reasoning:**
- Use bounded inspection tooling (`peek` CLI, `repo_map.toon`, interface skeletons) to render reduced, focused views of artifacts before either spending expensive-model tokens or handing the artifact to a subagent.
- A focused, reduced view is often enough for the expensive model; the full artifact usually is not needed.

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

### Session Artifact Reading

When the task involves prior sessions, transcript inspection, handoff recovery, or Copilot chat artifacts, follow [session-optimization instructions](./session-optimization.instructions.md).

Rules:
- Do not read raw session transcript files into the model prompt by default.
- Prefer the smallest durable artifact first: ticket, spec, handoff, validation note, or compact session summary.
- Treat raw transcript JSON, event streams, and chat-session resource payloads as `reference-only` unless a bounded slice is required to answer one specific question.
- Do not replay raw `toolRequests`, empty `reasoningText`, duplicated tool lifecycle events, or full spill-file bodies when a one-line summary or targeted extraction is sufficient.
- When you need evidence from a prior session, normalize it to: scope, finding, outcome, blocker, and pointer.

### Routine Action Discipline

Do not spend reasoning budget on actions whose next step is already obvious from the current local hypothesis.

Examples:
- If the touched slice has one relevant test, run it instead of narrating why that is probably the right test.
- If a command failed because of cwd drift, rerun it from the correct directory instead of exploring multiple explanations.
- If the correct tool is already loaded and known, call it instead of searching for it again.

Rules:
- Prefer direct execution over explanatory self-talk for routine operations.
- Avoid repeating unchanged state checks such as `git status`, board reads, or ticket fetches unless a write or external change occurred.
- After a long command spills output, inspect the spill artifact directly instead of re-running the command.
- Convert retries into one-line findings in subsequent summaries.

### Tool Result Guarding

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
