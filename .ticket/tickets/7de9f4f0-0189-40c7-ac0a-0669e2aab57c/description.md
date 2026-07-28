# Completion-claim audit mechanism

## Motivation

Ticket 9d527ad1 was found in state `done` with a description asserting AC1-AC6 were satisfied, while the claimed emission path and `duration_ms` field did not exist in the code at all. `history.ndjson` for that ticket shows two separate `"state":"done"` revisions before reversion to `in-review`.

The same iteration also found spec 7be68a48 with a `sections/Traceability` file of 0 bytes, claimed as an authored deliverable.

Both are the same failure mode: a completion claim recorded with no verifiable evidence behind it, which then poisons every downstream handoff that trusts the ticket state.

## Objective

Make it structurally difficult to record a `done` transition that is not backed by evidence.

## Candidate approaches (to be decided during refinement)

- A required `verified_by` field on the `done` transition, carrying a test-api execution id or validation-spec id rather than free prose.
- Reject `done` transitions when the ticket has acceptance criteria but no linked test-api execution.
- Extend `health_check` with a finding for `done` tickets that have zero linked validation evidence, so existing bad state is discoverable.
- Reject zero-byte spec section files at write time (spec-api side), so an "authored" section cannot be empty.

## Acceptance criteria

1. A `done` transition on a ticket with acceptance criteria cannot be recorded without a linked validation-evidence reference (test-api execution or validation spec id).
2. `health_check` reports existing `done` tickets that carry no validation evidence, so the current backlog of unverified closures is enumerable.
3. Writing a spec section with empty content is rejected or flagged rather than silently accepted.
4. The mechanism is exercised by tests that assert both the rejection path and the accepted-with-evidence path.

## Non-goals

- Retroactively re-verifying every historical `done` ticket. Criterion 2 only makes them discoverable.
- Any change to the `mcp-cost-gate` telemetry work itself.

## Origin

Raised during the Review->Interview->Commit->Handoff iteration on 9d527ad1, 2026-07-28. Related spec: 7be68a48.
