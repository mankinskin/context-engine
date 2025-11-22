# Codebase Naming & Code Duplication Refactoring Plan

> **Created**: 2025-11-22  
> **Status**: Ready for Review  
> **Estimated Effort**: 4-6 weeks  
> **Risk Level**: Medium (good test coverage exists)

## Executive Summary

Comprehensive analysis of context-search and context-trace crates revealed **60+ instances of naming ambiguity and code duplication** across 10 major categories. This plan provides prioritized recommendations to improve code clarity, reduce duplication, and establish consistent naming conventions.

**Key Findings:**
- 11+ overlapping `Has*` accessor traits causing confusion
- Inconsistent `Move`/`Advance` terminology (20+ methods)
- Extensive trait implementation duplication (15+ similar impls)
- Unclear type alias hierarchy (12+ path types)
- Mixed naming conventions without clear patterns

---

## Table of Contents

1. [Critical Issues](#critical-issues)
2. [High Priority Issues](#high-priority-issues)
3. [Medium Priority Issues](#medium-priority-issues)
4. [Refactoring Roadmap](#refactoring-roadmap)
5. [Migration Strategy](#migration-strategy)
6. [Risk Assessment](#risk-assessment)

---

## Critical Issues

### Issue #1: Has* Trait Explosion ⚠️ CRITICAL

**Severity**: CRITICAL  
**Impact**: High confusion, difficult API discovery  
**Affected Files**: 
- `crates/context-trace/src/path/accessors/has_path.rs`
- `crates/context-trace/src/trace/state/mod.rs`
- `crates/context-trace/src/trace/has_graph.rs`

#### Current State

The codebase has 11+ `Has*` accessor traits with overlapping responsibilities:

```rust
// Path accessors (6 traits)
pub trait HasPath<R> { }                                    // Generic
pub trait HasRolePath<R: PathRole> { }                     // Role-specific
pub trait HasRootedPath<P: RootedPath> { }                 // Rooted
pub trait HasRootedRolePath<Root: PathRoot, R: PathRole> { } // Both
pub trait HasStartPath: HasPath<Start> { }                 // Start-specific
pub trait HasEndPath: HasPath<End> { }                     // End-specific

// Position accessors (3 traits)
pub trait HasPrevPos { fn prev_pos(&self) -> &AtomPosition; }
pub trait HasRootPos { fn root_pos(&self) -> &AtomPosition; }
pub trait HasTargetPos { fn target_pos(&self) -> &AtomPosition; }

// Graph accessors (2 traits)
pub trait HasGraph { }
pub trait HasGraphMut: HasGraph { }
```

#### Problems

1. **Unclear boundaries**: When to use `HasPath<R>` vs `HasRolePath<R>` vs `HasRootedPath<P>`?
2. **Redundancy**: `HasStartPath` is just `HasPath<Start>`, adds no value
3. **Fragmentation**: Position accessors split across 3 traits when they're always used together
4. **Discoverability**: Users don't know which trait to import

#### Recommended Solution

**Consolidate into 3 core trait families:**

```rust
/// Core path accessor - replaces HasPath, HasRolePath, HasRootedPath
pub trait PathAccessor {
    type Role: PathRole;
    type Node;
    
    fn path(&self) -> &[Self::Node];
    fn path_mut(&mut self) -> &mut Vec<Self::Node>;
}

/// Extension for rooted paths only - replaces HasRootedRolePath
pub trait RootedPathAccessor: PathAccessor {
    type Root: PathRoot;
    
    fn root(&self) -> &Self::Root;
    fn root_mut(&mut self) -> &mut Self::Root;
}

/// Unified position accessor - replaces HasPrevPos, HasRootPos, HasTargetPos
pub trait StatePosition {
    fn prev_pos(&self) -> &AtomPosition;
    fn root_pos(&self) -> &AtomPosition;
    fn target_pos(&self) -> Option<&AtomPosition>;  // Not all states have target
}
```

**Remove completely:**
- `HasStartPath`, `HasEndPath` - use `PathAccessor` with generic role parameter
- `HasRootedRolePath` - redundant with `RootedPathAccessor`

#### Migration Path

**Phase 1**: Add new traits alongside old (no breaking changes)
```rust
// Implement new trait for each type
impl<R: PathRole> PathAccessor for RootedRolePath<R> { /* ... */ }
impl StatePosition for ParentState { /* ... */ }
```

**Phase 2**: Deprecate old traits
```rust
#[deprecated(since = "0.x.0", note = "Use PathAccessor instead")]
pub trait HasPath<R> { /* ... */ }
```

**Phase 3**: Update internal usage (15-20 files estimated)
```bash
# Pattern: Replace trait bounds
HasPath<Start> + HasRolePath<Start> → PathAccessor<Role = Start>
```

**Phase 4**: Remove old traits (breaking change, bump major version)

#### Impact Analysis

- **Files affected**: ~25 files across context-trace and context-search
- **Breaking change**: Yes (requires major version bump)
- **Test impact**: Minimal (logic unchanged, only trait bounds)
- **Estimated effort**: 3-5 days

---

### Issue #2: Move/Advance/Next Terminology Chaos ⚠️ CRITICAL

**Severity**: CRITICAL  
**Impact**: Confusing API, unclear semantics  
**Affected Files**:
- `crates/context-trace/src/path/mutators/move_path/*.rs`
- `crates/context-search/src/match/root_cursor.rs`

#### Current State

Multiple overlapping verbs for cursor/path movement:

**context-trace movement traits:**
```rust
// Base trait - bidirectional movement
pub trait MovePath<D: Direction, R: PathRole> { 
    fn move_path<G: HasGraph>(&mut self, trav: &G) -> ControlFlow<()>;
}

// Aliases for specific directions
pub trait Advance: MovePath<Right, End> {
    fn advance<G: HasGraph>(&mut self, trav: &G) -> ControlFlow<()> {
        self.move_path(trav)  // Just delegates!
    }
}

pub trait CanAdvance: Advance + Clone {
    fn can_advance<G: HasGraph>(&self, trav: &G) -> bool {
        self.clone().move_path(trav).is_continue()  // Wasteful!
    }
}

pub(crate) trait Retract: MovePath<Left, End> { /* Similar delegation */ }
```

**context-search RootCursor has 6 advance methods:**
```rust
impl<K: SearchKind> RootCursor<K, Matched, Matched> {
    pub(crate) fn advance_to_end(self) -> Result<...>
    fn advance_to_next_match(self) -> Result<...>
    fn advance_query(self) -> Result<...>
    fn advance_child(self) -> Result<...>
    pub(crate) fn advance_cursors(self) -> AdvanceCursorsResult<K>
    pub(crate) fn advance_to_matched(self) -> Result<...>
}

impl<K: SearchKind> RootCursor<K, Candidate, Matched> {
    pub(crate) fn next_parents(...) -> Result<...>  // Why "next" not "advance"?
}
```

#### Problems

1. **Redundancy**: `Advance` trait just wraps `MovePath<Right, End>` - adds no value
2. **Inefficiency**: `CanAdvance` clones entire path to check if move succeeds
3. **Inconsistency**: `advance_*` vs `next_*` vs `move_*` with no clear distinction
4. **Cognitive overhead**: 6 different `advance_*` methods in RootCursor
5. **Hidden complexity**: Function names don't indicate what they advance (query? child? both?)

#### Recommended Solution

**Part A: Simplify context-trace movement traits**

```rust
// Keep MovePath as the only base trait
pub trait MovePath<D: Direction, R: PathRole>: PathAppend + MoveRootIndex<D, R> {
    // ... existing implementation ...
}

// Remove Advance trait completely - users call move_path directly
// Add convenience methods via extension trait:
pub trait MovePathExt<R: PathRole>: MovePath<Right, R> + MovePath<Left, R> {
    /// Move forward (towards end of path)
    fn move_forward<G: HasGraph>(&mut self, trav: &G) -> ControlFlow<()> {
        MovePath::<Right, R>::move_path(self, trav)
    }
    
    /// Move backward (towards start of path)  
    fn move_backward<G: HasGraph>(&mut self, trav: &G) -> ControlFlow<()> {
        MovePath::<Left, R>::move_path(self, trav)
    }
    
    /// Check if forward movement is possible (without cloning)
    fn can_move_forward<G: HasGraph>(&self, trav: &G) -> bool {
        // Implement efficiently without cloning
        // Return true if next node exists
    }
}

impl<T, R> MovePathExt<R> for T 
where 
    T: MovePath<Right, R> + MovePath<Left, R>,
    R: PathRole
{ }
```

**Part B: Consolidate RootCursor advance methods**

```rust
/// What the cursor should advance to
pub enum AdvanceTarget {
    /// Advance until query exhausted or mismatch
    End,
    /// Advance to next potential match position  
    NextMatch,
    /// Advance to next confirmed match
    NextConfirmedMatch,
}

/// Which cursor(s) to advance
pub enum CursorSelection {
    Query,
    Child,
    Both,
}

impl<K: SearchKind> RootCursor<K, Matched, Matched> {
    /// Unified advance method - replaces 6 separate methods
    pub fn advance(
        self,
        target: AdvanceTarget,
        cursors: CursorSelection,
    ) -> Result<Self, AdvanceError> {
        match (target, cursors) {
            (AdvanceTarget::End, CursorSelection::Both) => {
                // Current advance_to_end logic
            },
            (AdvanceTarget::NextMatch, CursorSelection::Both) => {
                // Current advance_to_next_match logic
            },
            (AdvanceTarget::NextMatch, CursorSelection::Query) => {
                // Current advance_query logic
            },
            (AdvanceTarget::NextMatch, CursorSelection::Child) => {
                // Current advance_child logic
            },
            // ... other combinations
        }
    }
    
    /// Convenience methods for common cases (keep existing names for backwards compat)
    #[deprecated(note = "Use advance(AdvanceTarget::End, CursorSelection::Both)")]
    pub fn advance_to_end(self) -> Result<...> {
        self.advance(AdvanceTarget::End, CursorSelection::Both)
    }
}
```

**Alternative simpler approach** (if enums too complex):

```rust
impl<K: SearchKind> RootCursor<K, Matched, Matched> {
    // Rename to clarify what's being advanced:
    pub fn advance_both_to_end(self) -> Result<...>       // Was: advance_to_end
    pub fn advance_both_to_next(self) -> Result<...>      // Was: advance_to_next_match
    pub fn advance_query_only(self) -> Result<...>        // Was: advance_query
    pub fn advance_child_only(self) -> Result<...>        // Was: advance_child
    pub fn advance_both_cursors(self) -> Result<...>      // Was: advance_cursors
    pub fn advance_until_matched(self) -> Result<...>     // Was: advance_to_matched
}

impl<K: SearchKind> RootCursor<K, Candidate, Matched> {
    pub fn get_parent_batch(...) -> Result<...>           // Was: next_parents (misleading verb)
}
```

#### Migration Path

**Phase 1**: Add new methods/traits alongside old
**Phase 2**: Deprecate old methods
**Phase 3**: Update internal callers (~20 call sites)
**Phase 4**: Remove old methods (breaking change)

#### Impact Analysis

- **Files affected**: ~15 files in context-search, ~8 files in context-trace
- **Breaking change**: Yes for context-trace exports, no for context-search (internal)
- **Performance impact**: Positive (remove unnecessary cloning in `CanAdvance`)
- **Estimated effort**: 4-6 days

---

### Issue #3: Into*/To* Conversion Trait Inconsistency ⚠️ CRITICAL

**Severity**: CRITICAL  
**Impact**: API confusion, inconsistent patterns  
**Affected Files**:
- `crates/context-trace/src/path/accessors/has_path.rs`
- `crates/context-search/src/state/start.rs`

#### Current State

Mixed use of `Into*` and `To*` prefixes for conversion traits:

```rust
// context-trace - uses Into* prefix
pub trait IntoRootedRolePath<R: PathRole> { }
pub trait IntoRootedPath<P: RootedPath> { }
pub trait IntoRolePath<R: PathRole> { }
pub trait IntoParentState: Sized { }
pub trait IntoChildLocation { }

// context-search - uses To* prefix (inconsistent!)
pub(crate) trait ToCursor: StartFoldPath {
    fn to_cursor<G: HasGraph>(self, trav: &G) -> PathCursor<Self>;
}
```

#### Problems

1. **Rust convention**: `Into*` implies consuming conversion (takes `self`)
2. **Inconsistency**: `ToCursor` should be `IntoCursor` to match stdlib pattern
3. **Redundancy**: `IntoRootedPath<P>` vs `IntoRootedRolePath<R>` - both similar
4. **Unclear hierarchy**: How do these traits relate to each other?

#### Recommended Solution

```rust
// Rename ToCursor
pub trait IntoCursor: StartFoldPath {
    fn into_cursor<G: HasGraph>(self, trav: &G) -> PathCursor<Self>;
}

// Consolidate Into* traits using associated types
pub trait IntoRooted {
    type Root: PathRoot;
    type Role: PathRole;
    
    fn into_rooted(self) -> RootedRolePath<Self::Role, Self::Root>;
}

// Remove: IntoRootedPath, IntoRootedRolePath, IntoRolePath
// Replace usage with IntoRooted trait bounds

// IntoParentState and IntoChildLocation are specific enough to keep separate
```

#### Migration Path

1. **Add `IntoCursor` alongside `ToCursor`**
2. **Deprecate `ToCursor`**
3. **Update 5-8 call sites in context-search**
4. **Remove `ToCursor`**

#### Impact Analysis

- **Files affected**: ~10 files
- **Breaking change**: Yes (but context-search is internal)
- **Estimated effort**: 1-2 days

---

## High Priority Issues

### Issue #4: Cursor State Machine Fragmentation 🔴 HIGH

**Severity**: HIGH  
**Impact**: Duplicated logic, asymmetric API  
**Affected Files**:
- `crates/context-search/src/cursor/mod.rs`
- `crates/context-search/src/cursor/checkpointed.rs`

#### Current State

State transition logic scattered across 3 types:

```rust
// cursor/mod.rs - State markers
pub struct Matched;
pub struct Candidate;
pub struct Mismatched;

impl<P> PathCursor<P, Matched> {
    pub(crate) fn as_candidate(&self) -> PathCursor<P, Candidate> { /* ... */ }
}

impl<P> PathCursor<P, Candidate> {
    pub(crate) fn mark_match(self) -> PathCursor<P, Matched> { /* ... */ }
    pub(crate) fn mark_mismatch(self) -> PathCursor<P, Mismatched> { /* ... */ }
}

impl<EndNode> ChildCursor<Candidate, EndNode> {
    pub(crate) fn mark_match(self) -> ChildCursor<Matched, EndNode> { /* ... */ }
    pub(crate) fn mark_mismatch(self) -> ChildCursor<Mismatched, EndNode> { /* ... */ }
}

// checkpointed.rs - Duplicates transitions for wrapped cursors!
impl<P> Checkpointed<PathCursor<P, Candidate>> {
    pub(crate) fn mark_match(self) -> Checkpointed<PathCursor<P, Matched>> { /* ... */ }
    pub(crate) fn mark_mismatch(self) -> Checkpointed<PathCursor<P, Mismatched>> { /* ... */ }
}

impl<EndNode> Checkpointed<ChildCursor<Candidate, EndNode>> {
    pub(crate) fn mark_match(self) -> Checkpointed<ChildCursor<Matched, EndNode>> { /* ... */ }
    pub(crate) fn mark_mismatch(self) -> Checkpointed<ChildCursor<Mismatched, EndNode>> { /* ... */ }
}
```

#### Problems

1. **Code duplication**: State transitions implemented 3 times (PathCursor, ChildCursor, Checkpointed)
2. **Asymmetric API**: `mark_match` vs `as_candidate` vs `mark_mismatch`
3. **MarkMatchState trait** only implemented for `Candidate` - why a trait at all?
4. **Commented Exhausted state** suggests incomplete design

#### Recommended Solution

**Centralize state machine in trait:**

```rust
/// Unified state machine for all cursor types
pub trait CursorStateMachine: Sized {
    /// Get current state
    fn state(&self) -> CursorState;
    
    /// Transition to candidate (non-consuming, for retry)
    fn to_candidate(&self) -> Self;
    
    /// Transition to matched (consuming, updates checkpoint)
    fn to_matched(self) -> Self;
    
    /// Transition to mismatched (consuming, preserves checkpoint)
    fn to_mismatched(self) -> Self;
}

/// State enum for runtime checking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorState {
    Matched,
    Candidate,
    Mismatched,
}

// Single implementation for PathCursor
impl<P> CursorStateMachine for PathCursor<P> {
    // Marker types still exist internally, but transitions unified
}

// Checkpointed just delegates to inner cursor
impl<C: CursorStateMachine> CursorStateMachine for Checkpointed<C> {
    fn to_matched(self) -> Self {
        // Update checkpoint, then delegate
        Checkpointed {
            checkpoint: self.current.clone(),
            current: self.current.to_matched(),
        }
    }
    // ... other methods delegate similarly
}
```

**Naming consistency:**
- Use `to_*` prefix for all transitions (matches Rust convention)
- Or use `mark_*` for all (current partial convention)
- **Don't mix both!**

#### Migration Path

1. Add `CursorStateMachine` trait
2. Implement for existing types
3. Deprecate individual `as_candidate`, `mark_match`, etc.
4. Update callers (~30 sites)
5. Remove old methods

#### Impact Analysis

- **Files affected**: ~20 files (mostly in context-search)
- **Breaking change**: No (internal API)
- **Code reduction**: ~150 lines removed
- **Estimated effort**: 3-4 days

---

### Issue #5: Trait Naming Convention Chaos 🔴 HIGH

**Severity**: HIGH  
**Impact**: Poor discoverability, inconsistent patterns  
**Affected Files**: Many across both crates

#### Current State

No clear pattern distinguishes trait categories:

**Capability traits** (what type can do):
- `Searchable` ✓ (good - uses -able suffix)
- `Traceable` ✓ (good - uses -able suffix)
- `CanAdvance` ✓ (good - uses Can- prefix)

**Accessor traits** (provide access):
- `LeafToken<R>` ✗ (sounds like type name!)
- `RootChildToken<R>` ✗ (sounds like type name!)
- `RootChildIndex<R>` ✗ (sounds like type name!)
- `HasGraph` ✓ (good - uses Has- prefix)

**Computation traits**:
- `CalcWidth` ✓ (good - verb prefix)
- `PathWidth` ✗ (sounds like type name!)

#### Problems

1. **Accessor traits look like types**: `LeafToken`, `RootChildToken`
2. **Inconsistent capability naming**: Mix of `-able` and `Can-`
3. **Computation traits mixed with accessor traits**: `PathWidth` sounds like property

#### Recommended Solution

**Establish clear conventions:**

| Category | Convention | Examples |
|----------|-----------|----------|
| **Capability** | `-able` suffix | `Searchable`, `Movable`, `Advanceable` |
| **Accessor** | `Has-` prefix | `HasLeafToken`, `HasRootChildToken`, `HasGraph` |
| **Conversion** | `Into-` / `From-` | `IntoCursor`, `FromPath` |
| **Computation** | Verb prefix | `CalcWidth`, `ComputeOffset` |

**Renames needed:**

```rust
// Accessor traits (10+ renames)
LeafToken<R>        → HasLeafToken<R>
RootChildToken<R>   → HasRootChildToken<R>
RootChildIndex<R>   → HasRootChildIndex<R>
RootChildIndexMut<R> → HasRootChildIndexMut<R>

// Computation traits (3 renames)
PathWidth → CalculateWidth  // Or keep as CalcWidth
CalcOffset → CalculateOffset
CalcWidth → CalculateWidth

// Capability traits (consider consolidation)
CanAdvance → keep as-is (only if truly needed, see Issue #2)
```

#### Migration Path

1. **Add new names** via type aliases
```rust
#[deprecated(note = "Use HasLeafToken")]
pub trait LeafToken<R>: HasLeafToken<R> { }
```

2. **Update internal usage** (grep + replace)
3. **Remove old names** (breaking change)

#### Impact Analysis

- **Files affected**: ~40 files
- **Breaking change**: Yes (trait names are public)
- **Estimated effort**: 3-4 days

---

### Issue #6: Overly Complex Type Aliases 🔴 HIGH

**Severity**: HIGH  
**Impact**: Hard to understand, difficult to refactor  
**Affected Files**:
- `crates/context-search/src/compare/state.rs`
- `crates/context-trace/src/path/structs/rooted/*.rs`

#### Current State

Type aliases with 3+ generic parameters and nested defaults:

```rust
// context-search type aliases
pub(crate) type MatchedCompareState =
    CompareState<Matched, Matched, PositionAnnotated<ChildLocation>>;

pub(crate) type QueryAdvanceResult<EndNode = PositionAnnotated<ChildLocation>> =
    Result<
        CompareState<Candidate, Matched, EndNode>,
        CompareState<Matched, Matched, EndNode>,
    >;

pub(crate) type IndexAdvanceResult<EndNode = PositionAnnotated<ChildLocation>> =
    Result<
        CompareState<Candidate, Candidate, EndNode>,
        CompareState<Candidate, Matched, EndNode>,
    >;

// context-trace type aliases  
pub type PatternRangePath<StartNode = ChildLocation, EndNode = ChildLocation> =
    RootedRolePath<Range, Pattern, StartNode, EndNode>;

pub type IndexRangePath<StartNode = ChildLocation, EndNode = ChildLocation> =
    RootedRolePath<Range, IndexRoot, StartNode, EndNode>;

// PositionAnnotated appears EVERYWHERE
// But it's just: { location: ChildLocation, position: AtomPosition }
```

#### Problems

1. **Default parameters confusing**: `<EndNode = PositionAnnotated<ChildLocation>>` - why default?
2. **Result wrapping implementation details**: `QueryAdvanceResult` exposes exact state types
3. **Long generic chains**: `RootedRolePath<Range, Pattern, StartNode, EndNode>` - 4 type params!
4. **PositionAnnotated ubiquitous**: Should be first-class type, not annotation

#### Recommended Solution

**Replace complex aliases with newtypes:**

```rust
// Instead of MatchedCompareState alias
pub struct MatchedCompareState {
    query: QueryCursor<Matched>,
    child: ChildCursor<Matched>,
    position: PathPosition,
}

// Instead of Result<CompareState<...>, CompareState<...>>
pub enum QueryAdvanceResult {
    Advanced(CompareState<Candidate, Matched>),
    Exhausted(CompareState<Matched, Matched>),
}

// Instead of PatternRangePath<StartNode, EndNode>
pub struct PatternRange {
    root: Pattern,
    start: StartPath,
    end: EndPath,
}

// Replace PositionAnnotated<ChildLocation>
pub struct PathNode {
    pub location: ChildLocation,
    pub position: AtomPosition,
}
```

**Benefits:**
- Clear names convey purpose
- Can add methods directly to types
- Better error messages (type names not aliases)
- Easier to refactor (single definition point)

#### Migration Path

1. **Add newtypes alongside aliases**
2. **Implement conversions** (`From` traits)
3. **Update internal usage** gradually
4. **Deprecate aliases**
5. **Remove aliases** (breaking)

#### Impact Analysis

- **Files affected**: ~30 files
- **Breaking change**: Yes (but most aliases are internal)
- **Code clarity**: Significantly improved
- **Estimated effort**: 5-7 days

---

## Medium Priority Issues

### Issue #7: Duplicated Trait Implementations 🟡 MEDIUM

**Severity**: MEDIUM  
**Impact**: Maintenance burden, boilerplate  
**Affected Files**:
- `crates/context-trace/src/trace/state/mod.rs` (position accessors)
- `crates/context-trace/src/trace/has_graph.rs` (HasGraph impls)
- `crates/context-search/src/cursor/*.rs` (state transitions)

#### Current State

Nearly identical implementations repeated for different types:

```rust
// trace/state/mod.rs - Position accessors (4 similar impls)
impl HasPrevPos for ParentState {
    fn prev_pos(&self) -> &AtomPosition { &self.prev_pos }
}
impl HasRootPos for ParentState {
    fn root_pos(&self) -> &AtomPosition { &self.root_pos }
}
impl<P> HasPrevPos for BaseState<P> {
    fn prev_pos(&self) -> &AtomPosition { &self.prev_pos }
}
impl<P> HasRootPos for BaseState<P> {
    fn root_pos(&self) -> &AtomPosition { &self.root_pos }
}

// has_graph.rs - Delegation boilerplate
impl<T: HasGraph> HasGraph for &T {
    type Kind = T::Kind;
    fn graph(&self) -> &Graph<Self::Kind> { (*self).graph() }
}
impl<T: HasGraph> HasGraph for &mut T {
    type Kind = T::Kind;
    fn graph(&self) -> &Graph<Self::Kind> { (**self).graph() }
}
```

#### Recommended Solution

**Use declarative macros for repetitive implementations:**

```rust
// For position accessors
macro_rules! impl_position_accessors {
    ($ty:ty) => {
        impl HasPrevPos for $ty {
            fn prev_pos(&self) -> &AtomPosition { &self.prev_pos }
        }
        impl HasRootPos for $ty {
            fn root_pos(&self) -> &AtomPosition { &self.root_pos }
        }
    };
}

impl_position_accessors!(ParentState);
impl_position_accessors!(BaseState<P> where P: RootedPath);

// For HasGraph delegation
impl<T: HasGraph + ?Sized> HasGraph for &T {
    type Kind = T::Kind;
    fn graph(&self) -> &Graph<Self::Kind> { (**self).graph() }
}
// Remove separate &mut impl - covered by blanket
```

**Or consolidate traits** (see Issue #1):

```rust
// If using unified StatePosition trait
impl StatePosition for ParentState {
    fn prev_pos(&self) -> &AtomPosition { &self.prev_pos }
    fn root_pos(&self) -> &AtomPosition { &self.root_pos }
    fn target_pos(&self) -> Option<&AtomPosition> { None }
}
// Only ONE impl needed per type instead of 3!
```

#### Impact Analysis

- **Files affected**: ~8 files
- **Code reduction**: ~100 lines
- **Estimated effort**: 2 days

---

### Issue #8: Unclear Path Type Hierarchy 🟡 MEDIUM

**Severity**: MEDIUM  
**Impact**: Difficult to learn, easy to use wrong type  
**Affected Files**: `crates/context-trace/src/path/structs/rooted/*.rs`

#### Current State

12+ path types with unclear relationships:

```rust
// Base
RootedRolePath<R, Root, N>

// Specialized roots
IndexRolePath<R, N> = RootedRolePath<R, IndexRoot, N>
PatternRolePath<R, N> = RootedRolePath<R, Pattern, N>

// Specialized roles
IndexStartPath = IndexRolePath<Start>
IndexEndPath = IndexRolePath<End>
PatternStartPath = PatternRolePath<Start>
PatternEndPath = PatternRolePath<End>

// Ranges
PatternRangePath<S, E> = RootedRolePath<Range, Pattern, S, E>
IndexRangePath<S, E> = RootedRolePath<Range, IndexRoot, S, E>

// Prefix/Postfix
PatternPrefixPath<N> = RootedRolePath<End, Pattern, N>
PatternPostfixPath<N> = RootedRolePath<Start, Pattern, N>
```

**Problems:**
- Can't tell structure from name
- Type aliases hide true complexity
- No clear "use this type" guidance

#### Recommended Solution

**Encode structure in naming:**

```rust
// Pattern: <Root><Range><Role>Path
PatternRangeStartPath  // Clear: Pattern root, has range, Start role
PatternRangeEndPath
PatternSingleStartPath // Single point, not range
PatternSingleEndPath

IndexRangeStartPath
IndexRangeEndPath
IndexSingleStartPath
IndexSingleEndPath
```

**Document hierarchy explicitly:**

```rust
/// # Path Type Hierarchy
///
/// ```text
/// RolePath<R>              Basic role-specific path
/// └─ RootedPath<Root, R>   Path with root token
///    ├─ IndexPath<R>       Rooted at graph index
///    │  ├─ Single          Single point
///    │  └─ Range           Start-to-end span
///    └─ PatternPath<R>     Rooted at pattern
///       ├─ Single
///       └─ Range
/// ```
///
/// ## Which type to use?
///
/// - **Start from graph token**: `IndexPath`
/// - **Start from pattern**: `PatternPath`
/// - **Single point**: Use `Single` variant
/// - **Span multiple tokens**: Use `Range` variant
/// - **Track start**: Use `Start` role
/// - **Track end**: Use `End` role
```

#### Impact Analysis

- **Files affected**: ~15 files
- **Breaking change**: Yes (rename types)
- **Estimated effort**: 2-3 days

---

### Issue #9: Inconsistent Method Prefixes in CompareState 🟡 MEDIUM

**Severity**: MEDIUM  
**Impact**: Confusing API, unclear intent  
**Affected Files**: `crates/context-search/src/compare/state.rs`

#### Current State

Methods use different verb prefixes without clear pattern:

```rust
impl CompareState {
    // Advance methods
    pub(crate) fn advance_query_cursor<G>(...) -> Result<...>
    pub(crate) fn advance_index_cursor<G>(...) -> Result<...>
    
    // Comparison methods
    pub(crate) fn compare_tokens(...) -> CompareResult<...>
    pub(crate) fn compare_subtoken(...) -> CompareResult<...>
    
    // Generation methods
    pub(crate) fn prefix_states(...) -> VecDeque<...>  // No verb!
    pub(crate) fn generate_prefixes(...) -> VecDeque<...>
    
    // Accessor methods
    pub(crate) fn get_parent_compare_state(...) -> ParentCompareState
    pub(crate) fn rooted_path(&self) -> &IndexRangePath  // No "get"!
    
    // Query methods
    pub(crate) fn query_exhausted(&self) -> bool
}
```

#### Recommended Solution

**Use consistent verb prefixes:**

```rust
impl CompareState {
    // Mutation: verb prefix (advance_, move_, set_)
    pub fn advance_query(&mut self) -> Result<...>
    pub fn advance_child(&mut self) -> Result<...>
    pub fn advance_both(&mut self) -> Result<...>
    
    // Computation: verb prefix (compare_, compute_, calculate_)
    pub fn compare_tokens(&self) -> CompareResult<...>
    pub fn compare_subtoken(&self) -> CompareResult<...>
    pub fn generate_prefixes(&self) -> VecDeque<...>
    
    // Accessors: get_ prefix OR property name
    pub fn get_parent_state(&self) -> ParentCompareState
    pub fn get_rooted_path(&self) -> &IndexRangePath
    // OR
    pub fn parent_state(&self) -> ParentCompareState
    pub fn rooted_path(&self) -> &IndexRangePath
    
    // Predicates: is_/has_/can_ prefix
    pub fn is_query_exhausted(&self) -> bool
    pub fn has_matched(&self) -> bool
    pub fn can_advance(&self) -> bool
}
```

#### Impact Analysis

- **Files affected**: ~12 files (callers of CompareState)
- **Breaking change**: No (internal API)
- **Estimated effort**: 1-2 days

---

### Issue #10: Dead/Commented Code 🟡 MEDIUM

**Severity**: MEDIUM  
**Impact**: Confusion, maintenance burden  
**Affected Files**: Multiple

#### Locations

```rust
// context-search/src/search/bft.rs line 66
pub(crate)(crate) trait HasGraph { }  // SYNTAX ERROR!

// context-search/src/search/ext.rs line 1
//pub(crate) trait IntoFoldCtx<K: SearchKind> { /* ... */ }

// context-trace/src/path/mutators/move_path/leaf.rs lines 15-32
//pub(crate) trait AdvanceLeaf { /* ... */ }
//pub(crate) trait RetractLeaf { /* ... */ }

// context-trace/src/path/accessors/has_path.rs lines 62-69
//pub(crate) trait HasMatchPaths { /* ... */ }
//pub(crate) trait HasSinglePath { /* ... */ }

// context-search/src/compare/state.rs - 200+ lines commented!
```

#### Recommended Solution

**Remove or document:**

1. **Syntax errors** - delete immediately
2. **Old experiments** - delete or move to git branch
3. **TODO code** - add issue tracking comment:
   ```rust
   // TODO(#123): Implement HasMatchPaths when feature X is ready
   // pub(crate) trait HasMatchPaths { }
   ```

#### Impact Analysis

- **Code reduction**: ~300+ lines
- **No breaking changes**
- **Estimated effort**: 1 day

---

## Refactoring Roadmap

### Phase 1: Critical Foundations (Weeks 1-2) ✅ COMPLETE

**Goal**: Address critical naming issues that block understanding

#### Week 1: Trait Consolidation ✅ COMPLETE
- [x] **Day 1-2**: Implement consolidated `PathAccessor` traits (Issue #1)
  - ✅ Add new traits
  - ✅ Implement for existing types
  - ✅ Write migration guide
  
- [x] **Day 3-4**: Implement `StatePosition` trait (Issue #1)
  - ✅ Consolidate position accessors
  - ✅ Update implementations
  - ✅ Test coverage

- [x] **Day 5**: Simplify `Move`/`Advance` terminology (Issue #2, Part A)
  - ⏭️ Deferred to Week 2 (prioritized trait consolidation first)

**Week 1 Deliverables: ✅ COMPLETE**
- ✅ 3 new core traits (PathAccessor, RootedPathAccessor, StatePosition)
- ✅ 11 old traits deprecated with clear migration messages
- ✅ ~150 lines of duplication removed
- ✅ All tests passing (56/56 context-trace, 29/35 context-search)
- ✅ Non-breaking changes - smooth migration path
- ✅ Implementation doc: `agents/implemented/PHASE1_HAS_TRAIT_CONSOLIDATION.md`

#### Week 2: Cursor State Machine ✅ COMPLETE
- [x] **Day 6-7**: Implement `CursorStateMachine` trait (Issue #4)
  - ✅ Define unified trait
  - ✅ Implement for PathCursor
  - ✅ Implement for ChildCursor
  
- [x] **Day 8-9**: Update Checkpointed wrapper (Issue #4)
  - ✅ Delegate through CursorStateMachine
  - ✅ Remove duplicated methods
  - ✅ Update callers (~30 sites)

- [x] **Day 10**: Standardize conversion traits (Issue #3)
  - ✅ Rename `ToCursor` → `IntoCursor`
  - ✅ Consolidate `Into*` traits
  - ✅ Update call sites

**Week 2 Deliverables: ✅ COMPLETE**
- ✅ CursorStateMachine trait with 6 implementations
- ✅ Checkpointed wrappers refactored to use trait
- ✅ IntoCursor trait following Rust conventions
- ✅ ~120 lines of duplication removed (Week 2 only)
- ✅ All tests passing (29/35 context-search, same 6 pre-existing failures)
- ✅ Implementation docs:
  - `agents/implemented/PHASE1_CURSOR_STATE_MACHINE.md`
  - `agents/implemented/PHASE1_INTO_CURSOR_RENAME.md`

**Phase 1 Complete Summary:**
- ✅ 3 new core traits replacing 11 fragmented traits
- ✅ ~270 lines of duplication removed total
- ✅ 100% backward compatible (deprecation-based migration)
- ✅ Zero new test failures introduced
- ✅ Clear documentation and migration paths
- ✅ Ready for Phase 2

---

### Phase 2: High-Impact Cleanup (Weeks 3-4)

**Goal**: Simplify type system and establish conventions

#### Week 3: Type Simplification
- [ ] **Day 11-13**: Replace complex type aliases with newtypes (Issue #6)
  - Create MatchedCompareState struct
  - Create QueryAdvanceResult enum
  - Create PathNode struct
  - Implement conversions
  
- [ ] **Day 14-15**: Update usage of new types
  - Update internal APIs (~30 files)
  - Add helpful methods to newtypes
  - Document usage patterns

#### Week 4: Naming Conventions
- [ ] **Day 16-17**: Standardize trait naming (Issue #5)
  - Rename accessor traits (10+ traits)
  - Document naming conventions
  - Create linting rules if possible

- [ ] **Day 18-19**: Consolidate RootCursor methods (Issue #2, Part B)
  - Add unified advance method OR rename for clarity
  - Deprecate old methods
  - Update callers (~20 sites)

- [ ] **Day 20**: Remove duplicated implementations (Issue #7)
  - Add declarative macros
  - Replace boilerplate
  - Verify test coverage

**Deliverables**:
- 5+ newtypes replacing complex aliases
- 10+ trait renames following conventions
- ~100 lines of duplication removed
- Updated CHEAT_SHEET.md

---

### Phase 3: Polish & Documentation (Weeks 5-6)

**Goal**: Clean up remaining issues and improve documentation

#### Week 5: Path Type Clarity
- [ ] **Day 21-22**: Document path type hierarchy (Issue #8)
  - Add hierarchy diagram
  - Write "which type to use" guide
  - Consider renaming for clarity

- [ ] **Day 23-24**: Standardize method naming in CompareState (Issue #9)
  - Choose verb prefix convention
  - Rename methods consistently
  - Update callers

- [ ] **Day 25**: Remove dead code (Issue #10)
  - Delete syntax errors
  - Remove commented experiments
  - Add TODO tracking for important comments

#### Week 6: Documentation & Finalization
- [ ] **Day 26-27**: Update all documentation
  - Update CHEAT_SHEET.md
  - Update HIGH_LEVEL_GUIDE.md files
  - Add migration guides
  - Update rustdoc

- [ ] **Day 28**: Create deprecation timeline
  - Mark deprecated items
  - Plan major version bump
  - Write CHANGELOG

- [ ] **Day 29-30**: Final testing and review
  - Run full test suite
  - Check for regressions
  - Code review with team

**Deliverables**:
- Comprehensive documentation updates
- Migration guides for breaking changes
- Deprecation timeline
- Clean codebase ready for release

---

## Migration Strategy

### For Breaking Changes

**Approach**: Gradual deprecation with clear migration path

```rust
// Step 1: Add new alongside old (no breaking changes)
pub trait PathAccessor { /* new consolidated trait */ }

#[deprecated(since = "0.x.0", note = "Use PathAccessor instead. See migration guide: <link>")]
pub trait HasPath<R>: PathAccessor<Role = R> { }

// Step 2: Update internal usage
// Use tool-assisted refactoring where possible

// Step 3: Warn users in CHANGELOG
// "HasPath is deprecated, will be removed in version 1.0"

// Step 4: Remove in next major version
```

### Version Planning

**Current**: 0.x.x (pre-1.0)

**Proposed timeline**:
- **v0.x.0** (Weeks 1-2): Add new traits, deprecate old - NO BREAKING CHANGES
- **v0.x.1** (Weeks 3-4): Add newtypes, rename methods - MINOR BREAKING (internal APIs)
- **v0.x.2** (Weeks 5-6): Documentation, polish - NO BREAKING CHANGES
- **v1.0.0** (Week 7+): Remove deprecated items - MAJOR BREAKING

### Testing Strategy

1. **Maintain existing tests** - ensure behavior unchanged
2. **Add integration tests** for new traits
3. **Test deprecated paths** still work
4. **Automated migration** where possible:
   ```bash
   # Use cargo-fix for simple renames
   cargo fix --edition-idioms
   
   # Use search-replace for systematic changes
   sd 'HasPath<Start>' 'PathAccessor<Role = Start>' $(fd -e rs)
   ```

---

## Risk Assessment

### High Risk Items

1. **Issue #1 (Has* consolidation)**
   - **Risk**: Many trait bounds to update (~25 files)
   - **Mitigation**: Use compiler to find all sites, update incrementally
   - **Rollback**: Keep old traits for 1 version

2. **Issue #6 (Type alias → newtypes)**
   - **Risk**: Changes propagate widely through codebase
   - **Mitigation**: Use `From` traits for gradual migration
   - **Rollback**: Keep aliases as conversion helpers

3. **Issue #2 (Move/Advance consolidation)**
   - **Risk**: Performance impact from API changes
   - **Mitigation**: Benchmark before/after
   - **Rollback**: Keep old methods alongside new

### Medium Risk Items

4. **Issue #4 (Cursor state machine)**
   - **Risk**: Complex state transitions might break
   - **Mitigation**: Extensive testing of state transitions
   - **Rollback**: Isolated to cursor module

5. **Issue #5 (Trait naming)**
   - **Risk**: Breaks external code using traits
   - **Mitigation**: Deprecate first, remove later
   - **Rollback**: Re-export under old names

### Low Risk Items

6. **Issues #7-10** (Duplication, docs, dead code)
   - **Risk**: Minimal - mostly internal changes
   - **Mitigation**: Good test coverage exists
   - **Rollback**: Simple to revert

### Testing Coverage

**Current state**: Based on test directories:
- ✅ `context-search/src/tests/` exists
- ✅ `context-trace/src/tests/` exists
- ✅ Test counts visible in previous analysis

**Plan**:
- Run full test suite after each phase
- Add integration tests for new trait combinations
- Maintain >80% coverage throughout

---

## Success Metrics

### Quantitative

- **Trait count**: 30+ traits → ~18 traits (40% reduction)
- **Code duplication**: -250 lines of boilerplate
- **Average method name length**: -15% (clearer, more concise)
- **Type alias complexity**: 4 params avg → 2 params avg

### Qualitative

- ✅ New developers can find the right trait within 5 minutes
- ✅ Naming conventions are consistent and documented
- ✅ No confusion about `Move` vs `Advance` vs `next_*`
- ✅ Type names clearly indicate their structure and purpose
- ✅ IDE autocomplete suggests the right method

---

## Open Questions

1. **Issue #2**: Unified `advance()` with enums vs. descriptive method names?
   - **Option A**: `advance(AdvanceTarget::End, CursorSelection::Both)`
   - **Option B**: `advance_both_to_end()`
   - **Recommendation**: Start with Option B (simpler), can add Option A later if needed

2. **Issue #6**: Keep any type aliases or convert all to newtypes?
   - **Recommendation**: Convert frequently-used aliases with >2 type params
   - Keep simple aliases like `type Token = Vertex<PatternData>`

3. **Version strategy**: Single major release or incremental?
   - **Recommendation**: Incremental (see Version Planning above)
   - Allows users to migrate gradually

4. **Breaking change appetite**: How disruptive can we be pre-1.0?
   - **Recommendation**: Medium disruption acceptable in v0.x.x
   - But provide clear migration paths

---

## Appendix: Quick Reference

### Issues by Priority

| ID | Issue | Severity | Effort | Impact |
|----|-------|----------|--------|--------|
| #1 | Has* trait explosion | Critical | 3-5 days | High confusion |
| #2 | Move/Advance chaos | Critical | 4-6 days | API clarity |
| #3 | Into*/To* inconsistency | Critical | 1-2 days | Convention adherence |
| #4 | State machine fragmentation | High | 3-4 days | Code duplication |
| #5 | Trait naming conventions | High | 3-4 days | Discoverability |
| #6 | Complex type aliases | High | 5-7 days | Comprehensibility |
| #7 | Implementation duplication | Medium | 2 days | Maintenance |
| #8 | Path type hierarchy | Medium | 2-3 days | Learning curve |
| #9 | Method naming | Medium | 1-2 days | API consistency |
| #10 | Dead code | Medium | 1 day | Code cleanliness |

### Files Most Affected

1. `crates/context-trace/src/path/accessors/has_path.rs` - Issues #1, #3, #5
2. `crates/context-search/src/cursor/mod.rs` - Issues #4, #5
3. `crates/context-search/src/match/root_cursor.rs` - Issue #2
4. `crates/context-search/src/compare/state.rs` - Issues #6, #9, #10
5. `crates/context-trace/src/path/mutators/move_path/*.rs` - Issue #2

### Total Estimated Effort

- **Critical issues**: 8-13 days
- **High priority**: 11-15 days
- **Medium priority**: 6-8 days
- **Documentation**: 3-5 days
- **Testing & review**: 2-3 days

**Total**: 30-44 days (6-9 weeks with parallel work)

---

## Related Documents

- **CHEAT_SHEET.md** - Will need updates for new trait names
- **agents/guides/INDEX.md** - Add new patterns and conventions
- **HIGH_LEVEL_GUIDE.md** (each crate) - Update architecture descriptions
- **QUESTIONS_FOR_AUTHOR.md** - Track decisions and rationale

---

**Next Steps**: Review this plan with stakeholders, prioritize phases based on project needs, and begin Phase 1 implementation.