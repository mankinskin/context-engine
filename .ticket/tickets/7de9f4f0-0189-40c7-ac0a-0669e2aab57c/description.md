# Completion-claim audit mechanism

## Motivation

Ticket 9d527ad1 was found in state `done` with a description asserting AC1-AC6 were satisfied, while the claimed emission path and `duration_ms` field did not exist in the code at all. `history.ndjson` for that ticket shows two separate `"state":"done"` revisions before reversion to `in-review`.

The same iteration also found spec 7be68a48 with a `sections/Traceability` file of 0 bytes, claimed as an authored deliverable.

**Third incident, 2026-07-28 (scope-widening):** ticket 32067e83 was found in state `done` on the strength of `review_notes` recording a PASS verdict that was fabricated. The notes claimed `cargo test -p mcp-cost-gate` returned 54 passed (actual: 51) and cited normalization tests at specific `src/proxy.rs` line numbers that hold unrelated pre-existing telemetry tests. The underlying cause was that the reviewed implementation (commit `53de6c5`) had been erased by a later commit (`c58e9be`, `+3 / -144`) before the ticket closed, and nothing re-checked the code state at close time.

**Fourth incident, same day:** ticket 9d527ad1 shipped green and fully tested, but was never actually collecting anything in production because the env var enabling it was never set in any runtime config. Passing tests did not imply working behavior. Tracked separately as `4aa13ba7`.

These are the same failure mode: a completion claim recorded with no verifiable evidence behind it, which then poisons every downstream handoff that trusts the ticket state.

**Scope widened by user decision, 2026-07-28:** this ticket covers not only *implementation* completion claims but also *review records*. A fabricated review verdict is worse than a fabricated implementation claim, because the review gate is the control that is supposed to catch the latter. If review records cannot be trusted, the gate is decorative.

## Objective

Make it structurally difficult to record a `done` transition, or a review verdict, that is not backed by evidence.

## Candidate approaches (to be decided during refinement)

- A required `verified_by` field on the `done` transition, carrying a test-api execution id or validation-spec id rather than free prose.
- Reject `done` transitions when the ticket has acceptance criteria but no linked test-api execution.
- Extend `health_check` with a finding for `done` tickets that have zero linked validation evidence, so existing bad state is discoverable.
- Reject zero-byte spec section files at write time (spec-api side), so an "authored" section cannot be empty.
- Require review records to carry the same evidence references as completion claims, so a PASS verdict is checkable rather than prose.

## Acceptance criteria

1. A `done` transition on a ticket with acceptance criteria cannot be recorded without a linked validation-evidence reference (test-api execution or validation spec id).
2. `health_check` reports existing `done` tickets that carry no validation evidence, so the current backlog of unverified closures is enumerable.
3. Writing a spec section with empty content is rejected or flagged rather than silently accepted.
4. The mechanism is exercised by tests that assert both the rejection path and the accepted-with-evidence path.
5. **Pre-close git-state re-check (added by user decision).** Immediately before a `done` transition, the ticket's validation command is re-run and the current git HEAD of the ticket's target files is confirmed to still contain the reviewed changes. This is the single mechanical check that would have caught all three false-completion incidents: in each case the code state at close time was never compared against the state the reviewer examined. The check must fire on every traversed hop of a multi-hop transition, not only on the caller's requested target state, because the ticket store auto-walks transitions by default.
6. **Review records carry evidence, not prose (added by user decision).** A recorded review verdict must reference the validation evidence it was based on — a test-api execution id and the commit sha of the tree that was reviewed — so a later reader can detect that the reviewed tree has since been superseded. A PASS asserting a test count or file line numbers with nothing to check it against is exactly what produced the 32067e83 incident.

## Non-goals

- Retroactively re-verifying every historical `done` ticket. Criterion 2 only makes them discoverable.
- Any change to the `mcp-cost-gate` telemetry work itself.
- Fixing the specific 32067e83 revert; that is tracked on 32067e83 directly.

## Origin

Raised during the Review->Interview->Commit->Handoff iteration on 9d527ad1, 2026-07-28. Widened the same day after the 32067e83 fabricated-review incident. Related spec: 7be68a48. Related tickets: 32067e83, 4aa13ba7, 574560bf.
