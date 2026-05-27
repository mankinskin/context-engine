# Problem

`context-insert::bundle_overlap` still branches on `self_overlap` and `overlap_is_shared_then_t1`, and it falls back to raw `insert_patterns`. The formula is branchy and hard to reason about.

## Deviation from the algorithm

Both sides of the bundle should be derived from the same shared-overlap witness and complement construction, not two side-channel formulas plus an ad hoc pair insertion.

## Design decisions

- `bundle_overlap` should expose one structural formula over the shared token plus left and right partitions.
- Self-overlap and direct-pair cases must be representable as degenerate partitions of the same formula, not as separate semantic branches.
- `context-read` should pass witnesses and participating tokens only; `context-insert` owns partition recovery and bundle materialization.

## Specification touchpoints

- Keep the `context-read pipeline` spec focused on block semantics, not on insert-side branching details.
- If a spec entry names overlap bundling explicitly, describe the general formula rather than preserving the current special cases.

## Manual validation guidelines

When a spec-backed implementation lands for this ticket:

1. Run focused insert-side checks:
	- `cargo test --manifest-path context-stack/context-insert/Cargo.toml overlap_bundle -- --nocapture`
2. Run at least one read-side consumer check:
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_repeating_known1 -- --nocapture`
3. Manually confirm from the resulting graph state or logs:
	- all bundled decompositions preserve the full bundle width;
	- self-overlap and direct-pair cases use the same structural reasoning as nested overlaps;
	- `context-read` is no longer compensating for insert-side branch behavior.

## Scope

- derive bundle shapes from one algebraic formulation over the shared token plus left/right partitions
- remove raw `insert_patterns` special-case construction where the general formulation should suffice
- extend insert-side tests for self-overlap, direct pair, nested overlap, and empty-side cases

## Acceptance criteria

- overlap bundle construction is described by one consistent formula
- insert-side tests cover self-overlap, direct pair, nested path, and empty partitions
- `bundle_overlap` no longer requires `overlap_is_shared_then_t1`
