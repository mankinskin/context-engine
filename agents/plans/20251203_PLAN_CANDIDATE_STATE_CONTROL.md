---
tags: `#plan` `#context-search` `#algorithm` `#testing` `#refactoring` `#api`
summary: The current `Checkpointed<C>` type uses `CheckpointedRef` enum to provide unified access to either the checkpoint or candidate cursor. However, thi...
status: 📋
---

# Plan: Candidate State Control Refactoring

## Problem Statement

The current `Checkpointed<C>` type uses `CheckpointedRef` enum to provide unified access to either the checkpoint or candidate cursor. However, this design has issues:

1. **State Information Loss**: `CheckpointedRef` returns owned/borrowed values without encoding whether we're at checkpoint or have a candidate
2. **Unnecessary Cloning**: `CheckpointedRef::Checkpoint(C)` requires cloning because `C::Checkpoint` and `C` are different types
3. **Runtime Ambiguity**: We know at compile time whether a candidate exists, but can't express this in the type system

## Current Implementation Analysis

### 1. Checkpointed Type Structure

```rust
pub struct Checkpointed<C: HasCheckpoint> {
    checkpoint: C::Checkpoint,  // Always Matched state
    candidate: Option<C>,       // None = at checkpoint, Some = advanced
}
```

**Current Methods:**
- `checkpoint() -> &C::Checkpoint` - access checkpoint
- `current() -> CheckpointedRef<C>` - unified access (problematic)
- `current_mut() -> &mut C` - mutable access (materializes candidate)
- `at_checkpoint() -> bool` - runtime check

**Current CheckpointedRef Enum:**
```rust
pub enum CheckpointedRef<'a, C> {
    Checkpoint(C),      // Owns converted cursor (requires clone)
    Candidate(&'a C),   // Borrows candidate
}
```

### 2. PathCursor Structure

```rust
pub struct PathCursor<P, State = Matched> {
    path: P,
    atom_position: AtomPosition,
    _state: PhantomData<State>,
}
```

**CursorState Markers:**
- `Matched` - confirmed match position
- `Candidate` - exploring ahead, needs comparison
- `Mismatched` - failed to match

**Key Pattern:** PathCursor uses phantom type to track cursor state, enabling:
- Type-safe state transitions via `mark_match()` / `mark_mismatch()`
- Different method sets per state
- Compile-time guarantees

### 3. CompareState Structure

```rust
pub struct CompareState<
    Q: CursorState = Candidate,
    I: CursorState = Candidate,
    EndNode: PathNode = PositionAnnotated<ChildLocation>,
> {
    query: Checkpointed<PathCursor<PatternRangePath, Q>>,
    child: Checkpointed<ChildCursor<I, EndNode>>,
    target: DownKey,
    mode: PathPairMode,
}
```

**Key Variants:**
- `CompareState<Matched, Matched>` - both cursors at confirmed match
- `CompareState<Candidate, Candidate>` - both exploring ahead
- `CompareState<Candidate, Matched>` - query advanced, child at checkpoint
- `CompareState<Mismatched, Mismatched>` - both failed

**State Transitions:**
```rust
impl MarkMatchState for CompareState<Candidate, Candidate> {
    type Matched = CompareState<Matched, Matched>;
    fn mark_match(self) -> Self::Matched {
        CompareState {
            query: self.query.mark_match(),  // Checkpointed<...Candidate> -> Checkpointed<...Matched>
            child: self.child.mark_match(),
            ...
        }
    }
}
```

### 4. MatchResult Structure

```rust
pub struct MatchResult {
    path: PathCoverage,
    cursor: Checkpointed<PatternCursor>,  // Always at checkpoint after match
}
```

**Usage:**
- Represents a confirmed match result
- `cursor()` returns `CheckpointedRef` to access position
- Used as `best_match` in search algorithm

### 5. Search Algorithm & best_match

**SearchState::search() flow:**
```rust
let mut best_match: Option<MatchResult> = None;

while let Some(matched_state) = self.next() {
    matched_state.trace(&mut trace_ctx);
    best_match = Some(matched_state);  // Update with latest match
}

let end = best_match.unwrap_or_else(|| create_empty_mismatch());
```

**finish_root_cursor() flow:**
```rust
let mut last_match = init_state;  // CompareState<Matched, Matched>

loop {
    last_match.update_checkpoint();  // Sync candidate → checkpoint
    match root_cursor.advance_to_next_match() {
        RootAdvanceResult::Advanced(next_match) => {
            last_match = next_match.state;  // Update best_match
        }
        RootAdvanceResult::Finished(end_result) => {
            match end_result {
                ConclusiveEnd::Exhausted => break last_match,
                ConclusiveEnd::Mismatch => break last_match,
                InconclusiveEnd => {
                    // Parent exploration needed
                    let parent_result = create_parent_exploration_state();
                    return parent_result;  // Has advanced candidate
                }
            }
        }
    }
}
```

**best_match Configurations:**

1. **At Checkpoint (No Candidate)**
   - State: `Checkpointed<PathCursor<P, Matched>>` with `candidate = None`
   - Meaning: Cursor position equals last confirmed match
   - Example: Just after `mark_match()`, before any advancement
   - Used in: Final result, checkpoint storage

2. **Advanced (Has Candidate)**
   - State: `Checkpointed<PathCursor<P, S>>` with `candidate = Some(...)`
   - Meaning: Cursor has explored ahead of checkpoint
   - Example: Parent exploration state, mid-comparison
   - Used in: Active search, state transitions

### 6. StateAdvance Trait Pattern

```rust
pub trait StateAdvance: Sized + Clone {
    type Next;
    fn advance_state<G: HasGraph>(self, trav: &G) -> Result<Self::Next, Self>;
}
```

**Usage Pattern:**
```rust
impl StateAdvance for CompareState<Matched, Matched> {
    type Next = Self;
    fn advance_state<G: HasGraph>(self, trav: &G) -> Result<Self, Self> {
        // Try to advance child cursor
        match child.advance() {
            Ok(advanced) => Ok(CompareState { child: advanced, .. }),
            Err(failed) => Err(self),  // Return original state
        }
    }
}
```

## Proposed Solution: CandidateState Type Parameter

### Design Goals

1. **Type-Safe Candidate Presence**: Encode at compile time whether candidate exists
2. **Zero-Cost Abstraction**: No runtime overhead vs current implementation
3. **Direct Access**: Access candidate without conversion or cloning
4. **State Transitions**: Mirror PathCursor/CompareState pattern

### Proposed CandidateState Trait

```rust
/// Marker trait for candidate state control
pub trait CandidateState: 'static {}

/// At checkpoint - no candidate exists
pub struct AtCheckpoint;
impl CandidateState for AtCheckpoint {}

/// Advanced - candidate exists
pub struct HasCandidate;
impl CandidateState for HasCandidate {}
```

### Proposed Checkpointed Type

```rust
pub struct Checkpointed<C: HasCheckpoint, S: CandidateState = AtCheckpoint> {
    checkpoint: C::Checkpoint,
    candidate: Option<C>,  // Internal storage remains Option
    _state: PhantomData<S>,
}
```

**Invariants Enforced by Type System:**
- `Checkpointed<C, AtCheckpoint>`: `candidate` is guaranteed `None`
- `Checkpointed<C, HasCandidate>`: `candidate` is guaranteed `Some`

### Proposed Method API

```rust
// Available on all states
impl<C: HasCheckpoint, S: CandidateState> Checkpointed<C, S> {
    fn checkpoint(&self) -> &C::Checkpoint;
    fn at_checkpoint(&self) -> bool;
}

// Only available at checkpoint
impl<C: HasCheckpoint> Checkpointed<C, AtCheckpoint> {
    fn new(checkpoint: C::Checkpoint) -> Self;
    
    // Transition: AtCheckpoint → HasCandidate
    fn as_candidate(&self) -> Checkpointed<C, HasCandidate>
    where C: Clone;
}

// Only available with candidate
impl<C: HasCheckpoint> Checkpointed<C, HasCandidate> {
    fn with_candidate(checkpoint: C::Checkpoint, candidate: C) -> Self;
    
    fn candidate(&self) -> &C;
    fn candidate_mut(&mut self) -> &mut C;
    
    // Access current position (guaranteed candidate)
    fn current(&self) -> &C;
    fn current_mut(&mut self) -> &mut C;
}

// State transitions for PathCursor
impl<P: Clone> Checkpointed<PathCursor<P, Candidate>, HasCandidate> {
    // Transition: HasCandidate → AtCheckpoint (with state change)
    fn mark_match(self) -> Checkpointed<PathCursor<P, Matched>, AtCheckpoint>;
    fn mark_mismatch(self) -> Checkpointed<PathCursor<P, Mismatched>, HasCandidate>;
}
```

### Type System Benefits

**Before (Current):**
```rust
let checkpointed: Checkpointed<PathCursor<P, Matched>>;
let current = checkpointed.current();  // CheckpointedRef - is it checkpoint or candidate?
match current {
    CheckpointedRef::Checkpoint(c) => { /* owned, cloned */ }
    CheckpointedRef::Candidate(c) => { /* borrowed */ }
}
```

**After (Proposed):**
```rust
// At checkpoint
let at_checkpoint: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>;
let checkpoint = at_checkpoint.checkpoint();  // Direct access, no clone

// With candidate
let has_candidate: Checkpointed<PathCursor<P, Candidate>, HasCandidate>;
let candidate = has_candidate.current();  // Direct access, guaranteed Some
```

### State Transition Examples

**Example 1: Initial Match**
```rust
// Start with matched cursor
let cursor = PathCursor::from(pattern_path);

// Create checkpointed at checkpoint
let checkpointed = Checkpointed::<_, AtCheckpoint>::new(cursor);

// Advance creates candidate
let with_candidate = checkpointed.as_candidate();
// Type: Checkpointed<PathCursor<P, Candidate>, HasCandidate>
```

**Example 2: Mark Match**
```rust
// Have candidate cursor in Candidate state
let with_candidate: Checkpointed<PathCursor<P, Candidate>, HasCandidate>;

// Mark match transitions both states
let matched = with_candidate.mark_match();
// Type: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>
```

**Example 3: CompareState Transitions**
```rust
// Both cursors at checkpoint
let state: CompareState<
    Matched, Matched,
    AtCheckpoint, AtCheckpoint  // New parameters
>;

// Advance query
let advanced = state.advance_query_cursor(trav);
// Type: CompareState<
//     Candidate, Matched,
//     HasCandidate, AtCheckpoint
// >

// Mark match
let matched = advanced.mark_match();
// Type: CompareState<
//     Matched, Matched,
//     AtCheckpoint, AtCheckpoint
// >
```

## Implementation Plan

### Phase 1: Core Type Refactoring

**Files to modify:**
- `crates/context-search/src/cursor/checkpointed.rs` (main changes)

**Changes:**
1. Add `CandidateState` trait and marker types
2. Update `Checkpointed<C>` to `Checkpointed<C, S: CandidateState = AtCheckpoint>`
3. Implement state-specific methods on `impl<C> Checkpointed<C, AtCheckpoint>` and `impl<C> Checkpointed<C, HasCandidate>`
4. Remove `CheckpointedRef` enum entirely
5. Update `HasCheckpoint` trait implementations

### Phase 2: Update CompareState

**Files to modify:**
- `crates/context-search/src/compare/state/core.rs`
- `crates/context-search/src/compare/state/transitions.rs`

**Changes:**
1. Add `CandidateState` parameters to `CompareState`:
   ```rust
   pub struct CompareState<
       Q: CursorState = Candidate,
       I: CursorState = Candidate,
       QS: CandidateState = HasCandidate,
       IS: CandidateState = HasCandidate,
       EndNode: PathNode = PositionAnnotated<ChildLocation>,
   >
   ```
2. Update type aliases:
   ```rust
   pub type MatchedCompareState = CompareState<
       Matched, Matched,
       AtCheckpoint, AtCheckpoint,
       PositionAnnotated<ChildLocation>
   >;
   ```
3. Update state transition methods to change candidate states appropriately

### Phase 3: Update MatchResult and Response

**Files to modify:**
- `crates/context-search/src/state/matched/mod.rs`
- `crates/context-search/src/state/result.rs`

**Changes:**
1. Update `MatchResult::cursor` field type:
   ```rust
   pub cursor: Checkpointed<PatternCursor, AtCheckpoint>
   ```
2. Update `cursor()` method to return `&PatternCursor<Matched>` directly
3. Remove `CheckpointedRef` usages

### Phase 4: Update Search Algorithm

**Files to modify:**
- `crates/context-search/src/search/mod.rs`
- `crates/context-search/src/match/root_cursor/advance.rs`

**Changes:**
1. Update `finish_root_cursor()` to use typed candidate states
2. Update `create_result_from_state()` to handle candidate states
3. Update parent exploration to use `HasCandidate` state

### Phase 5: Update Call Sites

**Files to modify:** (estimated 50+ files)
- All files using `.current()` → replace with `.checkpoint()` or `.candidate()` based on context
- All files creating `Checkpointed` → add `CandidateState` parameter
- All pattern matches on `CheckpointedRef` → use direct access

**Common patterns:**
```rust
// Before
let pos = checkpointed.current().atom_position;

// After (at checkpoint)
let pos = checkpointed.checkpoint().atom_position;

// After (with candidate)
let pos = checkpointed.current().atom_position;
```

### Phase 6: Update Tests

**Files to modify:**
- `crates/context-search/src/tests/**/*.rs` (40 test files)

**Changes:**
1. Update type annotations with candidate state parameters
2. Update assertions to use direct access methods
3. Verify all 40 tests pass

## Minimal Examples by Configuration

### Configuration 1: Initial Match (AtCheckpoint)

```rust
// Create initial matched cursor
let cursor = PathCursor {
    path: pattern_path,
    atom_position: 0.into(),
    _state: PhantomData::<Matched>,
};

// Wrap in Checkpointed at checkpoint
let checkpointed = Checkpointed::<_, AtCheckpoint>::new(cursor);

// Access checkpoint directly
let pos = checkpointed.checkpoint().atom_position;  // No clone!
```

### Configuration 2: Advancing (AtCheckpoint → HasCandidate)

```rust
// Start at checkpoint
let at_checkpoint: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>;

// Create candidate for comparison
let with_candidate = at_checkpoint.as_candidate();
// Type: Checkpointed<PathCursor<P, Candidate>, HasCandidate>

// Access candidate directly
let candidate_pos = with_candidate.current().atom_position;
```

### Configuration 3: Mark Match (HasCandidate → AtCheckpoint)

```rust
// Have candidate that matched
let with_candidate: Checkpointed<PathCursor<P, Candidate>, HasCandidate>;

// Mark as matched (updates checkpoint, removes candidate)
let matched = with_candidate.mark_match();
// Type: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>

// Now at checkpoint again
assert!(matched.at_checkpoint());
```

### Configuration 4: CompareState Matched → Advanced

```rust
// Both at checkpoint
let state = CompareState {
    query: Checkpointed::<PathCursor<P, Matched>, AtCheckpoint>::new(q),
    child: Checkpointed::<ChildCursor<Matched, N>, AtCheckpoint>::new(c),
    ...
};

// Advance query
let advanced = state.advance_query_cursor(trav);
// Type: CompareState<
//     Candidate, Matched,
//     HasCandidate, AtCheckpoint
// >

// Query has candidate, child at checkpoint
let query_pos = advanced.query.current().atom_position;
let child_pos = advanced.child.checkpoint().child_state.entry_pos;
```

### Configuration 5: best_match Update

```rust
// SearchState tracking best match
let mut best_match: Option<MatchResult> = None;

loop {
    match advance_search() {
        Some(match_result) => {
            // MatchResult always has AtCheckpoint cursor
            // Type: MatchResult { cursor: Checkpointed<..., AtCheckpoint> }
            best_match = Some(match_result);
        }
        None => break,
    }
}

// Access final match position
if let Some(result) = best_match {
    let final_pos = result.cursor.checkpoint().atom_position;
}
```

## Code Design Questions

### Q1: Should we share path data between checkpoint and candidate?

**Current:** Each stores independent path
**Future:** Could share common prefix, only store divergence

**Decision:** Defer to future optimization. Current refactoring focuses on type safety and API clarity.

### Q2: How to handle mutable access that materializes candidate?

**Option A:** Keep `current_mut()` that materializes
```rust
impl<C, S> Checkpointed<C, S> {
    fn current_mut(&mut self) -> &mut C {
        // Materializes candidate if None
    }
}
```

**Option B:** Remove materialization, require explicit state transition
```rust
// Must explicitly transition states
let with_candidate = at_checkpoint.as_candidate();
let pos = &mut with_candidate.current_mut().atom_position;
```

**Recommendation:** Option B for clearer semantics, but add convenience methods where needed.

### Q3: Default candidate state?

**Current proposal:** `AtCheckpoint` as default
```rust
pub struct Checkpointed<C, S: CandidateState = AtCheckpoint>
```

**Rationale:** Most common case is starting at checkpoint, explicit annotation required for advanced state.

### Q4: How to handle StateAdvance with candidate states?

**Pattern:**
```rust
impl StateAdvance for Checkpointed<PathCursor<P, Matched>, AtCheckpoint> {
    type Next = Checkpointed<PathCursor<P, Matched>, HasCandidate>;
    
    fn advance_state<G: HasGraph>(self, trav: &G) 
        -> Result<Self::Next, Self> 
    {
        // Advance creates candidate
    }
}
```

**Question:** Should `Next` change both cursor state and candidate state?

**Recommendation:** Yes, advancement naturally creates candidate.

## Summary

This refactoring will:

1. **Remove `CheckpointedRef`** - eliminates unnecessary enum and cloning
2. **Add `CandidateState` type parameter** - encodes candidate presence at compile time
3. **Provide direct access** - `candidate()` returns `&C` directly, no conversion needed
4. **Mirror existing patterns** - follows PathCursor/CompareState type-state pattern
5. **Enable future optimizations** - type-safe foundation for path sharing

The changes touch ~100+ files but are mostly mechanical updates to type annotations and method calls. The core logic remains the same, we're just making the type system encode information that was previously only known at runtime.
