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

---

## Review Findings (2026-07-27, Review Agent — verdict-only, no state transition applied)

**Independently re-verified all four acceptance criteria** — all pass:
- `--check` → exit 0, composite_sha256=6995d0164cba (matches claim)
- `--query MAI-Code-1-Flash` → github-copilot / in 0.75 / out 4.5 / cache_read 0.075 (matches claim)
- `test_cost_gate.py` → 36/36 OK (matches claim)
- Precedence: doc text in model-prices.instructions.md states the disjoint-namespace + composite-sha256 rule; code in sync_model_prices.py implements it exactly as documented (`build_document`, `composite_sha = sha256(genai_sha + github_sha)`, provider_id fixed to "github-copilot"). Zero duplicate (provider_id, model_id) pairs confirmed; 1285 + 29 = 1314 confirmed.
- Every model string in the model-routing.instructions.md tier ladder and roster sentence exact-matches a model_id in model_prices.json (verified by direct set comparison against github-copilot rows) — the two roster-string corrections claimed above are confirmed correct and complete; no other mismatches found.
- Spec 39983ddf traceability confirmed: spec has a dedicated "Price Table Provenance (2026-07-27)" section naming this ticket and both updated instruction files.
- Script remains stdlib-only (`argparse, datetime, hashlib, json, sys, urllib.error, urllib.request, re` — all stdlib).

**BLOCKING FINDING — reviewer verdict: send back to in-implementation.**
`.agents/instructions/orchestration/model-routing.instructions.md` line 50 still reads: "`MAI-Code-1-Flash` is offered but carries **no row in the price table**, so it cannot be cost-ranked. Do not route to it under a cost rationale." This is now **false** — this exact ticket gave MAI-Code-1-Flash a price row (verified above). The "Residual Risks" section above acknowledges the tier-ladder placement gap but does not flag that this line actively contradicts the change just shipped. Fix required: correct or remove this line before merge.

**Bundled with the above (reviewer: "small effort, fix in review reconciliation", no separate ticket):** place `MAI-Code-1-Flash` into the Tiered Model Ladder table (or explicitly document why it is deliberately excluded, e.g. reserved for a specific non-cost reason) rather than leaving it as a residual risk.

**Non-blocking, follow-up ticket opened:** [7a351e71](.ticket/tickets/7a351e71-52dd-484b-8a51-1e15744dcfb6/ticket.toml) — `tools/model-prices/__pycache__/cost_gate.cpython-314.pyc` is tracked in git and drifts on every test run (binary-only diff observed in this review, pre-existing, out of this ticket's stated scope).

**Verdict: fail** (4/4 ACs independently verified as passing, but a real defect — the stale/false MAI-Code-1-Flash claim — was found in the same file this ticket edited, and the reviewer ruled it blocking). Recommended target state: `in-implementation`. **No state transition was applied by this review** — see Iteration Agent / caller for the transition.


---

## Review round 2 — resolution (2026-07-27)

Independent review returned FAIL on one blocking finding: `.agents/instructions/orchestration/model-routing.instructions.md` still asserted that `MAI-Code-1-Flash` "carries no row in the price table", a claim this ticket's own change made false.

Resolved in-ticket (user decision, iteration interview):

- `MAI-Code-1-Flash` placed in the **T3** band of the tier ladder as a code-specialist peer of `GPT-5.4 mini` (identical pricing: 0.75 in / 0.075 cache-read / 4.5 out). The `ctx` figure is omitted because `context_window` is `null` in `model_prices.json` for this row — deliberately not invented.
- The false "no row in the price table" bullet was replaced with an accurate pointer to the T3 placement.
- Mechanical gate re-run after the edit: every model name in the tier ladder and the "Verified available" roster exact-matches a `model_id` in `tools/model-prices/model_prices.json`, except `Auto` (expected — an escape hatch, not a model).

Residual risks accepted, no follow-up tickets opened (user decision):

- **No upstream schema validation.** If GitHub reshapes `models-and-pricing.yml`, the mini parser fails opaquely rather than reporting a contract break. Accepted as a known risk.
- **No standing automated gate** enforcing that tier-ladder model strings exact-match priced `model_id`s. The check was run manually this round and in review. Accepted as a known risk.

Related: follow-up ticket `7a351e71` (tracked `__pycache__/*.pyc`) was **cancelled** as not worth tracking.