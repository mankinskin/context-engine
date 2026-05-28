# Problem

`read()` should build larger tokens by repeatedly finding the largest next overlap and joining that overlap into the running root. The current failing repeat and rotating-overlap cases show that this incremental join logic is missing expected decompositions or committing them incorrectly.

This ticket is explicitly **not** about choosing a canonical child-pattern order. Tokens may have multiple first-class decompositions.

## Stable constraints for this work

- each matching range maps to at most one token;
- there are no equal-width competing overlaps for the same match range;
- multiple decompositions of a token are first-class;
- the implementation materializes after each overlap expansion step before moving on.

## 2026-05-26 design clarification

The remaining failure is not just a missing decomposition ordering detail. It
is a dirty-cut replacement problem.

- The algorithm still requires a dedicated token for the requested span.
- If the requested span lands on a dirty cut, the implementation must still
  update the tight root, but it may do so either by a direct root pattern update
  or by splicing a beneficial wrapper token.
- Direct root updates are preferred whenever they preserve surviving outer
  context without introducing redundant wrapper structure.
- Wrapper tokens are still allowed when they create beneficial reusable
  adjacency or decomposition structure that the direct update cannot express as
  cleanly.
- Helper and inner ranges may be materialized to support that representation,
  but helper-only ranges must not be promoted into new authoritative clean
  boundaries.

## Current implementation gap

The current code already has most of the necessary surfaces, but the planning
state is implicit:

- `target_range` identifies the requested span;
- the merge operating range acts as the wrapper replacement span;
- `RequiredPartitions` identifies helper ranges;
- `add_root_pattern` splices the operating-range token into the root.

The gap is that these roles are not represented as one explicit replacement
plan, so helper materialization and legal replacement boundaries are easy to
confuse.

## Updated implementation plan

1. Introduce explicit root-merge planning state that separates the requested
  range from the pattern-local witnesses and from any wrapper range that is
  actually beneficial enough to splice.
2. Classify left, right, and interior witnesses as clean or dirty, and carry
  that authority alongside the plan.
3. Materialize the helper and inner ranges needed to expose the requested token
  inside the updated structure, but tag helper-only ranges so they do not
  become fake clean split boundaries.
4. Prefer direct root updates when they preserve surviving outer context without
  redundancy; use wrapper-backed replacement only when it is provably more
  useful.
5. Revalidate the overlap corpus, especially `complex_abcabababcaba`, to prove
  that the requested token becomes available without manufacturing extra clean
  cuts.

## Spec anchors

- [context-read pipeline](../../../.spec/specs/e0913182-7a5e-4c8f-a750-799afd58baae/body.md)
- [graph induction](../../../.spec/specs/16c3ad95-451d-4c09-a118-ca90bcefed9a/body.md)

## Per-step algorithm to preserve

1. Search the largest next overlap.
2. Complete the left and right complements from the resulting path start/end.
3. Commit that overlap to the current root properly, including edge cases around anchor replacement, root replacement, and last-child replacement.

## Reproduction

```bash
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_repeating_known1
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_multiple_overlaps1
```

## Current failures

- `tests::read::read_repeating_known1`
  - result width is `7` instead of `5`
  - expected root pattern includes the reusable token `xy`
- `tests::read::read_multiple_overlaps1`
  - an asserted branch resolves to `[b, c, d, ea]` instead of the larger overlap reuse `[bcde, a]`
  - the rolling sequence should keep reusing `bcde`, `cde`, `cdea`, `de`, `dea`, `deab`, and `abc` as related inputs are read

## Expected outcome

- Repeat and rotating-overlap reads follow the largest-overlap incremental join rule.
- The resulting graph contains the expected reusable overlap tokens and compatible decompositions after each step.
- Root commits handle edge cases without discarding valid decompositions that should remain first-class.

## Done when

- `read_repeating_known1` and `read_multiple_overlaps1` pass.
- The `context-read pipeline` spec documents the largest-overlap incremental join step precisely enough to explain these cases.
- The implementation carries an explicit requested-range versus wrapper-range
  replacement plan for dirty-cut commits.