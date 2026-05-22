# Problem

`context-read` should materialize graph state after each overlap expansion step to keep progression safe, but that materialized state must still obey retention policy and structural invariants.

Immediate visibility is not itself the bug. The bug class here is one of these:

- a materialized intermediate is retained when it should have been invalidated or replaced;
- a materialized commit breaks graph invariants such as width or border consistency;
- an overlap-step commit leaves the graph or trace cache in an invalid transitional state.

## Clarified policy

- retain results as long as possible, until they may be invalidated or released;
- materialize after each overlap expansion step;
- only abstract API surfaces are required to normalize results.

## Reproduction

```bash
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib sync_read_text2
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib repetition_aabbaabb
cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_infix2
```

## Current failures

- `tests::read::sync_read_text2`
  - after reading `heldld`, `held` is unexpectedly available as a complete match under the current test expectation
  - this case now needs to be resolved against the clarified retention/materialization policy rather than the older "hide intermediates" assumption
- `tests::linear::repetition_aabbaabb`
  - panic: pattern width mismatch in `T7w6` token pattern `["aa"(2), "abbabb"(10)]`
- `tests::read::read_infix2`
  - panic: duplicate border in the `subvisu` / `visub` overlap case between patterns `[su, b, visu]` and `[s, ub, visu]`

## Expected outcome

- Overlap-step materialization is safe and leaves the graph in a structurally valid state after every commit.
- Width and border invariants hold for all visible tokens.
- `sync_read_text2` is resolved in a way that matches the clarified retention policy for materialized intermediate results.

## Done when

- `repetition_aabbaabb` and `read_infix2` pass without width/border panics.
- `sync_read_text2` is either updated or fixed to match the documented retention/materialization policy.
- The `context-read pipeline` spec explicitly documents when step-local results are materialized and when they may be retained or invalidated.