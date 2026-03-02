# Graph Operation Events Guide

**Date:** 2026-03-01 (updated 2026-03-02)
**Tags:** visualization, events, transitions, CSS, search, insert, read

Complete reference for `GraphOpEvent` transitions, the data they carry, when they fire, and how the frontend maps them to visual styles.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Event Structure](#event-structure)
3. [Transition Reference](#transition-reference)
   - [Search-Specific Transitions](#search-specific-transitions)
   - [Insert-Specific Transitions](#insert-specific-transitions)
4. [LocationInfo Mapping](#locationinfo-mapping)
5. [QueryInfo Fields](#queryinfo-fields)
6. [GraphMutation & DeltaOp](#graphmutation--deltaop)
7. [CSS Class Mapping](#css-class-mapping)
   - [Node Classes](#node-classes)
   - [Search Path Classes](#search-path-classes)
   - [Insert Operation Classes](#insert-operation-classes)
8. [Edge Color Mapping](#edge-color-mapping)
9. [Phase Badge Colors](#phase-badge-colors)
10. [Context-Read Event Schema (Design)](#context-read-event-schema-design)

---

## Architecture Overview

```
┌──────────────────┐    tracing::info!     ┌───────────┐    JSON parse     ┌──────────────────┐
│  context-search  │ ──── graph_op ──────▷ │ log file  │ ────────────────▷ │ log-viewer UI    │
│  context-insert  │                       │ (.jsonl)  │                   │ (HypergraphView) │
└──────────────────┘                       └───────────┘                   └──────────────────┘
```

1. **Rust crates** construct `GraphOpEvent` with a `Transition` variant, `LocationInfo`, `QueryInfo`, optional `GraphMutation`, and a `VizPathGraph` snapshot.
2. `GraphOpEvent::emit()` serializes to JSON and writes a `tracing::info!` entry. The **message** is the human-readable `description` field; the `graph_op` field contains the serialized event payload.
3. **log-viewer frontend** detects events by the **presence of the `graph_op` field** (not by message text), parses it, and feeds it into `useVisualizationState` to derive node/edge roles, which drive CSS classes and WebGPU overlay edge colors.

### Path ID Format

Events are scoped by `path_id` — a namespaced identifier:

```
<op_type>/<module>/<semantic_id>
```

Examples:
- `search/context-search/token-42-1234567890`
- `insert/context-insert/seq-7-1234567890`

Use `parse_path_id()` to decompose, `OperationType::from_path_id()` to infer the operation type.

---

## Event Structure

```rust
pub struct GraphOpEvent {
    pub step: usize,              // Monotonic counter per operation
    pub op_type: OperationType,   // Search | Insert | Read
    pub transition: Transition,   // What happened (see below)
    pub location: LocationInfo,   // Styling hints for the frontend
    pub query: QueryInfo,         // Search pattern context
    pub description: String,      // Human-readable summary (also the log message)
    pub path_id: String,          // Operation scope identifier
    pub path_graph: VizPathGraph, // Full path snapshot after this step
    pub graph_mutation: Option<GraphMutation>, // Graph mutations (insert only)
}
```

Builder pattern:
```rust
GraphOpEvent::search(step, path_id, transition, path_graph, description)
    .with_location(LocationInfo::selected(node).with_root(root))
    .with_query(QueryInfo::new(tokens, cursor, width))
    .emit();
```

---

## Transition Reference

All transitions are categorized by the crate that emits them.

### Search-Specific Transitions

These transitions are only emitted by `context-search`.

#### `StartNode`

| Field | Type | Description |
|-------|------|-------------|
| `node` | `usize` | Token index the operation starts at |
| `width` | `usize` | Atom count of the start token |

**When fired:** First event of a search operation. Marks the entry point for upward/downward traversal.

**Emitted by:** `context-search` — when search begins at the initial candidate token.

**Frontend effect:** Node gets `viz-start` class (bright cyan, pulsing glow). Sets `startNode` in viz state.

---

#### `VisitParent`

| Field | Type | Description |
|-------|------|-------------|
| `from` | `usize` | Node we're ascending from |
| `to` | `usize` | Parent node being explored |
| `entry_pos` | `usize` | Position within parent where `from` appears |
| `width` | `usize` | Width (atom count) of the parent node |
| `edge` | `EdgeRef` | Edge connecting `from → to` in the snapshot |

**When fired:** During bottom-up traversal when exploring a parent candidate. The search walks upward from the start token to find the longest matching prefix.

**Emitted by:** `context-search` — when visiting a parent candidate from the BFS queue.

**Frontend effect:** `to` node gets `viz-candidate-parent` class (orange with pulse). Edge from→to colored as candidate edge (muted violet, 30% alpha). The parent is pushed onto `start_path`.

---

#### `VisitChild`

| Field | Type | Description |
|-------|------|-------------|
| `from` | `usize` | Parent node we're descending from |
| `to` | `usize` | Child node being explored |
| `child_index` | `usize` | Index within parent's pattern |
| `width` | `usize` | Width of the child node |
| `edge` | `EdgeRef` | Edge connecting `from → to` |
| `replace` | `bool` | Whether this replaces the end_path tail (vs. push) |

**When fired:** During top-down comparison when walking down children to verify the query matches.

**Emitted by:** `context-search` — when comparing children of a root node against the query pattern.

**Frontend effect:** `to` node gets `viz-candidate-child` class (purple with pulse). Edge from→to colored as candidate edge. Child pushed onto (or replaces tail of) `end_path`.

---

#### `ChildMatch`

| Field | Type | Description |
|-------|------|-------------|
| `node` | `usize` | The child node that matched |
| `cursor_pos` | `usize` | Atom position in the query where match occurred |

**When fired:** When a child token's content matches the expected query token at `cursor_pos`.

**Emitted by:** `context-search` — during child comparison in `process_child_comparison`.

**Frontend effect:** Node gets `viz-matched` class (green). `query.active_token` set to the compared node. `query.matched_positions` updated to include `cursor_pos`.

---

#### `ChildMismatch`

| Field | Type | Description |
|-------|------|-------------|
| `node` | `usize` | The child node that mismatched |
| `cursor_pos` | `usize` | Atom position where mismatch was detected |
| `expected` | `usize` | Token index that was expected |
| `actual` | `usize` | Token index that was found |

**When fired:** When a child token's content does not match the expected query token.

**Emitted by:** `context-search` — during child comparison when tokens diverge.

**Frontend effect:** Node gets `viz-mismatched` class (red). `query.active_token` set to the compared node. Query panel shows mismatch indicator at `cursor_pos`.

---

#### `Done`

| Field | Type | Description |
|-------|------|-------------|
| `final_node` | `Option<usize>` | Result node if successful, None if not |
| `success` | `bool` | Whether the operation succeeded |

**When fired:** Terminal event. Operation completed (either found a match or exhausted all candidates).

**Emitted by:** `context-search` — when search completes (match found or queue empty).

**Frontend effect:** If `success`, `final_node` gets `viz-completed`. No pulsing animations.

---

#### `CandidateMismatch`

| Field | Type | Description |
|-------|------|-------------|
| `node` | `usize` | Node that was rejected |
| `queue_remaining` | `usize` | Items left in queue after rejection |
| `is_parent` | `bool` | Whether this was a parent (true) or child (false) candidate |

**When fired:** When a candidate is rejected after processing — `ProcessResult::Skipped` was returned, meaning the candidate did not produce a root match.

**Frontend effect:** Node gets `viz-selected` class. `pendingParents`/`pendingChildren` from `LocationInfo` show remaining queue. The rejected parent is popped from `start_path` (undoing the prior `VisitParent`).

---

#### `CandidateMatch`

| Field | Type | Description |
|-------|------|-------------|
| `root` | `usize` | Root node being explored |
| `width` | `usize` | Width of the root node |
| `edge` | `EdgeRef` | Edge from start_path top → root |

**When fired:** When a parent candidate becomes the new root — confirmed match. The search will now explore children downward from this root.

**Frontend effect:** `root` gets `viz-root` class (gold ring via `::before` pseudo-element). Edge colored gold (`SP_ROOT_EDGE_COLOR`). The root is "graduated" from `start_path` — popped off the top and set as the `root` node.

---

#### `ParentExplore`

| Field | Type | Description |
|-------|------|-------------|
| `current_root` | `usize` | Current root whose boundary was reached |
| `parent_candidates` | `Vec<usize>` | Parent nodes added to queue for further exploration |

**When fired:** When the search reaches the top of the current root and needs to explore further parents. The root boundary has been fully matched, but the query extends beyond it.

**Frontend effect:** `parent_candidates` nodes added to `pendingParents`. Overlay renderer tracks these as carried-forward candidates for subsequent steps.

---

### Insert-Specific Transitions

These are only emitted by `context-insert`.

#### `SplitStart`

| Field | Type | Description |
|-------|------|-------------|
| `node` | `usize` | Token being split |
| `split_position` | `usize` | Atom position where split occurs |

**When fired:** At the beginning of a split operation — a token needs to be broken into two fragments at `split_position`.

**Emitted by:** `context-insert/src/insert/context.rs`

**Frontend effect:** Node gets `viz-split-source` class (warm orange, pulsing via `viz-insert-pulse` animation).

---

#### `SplitComplete`

| Field | Type | Description |
|-------|------|-------------|
| `original_node` | `usize` | The original node that was split |
| `left_fragment` | `Option<usize>` | Left fragment (atoms before split point) |
| `right_fragment` | `Option<usize>` | Right fragment (atoms after split point) |

**When fired:** After split completes — the original token has been divided into left and right fragments.

**Emitted by:** `context-insert/src/insert/context.rs`

**Frontend effect:**
- `original_node` → `viz-split-source`
- `left_fragment` → `viz-split-left` (orange-left tint)
- `right_fragment` → `viz-split-right` (orange-right tint)
- Insert edge keys added: `original_node → left_fragment`, `original_node → right_fragment`
- Edges colored warm orange (`INSERT_EDGE_COLOR`)

**GraphMutation:** Typically carries `AddNode` for fragments and `RemoveNode` or `UpdateNode` for the original.

---

#### `JoinStart`

| Field | Type | Description |
|-------|------|-------------|
| `nodes` | `Vec<usize>` | Nodes being joined |

**When fired:** At the beginning of a join operation. Multiple fragments will be merged into a single token.

**Emitted by:** `context-insert/src/insert/context.rs`

**Frontend effect:** First node in `nodes` becomes the primary node. Other nodes added to `involvedNodes`.

---

#### `JoinStep`

| Field | Type | Description |
|-------|------|-------------|
| `left` | `usize` | Left input node |
| `right` | `usize` | Right input node |
| `result` | `usize` | Result node (may be new or reuse existing) |

**When fired:** Each pairwise merge within a join. May occur multiple times per join operation.

**Emitted by:** `context-insert/src/join/context/frontier.rs`

**Frontend effect:**
- `left` → `viz-join-left` (cool blue)
- `right` → `viz-join-right` (cool blue)
- `result` → `viz-join-result` (green, pulsing via `viz-join-glow` animation)
- Insert edge keys: `result → left`, `result → right`
- Edges to join-result colored green (`INSERT_JOIN_EDGE_COLOR`); others warm orange (`INSERT_EDGE_COLOR`)

---

#### `JoinComplete`

| Field | Type | Description |
|-------|------|-------------|
| `result_node` | `usize` | Final result of the join |

**When fired:** After all join steps complete — the final merged token is ready.

**Emitted by:** `context-insert/src/insert/context.rs`

**Frontend effect:** `result_node` → `viz-join-result` (green glow).

---

#### `CreatePattern`

| Field | Type | Description |
|-------|------|-------------|
| `parent` | `usize` | Token that owns the new pattern |
| `pattern_id` | `usize` | Pattern index within the parent |
| `children` | `Vec<usize>` | Child token indices in the pattern |

**When fired:** When a new pattern (ordered sequence of children) is added to a parent token.

**Emitted by:** `context-insert/src/join/context/node/merge/iter.rs`

**Frontend effect:**
- `parent` → `viz-new-pattern` (yellow glow)
- Each child → `viz-new-pattern-child` (lighter yellow)
- Insert edge keys: `parent → child` for each child
- Edges colored warm orange (`INSERT_EDGE_COLOR`)

---

#### `CreateRoot`

| Field | Type | Description |
|-------|------|-------------|
| `node` | `usize` | Newly created root token |
| `width` | `usize` | Width of the new root |

**When fired:** When a new top-level token (root) is created in the graph.

**Emitted by:** `context-insert` (during pattern construction)

**Frontend effect:** `node` → `viz-new-root` (bright white-gold, pulsing via `viz-insert-pulse` animation).

---

#### `UpdatePattern`

| Field | Type | Description |
|-------|------|-------------|
| `parent` | `usize` | Token whose pattern is being updated |
| `pattern_id` | `usize` | Index of the updated pattern |
| `old_children` | `Vec<usize>` | Previous child sequence |
| `new_children` | `Vec<usize>` | Updated child sequence |

**When fired:** When an existing pattern's children are modified (e.g. after a split replaces a child with fragments).

**Emitted by:** `context-insert`

**Frontend effect:**
- `parent` → `viz-new-pattern` (yellow)
- Insert edge keys: `parent → child` for each new child
- Edges colored warm orange

---

## LocationInfo Mapping

`LocationInfo` provides styling hints independent of the transition variant. The frontend maps these fields to roles:

| Field | Viz State Property | CSS Class | Visual |
|-------|--------------------|-----------|--------|
| `selected_node` | `selectedNode` | `viz-selected` | White highlight, soft glow |
| `root_node` | `rootNode` | `viz-root` | Gold border ring (`::before` pseudo) |
| `trace_path[]` | `tracePath` | `viz-path` | Blue breadcrumb trail |
| `completed_nodes[]` | `completedNodes` | `viz-completed` | Green confirmed |
| `pending_parents[]` | `pendingParents` | `viz-pending-parent` | Dim orange |
| `pending_children[]` | `pendingChildren` | `viz-pending-child` | Dim purple |

**Priority:** Transition-derived roles (e.g. `viz-start`, `viz-candidate-parent`) take precedence over LocationInfo-derived roles. The `getNodeVizClasses` function applies them in order; CSS specificity handles overlaps.

**Dimming:** When any viz state is active (`hasVizState == true`), nodes not in `involvedNodes` get `viz-dimmed` (opacity 0.2, desaturated).

---

## QueryInfo Fields

| Field | Type | Description | Frontend Use |
|-------|------|-------------|--------------|
| `query_tokens` | `Vec<usize>` | Token indices comprising the input pattern | Nodes get `viz-query-token` (dashed purple border) |
| `cursor_position` | `usize` | Current atom index being matched | QueryPathPanel progress bar |
| `query_width` | `usize` | Total atoms in query | QueryPathPanel total width |
| `matched_positions` | `Vec<usize>` | Atom positions confirmed as matched | QueryPathPanel green highlights |
| `active_token` | `Option<usize>` | Graph node just compared | Gets `viz-query-active` (gold border + glow) |

---

## GraphMutation & DeltaOp

`GraphMutation` is populated only for insert operations. Contains an ordered list of `DeltaOp` mutations:

| DeltaOp Variant | Fields | Description |
|-----------------|--------|-------------|
| `AddNode` | `index`, `width` | New token created |
| `RemoveNode` | `index` | Token removed |
| `AddEdge` | `from`, `to`, `pattern_id` | New parent→child edge |
| `RemoveEdge` | `from`, `to`, `pattern_id` | Edge removed |
| `UpdateNode` | `index`, `detail` | Node data changed (detail is human-readable) |

**Frontend:** `InsertStatePanel` shows before/after diffs. Node indices in mutations are cross-referenced with viz state to highlight affected nodes.

---

## CSS Class Mapping

### Node Classes

Applied by `getNodeVizClasses()` in `useVisualizationState.ts`, rendered as `.hg-node.<class>` in `hypergraph.css`.

| CSS Class | Derived From | Color | Effect | Visual Description |
|-----------|-------------|-------|--------|-------------------|
| `viz-dimmed` | Not in `involvedNodes` | — | opacity: 0.2, saturate(0.3) | Faded out |
| `viz-query-token` | `queryTokens.has(node)` | Purple dashed | border: 2px dashed | Input pattern indicator |
| `viz-query-active` | `activeQueryToken == node` | Gold | border + box-shadow | Currently compared token |
| `viz-start` | `startNode == node` | Cyan | pulsing glow | Operation entry point |
| `viz-selected` | `selectedNode == node` | White | soft glow | Currently acted-upon node |
| `viz-root` | `rootNode == node` | Gold | ring via `::before` | Exploration root |
| `viz-candidate-parent` | `candidateParent == node` | Orange | pulsing, scale(1.05) | Parent being explored |
| `viz-candidate-child` | `candidateChild == node` | Purple | pulsing, scale(1.05) | Child being explored |
| `viz-matched` | `matchedNode == node` | Green | glow | Token matched query |
| `viz-mismatched` | `mismatchedNode == node` | Red | glow | Token didn't match query |
| `viz-path` | `tracePath.has(node)` | Blue | glow trail | Exploration breadcrumb |
| `viz-completed` | `completedNodes.has(node)` | Green | glow | Confirmed/explored |
| `viz-pending-parent` | `pendingParents.has(node)` | Dim orange | subtle | In parent queue |
| `viz-pending-child` | `pendingChildren.has(node)` | Dim purple | subtle | In child queue |

### Search Path Classes

Applied based on `VizPathGraph` data — more precise than `LocationInfo.trace_path`.

| CSS Class | Derived From | Color | Visual Description |
|-----------|-------------|-------|-------------------|
| `viz-sp-start` | `searchPath.start_node.index == node` | Cyan | Search path origin |
| `viz-sp-root` | `searchPath.root.index == node` | Gold ring | Search path root (highest point) |
| `viz-sp-start-path` | Node in `searchPath.start_path[]` | Warm orange | Upward exploration trail |
| `viz-sp-end-path` | Node in `searchPath.end_path[]` | Cool cyan | Downward comparison trail |

### Insert Operation Classes

Applied based on transition-specific fields.

| CSS Class | Derived From | Color | Animation | Visual Description |
|-----------|-------------|-------|-----------|-------------------|
| `viz-split-source` | `splitSource == node` | Warm orange | `viz-insert-pulse` | Node being split |
| `viz-split-left` | `splitLeft == node` | Orange-left | — | Left fragment |
| `viz-split-right` | `splitRight == node` | Orange-right | — | Right fragment |
| `viz-join-left` | `joinLeft == node` | Cool blue | — | Left join input |
| `viz-join-right` | `joinRight == node` | Cool blue | — | Right join input |
| `viz-join-result` | `joinResult == node` | Green | `viz-join-glow` | Join output |
| `viz-new-pattern` | `newPatternParent == node` | Yellow | — | Pattern host node |
| `viz-new-pattern-child` | `newPatternChildren.has(node)` | Light yellow | — | Pattern child |
| `viz-new-root` | `newRoot == node` | White-gold | `viz-insert-pulse` | New root token |

---

## Edge Color Mapping

Edge colors are set in the WebGPU overlay renderer (`useOverlayRenderer.ts`). Priority order (first match wins):

| Priority | Condition | Color Constant | RGB | Alpha | edgeType | Description |
|----------|-----------|----------------|-----|-------|----------|-------------|
| 1 | `searchRootEdgeKeys.has(pairKey)` | `SP_ROOT_EDGE_COLOR` | (1.0, 0.85, 0.3) | 0.95 | 3 | Gold — root edge (bidirectional radiance) |
| 2 | `searchStartEdgeKeys.has(pairKey)` | `SP_PATH_EDGE_COLOR` | (0.25, 0.75, 1.0) | 0.90 | 2 | Teal — upward start path (arrow toward parent/A) |
| 3 | `searchEndEdgeKeys.has(pairKey)` | `SP_PATH_EDGE_COLOR` | (0.25, 0.75, 1.0) | 0.90 | 4 | Teal — downward end path (arrow toward child/B) |
| 4 | Legacy `pathEdgeKeys.has(pairKey)` | `PATH_EDGE_COLOR` | (0.1, 0.75, 0.95) | 0.90 | 5 | Cyan — trace_path fallback |
| 5 | Candidate edge (endpoint in candidates) | `CANDIDATE_EDGE_COLOR` | (0.55, 0.4, 0.8) | 0.30 | 6 | Muted violet — pending |
| 6a | Insert edge + join result | `INSERT_JOIN_EDGE_COLOR` | (0.5, 0.85, 0.5) | 0.85 | 7 | Green — join result edge |
| 6b | Insert edge (non-join) | `INSERT_EDGE_COLOR` | (1.0, 0.55, 0.2) | 0.85 | 7 | Warm orange — insert edge |
| 7 | Selected node parent edge | `PARENT_EDGE_COLOR` | (0.95, 0.65, 0.2) | — | 1 | Amber — parent of selected |
| 8 | Selected node child edge | `CHILD_EDGE_COLOR` | (0.3, 0.7, 0.9) | — | 1 | Teal — child of selected |

Edge pair keys are computed as `(from << 16) | to` via `edgePairKey()`.

### Edge Type Encoding (WGSL shader)

| edgeType | Description | Arrow |
|----------|-------------|-------|
| 0 | Grid/simple | None |
| 1 | Normal edge | None (subtle energy flow) |
| 2 | Search path start | Arrow toward A (parent) |
| 3 | Search path root | Bidirectional golden radiance |
| 4 | Search path end | Arrow toward B (child) |
| 5 | Trace path | Gentle flow |
| 6 | Candidate edge | Muted violet pulse |
| 7 | Insert edge | Warm orange/green beam |

---

## Phase Badge Colors

Transition kind → timeline badge color in `SearchStatePanel` / `InsertStatePanel` (CSS classes `.ssp-phase.phase-<kind>`):

| Transition Kind | Badge Color | Hex |
|----------------|-------------|-----|
| `start_node` | Cyan | `#60d8ff` |
| `visit_parent` | Orange | `#ffa860` |
| `visit_child` | Purple | `#c090ff` |
| `child_match` | Green | `#70e080` |
| `child_mismatch` | Red | `#ff7060` |
| `candidate_mismatch` | Orange | `#ffa860` |
| `candidate_match` | Purple | `#c090ff` |
| `parent_explore` | Red-orange | `#ff9070` |
| `split_start` | Red-orange | `#ff9070` |
| `split_complete` | Green | `#70e080` |
| `join_start` | Purple | `#c090ff` |
| `join_step` | Light blue | `#a0c0ff` |
| `join_complete` | Green | `#70e080` |
| `create_pattern` | Yellow | `#ffdc60` |
| `create_root` | Orange | `#ffa860` |
| `update_pattern` | Light blue | `#a0c0ff` |
| `done` | (no specific badge) | — |

---

## Context-Read Event Schema (Design)

> **Status:** Design only — not yet implemented. For future `context-read` crate integration.

### Proposed Transitions

Context-read expands a matched node into its full context (surrounding tokens). Proposed transitions:

#### `ReadStart`
```rust
ReadStart {
    node: usize,        // Starting node (from search result)
    direction: ReadDirection, // Forward, Backward, or Both
    budget: usize,      // Max atoms to read
}
```
**When:** Read operation begins from a search result node.

#### `ExpandForward`
```rust
ExpandForward {
    from: usize,        // Current frontier node
    to: usize,          // Next node being read
    atoms_read: usize,  // Cumulative atoms consumed
    budget_remaining: usize,
}
```
**When:** Reading forward (right) from the current position.

#### `ExpandBackward`
```rust
ExpandBackward {
    from: usize,        // Current frontier node
    to: usize,          // Previous node being read
    atoms_read: usize,
    budget_remaining: usize,
}
```
**When:** Reading backward (left) from the current position.

#### `BoundaryReached`
```rust
BoundaryReached {
    node: usize,        // Node at the boundary
    direction: ReadDirection,
    reason: BoundaryReason, // EndOfGraph, BudgetExhausted, PatternBoundary
}
```
**When:** Expansion stops in one direction.

#### `ReadComplete`
```rust
ReadComplete {
    center_node: usize,
    context_range: (usize, usize), // (start_atom, end_atom) of full context
    total_atoms_read: usize,
}
```
**When:** Read operation finishes.

### Proposed CSS Classes

| Class | Color | Description |
|-------|-------|-------------|
| `viz-read-center` | Bright white | The search result node being expanded |
| `viz-read-forward` | Light green | Nodes read in forward direction |
| `viz-read-backward` | Light blue | Nodes read in backward direction |
| `viz-read-boundary` | Amber | Boundary nodes where expansion stopped |
| `viz-read-context` | Subtle highlight | Full context span |

### Proposed Edge Colors

| Edge Type | Color | Description |
|-----------|-------|-------------|
| Forward expansion | Green (0.4, 0.85, 0.5) | Reading rightward |
| Backward expansion | Blue (0.3, 0.65, 0.95) | Reading leftward |
| Boundary | Amber (0.95, 0.75, 0.3) | At expansion limit |

### path_id Format

```
read/context-read/<semantic-id>
```

---

## Key Source Files

| File | Role |
|------|------|
| `crates/context-trace/src/graph/visualization.rs` | Transition enum, GraphOpEvent, LocationInfo, QueryInfo, GraphMutation |
| `crates/context-trace/src/graph/search_path.rs` | VizPathGraph and apply_transition (start_path/end_path management) |
| `crates/context-search/src/search/mod.rs` | Search event emission (~10 call sites) |
| `crates/context-insert/src/insert/context.rs` | Insert event emission (split, join start/complete) |
| `crates/context-insert/src/join/context/frontier.rs` | JoinStep emission |
| `crates/context-insert/src/join/context/node/merge/iter.rs` | CreatePattern emission |
| `tools/log-viewer/frontend/src/components/HypergraphView/hooks/useVisualizationState.ts` | Transition → viz state derivation |
| `tools/log-viewer/frontend/src/components/HypergraphView/hooks/useOverlayRenderer.ts` | Edge color logic |
| `tools/log-viewer/frontend/src/search-path/edge-highlighting.ts` | Search path edge key computation |
| `tools/log-viewer/frontend/src/components/HypergraphView/hypergraph.css` | Node CSS classes |
| `tools/log-viewer/frontend/src/components/HypergraphView/hypergraph.wgsl` | WebGPU shader (arrow rendering) |

---

## Changelog

- **2026-03-02:** Renamed `Dequeue` → `CandidateMismatch`, `RootExplore` → `CandidateMatch`, removed `MatchAdvance`. Renamed `GraphDelta` → `GraphMutation`. Removed legacy path_id handling. Fixed emit message to use description. Moved StartNode, VisitParent, VisitChild, ChildMatch, ChildMismatch, Done from common to search-specific. Fixed start_path cleanup on CandidateMismatch. Added edgeType encoding table.
- **2026-03-01:** Initial guide created (Phase 8).
