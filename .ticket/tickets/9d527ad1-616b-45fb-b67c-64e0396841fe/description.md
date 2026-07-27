## Objective

Instrument the Copilot capture hook so that token, cost, and model usage data actually reaches the session store. The backend infrastructure landed with ticket `6549b6a7` is complete but reads zero in every real session because the upstream capture source never supplies the data.

## Problem

A post-hoc audit of `6549b6a7` (2026-07-27, against memory-api commit `8a8fb34`) confirmed all backend criteria are MET:

- `SessionTurnEventMeta` defines `input_tokens` / `output_tokens` / `cache_read_tokens` / `cache_write_tokens` / `cost_usd` / `model_id` — memory-api/crates/session-api/src/model.rs (L227-238)
- `compute_cost_usd` — memory-api/crates/session-api/src/price_loader.rs (L62-104), wired into persistence at memory-api/crates/session-api/src/store/config/persistence.rs (L265-283)
- `compute_subagent_rollups` per-sub-agent aggregation — memory-api/crates/session-api/src/subagent_rollup.rs

However, every `event_meta` token/cost field is null in sessions captured *after* the implementation landed. The extraction logic at memory-api/crates/session-api/src/hook.rs (L251-290) reads from `data_json.usage`, and the VS Code Copilot capture hook does not populate that field.

## Requirements

- Populate `data_json.usage` in capture payloads with input/output/cache-read/cache-write token counts and the model id from the Copilot API response.
- Ensure the values flow through `hook.rs` extraction into persisted `event_meta`.
- Confirm per-sub-agent rollups report non-zero token and cost values for a real delegated session.

## Acceptance criteria

- A newly captured session has non-null, non-zero token counts and `model_id` in its persisted `event_meta`.
- `session subagent-rollups --workspace-session-id <id>` returns non-zero token and cost totals for a session containing sub-agent delegations.
- Cost values are consistent with the token counts and the price table used by `compute_cost_usd`.
- A regression test asserts that a capture payload carrying `usage` produces populated `event_meta` token/cost/model fields.

## Anchor

Unblocks the cost half of the delegation quality/cost metric (ticket `8ad2581e`). Backend consumer work in ticket `41ff230b` can proceed without this, since it relies on model attribution rather than token/cost magnitudes, but `8ad2581e` will report zero cost until this lands.