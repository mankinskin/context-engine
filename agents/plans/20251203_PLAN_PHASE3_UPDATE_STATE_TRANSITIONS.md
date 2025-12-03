# Phase 3: Update State Transitions to Use Checkpointed StateAdvance

## Objective

Update `advance_query_cursor()` and related methods to use the new `StateAdvance` implementation on `Checkpointed` cursors instead of manual cursor advancement and state reconstruction.

## Context

**Current Implementation:**
- `advance_query_cursor()` calls `self.query.current_mut().advance(trav)` on the unwrapped cursor
- `advance_index_cursor()` manually calls `child_state.advance_state()` and reconstructs `Checkpointed`
- Manual `Checkpointed` construction with `checkpoint`, `candidate` fields

**After Phase 2:**
- `Checkpointed<PathCursor<P, Matched>>` implements `StateAdvance`
- `Checkpointed<ChildCursor<Matched, EndNode>>` implements `StateAdvance`
- These handle checkpoint/candidate management internally

**Goal:**
- Replace manual cursor advancement with `Checkpointed::advance_state()`
- Simplify state transition logic by delegating to the trait
- Reduce code duplication in cursor management

## Analysis

### Current Methods to Update

**1. `CompareState<Matched, Matched>::advance_query_cursor()` (transitions.rs:83-111)**

Current logic:
```rust
match self.query.current_mut().advance(trav) {
    Continue(_) => {
        let query_candidate = self.query.as_candidate();
        QueryAdvanceResult::Advanced(CompareState {
            query: query_candidate,
            // ... rest of state
        })
    },
    Break(_) => QueryAdvanceResult::Exhausted(self)
}
```

**Issue:** This mutates `current_mut()` and then calls `as_candidate()`. The new design should advance the checkpointed state directly.

**However:** Looking more closely, `advance_state()` on `Checkpointed` requires consuming self and returns a NEW `Checkpointed` with candidate set. But `advance_query_cursor()` wants to advance and convert to `Candidate` state, not keep as `Matched`.

**Key Insight:** `advance_query_cursor()` is NOT the same as `advance_state()`!
- `advance_state()` advances from checkpoint, returns `Checkpointed<Matched>` with new candidate
- `advance_query_cursor()` advances and returns `CompareState<Candidate, Matched>` (different Q state)

**Decision:** `advance_query_cursor()` should NOT use `advance_state()`. It has different semantics - it advances the query and transitions to `Candidate` state, while keeping the child in `Matched` state. This is a different operation.

**2. `CompareState<Candidate, Matched>::advance_index_cursor()` (transitions.rs:115-150)**

Current logic:
```rust
let candidate_child_cursor = self.child.current().as_candidate();
match candidate_child_cursor.child_state.advance_state(trav) {
    Ok(advanced_child_state) => {
        IndexAdvanceResult::Advanced(CompareState {
            child: Checkpointed {
                checkpoint: self.child.checkpoint().clone(),
                candidate: Some(ChildCursor {
                    child_state: advanced_child_state,
                    _state: PhantomData,
                }),
            },
            // ... rest
        })
    },
    Err(failed_child_state) => { /* similar reconstruction */ }
}
```

**Issue:** This manually constructs `Checkpointed` after advancing `child_state`. But we now have `StateAdvance` on `Checkpointed<ChildCursor<Matched, EndNode>>` which handles this internally!

**BUT:** The child cursor is currently `Checkpointed<ChildCursor<Matched, PositionAnnotated<ChildLocation>>>`, and we're at `CompareState<Candidate, Matched>`. The child is already in `Matched` state with a checkpoint.

**Problem:** We're calling `self.child.current().as_candidate()` which creates a `ChildCursor<Candidate>`, then calling `advance_state()` on its `child_state`. This is getting a candidate version of the current cursor and advancing its child_state directly.

**Can we use `self.child.advance_state()`?** YES! That would:
1. Assert candidate is None (child is at checkpoint)
2. Advance the child's child_state from checkpoint
3. Return `Ok(Checkpointed { checkpoint, candidate: Some(advanced) })` or `Err(self)`

This would simplify the logic significantly!

**3. StateAdvance impls for CompareState (transitions.rs:153-218)**

These already manually construct `Checkpointed` structures. They could potentially use the new implementations, but they have specific semantics:
- Both `Ok` and `Err` cases return `Ok(CompareState)` (always succeed, just propagate inner state)
- Different from standard `StateAdvance` pattern

**Decision:** These are fine as-is. They serve a different purpose (wrapping child state advancement in always-succeeding CompareState advancement).

## Implementation Plan

### Changes Needed

**Only one method needs updating:** `advance_index_cursor()`

### 1. Update `advance_index_cursor()` 

**File:** `crates/context-search/src/compare/state/transitions.rs`

**Current code (lines 115-150):**
```rust
pub(crate) fn advance_index_cursor<G: HasGraph>(
    self,
    trav: &G,
) -> IndexAdvanceResult<PositionAnnotated<ChildLocation>> {
    let candidate_child_cursor = self.child.current().as_candidate();
    match candidate_child_cursor.child_state.advance_state(trav) {
        Ok(advanced_child_state) => {
            IndexAdvanceResult::Advanced(CompareState {
                child: Checkpointed {
                    checkpoint: self.child.checkpoint().clone(),
                    candidate: Some(ChildCursor {
                        child_state: advanced_child_state,
                        _state: PhantomData,
                    }),
                },
                query: self.query,
                target: self.target,
                mode: self.mode,
            })
        },
        Err(failed_child_state) =>
            IndexAdvanceResult::Exhausted(CompareState {
                child: Checkpointed {
                    checkpoint: self.child.checkpoint().clone(),
                    candidate: Some(ChildCursor {
                        child_state: failed_child_state,
                        _state: PhantomData,
                    }),
                },
                query: self.query,
                target: self.target,
                mode: self.mode,
            }),
    }
}
```

**New code:**
```rust
pub(crate) fn advance_index_cursor<G: HasGraph>(
    self,
    trav: &G,
) -> IndexAdvanceResult<PositionAnnotated<ChildLocation>> {
    // Advance the checkpointed child cursor using StateAdvance
    match self.child.advance_state(trav) {
        Ok(advanced_child) => {
            // Successfully advanced the child cursor
            IndexAdvanceResult::Advanced(CompareState {
                child: advanced_child,
                query: self.query,
                target: self.target,
                mode: self.mode,
            })
        },
        Err(original_child) => {
            // Child cursor cannot advance (at boundary)
            IndexAdvanceResult::Exhausted(CompareState {
                child: original_child,
                query: self.query,
                target: self.target,
                mode: self.mode,
            })
        },
    }
}
```

**Benefits:**
- Removes manual `Checkpointed` construction
- Removes need to access `child_state` directly
- Delegates checkpoint/candidate management to `StateAdvance` trait
- Cleaner, more declarative code
- Removes TODO comment about updating positions

**Key Changes:**
- Call `self.child.advance_state(trav)` instead of manual child_state advancement
- Use returned `Checkpointed` directly instead of reconstructing
- Both success and failure cases simplified (just pass through the result)

### No Other Changes Needed

**Why `advance_query_cursor()` stays as-is:**
- It advances the query cursor AND transitions Q state from Matched → Candidate
- This is different from `StateAdvance` which keeps Q in Matched state
- The current implementation is correct for its purpose
- `as_candidate()` call creates the `Checkpointed<PathCursor<P, Candidate>>` state

**Why StateAdvance impls for CompareState stay as-is:**
- They wrap child state advancement in always-succeeding CompareState
- Both success and failure propagate as `Ok(CompareState)` with different inner states
- This is intentional behavior for the search algorithm
- Different semantics from standard StateAdvance pattern

## Testing Strategy

### Unit Tests

No new unit tests needed - Phase 2 already tested `StateAdvance` implementations.

### Integration Tests

Existing tests should continue to pass:
- `test_compare_state_candidate_advance` - Uses `StateAdvance` on `CompareState`
- `test_compare_state_matched_advance` - Uses `StateAdvance` on `CompareState`
- All advancement chain tests
- All parent compare state tests

### Regression Testing

Run full test suite to ensure no behavioral changes:
```bash
cargo test -p context-search --lib
```

Expected: 39 tests pass, 1 pre-existing failure (`find_consecutive1`)

## Validation

**Compilation:**
```bash
cargo check -p context-search
```

**Tests:**
```bash
cargo test -p context-search --lib state_advance
cargo test -p context-search --lib advance_index
```

## Risks & Mitigations

**Risk 1: Changed advancement semantics**
- Mitigation: `advance_state()` on `Checkpointed` has well-defined semantics from Phase 2, tested thoroughly

**Risk 2: Position tracking in child cursor**
- Mitigation: TODO comment mentioned updating positions, but this is handled by `ChildState::advance_state()` which already updates its path correctly

**Risk 3: Breaking existing callers**
- Mitigation: Only one method signature stays the same (`advance_index_cursor`), just implementation changes internally

## Success Criteria

✅ `advance_index_cursor()` uses `Checkpointed::advance_state()`
✅ No manual `Checkpointed` construction in `advance_index_cursor()`
✅ All existing tests continue to pass
✅ Code is simpler and more declarative
✅ TODO comment about updating positions is removed (handled by trait)

## Estimated Time

**Implementation:** 15 minutes
- Update `advance_index_cursor()`: 10 min
- Remove manual construction code: 5 min

**Testing:** 5 minutes (verify existing tests pass)

**Total:** 20 minutes

## Next Phase

After Phase 3: Phase 5 - Integrate with MatchResult (Phase 4 was merged with Phase 1)
