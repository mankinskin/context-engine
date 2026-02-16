---
tags: `#analysi` `#context-search` `#debugging` `#testing` `#api`
summary: The `find_consecutive1` test fails because after successfully matching "ghi", the search advances the query cursor to look for "a" but never finds ...
---

# Advanced Query Cursor in Response Analysis

## Problem Statement

The `find_consecutive1` test fails because after successfully matching "ghi", the search advances the query cursor to look for "a" but never finds it. The current `MatchResult` type only represents `<Matched, Matched>` states, but when we advance the query without finding a match, we have a `<Candidate, Matched>` state where:

- **checkpoint query cursor**: Points to last matched token (position 2, end_index=1, pointing to "h")
- **current query cursor**: Advanced beyond match (position 3, end_index=2, pointing to "i")

The test expects `end_index=3` (pointing to first unmatched token "a" after "i"), but gets `end_index=2` (pointing to last matched token "i").

## Current best_match Update Locations

### 1. **Initial Creation** (search/mod.rs:151)
```rust
let mut best_match = None;
```
- **When**: At start of search fold
- **Value**: None initially

### 2. **After Each Successful Match** (search/mod.rs:157)
```rust
best_match = Some(matched_state);
```
- **When**: After `matched_state.trace()` completes in the search loop
- **Value**: `MatchResult` from `create_result_from_state()` with `<Matched, Matched>` state
- **Purpose**: Update running best after each root match found

### 3. **After Advancing Root Cursor** (search/mod.rs:217-238)
```rust
last_match.update_checkpoint();
match root_cursor.advance_to_next_match() {
    RootAdvanceResult::Advanced(next_match) => {
        // Successfully advanced - always update best_match
        last_match = next_match.state;
    }
}
```
- **When**: Successfully advanced query AND child cursors, found another match
- **Value**: New `<Matched, Matched>` state after successful comparison
- **Purpose**: Track incremental progress within a root

### 4. **On Conclusive End: Exhausted** (search/mod.rs:255-260)
```rust
ConclusiveEnd::Exhausted => {
    // Query exhausted - best_match has final result
    // Return clone so best_match remains set
    break last_match;
}
```
- **When**: Query cursor cannot advance further (reached pattern end)
- **Value**: Uses existing `last_match` (already `<Matched, Matched>`)
- **Purpose**: Query fully consumed - complete match

### 5. **On Conclusive End: Mismatch** (search/mod.rs:261-262)
```rust
ConclusiveEnd::Mismatch(_mismatched) => {
    // Mismatch after progress - best_match has maximum match
    break last_match;
}
```
- **When**: Found mismatch after advancing (compared tokens didn't match)
- **Value**: Uses existing `last_match` (checkpoint before mismatch attempt)
- **Purpose**: Maximum match found before mismatch

### 6. **On Inconclusive End: Need Parent** (search/mod.rs:269-273)
```rust
RootEndResult::Inconclusive(need_parent) => {
    // Child exhausted, query continues - update with parent exploration state
    debug!("Inconclusive end - updating best_match");
    let checkpoint = need_parent.create_parent_exploration_state();
    best_match = Some(checkpoint);
    break last_match;
}
```
- **When**: Query advanced successfully BUT child cannot advance (root boundary reached)
- **Value**: Result from `create_parent_exploration_state()` - **THIS IS THE BUG LOCATION**
- **Purpose**: Capture state where we need to explore parents to continue matching

## The Bug: Missing Advanced Query Cursor

### Current Flow (Incorrect)

1. **Start**: Match "gh" → checkpoint at position 2, end_index=1 (pointing to "h")
2. **Advance Query**: Query advances to position 3, end_index=2 (pointing to "i")
3. **Try Advance Child**: Child exhausted (no more children in "gh")
4. **Create Parent Exploration State**: `create_parent_exploration_state()` is called
5. **BUG**: Uses `checkpoint.path` which has `end_index=1`, NOT `current().path` with `end_index=2`

### Expected Flow (Correct)

After step 3, the result should indicate:
- **Matched up to**: position 2 (two tokens: "g", "h")
- **Next token to match**: "i" at `end_index=2`

But the user expects:
- **Matched up to**: position 3 (three tokens: "g", "h", "i") 
- **Next token to match**: "a" at `end_index=3`

**Wait!** Looking at the log more carefully:

```
checkpoint: Cursor(Pattern["g"(6), "h"(7), "i"(8), "a"(0), "b"(1), "c"(2)][0..1], pos:2),
current: Cursor(Pattern["g"(6), "h"(7), "i"(8), "a"(0), "b"(1), "c"(2)][0..2], pos:3)
```

- checkpoint: `end_index=1`, `pos=2` → Last matched "h" 
- current: `end_index=2`, `pos=3` → Advanced to "i"

The second search finds "ghi" and matches completely. So the first search result should show:
- `checkpoint_pos=2` (matched "g" + "h")
- `end_index=2` (pointing to first unmatched "i")

But test expects after finding "ghi":
- `checkpoint_pos=3` (matched all three)
- `end_index=3` (pointing to first unmatched "a")

## Root Cause Analysis

### The Type System Issue

`MatchResult` contains:
```rust
pub struct MatchResult {
    pub path: PathCoverage,        // Graph location
    pub cursor: PatternCursor,      // Query position (always Matched state)
}
```

The `cursor` field is always `PatternCursor<Matched>` because `MatchResult` represents a checkpoint state. However:

1. **Scenario 1: Query Cannot Advance** (Exhausted)
   - `cursor.path` end_index points to last matched token
   - Correct: This IS the final match state

2. **Scenario 2: Query Advanced But No Match Found** (Need Parent)
   - `cursor.path` end_index should point to FIRST UNMATCHED token
   - **BUG**: Currently points to last matched token
   - **Cause**: `create_parent_exploration_state()` uses `checkpoint.path` instead of `current().path`

### The Fix Location

In `create_parent_exploration_state()` (advance.rs:355-395):

```rust
pub(crate) fn create_parent_exploration_state(&self) -> MatchResult {
    let checkpoint = self.state.query.checkpoint();
    // ...
    
    // BUG: Uses checkpoint path (end_index points to last matched)
    let end_cursor = PathCursor {
        path: self.state.query.current().path.clone(),  // ← ALREADY CORRECT!
        atom_position: checkpoint.atom_position,
        _state: std::marker::PhantomData::<Matched>,
    };
    // ...
}
```

**Wait, the code already uses `current().path`!** Let me check the test log again...

Looking at the log output for the first search that matches "gh":
```
cursor: PathCursor {
    path: RootedRangePath {
        pattern: ["g"(6), "h"(7), "i"(8), "a"(0), "b"(1), "c"(2)],
        start: entry=0, path=[],
        end: entry=1, path=[]    // ← end_index=1, pointing to "h"
    },
    atom_position: AtomPosition(2),
}
```

But the current query cursor when child exhausted was:
```
current: Cursor(Pattern["g"(6), "h"(7), "i"(8), "a"(0), "b"(1), "c"(2)][0..2], pos:3)
```
- `end: entry=2` → end_index=2, pointing to "i"

So `create_parent_exploration_state()` DOES use `current().path`, but the test result shows `end_index=1`!

### Wait... I Need to Recheck

Looking at test line 858 in log:
```
current: Cursor(Pattern["g"(6), "h"(7), "i"(8), "a"(0), "b"(1), "c"(2)][0..3], pos:4)
```

This is AFTER finding "ghi" (the second root). The query cursor has:
- `end: entry=3` → end_index=3, pointing to "a"

But the test failure shows `end_index=2`. So the issue is with the SECOND search starting from the first search's result!

## Revised Understanding

The test does TWO searches:
1. **First search**: Pattern ["g","h","i","a","b","c"] → Finds "ghi", returns cursor with end_index=2 (pointing to "i")
2. **Second search**: Uses returned cursor → Should find "abc", expects cursor with end_index=3 (pointing to "a")

But second search starts with cursor at end_index=2 instead of end_index=3!

The issue is that `create_parent_exploration_state()` uses:
```rust
let end_cursor = PathCursor {
    path: self.state.query.current().path.clone(),  // Has advanced end_index
    atom_position: checkpoint.atom_position,        // But position is from checkpoint!
    _state: std::marker::PhantomData::<Matched>,
};
```

This creates a mismatch:
- `path.end_index` = 2 (points to "i" - current position)
- `atom_position` = 2 (consumed 2 atoms: "g", "h")

But we matched ALL THREE tokens in "ghi", so:
- We consumed 3 atoms ("g", "h", "i")
- Next token to match is at position 3 (end_index=3, pointing to "a")

**The real bug**: When we match "ghi" as a complete token (EntireRoot), we're not updating the cursor's `atom_position` to reflect that we consumed all atoms in that root, including the advanced position!

Actually, wait. Let me trace through more carefully...

After matching "gh" and advancing query to look for "i":
- checkpoint_pos = 2 (matched "g" + "h")
- current_pos = 3 (advanced to look for "i")
- Need parent exploration

Then we find "ghi" which matches positions 0-2 completely. At this point:
- We entered at position 0 (g)
- We matched through position 2 (i)
- Total atoms consumed: 3
- Next token is at position 3 (a)

So the cursor should have:
- `atom_position` = 3
- `path.end_index` = 3

But `create_parent_exploration_state()` creates:
- `atom_position` = 2 (from checkpoint - only "g" and "h")
- `path.end_index` = 2 (from current - points to "i")

## Architectural Challenge

The core issue: **`MatchResult` assumes `cursor` is always a checkpoint (Matched state), but we need to represent advanced query cursors.**

### Current Representation

```rust
pub struct MatchResult {
    pub path: PathCoverage,           // Where we matched in graph
    pub cursor: PatternCursor,         // Query position (Matched only)
}
```

### What We Actually Need

When query advances but we don't find a match, we need to capture:
1. **Checkpoint state**: Last confirmed match (position N, end_index N)
2. **Advanced state**: Where we looked ahead (position N+1, end_index N+1)

For consecutive searches, the second search should start from the **advanced state**, not checkpoint.

## Solution Approaches

### Option 1: Add Optional Candidate Cursor to MatchResult

```rust
pub struct MatchResult {
    pub path: PathCoverage,
    pub cursor: PatternCursor,                    // Checkpoint (Matched)
    pub advanced_cursor: Option<PatternCursor>,   // Advanced query (Candidate)
}
```

**Pros:**
- Minimal API changes
- Clear separation of checkpoint vs advanced
- Backward compatible (advanced_cursor is optional)

**Cons:**
- Redundant data (both cursors share most fields)
- Two fields to manage synchronization
- Confusion about which cursor to use

**Impact:**
- **Code size**: +5% (extra field, Option handling)
- **API usability**: Medium (users must check `advanced_cursor.or(cursor)`)
- **Simplicity**: Medium (two cursor management)

### Option 2: Generic MatchResult Over Cursor State

```rust
pub struct MatchResult<Q: CursorState = Matched> {
    pub path: PathCoverage,
    pub cursor: PatternCursor<Q>,
}

pub enum MatchResultEnum {
    Matched(MatchResult<Matched>),
    Advanced(MatchResult<Candidate>),
}
```

**Pros:**
- Type-safe distinction
- Single cursor field
- No redundant data

**Cons:**
- Major API breaking change
- All existing code needs updates
- Enum adds matching overhead
- Generic complexity

**Impact:**
- **Code size**: +20% (generic monomorphization, enum matching)
- **API usability**: Low (breaks all existing code)
- **Simplicity**: Low (generics + enum complexity)

### Option 3: Store Query Cursor State in Response

```rust
pub struct Response {
    pub cache: TraceCache,
    pub end: MatchResult,              // Always checkpoint
    pub query_state: QueryState,       // New field
}

pub enum QueryState {
    Exhausted,                         // Query fully matched
    NeedsAdvancement(PatternCursor),   // Advanced cursor for next search
}
```

**Pros:**
- Keeps MatchResult simple
- Clear semantics (Response-level state)
- Non-breaking for MatchResult

**Cons:**
- Response becomes more complex
- Separation between match state and query state

**Impact:**
- **Code size**: +8% (enum + new field)
- **API usability**: Medium-High (clear enum cases)
- **Simplicity**: Medium-High (Response handles advancement)

### Option 4: Unified Cursor Position Type

```rust
pub struct CursorPosition {
    pub checkpoint: PatternCursor,      // Last matched
    pub current: PatternCursor,         // Current position (may be advanced)
}

pub struct MatchResult {
    pub path: PathCoverage,
    pub position: CursorPosition,
}
```

**Pros:**
- Mirrors `Checkpointed<C>` architecture in CompareState
- Clear checkpoint vs current semantics
- Single field for both states

**Cons:**
- More complex than current
- Breaking change
- Larger structure

**Impact:**
- **Code size**: +10% (always store both cursors)
- **API usability**: High (clear .checkpoint() and .current() accessors)
- **Simplicity**: High (consistent with internal architecture)

### Option 5: Extend PathCoverage with Advanced Query Info

```rust
pub enum PathCoverage {
    EntireRoot(IndexRangePath),
    Range(RangeEnd),
    Prefix(PrefixEnd),
    Postfix(PostfixEnd),
    AdvancedQuery {                     // New variant
        matched_path: IndexRangePath,
        checkpoint_cursor: PatternCursor,
        advanced_cursor: PatternCursor,
    },
}
```

**Pros:**
- Specifically handles advanced query case
- No changes to other variants

**Cons:**
- PathCoverage becomes less cohesive (mixes graph + query state)
- Complex variant
- Awkward enum semantics

**Impact:**
- **Code size**: +12% (large enum variant)
- **API usability**: Low (confusing mixing of concerns)
- **Simplicity**: Low (breaks enum cohesion)

## Recommendation: Option 4 (Unified Cursor Position)

### Rationale

1. **Mirrors Internal Architecture**: Matches `Checkpointed<C>` pattern used in `CompareState`
2. **Clear Semantics**: `.checkpoint()` = last match, `.current()` = exploration position
3. **Single Source of Truth**: One field manages both states
4. **Type Safety**: Both cursors are PatternCursor (no state generics needed)
5. **Migration Path**: Update accessors, internal logic stays similar

### Implementation Plan

#### Phase 1: Create CursorPosition Type (30 min)

**File**: `crates/context-search/src/cursor/position.rs` (new)

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CursorPosition {
    pub checkpoint: PatternCursor,  // Last confirmed match
    pub current: PatternCursor,     // Current exploration position
}

impl CursorPosition {
    pub fn new_matched(cursor: PatternCursor) -> Self {
        Self {
            checkpoint: cursor.clone(),
            current: cursor,
        }
    }
    
    pub fn new_advanced(checkpoint: PatternCursor, current: PatternCursor) -> Self {
        Self { checkpoint, current }
    }
    
    pub fn checkpoint(&self) -> &PatternCursor {
        &self.checkpoint
    }
    
    pub fn current(&self) -> &PatternCursor {
        &self.current
    }
    
    pub fn is_advanced(&self) -> bool {
        self.checkpoint.path != self.current.path
    }
    
    pub fn atom_position(&self) -> AtomPosition {
        self.checkpoint.atom_position
    }
}
```

#### Phase 2: Update MatchResult (45 min)

**File**: `crates/context-search/src/state/matched/mod.rs`

1. Replace `pub cursor: PatternCursor` with `pub position: CursorPosition`
2. Add convenience methods:
   - `pub fn cursor(&self) -> &PatternCursor` → `&self.position.checkpoint`
   - `pub fn checkpoint_cursor(&self) -> &PatternCursor` → `&self.position.checkpoint`
   - `pub fn current_cursor(&self) -> &PatternCursor` → `&self.position.current`
   - `pub fn is_query_advanced(&self) -> bool` → `self.position.is_advanced()`
3. Update `query_exhausted()` to use `position.current()`

#### Phase 3: Update create_parent_exploration_state() (20 min)

**File**: `crates/context-search/src/match/root_cursor/advance.rs`

```rust
pub(crate) fn create_parent_exploration_state(&self) -> MatchResult {
    let checkpoint = self.state.query.checkpoint().clone();
    let current = self.state.query.current().clone();
    
    // Use current path (advanced), checkpoint atom_position (confirmed matches)
    let position = CursorPosition::new_advanced(checkpoint, current);
    
    // ... existing path creation logic ...
    
    MatchResult { cursor: position, path: path_enum }
}
```

#### Phase 4: Update create_result_from_state() (20 min)

**File**: `crates/context-search/src/search/mod.rs`

```rust
pub(crate) fn create_result_from_state(
    &self,
    state: CompareState<Matched, Matched>,
) -> MatchResult {
    // ... existing logic ...
    
    let cursor = state.query.checkpoint().clone();
    let position = CursorPosition::new_matched(cursor);
    
    MatchResult { position, path: path_enum }
}
```

#### Phase 5: Update All Callers (60 min)

**Files**: Throughout `crates/context-search/src/`

1. Replace `.cursor()` with `.cursor()` (no change - accessor unchanged)
2. Replace `.cursor.path` with `.cursor().path` or `.position.checkpoint.path`
3. Replace `.cursor.atom_position` with `.cursor().atom_position`
4. Update test assertions to use accessor methods

#### Phase 6: Update Response API (15 min)

**File**: `crates/context-search/src/state/result.rs`

- `query_cursor()` → returns `&self.end.position.checkpoint` (or add `current_cursor()`)
- `cursor_position()` → returns `self.end.position.atom_position()`

#### Phase 7: Update Tests (30 min)

**File**: `crates/context-search/src/tests/search/consecutive.rs`

```rust
// After first search
assert_eq!(cursor.checkpoint_cursor().path end_index, 2);
assert_eq!(cursor.current_cursor().path end_index, 3); // Advanced!

// Start second search with current cursor
let query = fin1.end.current_cursor().clone();
```

### Total Effort: ~3.5 hours

### Migration Checklist

- [ ] Create `CursorPosition` type with checkpoint/current fields
- [ ] Add accessor methods: `.checkpoint()`, `.current()`, `.is_advanced()`
- [ ] Update `MatchResult` structure
- [ ] Update `create_parent_exploration_state()` to use `CursorPosition::new_advanced()`
- [ ] Update `create_result_from_state()` to use `CursorPosition::new_matched()`
- [ ] Update `Response` API methods
- [ ] Fix test assertions
- [ ] Run full test suite
- [ ] Update CHEAT_SHEET.md with new API patterns
- [ ] Update HIGH_LEVEL_GUIDE.md with cursor position concepts

## Questions for Design Decision

1. **Naming**: `CursorPosition` vs `QueryPosition` vs `CheckpointedCursor`?
   - Recommendation: `CursorPosition` (matches `Checkpointed<C>` pattern)

2. **Accessor preference**: `.cursor()` returns checkpoint or current by default?
   - Recommendation: `.cursor()` → checkpoint (backward compat), add `.current_cursor()`

3. **Breaking changes**: Is it acceptable to break `MatchResult.cursor` field access?
   - If yes: Direct field access OK
   - If no: Keep public accessor, make field private

4. **Test expectations**: Should consecutive searches start from checkpoint or current?
   - Recommendation: Current (advanced) - that's what user expects

5. **Documentation**: How prominently feature this in API docs?
   - High priority: Core concept for multi-step searches

## Ready to Proceed?

I've identified the bug location, analyzed all approaches, and recommend **Option 4: Unified Cursor Position** with a detailed 7-phase implementation plan.

**Key insights:**
- Bug is in how we represent advanced query state in responses
- Current code DOES use advanced path, but doesn't preserve it properly
- Need to track both checkpoint AND current cursor positions
- Similar to existing `Checkpointed<C>` architecture

Let me know:
1. If you agree with Option 4
2. Answers to the design questions above
3. If you want me to proceed with implementation
