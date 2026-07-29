## Problem

`output_char_sizes` is empty for every tool in every session. Implement capture from whichever sources T1 (76941e78) proved carry the data, layered by fidelity.

## Design

Populate `ToolCallSummary::output_char_sizes` in [tool_metrics.rs](memory-api/crates/session-api/src/tool_metrics.rs) `record_event_tool_call`, which today reads `output_chars` / `response_chars` from the event payload and finds nothing. The work is to make those fields real at capture time.

Layered sources, highest fidelity first:

1. **Hook payload** — extend `args_from_hook_stdin` in [args.rs](memory-api/crates/session-api/src/bin/copilot-capture-hook/args.rs) to retain `tool_response`/`tool_output` and thread it into the captured `tool.execution_complete` event as `output_chars`. Conditional on T1 AC1/AC2.
2. **Spill-file stat** — when `spill_pointer` is set (or a `chat-session-resources/<session>/<tool_call_id>__*` directory exists), stat `content.txt` and record its size. Must be resilient to the file being absent later; a missing spill file yields "unmeasured", never zero.
3. **MCP proxy telemetry** — set `COST_GATE_TELEMETRY_LOG` and write the aggregator that folds the JSONL into the rollup. Supersedes 4aa13ba7.

## Required: source attribution

Every recorded output size carries an `output_source` discriminant (`hook_payload` | `spill_file` | `mcp_telemetry`). A call with no source is recorded as **unmeasured**, distinct from a call measured at zero. This distinction is what makes the coverage metric in the sibling ticket meaningful, and its absence is why the previous track could not tell a broken pipeline from a healthy one.

## Acceptance criteria

- AC1 — After one real captured session, `.session/sessions/<id>/tool-metrics.json` contains a tool with a non-empty `output_char_sizes`, **verified by reading the file**. Cite the session id and the observed value in the ticket's status summary.
- AC2 — Each recorded size carries an `output_source`; a call whose size is unknown is absent from `output_char_sizes` rather than recorded as `0`.
- AC3 — An e2e test drives the real `copilot-capture-hook` binary over a producer-shaped transcript fixture and asserts a non-zero output size, in the style of `e2e_hook_binary_populates_tool_metrics_from_captured_tool_events` in [copilot_capture_hook_e2e.rs](memory-api/crates/session-api/tests/copilot_capture_hook_e2e.rs).
- AC4 — Re-running aggregation over the existing ~200-session store does not regress the already-restored call/duration/input metrics.

## Non-goals

- Re-calibrating `tokens_at_max` — downstream, 8c4d1d9c.