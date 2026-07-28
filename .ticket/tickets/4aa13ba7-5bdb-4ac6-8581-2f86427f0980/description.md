## Problem

Ticket 9d527ad1 delivered per-tool-call token-load *measurement* in `mcp-cost-gate`, but nothing ever turned collection on. Every telemetry record produced in production since it landed has been silently discarded, and the cost gate has been running fail-open on data that does not exist.

Verified 2026-07-28:

1. **The emitter is never enabled.** `memory-api/tools/mcp/mcp-cost-gate/src/main.rs:201` reads `COST_GATE_TELEMETRY_LOG` and, when unset, drops every `CallTelemetry` record silently (documented as "matching the other `COST_GATE_*` optional-config" convention). Neither `.github/mcp.json` nor `opencode.json` sets it. Both set `COST_GATE_TABLE`, `COST_GATE_TOOL_METRICS`, and `COST_GATE_GRANTS_DIR` on every server entry — `COST_GATE_TELEMETRY_LOG` is simply absent.
2. **The rollup the gate reads does not exist.** `COST_GATE_TOOL_METRICS` points at `.session/tool-metrics-rollup.json`; `fs_stat` reports `exists: false`. The gate has therefore had no empirical tool costs to gate on.
3. **There is no aggregator.** Nothing converts the append-only JSONL the proxy would emit into the rollup JSON the gate consumes. Even with (1) fixed, the loop stays open.

## Why it matters

`79c4ac3e` (delegation-cost epic, effort 40000) depends on 9d527ad1 and is waiting on data that is not being written. `mcp-cost-gate`'s `MIN_CALLS` empirical-cost path is dead code in practice until the rollup is populated.

## Acceptance criteria

- **AC1** — `COST_GATE_TELEMETRY_LOG` is set for every server entry in `.github/mcp.json` and `opencode.json`, pointing at a stable in-repo path.
- **AC2** — after exercising real MCP traffic, the telemetry JSONL file exists and contains records with non-zero `tokens_estimated` and `duration_ms`. Verified by reading the file, not by inspecting the code.
- **AC3** — an aggregator produces `.session/tool-metrics-rollup.json` from the telemetry JSONL, in the exact schema `mcp-cost-gate` already expects when reading `COST_GATE_TOOL_METRICS`. Confirm the schema against the reader, do not invent one.
- **AC4** — `fs_stat` on `.session/tool-metrics-rollup.json` reports `exists: true` with non-zero size, and `mcp-cost-gate` loads it without error.
- **AC5** — the telemetry JSONL and rollup paths are gitignored or explicitly tracked by deliberate decision, not left ambiguous. `.session/sessions/**` is already ignored; decide and state which applies here.
- **AC6** — decide and document whether telemetry stays opt-in behind an env var or becomes on-by-default with an opt-out. The current opt-in default is what caused this gap.

## Non-goals

- Do not re-litigate the mcp-cost-gate telemetry design or re-open the abandoned transcript-usage-extraction approach (three research passes established the raw Copilot transcript JSONL carries no `usage` and no `model` field).
- Do not touch `memory-api/crates/session-api/src/store/config/persistence.rs`; `cost_usd` stays null.
- The `session_tool_metrics` reader crash is tracked separately.

## Verification note

Per 7de9f4f0, do not accept a successful write call as proof. Read the artifact back — this ticket exists precisely because a verified-green implementation was never actually collecting anything.