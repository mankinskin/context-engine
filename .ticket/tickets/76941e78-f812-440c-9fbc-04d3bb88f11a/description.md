## Problem

Tool output size is the one number the entire graded-cost design depends on (`graded-cost-scale.md`: "LINEAR map of empirical `est-output-tokens`"), and it has never been captured. The raw Copilot transcript's `tool.execution_complete` payload is only `{success, toolCallId}`.

Three candidate sources exist. **Nobody has ever established which of them actually carries the data.** This ticket is a bounded probe, not an implementation. It exists first because the whole prior track's failure mode was building consumers before proving the producer.

## Candidate sources to probe

1. **PostToolUse hook payload (highest value, unverified).** `.github/hooks/hooks.json` already runs `copilot-capture-hook --from-hook-stdin` on `PostToolUse`. Sibling scripts read `tool_input` from that same stdin — see [tools/agent-hooks/preflight-write.sh](tools/agent-hooks/preflight-write.sh) (`data.get('tool_input', {})`) and [tools/agent-hooks/validate-docs.sh](tools/agent-hooks/validate-docs.sh). Today [args.rs](memory-api/crates/session-api/src/bin/copilot-capture-hook/args.rs) `args_from_hook_stdin` extracts only `transcript_path`, `workspace_slug`, `hook_event_name` and **discards the entire rest of the payload**. If the payload carries `tool_response` / `tool_output`, we get exact output size for every tool with no new plumbing.
2. **Spill files (verified present).** `chat-session-resources/<session-id>/<tool_call_id>__vscode-<ts>/content.txt`. Confirmed 2026-07-29: the directory-name prefix before `__` **is** the `tool_call_id` from `events.json`, so byte counts are directly joinable. Covers spilled (large) results only — which are exactly the expensive ones. `has_spill` / `spill_pointer` already exist in [tool_execution.rs](memory-api/crates/session-api/src/hook/tool_execution.rs).
3. **MCP proxy telemetry (exists, switched off).** `mcp-cost-gate` computes `request_chars` / `response_chars`, but `COST_GATE_TELEMETRY_LOG` is unset in both [.vscode/mcp.json](.vscode/mcp.json) and [opencode.json](opencode.json), so records go nowhere. Tracked separately as 4aa13ba7. Covers MCP tools only.

## Acceptance criteria

- AC1 — A captured, checked-in sample of the **actual** `PostToolUse` hook stdin payload exists in the repo as a fixture, obtained by dumping real stdin to a file. Verified by reading the fixture, not by reasoning about the hook contract.
- AC2 — The probe states, per source, exactly which field yields output size, its units (bytes vs chars), and what fraction of this session's tool calls it covers. A source that turns out to carry nothing is recorded as such — a negative result closes this ticket successfully.
- AC3 — A written source-precedence recommendation for T2, ranked by coverage × fidelity.
- AC4 — No production code path is changed by this ticket.

## Non-goals

- Implementing capture. That is the child ticket.