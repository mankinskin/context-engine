## Problem

Session 51701334 (instruction-file migration) cost ~$9 total; the orchestrator model used only ~$0.90, so ~90% of spend went to sub-agents. This spend is currently **unattributable from the session store**.

Investigation of `.session/sessions/51701334-cf77-4a4b-97f3-5df8753631e1/events.json` (86k lines, 554 tool executions) found **zero** `tokens`/`cost`/`usage`/`model_id`/`price` fields anywhere. `model` survives only as a `runSubagent` argument, never as per-turn attribution. `session.json` has no cost fields.

Result: we cannot answer "which sub-agent / which model / which loop cost the most," which is exactly the question needed to tune the orchestration/delegation policy.

## Goal

Capture, per assistant turn and per sub-agent spawn:
- input/output/cache token counts
- resolved USD cost (via the existing model price table `tools/model-prices/model_prices.json`)
- the acting `model_id` (per turn, not just as a runSubagent arg)

Aggregate a per-sub-agent rollup (model, turns, tools, tokens, cost, wall time, outcome) so a spawn's total cost is directly inspectable.

## Acceptance criteria
- events.json (or a sibling capture) records token + cost + model_id per turn.
- A per-`runSubagent` cost rollup is queryable (e.g. via session-mcp or session-cli).
- Re-running a delegating session lets a reviewer attribute total cost across sub-agents without external tooling.

## Evidence
- events.json L44080: 31.2-min sub-agent, no cost recorded.
- events.json L80467: 20.1-min sub-agent, no cost recorded.
- Owning code: memory-api/crates/session-api, memory-api/tools (session-mcp/session-cli).

## Validation Evidence (recorded 2026-07-27, post-hoc audit)

Audited against memory-api commit `8a8fb34`. Verdict: implementation architecturally complete; one upstream data-pipeline gap.

| Criterion | Verdict | Evidence |
|---|---|---|
| Per-turn token counts recorded | MET | `SessionTurnEventMeta` fields `input_tokens` / `output_tokens` / `cache_read_tokens` / `cache_write_tokens` in memory-api/crates/session-api/src/model.rs (L227-234) |
| Per-sub-agent rollup queryable | MET | `SubAgentRollup` + `compute_subagent_rollups` in memory-api/crates/session-api/src/subagent_rollup.rs; query surface `SessionStoreConfig::subagent_rollups` in memory-api/crates/session-api/src/store/config/subagent_rollup_query.rs |
| Cost computed and recorded | MET | `compute_cost_usd` in memory-api/crates/session-api/src/price_loader.rs (L62-104); wired into persistence at memory-api/crates/session-api/src/store/config/persistence.rs (L265-283); `cost_usd` field at model.rs L236 |
| Model attribution recorded | MET | `model_id` at memory-api/crates/session-api/src/model.rs L238; `SessionTurn.model` at L266-270 |
| Real telemetry present in on-disk artifacts | NOT MET | All `event_meta` token/cost fields are null in sessions captured after the implementation landed. Root cause: the VS Code Copilot capture hook does not populate `data_json.usage`, which the extraction logic at memory-api/crates/session-api/src/hook.rs (L251-290) reads from. Gap is in the upstream capture source, not in session-api. |

Tests: `cargo test --package session-api --lib subagent_rollup::tests` (2 passed) and `cargo test --package session-api price_loader` (3 passed).
CLI: `session subagent-rollups --workspace-session-id <id>` executes and returns rollups (zero-valued until the capture source supplies usage data).

Known follow-up: instrument the Copilot capture hook to populate `data_json.usage` with token counts and model id. Until then all token/cost telemetry reads zero despite complete backend infrastructure.