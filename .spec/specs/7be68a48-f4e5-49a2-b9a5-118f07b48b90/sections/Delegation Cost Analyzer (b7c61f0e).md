## Sub-agent span attribution

`session-api`'s capture hook (`hook::transcript`) resolves each event's owning
`runSubagent` span at capture time using true `parent_event_id` ancestry (not
raw event-index overlap), and stamps the result onto
`SessionTurnEventMeta::subagent_run_id` (the `tool_call_id` of the nearest
enclosing `runSubagent` invocation, or `None` for top-level orchestrator
turns). Because every event has exactly one ancestor chain, this attributes
nested and parallel/overlapping sub-agent spans without double-counting.

`subagent_rollup::compute_subagent_rollups` groups turns by
`subagent_run_id` when present (falling back to the parent session id for
transcripts captured before this field existed), giving real per-sub-agent
token/cost/tool-call totals sourced from `data_json.usage` once ticket
9d527ad1 populates it.

## Delegation cost report

`delegation_cost::compute_delegation_cost_report` (exposed as
`SessionStoreConfig::delegation_cost_report`) is the supported command that
reproduces the analysis behind epic 79c4ac3e: per-sub-agent tool histograms,
within-agent repeat reads/commands, cross-agent duplicate reads (path
normalization safe via `normalize_path_for_dedup`), cross-agent duplicate
commands, failure classification, and real per-sub-agent token/cost totals.
It supersedes the throwaway `tmp/subagent_cost_probe.py` analyzer, which has
been removed.

## tool-metrics.json at capture time

`SessionStorePlan::persist` now computes and writes `tool-metrics.json`
immediately after every capture (not only lazily on first aggregate read), so
newly captured sessions always have a non-empty, up-to-date per-tool summary.
