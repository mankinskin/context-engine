# Matching State Architecture Analysis

## Problem Statement

The current codebase lacks explicit type-level distinction between different phases of path states during the matching process. This leads to:

1. **Confusion about cursor/path positions** - Are we pointing at the last matched token or the next candidate?
2. **AtomPosition tracking errors** - Position 7 vs 3 suggests incorrect phase interpretation
3. **Unclear advancement semantics** - When should paths be advanced, and what does advancement mean?

## Current State: Implicit Phases

Currently, `PathCursor` and traversal paths transition through multiple conceptual phases, but these are not explicitly typed:

### Phase 1: Match Candidate (Before Comparison)
```rust
// Cursor points to NEXT token to match
// Path represents: "already matched" + "candidate token"
cursor.end.root_entry = N  // Points to token[N] (next to match)
atom_position = X          // Position where token[N] should be found
```

### Phase 2: Comparison State (During compare())
```rust
// CompareIterator checks if candidate token matches at position
// If match: transition to "already matched" state
// If mismatch: backtrack or search parents
```

### Phase 3: Already Matched (After Successful Match)
```rust
// Cursor represents successfully matched range
// Ready to be advanced to next candidate
cursor.end.root_entry = N  // Last successfully matched token
atom_position = X          // Where token[N] was found
```

### Phase 4: Exhausted (Query Complete)
```rust
// Cursor has matched entire pattern
// No more tokens to match
cursor.end.root_entry >= pattern.len()
```

## The Root Cause of AtomPosition Bug

The bug manifests as `AtomPosition(7)` instead of expected `AtomPosition(3)`:

### Hypothesis
```
Expected: atom_position = 3 (position where token 'e' at index 2 was found)
Actual:   atom_position = 7 (position = 3 + 4, double-counted?)

This suggests:
- Cursor is being advanced to "candidate" state (position 7)
- But test expects "already matched" state (position 3)
- The advancement happened at wrong time or was counted twice
```

### Where This Matters

1. **In `advanced()` method**:
   - When `query_advanced()` is called, does it transition cursor to candidate state?
   - When should we save the "already matched" state for the result?

2. **In `next()` iterator**:
   - After `advanced()`, we call `compare()`
   - If match succeeds, do we advance again? Or is cursor already advanced?

3. **In final result**:
   - Should `EndState.cursor` represent the last matched position?
   - Or the candidate position where matching stopped?

## Proposed Solution: Explicit State Types

### Option A: Phantom Type States

```rust
// Type-level state markers
struct MatchedState;
struct CandidateState;
struct ExhaustedState;

struct PathCursor<Path, State = MatchedState> {
    path: Path,
    atom_position: AtomPosition,
    _state: PhantomData<State>,
}

impl<P> PathCursor<P, MatchedState> {
    /// Advance to candidate state - ready for next comparison
    fn advance_to_candidate(self, trav: &impl HasGraph) 
        -> ControlFlow<(), PathCursor<P, CandidateState>> 
    {
        // ...
    }
}

impl<P> PathCursor<P, CandidateState> {
    /// After successful match, transition to matched state
    fn confirm_match(self) -> PathCursor<P, MatchedState> {
        // Cursor already points to candidate, this is identity operation
        // but makes the state transition explicit
        PathCursor {
            path: self.path,
            atom_position: self.atom_position,
            _state: PhantomData,
        }
    }
    
    /// After failed match, revert to previous matched state
    fn revert_to_matched(self, prev: PathCursor<P, MatchedState>) 
        -> PathCursor<P, MatchedState> 
    {
        prev
    }
}
```

### Option B: Explicit Enum Wrapper

```rust
enum CursorState<P> {
    /// Last successfully matched position
    Matched(PathCursor<P>),
    
    /// Advanced position, ready for comparison
    Candidate {
        cursor: PathCursor<P>,
        previous_matched: Box<PathCursor<P>>,
    },
    
    /// Query pattern exhausted
    Exhausted(PathCursor<P>),
}

impl<P> CursorState<P> {
    fn advance_for_matching(self, trav: &impl HasGraph) 
        -> ControlFlow<(), Self> 
    {
        match self {
            Self::Matched(cursor) => {
                match cursor.advance(trav) {
                    Continue(advanced) => Continue(Self::Candidate {
                        cursor: advanced,
                        previous_matched: Box::new(cursor),
                    }),
                    Break(()) => Break(()),
                }
            },
            Self::Candidate { .. } => {
                panic!("Cannot advance candidate state - must confirm or revert first")
            },
            Self::Exhausted(_) => Break(()),
        }
    }
    
    fn confirm_match(self) -> Result<Self, &'static str> {
        match self {
            Self::Candidate { cursor, .. } => Ok(Self::Matched(cursor)),
            _ => Err("Can only confirm candidate state"),
        }
    }
}
```

### Option C: Separate Types

```rust
/// Represents a successfully matched range up to this position
struct MatchedCursor<P> {
    path: P,
    atom_position: AtomPosition,
}

/// Represents a candidate for the next match
struct CandidateCursor<P> {
    path: P,
    atom_position: AtomPosition,
    previous_matched: MatchedCursor<P>,
}

impl<P> MatchedCursor<P> {
    fn advance_to_candidate(self, trav: &impl HasGraph) 
        -> ControlFlow<(), CandidateCursor<P>> 
    {
        // ...
    }
}

impl<P> CandidateCursor<P> {
    fn confirm(self) -> MatchedCursor<P> {
        MatchedCursor {
            path: self.path,
            atom_position: self.atom_position,
        }
    }
    
    fn reject(self) -> MatchedCursor<P> {
        self.previous_matched
    }
}
```

## Impact on Current Code

### `RootCursor::advanced()`

Currently:
```rust
fn advanced(&mut self) -> ControlFlow<Option<EndReason>> {
    // Advances cursor, but unclear what state cursor is in after
    match self.query_advanced() {
        Continue(_) => {
            // Is cursor now in "candidate" or "matched" state?
            // When does it transition?
        }
    }
}
```

With explicit states:
```rust
fn advanced(&mut self) -> ControlFlow<Option<EndReason>> {
    // Explicitly: cursor starts in MatchedState
    // We advance it to CandidateState
    match self.state.cursor.advance_to_candidate(&self.trav) {
        Continue(candidate_cursor) => {
            // Clearly in candidate state now
            self.state.cursor = CursorState::Candidate(candidate_cursor);
            // ...
        }
    }
}
```

### `RootCursor::next()`

Currently:
```rust
fn next(&mut self) -> Option<Self::Item> {
    let prev_state = self.state.clone();
    match self.advanced() {
        Continue(_) => {
            match CompareIterator::new(...).compare() {
                Match(c) => {
                    *self.state = c;  // State updated, but cursor state unclear
                    Continue(())
                },
                Mismatch(_) => {
                    self.state = prev_state;  // Revert to what state?
                    Break(EndReason::Mismatch)
                },
            }
        }
    }
}
```

With explicit states:
```rust
fn next(&mut self) -> Option<Self::Item> {
    match self.advanced() {  // Returns cursor in candidate state
        Continue(_) => {
            match CompareIterator::new(...).compare() {
                Match(c) => {
                    // Explicitly confirm the candidate
                    self.state.cursor = self.state.cursor.confirm_match();
                    Continue(())
                },
                Mismatch(_) => {
                    // Explicitly revert to previous matched state
                    self.state.cursor = self.state.cursor.reject();
                    Break(EndReason::Mismatch)
                },
            }
        }
    }
}
```

## Addressing the AtomPosition Bug

With explicit state tracking, the bug becomes obvious:

```rust
// Current buggy flow (hypothetical):
1. cursor at position 3 (matched state)
2. query_advanced() -> cursor at position 7 (candidate state)
3. Test expects position 3, but cursor is in candidate state!

// With explicit states:
1. cursor = MatchedCursor { atom_position: 3 }
2. advance_to_candidate() -> CandidateCursor { atom_position: 7, previous: 3 }
3. When building result, we explicitly choose:
   - Return candidate.atom_position (7) if we want "next position"
   - Return candidate.previous.atom_position (3) if we want "last matched"
4. Test expectations become explicit about which state they want
```

## Migration Strategy

### Phase 1: Document Current Behavior
- [ ] Add comments marking where each phase transition occurs
- [ ] Document what state cursor/path is in at each point
- [ ] Identify all places where state confusion causes bugs

### Phase 2: Introduce State Wrapper (Option B - Least Invasive)
- [ ] Create `CursorState` enum as wrapper around `PathCursor`
- [ ] Update `CompareState` to use `CursorState`
- [ ] Gradually convert code to use explicit state transitions

### Phase 3: Refactor for Type Safety (Option A or C)
- [ ] Consider phantom types for compile-time guarantees
- [ ] Or separate types for maximum clarity
- [ ] Update all code to use new types

### Phase 4: Fix Tests
- [ ] Update test expectations to match correct state semantics
- [ ] Add tests that explicitly check state transitions
- [ ] Document what state each test expects

## Related Issues

This architectural issue likely affects:

1. **TraceCache positions** - Are cached positions in matched or candidate state?
2. **EndState paths** - Should they represent last matched or candidate?
3. **Parent search** - When transitioning to parent level, which state?
4. **Range paths** - Start/End roles may be in different states

## Recommendations

### Immediate (Fix Current Bug)
1. **Document current state semantics** in comments
2. **Identify where AtomPosition 7 comes from** - which advancement?
3. **Fix by choosing consistent state** - either always matched or always candidate
4. **Update test expectations** to match chosen semantics

### Short-term (Reduce Confusion)
1. **Implement Option B** - explicit enum for state tracking
2. **Add assertions** checking state invariants
3. **Update documentation** explaining state model

### Long-term (Prevent Future Bugs)
1. **Consider Option A or C** - type-level state distinction
2. **Make state transitions explicit** in method names
3. **Add compile-time guarantees** where possible

## Questions to Answer

1. **What should `EndState.cursor` represent?**
   - Last successfully matched position? (matched state)
   - Position where matching stopped? (candidate state)
   - First unmatched position? (exhausted state)

2. **When should `query_advanced()` be called?**
   - Before comparison? (transitions to candidate)
   - After comparison? (transitions to next matched)
   - Both? (different meanings)

3. **How should parent search work?**
   - Does cursor stay in same state when moving to parent?
   - Or does it reset to a specific state?

4. **What about TraceCache?**
   - Should cache store matched positions only?
   - Or also candidate positions?
   - How to distinguish?

## Next Steps

1. Review current `advanced()` and `next()` flow with explicit state annotations
2. Identify exact point where AtomPosition becomes 7 instead of 3
3. Decide on state semantics for `EndState.cursor`
4. Implement minimal fix with clear documentation
5. Plan gradual migration to explicit state types
