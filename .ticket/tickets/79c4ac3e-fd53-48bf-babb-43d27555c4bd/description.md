## Story

Two orchestrated sessions ran in parallel on 2026-07-27 and were unexpectedly expensive. The cost was **not** in the top-level orchestrator — it made **zero** tool calls in both sessions and only planned and dispatched. The cost was in the fine-grained sub-agents that were supposed to be the cheap part: each delegation landed between **$1 and $3**, with **12 delegations per session**.

Sessions analysed (event logs under `.session/sessions/`):

| | `3e9bc20b` | `41966513` |
|---|---|---|
| Subagents | 12 | 12 |
| Total assistant turns | 269 | 227 |
| Turns inside subagents | 293* | 256* |
| Orchestrator's own tool calls | 0 | 0 |
| Terminal commands | 177 | 121 |
| Tool failures | 15 | 9 |
| Models used | Sonnet 4.5 x12 | Sonnet 4.5 x12 |

\*exceeds session total because two pairs ran in parallel and their spans overlap.

Method: both sessions log sub-agent activity inline — `runSubagent` start/complete brackets the child's own tool calls, with `turn_id` resetting per child. Segmenting the event stream by those spans yields per-subagent tool usage, turn counts, failures, and duplicate work.

## Root causes (measured, in impact order)

### 1. Every subagent carries a ~135-tool schema payload, and `tool_search` is disabled for them

Session `41966513` proved this empirically. Subagent `[2] Empirical subagent tool-list probe` reported verbatim:

> **Total tools available:** 164 — **MCP tools (135 total)** ... `tool_search` is not available in my current tool set.

Two subagents in `3e9bc20b` independently reasoned *"Tool search is disabled"* after a failure and re-planned around it.

Measured `tools/list` payloads:

| server | tools | schema chars |
|---|---|---|
| session-mcp | 35 | 23,896 |
| context-mcp | 3 | 12,898 |
| spec-mcp | 19 | 11,103 |
| rule-mcp | 16 | 8,448 |
| test-mcp | 6 | 8,096 |
| audit + feedback + peek + compact-terminal | 17 | 9,184 |
| ticket-mcp | 33 (lazy-reports 1) | ~20,000 est. |

That is **~94k characters, an estimated ~24k tokens of MCP schema** (character-derived, not tokenizer-measured), plus ~29 verbose built-in tool schemas, plus `AGENTS.md` (12.8 KB), a 40-entry instruction index, and an 11-entry skills index. Estimated fixed prefix: **~35-40k tokens re-sent on every single turn**.

**Measurement status.** The tool counts (164 total, 135 MCP), the per-server schema character counts, and every behavioural count in this epic are directly measured. The token figures are derived from character counts at an assumed ~4 chars/token and are **estimates pending `9d527ad1`**, which fixes the capture hook so real per-turn token data exists. Treat schema-payload dominance as the leading hypothesis, not a proven diagnosis.

Every agent template in `.agents/agents/` grants the full wildcard set — `'audit-mcp/*', 'context-mcp/*', 'feedback-mcp/*', 'log-viewer-mcp/*', 'rule-mcp/*', 'session-mcp/*', 'spec-mcp/*', 'test-mcp/*', 'ticket-mcp/*'` — including `explore.agent.md`, whose job is read-only lookup.

Cost arithmetic (estimated): a 40-turn subagent x ~37k prefix ~= **1.5M input tokens before a single byte of real work**. Add transcript growth and you land in the observed $1-3 range. A 3-turn subagent that reads one file still pays an estimated ~110k tokens of schema. These products multiply two estimates and are indicative only until `9d527ad1` lands.

### 2. Shell-first behavior inflates turn count

Terminal was the most-used tool in both sessions, and most of it duplicated already-loaded capabilities:

| category | `3e9bc20b` | `41966513` |
|---|---|---|
| `grep`/`find`/`ls`/`cat`/`wc` (peek-mcp / grep_search territory) | 83 | 18 |
| CLI shadowing an MCP tool (`ticket.exe get`, `spec.exe get`, `spec.exe health`) | 33 | 23 |
| `cargo run -p test-cli -- record` instead of `test_record_execution` | 3 | 0 |
| `cargo build`/`test` | 25 | 34 |

Subagent `[11] Materialize spec and validation files` ran **72 terminal commands in 42 turns**, including compiling `test-cli` and `spec-cli` from source at runtime. Subagent `[9] Implement ticket 41ff230b` spent 32 terminal calls on `find`/`ls`/`grep` after guessing `memory-api/session-api/` instead of `memory-api/crates/session-api/`.

Each shell round-trip is a full turn, so this multiplies root cause 1 directly.

### 3. Tool failures push agents onto expensive fallback paths

- `mcp_test-mcp_test_record_spec` x2 and `test_record_execution` x2 failed; `mcp_spec-mcp_spec_create` failed with the agent's own diagnosis: *"I need to provide an explicit workspace path, not just `default`."* The failure was opaque enough that the agent abandoned MCP entirely and rebuilt CLIs from source.
- 5 `read_file` failures in `3e9bc20b` and 3 `list_dir` failures in `41966513`, all wrong repo paths (`memory-api/session-api` vs `memory-api/crates/session-api`, `agent-tooling/peek-*` vs `memory-api/tools/`). Handoff packages carry crate names but not physical layout.

### 4. No context sharing; the same artifacts are re-read by 4-6 agents

| artifact | distinct subagents | total reads |
|---|---|---|
| `handoffs/dcf86212-*.json` | 6 | 14 |
| `compact-terminal-mcp/src/server.rs` | 6 | 21 |
| `.vscode/mcp.json` + `.github/mcp.json` | 3 | 10 |
| `.spec/specs/63c60c9d*/body.md` | 3 | 3 |

Subagents also read `orchestrator.agent.md`, `explore.agent.md`, and `iteration.agent.md` — burning tokens introspecting the delegation system itself. Cross-agent duplicate commands: `cargo test -p compact-terminal-cli` x5, `cargo test -p compact-terminal-mcp` x4, `git status --short` x4, MCP JSON-RPC handshake probe x4.

### 5. Rework chains from failed post-delegation gates

From the orchestrator's own messages:

- `@567` *"Schema issue found — commands were dropped."* The handoff package silently lost its validation commands, so **four consecutive subagents** (`Correct handoff validation section`, `Locate canonical handoff artifact`, `Fix canonical handoff validation`, `Verify validation gate schema` = 67 turns) chased one artifact.
- `@692` *"Implementation correctly blocked: no spec covers 41ff230b."* `Implement ticket 41ff230b` was dispatched, blocked, and re-dispatched — the second run cost **64 turns**, the largest single subagent in either session.
- `41966513` needed `Review` (42 turns) -> `Add integration tests` (25) -> `Re-review` (13) = 80 turns of review round-trip.

### 6. No model tiering at all

All 24 delegations passed `model: "Claude Sonnet 4.5 (copilot)"`. **No agent template declares a `model:` field**, so routing is entirely the orchestrator's per-call choice, and it always chose the same tier. Sonnet 4.5 ran:

- `Compact the spilled briefing` — 3 turns, 1 `read_file`, pure summarization
- `Locate canonical handoff artifact` — 8 turns, pure file lookup
- `Empirical subagent tool-list probe` — 3 turns, zero tool calls
- `Commit compact-terminal-api extraction` — 28 turns, 26 of them `git status`/`git diff`/`git commit`

## Goal

Make delegation cheap enough that fine-grained sub-agent decomposition is the economical choice rather than a cost multiplier. Target: **>=60% reduction in per-delegation cost** at equal or better task outcome, verified against recorded per-turn token/cost telemetry.

## Child work

1. Scope MCP tool grants per agent template and restore lazy tool discovery for sub-agents — the dominant lever.
2. Enforce MCP-over-shell in agent templates and guidance.
3. Carry physical repo paths in handoff packages and delegation prompts.
4. Fix `workspace` parameter semantics and error messages across spec-mcp / test-mcp.
5. Pass a shared context bundle to fan-out siblings instead of letting each rediscover it.
6. Add `model:` to agent templates and route by agent class.
7. Move gates before dispatch: orchestrator validates preconditions itself.
8. Promote the cost analyzer into session-api with real token attribution.
9. Define a synthetic benchmark session with a checked-in baseline, so every cost claim above is falsifiable.

## Acceptance Criteria

1. A freshly spawned Explore Agent reports a tool count and schema payload reduced by >=60% versus the measured 172-tool / ~24k-token baseline recorded in [cd19fed4](.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0/ticket.toml), using [the regression probe](.agents/prompts/tool-grant-regression-probe.prompt.md) as the evidence source.
	Rationale: the earlier 164-tool / ~37k-token figures were superseded by cd19fed4's measured baseline.
2. Either `tool_search` is available to sub-agents, or every agent template declares an explicitly scoped, non-wildcard tool set justified by its role.
3. At least two agent classes are routed to a model tier cheaper than Sonnet 4.5, declared in the agent template rather than chosen ad hoc per call.
4. Measured against the synthetic benchmark defined in `10d21210`, sub-agent turn count and substitutable-shell command count both drop versus the checked-in baseline, by the thresholds that ticket sets.
5. Post-delegation gate failures that caused re-dispatch in these sessions (missing spec, dropped validation commands) are caught pre-dispatch instead.
6. Cost reduction is demonstrated with recorded telemetry, not estimated. Requires `9d527ad1` so `data_json.usage` is non-zero; this epic depends on it directly, not only through `b7c61f0e`.

## Evidence

- Session event logs: `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`, `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json`
- Analysis probe: `tmp/subagent_cost_probe.py`
- Agent templates: `.agents/agents/`
- MCP server registry: `.vscode/mcp.json`, `.github/mcp.json`

## Related active work

- `9d527ad1` — capture hook does not populate `data_json.usage`; blocks measuring any of this with real numbers.
- `6549b6a7` — session store records per-turn / per-sub-agent token and cost with model attribution; the measurement substrate.
- `41ff230b` — quality gates and session/tool-call data collection for delegated sessions; the session that surfaced this.
- `445a2d76` — model price awareness / orchestrator-mode enforcement; the routing mechanism this epic extends.
- `9b9df133`, `e342cc4c` — token-efficiency tooling rollout; peek/compact-terminal are the MCP replacements for the shell calls counted above.
- `8c67b96a`, `0d3fdba6` — handoff package ownership and completeness gate; root causes 3 and 5.