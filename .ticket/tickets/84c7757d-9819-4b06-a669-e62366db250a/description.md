## Problem

The session store's failure signal is misleading. In session 51701334, only **10 of 554** tool executions (1.8%) recorded `tool_success: false`, yet the session was dominated by friction — 5-minute timeouts, a hung `rule scan` killed twice, a wrong-work sub-agent redo, and an unauthorized commit. Every one of those recorded as `success: true`.

Three concrete deficiencies found:
1. `.session/sessions/<id>/tool-metrics.json` is **empty** (`"tools": {}`) despite 554 tool calls — the aggregation file exists but is never populated.
2. `tool.execution_complete` records only `tool_success` (bool), `result_code`, `has_spill`, `duration_ms` — **never the result body or error text**. You can see *that* a call failed, never *why*.
3. Timeouts and hangs are invisible: eight executions pinned at exactly ~300,000 ms (the 5-min timeout cap) all recorded `success: true` (events.json L25664, L37503, L37995, L38711, L42579, L77293, L77820, L77999). Two executions ran 31 min and 20 min (L44080, L80467), also `success: true`.

## Goal
- Populate tool-metrics.json with a real per-tool rollup (count, success/fail, p50/p95 duration, timeout count).
- Capture tool error text / non-zero exit / rejection reason on `execution_complete`.
- Classify timeout-cap-hit and hang/kill outcomes as a distinct non-success `result_code` so they surface in metrics.

## Acceptance criteria
- tool-metrics.json is non-empty after a real session and reflects failures + slow tools.
- A reviewer can read the failure reason for a failed tool from the store.
- Timeout/hang executions are countable and distinguishable from clean successes.

## Owning code
memory-api/crates/session-api, memory-api/tools (session hook / session-mcp / session-cli).


---

## Reopened 2026-07-29 — acceptance criterion was never met

This ticket's AC — *"tool-metrics.json is non-empty after a real session and reflects failures + slow tools"* — was the **only correctly-shaped end-to-end acceptance criterion in the entire tool-metrics track**. It was nonetheless closed `done` without ever being checked against a real session.

Post-mortem evidence (review + roast + session forensics, 2026-07-29):

- The completion was verified by **code-existence tracing**: *"all 3 acceptance criteria traced to tool_metrics.rs, hook.rs, hook/tool_execution.rs"* (session `aaf84892`). No one read the produced file.
- At the moment of closure and for months after, `.session/sessions/<id>/tool-metrics.json` was `{"tools":{}}` for **every** session — 114 of 195 sidecar files were empty.
- Root cause: `compute_session_summary` read only transcript turns with `role == SessionRole::Tool`; the Copilot producer never emits that role. All tool telemetry lives in captured events. Zero tests ran against a real or producer-shaped transcript.

## What has now landed

memory-api commit `7df14ea` — "feat(session-api): enhance tool metrics computation with event data and lazy persistence":

- `compute_session_summary_with_events` derives calls from `tool.execution_complete` / `tool.execution_result` events, de-duplicated by `tool_call_id`.
- `tool-metrics.json` is now created **lazily** — never written when no tool call was observed, and a stale empty sidecar is removed.
- Aggregation skips unreadable session directories instead of hard-failing the whole store (this also resolves 574560bf).
- New e2e test `e2e_hook_binary_populates_tool_metrics_from_captured_tool_events` drives the real `copilot-capture-hook` binary over a producer-shaped transcript.

Observed after re-aggregation over the real store: **186 sidecars, 0 empty**, 203 sessions in the rollup, e.g. `apply_patch` 633 calls / 623 ok / 10 failed / p50 27 ms / p95 3081 ms.

## Remaining gap on this ticket's own AC

The AC says "reflects failures + slow tools" — failures and durations are now real. **Output sizes are still unmeasured**: the raw `tool.execution_complete` payload carries only `{success, toolCallId}`. That work is tracked under epic 341f4bf2 and ticket 44119807.

## Re-closure criteria

- Cite the session id and the concrete observed values read from a real `tool-metrics.json` — not a code path, not a test count.
- Link the recorded validation execution from `val-session-api-tool-metrics-e2e` (ticket ce7b7bde).