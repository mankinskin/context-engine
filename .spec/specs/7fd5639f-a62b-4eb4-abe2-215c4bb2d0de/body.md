# read_sequence

`WorkspaceManager::read_sequence` reads text through the graph and returns a
deterministic projection of the resulting root token.

See also the parent [graph induction](spec:16c3ad95-451d-4c09-a118-ca90bcefed9a) spec.

## Public contract

- The empty string is rejected with `ReadError::SequenceTooShort { len: 0 }`.
- A single character is handled as a special case: the atom is ensured to exist
  and the returned `PatternReadResult` describes that atom as a leaf node.
- Multi-character input delegates to `context_read::pipeline::ReadCtx`, which is
  also the implementation behind the lower-level `context_read::read` entry
  point.
- On success, `PatternReadResult.root.width` equals the number of input
  characters and `PatternReadResult.text` equals the concatenated leaf text of
  the returned projection.
- The graph may store multiple first-class decompositions for the same token.
  `PatternReadResult` exposes one deterministic projection over that set; it is
  not a proof that the graph has only one decomposition.
- The returned `ReadNode` tree is deterministic for a fixed graph snapshot
  because the helper sorts child patterns by `PatternId` and then follows one
  path. That ordering is operational, not semantic or canonical.
- The command may enrich the graph while it reads. Repeated or related reads may
  tighten roots, materialize new overlap tokens, or add alternate
  decompositions instead of duplicating structure blindly.
- This public command may expose normalized facets of read results when that is
  useful for callers, even when lower-level path and cursor surfaces retain
  non-normalized embedded-path coverage for longer.

## Spec hierarchy

This public command spec is refined by two child specs:

- [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
  describes how `context-read` partitions the input into unknown and known
  segments and applies the largest-overlap incremental join rule.
- [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
  describes the graph-shape guarantees already fixed by the `context-read`
  regression corpus.

## Worked trace corpus

The child pipeline spec includes step-by-step worked traces for the current
clarified overlap model. The active corpus is:

- `heldld -> hell`
- `aabb -> aabbaabb`
- `xyyxy`
- `abcde -> bcdea -> cdeab -> deabc`
- `subdivision -> visualization` and `subvisu -> visub`
- `abcabababcaba`

Those traces document `root`, `anchor`, `flat_root`, overlap selection,
complement completion, and root commit behavior. They are normative examples of
the read progression, not a canonical ordering over `child_patterns`.

## Confidence boundary

This spec freezes the command boundary, error handling, return shape, and the
fact that `PatternReadResult` is a deterministic projection over the induced
graph.

It does not freeze the exact internal order in which lower-level traversal
surfaces enumerate alternative decompositions, nor every intermediate token that
may appear while `context-read` is still under algorithmic cleanup. Those
details live in the child specs and remain partially draft.
