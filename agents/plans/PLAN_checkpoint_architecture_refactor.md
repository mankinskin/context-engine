# Plan: Checkpoint Architecture Refactor

**Date:** 2025-11-23  
**Status:** Planning  
**Objective:** Centralize checkpoint management to eliminate synchronization issues and clarify cursor position semantics

## Problem Analysis

### Current Issues

1. **Scattered Checkpoint State**
   - Each `CompareState` has `Checkpointed<PathCursor<..>>` for query and child
   - Each `RootCursor` wraps `CompareState` 
   - Manual synchronization across `mark_match()`, `advance_index_cursor()`, `advance_query_cursor()`
   - Bug: Adding last token width inconsistently (works for root matching, breaks parent exploration)

2. **Ambiguous Position Semantics**
   - `atom_position` unclear: AT token (Candidate) vs PAST token (Matched)?
   - Current behavior mixes both depending on context
   - No clear contract when position includes last matched token width

3. **Global vs Local Checkpoints**
   - **Global checkpoint**: `best_match` in search fold (best result so far)
   - **Local checkpoints**: In each `Checkpointed<Cursor>` (last confirmed match in this branch)
   - Confusion between "what globally matched best" vs "where this cursor last matched"

### Root Cause

The checkpoint serves two purposes that should be separate:
- **Progress tracking**: How far has THIS comparison progressed successfully?
- **Best result tracking**: What's the BEST match found globally across all candidates?

Currently mixing these in `Checkpointed<Cursor>` with manual sync.

## Proposed Architecture

### Phase 1: Clarify Position Semantics

**Define clear contracts:**
- `Candidate` cursor: `atom_position` = cumulative width consumed UP TO (not including) current token
- `Matched` cursor: `atom_position` = cumulative width consumed INCLUDING matched token
- Transition `Candidate → Matched`: Add matched token width

**Changes:**
- Document this in cursor state types
- Update `mark_match()` to explicitly add token width
- Remove ad-hoc width additions in `advance_query_cursor`

### Phase 2: Centralize Global Checkpoint

**Move global checkpoint to `SearchState`:**
```rust
pub struct SearchState<K: SearchKind> {
    pub(crate) matches: SearchIterator<K>,
    pub(crate) query: PatternRangePath,
    pub(crate) best_checkpoint: Option<MatchResult>,  // NEW: Global best match
}
```

**Access pattern:**
- `SearchState` owns the global checkpoint
- Compare states READ checkpoint via reference (no cloning)
- Only `SearchState` WRITES checkpoint (after validating new match is better)
- Clear queue when better match found (already doing this)

### Phase 3: Simplify Local Checkpoints

**Checkpointed cursors track LOCAL progress only:**
- Not synced across states
- Only updated when THIS cursor confirms a match
- Used for backtracking within this comparison branch

**Simplify `CompareState`:**
- Remove checkpoint sync logic from `mark_match()`
- Remove checkpoint updates from `advance_*_cursor()`
- Let each branch manage its own checkpoint independently

### Phase 4: Add Unit Tests

**Test compare state transitions:**
- `test_candidate_to_matched_position()` - Verify width added
- `test_mark_match_semantics()` - Verify checkpoint update
- `test_advance_query_exhausted()` - Verify no double-counting
- `test_advance_index_exhausted()` - Verify child exhaustion handling

**Test checkpoint isolation:**
- `test_parallel_candidates_independent()` - Verify branches don't interfere
- `test_global_checkpoint_best_match()` - Verify best match selection

## Implementation Strategy

### Immediate Fix (Current Session)

For now, fix the immediate bug with minimal changes:

**Issue**: When `QueryExhausted` happens:
- In parent exploration: checkpoint already updated (position = 3) ✓
- In root matching: checkpoint NOT updated (position = 5, should be 6) ✗

**Solution**: Check if we're in a Matched or Candidate state:
- If query cursor is `Matched`: position already includes last token → don't add width
- If query cursor is `Candidate`: position doesn't include last token → add width

**Code location**: `crates/context-search/src/compare/state/transitions.rs:109`

### Long-term Refactor (Next Session)

1. Create `agents/plans/PLAN_checkpoint_refactor_execution.md` with detailed steps
2. Implement Phase 1 (position semantics) - ~2-3 files
3. Implement Phase 2 (global checkpoint) - ~4-5 files  
4. Implement Phase 3 (local checkpoints) - ~3-4 files
5. Add Phase 4 (unit tests) - new test file

**Estimated scope**: 10-15 files, ~500-800 LOC changes

## Current Session Summary (2025-11-23)

### Work Done

1. **Fixed 17 tests**: Token label display issue (moved `init_test_tracing!()` after graph population)
2. **Fixed find_ancestor2/3**: Removed incorrect manual width addition in `advance_query` 
3. **Fixed find_sequence/find_ancestor1_long_pattern**: Used `current()` instead of `checkpoint()` for QueryExhausted
4. **Identified root cause**: `mark_match()` doesn't update position, but should
5. **Updated `advance_index_cursor`**: Now adds token width when updating checkpoint

### Current Status

**31 of 35 tests passing**

**Still failing (4 tests):**
- `find_consecutive1`
- `find_pattern1`  
- `prefix1`
- `range1`

**Root cause identified**: Position semantics are fundamentally broken:
- `mark_match()` changes `Candidate → Matched` but doesn't update `atom_position`
- Position should be AT token (Candidate) vs PAST token (Matched)
- Currently position stays the same regardless of state
- Leads to off-by-one errors when QueryExhausted

**Why the fix is complex:**
- Some tokens matched via `advance_index_cursor` (both cursors advance)
- Other tokens matched as roots in parent exploration (no `advance_index_cursor` call)
- Different code paths update checkpoints differently
- No unified place to add token width consistently

### Attempted Fixes (All Failed)

1. ❌ Add width in `advance_query_cursor` when QueryExhausted → Broke find_ancestor2/3 
2. ❌ Conditionally add width only if `checkpoint != current` → Didn't work (both equal)
3. ❌ Don't add width (assume Matched includes it) → Find_consecutive1 still broken
4. ❌ Add width in `advance_index_cursor` when updating checkpoint → Didn't help (not called for last token in some cases)
5. ❌ Copy checkpoint position to current in QueryExhausted → Still position 5 instead of 6

### Key Insights

1. **Token matching has multiple code paths:**
   - Path A: Parent exploration → `advance_index_cursor` → checkpoint updated
   - Path B: Found root → `advance_to_end` → NO `advance_index_cursor` call
   
2. **In find_consecutive1:**
   - Query: `["g", "h", "i", "a", "b", "c"]`
   - Tokens "g"-"b" matched via parent exploration (Path A)
   - Token "c" found as root "abc" (Path B) → checkpoint NOT updated for "c"
   - QueryExhausted happens with checkpoint at 5 (after "b"), should be 6 (after "c")

3. **The fundamental issue:**
   - `mark_match()` is called but doesn't update position
   - Need to add token width somewhere, but no single chokepoint
   - Different paths need different fixes

## Recommended Approach

This is too complex to fix incrementally. Need the full refactor outlined in this plan.

**Next session should:**
1. Create detailed execution plan
2. Fix `mark_match()` to take `&Trav` parameter and add token width
3. Update all call sites
4. Remove manual width additions everywhere else
5. Add comprehensive unit tests

**Estimated effort:** 2-3 hours with full test suite verification

## Questions for Author

1. **Position semantics**: Confirm Candidate (AT) vs Matched (PAST) interpretation correct?
2. **Global checkpoint**: Should it live in `SearchState` or separate `CheckpointManager`?
3. **Queue clearing**: Currently clear on better match - keep this behavior?
4. **Parallel candidates**: Graph invariants guarantee single match - always clear queue?
5. **Test coverage**: Which scenarios are most critical to test?

## Related Files

- `crates/context-search/src/compare/state/transitions.rs` - mark_match, advance cursors
- `crates/context-search/src/match/root_cursor/advance.rs` - root matching logic
- `crates/context-search/src/match/root_cursor/state.rs` - create_end_state
- `crates/context-search/src/search/mod.rs` - global checkpoint tracking
- `crates/context-trace/src/cursor/checkpointed.rs` - Checkpointed wrapper

## Success Criteria

- [ ] All 35 context-search tests pass
- [ ] No manual checkpoint synchronization code
- [ ] Clear position semantics documented
- [ ] Global checkpoint centralized
- [ ] Unit tests for compare state transitions
- [ ] No regression in existing tests
