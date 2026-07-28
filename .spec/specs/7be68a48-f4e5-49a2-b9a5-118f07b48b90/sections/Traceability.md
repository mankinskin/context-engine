## Implementing Tickets

- [9d527ad1 Per-tool-call token-load telemetry via mcp-cost-gate (proxy observes payloads, not usage)](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) — implements R4 (token-load coverage) and AC3/AC6 of this spec.
- [b7c61f0e Promote the sub-agent cost analyzer into session-api with real token attribution](.ticket/tickets/b7c61f0e-ed42-4eef-8d3b-da934d7c0628/ticket.toml) — done; implements the R2/R3 analyzer surfaces described in the "Delegation Cost Analyzer" section (`delegation_cost.rs`, `subagent_rollup.rs`, `tool_metrics.rs`, `quality_gate.rs`). Review verdict pass-with-note; `cargo test -p session-api` 195 passed / 0 failed at review time.
- [10d21210 Define a synthetic benchmark session with a checked-in baseline](.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml) — done; extended the analyzer with `model_distribution`, `substitutable_shell_count`, `exploratory_find_ls_count`, `path_resolution_failures`, `redispatch_count`, and the `compute_delegation_cost_report_from_events` entry point documented above. Also recorded the honest AC6/R4 limitation: both checked-in baseline sessions predate 9d527ad1's token capture, so `data_json.usage` (and therefore all token/cost fields) is legitimately `0`/`null` in that specific baseline, not an analyzer defect.

## Implementation

- [memory-api/crates/session-api/src/delegation_cost.rs](memory-api/crates/session-api/src/delegation_cost.rs) — `DelegationCostReport` (line 148), `compute_delegation_cost_report` (line 194), `compute_delegation_cost_report_from_events` (line 557).
- [memory-api/crates/session-api/src/subagent_rollup.rs](memory-api/crates/session-api/src/subagent_rollup.rs) — `SubAgentRollup` (line 8), `compute_subagent_rollups` (line 36).
- [memory-api/crates/session-api/src/tool_metrics.rs](memory-api/crates/session-api/src/tool_metrics.rs) — `compute_session_summary` (line 161), `aggregate` / `aggregate_with_cost` (lines 269/278), `write_rollup` (line 474).
- [memory-api/crates/session-api/src/quality_gate.rs](memory-api/crates/session-api/src/quality_gate.rs) — `QualityGatePhase` (line 12), `QualityGateOutcome` (line 22), `QualityGate` (line 38), `pre_delegation_gate` (line 91), `post_delegation_gate` (line 107).
- [memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs](memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs) — the `CallTelemetry` struct records request/response byte and char counts and the derived `tokens_estimated` field for each MCP tool call the proxy observes.

## Validation Evidence

- `cargo test -p mcp-cost-gate` — baseline 50 passed / 0 failed. See [memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs](memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs).
- `cargo test -p session-api` — 195 passed / 0 failed at b7c61f0e review; 196 passed / 1 ignored (generator test) at 10d21210 completion; `cargo clippy -p session-api --all-targets` clean (no new warnings) per 10d21210's status summary.
- [memory-api/crates/session-api/tests/delegation_cost_benchmark.rs](memory-api/crates/session-api/tests/delegation_cost_benchmark.rs) — `replay_reproduces_checked_in_baseline_report_exactly` replays both checked-in baseline event logs through `compute_delegation_cost_report_from_events` and asserts exact JSON equality with the checked-in `.benchmark/10d21210/baseline/delegation_cost_report.json`.

## Non-Regression Constraint

- [memory-api/crates/session-api/src/store/config/persistence.rs](memory-api/crates/session-api/src/store/config/persistence.rs) must remain untouched by token-load telemetry work: `cost_usd` stays `null` in `session-api` records, per the honest-absence constraint in R4.

## Related Ticket

- [7de9f4f0 Completion-claim audit: require verified-by evidence before a ticket may reach done](.ticket/tickets/7de9f4f0-0189-40c7-ac0a-0669e2aab57c/ticket.toml) — exists because two prior `done` revisions in ticket 9d527ad1's `history.ndjson` were recorded against work that did not exist; this audit ticket tracks the review-integrity follow-up.

## Known Gap — Epic AC6 Real-Telemetry Baseline

The 10d21210 benchmark is replay-only: both checked-in baseline sessions
predate 9d527ad1, so `input_tokens`/`output_tokens`/`cost_usd` are honestly
all-zero in `.benchmark/10d21210/baseline/`. A before/after comparison against
that baseline would be real-telemetry-vs-zero, not a valid measurement.
Tracked by a dedicated follow-up ticket
([5cbae4be Capture a real post-9d527ad1 delegation session and replay it
through the 10d21210 harness for epic AC6 evidence](.ticket/tickets/5cbae4be-9f62-49ca-827e-44bed8242bc6/ticket.toml),
depends on 10d21210, linked to epic 79c4ac3e and 9d527ad1) to capture a
fresh post-9d527ad1 session and replay it through this same harness to
produce real before/after evidence.
