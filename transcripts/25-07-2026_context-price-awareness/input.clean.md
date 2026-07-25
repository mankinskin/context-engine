# Model Price Awareness: Orchestrator Mode for Expensive Models

## Problem

Our context costs are too high. We rely heavily on expensive models because they produce very good outputs and make very good decisions. We want to keep that quality for the work where it matters most:

- Making strategic decisions.
- Developing new code, or planning code changes.
- Planning tool calls.

At the same time, we want to protect the context window of complex, large, and especially expensive models in order to keep costs low.

## Proposed Approach

Build a price-awareness instruction into our context system so that an agent switches into an "orchestrator" mode based on its own model's size/cost. In orchestrator mode, the agent delegates most of the work to sub-agents instead of doing it directly, including:

- Most tasks and tool calls.
- Reading or editing files.

## Requirements

1. Provide a mapping from the available models to their context/token cost.
2. Add an instruction to every agent's system prompt (i.e. in `AGENTS.md`): if a model is more expensive than a threshold `X`, it should act as an orchestrator and delegate rather than execute directly.

## Price Data Source

Model costs are sourced from the [pydantic/genai-prices](https://github.com/pydantic/genai-prices) repository and synchronized into a local price table:

- Sync tool: [tools/model-prices/sync_model_prices.py](../../tools/model-prices/sync_model_prices.py)
- Generated table (per-million-token USD prices): [tools/model-prices/model_prices.json](../../tools/model-prices/model_prices.json)

The tool downloads the upstream data, flattens it to a `provider_id`/`model_id`/price rows table, and only rewrites the output when the upstream content hash changes (`--check` for a stale check, `--force` to override).

### Querying the Price Table

The same tool queries the local table offline (no network), either for a specific model or as a compact listing:

- Look up a specific model (case-insensitive substring on provider or model id), aligned table:

  ```bash
  python tools/model-prices/sync_model_prices.py --query claude-opus-5
  ```

- Compact machine-readable listing for pipelines:

  ```bash
  python tools/model-prices/sync_model_prices.py --query gpt-5 --format csv
  python tools/model-prices/sync_model_prices.py --list --format json
  ```

Prices are USD per 1,000,000 tokens. Columns: `in$/M` (input), `out$/M` (output), `cread$/M` (cache read), `cwrite$/M` (cache write), `ctx` (context window). These per-model input/output rates are the values that feed the model→cost mapping and the orchestrator threshold `X`.

## Framing

This is part of "context stack price awareness," intended to help us reduce token costs while preserving output quality for high-value reasoning.

## Open Questions

- The cost threshold `X` and the concrete cost/token mapping values were not specified in the transcript.

## Resolved Decisions (2026-07-25)

- **Driving price field:** `output_mtok` (the `out$/M` column, USD per 1M output tokens). A blended or input-based metric is explicitly not used — input is heavily cache-discounted and blending requires assuming a token ratio, which makes the enforcement threshold fragile.
- **Threshold `X`:** `15` USD per 1M output tokens (equivalently `1500` credits/1M at 100 credits = $1). Rule fires when `output_mtok > X` (strictly greater).
- **Effect at `X = 15`:** Opus-tier (`out$/M` 25–75) and o3 (`40`) operate as orchestrators and delegate routine work; Sonnet (`15`), GPT-5 (`10`), Gemini Pro (`10`), Haiku, Flash, and mini execute directly.
- **Mapping location:** the shared table [tools/model-prices/model_prices.json](../../tools/model-prices/model_prices.json); the AGENTS.md "Model cost awareness & routing" bullet references it rather than duplicating values.
