# graph induction

Graph induction is the part of the context stack that accepts token sequences or
text and returns graph vertices representing that material.

Across the currently specified command surfaces, the shared invariants are:

- induction is structural rather than textual concatenation only; the engine may
  reuse existing subpatterns and decompositions;
- text-driven induction may create missing atoms, while token-ref induction starts
  from already resolved tokens;
- successful induction must preserve graph validity;
- repeated induction of related material is allowed to tighten or reuse existing
  graph structure instead of duplicating it blindly.

## Confidence boundary

This spec freezes the shared command-level invariants above.

It does not yet freeze every internal overlap-collapse choice in `context-read`.
That area is still under active design and some of the lower-level `context-read`
tests linked from this spec are better treated as intended regression targets than
as a fully stabilized implementation contract.

## Sources consulted

The current wording was derived from these repository sources:

- old doc-viewer output for `context-insert` root and `insert` module
- old doc-viewer output for `context-read` root and `expansion` module
- `crates/context-insert/agents/docs/README.md`
- `crates/context-read/agents/docs/README.md`
- `agents/guides/20260314_CONTEXT_API_INSERT_SEMANTICS_GUIDE.md`
- `crates/context-read/agents/designs/20260315_DESIGN_COMPLEMENT_PATH_BUILDING.md`

The insert semantics guide is partially stale relative to the live code. In
particular, it still describes `insert_sequence` as a minimum-two-character
operation and describes the API as a direct forward to `context-insert` only.
The child specs below instead record the current command behavior enforced by the
present `context-api` implementation and tests.