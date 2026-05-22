# Problem

The current failing assertions around infix and overlap matches assume that lower-level path results must always normalize to an `EntireRoot` materialized token.

The clarified contract is narrower:

- a materialized token and an embedded path may refer to the same substring;
- normalization is an API policy choice, not a graph identity rule;
- only the most abstract API surfaces are required to expose normalized facets.

This means the current failures are tracking an inconsistent normalization boundary rather than automatically proving graph corruption.

## Reproduction

```bash
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib validate_mixed_pattern
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib complex_abcabababcaba
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_infix1
```

## Current failures

- `tests::ngrams_validation::validate_mixed_pattern`
  - `bb`: expected `EntireRoot`, got `PathCoverage::Postfix`
- `tests::overlapping::complex_abcabababcaba`
  - `aba`: expected `EntireRoot`, got `PathCoverage::Postfix`
- `tests::read::read_infix1`
  - `vis`: expected `EntireRoot`, got `PathCoverage::Prefix`

## Expected outcome

- The normalization boundary is documented explicitly.
- Abstract read/search surfaces normalize semantically equivalent embedded-path results when required.
- Lower-level path/cursor surfaces may retain `Prefix` / `Postfix` coverage until an explicit normalization step.
- The failing tests are resolved accordingly, either by routing them through a normalized surface or by asserting semantic equivalence on the lower-level surface they are actually exercising.

## Done when

- `validate_mixed_pattern`, `complex_abcabababcaba`, and `read_infix1` are resolved under the documented normalization contract.
- The `context-read pipeline` and `read_sequence` specs make the normalization boundary explicit.