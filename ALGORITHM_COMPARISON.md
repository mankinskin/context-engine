# Algorithm Comparison: Desired vs Current Implementation

This document compares the desired search algorithm (DESIRED_SEARCH_ALGORITHM.md) with the current `find_ancestor` implementation in context-search.

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

## Recommendations

### 1. Queue Clearing (High Priority)

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
