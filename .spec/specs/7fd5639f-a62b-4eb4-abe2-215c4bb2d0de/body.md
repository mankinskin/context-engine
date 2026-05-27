# read_sequence

`WorkspaceManager::read_sequence` reads text through the graph and returns one
deterministic projection of the root token induced by that read.

See also the parent [graph induction](spec:16c3ad95-451d-4c09-a118-ca90bcefed9a)
spec.

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
  not proof that the graph has only one decomposition.
- Repeated or related reads may tighten earlier roots, materialize reusable
  overlap tokens, or add alternate decompositions instead of duplicating
  structure.
- The returned tree is deterministic for a fixed graph snapshot, but that order
  is operational only. It is never semantic or canonical.

## Semantic model

Multi-character reads proceed as a sequence of committed blocks.

```text
input suffix
    |
    v
t_block_0 -> t_block_1 -> ... -> t_block_n
              ^
              |
  each step follows the current block's longest postfix path(s)
  and chooses the first postfix on those paths that can extend rightward
```

At each step the implementation may materialize new tokens and new compatible
decompositions for an already-known span. The public API returns one
deterministic tree over that induced graph.

## Spec hierarchy

This public command spec is refined by two child specs:

- [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
  defines the block/postfix/overlap algorithm and the worked traces.
- [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
  defines the graph facts that must hold after those reads complete.

## Worked trace families

The child specs cover the current read corpus:

- `heldld -> hell`
- `aabb -> aabbaabb`
- `xyyxy`
- `abcde -> bcdea -> cdeab -> deabc`
- `subdivision -> visualization` and `subvisu -> visub`
- `abcabababcaba`

These traces are normative examples of the read progression. They do not define
any canonical ordering over `child_patterns`.

## Boundary

This spec freezes the command boundary, error handling, return shape, and the
fact that `PatternReadResult` is a deterministic projection over the induced
graph.

It does not freeze the exact order in which lower-level traversal surfaces
enumerate peer decompositions, nor require every lower-level path surface to
normalize embedded coverage immediately.
