# CompareState Refactoring Proposal

## Current Issue

```rust
pub(crate) struct CompareState {
    pub(crate) child_state: ChildState,
    pub(crate) cursor: PatternCursor,              // Redundant full path
    pub(crate) matched_cursor: PatternCursor,      // Redundant full path
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
```

Problems:
1. **Redundant data**: Both cursors store full `PatternRangePath` but share the same `start` path
2. **No type safety**: Can't distinguish at compile-time whether cursor is Matched/Candidate
3. **Unclear semantics**: `matched_cursor` vs `cursor` naming doesn't reveal the relationship

## Proposed Solution 1: Generic Over Cursor State

```rust
pub(crate) struct CompareState<S: CursorState> {
    pub(crate) child_state: ChildState,
    
    /// Current cursor (may be Matched, Candidate, or Mismatched)
    pub(crate) cursor: PathCursor<PatternRangePath, S>,
    
    /// Checkpoint: cursor position before current token comparison
    pub(crate) matched_cursor: PathCursor<PatternRangePath, Matched>,
    
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}

// Type aliases for clarity
pub(crate) type MatchedCompareState = CompareState<Matched>;
pub(crate) type CandidateCompareState = CompareState<Candidate>;
pub(crate) type MismatchedCompareState = CompareState<Mismatched>;
```

### API Changes

```rust
impl CompareState<Candidate> {
    /// Called when tokens match
    pub(crate) fn on_match(self) -> CompareState<Matched> {
        CompareState {
            cursor: self.cursor.confirm_match(),
            matched_cursor: self.matched_cursor,
            child_state: self.child_state,
            target: self.target,
            mode: self.mode,
        }
    }
    
    /// Called when tokens mismatch  
    pub(crate) fn on_mismatch<G: HasGraph>(self, trav: &G) -> EndState {
        // Mark as mismatched to preserve atom_position semantics
        let mismatched_cursor = self.cursor.mark_mismatch();
        
        // ... build EndState using mismatched_cursor.atom_position ...
    }
}
```

### Benefits
- ✅ Type safety: can't call `on_match` on non-candidate states
- ✅ Clear semantics: `CompareState<Candidate>` means "needs decision"
- ✅ Still redundant paths, but at least type-safe

## Proposed Solution 2: Shared Start Path

```rust
pub(crate) struct CompareState {
    pub(crate) child_state: ChildState,
    
    /// Shared base path (matched portion)
    pub(crate) base_path: PatternStartPath,  // Only the `start` role path
    pub(crate) matched_atom_position: AtomPosition,
    
    /// Candidate end path (may differ from matched)
    pub(crate) candidate_end: Option<RolePath<End>>,
    pub(crate) candidate_atom_position: AtomPosition,
    
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
```

### Benefits
- ✅ No redundant paths
- ❌ Loses type-state safety
- ❌ More complex to work with (need to reconstruct full paths)

## Proposed Solution 3: Hybrid Approach

```rust
pub(crate) struct CompareState<S: CursorState> {
    pub(crate) child_state: ChildState,
    
    /// Checkpoint cursor (always Matched state)
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,
    
    /// Current cursor state
    pub(crate) cursor: PathCursor<PatternRangePath, S>,
    
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}

impl<S: CursorState> CompareState<S> {
    /// Get the matched portion (from checkpoint)
    pub(crate) fn matched_path(&self) -> &PatternRangePath {
        &self.checkpoint.path
    }
    
    /// Get the matched atom position
    pub(crate) fn matched_atom_position(&self) -> AtomPosition {
        self.checkpoint.atom_position
    }
}

impl CompareState<Matched> {
    /// Create a candidate for comparison
    pub(crate) fn as_candidate(&self) -> CompareState<Candidate> {
        CompareState {
            checkpoint: self.checkpoint.clone(),
            cursor: self.cursor.as_candidate(),
            child_state: self.child_state.clone(),
            target: self.target,
            mode: self.mode,
        }
    }
}

impl CompareState<Candidate> {
    /// Confirm the match
    pub(crate) fn confirm_match(self) -> CompareState<Matched> {
        CompareState {
            checkpoint: self.checkpoint, // Keep checkpoint
            cursor: self.cursor.confirm_match(),
            child_state: self.child_state,
            target: self.target,
            mode: self.mode,
        }
    }
    
    /// Handle mismatch
    pub(crate) fn on_mismatch<G: HasGraph>(self, trav: &G) -> EndState {
        let mismatched = self.cursor.mark_mismatch();
        // Use mismatched.atom_position for EndState
        // ...
    }
}
```

### Benefits
- ✅ Type-safe state transitions
- ✅ Clear naming (`checkpoint` vs `cursor`)
- ✅ Can optimize later to share paths if needed
- ✅ Better semantics: checkpoint = "where we were", cursor = "where we are"

## Recommendation

**Go with Solution 3 (Hybrid Approach)**:
1. Rename `matched_cursor` → `checkpoint` for clarity
2. Make `CompareState` generic over cursor state
3. Use type-states to enforce correct transitions
4. Keep redundant paths for now (optimize later if needed)

This provides maximum type safety and clarity while being minimally invasive to existing code.

## Migration Path

1. Add `Mismatched` cursor state ✅ (already done)
2. Make `CompareState` generic: `CompareState<S: CursorState>`
3. Update all functions to use typed states
4. Rename `matched_cursor` → `checkpoint`
5. Update `on_mismatch` to use `cursor.atom_position` (which will be correct after fix)
6. Fix cursor advancement during prefix decomposition

## The Real Fix for atom_position

The type safety is good, but the ACTUAL bug is:

**When creating prefix cursors during decomposition, we need to calculate the correct `atom_position` for each prefix.**

Current `prefix_states` just clones paths without adjusting `atom_position`:

```rust
// BAD: Doesn't adjust atom_position
fn prefix_states(&self, trav: &G) -> VecDeque<(SubToken, Self)> {
    // ... creates new paths but atom_position stays the same ...
}
```

Should be:

```rust
// GOOD: Accumulate atom_position for each prefix
fn prefix_states(&self, trav: &G) -> VecDeque<(SubToken, Self)> {
    let mut accumulated_position = self.atom_position;
    
    prefixes.map(|(sub, mut cursor)| {
        // For each prefix, set atom_position to accumulated value
        cursor.atom_position = accumulated_position;
        
        // Advance for next prefix
        accumulated_position = accumulated_position + sub.width();
        
        (sub, cursor)
    })
}
```

But this needs more thought - which prefix should have which position?
