---
tags: `#analysi` `#debugging` `#testing` `#api`
summary: Analysis of `Checkpointed<C>` usage patterns to determine which cursor state (checkpoint vs candidate) should be accessed at each call site and whe...
---

# Checkpointed State Analysis

## Executive Summary

Analysis of `Checkpointed<C>` usage patterns to determine which cursor state (checkpoint vs candidate) should be accessed at each call site and whether a type-level `MatchState` parameter would improve type safety.

## Current MatchState Type

`MarkMatchState` is a **trait** (not a type parameter) that provides state transitions:

```rust
pub trait MarkMatchState {
    type Matched;
    type Mismatched;
    fn mark_match(self) -> Self::Matched;
    fn mark_mismatch(self) -> Self::Mismatched;
}
```

Implemented for:
- `PathCursor<P, Candidate>` → `PathCursor<P, Matched>` / `PathCursor<P, Mismatched>`
- `ChildCursor<Candidate, N>` → `ChildCursor<Matched, N>` / `ChildCursor<Mismatched, N>`
- `CompareState<Candidate, Candidate, N>` → `CompareState<Matched, Matched, N>` / `CompareState<Mismatched, Mismatched, N>`

## Existing CursorState Types

The codebase already has a `CursorState` trait with three states:
- `Matched` - confirmed match position
- `Candidate` - tentative position being tested
- `Mismatched` - failed match position

These are used as **type parameters on cursors**:
- `PathCursor<P, State>`
- `ChildCursor<State, Node>`

## Checkpointed Structure

Current design:
```rust
pub struct Checkpointed<C: HasCheckpoint> {
    pub(crate) checkpoint: C::Checkpoint,  // Always Matched state
    pub(crate) candidate: Option<C>,       // None when at checkpoint, Some when advanced
}
```

## Usage Pattern Analysis

### Pattern 1: Access Current Position (checkpoint OR candidate)

**Method:** `.current()` - returns `CheckpointedRef` that abstracts over both

**Use cases:**
1. **Reading position for logging/comparison** (90% of uses)
   - `state.query.current().atom_position()` - need current position
   - `state.child.current().child_state()` - need current child state
   - Expected state: **Either** (don't care which, just want current)
   
2. **Decomposing for next step**
   - `self.child.current().decompose_into_prefixes()` - work with current position
   - Expected state: **Either** (operation doesn't depend on which)

3. **Creating new states from current**
   - `state.query.current().clone().mark_match()` - mark current as matched
   - Expected state: **Candidate** (about to mark as matched)

**Observations:**
- Most `.current()` calls don't care if it's checkpoint or candidate
- They just need the current cursor position for reading
- Very few need to know the distinction

### Pattern 2: Access Checkpoint Only

**Method:** `.checkpoint()` - returns `&C::Checkpoint` (always Matched)

**Use cases:**
1. **Get last confirmed match position**
   - `state.query.checkpoint().atom_position` - where we last matched
   - Expected state: **Matched** (by definition)

2. **Create result from checkpoint**
   - `state.query.checkpoint().clone()` - use confirmed position
   - Expected state: **Matched** (creating from checkpoint)

3. **Compare current vs checkpoint**
   - Log both for debugging advancement
   - Expected state: **Matched** (checkpoint) vs **Either** (current)

**Observations:**
- All `.checkpoint()` calls expect Matched state
- This is guaranteed by design (checkpoint is always `C::Checkpoint = Matched`)
- Type safety already enforced

### Pattern 3: State Transitions

**Methods:** `.mark_match()`, `.mark_mismatch()`, `.as_candidate()`

**Use cases:**
1. **Mark current position as matched**
   - `self.query.mark_match()` - move candidate → checkpoint
   - **Input:** `Checkpointed<C: Candidate>` with candidate=Some
   - **Output:** `Checkpointed<C: Matched>` with candidate=None
   - Expected state: **Candidate** → **Matched**

2. **Convert to candidate for next comparison**
   - `self.query.as_candidate()` - prepare for testing
   - **Input:** `Checkpointed<PathCursor<P, Matched>>`
   - **Output:** `Checkpointed<PathCursor<P, Candidate>>`
   - Expected state: **Matched** → **Candidate**

**Observations:**
- Type transitions already tracked at cursor level
- `Checkpointed` wraps cursors that already have state type parameters
- State is controlled by the wrapped cursor's type parameter

### Pattern 4: CompareState Usage

**Structure:**
```rust
pub struct CompareState<Q: CursorState, I: CursorState, EndNode> {
    pub(crate) query: Checkpointed<PathCursor<PatternRangePath, Q>>,
    pub(crate) child: Checkpointed<ChildCursor<I, EndNode>>,
    // ...
}
```

**State combinations used:**
1. `CompareState<Matched, Matched, N>` - both at confirmed match
   - Used after successful match
   - Both checkpoints represent confirmed positions
   
2. `CompareState<Candidate, Candidate, N>` - both exploring
   - Used during active comparison
   - Can call `.mark_match()` to convert to Matched,Matched
   
3. `CompareState<Candidate, Matched, N>` - query exploring, child at checkpoint
   - Used after query advances but before child comparison
   - Query has advanced, child waiting
   
4. `CompareState<Matched, Candidate, N>` - query at checkpoint, child exploring
   - Less common, used in specific traversal patterns

**Observations:**
- State is **already tracked at the CompareState level**
- The `Q` and `I` type parameters control cursor states
- `Checkpointed` wraps cursors that have these states

## Current Type Safety

The type safety chain:
1. **Cursor level:** `PathCursor<P, S: CursorState>` - state is type parameter
2. **Checkpointed level:** `Checkpointed<C>` where `C` is the cursor type with its state
3. **CompareState level:** `CompareState<Q: CursorState, I: CursorState>` - controls both cursors' states

Example:
```rust
Checkpointed<PathCursor<PatternRangePath, Matched>>  // Wraps Matched cursor
Checkpointed<PathCursor<PatternRangePath, Candidate>> // Wraps Candidate cursor
```

The `C::Checkpoint` associated type ensures checkpoint is always the Matched version:
```rust
impl<P, S: CursorState> HasCheckpoint for PathCursor<P, S> {
    type Checkpoint = PathCursor<P, Matched>;  // Always Matched
    //...
}
```

## Analysis: Should Checkpointed Have a State Type Parameter?

### Current Design
```rust
pub struct Checkpointed<C: HasCheckpoint> {
    checkpoint: C::Checkpoint,  // Always Matched
    candidate: Option<C>,       // State determined by C's type parameter
}
```

### Proposed Design
```rust
pub struct Checkpointed<C: HasCheckpoint, S: CheckpointState> {
    checkpoint: C::Checkpoint,  // Always Matched
    candidate: Option<C>,       // Presence/state controlled by S
}
```

### Option A: CheckpointState Controls Candidate Presence

```rust
pub trait CheckpointState {
    const HAS_CANDIDATE: bool;
}

pub struct AtCheckpoint;  // candidate = None
pub struct Advanced;      // candidate = Some

impl CheckpointState for AtCheckpoint {
    const HAS_CANDIDATE: bool = false;
}

impl CheckpointState for Advanced {
    const HAS_CANDIDATE: bool = true;
}
```

**Benefits:**
- Type-level guarantee: `Checkpointed<C, AtCheckpoint>` → candidate is None
- Type-level guarantee: `Checkpointed<C, Advanced>` → candidate is Some
- `.checkpoint()` only available on `AtCheckpoint`
- `.candidate()` only available on `Advanced`

**Drawbacks:**
- Adds complexity: need to track two state parameters (cursor's + checkpointed's)
- Most code doesn't care: 90% of `.current()` calls don't need the distinction
- State transitions become more verbose
- Need conversions between `AtCheckpoint` ↔ `Advanced`

### Option B: State Mirrors Cursor State

```rust
pub struct Checkpointed<C: HasCheckpoint> {
    checkpoint: C::Checkpoint,  // Always Matched
    candidate: Option<C>,       // C's state determines behavior
}

// For PathCursor<P, Matched>:
impl Checkpointed<PathCursor<P, Matched>> {
    // Can only access checkpoint (since cursor is Matched)
    pub fn checkpoint(&self) -> &PathCursor<P, Matched> { &self.checkpoint }
    pub fn current(&self) -> &PathCursor<P, Matched> {
        self.candidate.as_ref().unwrap_or(&self.checkpoint)
    }
}

// For PathCursor<P, Candidate>:
impl Checkpointed<PathCursor<P, Candidate>> {
    // Can access both (since cursor is Candidate, might have advanced)
    pub fn checkpoint(&self) -> &PathCursor<P, Matched> { &self.checkpoint }
    pub fn candidate(&self) -> Option<&PathCursor<P, Candidate>> { self.candidate.as_ref() }
}
```

**Benefits:**
- Reuses existing state system
- Type already tracked: `Checkpointed<PathCursor<P, Matched>>` vs `Checkpointed<PathCursor<P, Candidate>>`
- Methods can be state-specific via impl blocks

**Drawbacks:**
- Doesn't solve the core issue: candidate presence is runtime (Option)
- State mismatch possible: `Checkpointed<PathCursor<P, Matched>>` with candidate=Some is nonsensical

## Recommendation

**Keep the current design** for these reasons:

1. **State is already tracked where it matters** - at the cursor and CompareState levels
   
2. **Most code doesn't need the distinction** - 90% of uses just call `.current()` for reading

3. **The runtime Option<C> is semantically correct**:
   - `candidate: None` = "at checkpoint"
   - `candidate: Some(c)` = "advanced beyond checkpoint"
   - This is independent of whether cursor is Matched/Candidate

4. **Type safety where it counts**:
   - `C::Checkpoint` is always Matched (enforced by trait)
   - Cursor state (`Q`, `I` in CompareState) already tracked
   - CompareState controls overall state machine

5. **Adding state parameter would duplicate existing tracking**:
   - Cursor already has state: `PathCursor<P, Matched>` vs `PathCursor<P, Candidate>`
   - Checkpointed state would be redundant

## Alternative: Better API Design

Instead of adding a state parameter, **improve the accessor methods**:

### Current Issues
```rust
// Can borrow from Checkpointed but candidate might not exist
let current = checkpointed.current();  // Returns CheckpointedRef enum
```

### Better Design: Pattern Matching API

```rust
impl<C: HasCheckpoint> Checkpointed<C> {
    /// Always available - get checkpoint
    pub fn checkpoint(&self) -> &C::Checkpoint;
    
    /// Pattern match on presence of candidate
    pub fn state(&self) -> CheckpointedState<&C::Checkpoint, &C> {
        match &self.candidate {
            None => CheckpointedState::AtCheckpoint(&self.checkpoint),
            Some(c) => CheckpointedState::Advanced { 
                checkpoint: &self.checkpoint, 
                candidate: c 
            },
        }
    }
}

pub enum CheckpointedState<Chk, Cand> {
    AtCheckpoint(Chk),
    Advanced { checkpoint: Chk, candidate: Cand },
}
```

**Benefits:**
- Explicit about runtime state
- Pattern matching forces handling both cases
- Type-safe access to candidate (only when Some)
- No additional type parameters

## Conclusion

**Current design is sound.** The issue isn't missing type-level state tracking - it's that:

1. **State is already tracked** at cursor and CompareState levels
2. **Checkpointed's role** is to manage checkpoint vs advanced position, not cursor state
3. **The Option<C> models the right concept** - presence of advancement

**Recommended improvements:**
- Keep current `Checkpointed<C>` without additional state parameter
- Consider better accessor API (pattern matching enum) if needed
- Focus type safety efforts on CompareState level (already done well)
- Document that cursor state != checkpointed state (orthogonal concerns)

## Usage Summary Table

| Call Site | Method | Expected Value | Cursor State | Notes |
|-----------|---------|---------------|--------------|-------|
| Logging position | `.current().atom_position()` | Either | Any | Just reading |
| Logging checkpoint | `.checkpoint().atom_position` | Checkpoint | Matched | Always checkpoint |
| Mark as matched | `.mark_match()` | Candidate | Candidate | Transitions to Matched |
| Convert to candidate | `.as_candidate()` | Checkpoint | Matched | Creates candidate |
| Decompose for next step | `.current().decompose()` | Either | Any | Work with current |
| Create result | `.checkpoint().clone()` | Checkpoint | Matched | Use confirmed |
| Compare positions | Both `.current()` & `.checkpoint()` | Both | Any | Debugging |
| Update checkpoint | `.candidate.take()` + assign | Candidate | Candidate | State machine |

**Key insight:** The "current" concept (checkpoint OR candidate) is what matters most, not the cursor's Matched/Candidate state. These are orthogonal dimensions:
- **Cursor state:** Matched (confirmed) vs Candidate (testing) vs Mismatched (failed)
- **Position state:** At checkpoint vs Advanced (has candidate)
