# Problem

`context-read` tests mostly assert whole decomposition families after long worked traces. Lower crates use smaller fixture-based tests to pin one primitive at a time. The current read suite turns every regression into a wide, ambiguous failure.

## Deviation from lower-crate practice

`context-trace`, `context-search`, and `context-insert` isolate traversal, witness, and bundling primitives. `context-read` needs the same layering so design failures and projection-order noise stop masquerading as the same bug.

## Design decisions

- Primitive-layer tests should fail near the owning abstraction: witness selection, overlap-state construction, block commit, or revisitation.
- Long worked traces remain valuable, but only as integration/regression coverage for stable graph outcomes.
- Reuse lower-crate fixture patterns wherever possible instead of inventing one-off `context-read` mega-fixtures for every family.

## Specification touchpoints

- Each spec-backed behavior in the read-spec chain should map to at least one focused test layer and one integration trace where appropriate.
- When spec entries are split or clarified, update the test matrix so the owning layer is obvious from the ticket.

## Manual validation guidelines

When a spec-backed implementation lands for this ticket:

1. Add or update the focused unit tests first, then rerun only the affected slice.
2. Manually confirm that a failure now points at one primitive layer rather than a full decomposition family.
3. Keep one worked-trace check for each family, but verify manually that those traces assert only stable graph facts and not incidental child-pattern ordering.

Suggested slices:

- postfix witness selection
- overlap-state construction
- block materialization / commit
- revisitation of equal-span roots

## Scope

- add focused tests for postfix witness selection, overlap-state construction, block commit, and revisitation
- keep broad worked traces as integration and regression tests only
- reuse lower-crate fixture patterns where possible

## Acceptance criteria

- new focused unit tests exist for each read primitive layer
- long worked-trace tests assert only stable graph-level outcomes
- failures point to one primitive layer instead of an entire overlap family
