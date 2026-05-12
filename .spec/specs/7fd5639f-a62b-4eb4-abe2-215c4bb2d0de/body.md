# read_sequence

`WorkspaceManager::read_sequence` reads text through the graph and returns a
deterministic decomposition view of the resulting root token.

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
  the chosen decomposition tree.
- The returned `ReadNode` tree is deterministic for a fixed graph snapshot
  because the helper walks the first child pattern after sorting by
  `PatternId`.
- The command may enrich the graph while it reads. Repeated or related reads may
  tighten existing roots or add alternate decompositions instead of duplicating
  structure blindly.

## Spec hierarchy

This public command spec is refined by two child specs:

- [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
  describes how `context-read` partitions the input into unknown and known
  segments and accumulates the root token.
- [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
  describes the graph-shape guarantees already fixed by the `context-read`
  regression corpus.

## Confidence boundary

This spec freezes the command boundary, error handling, return shape, and the
deterministic tree projection exposed through `PatternReadResult`.

It does not freeze the exact internal order of overlap discovery or every
intermediate token that may appear while `context-read` is still under
algorithmic cleanup. Those details live in the child specs and remain partially
draft.
