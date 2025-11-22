# Plan: Unified Checkpointed Cursor Architecture

**Date:** 2024-11-22  
**Status:** Phase 1 Complete, Phase 2 In Progress (40% done)  
**Priority:** High (fixes atom_position bugs + improves architecture)

## Progress Summary

**Completed:**
- ✅ Phase 1: `Checkpointed<C>` type fully implemented and compiling
- ✅ Phase 2 (partial): CompareState structure updated with new fields
- ✅ Core methods updated: mark_match, mark_mismatch, advance_query_cursor, parent_state, rooted_path

**In Progress:**
- ⏳ Phase 2: ~80 field access sites need updating in compare/state.rs
- ⏳ Phase 2: ~20 CompareState construction sites need updating

**Not Started:**
- ⬜ Phase 3: Update RootCursor
- ⬜ Phase 4: Update consumers (iterator, parent, etc.)
- ⬜ Phase 5: Testing & validation

## Objective

Replace the scattered checkpoint logic in `CompareState` with a unified `Checkpointed<T>` wrapper type that encapsulates cursor advancement and checkpoint management for both query and child cursors.

## Current Problems

1. **Scattered State**: `CompareState` has 4 separate fields:
   - `cursor: PathCursor<PatternRangePath, Q>`
   - `child_cursor: ChildCursor<I, EndNode>`
   - `checkpoint: PathCursor<PatternRangePath, Matched>`
   - `checkpoint_child: ChildCursor<Matched, EndNode>`

2. **Inconsistent Updates**: Checkpoint updates happen in multiple places:
   - `mark_match()` - updates both cursors and checkpoints
   - `create_checkpoint_state()` - extracts checkpoint
   - `advance_query()` - manipulates checkpoint position
   - `advance_child()` - propagates checkpoints

3. **Position Management Bugs**: 
   - `atom_position` calculation is inconsistent
   - Path advancement sometimes includes width, sometimes doesn't
   - Tests failing with off-by-one errors

4. **Lack of Uniformity**: Query and child cursors handled differently despite similar semantics

## Proposed Solution

### New Type: `Checkpointed<C>`

```rust
/// Encapsulates a cursor with its checkpoint state
/// Ensures cursor and checkpoint are always in sync
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Checkpointed<C> {
    /// Current cursor position (may be Candidate/Matched/Mismatched)
    pub(crate) current: C,
    
    /// Last confirmed match position (always Matched state)
    pub(crate) checkpoint: /* Matched version of C */,
}

impl<C> Checkpointed<C> {
    /// Create from initial position (cursor = checkpoint = initial)
    pub(crate) fn new(initial: /* Matched version of C */) -> Self;
    
    /// Mark current position as matched, updating checkpoint
    pub(crate) fn mark_match(self) -> Checkpointed</* Matched version of C */>;
    
    /// Mark current position as mismatched, keeping checkpoint
    pub(crate) fn mark_mismatch(self) -> Checkpointed</* Mismatched version of C */>;
    
    /// Convert current to Candidate state (for next comparison)
    pub(crate) fn as_candidate(&self) -> Checkpointed</* Candidate version of C */>;
    
    /// Get checkpoint (always Matched)
    pub(crate) fn checkpoint(&self) -> &/* Matched version of C */;
    
    /// Get current cursor
    pub(crate) fn current(&self) -> &C;
}
```

### Updated `CompareState`

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CompareState<
    Q: CursorState = Candidate,
    I: CursorState = Candidate,
    EndNode: PathNode = PositionAnnotated<ChildLocation>,
> {
    /// Query cursor with checkpoint
    pub(crate) query: Checkpointed<PathCursor<PatternRangePath, Q>>,
    
    /// Index cursor with checkpoint
    pub(crate) child: Checkpointed<ChildCursor<I, EndNode>>,
    
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
```

## Implementation Steps

### Phase 1: Create Checkpointed Type (New File) ✅ COMPLETE

1. ✅ Create `crates/context-search/src/cursor/checkpointed.rs`
2. ✅ Define `Checkpointed<C>` generic type
3. ✅ Implement for `PathCursor<P, S>` where S: CursorState
4. ✅ Implement for `ChildCursor<S, EndNode>`
5. ✅ Add conversion methods (mark_match, mark_mismatch, as_candidate)
6. ✅ Add checkpoint access methods
7. ✅ Add missing as_candidate() methods to cursor types

### Phase 2: Update CompareState Structure ⏳ IN PROGRESS

1. ✅ Replace 4 fields with 2 `Checkpointed` fields in `compare/state.rs`
2. ✅ Update `mark_match()` to use `query.mark_match()` and `child.mark_match()`
3. ✅ Update `mark_mismatch()` similarly
4. ✅ Update `advance_query_cursor()` to work with `Checkpointed`
5. ⏳ Update all CompareState construction sites (~20+ locations)
6. ⏳ Update all field accesses (`.cursor` → `.query.current()`, etc.) (~80+ locations)
7. ⏳ Update `compare_and_match()` method
8. ⏳ Update `prefix_states()` method
9. ⏳ Update `advance_index_cursor()` (if exists)

**Remaining work in Phase 2:**
- CompareState struct instantiations need field name updates:
  - `cursor:` → `query: Checkpointed::new()` or similar
  - `child_cursor:` → `child: Checkpointed::new()` or similar
  - Remove `checkpoint:` and `checkpoint_child:` fields
- All `.cursor`, `.child_cursor`, `.checkpoint`, `.checkpoint_child` accesses need updating:
  - `.cursor` → `.query.current()`
  - `.child_cursor` → `.child.current()`
  - `.checkpoint` → `.query.checkpoint()`
  - `.checkpoint_child` → `.child.checkpoint()`

### Phase 3: Update RootCursor

1. Update `root_cursor.rs` to use `CompareState` with `Checkpointed` fields
2. Simplify `create_checkpoint_state()` - just extract `query.checkpoint()` and `child.checkpoint()`
3. Remove manual checkpoint management logic
4. Fix `advance_query()` to not manually adjust atom_position

### Phase 4: Update Consumers

1. Update `match/iterator.rs` to access cursors via `.current()` and `.checkpoint()`
2. Update `compare/parent.rs` similarly
3. Update all test assertions to use new structure
4. Fix any remaining compilation errors

### Phase 5: Testing & Validation

1. Run all `find_ancestor*` tests - should now pass
2. Run `range*`, `prefix*`, `postfix*` tests
3. Run `find_consecutive*` tests
4. Verify no regressions in other tests
5. Check log output for correct atom_position values

## Benefits

1. **Single Source of Truth**: Checkpoint logic centralized in one type
2. **Type Safety**: Can't accidentally use wrong cursor or checkpoint
3. **Clearer Intent**: `query.mark_match()` vs manually updating 4 fields
4. **Bug Fix**: Fixes atom_position off-by-one errors
5. **Easier Debugging**: Checkpoint state always visible together with cursor
6. **Uniformity**: Query and child handled identically

## Risks & Mitigations

**Risk**: Large refactor touching many files  
**Mitigation**: Phase implementation, test after each phase

**Risk**: Breaking existing tests  
**Mitigation**: Many tests already broken, this fixes them

**Risk**: Performance impact from wrapper type  
**Mitigation**: Zero-cost abstraction (no heap allocation), compiler will inline

**Risk**: Complex generic constraints  
**Mitigation**: Keep constraints minimal, use associated types where needed

## Validation Criteria

- [ ] All `find_ancestor*` tests pass (currently 2 failing)
- [ ] All `range*`, `prefix*`, `postfix*` tests pass (currently 3 failing)
- [ ] All `find_consecutive*` tests pass
- [ ] No regressions in passing tests
- [ ] Checkpoint access is uniform across query and child
- [ ] No manual atom_position manipulation outside `Checkpointed`
- [ ] Code is more readable and maintainable

## Related Files

- `crates/context-search/src/cursor/mod.rs` - cursor types
- `crates/context-search/src/cursor/checkpointed.rs` - NEW
- `crates/context-search/src/compare/state.rs` - CompareState
- `crates/context-search/src/match/root_cursor.rs` - RootCursor
- `crates/context-search/src/match/iterator.rs` - iterator
- Tests in `crates/context-search/src/tests/`

## Questions for Review

1. Should `Checkpointed` be generic over cursor type or have specific impls?
   - **Decision**: Generic with trait bounds, more flexible
   
2. How to handle state transitions (Candidate↔Matched↔Mismatched)?
   - **Decision**: Methods return new `Checkpointed` with updated state type
   
3. Should checkpoint be mutable or immutable?
   - **Decision**: Immutable, updated only via `mark_match()`
   
4. How to handle the different cursor types (PathCursor vs ChildCursor)?
   - **Decision**: Generic type works for both, different impls if needed

## Next Steps

1. Get approval for approach
2. Implement Phase 1 (new type)
3. Test Phase 1 in isolation
4. Proceed with Phases 2-5 sequentially
