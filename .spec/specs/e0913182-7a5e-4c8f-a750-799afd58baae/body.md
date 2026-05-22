# context-stack/graph-induction/read-sequence/context-read-pipeline

This internal child spec describes how multi-character reads are turned into a
root token inside `context-read`. It is the algorithmic detail beneath the
public [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
command spec.

See also the sibling [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
spec.

## Core model

- Each matched range corresponds to at most one token. The token is the unique
  graph identifier for that substring span.
- A token may have multiple first-class decompositions in `child_patterns`.
- `child_patterns` are not canonically ordered. Any traversal order is
  operational and depends on the caller.
- An embedded path and a materialized token may refer to the same substring.
  Normalization from one to the other is an API policy question, not a graph
  identity rule.
- For safety, the current implementation track materializes graph state after
  each overlap expansion step before searching for the next overlap.

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
5. `BlockExpansionCtx` handles each non-empty `known` run one overlap expansion
   step at a time.
   - `ExpansionCtx` scans left-to-right from its internal cursor.
   - `insert_next_match` chooses the largest next overlap reachable at the
     current cursor position.
   - Equal-width competing overlaps for the same matched range do not occur,
     because that matched range already identifies a unique token.
   - From the matched path, the step reconstructs the left and right
     complements implied by the path start and end.
6. Each overlap expansion step is then committed immediately through
   `RootManager::commit_state`.
   - The step is materialized before the next overlap search begins.
   - The next search uses the refreshed `anchor` from that commit as its
     left-side context.
7. `ReadCtx::read_sequence` returns the final root after all segments are
   consumed, or `None` for empty input.

## Per-step overlap rule

For the current implementation track, every overlap expansion step should be
describable as:

1. search the largest next overlap;
2. complete the left and right complements from the resulting path start/end;
3. commit that overlap to the current root, including the relevant edge-case
   handling.

This rule is intentionally about step progression, not about choosing a
canonical child-pattern ordering for the affected token.

## Root accumulation state

`RootManager` maintains three pieces of state:

- `root`: the running token covering everything committed so far;
- `anchor`: the last committed token that should act as left-side context for
  the next overlap search;
- `flat_root`: whether the current root is still a flat unknown-atom container
  that may be extended in place without semantic overlap handling.

`BandState::Single` commits by:

- creating the root on the first token,
- extending an in-place flat root when that is still safe,
- otherwise wrapping the current semantic root with the new token,
- and optionally applying `try_extend_tail_with` when the repeated-single-atom
  path can safely materialize a wider token.

`BandState::WithOverlap` commits by:

- collapsing the overlap bundle into one token,
- replacing the root entirely when the root is still the left-side anchor,
- otherwise replacing the last child of the accumulated root,
- and setting `anchor` to the right-side expansion token (`t2`), not the
  bundled overlap token.

## Ordering and normalization

- The graph may store several valid decompositions for the same token.
- Search, path traversal, and projection APIs may use different orders over
  those decompositions depending on the operation.
- Only the most abstract API surfaces are required to expose normalized facets
  of results. Lower-level path and cursor surfaces may retain embedded-path
  coverage information until an explicit normalization step occurs.

## Worked traces

These traces intentionally document stable step progression, state variables,
and resulting decompositions without freezing any canonical ordering over
`child_patterns`.

### `heldld -> hell`

`heldld` demonstrates the boundary between immediate materialization and
abstract-surface normalization.

- `root` starts as a `flat_root` while the early unknown atoms are appended.
- Once `ld` is recognized as the largest next overlap, that step is
  materialized immediately and `root` becomes `[[h, e, ld, ld]]`.
- `anchor` refreshes to the token produced by the most recent committed step,
  so the second `ld` reuse can be discovered from up-to-date context.
- The stable point is the materialized `ld` token and the final root
  decomposition `heldld -> [[h, e, ld, ld]]`.
- Whether `he`, `hel`, or `held` are exposed on a given surface is a retention
  and normalization policy question, not proof of a distinct canonical
  decomposition.

Reading `hell` later shows refinement from the already-materialized graph:

- `he` becomes available as `[[h, e]]`.
- `hel` becomes `[[he, l]]`.
- `held` may be represented as both `[[hel, d]]` and `[[he, ld]]`.
- The progression illustrates that later related reads can materialize tighter
  tokens and additional decompositions without invalidating the earlier `heldld`
  root.

### `aabb -> aabbaabb`

`aabb` is the clearest small worked trace for the three-step overlap rule.

For `aabb`:

1. `aa` is materialized as soon as the repeated-atom overlap is available.
   - `root` transitions from a `flat_root` atom chain to a semantic compound.
   - `anchor` becomes the right-side token committed by that step.
2. The first trailing `b` is appended as a complement, yielding the transient
   root `aab`.
3. The next `b` produces the largest next overlap with the current tail.
   - left complement: the existing `aa`
   - overlap token: `bb = [[b, b]]`
   - committed root: `aabb = [[aa, bb]]`

For `aabbaabb`:

- the already-materialized token `aabb` is reused as the next largest overlap;
- the final committed root is `[[aabb, aabb]]`;
- the exact internal decomposition set of `aabb` is not frozen here beyond the
  stable root-level reuse in `aabbaabb`.

### `xyyxy`

`xyyxy` is the minimal repeat-reuse trace.

- `xy` is materialized as the first reusable overlap token.
- The middle `y` remains as the complement between the two reused `xy` spans.
- The final committed root is `xyyxy -> [[xy, y, xy]]`.
- `anchor` advances after each committed overlap step so the rightmost `xy`
  reuse is found against the latest materialized state, not against the initial
  flat read.

### `abcde -> bcdea -> cdeab -> deabc`

This rotating-overlap family shows how related reads grow the token set over
multiple iterations.

`abcde`:

- no useful overlap is available yet;
- `root` remains a `flat_root` chain `[[a, b, c, d, e]]`.

`bcdea`:

- the largest next overlap is `bcde`;
- the right complement is `a`;
- committed root: `bcdea -> [[bcde, a]]`;
- the earlier root can now also be refined as `abcde -> [[a, bcde]]`.

`cdeab`:

- new materialized overlap tokens include `cde`, `ab`, and `cdea`;
- committed root: `cdeab -> [[cde, ab], [cdea, b]]`;
- related roots accumulate compatible decompositions immediately after each
  overlap step is committed.

`deabc`:

- further largest-overlap steps materialize `de`, `dea`, `bc`, `deab`, and
  `abc`;
- committed root: `deabc -> [[de, abc], [dea, bc], [deab, c]]`.

The stable rule is that each read proceeds by largest next overlap, then left /
right complement completion, then immediate commit to the root before the next
search begins.

### `subdivision -> visualization` and `subvisu -> visub`

These traces separate graph correctness from normalization policy.

`subdivision -> visualization`:

- `subdivision` begins as a flat character chain on its first read.
- The later read of `visualization` materializes reusable overlap tokens
  `su`, `vi`, `vis`, `visu`, and `ion`.
- After those commits:
  - `visualization -> [[visu, a, l, i, z, a, t, ion]]`
  - `subdivision -> [[su, b, d, i, vis, ion]]`
- `flat_root` is no longer sufficient once these semantic overlap tokens are
  being committed.

`subvisu -> visub`:

- `subvisu` first materializes `su` and yields `[[su, b, v, i, su]]`.
- Reading `visub` then reuses `vi`, `sub`, and `visu` as overlap products.
- The committed decompositions include:
  - `visub -> [[visu, b], [vi, sub]]`
  - `subvisu -> [[visu, b], [vi, sub]]`
- Lower-level search/path surfaces may still expose embedded coverage while
  abstract surfaces choose whether to normalize those results.

### `abcabababcaba`

This is the large overlap-family trace that exercises repeated step-wise
materialization.

The stable progression materializes the following reusable tokens as the read
advances:

- `ab -> [[a, b]]`
- `aba -> [[ab, a]]`
- `abab -> [[ab, ab], [aba, b]]`
- `ababa -> [[ab, aba], [abab, a]]`
- `ababab -> [[ab, abab], [ababa, b]]`
- `caba -> [[c, aba]]`
- `abc -> [[ab, c]]`
- `abcaba -> [[ab, caba], [abc, aba]]`
- `abcabab -> [[abc, abab], [abcaba, b]]`
- `abcababa -> [[abc, ababa], [abcabab, a]]`
- `abcababab -> [[abc, ababab], [abcababa, b]]`
- `ababcaba -> [[ab, abcaba], [abab, caba]]`
- `abababcaba -> [[ab, ababcaba], [ababab, caba]]`

The final root is materialized with the compatible decompositions:

- `abcabababcaba -> [[abc, abababcaba], [abcababab, caba]]`

This trace does not freeze the traversal order in which those peer
decompositions are later enumerated. It freezes the per-step overlap rule and
the resulting token/decomposition set once the read completes.

## Stable vs draft

The current code and regression corpus support these internal invariants:

- unknown runs are concatenated directly and never participate in overlap search
  inside the same segment step;
- known runs are processed one materialized overlap step at a time;
- the final root covers the input in order and preserves graph validity;
- overlap detection is driven only by postfix expansion from the current
  anchor;
- multiple valid decompositions may coexist without implying any canonical
  child-pattern order.

The following details are not frozen as stable contract yet:

- the exact abstract API surfaces that must normalize embedded paths;
- the precise retention and invalidation policy for all intermediate
  materialized results;
- the lazy/eager constructor equivalence as a fully regression-tested
  guarantee;
- lower-layer behavior currently tracked by active tickets, especially the
  width/border invariant failures and the remaining normalization-boundary
  tests.