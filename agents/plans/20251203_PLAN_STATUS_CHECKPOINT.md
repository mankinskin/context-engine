---
tags: `#plan` `#context-search` `#debugging` `#testing` `#api`
summary: Began implementing the plan to add support for advanced query cursors in `MatchResult` by making `Checkpointed<C>` space-efficient.
status: 📋
---

# Implementation Status Checkpoint

## Session Summary

Began implementing the plan to add support for advanced query cursors in `MatchResult` by making `Checkpointed<C>` space-efficient.

## Completed

### Phase 1: Enhance Checkpointed Core ✅ (PARTIALLY)

**File:** `crates/context-search/src/cursor/checkpointed.rs`

**Changes Made:**
1. ✅ Changed structure from `current: C` → `candidate: Option<C>`
2. ✅ Added `CheckpointedRef<'a, C>` enum for unified current access
3. ✅ Updated `new()` constructors to set `candidate = None`
4. ✅ Updated `as_candidate()` to handle Option
5. ✅ Updated `mark_match()` to set `candidate = None`
6. ✅ Updated `mark_mismatch()` to preserve candidate in Option
7. ✅ Added `HasCheckpoint::from_checkpoint()` method for type conversion
8. ✅ Updated Display/CompactFormat implementations

**Compatibility Layer Added:**
- `current()` - Returns `&C` using unsafe cast (temporary, for backwards compat)
- `current_full()` - Returns `CheckpointedRef<C>` enum (proper way)
- `current_mut()` - Materializes candidate if needed
- `at_checkpoint()` - Check if candidate is None

## Blocking Issue

**Compilation Error:** Type inference failure in `CompareLeafResult` and `CompareEndResult`

The `#[derive(Clone)]` on these types is failing because the compiler can't infer that `CompareState<Matched, Matched, EndNode>` implements Clone. This is likely related to:

1. The unsafe cast in `current()` method
2. Or missing Clone bounds somewhere in the trait hierarchy

**Error Location:**
```
crates\context-search\src\compare\state\core.rs:174:14
crates\context-search\src\compare\state\core.rs:184:16
```

## What Needs to Be Done

### Option A: Fix Type Inference (Recommended)

1. Remove the unsafe cast in `current()` 
2. Instead, make `candidate` always `Some(C)` and track equality separately:
   ```rust
   pub(crate) struct Checkpointed<C: HasCheckpoint> {
       checkpoint: C::Checkpoint,
       candidate: C,  // Always exists
       at_checkpoint: bool,  // True when candidate == checkpoint
   }
   ```
3. This loses some space efficiency but avoids Option complexity

### Option B: Fix Clone Bounds

1. Keep Option-based approach
2. Add explicit Clone bounds to CompareState/CompareLeafResult/CompareEndResult
3. Figure out why type inference is failing

### Option C: Simpler Backwards-Compatible current()

Instead of unsafe cast, store a "materialized" candidate:
```rust
pub(crate) fn current(&self) -> &C {
    // Materialize on first access and cache
    self.materialized_candidate.get_or_insert_with(|| {
        match &self.candidate {
            Some(c) => c.clone(),
            None => C::from_checkpoint(&self.checkpoint),
        }
    })
}
```

But this requires adding a `Cell` or similar for interior mutability.

## Remaining Phases

### Phase 2: Implement StateAdvance (NOT STARTED)
- Implement `StateAdvance` for `Checkpointed<PathCursor<P, Matched>>`
- Implement `StateAdvance` for `Checkpointed<ChildCursor<Matched, EndNode>>`
- Returns `Ok(advanced)` with `Some(candidate)` or `Err(original)` when exhausted

### Phase 3: Update State Transitions (NOT STARTED)
- Update `advance_query_cursor()` to use `advance_state()`
- Handle Result pattern matching
- Update any other advancement call sites

### Phase 4: Update All Callers (NOT STARTED)
- ~51 call sites use `.current()` on checkpointed cursors
- Need to either:
  - Keep unsafe compatibility layer, OR
  - Migrate all to use `current_full()` and pattern match on `CheckpointedRef`

### Phase 5: Integrate with MatchResult (NOT STARTED)
- Fix `create_parent_exploration_state()` to use current cursor position
- Update `create_result_from_state()` to handle `CheckpointedRef`
- Ensure `atom_position` alignment

### Phase 6: Update Response API (NOT STARTED)
- Add `current_cursor()` method
- Add `has_advanced_query()` helper

### Phase 7: Fix Tests (NOT STARTED)
- Update `consecutive.rs` to use proper cursor for second search
- Verify `find_consecutive1` passes (end_index=3)

### Phase 8: Documentation (NOT STARTED)
- Update CHEAT_SHEET.md
- Update HIGH_LEVEL_GUIDE.md
- Update inline documentation

## Recommendation for Next Session

**Start Fresh with Option A:**

1. Revert checkpointed.rs changes (git checkout)
2. Implement simpler version where `candidate: C` always exists
3. Add `at_checkpoint: bool` flag
4. This avoids Option complexity and type inference issues
5. Space cost: 1 byte per Checkpointed (still saves memory vs duplicating full cursor)

**Alternatively, Debug Option B:**

1. Add explicit Clone bounds to resolve type inference
2. Keep Option-based approach for maximum space efficiency
3. Fix or remove unsafe cast in current()

## Files Modified

- `crates/context-search/src/cursor/checkpointed.rs` - Core changes
- `agents/plans/20251127_PLAN_EFFICIENT_CHECKPOINTED_CURSOR.md` - Updated with StateAdvance decision

## Test Status

- **Before changes:** 39/40 tests passing (find_consecutive1 failing)
- **After changes:** Won't compile due to type inference error
- **Expected after fix:** All phases needed before tests pass

## Time Spent

- ~60 minutes on Phase 1 (including debugging)
- ~9 hours remaining work (8 phases)

## Git Status

**Branch:** main (no branch created yet)  
**Uncommitted changes:** checkpointed.rs modified  
**Recommendation:** Either commit checkpoint or revert before next session
