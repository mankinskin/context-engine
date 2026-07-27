## Problem
`Gate::tool_cost()` in memory-api/tools/mcp/mcp-cost-gate/src/gate.rs falls back to a hardcoded static classification whenever the empirical rollup lacks data for a tool. Tools matching `TOKEN_HEAVY_TOOL_SUBSTRINGS` are assigned `heavy_fallback_cost()` (75 at the default calibration), which exceeds an expensive model's budget (~58). Because .session/tool-metrics-rollup.json is currently empty (`turn_count: 0`, `tools: []`), no tool ever reaches the empirical path, so read_file/peek_*/grep_search/get_log/spec_get/get_ticket_description/subgraph are permanently blocked for large models. This is assumed cost, not proven cost, and it prevents the very measurements that would make the gate accurate.

## Decisions (approved)
1. Fail-open for unmeasured tools. A tool with no rollup entry gets cost **0** — a true bypass, gate inert.
2. Remove `TOKEN_HEAVY_TOOL_SUBSTRINGS` **entirely**. No hardcoded token-heavy constants, no `heavy_fallback_cost()`, no `ToolClass::TokenHeavy`, and no remaining references to those names.
3. Remove `ALWAYS_ALLOWED_TOOL_SUBSTRINGS` **entirely** as well. No hardcoded lists at all — every cost is empirical or zero.
4. Threshold: `MIN_CALLS` drops to **1**. A single recorded measurement is enough to start gating that tool; cost is the average over the existing rollup window.
5. Rolling window: reuse the existing rollup aggregation window. Do **not** add a new window knob.
6. Unconditional behavior change. No env flag, no escape hatch.
7. Rust only. `tools/model-prices/cost_gate.py` is the advisory helper and may lag; do not change it in this ticket.
8. Fixing the empty measurement pipeline (the rollup writer in memory-api/crates/session-api/) is **out of scope** here and is already tracked by existing tool_metrics tickets.

## Scope of change
- memory-api/tools/mcp/mcp-cost-gate/src/gate.rs
  - Delete `TOKEN_HEAVY_TOOL_SUBSTRINGS`, `ALWAYS_ALLOWED_TOOL_SUBSTRINGS`, `ToolClass`, `classify_tool()`, and `heavy_fallback_cost()`.
  - `tool_cost()` becomes: look up the tool in the rollup; if present with `call_count >= MIN_CALLS`, return the graded cost; otherwise return 0.
  - `MIN_CALLS` = 1.
  - `evaluate()` keeps its existing shape: cost 0 → allow; `cost <= effective_budget` → allow; else delegate. Grants and the offset mechanism are unchanged.
- Remove any now-dead imports/helpers left behind.

## Acceptance criteria
- AC1: `grep -ri "token_heavy\|TOKEN_HEAVY\|always_allowed\|ALWAYS_ALLOWED\|heavy_fallback" memory-api/tools/mcp/mcp-cost-gate/src/` returns no matches.
- AC2: With an empty or missing rollup, `Gate::evaluate()` allows every tool for every known caller_model, including previously heavy ones (read_file, peek_read, grep_search, get_log, get_ticket_description, spec_get, subgraph).
- AC3: With a rollup entry having `call_count >= 1` and a graded cost above an expensive model's budget, that tool is delegated (blocked) for that model and allowed for a cheap model. Regression test covers both sides.
- AC4: An unknown `caller_model` is still rejected — that behavior is unchanged.
- AC5: The existing missing-price-table fail-open in main.rs `load_gate()` and proxy.rs is unchanged.
- AC6: `cargo test -p mcp-cost-gate` passes; tests asserting the old static fallback of 75 are replaced by tests asserting the fail-open-to-0 behavior.
- AC7: Manual smoke check: an expensive-model MCP call to `get_ticket_description` succeeds instead of returning the "requires cost 75 but model has effective budget 58" error.

## Non-goals
- Populating or repairing the tool-metrics rollup writer.
- Argument-based dynamic cost estimation (tracked separately).
- Re-tuning the graded-cost calibration constants (tracked separately).
- Changes to tools/model-prices/cost_gate.py or its Python tests.
