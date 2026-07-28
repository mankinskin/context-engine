## Implementation summary

Promoted `tmp/subagent_cost_probe.py` into a supported, tested analyzer in `session-api` (deleted the probe script).

### Changes (all within `memory-api/crates/session-api/`)

- `src/model.rs`: added `SessionTurnEventMeta::subagent_run_id` — the `tool_call_id` of the nearest enclosing `runSubagent` span, resolved via true `parent_event_id` ancestry (not event-index overlap).
- `src/hook.rs` + `src/hook/transcript.rs` + `src/hook/parser.rs`: capture-time span attribution. A single-pass ancestor map (`event_id -> owning span`) is built as events are consumed in order, so nested and parallel/overlapping `runSubagent` spans are attributed without double-counting, and stamped onto every turn's `event_meta.subagent_run_id`.
- `src/delegation_cost.rs` (new): `compute_delegation_cost_report` — per-sub-agent tool histograms, within-agent repeat reads/commands, cross-agent duplicate reads (path-normalization safe via `normalize_path_for_dedup`), cross-agent duplicate commands (>2x total), failure classification, and real per-sub-agent token/cost totals from `event_meta`. 5 unit tests covering path normalization, parallel-span no-double-count, failure attribution, and token/cost flow-through.
- `src/store/config/capture_query.rs`: `SessionStoreConfig::delegation_cost_report(selector)` — the supported command surface (analogous to the existing `session_audit`).
- `src/subagent_rollup.rs`: `compute_subagent_rollups` now groups turns by `subagent_run_id` when present, giving real per-span attribution instead of lumping every turn into the parent session's bucket (falls back to prior behavior when `subagent_run_id` is absent, preserving existing tests).
- `src/store.rs`: `SessionStorePlan::persist` now computes and writes `tool-metrics.json` immediately at capture time (previously only computed lazily on first aggregate read).
- Spec [7be68a48 Quality gates and session data collection for delegated sessions](.spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/spec.toml): added a "Delegation Cost Analyzer (b7c61f0e)" section documenting the new attribution field and report contract.

### Acceptance criteria

1. ✅ Met — `SessionStoreConfig::delegation_cost_report` reproduces the epic's analysis for any captured session (verified via smoke capture + report).
2. ✅ Met — parallel/overlapping spans attributed via `parent_event_id` ancestry; regression test `parallel_spans_are_attributed_without_double_counting`.
3. ✅ Met — `normalize_path_for_dedup` unifies backslash/forward-slash and drive-letter case; regression test `duplicate_reads_are_path_normalization_safe`.
4. ✅ Met — `tool-metrics.json` now written non-empty at capture time; verified via smoke capture (`"tools": {"read_file": {...}, "runSubagent": {...}}`).
5. ✅ Already correct — `has_spill`/`find_spill_pointer` in `hook/tool_execution.rs` predates this ticket and was verified still functioning (existing coverage untouched).
6. ✅ Met — real `input_tokens`/`output_tokens`/`cost_usd` flow per sub-agent from `event_meta` (test `real_token_and_cost_totals_flow_per_span`); values are real once the capture hook populates `data_json.usage` per ticket 9d527ad1 (done).
7. ⏳ Deferred — sibling tickets under 79c4ac3e must themselves consume this report as evidence; that consumption is out of this ticket's scope and depends on each sibling's own implementation.

No CLI/MCP wiring (e.g. `audit-cli`, `session-mcp`) was added — those live outside `session-api` and were out of file-ownership scope for this session. `SessionStoreConfig::delegation_cost_report` is the supported library-level command; wiring an external CLI/MCP surface to call it is a small follow-up.

### Validation

- `cargo build -p session-api` — pass.
- `cargo test -p session-api` — pass, 158/158 lib tests + all integration suites (0 failed).
- Manual smoke: synthetic transcript capture via `capture_copilot_transcript` → confirmed `tool-metrics.json` non-empty and `delegation_cost_report` correctly attributed a nested `read_file` call to its `runSubagent` span (ephemeral example removed after validation).
