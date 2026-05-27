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
- If the requested span lands on a dirty cut, the parent pattern must not
  replace at that dirty edge directly.
- Instead, the implementation must extend to a clean wrapper range, create or
  reuse the wrapper token, splice that wrapper token into the parent or root
  pattern, and ensure the requested token appears as a first-class
  decomposition of the wrapper token.
- Helper ranges may be materialized to support that representation, but helper
  ranges must not be promoted into new authoritative clean boundaries.

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

1. Introduce an explicit replacement plan for root merge that separates the
  requested range from the wrapper replacement range.
2. Classify left, right, and interior witnesses as clean or dirty, and carry
  that authority alongside the plan.
3. Materialize the helper ranges needed to expose the requested token inside the
  wrapper token, but tag helper-only ranges so they do not become fake clean
  split boundaries.
4. Keep root or parent replacement at wrapper scope whenever the requested edge
  is dirty.
5. Revalidate the overlap corpus, especially `complex_abcabababcaba`, to prove
  that the requested token becomes available without manufacturing extra clean
  cuts.

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