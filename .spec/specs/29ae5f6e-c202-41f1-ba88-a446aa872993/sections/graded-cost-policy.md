## Graded Cost Policy

The gate operates on a **graded numeric model** (1–100 scale) rather than binary heavy/light classification.

### Tool Cost Assignment

A tool's cost is derived from **empirical output-token estimates**:

1. **With sufficient data** (≥ 5 calls in window):
   - LINEAR map of p90 output size → 1–100, clamped
   - Tunable anchor points calibrate the scale
   - Tools with larger typical outputs map to higher costs

2. **Insufficient data** (< 5 calls):
   - **Fail open** to static `TOKEN_HEAVY_TOOL_SUBSTRINGS` list
   - Static-heavy → high cost
   - Static-light → low cost
   - Never silently assume "light"

### Gate Decision Flow

```
1. Resolve caller's base_budget from model's output_mtok
2. Resolve offset from grant_id (if provided)
3. Compute effective_budget = base_budget + offset
4. Resolve tool.cost (empirical or fallback)
5. Allow if tool.cost ≤ effective_budget, else return delegate
```

### Calibration

Default anchor constants are tuned so:
- Today's **threshold X=15 boundary** is reproduced
- Current `TOKEN_HEAVY_TOOL_SUBSTRINGS` behavior is preserved
- Orchestrator-tier models (output_mtok > 15) get low budgets by default
- Cheap models (output_mtok ≤ 15) get high budgets by default

### Rust + Python Parity

Both `memory-api/tools/mcp/mcp-cost-gate/src/gate.rs` (Rust) and `tools/model-prices/cost_gate.py` (Python) implement the graded model with matching logic.

See **T3** [4e7e53f5](../../../.ticket/tickets/4e7e53f5-b3de-477f-8cbd-f88b6c103bb5/ticket.toml) for implementation details.
