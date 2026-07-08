<!-- aligned-structure:v1 -->

# Summary

This branch captures the most confident specification material currently available for the context engine.

## Behavior Story

This branch captures the most confident specification material currently available for the context engine.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# context-stack

This branch captures the most confident specification material currently
available for the context engine.

## Layered structure

The stack is specified in this order:

1. `context-trace` defines graph, token, pattern, and tracing primitives.
2. `context-search` defines matching and traversal over existing graph
   structure.
3. `context-insert` performs structural mutation through split and join.
4. `context-read` turns token streams or text into graph structure through
   segmentation, expansion, overlap handling, and root accumulation.
5. `context-api` exposes workspace-scoped commands over those lower layers.

## Current spec hierarchy

The first populated child area is [graph induction](spec:16c3ad95-451d-4c09-a118-ca90bcefed9a),
which currently contains:

- [insert-first-match](spec:92112188-8acd-4573-bb22-f784fcf371ca)
- [insert-sequence](spec:e631e914-1840-4fec-9df5-b50a85c6cf00)
- [insert-sequences](spec:bbd92962-33f4-4b9e-b301-f4ce9909c135)
- [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de)
  - [context-read pipeline](spec:e0913182-7a5e-4c8f-a750-799afd58baae)
  - [induced graph structure](spec:904871fa-0b97-4484-9540-f2926e32476f)

The hierarchy is deliberate:

- public command contracts stay near the top of the tree;
- shared invariants live at the [graph induction](spec:16c3ad95-451d-4c09-a118-ca90bcefed9a) node;
- `context-read` algorithm details that matter to [read-sequence](spec:7fd5639f-a62b-4eb4-abe2-215c4bb2d0de) live as child
  specs beneath that command, not as unrelated sibling specs.

## Specification policy

This branch is intentionally conservative. It records behavior that is supported
by the current source, current tests, and a small amount of narrowly framed
design intent where the implementation is still in transition.

Where older documentation conflicts with the live code or regression corpus, the
spec tree prefers the live implementation and calls out the conflict explicitly
instead of freezing stale behavior.

Public specs own boundary-level promises such as input validation, return
shapes, and externally visible invariants. Internal child specs own the current
algorithm description and must state clearly when a detail is still draft rather
than frozen contract.
