# context-stack/graph-induction/read-sequence/context-read-pipeline

This internal child spec describes how multi-character reads are turned into a
root token inside `context-read`. It is the algorithmic detail beneath the
public [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
command spec.

See also the sibling [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
spec.

## Pipeline

1. `context_read::read` is a one-shot entry point over
   `ReadCtx::new(...).read_sequence()`.
2. Input is first resolved into `NewAtomIndex` values.
   - `ReadCtx::new` resolves eagerly via `IntoReadInput`.
   - `ReadCtx::from_chars` and `ReadCtx::from_reader` resolve lazily via
     `LazyAtomIter`.
   - Unknown characters are inserted as atoms when they are first resolved.
3. `SegmentIter` partitions the resulting stream into
   `NextSegment { unknown, known }`.
   - `unknown` is the contiguous run of atoms that were new at resolution time.
   - `known` is the following contiguous run of atoms that already existed in
     the graph.
4. `RootManager::append_pattern` appends `unknown` runs directly to the running
   root.
   - This path may keep a temporary `flat_root` that is safe to extend in
     place.
   - Unknown runs do not perform overlap search.
5. `BlockExpansionCtx` handles each non-empty `known` run.
   - `ExpansionCtx` scans left-to-right from its internal cursor.
   - `insert_next_match` chooses the current largest token at the cursor.
   - `find_overlap` probes true postfixes of the current anchor and emits
     `BandState::WithOverlap` only when a wider token extends one of those
     postfixes.
6. Each emitted `BandState` is committed immediately through
   `RootManager::commit_state`, and the next expansion step uses the refreshed
   anchor from that commit.
7. `ReadCtx::read_sequence` returns the final root after all segments are
   consumed, or `None` for empty input.

## Root accumulation

`RootManager` maintains three pieces of state: the running `root`, the last
committed `anchor`, and whether the current root is still a flat unknown-atom
container.

`BandState::Single` commits by:

- creating the root on the first token,
- extending an in-place flat root when that is still safe,
- otherwise wrapping the current semantic root with the new token,
- and optionally applying the `try_extend_tail_with` repeated-single-atom
  optimization.

`BandState::WithOverlap` commits by:

- collapsing the overlap bundle into one token,
- replacing the root entirely when the root is still the left-side anchor,
- otherwise replacing the last child of the accumulated root,
- and setting `anchor` to the right-side expansion token (`t2`), not the
  bundled overlap token.

## Stable vs draft

The current code and regression corpus support these internal invariants:

- unknown runs are concatenated directly and never participate in overlap search
  inside the same segment step;
- known runs are processed one committed state at a time;
- the final root covers the input in order and preserves graph validity;
- overlap detection is driven only by postfix expansion from the current
  anchor.

The following details are not frozen as stable contract yet:

- the exact order in which alternative overlaps are discovered when several are
  possible;
- the lazy/eager constructor equivalence as a fully regression-tested
  guarantee;
- lower-layer behavior currently tracked by active tickets, especially the
  search-cache root mismatch and repeated-single-character width bug.