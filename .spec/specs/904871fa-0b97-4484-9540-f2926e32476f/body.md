# context-stack/graph-induction/read-sequence/induced-graph-structure

This internal child spec records the graph-shape guarantees that are already
enforced by the `context-read` regression corpus. It focuses on the graph that
is induced by reads, not on the API wrapper that turns one child pattern into a
`PatternReadResult.tree`.

See also the public [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
spec and the sibling [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
spec.

## Core guarantees

- The root token returned by a successful read has width equal to the number of
  input characters or input tokens consumed.
- Every stored child pattern preserves ordered concatenation of the underlying
  leaf atoms.
- Reads may reuse existing tokens, add tighter decompositions to existing
  tokens, or add alternate decompositions when the same span can be represented
  multiple valid ways.
- The graph is not required to contain every substring of the input. It
  contains the tokens reached by the current segmentation and overlap-discovery
  algorithm plus any tighter decompositions learned from later related reads.

## Representative cases

### Linear input

For inputs with no useful repeated structure, the induced root is a flat atom
chain. `linear_read_abc` and the related linear tests freeze this behavior.

### Later rereads can refine earlier roots

The graph is allowed to become tighter over time. `read_sequence1` shows
`hypergraph` first stored as a flat pattern and later represented as
`[[hyper, graph]]` after `hyper` and `graph` are read later. `read_sequence2`
shows the same refinement pattern for repeated blocks such as `heldld` and
`hell`.

### Repeated blocks and adjacent overlap

`repetition_abcabcabc` freezes the minimal reusable structure for a triple
repeat:

- `abc` exists as `[a, b, c]`;
- `abcabc` exists as `[abc, abc]`;
- the root stores both valid adjacent decompositions `[abcabc, abc]` and
  `[abc, abcabc]`.

### Infix rereads and rotating overlaps

`read_infix1` and `read_multiple_overlaps1` show that later reads can add
reusable infix tokens and several compatible decompositions across related
roots. The graph may therefore hold more than one child pattern for the same
token when each pattern is a valid ordered decomposition of the same span.

### Complex overlap corpus

`complex_abcabababcaba` records a larger overlap family derived from the
repository's ngrams-oracle work. The stable point is not a specific search
order. It is the set of valid decompositions that must exist once the graph has
processed that input.

## Relation to `PatternReadResult`

The public `read_sequence` command returns `PatternReadResult` and `ReadNode`,
but that tree is only one deterministic projection of the induced graph.
`build_read_tree` sorts child patterns by `PatternId` and follows the first one,
so the returned tree may show fewer decompositions than the graph actually
stores.

## Draft boundary

This spec does not freeze:

- the exact moment during a multi-step read when each alternate decomposition is
  materialized;
- behavior currently identified as defects in active lower-layer tickets, such
  as repeated-single-character width mismatches;
- a requirement that the induced graph match a full ngrams closure rather than
  the current regression corpus.