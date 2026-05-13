---
tags: `#guide` `#context-search` `#algorithm` `#debugging` `#testing` `#api` `#performance`
summary: > **Tags:** #advance-cycle #checkpoint #parent-exploration #search-flow #root-cursor
---

# Advance Cycle Guide

> **Tags:** #advance-cycle #checkpoint #parent-exploration #search-flow #root-cursor

Complete guide to the hierarchical search advance cycle with checkpointed cursors.

## Overview

The advance cycle is the core search algorithm that matches query patterns against the graph through hierarchical parent exploration. It uses a **checkpointed cursor system** where each cursor tracks both:
- **Current state** (optimistic exploration position)
- **Checkpoint state** (confirmed last match position)

This dual-tracking enables **partial match recovery**: when a root exhausts without a complete match, we can recover the best partial match and continue searching in parent tokens.

## Architecture Components

### 1. SearchIterator (`iterator.rs`)

**Purpose:** Orchestrates hierarchical search across tokens of increasing size.

**Key Fields:**
- `queue: SearchQueue` - Priority queue (min-heap by token width)
- `best_checkpoint: Option<MatchResult>` - Best partial/complete match found so far
- `trace_ctx: TraceCtx` - Graph traversal context and trace cache

**Responsibilities:**
- Pop roots from queue (smallest width first)
- Create RootCursor for matched roots
- Track best partial match across all explored roots
- Trigger parent exploration when roots exhaust
- Clear queue when finding matching root (graph invariant: larger matches found via parents)

### 2. RootCursor (`root_cursor.rs`)

**Purpose:** Advances comparison within a single matched root token.

**Key Type:**
```rust
RootCursor<K, Q, I> {
    trav: K::Trav,                    // Graph traversal
    state: Box<CompareState<Q, I>>,   // Dual cursor state
}
```

**Responsibilities:**
- Advance both query and child cursors in lockstep
- Detect end conditions (QueryExhausted, ChildExhausted, Mismatch)
- Create MatchResult when ending
- Provide parent batch when child exhausts but query continues

### 3. CompareState (`compare/state.rs`)

**Purpose:** Dual cursor system for pattern matching.

**Key Fields:**
```rust
CompareState<Q, I> {
    query: Checkpointed<PathCursor<PatternRangePath, Q>>,  // Pattern position
    child: Checkpointed<ChildCursor<I, EndNode>>,          // Graph position
}
```

**Invariants:**
- `checkpoint.atom_position <= current.atom_position` (always)
- Checkpoint represents last confirmed match
- Current represents exploring/candidate position

### 4. Checkpointed Wrapper (`cursor/checkpointed.rs`)

**Purpose:** Prevent cursor/checkpoint desynchronization.

**Key Fields:**
```rust
Checkpointed<C> {
    current: C,      // Exploring state (Candidate or Matched)
    checkpoint: C,   // Last confirmed Matched state
}
```

**Operations:**
- `mark_match()` - Advance checkpoint to current (confirm match)
- `mark_mismatch()` - Keep checkpoint, reset current (undo speculation)
- `checkpoint()` - Always returns Matched state
- `current()` - Returns current exploration state

## The Advance Cycle (Complete Flow)

### Phase 1: Initial Root Match

```
SearchIterator.next()
  └─> Pop SearchNode from queue (smallest width)
      └─> NodeConsumer.consume()
          └─> CompareIterator: advance until match
              └─> Create MatchedCompareState
                  └─> Create RootCursor<K, Matched, Matched>
```

**State:** Both query and child are at `Matched` state, positioned at start of matched root.

### Phase 2: Root Advancement

```
RootCursor::advance_to_end()
  └─> Loop: advance_to_next_match()
      ├─> Can both advance?
      │   ├─> Yes: continue loop
      │   └─> No: check end reason
      ├─> Query exhausted? → Ok(MatchResult) [COMPLETE MATCH]
      ├─> Child exhausted but query continues?
      │   └─> Err((checkpoint_state, root_cursor)) [NEED PARENT EXPLORATION]
      └─> Mismatch with progress?
          └─> Ok(MatchResult) [PARTIAL MATCH]
```

**Three Possible Outcomes:**

1. **Complete Match:** `Ok(MatchResult)` with `query_exhausted() == true`
   - Query fully matched within this root
   - Return to SearchIterator → update best_checkpoint → continue iteration

2. **Partial Match:** `Ok(MatchResult)` with `query_exhausted() == false`
   - Hit mismatch after making progress
   - Return to SearchIterator → update best_checkpoint → continue iteration

3. **Need Parent Exploration:** `Err((checkpoint_state, root_cursor))`
   - Child cursor exhausted but query cursor can continue
   - Need larger tokens to continue matching
   - Trigger Phase 3

### Phase 3: Parent Exploration

```
SearchIterator.next() receives Err((checkpoint_state, root_cursor))
  └─> Update best_checkpoint with checkpoint_state
  └─> Clear search queue (graph invariant)
  └─> Call root_cursor.next_parents()
      └─> DirectedTraversalPolicy::next_batch()
          └─> Get parent tokens containing this root
              └─> CompareParentBatch
  └─> Add parents to queue as SearchNode::ParentCandidate
  └─> Recursively call self.next() → back to Phase 1
```

**Key Insight:** Larger matches are found by exploring parents of matched roots, not by searching larger roots independently (graph containment invariant).

### Phase 4: Result Selection

```
SearchState::search() consumes SearchIterator
  └─> Loop: call iterator.next()
      └─> Each MatchResult updates best_checkpoint
          └─> Prefer: more query tokens matched
          └─> Tie-break: complete over incomplete
  └─> When iterator exhausts:
      └─> Return Response { cache, end: best_checkpoint }
```

**SearchState tracks:** Best match across entire hierarchical search, not just current root.

## Checkpoint vs Current Semantics

### When Child Ends but Query Continues

This is the critical case that requires hybrid cursor construction:

```rust
// In create_checkpoint_state():
let end_cursor = PathCursor {
    path: self.state.query.current().path.clone(),  // ← Advanced position (where to search next)
    atom_position: checkpoint.atom_position,         // ← Matched count (what confirmed)
    _state: PhantomData::<Matched>,
};
```

**Why Hybrid?**
- `current.path` points to next token to match (in parent tokens)
- `checkpoint.atom_position` counts tokens successfully matched so far
- Parent exploration needs both: where to continue + what matched

**Wrong Approaches:**
- ❌ Use `checkpoint.path` → Points to last matched token, not next
- ❌ Increment `checkpoint.path` → Violates checkpoint immutability
- ❌ Use `current.atom_position` → May be ahead of confirmed matches

**Correct Understanding:**
- Checkpoint is conservative (last confirmed state)
- Current is optimistic (exploring ahead)
- When child exhausts: we're Matched at `checkpoint.atom_position`, exploring at `current.path`

## State Transitions

### Checkpointed Cursor State Machine

```
Initial: Candidate
    │
    ├─> mark_match() → Matched (checkpoint = current)
    │       │
    │       ├─> mark_match() → Matched (advance both)
    │       └─> mark_mismatch() → Candidate (rollback current)
    │
    └─> mark_mismatch() → Candidate (no checkpoint update)
```

### CompareState Transitions

```
RootCursor<K, Candidate, Matched>  // Initial: query exploring, child confirmed
    │
    ├─> Both advance → RootCursor<K, Candidate, Matched>
    │
    ├─> Query mark_match() → RootCursor<K, Matched, Matched>
    │
    ├─> Query ends → Ok(MatchResult)
    │
    ├─> Child exhausts → Err((checkpoint_state, root_cursor))
    │
    └─> Mismatch → Ok(MatchResult) or continue
```

## Key Functions

### `advance_to_end()` (`root_cursor.rs:82-169`)

**Signature:**
```rust
fn advance_to_end(mut self) -> Result<
    MatchResult,
    (MatchResult, RootCursor<K, Candidate, Matched>),
>
```

**Returns:**
- `Ok(MatchResult)` - Query exhausted or mismatch with progress
- `Err((checkpoint, cursor))` - Child exhausted, need parent exploration

**Algorithm:**
1. Loop: call `advance_to_next_match()`
2. If query ended → create end state, return Ok
3. If child ended → create checkpoint state, return Err
4. If mismatch → determine if progress made, return Ok or continue

### `create_checkpoint_state()` (`root_cursor.rs:384-421`)

**Purpose:** Create partial match state for parent exploration.

**Critical Code:**
```rust
pub(crate) fn create_checkpoint_state(&self) -> MatchResult {
    let checkpoint = self.state.query.checkpoint();
    
    // Hybrid cursor: current.path + checkpoint.atom_position
    let end_cursor = PathCursor {
        path: self.state.query.current().path.clone(),  // Next token to match
        atom_position: checkpoint.atom_position,         // Confirmed matches
        _state: PhantomData::<Matched>,
    };
    
    MatchResult {
        path: PathCoverage::EntireRoot(
            self.state.child.checkpoint().child_state.path.clone()
        ),
        cursor: end_cursor,
    }
}
```

**When Called:** Child cursor exhausted but query can continue → need parent exploration.

**Returned Value:** MatchResult representing best match in this root before exhaustion.

### `create_end_state()` (`root_cursor.rs:621-680`)

**Purpose:** Create end state when query exhausts or mismatches.

**End Reasons:**
1. `QueryExhausted` - Complete match (query fully consumed)
2. `Mismatch` - Hit token that doesn't match, but made progress

**Critical Decision:**
```rust
let cursor = match reason {
    EndReason::QueryExhausted => {
        // Use current state - query advanced to end
        self.state.query.current().clone()
    },
    EndReason::Mismatch => {
        // Use checkpoint - last confirmed match
        self.state.query.checkpoint().clone()
    },
};
```

## Parent Exploration Mechanism

### Why Parent Exploration?

**Graph Containment Invariant:** If pattern `P` matches at position `X`, and `X` is contained in larger token `Y`, then `P` might match across the boundary of `X` within `Y`.

**Example:**
```
Pattern: [A, B, C]
Graph:
  Token T1: [A, B]      ← Matches [A, B], exhausts at position 2
  Token T2: [T1, C]     ← T1 is child of T2
                        ← Can continue matching C in T2
```

### Parent Exploration Flow

1. **Child Exhausts:**
   - RootCursor reaches end of child token
   - Query still has tokens to match
   - Return `Err((checkpoint_state, root_cursor))`

2. **SearchIterator Receives Error:**
   - Update `best_checkpoint` with checkpoint_state
   - Clear queue (larger matches via parents only)
   - Call `root_cursor.next_parents()`

3. **Get Parent Batch:**
   - TraversalPolicy finds parent tokens containing current root
   - Return `CompareParentBatch` with parent tokens
   - Each parent becomes `SearchNode::ParentCandidate`

4. **Queue Parents:**
   - Add all parents to priority queue
   - Priority: smaller width first (min-heap)
   - Recursively call `SearchIterator.next()`

5. **Continue Matching:**
   - Parents popped from queue
   - Each creates new RootCursor
   - Matching continues from checkpoint position
   - Process repeats (hierarchical expansion)

### Parent Batch Structure

```rust
CompareParentBatch {
    parents: Vec<ParentState>,  // Parent tokens to explore
}

ParentState {
    path: IndexRangePath,       // Path to parent token
    // ... other fields
}
```

## Priority Queue Ordering

**Queue Type:** `BinaryHeap<SearchNode>` (max-heap by default)

**Ordering:** Reversed to create min-heap behavior (smallest width first)

**Rationale:**
- Smaller tokens are more specific matches
- Prefer small precise matches over large fuzzy matches
- Hierarchical expansion naturally explores small → large
- Graph invariant: larger matches found via parent exploration

**Implementation:**
```rust
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_priority = self.width();
        let other_priority = other.width();
        
        // Reverse: smaller widths first (min-heap)
        other_priority.cmp(&self_priority)
    }
}
```

## Tracing and Debugging

### Key Trace Points

1. **Queue Operations:**
   - `"Popped SearchNode from priority queue"` - What was popped
   - `"parent batch widths"` - Verify ordering

2. **Checkpoint Updates:**
   - `"Updating best_checkpoint"` - When/why updated
   - `"Not updating best_checkpoint"` - Why rejected

3. **Parent Exploration:**
   - `"need parent exploration"` - Child exhausted
   - `"found parent batch"` - Parent count and widths
   - `"no more parents available"` - Exhausted hierarchy

4. **Cursor State:**
   - `checkpoint_pos` - Confirmed match count
   - `current_pos` - Exploring position
   - `is_complete` - Query exhausted flag

### Debug Commands

```bash
# Full trace with stdout output
LOG_STDOUT=1 LOG_FILTER=trace cargo test -p context-search <test> -- --nocapture

# Check structured logs
cat target/test-logs/<test>.log | grep "checkpoint"
cat target/test-logs/<test>.log | grep "parent_batch"
cat target/test-logs/<test>.log | grep "queue_remaining"
```

## Common Patterns

### Pattern 1: Complete Match in Single Root

```
SearchIterator.next()
  → RootCursor.advance_to_end()
    → Loop: both advance until query exhausted
      → Ok(MatchResult { query_exhausted: true })
  → Update best_checkpoint
  → Continue iteration (may find better match)
```

### Pattern 2: Partial Match with Parent Exploration

```
SearchIterator.next()
  → RootCursor.advance_to_end()
    → Loop: advance until child exhausts
      → Err((checkpoint_state, root_cursor))
  → Update best_checkpoint
  → Get parent batch
  → Queue parents
  → Recursive next() → Pattern 1 or Pattern 2
```

### Pattern 3: Mismatch After Progress

```
SearchIterator.next()
  → RootCursor.advance_to_end()
    → Loop: advance until mismatch
      → Check if checkpoint.atom_position > 0
        → Yes: Ok(MatchResult { query_exhausted: false })
  → Update best_checkpoint if better
  → Continue iteration
```

## Testing Strategy

### Test Checkpoint Semantics

```rust
#[test]
fn test_checkpoint_vs_current() {
    // Verify checkpoint <= current invariant
    // Verify checkpoint only advances on mark_match
    // Verify current resets on mark_mismatch
}
```

### Test Parent Exploration

```rust
#[test]
fn test_parent_exploration_trigger() {
    // Pattern longer than single token
    // Verify child exhausts, query continues
    // Verify Err((checkpoint, cursor)) returned
    // Verify parents queued and explored
}
```

### Test Priority Ordering

```rust
#[test]
fn test_queue_ordering() {
    // Add tokens of various widths
    // Verify smallest popped first
    // Verify min-heap behavior
}
```

## Troubleshooting

### Problem: Wrong end_index in MatchResult

**Symptoms:** Expected end_index X, got Y (off by one)

**Root Cause:** Confusion between checkpoint and current positions

**Solution:** Use `current.path` (where to search) + `checkpoint.atom_position` (what matched)

### Problem: Queue not clearing after match

**Symptoms:** Multiple roots processed after finding match

**Solution:** Verify `queue.nodes.clear()` called after finding matching root

### Problem: Parent exploration not triggered

**Symptoms:** Pattern spans tokens but no parents explored

**Solution:**
1. Check child cursor exhaustion detection
2. Verify `advance_to_end()` returns `Err` for child exhaustion
3. Check `next_parents()` returns parent batch

### Problem: Best checkpoint not optimal

**Symptoms:** Final result worse than intermediate match

**Solution:**
1. Verify comparison logic: prefer more matched tokens
2. Check tie-break: complete over incomplete
3. Trace checkpoint updates with `LOG_FILTER=debug`

## Performance Considerations

### Queue Clearing

**Why Clear?** Graph invariant: all larger matches found via parent exploration of current match.

**Impact:** Prevents redundant exploration of roots that cannot contain better matches.

### Min-Heap Priority

**Why Smallest First?** Smaller tokens are more specific, prefer precise matches.

**Impact:** Finds best small matches before exploring large fuzzy matches.

### Checkpoint Tracking

**Why Track Best?** SearchIterator explores many roots, need best across all.

**Impact:** O(1) final result selection, no need to re-compare all matches.

## Related Documentation

- **CHEAT_SHEET.md:** Quick reference for types and patterns
- **crates/context-stack/context-search/HIGH_LEVEL_GUIDE.md:** Architecture overview
- **agents/guides/20251203_UNIFIED_API_GUIDE.md:** Response API and usage
- **agents/guides/20251203_TRACING_GUIDE.md:** Debugging and log analysis
- **agents/bug-reports/INDEX.md:** Known issues and patterns

## Summary

The advance cycle implements hierarchical pattern matching through:
1. **Checkpointed cursors** tracking confirmed vs exploring positions
2. **Root advancement** within single tokens
3. **Parent exploration** for cross-token patterns
4. **Priority queue** for optimal match ordering
5. **Best checkpoint tracking** across entire search

The key insight: checkpoint = what we know, current = what we're testing.
