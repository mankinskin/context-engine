## Problem

The entire analysis behind epic `79c4ac3e` was produced by an ad-hoc script written into `tmp/subagent_cost_probe.py` during a single session. It is not durable, not tested, and not reachable from any tool surface. Every future cost investigation would start from scratch.

Worse, the analysis had to reason about cost **indirectly** — via turn counts, tool-call counts, and measured schema payloads — because `data_json.usage` is not populated by the capture hook (`9d527ad1`). Per-session `tool-metrics.json` is written but empty (`"tools": {}`), and no spill accounting is recorded (`has_spill` was `false` for all 711 tool completions across both sessions despite several outputs exceeding 20 KB).

Without real token/cost attribution, none of the sibling tickets in this epic can prove their effect. Every acceptance criterion that says "cost drops by N%" is currently unverifiable.

## What the analysis needed and had to reconstruct manually

- Sub-agent span segmentation: `runSubagent` `tool.execution_start` / `tool.execution_complete` brackets the child's own events, and `turn_id` resets per child. Parallel fan-out produces overlapping spans that must be attributed carefully — in `41966513` spans `[0]` (events 6-149) and `[1]` (7-216) overlap, so a naive segmenter double-counts 46 tool calls.
- Per-sub-agent: turn count, tool-call histogram, reasoning-token volume, failure list with recovery reasoning.
- Cross-agent duplicate reads keyed by normalized path — the same handoff file appeared under both forward-slash and backslash spellings and had to be reconciled by hand.
- Terminal command classification into substitutable vs legitimate categories.
- Fixed-prefix estimation, which required probing every MCP server's `tools/list` over stdio by hand.

## Scope

- Promote the probe into a supported analyzer in `session-api`, exposed via `session-cli` and `session-mcp`, producing a per-session delegation cost report.
- Report shape, per sub-agent: agent name, description, declared model, turn count, tool histogram, failures with codes, repeat reads, substitutable-shell count, and — once `9d527ad1` lands — real input/output/cached tokens and cost.
- Roll up to a session-level summary: total delegations, cost distribution, top duplicate artifacts, top duplicate commands, rework chains (same task dispatched more than once).
- Normalize path spellings before deduplication.
- Handle overlapping parallel spans explicitly rather than double-counting.
- Populate the empty per-session `tool-metrics.json`, and record spill occurrence and size.
- Fix or remove `has_spill`, which reported `false` for every completion in both sessions.
- Delete `tmp/subagent_cost_probe.py` once superseded.

## Acceptance Criteria

1. A supported command reproduces the epic's analysis for any captured session without ad-hoc scripting.
2. Parallel sub-agent spans are attributed without double-counting.
3. Duplicate-read detection is path-normalization safe.
4. `tool-metrics.json` is non-empty for newly captured sessions.
5. `has_spill` correctly reflects spilled tool output.
6. Once `9d527ad1` lands, the report carries real token and cost figures per sub-agent, not derived estimates.
7. The report is the evidence source for the acceptance criteria of every sibling ticket under `79c4ac3e`.

## Evidence

- Throwaway analyzer to be superseded: `tmp/subagent_cost_probe.py`
- Empty metrics: `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/tool-metrics.json`
- Overlapping-span example: `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json` events 6/7 and 240/242
- Blocking telemetry gap: `9d527ad1`
- Measurement substrate: `6549b6a7`, `41ff230b`

## Review note: why this is not premature

Reviewed 2026-07-27. One reviewer argued this is "building the dashboard before the sensor works" and should be demoted to a research note until `9d527ad1` lands.

**Kept as a child.** The analyzer's non-token metrics — sub-agent turn counts, cross-agent duplicate reads, substitutable-shell classification, failure taxonomy, rework-chain detection — need no token data at all, and produced the entire investigation behind epic `79c4ac3e` without it. `9d527ad1` gates only the cost column. Blocking the whole analyzer on it would strand the metrics that are computable today and that five sibling tickets need for verification.

The `depends_on 9d527ad1` edge already correctly expresses that the cost attribution specifically is blocked.

## Scope boundary vs `8ad2581e`

`8ad2581e` (Delegation quality/cost metric and self-optimization loop) is **forward-looking**: a rolling-window composite score per model producing a "cheapest model meeting standards" recommendation.

This ticket is **retrospective and diagnostic**: given one captured session, explain *why* a delegation was expensive — duplicate reads, substitutable shell calls, rework chains, failure-driven fallbacks, fixed-prefix size.

They compose: this analyzer produces the per-delegation cost attribution and waste classification that `8ad2581e` scores. Do not merge them; do share the extraction layer.