# context-stack/graph-induction/read-sequence/induced-graph-structure

This internal child spec records the graph facts that must hold after the
`context-read` algorithm completes. It focuses on the induced graph, not on the
API wrapper that chooses one child pattern for `PatternReadResult.tree`.

See also the public [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
spec and the sibling [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
spec.

## Core guarantees

- The root token returned by a successful read has width equal to the number of
  input characters or tokens consumed.
- Every stored child pattern preserves ordered concatenation of the underlying
  leaf atoms.
- Each matched span corresponds to at most one token.
- A token may hold multiple first-class decompositions in `child_patterns`.
- `child_patterns` are semantically unordered. Any order used by search,
  traversal, or projection code is operational only.
- Later related reads may tighten earlier roots and may add alternate
  decompositions to an already-known equal span.
- The graph is not required to contain every substring of the input. It only
  needs the tokens induced by the algorithm and any tighter decompositions
  learned from later related reads.

## Structural rule

When a read chooses postfix `P` inside block `B` and extends it to a wider token
`O`, the next block `B'` must be able to store both compatible decompositions.

```text
B  = [C_left,  P]
O  = [P,       C_right]

B' has peer decompositions:
|- [B,      C_right]
`- [C_left, O]
```

Those peer decompositions are first-class graph facts. One does not canonically
replace the other.

## Representative families

### Linear input

For inputs with no reusable structure, the induced root is a flat atom chain.

### Later rereads can refine earlier roots

`hypergraph` may first exist as a flat pattern and later as `[[hyper, graph]]`.
The same refinement rule applies to `heldld` and `hell`.

### Repeated blocks and adjacent overlap

`abcabcabc` stores both `[abcabc, abc]` and `[abc, abcabc]` because both are
valid decompositions of the same span.

### Infix rereads and rotating overlaps

Later reads may add reusable infix tokens and several compatible decompositions
across related roots.

```text
bcdea  => [bcde, a], [b, cdea], [bc, dea]
cdeab  => [cde, ab], [cdea, b]
deabc  => [de, abc], [dea, bc], [deab, c]
```

The key guarantee is accumulation: once later reads materialize `bc` and `dea`,
the already-known span `bcdea` must gain `[bc, dea]` as another valid
decomposition.

### Shared roots across related rereads

`subdivision`, `visualization`, `subvisu`, and `visub` may share tokens such as
`su`, `vi`, `vis`, `visu`, `ion`, and `sub`, and equal-span roots may retain
multiple peer decompositions.

### Larger overlap families

`abcabababcaba` freezes the same rule at larger scale: the stable point is the
set of valid tokens and peer decompositions, not a traversal order.

## Relation to `PatternReadResult`

The public `read_sequence` command returns one deterministic projection of the
induced graph. `build_read_tree` may therefore show fewer decompositions than
the graph actually stores.

## Boundary

This spec does not require:

- a canonical ordering over `child_patterns`;
- full substring or ngram closure;
- every intermediate token to be exposed on every API surface;
- a specific moment during a multi-step read when a lower-level helper must
  become externally visible.