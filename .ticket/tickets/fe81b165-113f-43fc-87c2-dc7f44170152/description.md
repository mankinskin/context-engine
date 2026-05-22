# Objective

Remediate the remaining `context-read` crate test failures that block full `context-stack` integration, while aligning the ticket set and spec language with the clarified read algorithm.

## Clarified algorithm model

- Each matching range corresponds to at most one token; the token is the unique identifier for that substring.
- A token may have multiple first-class decompositions in `child_patterns`.
- `child_patterns` are not canonically ordered. Any traversal order is operational and depends on the caller.
- Normalization from an embedded path to a materialized token is only required on the most abstract API surfaces.
- For safety, the implementation should materialize state after each overlap expansion step before searching for the next overlap.

## Per-step read rule

For the current implementation track, each overlap expansion step should proceed as:

1. Search the largest next overlap.
2. Complete the left and right complements implied by the resulting path start/end.
3. Commit that overlap to the current root, including edge-case handling for anchor/root replacement.

## Validation snapshot

Command run on 2026-05-22:

```bash
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib
```

Current result:
- 73 passed
- 8 failed

Failing tests:
- `tests::linear::repetition_aabbaabb`
- `tests::ngrams_validation::validate_mixed_pattern`
- `tests::overlapping::complex_abcabababcaba`
- `tests::read::read_infix1`
- `tests::read::read_infix2`
- `tests::read::read_multiple_overlaps1`
- `tests::read::read_repeating_known1`
- `tests::read::sync_read_text2`

## Active work buckets

- normalization policy on abstract surfaces versus lower-level embedded paths
- overlap-step materialization, retention, and structural invariant safety
- largest-overlap incremental join behavior for repeat and rotating-overlap cases

## Done when

- The child remediation tickets are resolved under the clarified model above.
- The existing `read-sequence` / `context-read pipeline` specs are updated to match that model.
- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib` passes, or any rebaselined tests are explicitly justified by the clarified contract.