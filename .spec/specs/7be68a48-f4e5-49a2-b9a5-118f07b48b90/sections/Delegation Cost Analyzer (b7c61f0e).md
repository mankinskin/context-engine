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

`DelegationCostReport` (memory-api/crates/session-api/src/delegation_cost.rs:148)
also carries these metrics, added while implementing the 10d21210 replay
benchmark and confirmed present in the shipped analyzer:

- `model_distribution: BTreeMap<String, u64>` — count of turns per resolved
  `model_id`, so per-model mix is visible even while `cost_usd` is null.
- `substitutable_shell_count` — shell tool calls whose command matches a
  pattern with a cheaper native-tool substitute (e.g. `find`/`ls` via a
  filesystem tool instead of a shell invocation).
- `exploratory_find_ls_count` — subset of `substitutable_shell_count` most
  associated with exploratory crate/file discovery rather than a targeted
  operation.
- `path_resolution_failures` — tool calls that failed because a referenced
  path did not resolve, signal for wasted exploratory round-trips.
- `redispatch_count` — count of repeat `runSubagent` dispatches for the same
  logical unit of work after an initial attempt, computed once `per_run` is
  fully populated.

`delegation_cost::compute_delegation_cost_report_from_events`
(memory-api/crates/session-api/src/delegation_cost.rs:557) is a second,
events-based entry point that computes the same `DelegationCostReport` shape
directly from a `PersistedSessionEvents` log (walking `parent_event_id`
ancestry to attribute `runSubagent` spans) instead of from an already-built
`SessionRecord`'s turns. It exists because the two 10d21210 baseline sessions
only populated tool-call data in their raw `events.json`, not in
`transcript.json` (per `hook::transcript::handle_message_event`), so the
turn-based `compute_delegation_cost_report` alone could not replay them. Both
entry points return the identical `DelegationCostReport` schema; the events
path is the one exercised by the 10d21210 checked-in replay test
(`memory-api/crates/session-api/tests/delegation_cost_benchmark.rs::replay_reproduces_checked_in_baseline_report_exactly`).

## tool-metrics.json at capture time

`SessionStorePlan::persist` now computes and writes `tool-metrics.json`
immediately after every capture (not only lazily on first aggregate read), so
newly captured sessions always have a non-empty, up-to-date per-tool summary.

## Quality gate primitives

`quality_gate::QualityGate`, `QualityGatePhase` (Pre/Post), and
`QualityGateOutcome` (Pass/Fail/Blocked) (memory-api/crates/session-api/src/quality_gate.rs:12-91)
implement the R1 quality-gate schema: `pre_delegation_gate` and
`post_delegation_gate` construct a gate for the corresponding phase, and
`QualityGate::with_validation_spec_id` / `with_detail` attach the `test-api`
validation spec id and outcome detail required by AC2.
