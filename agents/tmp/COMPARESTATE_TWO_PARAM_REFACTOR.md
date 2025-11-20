# CompareState Two-Parameter Refactoring - Completed

## Summary

Successfully refactored `CompareState` from single generic parameter to two generic parameters to enable independent tracking of query cursor state and index path state.

## Changes Made

### 1. Core Struct Definition
**Before:**
```rust
pub(crate) struct CompareState<S: CursorState = Candidate> {
    #[deref]
    #[deref_mut]
    pub(crate) child_state: ChildState,
    pub(crate) cursor: PathCursor<PatternPrefixPath, S>,
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
```

**After:**
```rust
pub(crate) struct CompareState<Q: CursorState = Candidate, I: CursorState = Candidate> {
    pub(crate) child_state: ChildState,
    pub(crate) cursor: PathCursor<PatternPrefixPath, Q>,
    pub(crate) index_cursor: PathCursor<PatternPrefixPath, I>,
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
```

Key changes:
- Added second generic parameter `I` for index cursor state
- Added new `index_cursor` field tracking index path position/state
- Removed `Deref`/`DerefMut` derives (no longer delegating to child_state)
- Added helper method `rooted_path()` to access child_state path

### 2. Type Aliases Updated
```rust
// Before
type CompareQueue = VecDeque<CompareState<Candidate>>;
type CandidateCompareState = CompareState<Candidate>;
type MatchedCompareState = CompareState<Matched>;

// After
type CompareQueue = VecDeque<CompareState<Candidate, Candidate>>;
type CandidateCompareState = CompareState<Candidate, Candidate>;
type MatchedCompareState = CompareState<Matched, Matched>;
```

### 3. Enum Variants Updated
```rust
pub(crate) enum CompareNext {
    Match(CompareState<Matched, Matched>),
    Mismatch(CompareState<Mismatched, Mismatched>),
    Prefixes(ChildQueue<CompareState<Candidate, Candidate>>),
}
```

### 4. Implementation Blocks Updated
- `impl CompareState<Candidate>` → `impl CompareState<Candidate, Candidate>`
- `impl CompareState<Matched>` → `impl CompareState<Matched, Matched>`
- `impl MarkMatchState for CompareState<Candidate>` → `impl MarkMatchState for CompareState<Candidate, Candidate>`
- `impl IntoAdvanced for CompareState<Candidate>` → `impl IntoAdvanced for CompareState<Candidate, Candidate>`
- `impl IntoAdvanced for CompareState<Matched>` → `impl IntoAdvanced for CompareState<Matched, Matched>`

### 5. Method Updates

#### mark_match() and mark_mismatch()
Now transitions BOTH cursors:
```rust
fn mark_match(self) -> Self::Matched {
    CompareState {
        cursor: self.cursor.mark_match(),
        index_cursor: self.index_cursor.mark_match(),  // NEW
        // ... other fields
    }
}
```

#### into_next_candidate()
Updated to handle both cursors:
```rust
pub(crate) fn into_next_candidate<G: HasGraph>(
    mut self,
    trav: &G,
) -> Result<CompareState<Candidate, Candidate>, CompareState<Matched, Matched>> {
    // ... advance query cursor ...
    let candidate_cursor = self.cursor.as_candidate();
    let candidate_index_cursor = self.index_cursor.as_candidate();  // NEW
    
    // Return both cursors
    Ok(CompareState {
        cursor: candidate_cursor,
        index_cursor: candidate_index_cursor,  // NEW
        // ...
    })
}
```

#### prefix_states() and all constructor sites
All places creating `CompareState` now initialize both cursors:
```rust
CompareState {
    cursor: /* query cursor */,
    index_cursor: /* index cursor */,  // NEW
    // ... other fields
}
```

### 6. Related Files Updated
- `context-search/src/compare/state.rs` - Main struct and implementations
- `context-search/src/compare/parent.rs` - CompareRootState initialization
- `context-search/src/compare/iterator.rs` - Type signatures
- `context-search/src/match/root_cursor.rs` - Candidate state creation

## Compilation Status

✅ **Successfully compiles** with only warnings (no errors)

## Test Status

⚠️ **Test still fails** - Expected behavior, as this is just the structural refactoring. 

The test `find_pattern1` still fails with missing cache entries because we haven't yet implemented the logic to **advance the index cursor independently** from the query cursor.

## Next Steps

### TODO: Implement Independent Cursor Advancement

The refactoring enables, but doesn't implement, the core algorithmic fix:

1. **After Match**: Need to advance BOTH cursors separately
   - Advance query cursor (to get next query token)
   - Advance index cursor (to get next graph token)
   - Each can fail independently

2. **New Methods Needed**:
   ```rust
   impl CompareState<Matched, Matched> {
       /// Advance only the query cursor
       pub(crate) fn advance_query_cursor<G: HasGraph>(
           self, trav: &G
       ) -> Result<CompareState<Candidate, Matched>, CompareState<Matched, Matched>>;
       
       /// Advance only the index cursor
       pub(crate) fn advance_index_cursor<G: HasGraph>(
           self, trav: &G
       ) -> Result<CompareState<Matched, Candidate>, CompareState<Matched, Matched>>;
   }
   ```

3. **Update RootCursor::next()**: After `Match`, call both advancement methods:
   ```rust
   Match(matched_state) => {
       // Try to advance query cursor
       match matched_state.advance_query_cursor(trav) {
           Ok(query_advanced) => {
               // Try to advance index cursor
               match query_advanced.advance_index_cursor(trav) {
                   Ok(both_advanced) => {
                       // Both advanced - continue comparing
                       *self.state = both_advanced.as_candidate_candidate();
                       Some(Continue(()))
                   },
                   Err(_) => {
                       // Index ended but query continues - explore parent
                       // This is where we add cache entry and move up
                   }
               }
           },
           Err(_) => {
               // Query ended - complete match
           }
       }
   }
   ```

4. **Cache Entry Addition**: When index path ends but query continues, that's when we add the cache entry and explore parents.

## Architecture Notes

### Why Two Parameters?

The algorithm needs to track two independent state machines:
- **Query cursor (Q)**: Tracks position in the search pattern
- **Index cursor (I)**: Tracks position in the graph path being matched

These advance independently:
- Query advances when we successfully match a token (consume query)
- Index advances when we successfully match a token (consume graph)
- When index ends but query continues → parent exploration
- When query ends → complete match found

### State Combinations

Currently using symmetric states (both Candidate, both Matched, etc.), but the architecture now supports asymmetric states:
- `CompareState<Matched, Candidate>` - Query matched, index not yet matched
- `CompareState<Candidate, Matched>` - Index matched, query not yet matched

This flexibility will be useful for the independent advancement logic.

### Generic Helper Method

Added to base implementation (generic over both states):
```rust
impl<Q: CursorState, I: CursorState> CompareState<Q, I> {
    pub(crate) fn rooted_path(&self) -> &IndexRangePath {
        self.child_state.rooted_path()
    }
}
```

Replaces the previous `Deref` delegation, providing controlled access to child_state path.

## Verification

### Build Command
```bash
cargo build -p context-search
```
Result: ✅ Compiles successfully

### Test Command
```bash
cargo test -p context-search find_pattern1 -- --nocapture
```
Result: ⚠️ Still fails (expected - need to implement advancement logic)

### What Was Preserved
- All existing logic continues to work
- Query cursor advancement works as before
- Index cursor now tracked but not yet independently advanced
- Test infrastructure intact and working

### What Changed
- Type signatures now require two state parameters
- Construction sites initialize both cursors (currently to same values)
- State transitions affect both cursors simultaneously
- Structure ready for independent advancement implementation
