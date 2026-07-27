## Overview

"Context stack price awareness" makes an agent switch into **orchestrator mode** based on the cost of its own underlying model, so expensive models are reserved for strategic decisions, code/change planning, and tool-call planning, while routine execution is delegated to cheaper sub-agents. Enforcement is layered across three tickets.

Design source: [transcripts/25-07-2026_context-price-awareness/input.clean.md](../../transcripts/25-07-2026_context-price-awareness/input.clean.md).

## Requirements

1. A machine-readable model→cost mapping is the single source of truth for cost, never hardcoded per tool or per agent.
2. The orchestrator decision is keyed to a single named price field and a concrete threshold `X`.
3. Enforcement is layered: prose rule (AGENTS.md) → tool-boundary wrapper → structurally-constrained orchestrator agent.

## Resolved Decisions

- **Driving field:** `output_mtok` (the `out$/M` column of the mapping, USD per 1M output tokens). Not a blended or input-based metric.
- **Threshold `X`:** `15` USD per 1M output tokens (equivalently `1500` credits/1M at 100 credits = $1). The rule fires when `output_mtok > X` (strictly greater).
- **Effect at `X = 15`:** Opus-tier (`out$/M` 25–75) and o3 (`40`) orchestrate; Sonnet (`15`), GPT-5 (`10`), Gemini Pro (`10`), Haiku, Flash, and mini execute directly.
- **Mapping location:** [tools/model-prices/model_prices.json](../../tools/model-prices/model_prices.json), generated from two upstreams by [tools/model-prices/sync_model_prices.py](../../tools/model-prices/sync_model_prices.py): (1) pydantic/genai-prices `prices/data_slim.json` (primary, 1285 rows) and (2) GitHub Copilot's per-token pricing table (29 rows). Sources occupy disjoint provider namespaces; the Copilot row is authoritative for Copilot dispatch cost-gate decisions.

## Enforcement Layers (tickets)

- **T-PARENT** `445a2d76-5795-4d7a-aec8-d1536ec61416` — AGENTS.md prose rule + model→cost mapping reference. **Done (in-review).** The "Model cost awareness & routing" bullet in [AGENTS.md](../../AGENTS.md) now carries the enforceable `output_mtok > 15` rule and the mapping source-of-truth.
- **T-WRAP** `a5ad2721-2d07-47dd-85f5-f180d4a030fa` — model-aware MCP tool wrapper that requires the calling model identity, resolves cost from the mapping, and returns a delegation refusal for expensive-model calls to token-heavy tools. Depends on T-PARENT. **Blocked on BLOCK-3** (how model identity is transported into each MCP call).
- **T-ORCH** `8418fa92-bf46-42d9-a93f-9240032893b7` — dedicated orchestrator agent exposing exactly one sub-agent tool, doing only planning + delegation + aggregation. Depends on T-PARENT and T-WRAP. **Blocked on BLOCK-4** (exact sub-agent invocation primitive).

## Acceptance Criteria

- [x] Model→cost mapping derivable from `model_prices.json`, referenced (not duplicated) by guidance.
- [x] AGENTS.md contains an explicit orchestrator-mode rule keyed to `output_mtok > X` with `X = 15`.
- [x] Threshold field + value recorded in T-PARENT and the transcript.
- [ ] All MCP tools reachable only through the model-aware wrapper; expensive-model calls to token-heavy tools refused with delegation guidance; cheap-model calls pass unchanged (T-WRAP).
- [ ] A dedicated orchestrator agent exists with exactly one tool (spawn sub-agent) (T-ORCH).

## Price Table Provenance (2026-07-27)

**Deterministic cross-source precedence by construction:** the two sources (genai-prices and GitHub Copilot) occupy disjoint provider namespaces, so neither can overwrite the other. Copilot rows use `provider_id` "github-copilot" / `provider_name` "GitHub Copilot" with `model_id` set to the model's display name verbatim — deliberately the same string the `runSubagent` model surface uses, so an orchestrator can resolve the model it is about to dispatch to without a name-mapping table.

**Consequence:** the same underlying model may appear twice (vendor slug + github-copilot). For routing and cost-gate decisions about a Copilot dispatch, the github-copilot row is authoritative.

**Tier resolution within the Copilot source:** models listed multiple times under different pricing tiers/thresholds resolve to the `Default` tier row (else no-threshold, else first); exactly one row per model_id.

**Change detection:** `_meta.source_sha256` remains the single key driving `--check`, now a composite digest over both upstream hashes in fixed order. New `_meta.sources` array carries per-source `{name, url, sha256, model_count}`. `_meta.source` / `_meta.source_url` retained for backward compatibility (primary source).

**Fail-loud:** if the Copilot upstream cannot be fetched the sync exits 2, rather than writing a genai-prices-only table that would falsely read as up to date.

**Row schema unchanged**, so the `cost_gate.py` consumer contract (`provider_id`, `model_id`, `output_mtok`) is unaffected.

**Practical outcome this unblocks:** models offered by the dispatch surface but absent from genai-prices (notably MAI-Code-1-Flash, in 0.75 / out 4.5) now have priced rows and can be cost-ranked.

**Validation:** `python sync_model_prices.py --check` → exit 0, composite_sha256=6995d0164cba; `python test_cost_gate.py` → 36/36 OK; idempotent re-sync produces no churn; zero duplicate (provider_id, model_id) pairs; rows sorted by (provider_id, model_id).

**Implemented by:** [ticket b0d6bb1c](../../.ticket/tickets/b0d6bb1c-1a74-478a-aac1-1943b5454e96/ticket.toml).  
**Updated guidance:** [.agents/instructions/orchestration/model-prices.instructions.md](../../.agents/instructions/orchestration/model-prices.instructions.md), [.agents/instructions/orchestration/model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md).  
**Prompt:** [.agents/prompts/sync-model-prices.prompt.md](../../.agents/prompts/sync-model-prices.prompt.md).

## Open Dependencies

- **BLOCK-3 (T-WRAP):** model-identity transport into MCP calls (explicit parameter vs. session/context header).
- **BLOCK-4 (T-ORCH):** sub-agent invocation primitive in the agent runtime (`runSubagent` vs. custom).