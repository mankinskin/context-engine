# Objective

Align the context-read failing tests with the clarified normalization/materialization contract before wider algorithm edits proceed.

## Contract to encode

- embedded paths and materialized tokens may be semantically equivalent;
- normalization is required only on the most abstract API surfaces;
- lower-level path/cursor surfaces may retain Prefix/Postfix-style coverage information;
- overlap expansion steps are materialized immediately for safety;
- results are retained until invalidated or released;
- visible graph state must always preserve structural invariants.

## Main impact area

This work is expected to narrow or reframe the expectations inside the current normalization/materialization failure buckets, especially where tests currently assume mandatory `EntireRoot` normalization or delayed visibility of intermediate results.

## Done when

- the affected failing tests and acceptance criteria are reviewed against the clarified contract;
- any tests that encoded the old assumptions are updated or explicitly kept with a documented reason;
- the spec/ticket language for normalization and materialization remains consistent with the resulting test expectations.

## Review outcome

- lower-level ancestor search assertions no longer require `EntireRoot` normalization when the contract only needs query exhaustion plus stable root-token identity;
- `context_search::assert_exhausted_indices!` now covers the low-level substring-token checks that were previously over-constrained by `assert_indices!`;
- the normalization-boundary callers in `read_infix1`, `validate_mixed_pattern`, and `complex_abcabababcaba` were switched to the weaker exhausted-match assertion;
- the delayed-visibility expectation in `sync_read_text2` was removed because lower-level `find_ancestor` surfaces may expose materialized intermediates immediately once overlap commits;
- the clarified spec/ticket language remains consistent with this outcome; no additional spec rewrite was needed in this ticket after the earlier overlap/materialization clarification work.

## Retained behavioral failures

- `validate_mixed_pattern` still fails after the contract cleanup because the produced root width is `5` instead of the expected `4`;
- `read_infix1` still fails because the observed graph shape materializes a wider `vis` pattern than the test expects;
- `sync_read_text2` still fails because `hell` materializes as `[he, ll]` instead of `[hel, l]`;
- `complex_abcabababcaba` still fails because at least one expected substring remains a non-exhausted `Prefix` match rather than an exhausted token match.

## Validation

- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib --no-run` passes after the contract-cleanup edits;
- focused test runs for `validate_mixed_pattern`, `read_infix1`, `sync_read_text2`, and `complex_abcabababcaba` still fail, but the failures now occur after the helper-level contract cleanup and point at real graph-shape or materialization behavior.