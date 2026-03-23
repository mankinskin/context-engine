# Use Case: Branch-Boundary Implementation

## Goal

Track a ticket that begins implementation on a feature branch and is only marked done at the merge commit to main.

## Preconditions

- Git-backed history is enabled.
- Ticket schema supports branch linkage fields (`branch`, `merge_commit`, `integration_state`).
- Merge event hooks are available (or polling against git refs).

## Scenario

1. Agent claims ticket and creates branch `feat/<ticket-id>-<slug>`.
2. Ticket moves to `in-progress` with branch metadata attached.
3. Commits on the branch are linked to ticket UUID in commit trailers.
4. Ticket remains in integration states (`review`, `qa`, `merge-queued`) while PR is open.
5. On merge commit to main, system records `merge_commit` and final integration checks.
6. Only then ticket transitions to `done`.
7. Optional post-merge archival policy moves ticket to `archived` after retention period.

## Data Flows

- Git: branch refs, commit graph, merge commit SHA.
- Ticket files: implementation notes, verification checklist, PR references.
- Index: branch status, PR URL, merge readiness, integration blockers.

## Concurrency Rules

- Ticket lock guards branch metadata updates.
- Multiple tickets can share one branch only if schema allows multi-ticket branch mapping.
- Merge finalization is atomic: write ticket state + merge metadata in one transaction boundary.

## Failure Modes

- Branch deleted before merge: ticket transitions to `needs-branch-recovery`.
- Squash merge loses commit links: fallback to PR metadata mapping.
- Cherry-pick workflows: ticket may have multiple terminal SHAs; preserve all.

## Success Metrics

- Accuracy of ticket->merge commit mapping.
- Mean delay between merge and ticket closure.
- Number of tickets closed without verified merge linkage.
