---
tags: `#analysi` `#algorithm` `#testing` `#refactoring` `#api` `#performance`
summary: > Remove CheckpointedRef because it incorrectly discards state information about whether the Path is a candidate or not. We should have something l...
---

# Summary: Removing CheckpointedRef and Adding Candidate State Control

## Your Request

> Remove CheckpointedRef because it incorrectly discards state information about whether the Path is a candidate or not. We should have something like CandidateState which is either Checkpoint (checkpoint only) or Candidate (checkpoint and candidate) and controls the type of the Checkpointed.candidate value.

## Core Problem

**CheckpointedRef is Problematic:**
1. Returns `CheckpointedRef::Checkpoint(C)` as owned value (requires cloning `C::Checkpoint → C`)
2. Loses type information about whether we're at checkpoint or have advanced
3. Runtime check (`match` on enum) for information known at compile time
4. Cannot guarantee candidate existence in type system

**Current Design:**
```rust
pub enum CheckpointedRef<'a, C> {
    Checkpoint(C),      // Owned (cloned from checkpoint)
    Candidate(&'a C),   // Borrowed
}

pub struct Checkpointed<C> {
    checkpoint: C::Checkpoint,
    candidate: Option<C>,  // Runtime optional
}

// Usage
let current = checkpointed.current();  // Returns CheckpointedRef
match current {
    CheckpointedRef::Checkpoint(c) => { /* owns cloned cursor */ }
    CheckpointedRef::Candidate(c) => { /* borrows candidate */ }
}
```

## Proposed Solution

**Add CandidateState Type Parameter:**
```rust
pub trait CandidateState: 'static {}

pub struct AtCheckpoint;      // No candidate
impl CandidateState for AtCheckpoint {}

pub struct HasCandidate;      // Has candidate
impl CandidateState for HasCandidate {}

pub struct Checkpointed<C: HasCheckpoint, S: CandidateState = AtCheckpoint> {
    checkpoint: C::Checkpoint,
    candidate: Option<C>,      // Internal storage
    _state: PhantomData<S>,
}
```

**Type System Guarantees:**
- `Checkpointed<C, AtCheckpoint>` → `candidate` is `None` (guaranteed by construction)
- `Checkpointed<C, HasCandidate>` → `candidate` is `Some(C)` (guaranteed by construction)

**Direct Access (No Cloning):**
```rust
// At checkpoint
impl<C> Checkpointed<C, AtCheckpoint> {
    fn checkpoint(&self) -> &C::Checkpoint;  // Direct access
}

// With candidate
impl<C> Checkpointed<C, HasCandidate> {
    fn candidate(&self) -> &C;         // Direct access, guaranteed Some
    fn current(&self) -> &C;           // Alias for candidate
    fn candidate_mut(&mut self) -> &mut C;
}
```

## Why This Design

### 1. Mirrors Existing Patterns

**PathCursor already uses phantom types for state:**
```rust
pub struct PathCursor<P, State = Matched> {
    path: P,
    atom_position: AtomPosition,
    _state: PhantomData<State>,
}

// State transitions change types
impl PathCursor<P, Candidate> {
    fn mark_match(self) -> PathCursor<P, Matched>;  // State transition
}
```

**CompareState uses multiple state parameters:**
```rust
pub struct CompareState<
    Q: CursorState = Candidate,    // Query cursor state
    I: CursorState = Candidate,    // Index cursor state
    EndNode: PathNode = ...,
> {
    query: Checkpointed<PathCursor<P, Q>>,
    child: Checkpointed<ChildCursor<I, N>>,
}

// Different configurations have different capabilities
impl CompareState<Candidate, Candidate> {
    fn mark_match(self) -> CompareState<Matched, Matched>;
}
```

**Proposed Checkpointed extends this pattern:**
```rust
pub struct Checkpointed<
    C: HasCheckpoint,
    S: CandidateState = AtCheckpoint,  // New parameter
> { ... }

// State transitions change candidate state type
impl Checkpointed<PathCursor<P, Candidate>, HasCandidate> {
    fn mark_match(self) -> Checkpointed<PathCursor<P, Matched>, AtCheckpoint>;
}
```

### 2. Compile-Time Guarantees Replace Runtime Checks

**Before (Runtime):**
```rust
if checkpointed.at_checkpoint() {  // Runtime check
    // Use checkpoint
} else {
    // Use candidate
}
```

**After (Compile-Time):**
```rust
// Type tells us the state
fn process_checkpoint<C>(cp: Checkpointed<C, AtCheckpoint>) {
    // Guaranteed no candidate
}

fn process_candidate<C>(cp: Checkpointed<C, HasCandidate>) {
    let c = cp.candidate();  // Guaranteed Some, no unwrap needed
}
```

### 3. Zero-Cost Abstraction

**Memory layout identical:**
- Both designs use `Option<C>` internally
- Phantom type has zero runtime cost
- No additional memory overhead

**Performance benefits:**
- No more cloning `C::Checkpoint → C` for temporary access
- Direct access via `candidate()` instead of enum matching
- Compiler can optimize based on type information

### 4. Simpler API, Fewer Conversions

**Before:**
```rust
let current = checkpointed.current();  // CheckpointedRef
let pos = current.atom_position();      // Method call through trait
```

**After:**
```rust
// At checkpoint
let pos = checkpointed.checkpoint().atom_position;  // Direct field access

// With candidate
let pos = checkpointed.candidate().atom_position;   // Direct field access
```

## State Transition Examples

### Example 1: Creating Initial Checkpoint
```rust
// Start with matched cursor
let cursor = PathCursor::<PatternPath, Matched> { ... };

// Create checkpointed at checkpoint
let checkpointed = Checkpointed::<_, AtCheckpoint>::new(cursor);
// Type: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>

// Access checkpoint directly (no clone)
let pos = checkpointed.checkpoint().atom_position;
```

### Example 2: Advancing Creates Candidate
```rust
// Start at checkpoint
let at_checkpoint: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>;

// Convert to candidate for comparison
let with_candidate = at_checkpoint.as_candidate();
// Type: Checkpointed<PathCursor<P, Candidate>, HasCandidate>

// Access candidate directly
let candidate = with_candidate.candidate();  // Returns &PathCursor<P, Candidate>
let pos = candidate.atom_position;
```

### Example 3: Marking Match Removes Candidate
```rust
// Have candidate cursor after comparison
let with_candidate: Checkpointed<PathCursor<P, Candidate>, HasCandidate>;

// Mark as matched (moves candidate → checkpoint)
let matched = with_candidate.mark_match();
// Type: Checkpointed<PathCursor<P, Matched>, AtCheckpoint>

// Back at checkpoint
assert_eq!(matched.candidate, None);  // Guaranteed by type
```

### Example 4: CompareState Integration
```rust
// Both cursors at checkpoint
let state: CompareState<
    Matched,       // Query cursor state
    Matched,       // Child cursor state
    AtCheckpoint,  // Query candidate state (NEW)
    AtCheckpoint,  // Child candidate state (NEW)
>;

// Advance query creates candidate
let advanced = state.advance_query_cursor(trav);
// Type: CompareState<Candidate, Matched, HasCandidate, AtCheckpoint>

// Query has candidate, child at checkpoint
let query_pos = advanced.query.candidate().atom_position;
let child_pos = advanced.child.checkpoint().child_state.entry_pos;

// Mark match removes candidate
let matched = advanced.mark_match();
// Type: CompareState<Matched, Matched, AtCheckpoint, AtCheckpoint>
```

### Example 5: best_match in Search
```rust
let mut best_match: Option<MatchResult> = None;

while let Some(matched_state) = self.next() {
    // MatchResult always stores checkpoint state
    // Type: MatchResult { 
    //     cursor: Checkpointed<PatternCursor<Matched>, AtCheckpoint>
    // }
    best_match = Some(matched_state);
}

// Access final position (no CheckpointedRef needed)
if let Some(result) = best_match {
    let final_pos = result.cursor.checkpoint().atom_position;
}
```

## Implementation Strategy

### Phase 1: Core Types (checkpointed.rs)
1. Add `CandidateState` trait and markers
2. Update `Checkpointed<C>` → `Checkpointed<C, S>`
3. Implement state-specific methods
4. Remove `CheckpointedRef` enum

### Phase 2: CompareState (compare/state/)
1. Add candidate state parameters to `CompareState`
2. Update all type aliases
3. Fix state transitions to change candidate states

### Phase 3: MatchResult & Response (state/)
1. Specify `AtCheckpoint` state for `MatchResult::cursor`
2. Remove `CheckpointedRef` from API
3. Return `&PathCursor<Matched>` directly

### Phase 4: Search Algorithm (search/, match/)
1. Use typed candidate states
2. Update `best_match` handling
3. Fix parent exploration states

### Phase 5: Call Sites (~100+ files)
1. Replace `.current()` with `.checkpoint()` or `.candidate()`
2. Add candidate state parameters to types
3. Remove `CheckpointedRef` pattern matches

### Phase 6: Tests
1. Update 40 test files
2. Verify all tests pass
3. Document new patterns

## Benefits Summary

✅ **Type Safety**: Candidate presence encoded in type system
✅ **No Cloning**: Direct access eliminates `C::Checkpoint → C` conversion
✅ **Clear Intent**: Method names reflect what you're accessing
✅ **Follows Patterns**: Consistent with PathCursor/CompareState design
✅ **Zero Cost**: No runtime overhead, compiler optimizes better
✅ **Future Ready**: Type-safe foundation for path sharing optimization

## Questions for Clarification

### Q1: Default State
Should `AtCheckpoint` be the default? This matches "most checkpointed cursors start at checkpoint."
```rust
Checkpointed<C, AtCheckpoint>  // Most common
Checkpointed<C>                // Uses default
```

### Q2: Mutable Access
Should we keep `current_mut()` that materializes candidate, or require explicit state transition?
```rust
// Option A: Materialization (current behavior)
fn current_mut(&mut self) -> &mut C { /* creates candidate if None */ }

// Option B: Explicit transition (clearer but more verbose)
let with_candidate = at_checkpoint.as_candidate();
with_candidate.candidate_mut().atom_position += 1;
```

### Q3: StateAdvance Return Type
Should advancing change candidate state?
```rust
impl StateAdvance for Checkpointed<PathCursor<P, Matched>, AtCheckpoint> {
    type Next = Checkpointed<PathCursor<P, Matched>, HasCandidate>;
    // Advancement creates candidate
}
```

### Q4: Naming
- `AtCheckpoint` vs `NoCandidate`?
- `HasCandidate` vs `Advanced`?
- `candidate()` vs `current()` for `HasCandidate` state?

Current proposal uses `AtCheckpoint`/`HasCandidate` because:
- Describes what the state represents
- Avoids overloading "current" (checkpoint is also current when at checkpoint)
- Parallel to other state names (Matched, Candidate, etc.)

## Next Steps

1. Review this summary and implementation plan
2. Answer clarifying questions above
3. Proceed with Phase 1 implementation (checkpointed.rs)
4. Iterate through phases with test-driven development
5. Update documentation as we go

All tests currently pass, so we can refactor confidently with TDD verification at each step.
