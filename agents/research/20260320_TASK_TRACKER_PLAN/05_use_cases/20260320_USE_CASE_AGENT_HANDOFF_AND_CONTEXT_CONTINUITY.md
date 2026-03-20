# Use Case: Agent Handoff and Context Continuity

## Goal

Enable one agent to pause and another agent to continue work on the same ticket with minimal context loss and strong auditability.

## Preconditions

- Ticket schema supports handoff metadata (`handoff_from`, `handoff_to`, `handoff_notes`).
- Work lease mechanism is active.
- History/diff is queryable per ticket.

## Scenario

1. Agent A works on a ticket under active lease.
2. Agent A creates structured handoff note summarizing intent, current status, open risks.
3. Agent A releases lease and sets state to `handoff-ready`.
4. Agent B acquires lease, reviews diff+handoff, and continues implementation.
5. Any divergence from handoff plan is recorded as follow-up refinement notes.

## Data Flows

- Ticket files: `handoff.md` or handoff section in manifest.
- Index: handoff chain metadata for traceability.
- Git history: shows exact cutover between agents.

## Concurrency Rules

- Handoff is atomic: write handoff data and release lease in one operation.
- New assignee cannot acquire lease until handoff operation commits.
- If handoff is stale beyond TTL, ticket returns to planning queue.

## Failure Modes

- Handoff without sufficient context: automated quality check flags missing sections.
- Lease release fails after writing handoff: watchdog force-clears stale lease after timeout.
- Agent B starts without reading handoff: optional policy gate requires acknowledgment.

## Success Metrics

- Time from handoff-ready to resumed work.
- Rework caused by handoff ambiguity.
- Percentage of handoffs with complete context package.
