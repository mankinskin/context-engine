## Summary

The cost gate assigns each tool a single per-tool cost that does not vary with the call. Under the single-default model (ticket 9185d8f2), every not-yet-measured tool shares one default cost. This ignores that a single tool's real token cost depends heavily on its call ARGUMENTS. Peek tools are designed for token-efficient bounded reads: a peek_read with a small window/line-count costs far less than one with a large window. Treating every peek call as the same default over-restricts large/expensive models from cheap, high-value bounded reads.

## Context

- Cost gate implementation: `tools/model-prices/cost_gate.py` and `memory-api/tools/mcp/mcp-cost-gate/src/gate.rs`
- Cost scale is 1-100. Historically, name-matched token-heavy tools got a STATIC fallback cost of 75 via `heavy_fallback_cost()`; ticket 9185d8f2 removes that name-based categorization in favor of a single default cost for all not-yet-measured tools.
- Under either model the per-tool cost is fixed regardless of actual call size; this ticket adds argument-based variation on top of that base cost.
- Canonical spec: 29ae5f6e-c202-41f1-ba88-a446aa872993 "Empirical tool-metrics driven cost-gate classification"
- Parent ticket: 445a2d76-5795-4d7a-aec8-d1536ec61416 "Model price awareness: enforce orchestrator mode for expensive models"
- Related ticket: 8c4d1d9c-1004-4539-9880-0a0e8aa03dd3 "Re-tune graded-cost calibration from real rollup data"

## Acceptance Criteria

1. The cost gate computes the effective cost of a specific tool CALL as a function of a per-tool BASE cost plus an ARGUMENT-DERIVED dynamic estimate: `effective_cost = clamp(base + sum(arg_estimator_i(arg_value_i)), 1, 100)`.

2. Per-tool cost models are declarative: a tool maps to `{ base_cost, arg_estimators: [ { param_path, scaling (e.g. linear/log per unit), unit_weight, cap } ] }`. Missing/absent args fall back to base cost.

3. Peek tools get concrete estimators for their windowing params (e.g. read window size / line count / byte limit, grep max-results / context radius, skeleton depth, count scope). Small windows land in the light/medium tier; only large windows approach heavy.

4. Argument-based estimation integrates with the existing empirical calibration path: once >=5 empirical calls exist, arg values feed the size->cost regression (future work hook), but the declarative estimators provide the pre-calibration fallback (refining the single default cost for arg-bounded tools instead of applying the flat default).

5. Both the Python gate (`cost_gate.py`) and Rust proxy (`mcp-cost-gate/src/gate.rs`) must share the same per-tool cost-model definitions (single source of truth or mirrored, matching the existing Python/Rust parity pattern).

6. Manual grant overrides (`.session/grants/`) continue to work and stack on top of arg-based cost.

## References

- Spec: 29ae5f6e-c202-41f1-ba88-a446aa872993
- Parent: 445a2d76-5795-4d7a-aec8-d1536ec61416
- Related: 8c4d1d9c-1004-4539-9880-0a0e8aa03dd3