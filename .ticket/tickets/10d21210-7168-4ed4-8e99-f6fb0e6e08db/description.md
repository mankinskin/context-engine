## Problem

Nine acceptance criteria across epic `79c4ac3e` and its children are phrased as *"on a comparable follow-up session"* or *"a replayed equivalent of either session"*. Neither phrase is defined. There is no specification of what prompt to use, what initial repo state to establish, or how to control for model non-determinism.

Both the review and the roast of the ticket set independently rated this the top defect. Roast: *"You cannot verify what you cannot operationalize."* Review: *"Without reproducible session conditions, comparison is meaningless."*

Affected acceptance criteria:

| ticket | AC | text |
|---|---|---|
| `79c4ac3e` | AC4 | "A replayed equivalent of either session shows measurably fewer sub-agent turns and fewer shell commands" |
| `77eb143b` | AC4 | "On a comparable follow-up session, substitutable shell commands drop by >=50% versus the 116/298 baseline" |
| `fb14754e` | AC4 | "On a comparable follow-up session, `read_file` / `list_dir` path-resolution failures drop to zero" |
| `fb14754e` | AC5 | "Exploratory `find` / `ls` commands issued solely to locate a named crate drop to zero" |
| `cc3324c9` | AC5 | "On a comparable follow-up session, the count of artifacts read by more than two distinct sub-agents drops to zero" |
| `66acb737` | AC5 | "On a comparable follow-up session, model distribution across delegations is no longer uniform" |
| `46d8b25d` | AC5 | "On a comparable follow-up session, re-dispatch of the same task after a blocked delegation drops to zero" |

## Decision

Define a synthetic benchmark session with fixed inputs and a checked-in baseline, and rewrite every affected AC to measure against it.

## Scope

- Author a benchmark scenario that exercises the failure modes observed in `3e9bc20b` and `41966513`:
  - a delegation whose handoff package names a crate without its physical path
  - a delegation requiring an MCP write whose `workspace` argument is ambiguous
  - a fan-out of two or more sibling sub-agents needing the same artifact
  - a delegation whose precondition (missing spec) fails post-dispatch
  - a mechanical delegation that should route to a cheap tier
- Fix the inputs: exact orchestrator prompt, required repo state or fixture, ticket/spec fixtures, and the expected task outcome.
- Capture a baseline run and check in its event log as the reference artifact.
- Define the comparison metric set, computed by the analyzer from `b7c61f0e`: sub-agent count, turns per sub-agent, tool histogram, substitutable-shell count, cross-agent duplicate reads, path-resolution failures, re-dispatch count. Add token and cost once `9d527ad1` lands.
- State explicitly how model non-determinism is handled: either N repeated runs with a reported spread, or metrics chosen to be robust to phrasing variation. Do not claim single-run comparability.
- Rewrite the seven ACs above to reference this benchmark and a numeric threshold.

## Acceptance Criteria

1. A benchmark scenario exists with fully specified inputs — prompt, fixtures, repo state — reproducible without further interpretation.
2. A baseline run is captured and its event log checked in as the reference artifact.
3. The comparison metric set is defined and computed by the `b7c61f0e` analyzer, not by ad-hoc scripting.
4. The non-determinism handling is stated, with either a run count and reported spread or a justification that the chosen metrics are phrasing-robust.
5. Every AC in the epic and its children that previously said "comparable follow-up session" names this benchmark and a numeric threshold instead.
6. Re-running the benchmark against the unchanged repo reproduces the baseline metrics within the stated spread — proving the harness measures the system, not noise.

## Graph position

This ticket `depends_on` `b7c61f0e` (the analyzer), because the benchmark's comparison metrics are computed by that analyzer. Epic `79c4ac3e` `depends_on` this ticket.

## Note on ordering

This ticket gates the *verification* of `77eb143b`, `fb14754e`, `cc3324c9`, `66acb737`, and `46d8b25d`, not their implementation. Those tickets can be implemented in parallel; they cannot be closed on cost-reduction grounds until this benchmark exists.

## Evidence

- Baseline sessions: `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`, `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json`
- Metric definitions and the throwaway implementation: `tmp/subagent_cost_probe.py`
- Analyzer that will own the metric computation: `b7c61f0e`