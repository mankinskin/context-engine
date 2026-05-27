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

## Boundary classification invariant

At the split/join layer, a queried atom offset may be a direct child boundary in
at most one stored pattern. Two peer patterns must not both claim the same atom
position as their own boundary.

This gives the induction stack one important decision rule:

- a queried offset has zero or one clean split witness;
- a clean split is the already-aligned case (`inner_offset = None`, the current
  "perfect split" terminology in `context-insert`);
- a dirty split is the child-interior case (`inner_offset = Some(...)`, the
  current "unperfect split" terminology in `context-insert`);
- wrapper and inner partitions exist to reconcile dirty cuts, not to mint extra
  clean boundaries.

### Boundary requirement matrix

| ID | Queried split state | Clean split available? | Replacement-range consequence | Required behavior |
| --- | --- | --- | --- | --- |
| G-B1 | No pattern exposes the queried atom offset as a direct child boundary | No | A replacement cannot anchor directly on that cut | Do not synthesize a clean split; only use surrounding dirty coverage if a higher layer still needs the span |
| G-B2 | Exactly one pattern exposes the queried offset as a clean split | Yes, unique | The replacement can reuse the aligned target cut directly | Treat the target range as authoritative and skip wrapper growth at that boundary |
| G-B3 | Exactly one pattern exposes the queried offset, but only as a dirty split | No | The replacement must extend to an aligned wrapper boundary before it can reuse structure safely | Materialize wrapper and any induced inner or overlap partitions, but keep the queried cut dirty |
| G-B4 | Multiple patterns cross the queried offset, but none exposes it as a direct boundary | No | Several dirty witnesses may justify the span, but there is still no clean anchor for replacement | Merge dirty evidence without inventing duplicate clean boundaries |
| G-B5 | Replacement-range selection discovers one clean cut inside the operating region | Yes, but only for that one cut | The clean cut can shorten or stabilize the replacement range on that side | Prefer the clean cut when choosing replacement extent; only dirty sides need wrapper handling |
| G-B6 | Replacement-range selection finds no clean cut in the operating region | No | Replacement extent is driven entirely by dirty wrapper coverage | Preserve ordered leaf coverage and compatible decompositions without promoting wrapper hints into new clean boundaries |

## Replacement scenario matrix

The split/join layer must distinguish the span the algorithm wants to
materialize from the aligned span it may need to replace in the parent pattern.

| Scenario | Left boundary | Right boundary | Clean witness availability | Replacement scope | Required graph outcome |
| --- | --- | --- | --- | --- | --- |
| R1 | Clean | Clean | Both requested edges are clean | Requested range itself | Reuse or create the requested token directly, and allow the parent or root to splice that token without wrapper growth |
| R2 | Dirty | Clean | Only the right edge is clean | Extend leftward to the smallest clean wrapper edge | Replace the parent or root at wrapper scope, and require the wrapper token to carry the requested token plus the dirty-left complement as a first-class decomposition |
| R3 | Clean | Dirty | Only the left edge is clean | Extend rightward to the smallest clean wrapper edge | Replace the parent or root at wrapper scope, and require the wrapper token to carry the requested token plus the dirty-right complement as a first-class decomposition |
| R4 | Dirty | Dirty | A single clean witness exists only inside the wider operating region | Extend to the smallest wrapper range whose outer edges are clean | Splice the wrapper token into the parent or root, use the interior clean witness only to stabilize helper ranges, and keep the requested edges dirty unless they themselves are clean |
| R5 | Dirty | Dirty | No clean witness exists inside the needed operating region | Extend by dirty coverage alone until a wrapper range can be expressed at clean outer boundaries | Splice only the wrapper token into the parent or root, and represent the requested token exclusively through wrapper decompositions and helper ranges |
| R6 | Mixed peers | Mixed peers | One peer offers a unique clean witness while other peers remain dirty | Use the clean witness only to choose wrapper extent or legal replacement edge | Preserve the dirty peer decompositions inside the equal-span token instead of discarding them or upgrading their dirty cuts into new clean boundaries |
| R7 | Earlier commit was dirty-only; a later reread discovers a clean witness | Later clean | The later read may reduce wrapper dependence | Reuse the same requested and wrapper spans | Add the tighter peer decomposition to the existing token set rather than duplicating the equal-span token |

These scenarios impose one shared rule: if the requested range is dirty at the
replacement edge, the parent pattern is updated through a clean wrapper range,
while the requested range remains represented as a first-class decomposition of
that wrapper token.

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
- `context-stack/context-insert/agents/docs/README.md`
- `context-stack/context-read/agents/docs/README.md`
- `agents/guides/20260314_CONTEXT_API_INSERT_SEMANTICS_GUIDE.md`
- `context-stack/context-read/agents/designs/20260315_DESIGN_COMPLEMENT_PATH_BUILDING.md`

The insert semantics guide is partially stale relative to the live code. In
particular, it still describes `insert_sequence` as a minimum-two-character
operation and describes the API as a direct forward to `context-insert` only.
The child specs under this node instead record the current behavior enforced by
the present `context-api` implementation and tests.