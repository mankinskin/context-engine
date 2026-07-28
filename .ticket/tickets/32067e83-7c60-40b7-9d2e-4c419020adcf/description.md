## Problem

`resolve_output_mtok` in memory-api/tools/mcp/mcp-cost-gate/src/gate.rs (~L156-174) lowercases the caller-supplied model, tries an exact `model_id` match, then falls back to a substring match against `provider_id`/`model_id`. `evaluate()` (~L235-245) rejects the entire call via `unknown_model_guidance` when that returns `None`.

Some clients pass the model with the agent client appended in parentheses, e.g. `"Claude Sonnet 5 (copilot)"`. The price-table id is hyphenated with no suffix, e.g. `claude-sonnet-5`. Neither string contains the other once spaces, casing, and the trailing `(copilot)` are involved — so a completely unambiguous model is rejected outright and the whole tool call fails.

## Decisions (interview-resolved)

- Normalization: strip a trailing parenthetical agent/client qualifier, AND normalize separators (spaces and underscores to hyphens).
- Apply normalization as a **fallback only**, after exact and substring matching both fail. Current exact-match precedence is preserved.
- A normalized (non-exact) match **succeeds with a soft warning** in the response.
- Normalization lives in the **MCP transport wrapper, before the request reaches `Gate`** — not inside `resolve_output_mtok`.
- Improve the rejection message: when a model still cannot be resolved, include a compact list of the detected/available models from the price table.
- Update `unknown_model_guidance` (~L312) to reflect the new tolerance.
- Update MCP tool docs and examples across servers for the relaxed `caller_model` format.

## Safety property

"Reject genuinely unknown models" must be preserved. Only harmless formatting deviations become tolerated.

## Tests

Existing unit tests are in the same file's `#[cfg(test)] mod tests` (~L340+): `exact_match_and_case_insensitive`, `ambiguous_substring_takes_max`, `exact_wins_over_substring`, `unknown_model_none`. None cover parenthetical suffixes or separator normalization.