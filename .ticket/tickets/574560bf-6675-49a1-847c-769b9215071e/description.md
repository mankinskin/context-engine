## Problem

`session_tool_metrics` hard-fails for the entire workspace store when any single session directory is missing `session.json`.

Reproduced 2026-07-28 with `session_tool_metrics(workspace="c:/Users/linus/git/graph_app/context-engine", days=7)`:

```
MPC -32602: session data was not found at
c:/Users/linus/git/graph_app/context-engine\.session\sessions\03baab6c-0fdb-4ffc-8159-b83066a6283f\session.json
```

That directory is not empty or corrupt in any interesting way — `fs_list_dir` shows `context.json` (38KB) and a `handoffs/` directory containing 39 handoff records. It simply has no `session.json`. The aggregator treats that as a fatal error for the whole scan instead of skipping the directory, so **no tool metrics can be read for any session in the store**.

## Impact

The read side of tool-cost observability is unavailable. This compounds the collection gap tracked in the companion ticket: even once telemetry is being written, this reader cannot surface it. Any cost-aware routing or delegation-cost analysis that depends on `session_tool_metrics` is blocked.

## Acceptance criteria

- **AC1** — a session directory missing `session.json` is skipped, not fatal. `session_tool_metrics` returns results for the remaining sessions.
- **AC2** — skipped sessions are surfaced, not swallowed: the response reports which session ids were skipped and why. Silent skipping would trade one failure mode for a worse one.
- **AC3** — `session_tool_metrics(workspace=<repo root>, days=7)` returns a successful result against the real store, which currently contains at least one such directory (`03baab6c-0fdb-4ffc-8159-b83066a6283f`). Verified by running it, not by reading the code.
- **AC4** — a regression test covers a store fixture containing a session directory with `context.json` and `handoffs/` but no `session.json`.
- **AC5** — establish whether such a directory is legitimate or itself a bug. If sessions are supposed to always write `session.json`, note the root cause; do not paper over a write-path defect with a read-path skip alone.

## Non-goals

- Do not restructure the session store layout or migrate existing session directories.
- Do not touch `memory-api/crates/session-api/src/store/config/persistence.rs`.

## Verification note

Per 7de9f4f0, verify by executing the tool against the real store and reading the result — not by asserting the fix compiles.