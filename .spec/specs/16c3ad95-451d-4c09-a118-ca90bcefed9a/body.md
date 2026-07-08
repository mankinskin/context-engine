<!-- aligned-structure:v1 -->

# Summary

Graph induction is the part of the context stack that accepts token sequences or text and returns graph vertices representing that material.

## Behavior Story

Graph induction is the part of the context stack that accepts token sequences or text and returns graph vertices representing that material.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

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

## Duplication replacement invariant

Across all first-class decompositions, an exposed adjacent child sequence must
not remain un-tokenized when the same atom span is already covered as a proper
subrange of another existing child token.

Concretely:

- if some pattern exposes adjacent children whose concatenation covers span `S`;
- and some other existing child token already covers `S` as a strict subrange of
  its own atom coverage;
- then `S` must itself already be represented by a dedicated token.

A witness set that leaves such a span un-tokenized is illegal. Merge must not
reason from it as if it were a valid starting point.

Examples:

- if `[a][bcd]` is exposed while token `abcde` already exists, then token
  `abcd` must already exist;
- if `[fg][h]` is exposed while token `efgh` already exists, then token `fgh`
  must already exist.

This is the ubiquitous form of the "replace duplication" rule. It applies
before wrapper choice, before inner-range planning, and before merge examples
are considered valid.

## Representation preservation invariant

When merge updates a token's decomposition set, it must preserve that token's
existing representation closure.

Concretely:

- if token `R` represented token `T` before the merge, whether directly as a
  child or indirectly through descendant decompositions;
- then after the merge, `R` must still represent `T` through some
  decomposition path.

Direct child status may change, but reachability from the updated root must not
be lost.

This means:

- merge must reject candidate root updates that would make a previously
  represented token unreachable from the root;
- replacing a direct child with a wrapper is only legal if that wrapper still
  exposes the displaced token somewhere in its own decomposition closure;
- preserving the requested token is not sufficient if other previously
  represented tokens, such as `abcde` or `gh`, become unreachable.

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

`context-search` hands merge the smallest existing root token that still covers
the requested atom range. A valid merge step therefore must always update the
root decomposition set. If a merge would leave the root unchanged, the search
result was not actually the tightest existing cover.

The split/join layer must reason per root child pattern. Each root child
pattern whose atom coverage contains the requested range contributes its own
pattern-local witness, and the merge may need to add more than one root-level
decomposition to account for all legal witnesses.

| Scenario | Pattern-local witness set | Root-level consequence | Required graph outcome |
| --- | --- | --- | --- |
| R1 | One clean-clean witness | Direct root update | Add a root pattern that places the requested token next to the surviving outer context; the requested token exposes the clean subrange decomposition |
| R2 | One one-sided dirty witness | Direct root update unless a wrapper is provably more useful | Materialize the dirty-side helper or inner partitions inside the requested or wrapper token, and preserve the surviving outer context at the root |
| R3 | One dirty-dirty witness | Prefer direct root update unless a wrapper adds beneficial reusable adjacency | Keep the surviving outer context visible and do not introduce a wrapper that only mirrors the direct result |
| R4 | Multiple root patterns with disjoint clean boundaries | Consider all witnesses, not just the first pattern | The resulting decomposition set must preserve the compatible per-pattern witnesses induced by each root pattern and must already satisfy duplication-replacement closure |
| R5 | Dirty cuts strictly inside the requested or wrapper range | Inner materialization required | Inner partitions become first-class decomposition members of the requested or wrapper token, may recurse downward, and must not masquerade as new authoritative root boundaries |
| R6 | An exposed witness would leave a repeated subrange un-tokenized while another token already contains it | Invalid witness input to merge | Materialize the repeated subrange first or reject the witness set as illegal; merge must not build on unreplaced duplication |
| R7 | A candidate root update would orphan previously represented tokens | Invalid merge output | Add the wrapper-backed or multi-pattern update needed to keep those tokens root-reachable |
| R8 | Merge would not change the root | Invalid search input to merge | Search should have returned a tighter token instead; merge must not be a no-op at the root |

The reviewed merge examples and the focused validation anchors for these cases
live in the child [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
spec.

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
