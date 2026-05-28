# context-stack/graph-induction/read-sequence/context-read-pipeline

This internal child spec defines the intended `context-read` algorithm for
building a multi-character root token. It is the algorithmic detail beneath the
public [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
command spec.

See also the sibling [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
spec.

## Core model

- Each matched span corresponds to at most one token.
- A token may have multiple first-class decompositions in `child_patterns`.
- `child_patterns` are never canonically ordered.
- The stable unit of progress is a committed block-expansion step.
- Segmenting input into unknown and known runs is an optimization boundary. It
  does not change the block algorithm.
- `context-read` should orchestrate existing lower primitives:
   - `context-trace` owns postfix/prefix path traversal;
  - `context-search` / `context-insert` own largest-match discovery;
  - `context-insert` owns structural overlap bundling.

## Block algorithm

For unread suffix `q_i`, current block `t_block_i`, selected postfix
`t_postfix_{i+1}`, and right expansion `t_overlap_{i+1}`:

```text
current block
+------------------------------------------------+
| t_complement_block_{i+1} | t_postfix_{i+1}     |
+------------------------------------------------+

right expansion
+------------------------------------------------+
| t_postfix_{i+1}          | t_complement_overlap_{i+1} |
+------------------------------------------------+
```

1. Materialize the largest initial block `t_block_0` available at the current
   unread position.
2. From the current block, follow each longest postfix path induced by repeated
   largest-direct-postfix links. Those paths already visit the smaller postfix
   tokens largest to smallest, so the search does not need to rescan every
   postfix of every node.
3. Choose the first postfix `t_postfix_{i+1}` on those paths that can be
   extended rightward into a strictly wider token `t_overlap_{i+1}`.
4. Derive complements so that:
   - `t_block_i = [t_complement_block_{i+1}, t_postfix_{i+1}]`
   - `t_overlap_{i+1} = [t_postfix_{i+1}, t_complement_overlap_{i+1}]`
5. Materialize the next block `t_block_{i+1}` with both first-class
   decompositions:

```text
t_block_{i+1}
|- [t_block_i,                t_complement_overlap_{i+1}]
`- [t_complement_block_{i+1}, t_overlap_{i+1}]
```

6. Commit `t_block_{i+1}` and continue from that block.
7. If no postfix can extend, commit the current largest standalone token and
   continue at the next unread position.

The next search always starts from the newly committed block, not from a stale
predecessor and not from a one-off root mutation shortcut.

## Segment boundary rule

`SegmentIter` may partition the input into `unknown` and `known` runs.

- Unknown runs may be concatenated directly because they cannot yet reuse
  existing compound structure.
- Known runs must follow the block algorithm above.

That split is an implementation detail for work scheduling. It does not change
the overlap semantics.

## Worked traces

These traces freeze the algorithmic progression and stable decomposition sets.
They do not freeze any canonical traversal order over `child_patterns`.

### `heldld -> hell`

```text
heldld => [h, e, ld, ld]
hell   => [hel, l]
held   => [hel, d] and [he, ld]
```

Later related reads may tighten earlier roots without invalidating them.

### `aabb -> aabbaabb`

```text
aabb     => [aa, bb]
aabbaabb => [aabb, aabb]
```

The same overlap rule explains both the local repeat and the later reuse of the
larger block.

### `xyyxy`

```text
xyyxy => [xy, y, xy]
```

The middle `y` is the complement between two reuses of the same overlap token.

### `abcde -> bcdea -> cdeab -> deabc`

```text
abcde => [a, b, c, d, e]
bcdea => [bcde, a], [b, cdea], [bc, dea]
cdeab => [cde, ab], [cdea, b]
deabc => [de, abc], [dea, bc], [deab, c]
```

The important rule is revisitation: once later reads materialize `bc` and
`dea`, the already-known span `bcdea` gains `[bc, dea]` as another first-class
decomposition.

### `subdivision -> visualization` and `subvisu -> visub`

```text
visualization => [visu, a, l, i, z, a, t, ion]
subdivision   => [su, b, d, i, vis, ion]
visub         => [visu, b] and [vi, sub]
subvisu       => [visu, b] and [vi, sub]
```

Later related reads may share reusable infix tokens across equal-span roots.

### `abcabababcaba`

This larger overlap family exercises the same rule repeatedly. The stable point
is the resulting token/decomposition set, not the order in which peer
decompositions are later enumerated.

```text
abcabababcaba => [abc, abababcaba] and [abcababab, caba]
```

## Boundary and replacement matrix

The block algorithm depends on the split/join layer classifying the cut used
for the next overlap commit. Because a given atom position may be a direct
boundary in at most one stored pattern, each queried cut has zero or one clean
split witness.

When a clean split exists, replacement-range selection may use it to avoid
unnecessary wrapper growth. When it does not, the commit must stay structural
and dirty-aware.

| Case | Overlap-commit condition | Clean split present? | Pipeline consequence |
| --- | --- | --- | --- |
| P1 | The next commit cuts on an already aligned child edge | Yes | The running root may reuse the target span directly; that side does not need wrapper expansion |
| P2 | The next commit cuts through a child interior | No | The step must keep the cut dirty and rely on wrapper or inner partitions to preserve compatible decompositions |
| P3 | Several peer decompositions cover the same span, but only one offers a clean cut | Yes, unique | The clean witness may stabilize the replacement range, while the other peers remain dirty supporting decompositions |
| P4 | No peer decomposition offers a clean cut for the needed replacement | No | The algorithm must not invent an aligned cut just to simplify root replacement; it must commit from dirty coverage instead |
| P5 | A later reread introduces a clean cut that earlier commits could not use | Yes, later | The later step may tighten the existing token or add a peer decomposition, but it still reuses the same equal-span token rather than duplicating structure |

`abcabababcaba` is sensitive to `P3` and `P4`: the missing decomposition case
is not solved by manufacturing extra clean cuts. The fix has to preserve the
unique-clean-cut invariant while still exposing the dirty coverage needed for
the replacement range.

## Merge-step requirements

Search hands merge the smallest existing root token that still covers the
requested atom range. A merge step therefore must always add at least one new
root-level decomposition. If no root pattern changes, the chosen root was not
actually tight.

For requested range `Q` inside root `R`:

1. `Q` must map to exactly one equal-span token.
2. Merge must inspect every root child pattern whose atom coverage contains `Q`,
   not only the first pattern encountered.
3. Each such pattern contributes a pattern-local witness: the smallest
   contiguous child slice in that pattern whose atom coverage contains `Q`.
4. A single merge may add one or more new root patterns. All first-class
   witnesses that remain legal after the update must be represented somewhere in
   the resulting decomposition set.
5. Because a given atom position may be a direct boundary in at most one
   pattern, at most one root pattern may witness a given requested edge cleanly.
   Complementary peers may clean different edges, but they must not duplicate
   the same clean boundary.
6. If an exposed adjacent child sequence covers span `S` while some other
  existing child token already covers `S` as a proper subrange, then `S` must
  already be tokenized. Merge must not use witness sets that leave such
  duplication unreplaced.
7. Prefer a direct root update whenever the root can expose `Q` next to the
   surviving outer context without losing prior child-token exposure or
   introducing redundant equal-span structure.
8. Use a wrapper only when it is beneficial. A wrapper is beneficial only if it
   places the covered span next to outer context in a way the direct root update
   cannot, or if it creates a reusable decomposition that avoids redundancy.
9. Dirty cuts strictly inside `Q` or inside a chosen wrapper induce inner
   partitions. Those inner partitions materialize as first-class members of `Q`
   or the wrapper token's `child_patterns`; they are not standalone root
   replacements.
10. Inner materialization is recursive. If an induced inner partition still
   crosses dirty child boundaries, merge must propagate the needed split edges
   and helper tokens into descendant nodes.
11. Merge must preserve root-reachable representation closure. Any token that
  the root represented before the update must remain reachable from the root
  after the update, even if it is no longer a direct child of a root pattern.
12. Existing compatible root and child decompositions remain first-class when
  they still fit the updated structure. Merge must not orphan prior witnesses
  just because a tighter one was added.

### Requirement matrix seed

| Case | Witness set at root | Preferred root action | Inner materialization | Validation focus |
| --- | --- | --- | --- | --- |
| M1 | Single clean-clean witness | Add a direct root pattern using `Q` | Only clean subranges inside `Q` | Direct clean-clean root update |
| M2 | Complementary one-sided dirty witnesses across multiple root patterns | Add the wrapper-backed root updates needed to preserve previously represented tokens indirectly | Required on each dirty side as needed | Multi-pattern witness and representation-closure |
| M3 | Single dirty-dirty witness with no beneficial wrapper | Add a direct root pattern and reject wrapper-only shortcuts | Required on both dirty sides as needed | Direct dirty-dirty root update |
| M4 | Dirty edge plus repeated subsequence inside `Q` | Add a direct root pattern or a proven-beneficial wrapper, and materialize the repeated inner partition inside it | Mandatory | Inner partition closure |
| M5 | Wrapper-beneficial witness set | Add a wrapper-backed root pattern update and expose `Q` inside the wrapper while preserving prior root-reachable tokens | Required wherever the wrapper still crosses dirty interior boundaries | Wrapper benefit and preservation proof |

### Planning surface

Any eventual merge-plan type has to represent at least:

- the requested equal-span range `Q`;
- the set of root-pattern witnesses keyed by pattern identity;
- the chosen root updates, which may be more than one pattern addition;
- any wrapper candidate together with the reason it is beneficial rather than
  redundant;
- the prior root-representation obligations that each chosen update must keep
  reachable;
- the induced inner materialization obligations and their clean-versus-dirty
  authority.

## Solved examples

Each solved example below follows the same procedure:

1. verify that the root is the tightest existing token containing `Q`;
2. reject the setup if any exposed adjacent child sequence violates
  duplication-replacement closure;
3. list the root-pattern witnesses that cover `Q`;
4. reject any candidate update that would make previously represented tokens
   unreachable from the root;
5. decide whether the root update should be direct or wrapper-backed;
6. materialize any required inner partitions inside `Q` or the chosen wrapper;
7. add the new root pattern and the needed child decompositions.

These examples intentionally avoid duplicated boundary positions across the root
pattern set.

### E1 Solve a clean-clean request directly

```text
atoms:           a  b  c  d  e  f  g
existing root:   [ab][cd][ef][g]
request Q:             c  d  e  f
```

How to solve it:

1. Tight-root check:
  no child token in the root already covers `cdef`, so the root is the right
  merge target.
2. Witness collection:
  the single witness is `[cd][ef]`.
3. Boundary classification:
  left edge `b|c` is clean and right edge `f|g` is clean.
4. Representation-preservation check:
  the direct update does not orphan any previously represented token.
5. Root action:
  choose a direct root update because no wrapper is needed.
6. Child materialization:
  create or reuse `Q = cdef` with decomposition `[cd][ef]`.
7. Root update:

```text
add root pattern: [ab][Q][g]
Q exposes:        [cd][ef]
```

What this proves:

- clean-clean requests are the baseline direct case;
- no helper or inner token is needed when both edges are already aligned.

### E2 Solve complementary one-sided dirty witnesses with two wrappers

```text
atoms:           a  b  c  d  e  f  g  h
existing P0:     [ab][cdef][gh]
existing P1:     [abcde][fg][h]
request Q:             c  d  e  f  g
```

How to solve it:

1. Tight-root check:
  neither `P0` nor `P1` has a child token that already covers `cdefg`, so the
  root remains the tightest existing token.
2. Duplication-closure check:
  `P0` is legal because it exposes `[ab][cdef]`, not `[a][bcd]`, so it does not
  leave `abcd` un-tokenized while `abcde` exists.
  `P1` is legal because `[fg][h]` is already tokenized by `gh` in `P0` only as
  `gh`, not as `fgh`; there is no unreplaced exposed `fgh` witness.
3. Witness collection:
  `P0` contributes witness `[cdef][gh]` and `P1` contributes witness
  `[abcde][fg]`.
4. Boundary classification:
  `P0` is clean on the left and dirty on the right;
  `P1` is dirty on the left and clean on the right.
5. Representation-preservation check:
  the naive direct update `[ab][Q][h]` is illegal because it would make the
  previously represented tokens `abcde` and `gh` unreachable from the updated
  root.
6. Root action:
  create two wrappers, one for each clean-dirty witness, so the root keeps both
  previously represented sides reachable while still introducing `Q` legally.
7. Child materialization:

```text
create wrappers: W0, W1

root P0 = [ab][W0]
root P1 = [W1][h]

W0 exposes:
  W0P0 = [cdef][gh]
  W0P1 = [Q][h]

W1 exposes:
  W1P0 = [abcde][fg]
  W1P1 = [ab][Q]
```

What this proves:

- merge must inspect every root pattern that covers `Q`;
- complementary clean edges from different patterns may require multiple
  wrapper-backed root updates rather than one direct root update;
- wrapper choice is justified here by representation preservation, not by dirty
  cuts alone;
- the starting witness set itself must already satisfy duplication-replacement
  closure.

### E3 Solve a dirty-dirty request by rejecting a redundant wrapper

```text
atoms:           a  b  c  d  e  f  g  h
existing root:   [ab][cdef][gh]
request Q:                d  e  f  g
```

How to solve it:

1. Tight-root check:
  no child token already covers `defg`, so the root is still the correct merge
  target.
2. Duplication-closure check:
  the single root pattern is legal; it does not expose an adjacent child span
  that some other child token already contains as a proper subrange.
3. Witness collection:
  the witness is `[cdef][gh]`.
4. Boundary classification:
  both requested edges are dirty.
5. Representation-preservation check:
  keeping the old root pattern means the direct update can preserve prior tokens
  without extra wrapper help.
6. Wrapper test:
  candidate wrappers such as `cdefg` do not buy any useful outer-context
  adjacency at the root. They would only hide structure that the direct update
  can already expose.
7. Root action:
  reject the wrapper and split only what is needed for a direct root update.
8. Child materialization:

```text
Q exposes: [def][g]
```

9. Root update:

```text
add root pattern: [ab][c][Q][h]
```

10. Preservation check:
  keep the old root pattern `[ab][cdef][gh]` as a first-class decomposition so
  `cdef` and `gh` remain exposed.

What this proves:

- dirty-dirty does not imply wrapper-backed replacement;
- the correct question is whether the wrapper is beneficial, not whether one can
  be drawn.

### E4 Solve a one-sided dirty request with an inner partition inside `Q`

```text
atoms:           x  x  a  b  c  d  e  f  w
existing root:   [x][x][ab][cdef][w]
request Q:                   a  b  c  d
```

How to solve it:

1. Tight-root check:
  no child token in the existing root already covers `abcd`.
2. Duplication-closure check:
  the starting root is legal because no exposed adjacent child span is already a
  proper subrange of another existing child token without being tokenized.
3. Witness collection:
  the witness is `[ab][cdef]`.
4. Boundary classification:
  left edge is clean and right edge is dirty.
5. Representation-preservation check:
  the direct update remains legal because the old root pattern can stay and keep
  `ab` and `cdef` reachable.
6. Root action:
  choose a direct root update, because `[x][x][Q][ef][w]` already preserves the
  outer context. A wrapper such as `abcdef` is not beneficial at the root.
7. Inner materialization:
  create `I = cd` inside `Q`. The dirty right cut through `cdef` means `Q`
  cannot stop at `d` without materializing the repeated subrange `cd`.
8. Child materialization:

```text
Q exposes: [ab][I]
I = cd
```

9. Root update:

```text
add root pattern: [x][x][Q][ef][w]
```

10. Recursion rule:
  if `I` itself still crossed dirty child boundaries, the same reasoning would
  recurse below `Q`.

What this proves:

- inner partitions are not abstract helper labels;
- they are concrete child-pattern materialization obligations inside the updated
  token.

### E5 wrapper-beneficial case is still deferred

`M5` remains a real requirement, but this spec does not yet freeze a concrete
worked wrapper-beneficial example. The next example added here must satisfy all
of the following before it becomes authoritative:

- the root is still the tightest existing token covering `Q`;
- no root patterns duplicate a boundary position;
- the wrapper preserves or creates reusable outer-context adjacency that the
  direct root update cannot express without redundancy;
- the wrapper's internal decomposition set explains exactly why the wrapper is
  better than the direct alternative.

## Validation and documentation flow

- The shared [graph induction](spec:16c3ad95-451d-4c09-a118-ca90bcefed9a)
  spec owns the tight-root invariant, the one-clean-boundary-per-offset rule,
  and the wrapper-benefit constraint.
- This child spec owns the reviewed merge examples and the future requirement
  matrix used by implementation tickets.
- The sibling [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
  spec remains the place that freezes the stable decomposition shapes that the
  read pipeline should observe after merge succeeds.

Focused validation should land in `context-insert` before broader
`context-read` regression repair:

- preserve and extend the nearby interval coverage around
  `test_perfect_split_no_wrapper_offset` and
  `test_required_partitions_perfect_vs_unperfect`;
- add focused merge cases for `E1` through `E4` before broad overlap tests;
- only then re-run `complex_abcabababcaba` and the wider overlap corpus.

Relevant implementation tickets should link here before code changes so the
same reviewed examples and validation anchors drive the design discussion.

## Boundary

This spec requires:

- largest-postfix overlap selection;
- structural complement completion from postfix paths;
- immediate materialization of each committed step before the next search;
- accumulation of compatible equal-span decompositions over time.

This spec does not require:

- a canonical child-pattern order;
- flat-root or last-child mutation as part of the algorithmic contract;
- full substring closure;
- every lower-level path surface to normalize embedded coverage immediately.