# Phase 5: Integrate MatchResult with Checkpointed Implementation

**Created:** 2025-11-27  
**Status:** Planning  
**Estimated Time:** 60 min

## Objective

Integrate the `Checkpointed<PatternCursor>` architecture with `MatchResult` to properly track both checkpoint (last confirmed match) and current position (advanced for exploration).

## Context

### Current Issue

`MatchResult` currently stores only a single `PatternCursor`:
```rust
pub struct MatchResult {
    pub path: PathCoverage,
    pub cursor: PatternCursor,  // Only checkpoint position
}
```

This creates a mismatch in `create_parent_exploration_state()`:
```rust
// Hybrid cursor: uses current().path + checkpoint.atom_position
let end_cursor = PathCursor {
    path: self.state.query.current().path.clone(),  // Advanced position
    atom_position: checkpoint.atom_position,        // Confirmed matches
    _state: std::marker::PhantomData::<Matched>,
};
```

**Problem:** This hybrid approach is fragile and doesn't align with our `Checkpointed<C>` architecture where:
- `checkpoint` = last confirmed match position
- `candidate` = current exploration position (may be advanced beyond checkpoint)

### Why This Matters

The `find_consecutive1` test failure (end_index=2 vs expected 3) likely stems from this misalignment:
1. First search matches "ghi" → returns cursor with end_index=2 (pointing to "i")
2. Second search should start from end_index=3 (next token "a") but starts from end_index=2
3. This happens because `create_parent_exploration_state()` uses a hybrid cursor instead of properly tracking advancement

### Architectural Insight

From user guidance: "After matching, we update best_match and start a new search cycle from the checkpoint."

This means:
- **During search:** `Checkpointed<C>` tracks checkpoint + candidate
- **At match result:** Need to preserve both positions for parent exploration
- **For consecutive searches:** Start from current position (advanced), not checkpoint

## Current Implementation Analysis

### 1. `create_parent_exploration_state()` (advance.rs:356-392)

**Purpose:** Create MatchResult when child exhausts but query can continue → need parent exploration

**Current Code:**
```rust
pub(crate) fn create_parent_exploration_state(&self) -> MatchResult {
    let checkpoint = self.state.query.checkpoint();
    let checkpoint_child = self.state.child.checkpoint();
    
    // ... path construction from checkpoint_child ...
    
    // HYBRID CURSOR: current path + checkpoint atom_position
    let end_cursor = PathCursor {
        path: self.state.query.current().path.clone(),
        atom_position: checkpoint.atom_position,
        _state: std::marker::PhantomData::<Matched>,
    };
    
    MatchResult {
        cursor: end_cursor,
        path: path_enum,
    }
}
```

**Issue:** Creating a hybrid cursor by mixing current.path with checkpoint.atom_position is fragile and doesn't properly represent the state.

### 2. `create_result_from_state()` (search/mod.rs:305-336)

**Purpose:** Create MatchResult from confirmed matched state (both query and child at Matched)

**Current Code:**
```rust
pub(crate) fn create_result_from_state(
    &self,
    state: CompareState<Matched, Matched>,
) -> MatchResult {
    let result_query = state.query.current();
    let result_child = state.child.current();
    
    // ... path construction from result_child ...
    
    // Simplify query cursor path
    let mut simplified_cursor = result_query.clone();
    Self::simplify_query_cursor(&mut simplified_cursor, trav);
    
    MatchResult {
        cursor: simplified_cursor,
        path: path_enum,
    }
}
```

**Issue:** Uses `current()` which is correct for checkpoint position, but doesn't preserve advancement state for parent exploration.

### 3. MatchResult Usage Locations (30 matches)

**Critical locations:**
1. `query_exhausted()` - checks if cursor reached end of pattern
2. Test assertions - check atom_position values
3. Parent exploration - creates new search nodes from MatchResult
4. Consecutive searches - starts second search from first result's cursor

**Key observation:** Most code accesses `.cursor.path` or `.cursor.atom_position` directly, which will need accessor methods after changing to `Checkpointed<PatternCursor>`.

## Design Decision

### Option A: Change MatchResult.cursor to Checkpointed<PatternCursor>

**Pros:**
- Mirrors internal search state architecture
- Naturally tracks both checkpoint and current position
- Aligns with user's mental model
- Eliminates hybrid cursor construction

**Cons:**
- Breaking change to public API
- Need accessor methods for compatibility
- More complex type signature

### Option B: Keep PatternCursor but add optional current field

**Pros:**
- Less invasive change
- Backward compatible

**Cons:**
- Doesn't align with existing architecture
- Optional field suggests incomplete design
- Still need hybrid cursor logic

### Decision: **Option A** - Use `Checkpointed<PatternCursor>`

**Rationale:**
1. Consistency with internal architecture
2. User's guidance: "after matching, update best_match and start new search cycle from checkpoint"
3. Eliminates fragile hybrid cursor construction
4. Proper separation of checkpoint (confirmed) vs current (exploration) positions
5. Tests already use `.current()` accessor pattern

## Implementation Plan

### Step 1: Update MatchResult Structure (10 min)

**File:** `crates/context-search/src/state/matched/mod.rs`

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchResult {
    pub path: PathCoverage,
    pub cursor: Checkpointed<PatternCursor>,  // Changed!
}
```

Add/update accessor methods:
```rust
impl MatchResult {
    /// Get the checkpoint cursor (last confirmed match position)
    pub fn cursor(&self) -> &PatternCursor {
        self.cursor.checkpoint()
    }
    
    /// Get the current cursor (may be advanced beyond checkpoint)
    pub fn current_cursor(&self) -> CheckpointedRef<'_, PatternCursor> {
        self.cursor.current()
    }
    
    /// Check if query has advanced beyond checkpoint
    pub fn has_advanced_query(&self) -> bool {
        self.cursor.has_candidate()
    }
}
```

### Step 2: Update query_exhausted() (5 min)

**File:** `crates/context-search/src/state/matched/mod.rs`

```rust
pub fn query_exhausted(&self) -> bool {
    use context_trace::{path::accessors::role::End, HasPath, HasRootChildIndex};
    
    // Check current position (may be advanced)
    let current = self.cursor.current();
    let at_end = current.path.is_at_pattern_end();
    let path_empty = HasPath::path(current.path.end_path()).is_empty();
    let end_index = HasRootChildIndex::<End>::root_child_index(&current.path);
    
    tracing::debug!(
        at_end,
        path_empty,
        end_index,
        end_path_len=%HasPath::path(current.path.end_path()).len(),
        "query_exhausted check"
    );
    at_end && path_empty
}
```

**Key Change:** Use `current()` instead of direct field access to check if query is exhausted.

### Step 3: Update create_parent_exploration_state() (25 min)

**File:** `crates/context-search/src/match/root_cursor/advance.rs`

**Before:**
```rust
let end_cursor = PathCursor {
    path: self.state.query.current().path.clone(),
    atom_position: checkpoint.atom_position,
    _state: std::marker::PhantomData::<Matched>,
};

MatchResult {
    cursor: end_cursor,
    path: path_enum,
}
```

**After:**
```rust
// Clone the entire Checkpointed<PatternCursor> state
// This preserves:
// - checkpoint: last confirmed match position
// - candidate: current exploration position (advanced beyond checkpoint)
let cursor_state = self.state.query.clone();

MatchResult {
    cursor: cursor_state,
    path: path_enum,
}
```

**Rationale:** Just clone the entire checkpointed state! No hybrid cursor construction needed.

### Step 4: Update create_result_from_state() (10 min)

**File:** `crates/context-search/src/search/mod.rs`

**Before:**
```rust
let mut simplified_cursor = result_query.clone();
Self::simplify_query_cursor(&mut simplified_cursor, trav);

MatchResult {
    cursor: simplified_cursor,
    path: path_enum,
}
```

**After:**
```rust
// Clone the checkpoint cursor and simplify
let mut simplified_cursor = state.query.checkpoint().clone();
Self::simplify_query_cursor(&mut simplified_cursor, trav);

// Create Checkpointed with only checkpoint (no candidate)
// This represents a confirmed match state with no advancement
let cursor_state = Checkpointed::new(simplified_cursor);

MatchResult {
    cursor: cursor_state,
    path: path_enum,
}
```

**Rationale:** This creates a match result at checkpoint position (candidate=None). If we need parent exploration, `create_parent_exploration_state()` will preserve the advanced state.

### Step 5: Fix Test Assertions (10 min)

**Files:** `crates/context-search/src/tests/state_advance/*.rs`

**Pattern:** Find direct cursor field access and add `.current()` or `.checkpoint()`:

```rust
// Before:
returned.cursor.atom_position

// After (most common):
returned.cursor.current().atom_position

// Or for checkpoint:
returned.cursor.checkpoint().atom_position
```

**Locations to update (~10-15 test assertions):**
- `tests/state_advance/integration.rs` (3 locations)
- `tests/state_advance/parent_compare_state.rs` (6 locations)

## Implementation Strategy

### Order of Operations

1. **Update MatchResult structure** → Will cause compilation errors everywhere
2. **Fix create_result_from_state()** → Fixes creation in matched state
3. **Fix create_parent_exploration_state()** → Fixes creation for parent exploration
4. **Update query_exhausted()** → Fixes exhaustion check
5. **Fix test assertions** → Bulk fix with sed if possible
6. **Run tests** → Verify functionality

### Compilation Error Strategy

Expect errors like:
- "expected `PatternCursor`, found `Checkpointed<PatternCursor>`"
- "no field `cursor` on type `MatchResult`" (for direct access)

**Fix strategy:** Add `.current()` or `.checkpoint()` as appropriate:
- For comparison/checking: use `.current()` (may be advanced)
- For starting new search: use `.checkpoint()` (confirmed position)
- For parent exploration: use entire `Checkpointed` state

### Sed Commands for Bulk Fixes

```bash
# Fix test assertions (preliminary - may need adjustment)
find crates/context-search/src/tests -name "*.rs" -type f -exec sed -i \
  's/returned\.cursor\.atom_position/returned.cursor.current().atom_position/g' {} \;

find crates/context-search/src/tests -name "*.rs" -type f -exec sed -i \
  's/parent_compare_state\.cursor\.current()/parent_compare_state.cursor/g' {} \;
```

## Expected Outcomes

### Fixes

1. ✅ Eliminate hybrid cursor construction in `create_parent_exploration_state()`
2. ✅ Properly track checkpoint vs current position in MatchResult
3. ✅ Align MatchResult with internal `Checkpointed<C>` architecture
4. ✅ Enable consecutive searches to start from correct position

### Test Impact

**Expected to fix:** `find_consecutive1` test
- First search matches "ghi" → returns cursor with checkpoint at end_index=2, candidate at end_index=3
- Second search starts from current position (end_index=3) → matches "abc"
- Result: end_index=3 as expected ✅

**Expected to pass:** All existing tests after assertion updates

## Validation

### Manual Verification Steps

1. Compile library: `cargo check -p context-search`
2. Run state advancement tests: `cargo test -p context-search --lib state_advance`
3. Run consecutive search test: `cargo test -p context-search --lib consecutive`
4. Run full test suite: `cargo test -p context-search --lib`

### Success Criteria

- [ ] Library compiles without errors
- [ ] All state advancement tests pass (13/13)
- [ ] `find_consecutive1` test passes with end_index=3 ✅
- [ ] No regressions in other tests (40/40 total)

## Risks & Mitigations

### Risk 1: Breaking Change to Public API

**Impact:** External code using `MatchResult.cursor` directly will break

**Mitigation:**
- Provide `.cursor()` accessor that returns checkpoint (most common use case)
- Provide `.current_cursor()` for advanced position
- Clear documentation in migration guide

### Risk 2: Test Assertion Complexity

**Impact:** Many test assertions may need careful updates

**Mitigation:**
- Use bulk sed commands where pattern is clear
- Review each change carefully
- Run tests incrementally after each fix batch

### Risk 3: Misunderstanding Checkpoint vs Current Semantics

**Impact:** Incorrect accessor choice could lead to subtle bugs

**Mitigation:**
- Clear documentation: checkpoint = confirmed, current = exploration
- Follow existing patterns in CompareState usage
- Verify with tracing logs during testing

## Post-Implementation Tasks

1. Update `CHEAT_SHEET.md` with MatchResult accessor patterns
2. Update `HIGH_LEVEL_GUIDE.md` with MatchResult architecture explanation
3. Document checkpoint vs current semantics in inline comments
4. Consider adding migration guide for external users (if applicable)

## Related Work

- **Phase 1-3:** Implemented `Checkpointed<C>` with candidate: Option<C>
- **Phase 4:** Merged with Phase 1 (Deref pattern)
- **Phase 6:** Next - Update Response API with new accessors
- **Phase 7:** Next - Fix remaining tests and verify consecutive search behavior

## Notes

- The hybrid cursor construction in `create_parent_exploration_state()` was a clever workaround but doesn't align with our architecture
- By switching to `Checkpointed<PatternCursor>`, we get proper checkpoint/candidate tracking for free
- This change makes the code more maintainable and aligns with user's mental model
- The test fix (end_index=3) should naturally fall out from proper position tracking
