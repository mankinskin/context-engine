## Problem

`tools/model-prices/sync_model_prices.py` sources prices only from pydantic/genai-prices `prices/data_slim.json`, which is a vendor catalogue. Models offered by the Copilot `runSubagent` surface can be absent from it — `MAI-Code-1-Flash` is offered but has no row, so it cannot be cost-ranked and the cost gate silently resolves it to a zero-cost budget.

## Objective

Add a second upstream source: `https://github.com/github/docs/blob/main/data/tables/copilot/models-and-pricing.yml` (use the raw URL), which lists prices for GitHub Copilot-provided models. Merge it into `model_prices.json` so every model the surface offers has a priced row.

## Scope

- `tools/model-prices/sync_model_prices.py` — add the second fetch, parse the YAML (script is stdlib-only today; keep it that way or justify a dependency), define merge/precedence rules when a model appears in both sources, and extend `source_sha256` to cover both inputs.
- `tools/model-prices/model_prices.json` — regenerate (generated artifact, commit with the change).
- `.agents/instructions/orchestration/model-prices.instructions.md` — document the second source and the precedence rule.
- `.agents/prompts/sync-model-prices.prompt.md` — update the workflow if step semantics change.

## Acceptance criteria

- `python sync_model_prices.py --check` still round-trips correctly with both sources.
- `MAI-Code-1-Flash` resolves to a real price via `--query`.
- Precedence between the two sources is documented and deterministic.
- `python test_cost_gate.py` passes.

## Context

Raised during the 2026-07-27 model-routing iteration, where the verified `runSubagent` roster was reconciled against the price table. See `.agents/instructions/orchestration/model-routing.instructions.md` section "Roster is not the catalogue".

---

## Validation Evidence (2026-07-27)

All four acceptance criteria met:

**AC1: `--check` round-trips with both sources**
- `python sync_model_prices.py --check` → exit 0, "up to date (model_prices.json, composite_sha256=6995d0164cba)"
- Second sync writes nothing, zero git churn (idempotency verified)

**AC2: MAI-Code-1-Flash resolves to real price**
- `python sync_model_prices.py --query MAI-Code-1-Flash` → github-copilot provider, in: 0.75, out: 4.5, cache_read: 0.075 (was previously absent, now present)

**AC3: Precedence documented and deterministic**
- Disjoint provider namespaces: genai-prices rows keep upstream provider_id, github-copilot rows use "github-copilot" provider_id
- Zero duplicate (provider_id, model_id) pairs in output
- Intra-file duplicates resolve to Default tier → no-threshold row → first occurrence
- _meta.source_sha256 = composite digest (genai-prices + github-copilot, fixed order)
- New _meta.sources array carries per-source {name, url, sha256, model_count}
- `.agents/instructions/orchestration/model-prices.instructions.md` gained "Source Precedence" subsection

**AC4: test_cost_gate.py passes**
- `python test_cost_gate.py` → "Ran 36 tests ... OK" (36/36)

**Additional verification**
- model_count: 1314 = 1285 genai-prices + 29 github-copilot
- Parser spot-check: Claude Haiku 4.5, GPT-5.4 (Default tier), GPT-5.4 mini all match upstream
- No leaked YAML syntax in any model_id
- Tier bands consistent: T3 max 1.140 < T2 min 2.200 < T1 max 2.750 < T0 5.500

## Residual Risks

**Risk 1: No upstream schema validation**
If GitHub's YAML changes shape (new keys, nested structure, tier format), the ~54-line mini parser may error opaquely or miss data. No automated upstream schema contract exists.

**Risk 2: MAI-Code-1-Flash still missing from tier ladder**
MAI-Code-1-Flash now has a price but has no routing guidance in `.agents/instructions/orchestration/model-routing.instructions.md` tier ladder — a user trap for anyone trying to cost-rank it.

## Follow-on fix from audit

`.agents/instructions/orchestration/model-routing.instructions.md` had two tier-ladder model strings that did not exact-match the new github-copilot model_ids:
- "Claude Opus 4.8 (fast mode) (Preview)" → "(preview)" 
- "Gemini 3.1 Pro (Preview)" → "Gemini 3.1 Pro"

Corrected to match exact github-copilot model_id strings.