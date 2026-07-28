## Problem

Two instruction files still describe cost-gate behavior that does not match the code, and the drift predates ticket `32067e83`:

- [.agents/instructions/orchestration/model-prices.instructions.md](.agents/instructions/orchestration/model-prices.instructions.md) L28 claims an unrecognized `caller_model` "resolves to a zero-cost budget, which silently breaks price-awareness enforcement".
- [.agents/instructions/orchestration/orchestrator-delegation.instructions.md](.agents/instructions/orchestration/orchestrator-delegation.instructions.md) L26-31 repeats the same "zero-cost budget" framing.

Neither is true. `Gate::evaluate` in `memory-api/tools/mcp/mcp-cost-gate/src/gate.rs` rejects an unresolvable model outright rather than falling back to zero cost. Neither doc mentions the normalization tolerance added by `32067e83` (trailing parenthetical client qualifier stripped; spaces and underscores folded to hyphens; fallback-only, after exact and substring matching fail; soft warning on a normalized match).

## Scope

Correct both instruction files to describe actual gate behavior: reject-on-unresolvable, exact-before-substring precedence, and the fallback normalization tolerance with its soft-warning semantics.

## Notes

Split out of ticket `32067e83` during review. Criterion 10 of that ticket was waived as satisfied by the `tools/list` schema description in `memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs`; this ticket carries the remaining instruction-doc drift. Normative behavior is pinned by spec `9f0b9e30-e32c-4092-b2a2-68179141cfc4`.