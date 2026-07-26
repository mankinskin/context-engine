## Objective

Establish **quality gates before and after delegated sessions** and collect the underlying data (sessions, tool calls, delegated sessions) needed to understand how often delegated sessions produce satisfactory work.

## Requirements

- Place a quality gate before and after each delegated session.
- Collect data from: sessions, tool calls, and delegated sessions.
- The collected data must support measuring how often delegated sessions do satisfactory work, per model.

## Data store (decided in refinement)

Reuse existing infrastructure — do **not** build a new store:

- `session-api` for session/delegated-session records.
- `test-api` for validation/quality-gate evidence.
- existing session tool-metrics for tool-call data.

## Acceptance criteria

- Pre- and post-session quality gates are defined and recorded for delegated sessions.
- Session/tool-call/delegated-session data is captured via session-api, test-api evidence, and tool-metrics.
- Data schema is sufficient to compute a per-model satisfactory-work rate downstream (ticket 8ad2581e).

## Anchor

Foundation for the delegation quality/cost metric (ticket 8ad2581e); post-session gate outcomes also feed the escalation policy (ticket 22c55989).