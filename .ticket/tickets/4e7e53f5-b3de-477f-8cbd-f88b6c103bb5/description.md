## Overview
Implement the graded cost model (1–100 scale) for tool classification and budget-based gating decisions.

## Key Design Points
- **Graded cost scale**: 1–100 configurable scale with named tiers layered on top
- **Gate decision**: `effective_budget = base_budget(model) + offset(grant)`; allow if `tool.cost ≤ effective_budget`
- **tool.cost**: LINEAR map of empirical est-output-tokens (from tool_metrics T1/T2) → 1–100, clamped
  - Tools with insufficient data fall back to existing static heavy/light list (TOKEN_HEAVY_TOOL_SUBSTRINGS)
- **base_budget(model)**: LINEAR inverse of `output_mtok` from tools/model-prices/model_prices.json
  - Cheap model → high budget, expensive → low
  - Tunable anchor constants calibrated so today's threshold X=15 boundary + current heavy list is reproduced as default
- **offset resolution**: looked up from DURABLE grant record via `grant_id`/`session_id` (offset value is NEVER self-declared by caller)
- **Rust + Python parity**: update memory-api/tools/mcp/mcp-cost-gate/src/gate.rs and tools/model-prices/cost_gate.py with matching logic
- **AGENTS.md update**: update "Model cost awareness & routing" orchestrator section to describe graded budget model

## Acceptance Criteria
- [ ] Linear tool.cost mapping from est-tokens with tunable anchors + clamping
- [ ] Linear inverse base_budget from output_mtok with tunable calibration
- [ ] Gate decision: allow if cost ≤ base+offset
- [ ] Static-list fallback for insufficient-data tools (TOKEN_HEAVY_TOOL_SUBSTRINGS)
- [ ] grant_id lookup (offset never passed raw per call)
- [ ] Python/Rust parity: matching logic in cost_gate.py and gate.rs
- [ ] Calibration: default settings reproduce today's X=15 threshold behavior
- [ ] Tests: linear mappings, clamping, fallback, parity
- [ ] AGENTS.md "Model cost awareness & routing" updated with graded model

## References
- Depends on: T1 (tool_metrics core), T5 (grant records)
- Targets: memory-api/tools/mcp/mcp-cost-gate/src/gate.rs, tools/model-prices/cost_gate.py, AGENTS.md