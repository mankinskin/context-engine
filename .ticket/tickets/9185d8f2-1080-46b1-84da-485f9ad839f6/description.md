## Problem

The cost gate hardcodes a name-based list (TOKEN_HEAVY_TOOL_SUBSTRINGS) that assigns a flat 75 to specific tools by name. This bakes in assumptions about tools we cannot actually know from their names, and unfairly penalizes token-efficient tools like peek. We do not want to hardcode any specific tools or infer cost from names.

## Acceptance Criteria

1. Remove the TOKEN_HEAVY_TOOL_SUBSTRINGS name-based categorization ENTIRELY from both cost_gate.py and gate.rs (and remove heavy_fallback_cost's role as a per-name classifier). No tool is special-cased by name.

2. Every tool WITHOUT sufficient empirical data gets ONE single default cost (an "unknown tool" default). No per-tool or name-substring assumptions.

3. Set the single default so it gates expensive/orchestrator-tier models (default cost above their budget) BUT remains BELOW the budget of cheaper/smaller agents, so cheaper agents CAN still call unknown tools. This avoids a chicken-and-egg deadlock: unknown tools must be callable by someone to ever produce metrics.

4. Cheaper-agent calls to unknown tools record tool-metrics into .session/tool-metrics-rollup.json; once >=N (existing threshold, currently 5) empirical calls exist, the empirical p90-output->cost mapping takes over and drives the real cost/rating. Peek being blocked for big models at first is acceptable and expected until data is gathered.

5. Behavior must be mirrored/consistent across the Python gate and the Rust proxy.

6. Preserve grant overrides (.session/grants/) and keep compatibility with the argument-based dynamic estimation work (ticket 9c9e2edc): the single default is the base for unknown tools; arg estimators still refine per-call cost where declared.

## Implementation Notes

Code paths:
- Python: tools/model-prices/cost_gate.py ~lines 40-65
- Rust: memory-api/tools/mcp/mcp-cost-gate/src/gate.rs ~lines 27-51

## Related

- Parent ticket: 445a2d76-5795-4d7a-aec8-d1536ec61416
- Spec: 29ae5f6e-c202-41f1-ba88-a446aa872993
- Related: 8c4d1d9c-1004-4539-9880-0a0e8aa03dd3 (re-tune calibration from rollup)
- Related: 9c9e2edc-81fc-489e-9153-bf4ac0bf1a13 (dynamic argument-based cost estimation)