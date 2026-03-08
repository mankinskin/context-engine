---
tags: `#plan` `#context-search` `#algorithm` `#debugging` `#testing` `#performance`
summary: Implement proper best match tracking and trace cache commitment according to the desired algorithm:
status: 📋
---

# Best Match Checkpointing & Trace Cache Implementation Strategy

## Goal
Implement proper best match tracking and trace cache commitment according to the desired algorithm:
1. Track the best (smallest) Complete match
2. Clear search queue when Complete match found
3. Commit traces incrementally for start paths, final trace for end path
4. Only trace the final best match to the cache

## Current State Analysis

### What Works ✅
- BinaryHeap processes tokens in ascending width order (smallest first)
- `last_match` tracking in SearchState
- Separate checkpoint tracking within CompareState
- PatternRangePath tracks both start and end positions

### What Needs Fixing ⚠️
1. **No queue clearing** on Complete match
2. **Traces on every update** instead of only final
3. **No width comparison** between Complete matches
4. **Unclear distinction** between local checkpoint vs global best match

## Implementation Strategy

### Phase 1: Add Queue Clearing on First Match in Root

**Goal**: When we find the first match in any root (transition from candidate parent to matched root cursor), clear the queue of unmatched candidate parents and only explore parents of the matched root.

**Key Concepts**:
- **Candidate parent paths**: States in queue with no match yet (`ParentCompareState`)
- **Matched root cursors**: States that matched at least once (`RootCursor`)
- **Substring invariant**: All future matches reachable from matched root's ancestors
- **EndReason semantics**:
  - **QueryEnd**: Query pattern exhausted in root (all tokens matched)
  - **Mismatch**: Comparison failed in root (partial or no match)
  - Both represent valid match results, not search termination

**Status**: ✅ Width comparison implemented, ⏳ Queue clearing TODO

**Location**: `context-search/src/search/mod.rs` - SearchState::next()

**Current Code** (as of latest fix):
```rust
// QueryEnd means query pattern exhausted in root (match found)
// Mismatch means comparison failed in root
// Both are valid match results

let is_first_match = matches!(&self.last_match, MatchState::Query(_));

let should_update = match &self.last_match {
    MatchState::Located(prev_end) => {
        // Compare widths - smaller root is better
        let current_width = end.path.root_parent().width.0;
        let prev_width = prev_end.path.root_parent().width.0;
        current_width < prev_width
    },
    MatchState::Query(_) => true,  // First match - transition to matched root
};

if should_update {
    if is_first_match {
        // TODO: Clear queue and add parents of matched root
        // TODO: Incremental start path tracing
    }
    self.last_match = MatchState::Located(end.clone());
}
```

**Next Steps**:
1. ✅ Width comparison - DONE
2. ⏳ Queue clearing on first match - TODO
3. ⏳ Incremental start path tracing - TODO (infrastructure ready)

**New Code** (with queue clearing):
```rust
if end.reason == EndReason::QueryEnd {
    // Check if this is a first match in root (candidate parent → matched root cursor)
    let is_first_match_in_root = matches!(&self.last_match, MatchState::Query(_));
    
    let should_update = match &self.last_match {
        MatchState::Located(prev_end) => {
            // Both are matched roots - compare widths (smaller root is better)
            let current_width = end.root_parent_width();
            let prev_width = prev_end.root_parent_width();
            current_width < prev_width
        },
        MatchState::Query(_) => true,  // First match - transition from candidate to matched root
    };
    
    if should_update {
        debug!("Found better match - width={}, is_first={}", end.root_parent_width(), is_first_match_in_root);
        
        // Clear queue on first match: remove unmatched candidate parents
        // Substring invariant: all future matches reachable from this matched root's ancestors
        if is_first_match_in_root {
            debug!("First match in root - clearing candidate parents from queue");
            self.matches.queue.nodes.clear();
            
            // Add parents of this matched root for continued exploration
            // These are the only nodes we need to explore (matched root's ancestors)
            if let Some(parent_batch) = get_parent_batch_for_matched_root(&end, &self.matches) {
                debug!("Adding {} parent nodes of matched root", parent_batch.len());
                self.matches.queue.nodes.extend(parent_batch);
            }
        }
        
        // Don't trace yet - only trace final best match at end
        self.last_match = MatchState::Located(end.clone());
    }
}
```

**Changes**:
1. Add width comparison for Complete vs Complete matches
2. Clear queue when Complete match found
3. Add parents of matched root to queue
4. **Remove immediate tracing** - defer to end

**New Helper Method** (add to EndState):
```rust
impl EndState {
    pub(crate) fn root_parent_width(&self) -> usize {
        match &self.path {
            PathEnum::Complete(p) => p.path.root_parent().width.0,
            PathEnum::Range(p) => p.path.root_parent().width.0,
            PathEnum::Postfix(p) => p.path.root_parent().width.0,
            PathEnum::Prefix(p) => p.path.root_parent().width.0,
        }
    }
}
```

### Phase 2: Remove Redundant Tracing During Iteration

**Goal**: Only trace the final best match, not intermediate matches.

**Location**: `context-search/src/search/mod.rs` - SearchState::next()

**Current Code** (lines 195-203):
```rust
while let Some(end) = &mut self.next() {
    iteration += 1;
    debug!(iteration, "tracing end state");
    end.trace(&mut self.matches.trace_ctx);  // ⚠️ Traces every EndState
}
```

**New Code**:
```rust
while let Some(end) = &mut self.next() {
    iteration += 1;
    debug!(iteration, "processing end state");
    // Don't trace intermediate states - only trace final best match
}
```

**Changes**:
1. Remove `end.trace()` call in loop
2. Keep final trace at end (lines 241-242)

### Phase 3: Trace Only Final Best Match

**Goal**: Ensure only the final best match is traced to cache.

**Location**: `context-search/src/search/mod.rs` - SearchState::search()

**Current Code** (lines 238-248):
```rust
let end = match self.last_match {
    MatchState::Located(end_state) => end_state,
    MatchState::Query(query_path) => {
        // Create minimal EndState for no-match case
    }
};

let trace_ctx = &mut self.matches.trace_ctx;
end.trace(trace_ctx);  // ✅ This is correct - trace final state

let response = Response {
    cache: self.matches.trace_ctx.cache,
    end,
};
```

**No changes needed** - this is already correct. The final `end.trace()` commits the best match to cache.

### Phase 4: Add Parent Batch Extraction for Matched Root

**Goal**: When clearing queue (on first match in root), add parents of the matched root for continued exploration.

**Location**: `context-search/src/search/mod.rs` - new helper function

**New Function**:
```rust
/// Extract parents of a matched root cursor for continued exploration
/// Called when first match found in root (candidate parent → matched root transition)
/// Works for all path types: Complete, Range, Prefix, Postfix
fn get_parent_batch_for_matched_root<K: TraversalKind>(
    end: &EndState,
    matches: &SearchIterator<K>,
) -> Option<Vec<SearchNode>> {
    // Get the root token from the matched path (any path type)
    let root_parent = match &end.path {
        PathEnum::Complete(p) => p.path.root_parent(),
        PathEnum::Range(p) => p.path.root_parent(),
        PathEnum::Postfix(p) => p.path.root_parent(),
        PathEnum::Prefix(p) => p.path.root_parent(),
    };
    
    debug!("Extracting parents for matched root: {}", root_parent);
    
    // Create a cursor state for parent exploration
    // The cursor should be at the position where the match completed
    let cursor = end.cursor.clone();
    
    // Get the IndexRangePath from the Complete path
    let index_path = match &end.path {
        PathEnum::Complete(p) => &p.path,
        _ => return None,
    };
    
    // Create parent state from the matched root
    let parent_state = ParentState {
        path: index_path.clone(),
    };
    
    // Get next batch of parents using the traversal policy
    if let Some(batch) = K::Policy::next_batch(&matches.trace_ctx.trav, &parent_state) {
        let parent_nodes: Vec<SearchNode> = batch
            .parents
            .into_iter()
            .map(|parent_state| {
                SearchNode::ParentCandidate(ParentCompareState {
                    parent_state,
                    cursor: cursor.clone(),
                })
            })
            .collect();
        
        debug!("Found {} parents for continued exploration", parent_nodes.len());
        Some(parent_nodes)
    } else {
        debug!("No parents available for matched root");
        None
    }
}
```

**Note**: This may require importing/exposing additional types from match/compare modules.

### Phase 5: Incremental Start Path Tracing (Future Enhancement)

**Goal**: Trace start paths incrementally as we find larger matching roots.

**Rationale**: The desired algorithm states "start paths should be committed to the trace cache incrementally while finding larger matching roots which must be contained in the final root."

**Current Behavior**: Only traces final match (end path includes start path).

**Analysis**: 
- Current approach is simpler and may be sufficient
- Incremental tracing could help with:
  - Early cache population for partial results
  - Debugging/observability of search progress
  
**Decision**: **DEFER** to Phase 5 (optional enhancement)
- Focus on queue clearing and final trace first
- Measure if incremental tracing provides value

**Implementation** (if needed):
```rust
// In SearchState::next(), when updating last_match:
if should_update && current_is_complete {
    // Trace just the start path portion incrementally
    if let Some(start_path) = end.start_path() {
        // Create partial trace for start path only
        TraceStartOnly { 
            path: start_path.clone(), 
            root_pos: end.root_pos() 
        }.trace(&mut self.matches.trace_ctx);
    }
    
    self.last_match = MatchState::Located(end.clone());
}
```

## Testing Strategy

### Test 1: Queue Clearing Verification
**File**: New test in `context-search/src/tests/search/ancestor.rs`

```rust
#[test]
fn find_ancestor_queue_clearing() {
    // Setup: Graph with abc (width 3) and abcd (width 4)
    // Query: [a, b, c]
    // Expected: Find abc first, clear queue, don't process abcd
    
    // Add debug logging to track queue state
    // Verify queue is cleared after abc match
    // Verify abcd is not processed
}
```

### Test 2: Width Comparison
**File**: New test in `context-search/src/tests/search/ancestor.rs`

```rust
#[test]
fn find_ancestor_width_comparison() {
    // Setup: Graph with abc (width 3) and xyz (width 3) 
    //        and abcdefg (width 7)
    // Scenario: If both abc and xyz match [a,b,c]
    // Expected: First one found wins (priority queue order)
    
    // This tests width comparison between Complete matches
}
```

### Test 3: Parent Exploration After Complete Match
**File**: Modify existing `find_ancestor1_a_b_c`

```rust
#[test]
fn find_ancestor1_a_b_c() {
    // Existing test - should still pass
    // Now with queue clearing, verify:
    // 1. abc found first (width 3) - first match in root (becomes matched root cursor)
    // 2. Queue cleared - candidate parents removed
    // 3. Parents of abc explored (abcd, etc.) - only matched root's ancestors
    // 4. Final result is still abc (smallest root with match)
}
```

### Test 4: Fix find_ancestor1_a_b_c_c
**File**: `context-search/src/tests/search/ancestor.rs`

```rust
#[test]
fn find_ancestor1_a_b_c_c() {
    // Query: [a, b, c, c]
    // Expected: Match abc, then explore parents with remaining [c]
    // With queue clearing:
    // 1. Find abc - first match in root (candidate parent → matched root cursor)
    // 2. Clear queue - remove unmatched candidate parents
    // 3. Add abc's parents to queue - only explore matched root's ancestors
    // 4. Explore parents with query [a,b,c,c] and cursor at position 3
    // 5. Should find abc as best match (smallest root with match)
}
```

## Implementation Order

1. **Step 1**: Add `root_parent_width()` helper to EndState ✅
2. **Step 2**: Add width comparison to should_update logic ✅
3. **Step 3**: Add queue clearing on Complete match ✅
4. **Step 4**: Implement `get_parent_batch_for_complete_match()` helper ✅
5. **Step 5**: Remove intermediate tracing in loop ✅
6. **Step 6**: Test with find_ancestor1_a_b_c (should still pass) 🧪
7. **Step 7**: Test with find_ancestor1_a_b_c_c (should now pass) 🧪
8. **Step 8**: Add new queue clearing verification tests 🧪
9. **Step 9**: Profile and verify trace cache contents ✅
10. **Step 10**: (Optional) Implement incremental start path tracing 🔮

## Rollback Plan

If queue clearing causes issues:
1. Keep width comparison (improves best match selection)
2. Revert queue clearing
3. Keep single final trace (removes redundant tracing)
4. Investigate why queue clearing fails tests

Each phase is independently valuable:
- **Width comparison**: Better best match selection
- **Queue clearing**: Efficiency improvement + algorithm correctness
- **Single final trace**: Removes redundancy, cleaner cache

## Expected Outcomes

### After Phase 1-3:
- ✅ find_ancestor1_a_b_c continues to pass
- ✅ find_ancestor1_a_b_c_c should pass (key test)
- ✅ Fewer nodes processed (queue clearing removes unmatched candidate parents)
- ✅ Cleaner trace cache (no duplicate entries)
- ✅ Best match is always smallest root token with any match (first match due to priority)
- ✅ Only explores ancestors of matched roots (not unrelated candidate parents)

### Performance:
- **Fewer iterations**: Queue clearing stops unnecessary exploration
- **Cleaner cache**: Only final match traced
- **Same correctness**: Substring-graph invariant ensures we don't miss matches

### Debugging:
- Add logging for:
  - Queue clearing events
  - Width comparisons
  - Parent batch extraction
  - Final trace commitment

## Next Steps

1. Review this strategy with maintainer
2. Implement Phase 1 (queue clearing + width comparison)
3. Run test suite to verify no regressions
4. Debug find_ancestor1_a_b_c_c specifically
5. Add new tests for queue clearing behavior
6. Document the changes in HIGH_LEVEL_GUIDE.md
