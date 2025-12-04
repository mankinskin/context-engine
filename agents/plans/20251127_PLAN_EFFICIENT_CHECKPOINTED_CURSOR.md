# Efficient Checkpointed Cursor Implementation Plan

## Current State Summary

### Checkpointed Structure

**Location:** `crates/context-search/src/cursor/checkpointed.rs`

**Current Implementation:**
```rust
pub(crate) struct Checkpointed<C: HasCheckpoint> {
    pub(crate) current: C,           // Always stored
    pub(crate) checkpoint: C::Checkpoint,  // Always stored
}
```

**Key Characteristics:**
- Always stores BOTH checkpoint and current cursors
- No space optimization for when current == checkpoint
- Generic over cursor type C (PathCursor or ChildCursor)
- Type-safe state transitions via CursorStateMachine trait

### HasCheckpoint Trait

Maps cursor types to their checkpoint representation:
```rust
impl<P, S: CursorState> HasCheckpoint for PathCursor<P, S> {
    type Checkpoint = PathCursor<P, Matched>;
}

impl<S: CursorState, EndNode: PathNode> HasCheckpoint for ChildCursor<S, EndNode> {
    type Checkpoint = ChildCursor<Matched, EndNode>;
}
```

### Current Usage Patterns

#### 1. Creation (3 locations)
- **`Checkpointed::new(matched_cursor)`** - Start with matched position
  - `state/start/core.rs:202` - Create initial search cursor
  - Tests create initial states

#### 2. Advancement (11 locations)
- **`.as_candidate()`** - Convert Matched → Candidate state
  - `compare/state/transitions.rs:97` - Advance query cursor
  - `compare/state/transitions.rs:119` - Advance child cursor
  - 9 test locations creating candidate states

#### 3. State Transitions (6 locations)
- **`.mark_match()`** - Convert Candidate → Matched, updates checkpoint
  - `compare/state/transitions.rs:45,46` - Mark both cursors matched
  - `compare/state/decomposition.rs:216` - Mark after token match
  
- **`.mark_mismatch()`** - Convert Candidate → Mismatched, keeps checkpoint
  - `compare/state/transitions.rs:70,71` - Mark both cursors mismatched
  - `compare/state/decomposition.rs:223` - Mark after mismatch

#### 4. Checkpoint Access (30+ locations)
- **`.checkpoint()`** - Get checkpoint cursor (always Matched)
  - Read-only access throughout search logic
  - Used for position tracking, root keys, final states
  
- **`.current()`** - Get current cursor (any state)
  - Most frequent accessor (~50+ uses)
  - Used during comparisons, state creation, logging

#### 5. Mutation (2 locations)
- **`.current_mut()`** - Mutable access to current
  - `compare/state/transitions.rs:89` - `self.query.current_mut().advance()`
  - `compare/state/core.rs:150` - Path manipulation

### Key Insight from Usage Analysis

**Most Common Pattern:**
```rust
// 1. Start with Matched state
let matched = Checkpointed::new(cursor);

// 2. Advance to Candidate (current != checkpoint)
let candidate = matched.as_candidate();
candidate.current_mut().advance(trav);

// 3. Either:
//    a) mark_match() → checkpoint updated to match current
//    b) mark_mismatch() → checkpoint unchanged, current marked as mismatched
```

**Space Waste:** When `current == checkpoint` (after `mark_match()` or initial creation), we store the same cursor twice.

## Problem Statement

### Current Issues

1. **Redundant Storage**: After `mark_match()`, both `current` and `checkpoint` are identical
2. **No Space Optimization**: Always allocates space for both cursors
3. **Code Duplication Risk**: Might create similar structure in MatchResult

### Requirements

1. ✅ **Space Efficient**: Only store candidate when it differs from checkpoint
2. ✅ **Type Safe**: Preserve state machine guarantees
3. ✅ **Developer Friendly**: Clean API, clear semantics
4. ✅ **Encapsulated**: Advancement logic wrapped in trait/function
5. ✅ **Reusable**: Works for MatchResult without re-implementing
6. 🔮 **Future Ready**: Support shared data structures for start paths

## Proposed Solution: Optimized Checkpointed

### Design Philosophy

**Key Principle:** "Checkpoint is the source of truth, candidate is optional delta"

### New Structure

```rust
pub(crate) struct Checkpointed<C: HasCheckpoint> {
    /// Last confirmed match position (always present)
    pub(crate) checkpoint: C::Checkpoint,
    
    /// Advanced position when different from checkpoint
    /// None = current is same as checkpoint (space optimization)
    pub(crate) candidate: Option<C>,
}
```

**Space Savings:**
- `Matched` state: 50% reduction (no candidate stored)
- `Candidate` state: Same size as before
- Typical workflow: Start matched → advance → compare → matched (50% average savings)

### API Design

#### Core Methods

```rust
impl<C: HasCheckpoint> Checkpointed<C> {
    /// Get checkpoint cursor (always Matched state)
    pub(crate) fn checkpoint(&self) -> &C::Checkpoint;
    
    /// Get current cursor (checkpoint or candidate)
    pub(crate) fn current(&self) -> CheckpointedRef<'_, C>;
    
    /// Check if there's an advanced candidate
    pub(crate) fn has_candidate(&self) -> bool;
    
    /// Check if current == checkpoint (no advancement)
    pub(crate) fn is_at_checkpoint(&self) -> bool;
}
```

#### CheckpointedRef Helper

```rust
pub(crate) enum CheckpointedRef<'a, C: HasCheckpoint> {
    Checkpoint(&'a C::Checkpoint),
    Candidate(&'a C),
}

impl<'a, C: HasCheckpoint> CheckpointedRef<'a, C> {
    // Provide unified access to cursor methods
    pub fn atom_position(&self) -> AtomPosition { ... }
    pub fn path(&self) -> &impl HasPath { ... }
}
```

**Benefits:**
- No clones when accessing current in Matched state
- Clear API: `.current()` always works regardless of state
- Type-safe: Can't accidentally modify checkpoint

### State Machine Integration

#### New Trait: `AdvanceCheckpointed`

```rust
pub(crate) trait AdvanceCheckpointed<G: HasGraph> {
    type Advanced;
    
    /// Advance cursor to candidate state
    /// Encapsulates: checkpoint stays, current becomes candidate
    fn advance_to_candidate<F>(&self, f: F) -> Self::Advanced
    where
        F: FnOnce(&mut Self::Checkpoint) -> ControlFlow<()>;
}
```

**Usage:**
```rust
// Old way (manual)
let candidate = matched.as_candidate();
match candidate.current_mut().advance(trav) {
    Continue(_) => { /* use candidate */ },
    Break(_) => { /* exhausted */ }
}

// New way (encapsulated)
matched.advance_to_candidate(|cursor| cursor.advance(trav))
```

**Benefits:**
- Single call handles: clone checkpoint → advance → store as candidate
- Can't forget to clone
- Can't accidentally modify checkpoint
- Clear intent

#### State Transitions

```rust
impl<P> Checkpointed<PathCursor<P, Matched>> {
    /// Create from matched cursor (no candidate)
    pub(crate) fn new(cursor: PathCursor<P, Matched>) -> Self {
        Self {
            checkpoint: cursor,
            candidate: None,  // Space optimized!
        }
    }
    
    /// Advance to candidate state
    pub(crate) fn advance_to_candidate<G, F>(
        &self,
        advance_fn: F,
    ) -> Result<Checkpointed<PathCursor<P, Candidate>>, Self>
    where
        G: HasGraph,
        F: FnOnce(&mut PathCursor<P, Matched>) -> ControlFlow<()>,
    {
        let mut candidate = self.checkpoint.as_candidate();
        match advance_fn(&mut candidate) {
            Continue(_) => Ok(Checkpointed {
                checkpoint: self.checkpoint.clone(),
                candidate: Some(candidate),
            }),
            Break(_) => Err(self.clone()), // Exhausted
        }
    }
}

impl<P> Checkpointed<PathCursor<P, Candidate>> {
    /// Mark matched, collapse to checkpoint-only
    pub(crate) fn mark_match(self) -> Checkpointed<PathCursor<P, Matched>> {
        let matched = self.candidate.unwrap().mark_match();
        Checkpointed {
            checkpoint: matched,
            candidate: None,  // Space optimized!
        }
    }
    
    /// Mark mismatched, keep candidate separate
    pub(crate) fn mark_mismatch(self) -> Checkpointed<PathCursor<P, Mismatched>> {
        Checkpointed {
            checkpoint: self.checkpoint,
            candidate: Some(self.candidate.unwrap().mark_mismatch()),
        }
    }
}
```

## Integration with MatchResult

### Current MatchResult

```rust
pub struct MatchResult {
    pub path: PathCoverage,
    pub cursor: PatternCursor,  // Always Matched
}
```

### Proposed MatchResult

```rust
pub struct MatchResult {
    pub path: PathCoverage,
    pub cursor: Checkpointed<PatternCursor>,  // Reuse optimized type!
}
```

**Benefits:**
1. ✅ Reuses `Checkpointed` - no code duplication
2. ✅ Space efficient - matched results store single cursor
3. ✅ Can represent advanced query - stores candidate when needed
4. ✅ Type safe - state machine preserved
5. ✅ Clear API - `.checkpoint()` and `.current()` accessors

**For Response API:**
```rust
impl Response {
    pub fn cursor(&self) -> &PatternCursor {
        // Backward compat: return checkpoint
        self.end.cursor.checkpoint()
    }
    
    pub fn current_cursor(&self) -> CheckpointedRef<'_, PatternCursor> {
        // New API: get current (checkpoint or candidate)
        self.end.cursor.current()
    }
    
    pub fn has_advanced_query(&self) -> bool {
        self.end.cursor.has_candidate()
    }
}
```

## Implementation Plan

### Phase 1: Enhance Checkpointed Core (60 min)

**File:** `crates/context-search/src/cursor/checkpointed.rs`

1. **Update Checkpointed structure** (15 min)
   - Change `current: C` to `candidate: Option<C>`
   - Keep `checkpoint: C::Checkpoint`
   - Update documentation

2. **Add CheckpointedRef enum** (15 min)
   - Create enum with Checkpoint/Candidate variants
   - Implement common accessor methods
   - Add conversion traits

3. **Update current() method** (10 min)
   - Return `CheckpointedRef` instead of `&C`
   - Handle `Some(candidate)` vs `None` cases

4. **Add helper methods** (10 min)
   - `has_candidate()` → `self.candidate.is_some()`
   - `is_at_checkpoint()` → `self.candidate.is_none()`
   - Keep `checkpoint()` unchanged

5. **Update constructors** (10 min)
   - `Checkpointed::new()` → `candidate: None`
   - Update state transition methods

### Phase 2: Implement StateAdvance for Checkpointed (30 min)

**File:** `crates/context-search/src/cursor/checkpointed.rs`

**Rationale:** Reuse existing `StateAdvance` trait instead of creating new trait. Checkpointed advancement IS a state transition, so it fits the established pattern perfectly.

1. **Implement StateAdvance for Checkpointed<PathCursor<Matched>>** (15 min)
   ```rust
   impl<P> StateAdvance for Checkpointed<PathCursor<P, Matched>>
   where
       P: Clone + /* path movement traits */,
   {
       type Next = Checkpointed<PathCursor<P, Candidate>>;
       
       fn advance_state<G: HasGraph>(
           self,
           trav: &G,
       ) -> Result<Self::Next, Self> {
           let mut candidate = self.checkpoint.as_candidate();
           match candidate.move_root_index(trav) {
               Continue(_) => Ok(Checkpointed {
                   checkpoint: self.checkpoint,
                   candidate: Some(candidate),
               }),
               Break(_) => Err(self), // Exhausted
           }
       }
   }
### Phase 3: Update State Transitions (75 min)

**Files:** 
- `crates/context-search/src/compare/state/transitions.rs`
- `crates/context-search/src/cursor/checkpointed.rs`

1. **Update mark_match()** (20 min)
   - Extract candidate with `.unwrap()`
   - Convert to Matched
   - Create new Checkpointed with `candidate: None`

2. **Update mark_mismatch()** (15 min)
   - Keep candidate as Some(mismatched)
   - Checkpoint unchanged

3. **Update advance_query_cursor()** (40 min)
   - Replace manual `.as_candidate()` + `.current_mut().advance()`
   - Use `self.query.advance_state(trav)` (StateAdvance trait!)
   - Update return type handling:
     ```rust
     match self.query.advance_state(trav) {
         Ok(query_advanced) => QueryAdvanceResult::Advanced(CompareState {
             query: query_advanced,
             ...
         }),
         Err(_original) => QueryAdvanceResult::Exhausted(self),
     }
     ```
   - Remove old `as_candidate()` method (or keep as internal helper)tion
   - Add deprecation notice pointing to `advance_to_candidate`
   - Or keep as convenience wrapper

4. **Update advance_query_cursor()** (30 min)
   - Replace manual `.as_candidate()` + `.current_mut().advance()`
   - Use new `advance_to_candidate(|c| c.advance(trav))`
   - Handle Ok/Err results

### Phase 4: Update All Callers (120 min)

**Pattern:** Replace `.current()` with pattern matching or helper

**Files to update (by frequency):**
1. `compare/state/core.rs` - accessor usage (10 sites)
2. `compare/state/transitions.rs` - state transitions (8 sites)
3. `search/mod.rs` - result creation (5 sites)
4. `match/root_cursor/advance.rs` - cursor access (6 sites)
5. `match/iterator.rs` - root parent access (2 sites)
6. Tests - all test files (40+ sites)

**Migration strategy:**
```rust
// Old: .current() returns &C
let pos = state.query.current().atom_position;

// New: .current() returns CheckpointedRef
let pos = match state.query.current() {
    CheckpointedRef::Checkpoint(c) => c.atom_position,
    CheckpointedRef::Candidate(c) => c.atom_position,
};

// Or use helper:
let pos = state.query.current().atom_position();
```

**Breakdown:**
- Core comparison state updates: 30 min
- Search/match logic updates: 30 min  
- Test updates: 60 min

### Phase 5: Integrate with MatchResult (60 min)

**File:** `crates/context-search/src/state/matched/mod.rs`

1. **Update MatchResult structure** (10 min)
   ```rust
   pub struct MatchResult {
       pub path: PathCoverage,
       pub cursor: Checkpointed<PatternCursor>,  // Changed!
   }
   ```

2. **Update accessor methods** (15 min)
   - `.cursor()` → returns `&PatternCursor` (checkpoint for compat)
   - Add `.current_cursor()` → returns `CheckpointedRef`
   - Add `.has_advanced_query()` → returns `bool`

3. **Update query_exhausted()** (10 min)
   - Check `.current()` instead of `.cursor`
   - Use pattern matching or helper

4. **Update create_parent_exploration_state()** (25 min)
   **File:** `crates/context-search/src/match/root_cursor/advance.rs`
   
   - Create Checkpointed with candidate when query advanced
   - Use checkpoint path for checkpoint cursor
   - Use current path (advanced) for candidate cursor
   
   ```rust
   let checkpoint_cursor = self.state.query.checkpoint().clone();
   let current_path = self.state.query.current().path(); // Advanced!
   
   let candidate_cursor = PathCursor {
       path: current_path.clone(),
       atom_position: checkpoint_cursor.atom_position,
       _state: PhantomData,
   };
   
   MatchResult {
       path: path_enum,
       cursor: Checkpointed {
           checkpoint: checkpoint_cursor,
           candidate: Some(candidate_cursor),  // Preserve advanced state!
       }
   }
   ```

### Phase 6: Update Response API (30 min)

**File:** `crates/context-search/src/state/result.rs`

1. **Add new methods** (15 min)
   - `current_cursor()` → access advanced cursor
   - `has_advanced_query()` → check for candidate
   - Keep `query_cursor()` for backward compat

2. **Update existing methods** (15 min)
   - `query_pattern()` → use `.current()` or `.checkpoint()`?
   - `cursor_position()` → checkpoint position
   - Document which use checkpoint vs current

### Phase 7: Fix Tests (90 min)

**Files:** `crates/context-search/src/tests/search/consecutive.rs` and others

1. **Update consecutive test** (30 min)
   ```rust
   // After first search
   let fin1 = graph.find_ancestor(&query).unwrap();
   assert_eq!(fin1.end.cursor.checkpoint().path.end_index(), 2);
   assert!(fin1.end.cursor.has_candidate());
   assert_eq!(fin1.end.cursor.current().path().end_index(), 3);
   
   // Start second search with current (advanced) cursor
   let query = fin1.end.cursor.current().clone();
   let fin2 = graph.find_ancestor(&query).unwrap();
   ```

2. **Update other test assertions** (40 min)
   - Replace `.cursor` with `.cursor.checkpoint()` or `.cursor.current()`
   - Update pattern matching for new CheckpointedRef

3. **Run full test suite** (20 min)
   - Fix any remaining compilation errors
   - Verify all tests pass

### Phase 8: Documentation (45 min)

1. **Update CHEAT_SHEET.md** (15 min)
   - Add Checkpointed API patterns
   - Document space optimization
   - Show advancement patterns

2. **Update HIGH_LEVEL_GUIDE.md** (15 min)
   - Explain checkpoint vs current semantics
   - Document when candidate is stored
   - Show MatchResult usage

### Total Effort: ~9 hours (reduced from 9.5 by using StateAdvance)ted.rs docs** (15 min)
   - Document space optimization strategy
   - Explain when candidate is None vs Some
   - Add usage examples

### Total Effort: ~9.5 hours

## Migration Checklist

- [ ] Phase 1: Enhance Checkpointed core structure
  - [ ] Update structure with Option<C>
  - [ ] Add CheckpointedRef enum
  - [ ] Update current() to return CheckpointedRef
  - [ ] Add helper methods
  - [ ] Update constructors

- [ ] Phase 2: Add AdvanceCheckpointed trait
  - [ ] Define trait
  - [ ] Implement for PathCursor<Matched>
  - [ ] Implement for ChildCursor<Matched>

- [ ] Phase 3: Update state transitions
  - [ ] Update mark_match()
  - [ ] Update mark_mismatch()
  - [ ] Update advance_query_cursor()
  - [ ] Deprecate old as_candidate()

- [ ] Phase 4: Update all callers
  - [ ] Core comparison state (10 sites)
  - [ ] Search/match logic (5-6 sites each)
  - [ ] Tests (40+ sites)

- [ ] Phase 5: Integrate with MatchResult
  - [ ] Update structure
  - [ ] Update accessors
  - [ ] Update query_exhausted()
  - [ ] Fix create_parent_exploration_state()

- [ ] Phase 6: Update Response API
  - [ ] Add new methods
  - [ ] Update existing methods
  - [ ] Document API choices

- [ ] Phase 7: Fix tests
  - [ ] Update consecutive test
  - [ ] Update other assertions
  - [ ] Run full test suite

## Key Design Decision: Use StateAdvance ✅

**Resolved:** Implement `StateAdvance` for Checkpointed cursors instead of creating a separate trait.

**Rationale:**
1. ✅ Checkpointed advancement IS a state transition - fits StateAdvance perfectly
2. ✅ Consistent with existing patterns (ParentState, ChildState, CompareState)
3. ✅ Familiar interface - developers already understand StateAdvance
4. ✅ Less new code - reuse established trait
5. ✅ Better integration - works anywhere StateAdvance is expected
6. ✅ Simpler mental model - one trait pattern for all state transitions

**Usage:**
```rust
// Old manual way:
let candidate = matched.as_candidate();
candidate.current_mut().advance(trav);

// New StateAdvance way:
match matched.advance_state(trav) {
    Ok(advanced) => { /* use Checkpointed<Candidate> */ },
    Err(original) => { /* exhausted */ },
}
```

**See:** `agents/analysis/20251203_STATEADVANCE_CHECKPOINTED_INTEGRATION.md` for detailed analysis

## Design Questions

### 1. CheckpointedRef Return Type
  - [ ] Update inline docs

## Design Questions

### 1. CheckpointedRef Return Type

**Options:**
A) Return `CheckpointedRef<'_, C>` enum - requires pattern matching
B) Add `.current_atom_position()`, `.current_path()` helpers - convenience methods
C) Both - enum for flexibility, helpers for common cases

**Recommendation:** C (Both) - Best of both worlds

### 2. StateAdvance Integration ✅ RESOLVED

~~**Breaking changes:**~~
~~- `.current()` return type changes from `&C` to `CheckpointedRef<C>`~~

**Decision: Use StateAdvance trait for advancement**
- ✅ Fits established patterns
- ✅ Consistent interface across state machine
- ✅ No new trait needed
- ✅ Simpler implementation (~30% less code)

**See:** `agents/analysis/20251203_STATEADVANCE_CHECKPOINTED_INTEGRATION.md`

### 3. AdvanceCheckpointed Naming ❌ NOT NEEDED
### 3. Backward Compatibility

**Breaking changes:**
- `.current()` return type changes from `&C` to `CheckpointedRef<C>`
- Direct field access to `.current` no longer works
- Some methods need pattern matching

**Mitigation:**
- Add helper methods for common operations
- Clear migration guide in docs
- Compile errors will catch all issues

**Acceptable?** Yes - internal API, worth the improvement

### 4. MatchResult API
**Which cursor for these methods?**
- `query_pattern()` → checkpoint or current?
  - **Recommendation:** current (what user wants to match next)
- `cursor_position()` → checkpoint position
  - **Recommendation:** checkpoint (confirmed matches)
- `query_exhausted()` → check current or checkpoint?
  - **Recommendation:** current (has more tokens to match?)

### 5. Future Optimization

**Shared start path data:**
```rust
pub(crate) struct Checkpointed<C: HasCheckpoint> {
    checkpoint: C::Checkpoint,
    candidate: Option<CandidateDelta<C>>,  // Only stores delta!
}

pub(crate) struct CandidateDelta<C> {
    end_path: Vec<ChildLocation>,  // Only end path differs
    atom_position: AtomPosition,
    // start_path shared with checkpoint
}
```

**Should we design for this now?**
- **Recommendation:** No - YAGNI. Implement simple version first, optimize later if needed
- Can refactor CandidateDelta as internal detail without API changes

## Risk Assessment

### High Risk
- ❌ None identified

### Medium Risk
- ⚠️ **Migration effort** - Many call sites to update (~70)
  - Mitigation: Compiler catches all issues, systematic approach
- ⚠️ **Test complexity** - CheckpointedRef pattern matching in tests
  - Mitigation: Add helper methods, clear examples

### Low Risk
- ✅ **Performance** - Likely improvement from reduced allocations
- ✅ **Correctness** - Type system enforces invariants
- ✅ **API clarity** - More explicit about checkpoint vs current

## Success Criteria

1. ✅ `find_consecutive1` test passes with correct end_index=3
2. ✅ All existing tests continue to pass
3. ✅ MatchResult can represent both matched and advanced query cursors
4. ✅ Space optimization achieved (no redundant storage when at checkpoint)
5. ✅ Clear, type-safe API for cursor advancement
6. ✅ Documentation updated with new patterns

## Open Questions

1. **CheckpointedRef helpers:** Which accessor methods should we provide?
   - `atom_position()` ✅
   - `path()` ✅
   - `root_parent()` ✅
   - Others?

2. **Error handling:** Should `advance_to_candidate()` return Result or Option?
   - **Recommendation:** Result<Advanced, Exhausted> - more explicit

3. **Clone vs reference:** Should `advance_to_candidate` take `&self` or `self`?
   - **Recommendation:** `&self` - allows reuse, matches current pattern

4. **current_mut():** How to handle mutable access to candidate?
   - Current: `current_mut()` returns `&mut C`
   - Need: Ensure it only works when candidate exists
   - **Recommendation:** Return `Option<&mut C>` or panic if None

## Ready to Proceed?

This plan addresses all your requirements:
- ✅ Reuses `Checkpointed` structure (no re-implementation)
- ✅ Space efficient (Option for candidate storage)
- ✅ Encapsulated advancement (AdvanceCheckpointed trait)
- ✅ Developer friendly (clear API, type safe)
- 🔮 Future ready (can add shared data structures later)

**Awaiting your input on:**
1. Design questions 1-5 above
2. Any concerns about the 9.5 hour estimate
3. Priority/urgency for implementation
4. Any additional requirements or constraints

Let me know when you'd like me to begin implementation!
