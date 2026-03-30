# [context-editor][SDF-DAG] Heterogeneous SDF Atom DAG Architecture — Epic

## Summary

Replace the current flat-material Sparse Voxel Octree (SVO) with a Directed Acyclic Graph (DAG)
where each leaf node references a typed **SDF Atom** from a content-addressed coefficient pool.
The three phases build sequentially and layer on top of the completed Tiled Forward+ rasterizer (T6d).

### Phase 1 — SDF Atom Type System (highest priority)

Replace the per-voxel `color_data: u32` with a typed atom reference. Atoms carry full SDF
coefficients (sphere, rounded box, torus, polynomial SDF, glyph-SDF, semi-transparent volume),
enabling smooth curved surfaces, sub-voxel detail, and alpha-composited transparency.
The tiled rasterizer dispatches to the correct SDF evaluator via a `switch` on atom type.

### Phase 2 — DAG-Persistent Edit Operations

Convert all edit operations from simple tree mutation into hash-consed DAG operations.
Every write maintains structural sharing — identical nodes are never duplicated.
Edit algorithms are "duplication resistant": they modify the DAG directly and the structure
is always in a valid, consistent state after each operation. No post-edit compaction step.

### Phase 3 — 4D Spatio-Temporal Compression with Replay

Add a temporal axis. The DAG shares subtrees across time slices for scenes with minimal motion.
SDF atoms carry deformation functions for keyframed morphing. Full-timeline replay is supported
by a stored delta edit log. Physics and animation drive the timeline; identical frames share
100% of their DAG nodes.

## Architecture Context

Current leaf node: `OctreeNode { child_pointer: u32, color_data: u32 }` (8 bytes)
After Phase 1: leaf carries `atom_ref: u32` (type_id[8] | pool_idx[24])
After Phase 2: nodes live in a content-addressed `NodePool` with hash consing
After Phase 3: SVO becomes `VoxelTimeline` with keyframes and edit log

## Dependencies

- Tiled Forward+ Rasterizer (T6d, ticket `194ade77`) — must complete first; new architecture builds on top.
- Phase 2 `depends_on` Phase 1 (atom identity defines DAG node equality)
- Phase 3 `depends_on` Phase 2 (temporal sharing requires DAG structural sharing infrastructure)

## Acceptance Criteria (overall epic)

1. Arbitrary-type SDF atoms (sphere, torus, rounded box, glyph, semi-transparent) render correctly
   through the existing Tiled Forward+ rasterizer.
2. Edit operations maintain DAG consistency at all times — no duplicate subtrees ever created.
3. Adjacent SDF atoms blend smoothly (smooth-min) without hard seams at voxel boundaries.
4. A time-varying scene compresses identically-shaped time-slices to near-zero delta storage.
5. Replay of a recorded session reproduces identical frame output from stored edit log.
6. No regression on Tiled Forward+ performance baseline (< 5ms at 1080p for 1M voxels).
