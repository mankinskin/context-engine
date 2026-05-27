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

## Implementation design

The split/join layer should represent overlap replacement with an explicit plan
that separates the requested span from the wrapper span used for the actual
parent-pattern replacement.

```rust
struct RootReplacementPlan {
      requested_range: PartitionRange,
      replacement_range: PartitionRange,
      replacement_mode: ReplacementMode,
      left_boundary: BoundaryWitness,
      right_boundary: BoundaryWitness,
      interior_witnesses: Vec<BoundaryWitness>,
      helper_ranges: Vec<HelperRange>,
}

enum ReplacementMode {
      Direct,
      WrapLeft,
      WrapRight,
      WrapBoth,
}

enum BoundaryClass {
      Clean,
      Dirty,
}

struct BoundaryWitness {
      offset_index: usize,
      class: BoundaryClass,
}

struct HelperRange {
      range: PartitionRange,
      role: HelperRole,
      authority: BoundaryAuthority,
}

enum HelperRole {
      Wrapper,
      InnerPrefix,
      InnerSuffix,
      Overlap,
}

enum BoundaryAuthority {
      Authoritative,
      HelperOnly,
}
```

Implementation obligations:

- `requested_range` is the exact span the algorithm is trying to materialize as
   a dedicated token.
- `replacement_range` is the aligned wrapper span that is legally spliced into
   the parent or root pattern. In the current code, this is the role played by
   the merge operating range.
- If `requested_range == replacement_range`, the replacement is direct and the
   requested token is the same token that the parent or root splices.
- If `requested_range != replacement_range`, the parent or root must splice the
   `replacement_range` token, while the `requested_range` token must appear in
   the wrapper token's `child_patterns` as a first-class decomposition.
- `helper_ranges` may be materialized so the wrapper token can expose the
   requested token and any compatible peer decompositions, but `HelperOnly`
   ranges must not become authoritative new clean split boundaries.
- Only outer wrapper edges and explicit clean witnesses may authorize
   replacement boundaries or emitted split positions.

### Scenario-to-plan matrix

| Scenario | `requested_range` | `replacement_range` | `helper_ranges` | Root or parent splice | Required stored decomposition |
| --- | --- | --- | --- | --- | --- |
| D1 clean-clean | exact requested span | same as requested | none beyond direct subranges | splice requested token directly | requested token may expose any already-valid clean splits |
| D2 dirty-left clean-right | dirty-cut requested span | left-expanded wrapper | left wrapper + induced overlaps | splice wrapper token | wrapper token must expose `[left_dirty_complement, requested]` or an equivalent first-class decomposition |
| D3 clean-left dirty-right | dirty-cut requested span | right-expanded wrapper | right wrapper + induced overlaps | splice wrapper token | wrapper token must expose `[requested, right_dirty_complement]` or an equivalent first-class decomposition |
| D4 dirty-dirty with interior clean witness | dirty-cut requested span | outer clean wrapper | both-side wrapper helpers plus interior witness-guided helpers | splice wrapper token | wrapper token must expose the requested token without turning the requested dirty cuts into clean boundaries |
| D5 dirty-dirty without interior clean witness | dirty-cut requested span | outer dirty-driven wrapper that still lands on clean outer edges | wrapper + inner + overlap helpers | splice wrapper token | requested token exists only through wrapper decompositions and helper-supported merges |
| D6 mixed clean and dirty peers | equal-span token under review | wrapper chosen from the unique clean witness | helpers for dirty peers remain helper-only | splice wrapper token or direct token depending on witness location | preserve dirty peer decompositions alongside the clean-guided replacement |
| D7 later reread upgrades prior dirty case | existing requested token | existing wrapper or a tighter clean wrapper | only the helpers needed for the new peer path | reuse existing equal-span token | add the new peer decomposition instead of creating a duplicate token |

### Required code alignment

- `IntervalGraph.target_range` already plays the role of `requested_range`.
- The root merge operating range currently acts as `replacement_range`, but that
   role is implicit and should become explicit in the planning data.
- `RequiredPartitions` currently mixes two concerns: ranges that must be
   materialized and cuts that are authoritative for replacement. The
   implementation should either replace it with a richer replacement plan or add
   explicit authority metadata so helper-only ranges do not masquerade as clean
   replacement boundaries.
- `add_root_pattern` should continue to splice the replacement-range token into
   the root, while target tracking should continue to return the requested-range
   token separately.

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