---
tags: `#plan` `#context-trace` `#context-search` `#algorithm` `#debugging` `#testing` `#performance`
summary: Implement `StateAdvance` trait for `Checkpointed<PathCursor<P, Matched>>` and `Checkpointed<ChildCursor<Matched, EndNode>>` to enable advancing cur...
status: 📋
---

# Phase 2: Checkpointed StateAdvance Implementation Plan

## Objective

Implement `StateAdvance` trait for `Checkpointed<PathCursor<P, Matched>>` and `Checkpointed<ChildCursor<Matched, EndNode>>` to enable advancing cursors while maintaining checkpoint state.

## Context

**Current State:**
- Phase 1 complete: `Checkpointed<C>` structure with `candidate: Option<C>` is working
- Library and tests compile successfully
- `StateAdvance` trait exists in `context-trace`: `trait StateAdvance: Sized + Clone { type Next; fn advance_state<G: HasGraph>(self, trav: &G) -> Result<Self::Next, Self>; }`

**StateAdvance Contract:**
- Consumes `self` (ownership transfer)
- Returns `Result<Self::Next, Self>` - either advanced state or original state on failure
- Used for cursor advancement throughout the search algorithms

**Current Usage Sites:**
1. `CompareState<Candidate, Candidate>::advance_state()` - advances child cursor via `child_state.advance_state()`
2. `CompareState<Matched, Matched>::advance_state()` - advances child cursor via `child_state.advance_state()`
3. `ParentCompareState::advance_state()` - advances parent state via `parent_state.advance_state()`
4. Various test sites checking advancement behavior

## Design

### Key Insight: Checkpointed Advancement Pattern

**CRITICAL:** `StateAdvance` should only advance from checkpoint, NOT from existing candidates!

**Reasoning:**
- After matching a candidate, we update `best_match` and start a new search cycle
- New SearchNodes are created from the updated checkpoint
- Candidates are temporary comparison states, not advancement states
- Path appending (for leaf specificity during comparison) is separate from advancement

**Single Case Advancement:**

When advancing a `Checkpointed<C>` cursor:

1. **Assert:** `candidate.is_none()` (only advance from checkpoint)
2. Convert checkpoint to candidate state: `checkpoint.as_candidate()` 
3. Advance the candidate cursor: `candidate.advance(trav)`
4. If successful → Return `Ok(Checkpointed { checkpoint: old_checkpoint, candidate: Some(advanced) })`
5. If failed → Return `Err(self)` (original checkpointed state)

**Result invariant:** 
- `Ok(advanced)` always has `candidate: Some(...)` (just advanced beyond checkpoint)
- `Err(original)` has `candidate: None` (at checkpoint, cannot advance)

## Implementation

### 1. Implement StateAdvance for Checkpointed<PathCursor<P, Matched>>

**Location:** `crates/context-search/src/cursor/checkpointed.rs`

**Requirements:**
- `P: Clone + Advance` (where `Advance` is `MovePath<Right, End>` from context-trace)
- Consumes `self`, returns `Result<Checkpointed<PathCursor<P, Matched>>, Self>`
- Type stays `Matched` state (cursor remains in matched state, just at different position)

**Implementation:**

```rust
impl<P> context_trace::trace::state::StateAdvance 
    for Checkpointed<PathCursor<P, Matched>>
where
    P: Clone + context_trace::path::mutators::move_path::advance::Advance,
{
    type Next = Self;
    
    fn advance_state<G: context_trace::HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self::Next, Self> {
        // Only advance from checkpoint state
        debug_assert!(
            self.candidate.is_none(),
            "advance_state should only be called when at checkpoint"
        );
        
        // Convert checkpoint to candidate and advance
        let mut candidate = CursorStateMachine::to_candidate(&self.checkpoint);
        match candidate.advance(trav) {
            std::ops::ControlFlow::Continue(()) => {
                // Successfully advanced - return with new candidate
                Ok(Checkpointed {
                    checkpoint: self.checkpoint,
                    candidate: Some(candidate.mark_match()),
                })
            },
            std::ops::ControlFlow::Break(()) => {
                // Cannot advance from checkpoint
                Err(self)
            },
        }
    }
}
```

**Key Points:**
- Uses `debug_assert!` to enforce "only advance from checkpoint" invariant
- Converts to `Candidate` state for advancement, then back to `Matched` via `mark_match()`
- Returns `Ok` with `candidate: Some(advanced)` or `Err(self)` with `candidate: None`
- No handling of "already advanced" case - that's an error condition

### 2. Implement StateAdvance for Checkpointed<ChildCursor<Matched, EndNode>>

**Location:** `crates/context-search/src/cursor/checkpointed.rs`

**Requirements:**
- `EndNode: PathNode + Clone`
- Advances the underlying `ChildState<EndNode>` via its `StateAdvance` impl
- Similar pattern to PathCursor but delegates to `child_state.advance_state()`

**Implementation:**

```rust
impl<EndNode: PathNode> context_trace::trace::state::StateAdvance 
    for Checkpointed<ChildCursor<Matched, EndNode>>
where
    EndNode: Clone,
{
    type Next = Self;
    
    fn advance_state<G: context_trace::HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self::Next, Self> {
        // Only advance from checkpoint state
        debug_assert!(
            self.candidate.is_none(),
            "advance_state should only be called when at checkpoint"
        );
        
        // Advance checkpoint's child_state
        let child_state = self.checkpoint.child_state.clone();
        match child_state.advance_state(trav) {
            Ok(advanced_state) => {
                Ok(Checkpointed {
                    checkpoint: self.checkpoint,
                    candidate: Some(ChildCursor {
                        child_state: advanced_state,
                        _state: std::marker::PhantomData,
                    }),
                })
            },
            Err(_failed_state) => {
                // Cannot advance from checkpoint
                Err(self)
            },
        }
    }
}
```

**Key Points:**
- Uses `debug_assert!` to enforce "only advance from checkpoint" invariant
- Delegates to `ChildState::advance_state()` which already exists
- Returns `Ok` with `candidate: Some(advanced)` containing new `ChildState`
- Uses `PhantomData` for `Matched` state marker
- No handling of "already advanced" case - that's an error condition

### 3. Add Required Imports

**Location:** `crates/context-search/src/cursor/checkpointed.rs`

Add to imports section:
```rust
use context_trace::{
    path::mutators::move_path::advance::Advance,
    trace::state::StateAdvance,
    HasGraph,
};
```

### 4. Update mod.rs to Export StateAdvance Usage

**Location:** `crates/context-search/src/cursor/mod.rs`

Ensure trait is accessible where needed (check if re-export needed).

## Testing Strategy

### Unit Tests (in `checkpointed.rs` test module)

1. **Test PathCursor advancement at checkpoint:**
   - Create `Checkpointed<PathCursor<PatternRangePath, Matched>>` with `candidate: None`
   - Call `advance_state()`
   - Verify: `Ok` with `candidate: Some(advanced)`, checkpoint unchanged

2. **Test PathCursor advancement failure at end:**
   - Create cursor at end of pattern with `candidate: None`
   - Call `advance_state()`
   - Verify: `Err(original)` with `candidate: None`

3. **Test ChildCursor advancement at checkpoint:**
   - Create `Checkpointed<ChildCursor<Matched, ChildLocation>>` with `candidate: None`
   - Call `advance_state()`
   - Verify: `Ok` with `candidate: Some(advanced)`, checkpoint unchanged

4. **Test ChildCursor advancement failure:**
   - Create cursor at boundary with `candidate: None`
   - Call `advance_state()`
   - Verify: `Err(original)` with `candidate: None`

5. **Test debug_assert with existing candidate (debug build only):**
   - Create `Checkpointed` with `candidate: Some(advanced_cursor)`
   - Call `advance_state()` in debug build
   - Verify: Panics with assertion message

### Integration Tests (existing tests should pass)

After implementation, verify:
- `test_compare_state_candidate_advance` still passes (uses `child_state.advance_state()`)
- `test_compare_state_matched_advance` still passes
- All state advancement chain tests pass

## Validation

**Compilation:**
```bash
cargo check -p context-search
```

**Tests:**
```bash
cargo test -p context-search checkpointed
cargo test -p context-search state_advance
```

**Expected behavior:**
- New unit tests pass
- Existing state advancement tests continue to pass
- No compilation errors related to trait bounds

## Risks & Mitigations

**Risk 1: Trait bound complexity**
- Mitigation: Use fully qualified trait paths, check existing `StateAdvance` impls for guidance

**Risk 2: Calling advance_state on already-advanced checkpointed**
- Mitigation: Use `debug_assert!` to catch this in development. Document that callers must ensure `candidate.is_none()` before calling.

**Risk 3: State restoration on failure**
- Mitigation: Simple - just return `Err(self)` since we only advance from checkpoint

**Risk 4: mark_match() semantics**
- Mitigation: For PathCursor, we convert Candidate→Matched after advancing. This is correct for creating the initial candidate from checkpoint.

## Success Criteria

✅ Both `StateAdvance` implementations compile without errors
✅ Unit tests pass for all advancement scenarios (success, failure, at-checkpoint, already-advanced)
✅ Existing integration tests continue to pass
✅ No performance regressions in advancement hot paths
✅ Clear tracing output shows advancement behavior

## Estimated Time

**Implementation:** 45 minutes
- PathCursor impl: 20 min
- ChildCursor impl: 15 min  
- Imports and exports: 5 min
- Unit tests: 30 min (included in Phase 7 testing time)

**Next Phase:** Phase 3 - Update state transitions to use new `advance_state()` on Checkpointed
