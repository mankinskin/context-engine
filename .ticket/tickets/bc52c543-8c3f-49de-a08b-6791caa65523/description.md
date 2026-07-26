## Objective

Phase 2 of the self-optimization loop: **automatically auto-tune the delegation cost-class thresholds** (ticket 373072a9) from the recorded quality/cost metric (ticket 8ad2581e), removing the human-approval step from phase 1.

## Context

Phase 1 (ticket 8ad2581e) produces a human-approved "cheapest model meeting standards" recommendation using a rolling-window composite score. This ticket closes the loop by letting the system adjust thresholds itself.

## Requirements

- Consume the phase-1 composite metric and automatically adjust the capability-role → cost-class thresholds in the delegation policy.
- Include guardrails: bounded adjustment steps, rollback on regression, and an audit trail of automated changes.

## Acceptance criteria

- Automated threshold adjustment driven by the metric, with configurable bounds.
- Regression protection: an adjustment that worsens the composite score is reverted.
- Every automated change is recorded and inspectable.

## Depends on

- Phase 1 metric + recommendation (ticket 8ad2581e).