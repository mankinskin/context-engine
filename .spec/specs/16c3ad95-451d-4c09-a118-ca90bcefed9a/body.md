# graph induction

Graph induction is the part of the context stack that accepts token sequences or
text and returns graph vertices representing that material.

## Hierarchy within this branch

This node owns the shared invariants for four child command specs:

- [insert-first-match](spec:92112188-8acd-4573-bb22-f784fcf371ca) for
  token-ref induction through already-resolved tokens;
- [insert-sequence](spec:e631e914-1840-4fec-9df5-b50a85c6cf00) for text
  induction returning `InsertResult`;
- [insert-sequences](spec:bbd92962-33f4-4b9e-b301-f4ce9909c135) for
  bulk text induction;
- [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de) for text
  reads that return a deterministic decomposition view.

[read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de) then refines
its own implementation detail through two child specs:

- [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
  for segmentation, expansion, overlap handling, and root accumulation;
- [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)
  for the graph-shape guarantees already fixed by the `context-read` regression
  corpus.

## Shared invariants

Across the currently specified command surfaces:

- induction is structural rather than textual concatenation only; the engine may
  reuse existing subpatterns and decompositions;
- text-driven induction may create missing atoms, while token-ref induction
  starts from already resolved tokens;
- successful induction preserves graph validity and ordered leaf coverage;
- repeated induction of related material may tighten existing roots or add
  compatible alternate decompositions instead of duplicating structure blindly;
- the public command specs own input-validation and result-shape promises, while
  the `context-read` child specs under [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de) own the algorithmic detail beneath
  multi-character reads.

## Layer responsibilities

- `context-api` validates command inputs and shapes returned result values.
- `context-read` orchestrates segmentation, known-block expansion, overlap
  handling, and root accumulation.
- `context-insert`, `context-search`, and `context-trace` provide the
  mutation, search, and graph primitives that make those reads possible.

## Confidence boundary

This spec freezes the shared invariants and the tree structure above.

It does not freeze every internal overlap-collapse choice or every lower-layer
defect currently tracked in `context-search` and `context-trace` tickets. Those
boundaries are called out more precisely in the [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
child specs.

## Sources consulted

The current wording was derived from these repository sources:

- old doc-viewer output for `context-insert` root and `insert` module
- old doc-viewer output for `context-read` root and `expansion` module
- `crates/context-stack/context-insert/agents/docs/README.md`
- `crates/context-stack/context-read/agents/docs/README.md`
- `agents/guides/20260314_CONTEXT_API_INSERT_SEMANTICS_GUIDE.md`
- `crates/context-stack/context-read/agents/designs/20260315_DESIGN_COMPLEMENT_PATH_BUILDING.md`

The insert semantics guide is partially stale relative to the live code. In
particular, it still describes `insert_sequence` as a minimum-two-character
operation and describes the API as a direct forward to `context-insert` only.
The child specs under this node instead record the current behavior enforced by
the present `context-api` implementation and tests.