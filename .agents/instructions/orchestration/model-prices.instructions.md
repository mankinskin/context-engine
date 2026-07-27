---
description: "Use when resolving model prices, syncing the model cost table, or wiring cost-aware routing to real numbers. Covers tools/model-prices layout, the sync script, offline queries, the cost gate, and staleness policy."
---

## The Model Cost Table

`tools/model-prices/` is the single source of truth for model pricing in this repository. Every routing, delegation, and cost-gate decision resolves against it. **Never hardcode a price** into instructions, agent templates, or tooling — read it from the table.

| Path | Role |
|---|---|
| `tools/model-prices/model_prices.json` | The generated price table. Committed artifact; do not hand-edit. |
| `tools/model-prices/sync_model_prices.py` | Fetches upstream prices and regenerates the table. Also serves offline queries. |
| `tools/model-prices/cost_gate.py` | Resolves a model to an allow/delegate decision and a budget scale. |
| `tools/model-prices/test_cost_gate.py` | Tests for the gate. Run after changing gate logic. |

Upstream sources are [pydantic/genai-prices](https://github.com/pydantic/genai-prices) (`prices/data_slim.json`, MIT) and GitHub Copilot's published pricing table (`https://raw.githubusercontent.com/github/docs/main/data/tables/copilot/models-and-pricing.yml` from the github/docs repo). The script is stdlib-only — no PyYAML dependency, no virtualenv needed.

### Source Precedence

The two upstreams occupy **disjoint provider namespaces**, so neither can overwrite the other:

- All genai-prices rows retain their original `provider_id` (e.g. `anthropic`, `openai`, `google`).
- All GitHub Copilot rows use `provider_id: "github-copilot"` and `provider_name: "GitHub Copilot"`, with `model_id` set to the model's display name **verbatim** as it appears in the `runSubagent` model surface (e.g. `MAI-Code-1-Flash`, `Claude Opus 5`).

Consequence: the same underlying model can legitimately appear **twice** in the table — once under its vendor's genai-prices slug and once under `github-copilot`. For routing and cost-gate decisions about a Copilot dispatch, use the `github-copilot` row.

Within the GitHub Copilot source, models may appear multiple times with different pricing tiers/thresholds. The sync keeps only the `Default` tier row (or the row with no threshold specified), ensuring exactly one row per model_id.

### Table shape

```jsonc
{
  "_meta": {
    "source": "pydantic/genai-prices",
    "source_url": "...data_slim.json",
    "source_sha256": "...",       // composite digest over both upstreams; drives change detection
    "synced_at": "...",           // UTC ISO timestamp of last successful sync
    "model_count": 1285,
    "price_unit": "USD per 1,000,000 tokens",
    "sources": [                  // per-source detail; source/source_url retained for compatibility
      {"name": "pydantic/genai-prices", "url": "...", "sha256": "...", "model_count": 1200},
      {"name": "github-copilot", "url": "...", "sha256": "...", "model_count": 85}
    ]
  },
  "models": [
    {
      "provider_id": "anthropic",
      "provider_name": "Anthropic",
      "model_id": "claude-sonnet-5",
      "input_mtok": 2,
      "output_mtok": 10,
      "cache_read_mtok": 0.2,
      "cache_write_mtok": 2.5,
      "context_window": 1000000,
      "deprecated": false
    }
  ]
}
```

- All prices are **USD per 1M tokens**. A `null` (rendered `-` in table/CSV output) means upstream did not publish that field, not that it is free.
- Rows are sorted by `(provider_id, model_id)`, so a regenerated table diffs cleanly.
- The same model often appears under several `provider_id`s (`anthropic`, `aws`, `google`, …) at different prices. Match the provider you actually bill through; when unsure, use the model vendor's own row.
- Prices are **indicative estimates**, not authoritative billing data. Use them for relative routing decisions, not for invoicing.

## Reading Prices (Offline)

`--query` and `--list` never hit the network — they read the committed table. Use them freely.

```bash
cd tools/model-prices

# One model or family, human-readable
python sync_model_prices.py --query claude-sonnet-5

# Compare a family across providers
python sync_model_prices.py --query gpt-5.6 --format csv

# Whole table, machine-readable
python sync_model_prices.py --list --format json
```

Practical patterns:

```bash
# Cheapest models under $6/M output, sorted
python sync_model_prices.py --list --format csv \
  | awk -F, '$4 != "-" && $4+0 <= 6 && $4+0 > 0' | sort -t, -k4 -n

# Rank cheap-tier candidates by INPUT price (the metric that matters for bulk units)
python sync_model_prices.py --list --format csv \
  | grep -E '^(anthropic|openai|google),' | sort -t, -k3 -n | head -20
```

CSV columns are `provider,model,in$/M,out$/M,cread$/M,cwrite$/M,ctx` — note that `--format csv` uses these short headers while the JSON uses the full `*_mtok` field names.

## Syncing (Network)

```bash
cd tools/model-prices

# Is the local table stale? Exit 1 = out of date. Writes nothing.
python sync_model_prices.py --check

# Sync. No-ops when the upstream hash is unchanged.
python sync_model_prices.py

# Force a rewrite even when unchanged (refreshes synced_at)
python sync_model_prices.py --force

# Richer upstream dataset, or a pinned/mirrored source
python sync_model_prices.py --full
python sync_model_prices.py --source-url <url> --timeout 60
```

Sync rules:

- Run `--check` before trusting the table for a cost-sensitive decision, and sync when it reports stale.
- The script rewrites `model_prices.json` **only** when `source_sha256` changes, so a routine sync is a no-op and produces no diff.
- Treat `model_prices.json` as generated: commit the regenerated file, never hand-edit it. Fix pricing problems upstream or in the script.
- Sync is a network operation. If it fails (offline, upstream down), fall back to the committed table, note that prices may be stale, and continue — do not block work on a failed sync.
- A sync can **change tier assignments**. After a sync that produces a diff, re-check the canonical ladder in [model-routing.instructions.md](model-routing.instructions.md) against the new numbers. [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) references that ladder rather than duplicating it — keep it that way.
- The table is a **vendor catalogue**, not the roster of models the current surface offers. Many priced models will be refused by `runSubagent`. Confirm availability before routing to a model you found here; see "Roster is not the catalogue" in [model-routing.instructions.md](model-routing.instructions.md).

## The Cost Gate

`cost_gate.py` turns a model id into a routing decision.

```bash
python cost_gate.py --model claude-opus-5 --tool runSubagent
python cost_gate.py --model claude-sonnet-5 --tool read_file --format json
```

- Driving field is `output_mtok`. Exit `0` = `allow` (execute directly), exit `3` = `delegate` (operate as orchestrator).
- `--model` must be a real `model_id` key from the table (e.g. `claude-sonnet-5`, `gpt-5.3-codex`), not a vendor or product label like `copilot` or `anthropic`. An unrecognized id resolves to a **zero-cost budget**, which silently disables price awareness.
- The same rule applies to the `caller_model` field that `mcp-cost-gate` injects into MCP tool schemas: pass the id of the model actually issuing the call. A delegated sub-agent passes **its own** id, not the orchestrator's.
- The gate **fails open** when the price table is missing or unreadable. A silently permissive gate looks identical to a correctly permissive one — verify the table exists before concluding that routing is unrestricted.
- After changing gate logic, run `python test_cost_gate.py` (or the repo test runner) before relying on it.

## When Prices Move

Model pricing changes often, and a stale table quietly degrades every routing decision. When a sync produces a real diff:

1. Re-resolve the models named in the tier tables; confirm each still sits in its assigned band.
2. Confirm each still exists on the current surface — a price change is a good moment to re-verify the roster, since vendors retire models and the catalogue keeps pricing them.
3. Check whether a newer generation now dominates the current default on **every** axis (input, output, cache read, context window). If so, promote it and mark the old default as superseded rather than deleting it — existing sessions may still reference it.
4. Check whether the `X = 15` gate threshold still splits the catalogue sensibly.
5. Update the tier tables and record the price basis, so a later reader can tell whether a routing rule was reasoned or inherited.
