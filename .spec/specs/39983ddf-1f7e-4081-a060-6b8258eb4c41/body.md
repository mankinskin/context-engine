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
- **Mapping location:** [tools/model-prices/model_prices.json](../../tools/model-prices/model_prices.json), synced/queried by [tools/model-prices/sync_model_prices.py](../../tools/model-prices/sync_model_prices.py).

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

## Open Dependencies

- **BLOCK-3 (T-WRAP):** model-identity transport into MCP calls (explicit parameter vs. session/context header).
- **BLOCK-4 (T-ORCH):** sub-agent invocation primitive in the agent runtime (`runSubagent` vs. custom).