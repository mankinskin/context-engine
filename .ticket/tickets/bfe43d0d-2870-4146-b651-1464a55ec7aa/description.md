# Problem

`read()` should build larger tokens by repeatedly finding the largest next overlap and joining that overlap into the running root. The current failing repeat and rotating-overlap cases show that this incremental join logic is missing expected decompositions or committing them incorrectly.

This ticket is explicitly **not** about choosing a canonical child-pattern order. Tokens may have multiple first-class decompositions.

## Stable constraints for this work

- each matching range maps to at most one token;
- there are no equal-width competing overlaps for the same match range;
- multiple decompositions of a token are first-class;
- the implementation materializes after each overlap expansion step before moving on.

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