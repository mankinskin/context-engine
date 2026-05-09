# context-stack

This branch captures the most confident specification material currently available
for the context engine.

The stack is layered in this order:

1. `context-trace` defines the graph, token, pattern, and tracing primitives.
2. `context-search` defines matching and traversal over existing graph structure.
3. `context-insert` performs structural mutation through split and join.
4. `context-read` turns token streams or text into graph structure through
   segmentation, expansion, and overlap handling.
5. `context-api` exposes workspace-scoped commands over those lower layers.

This spec branch is intentionally conservative. It records only behavior that is
well-supported by current source and tests, plus a small amount of narrowly framed
design intent where the implementation is still in transition.

## Current focus

The first populated child area is graph induction: commands that turn token
sequences or text into graph vertices while preserving graph validity and reusing
existing structure when possible.

## Source policy

The wording in this branch was derived from the current code, the old doc-viewer
surfaces for `context-insert` and `context-read`, and the markdown material already
present in the repository.

Where older documentation conflicts with current code and tests, the child specs
prefer the live implementation and call out the conflict explicitly instead of
freezing stale behavior.