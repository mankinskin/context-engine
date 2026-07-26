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