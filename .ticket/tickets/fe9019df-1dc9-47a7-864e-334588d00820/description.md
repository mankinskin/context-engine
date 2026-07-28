## Problem

Two mis-attributed citations occurred in the same implementation track, both from agents asked to review rather than implement:

1. A fabricated reference to `spec-handoff-package-schema.md`.
2. The three `mcp-cost-gate` normalization tests cited as living in `memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs` at lines 589/620 — correct line numbers, wrong file. They actually live in `memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs` at L589, L620, and L648. `tests/integration_gate.rs` contains no normalization tests at all.

Both would have passed unchallenged had they not been cross-checked by hand. The pattern is not model noise: it is a systematic gap in the review agent templates, which cite locations without re-opening the path they cite.

## Scope

Add a mandatory citation re-verification step to the review-oriented agent templates. Before emitting any `path#Lnn` reference, the agent must open the file and confirm the symbol is present at that path and line. Unconfirmable citations must be reported as unverified, not asserted.

## Notes

Discovered while reviewing ticket `32067e83-7c60-40b7-9d2e-4c419020adcf`. None of epic `2558a279`'s existing eight children covers review-citation verification.