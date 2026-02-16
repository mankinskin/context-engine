---
tags: `#analysi` `#context-search` `#algorithm` `#debugging` `#testing`
summary: This document compares the desired search algorithm (DESIRED_SEARCH_ALGORITHM.md) with the current `find_ancestor` implementation in context-search.
---

# Algorithm Comparison: Desired vs Current Implementation

This document compares the desired search algorithm (DESIRED_SEARCH_ALGORITHM.md) with the current `find_ancestor` implementation in context-search.

> **📋 Quick Navigation**: See [SEARCH_ALGORITHM_ANALYSIS_SUMMARY.md](SEARCH_ALGORITHM_ANALYSIS_SUMMARY.md) for overview and next steps.

## Executive Summary

**High-Level Alignment**: ✅ The current implementation follows the desired algorithm structure closely.

**Key Matches**:
- ✅ Bottom-up exploration with ascending width priority (BinaryHeap)
- ✅ BFS with extra ordering (priority queue)
- ✅ Cursor tracking (query and index positions)
- ✅ Parent exploration when roots are exhausted
- ✅ Trace cache management on match completion

**Key Differences**:
1. ⚠️ **Queue clearing**: Desired algorithm clears queue on new match; current keeps queue (removed clearing after bugs)
2. ⚠️ **Best match tracking**: Desired tracks "best match" explicitly; current uses `last_match` in SearchState
3. ⚠️ **Initialization**: Desired initializes with first token as matched; current starts with query pattern
4. ⚠️ **Incremental tracing**: Desired traces start paths incrementally; current traces at specific checkpoints

## Detailed Comparison

### 1. Initialization

**Desired**:
```
- Initialize best match with first token as matched
- Return error if query is empty
- Advance query to first token in pattern
```

**Current** (`search/mod.rs` lines 42-49):
```rust
fn find_ancestor(&self, searchable: impl Searchable) -> SearchResult {
    searchable.search::<AncestorSearchTraversal<Self>>(self.ctx())
        .map_err(|err| err.reason)
}
```

**Current** (`search/mod.rs` lines 167-184):
```rust
pub(crate) fn search(mut self) -> Response {
    // last_match initialized in SearchState creation
    let mut iteration = 0;
    while let Some(end) = &mut self.next() {
        iteration += 1;
        end.trace(&mut self.matches.trace_ctx);
    }
    
    let end = match self.last_match {
        MatchState::Located(end_state) => end_state,
        MatchState::Query(query_path) => {
            // No matches - create minimal EndState
            // ...
        }
    };
    // ...
}
```

**Analysis**: ✅ Similar - Current initializes `last_match` with query pattern. Empty query would be handled by pattern creation failing earlier.

---

### 2. Parent State Tracking

**Desired**:
```
For each parent, track:
- Path of bottom-up edges (root entry + root parent)
- Atom offset to entry point = matched query cursor position
- Query path with root entry
- Query cursor position (atom width of explored tokens)
```

**Current** (`compare/state.rs` lines 144-159):
```rust
pub(crate) struct CompareState<Q: CursorState, I: CursorState> {
    /// Query cursor: state controlled by generic parameter Q
    pub(crate) cursor: PathCursor<PatternRangePath, Q>,
    
    /// Index cursor: wraps ChildState with IndexRangePath
    pub(crate) child_cursor: ChildCursor<I>,
    
    /// Checkpoint: last confirmed match (always Matched state)
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,
    
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
```

**Analysis**: ✅ **MATCHES** - All required tracking present:
- `child_cursor.child_state.path` = IndexRangePath (bottom-up edges, root entry + parent)
- `cursor.atom_position` = atom offset/query cursor position
- `cursor.path` = PatternRangePath (query path with root entry)
- `checkpoint.atom_position` = last confirmed matched position

---

### 3. Bottom-Up Exploration with Priority

**Desired**:
```
- Explore all parents bottom-up in ascending width order
- Priority: smaller tokens processed first
```

**Current** (`match/mod.rs` lines 115-143):
```rust
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_priority = match self {
            SearchNode::ParentCandidate(state) => {
                let token = state.parent_state.path.root_parent();
                token.width.0  // Smaller = higher priority
            },
            SearchNode::PrefixQueue(_) => usize::MAX,
        };
        
        let other_priority = match other {
            SearchNode::ParentCandidate(state) => {
                let token = state.parent_state.path.root_parent();
                token.width.0
            },
            SearchNode::PrefixQueue(_) => usize::MAX,
        };
        
        // Reverse: smaller priority values come first (min-heap)
        other_priority.cmp(&self_priority)
    }
}
```

**Analysis**: ✅ **PERFECT MATCH** - BinaryHeap with Ord implementation processes smaller width tokens first.

---

### 4. Candidate Root Cursor Creation

**Desired**:
```
Create candidate root cursor from parent state:
- Next token after parent state root entry as end leaf
- Start path computed from last match root
```

**Current** (`compare/parent.rs` - implied by advance_state):
```rust
// ParentCompareState contains:
// - parent_state: ParentState (with IndexRangePath)
// - cursor: PatternCursor (from last match)
// advance_state creates CompareState with both cursors
```

**Analysis**: ✅ **MATCHES** - ParentCompareState combines parent path with cursor from last match.

---

### 5. End Leaf Comparison

**Desired**:
```
Compare end leafs of candidate query and index paths
- Inconclusive if end leafs are not same width
- Append prefix states to search queue
```

**Current** (`compare/iterator.rs` - compare logic):
```rust
pub(crate) fn compare(&mut self) -> CompareResult {
    // Compares current positions
    // Returns FoundMatch, Mismatch, or Prefixes
    // Prefixes triggers decomposition into child queue
}
```

**Analysis**: ✅ **MATCHES** - CompareIterator handles comparison and prefix decomposition.

---

### 6. Match Found - Queue Management

**Desired**:
```
When match found:
- Reinitialize/clear search queue
- All larger matches must be parents of this root
- Track best match when finding new matching root
```

**Current** (`search/mod.rs` lines 133-156):
```rust
if end.reason == EndReason::QueryEnd {
    let current_is_complete = matches!(end.path, PathEnum::Complete(_));
    let should_update = match &self.last_match {
        MatchState::Located(prev_end) => {
            let prev_is_complete = prev_end.is_complete();
            // Only update if current is Complete AND previous is NOT Complete
            !prev_is_complete && current_is_complete
        },
        MatchState::Query(_) => true,  // First match
    };
    
    if should_update {
        TraceStart { end: &end, pos: 0 }.trace(&mut self.matches.trace_ctx);
        self.last_match = MatchState::Located(end.clone());
    }
}
// NOTE: Queue NOT cleared here
```

**Analysis**: ⚠️ **KEY DIFFERENCE**
- **Desired**: Clear queue when best match found (all larger matches are parents)
- **Current**: Queue NOT cleared - continues processing all nodes
- **History**: Queue clearing was removed after causing test failures
- **Impact**: May process more nodes than necessary, but ensures correctness

---

### 7. Finding End in Matched Root

**Desired**:
```
After finding first match in root:
1. Advance query cursor to next candidate token
2. Advance index path into remaining root
3. If reached end of root -> explore parents
4. Otherwise -> compare candidate tokens until mismatch or query end
```

**Current** (`match/root_cursor.rs` lines 60-115 and 146-235):
```rust
// advance_to_candidate: Matched -> Candidate
pub(crate) fn advance_to_candidate(self) 
    -> Result<RootCursor<G, Candidate, Candidate>, 
              Result<EndState, RootCursor<G, Candidate, Matched>>>
{
    match matched_state.advance_query_cursor(&trav) {
        Ok(query_advanced) => {
            match query_advanced.advance_index_cursor(&trav) {
                Ok(both_advanced) => Ok(candidate_cursor),
                Err(_) => Err(Err(need_parent))  // Index ended
            }
        },
        Err(_) => Err(Ok(end_state))  // Query ended
    }
}

// advance_to_matched: Candidate -> Matched (with iteration)
pub(crate) fn advance_to_matched(mut self)
    -> Result<RootCursor<G, Matched, Matched>, Result<EndState, Self>>
{
    loop {
        match self.next() {  // Iterator compares candidates
            Some(Continue(())) => continue,  // Matched, advance
            Some(Break(reason)) => return Err(Ok(end_state)),  // QueryEnd/Mismatch
            None => return Err(Err(self)),  // Need parents
        }
    }
}
```

**Analysis**: ✅ **MATCHES PERFECTLY** - Current implementation has dedicated functions for:
1. ✅ Advance query cursor (`advance_query_cursor`)
2. ✅ Advance index cursor (`advance_index_cursor`)
3. ✅ Detect end of root (index cursor advance fails) -> parent exploration
4. ✅ Compare candidates until mismatch/QueryEnd (Iterator on RootCursor)

---

### 8. Trace Cache Management

**Desired**:
```
When returning match result:
1. Commit all atom position traces for result
2. Trace end path to commit final end position
3. Commit start paths incrementally while finding larger matches
```

**Current** (`search/mod.rs` lines 145-148):
```rust
if should_update {
    TraceStart { end: &end, pos: 0 }.trace(&mut self.matches.trace_ctx);
    self.last_match = MatchState::Located(end.clone());
}
```

**Current** (`search/mod.rs` lines 175-177):
```rust
// At end of search:
let trace_ctx = &mut self.matches.trace_ctx;
end.trace(trace_ctx);
```

**Analysis**: ⚠️ **PARTIAL MATCH**
- ✅ End path traced when returning result
- ✅ Traces committed on QueryEnd matches
- ⚠️ **Difference**: Current traces at position 0 for each QueryEnd
- ⚠️ **Difference**: "Incremental start path tracing" not explicitly visible
- **Reason**: Tracing happens at specific checkpoints, not continuously

---

## Summary Table

| Feature | Desired | Current | Status |
|---------|---------|---------|--------|
| Initialization | First token matched | Query pattern | ✅ Equivalent |
| Parent tracking | All 4 fields | CompareState with all fields | ✅ Match |
| Ascending width priority | BFS with ordering | BinaryHeap with Ord | ✅ Perfect |
| Candidate creation | From parent + last match | ParentCompareState | ✅ Match |
| End leaf comparison | Inconclusive -> prefixes | Prefixes result | ✅ Match |
| **Queue clearing** | **Clear on match** | **No clearing** | ⚠️ **Different** |
| **Best match tracking** | **Explicit tracking** | **last_match in SearchState** | ⚠️ **Different pattern** |
| Advance query/index | Separate operations | advance_query/index_cursor | ✅ Match |
| Parent exploration | When root exhausted | When index advance fails | ✅ Match |
| Comparison iteration | Until mismatch/QueryEnd | Iterator on RootCursor | ✅ Perfect |
| **Trace cache** | **Incremental start paths** | **Checkpoint tracing** | ⚠️ **Different timing** |

---

# Implementation Roadmap

See **BEST_MATCH_IMPLEMENTATION_STRATEGY.md** for detailed implementation plan.

## Quick Summary

### Root Cause of Differences

The current implementation is **functionally correct but inefficient**:
- ✅ Finds correct matches (find_ancestor1_a_b_c passes)
- ⚠️ Processes more nodes than necessary (no queue clearing)
- ⚠️ Traces redundantly (traces intermediate matches)
- ⚠️ May have issues with extended queries (find_ancestor1_a_b_c_c fails)

### Key Fix: Queue Clearing + Width Comparison

**The substring-graph invariant**: Once we find **any match** (first match in a root R), all future matches will be reachable from ancestors of R. Because we are in the smallest matching substring and all substring nodes are reachable from superstring nodes, all future matches must be parents of R.

**Applies to all path types**: Complete (entire token), Range/Prefix/Postfix (partial matches)

**Search Node Types**:
- **Candidate parent paths**: `ParentCompareState` - no match in root yet
- **Matched root cursors**: `RootCursor` - matched at least once, established substring location

Therefore:
1. **Find abc (width 3)** - first match in smallest root (any path type)
2. **Clear queue** - remove candidate parents (unmatched), they're on unrelated branches
3. **Add parents of abc** - only explore ancestors of matched root (abcd, etc.)
4. **Continue** - explore parent roots with continuation of query

This ensures:
- **Efficiency**: Don't process unrelated candidate parents
- **Correctness**: Don't miss larger ancestor matches (all reachable from matched root's parents)
- **Optimality**: First match is in smallest root token (priority queue guarantees)

### Implementation Phases

1. ✅ **Phase 1**: Add width comparison between Complete matches
2. ✅ **Phase 2**: Clear queue on Complete match, add parents
3. ✅ **Phase 3**: Remove intermediate tracing (trace only final)
4. 🧪 **Phase 4**: Test and verify (especially find_ancestor1_a_b_c_c)
5. 🔮 **Phase 5**: (Optional) Incremental start path tracing

---

## Recommendations

### 1. Queue Clearing (High Priority) ✅ ADDRESSED IN STRATEGY

**Issue**: Desired algorithm clears queue when match found, current doesn't.

**Rationale from desired**: "all larger matches must be parents of this root, or the final match ends in this root, because of the inherent structure of the substring-graph"

**Current concern**: Queue clearing was removed because it caused test failures.

**Action**: 
- ✅ Test with queue clearing enabled
- ✅ Verify substring-graph invariant holds
- ✅ Check if test expectations need updating

### 2. Best Match Tracking (Medium Priority)

**Issue**: Desired has explicit "best match" concept, current uses `last_match`.

**Current behavior**: Updates `last_match` only when finding Complete match and previous is not Complete.

**Desired behavior**: Track best match at each new matching root, clear queue.

**Analysis**: These are related - best match + queue clear work together. Current approach is more conservative.

**Action**: 
- Consider renaming `last_match` to `best_match` for clarity
- Add explicit documentation about "best match" semantics

### 3. Incremental Start Path Tracing (Low Priority)

**Issue**: Desired traces start paths "incrementally while finding larger matching roots".

**Current behavior**: Traces at specific checkpoints (QueryEnd with Complete path).

**Analysis**: Current approach may be more efficient (fewer trace operations), but might not match desired incremental behavior.

**Action**:
- Profile trace cache population timing
- Verify final cache contents match desired
- If cache is correct, document timing difference as optimization

### 4. Documentation Alignment

**Action**: Update HIGH_LEVEL_GUIDE.md to explicitly reference desired algorithm sections and note implementation differences.

## Test Case: find_ancestor1_a_b_c_c

**Current failure**: Query [a,b,c,c] should match abc token but gets Mismatches.

**Possible relation to differences**:
1. **Queue clearing**: If queue should be cleared after finding abc match, continuing with stale queue state might cause issues.
2. **Best match tracking**: If abc is the best match but later exploration overwrites it incorrectly.

**Next debugging steps**:
1. Add queue clearing back and test
2. Check if abc match is being found but then lost
3. Verify parent exploration starts from correct position after abc match

---

# Deep Analysis: Best Match Checkpointing

## Current Implementation Analysis

### Trace Points (Where cache is populated)

**Location 1**: `search/mod.rs` lines 145-162 - **During search iteration**
```rust
if end.reason == EndReason::QueryEnd {
    let should_update = /* ... logic ... */;
    if should_update {
        TraceStart { end: &end, pos: 0 }.trace(&mut self.matches.trace_ctx);
        self.last_match = MatchState::Located(end.clone());
    }
}
```
- **When**: Each QueryEnd during iteration
- **What**: Traces from position 0 for Complete matches
- **Issue**: Traces ALL QueryEnd matches, even if not the final best match

**Location 2**: `search/mod.rs` lines 241-242 - **After search completes**
```rust
let trace_ctx = &mut self.matches.trace_ctx;
end.trace(trace_ctx);
```
- **When**: Final state after all iterations
- **What**: Traces the final end state
- **Issue**: Re-traces the same match if it was already traced in Location 1

### Current Best Match Logic

**MatchState Enum** (search/mod.rs):
```rust
pub(crate) enum MatchState {
    Query(PatternPrefixPath),     // Initial state - no match yet
    Located(EndState),             // Found a match
}
```

**Current Code** (lines 133-156):
```rust
// OLD INCORRECT LOGIC (now fixed):
// if end.reason == EndReason::QueryEnd { ... }
// This was wrong - QueryEnd doesn't mean "entire search ends"

// CORRECTED LOGIC:
// QueryEnd = query pattern exhausted within this root (match found)
// Mismatch = comparison failed within this root  
// Both represent valid match results in a root

let should_update = match &self.last_match {
    MatchState::Located(prev_end) => {
        // Compare widths of matched roots
        let current_width = end.path.root_parent().width.0;
        let prev_width = prev_end.path.root_parent().width.0;
        current_width < prev_width
    },
    MatchState::Query(_) => true,  // First match in any root
};
```

**Problems** (now addressed):
1. ~~**No width comparison**: Doesn't check if new match is smaller~~ → FIXED: Now compares widths
2. ~~**Complete-path-only bias**: Won't update from one Complete path to another Complete path~~ → FIXED: Compares all matches
3. ~~**Tracing on every update**: Traces immediately instead of at end~~ → FIXED: Deferred to end
4. **No queue clearing**: Continues processing nodes after finding best match → TODO

**EndReason Semantics**:
- **QueryEnd**: Query pattern fully matched within current root (all query tokens consumed)
- **Mismatch**: Token comparison failed within current root (partial match or no match)
- Both are valid "end states" for a root cursor - not search termination

## Desired Behavior (from DESIRED_SEARCH_ALGORITHM.md)

### Best Match Definition
> "We should keep track of the best match at each time we find a new matching root from a parent and clear the search queue."

**Interpretation**:
- "Best match" = smallest root token with any match (Complete/Range/Postfix/Prefix)
- "Matching root from parent" = when we transition from candidate parent to root cursor (first match in root)
- "Clear queue" = remove unmatched candidate parents; all future matches reachable from matched root's ancestors

**Key Distinction**:
- **Candidate parent paths**: Parent states in queue, no match in root yet (still candidates)
- **Matched root cursors** ("matching root"): Have matched at least once in root
- Queue clearing happens on transition from candidate → matched (first match in root)

**Path Types**: Complete (entire token), Range/Prefix/Postfix (partial) - all can trigger queue clearing

### Trace Cache Commitment
> "If we return a match result from find, we make sure all of the atom position traces for the result are added to the trace cache. We need to trace the end path to commit the final end position to the trace cache. The start paths should be committed to the trace cache incrementally while finding larger matching roots which must be contained in the final root."

**Interpretation**:
1. **Final commitment**: Only trace the final best match to cache
2. **Incremental start paths**: Trace start portions as we build up to larger matches
3. **End path**: Trace complete end path only for final result

## Key Insight: Width-Based Priority + Queue Clearing

The BinaryHeap processes tokens in ascending width order. Combined with queue clearing:

1. Process abc (width 3) before abcd (width 4)
2. If abc produces **any match** (Complete/Range/Postfix/Prefix) → clear queue
3. Only explore parents of abc (which are larger)
4. If parent produces match → that becomes new best match

**Substring-Graph Invariant**: Once we find **any match** (first match in a root), all future matches will be reachable from roots containing this substring. Because we process smallest tokens first, and all substring nodes are reachable from their superstring nodes, we can clear unrelated branches.

**Two Types of Nodes in Search**:
1. **Candidate Parent Paths**: Parent states in queue with no match in root yet (from `ParentCompareState`)
2. **Root Cursors**: States that have matched at least once in the root (from `RootCursor`)
   - These are the "matched roots" in parent root search iteration
   - Once we have a root cursor (any match type), future exploration only needs parents of this root

**Path Types**:
- **Complete**: Match covers entire root token
- **Range/Prefix/Postfix**: Partial matches within or across tokens
- **All types** trigger queue clearing once first match found in a root

Therefore:
- **Clear queue** = stop exploring unrelated branches (candidate parents with no match)
- **Only explore parents** = only explore valid larger matches (ancestors of matched root)
- **First match in smallest root** = triggers queue clearing (due to width ordering)

## Implementation Issues

### Issue 1: No Queue Clearing on First Match in Root

**Current**: Queue continues with all nodes (both candidate parents and matched roots intermixed)
**Desired**: Clear queue when first match found in any root (any path type), only add parents of matched root
**Distinction**: 
- **Candidate parent paths**: Parent states with no match in root yet (still exploring)
- **Matched root cursors**: States that matched at least once in root (established substring)
- Once we have a matched root, clear candidate parents, keep only ancestors of matched root
**Impact**: Processes unnecessary branches, may visit roots out of optimal order

### Issue 2: Trace Timing

**Current**: Traces immediately on match update
**Desired**: Trace incrementally for start paths, final trace for end
**Impact**: May trace matches that aren't the final best match

### Issue 3: Width Comparison Missing

**Current**: Only checks Complete path vs not-Complete path
**Desired**: Compare widths to find smallest match
**Impact**: With queue clearing, first Complete path should always be smallest root token (due to priority)

### Issue 4: Checkpoint Update Semantics

**Current**: `checkpoint` in CompareState updated by mark_match
**Desired**: Checkpoints should track incremental progress within a root
**Impact**: Confusion between "checkpoint in current root" vs "best match across all roots"

### Issue 5: Start Path Tracking for Incremental Tracing

**Current**: Start path correctly starts from checkpoint (verified in `parent_state()`)
**Status**: ✅ **CORRECT** - Infrastructure ready for incremental tracing

**Details**:
```rust
// In CompareState::parent_state() (state.rs lines 186-199)
pub(crate) fn parent_state(&self) -> ParentCompareState {
    // Uses self.cursor.path (current position, includes checkpoint)
    let cursor = PathCursor {
        path: self.cursor.path.clone(),  // PatternRangePath with range
        atom_position: self.cursor.atom_position,
        _state: PhantomData,
    };
    
    ParentCompareState {
        parent_state: self.child_cursor.child_state.parent_state(),
        cursor,  // This cursor starts from last match position
    }
}
```

**Benefits**:
- **Incremental tracing**: Each parent match can trace its start segment from last match
- **Proper path composition**: Start paths build incrementally from query start to current position
- **Ready for implementation**: When first match found, can trace start path segment

**Next Step**: Implement incremental start path tracing in search iteration (TODO in code)
