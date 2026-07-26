## Problem
The graded cost-gate calibration constants shipped as provisional placeholders:
- `GradedCostCalibration.tokens_at_max = 8000.0` (session-api tool_metrics)
- `ModelBudgetCalibration.budget_zero_price = 60.0` (mcp-cost-gate gate.rs + cost_gate.py)

These were chosen to reproduce the legacy X=15 binary boundary (heavy_fallback_cost = 75), not derived from observed data.

## Scope
- After the tool-metrics rollup accumulates real per-tool call data (>= MIN_CALLS=5 per tool), re-derive tokens_at_max and budget_zero_price from the empirical distribution.
- Keep Rust (gate.rs) and Python (cost_gate.py) parity when updating.
- Re-run parity tests + record evidence.

## Acceptance
- Calibration constants updated from real rollup data (or explicitly confirmed unchanged with rationale).
- Rust/Python parity preserved; tests green.

Source: review of the graded cost-gate feature (spec 29ae5f6e). Relates to T3.
