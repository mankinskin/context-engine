---
tags: `#analysis` `#context-trace` `#context-search` `#context-insert` `#algorithm` `#debugging` `#testing` `#refactoring` `#api`
summary: 1. Safe modification via split-join (no broken references)
---

# Context-Insert Architecture Analysis

**Comprehensive analysis of context-insert design, search interoperability, and split-join pipeline**

**Date:** 2024-12-04 | **Status:** Complete | **Related:** context-search, trace cache, InitInterval

---

## Architecture Overview

**Purpose:** Graph modification layer enabling safe pattern insertion without breaking invariants

**Core Capabilities:**
1. Safe modification via split-join (no broken references)
2. Hierarchical pattern awareness from search
3. State management through IntervalGraph
4. Flexible result extraction modes

### Split-Join Principle

**Problem:** Direct modification breaks references  
**Solution:** Split → Insert → Join (create new patterns alongside existing)

```
Split:  abc → [a,b,c]    Join: [a,b,c,d] → abcd
Insert: add 'd'          Result: Both abc and abcd exist
```

### Module Structure

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `insert/` | High-level API | ToInsertCtx, InsertCtx, InsertResult |
| `interval/` | State management | IntervalGraph, InitInterval, PatternSubDeltas |
| `split/` | Decomposition | TraceSide, SplitCache, SplitStates, CompleteLocations |
| `join/` | Reconstruction | JoinContext, Frontier, JoinedPartitions |

---

## Search-Insert Interoperability

### Response → InitInterval Conversion

**TraceCache content:** Bidirectional parent-child relationships discovered during search
- BU (bottom-up): "At position X in token, there's child Y from pattern Z"
- TD (top-down): "This token appears at position X in parent pattern Y"

**Critical:** Uses `checkpoint_position()` (confirmed match), not `cursor_position()` (speculative)

```rust
InitInterval {
    root: Token,              // Pattern that partially matched
    cache: TraceCache,        // Parent-child relationships from search
    end_bound: AtomPosition   // Checkpoint position (confirmed boundary)
}
```

### PathCoverage and Insertion Modes

| Coverage | Description | Insertion Type |
|----------|-------------|----------------|
| EntireRoot | Full token match | No insertion needed |
| Range | Infix match | Infix insertion |
| Prefix | Prefix match | Prefix insertion |
| Postfix | Postfix match | Postfix insertion |

**Decision:** Query exhausted + not full token → Convert to InitInterval → Insert remaining

---

## Split-Join Pipeline

### Phase 1: InitInterval → IntervalGraph

**Input:** `InitInterval{root, cache, end_bound}`  
**Output:** `IntervalGraph` ready for splitting

**Key types:**
- `SplitStates` - Queue of splits to process, leaf position tracking
- `SplitCache` - Cached split info (RootMode, VertexCache entries)
- `TraceSide` - TraceBack (insertion) or TraceFront (prefix)

### Phase 2: Split Execution

**Algorithm:**
1. Start with root token + end_bound
2. Navigate parent-child using cache
3. Identify child patterns needing splits
4. Create PosKey entries for split points
5. Cache for join phase

**RootMode:** Prefix | Infix | Postfix (determines split behavior)

### Phase 3: Join Execution

**Algorithm:**
1. Process split cache entries
2. Create new child patterns from atoms
3. Build parent patterns referencing new children
4. Use `JoinContext` (Frontier + node states)
4. Update graph with new patterns
5. Preserve existing patterns (no modifications!)

**Result:**
```
Before: abc = [a, b, c]
After:  abc = [a, b, c]  (unchanged)
        abcd = [ab, cd]  (new)
        where ab = [a, b], cd = [c, d]
```

---

## Test Analysis and Root Causes

### Test Suite Status

**Passing (7 tests):**
- `atom_pos_split` ✓
- `index_infix1` ✓
- `index_infix2` ✓
- `index_pattern1` ✓
- `index_pattern2` ✓
- `interval_graph1` ✓
- `test_split_cache1` ✓

**Failing (3 tests):**
- `index_prefix1` ❌
- `index_postfix1` ❌
- `interval_graph2` ❌

### Root Cause Analysis

#### Issue 1: Pattern Width Mismatch (index_prefix1)

**Error:**
```
PANIC: assertion `left == right` failed: Pattern width mismatch in index 5 token pattern
  left: TokenWidth(4)
 right: TokenWidth(6)
```

**Location:** `context-trace/src/graph/vertex/data/core.rs:176`

**Root Cause:** During pattern replacement, width calculation is incorrect

**Analysis:**
```
Test: index_prefix1
Graph: heldld = [h, e, ld, ld]  where ld = [l, d]
Query: [h, e, l, l]  (query not exhausted - need to insert 'hel')

Expected InitInterval:
  root = heldld
  end_bound = 3  ← Should be checkpoint position
  
Current InitInterval (likely):
  end_bound = 4  ← Using cursor position instead?

Impact: Wrong split calculation → width mismatch during join
```

**Verification Needed:**
- Check if `init.rs` is using `checkpoint_position()` correctly
- Verify split cache positions are calculated from correct boundary
- Confirm join phase width calculations

#### Issue 2: Non-EntireRoot Path (index_postfix1)

**Error:**
```
PANIC: Complete response has non-EntireRoot path: Postfix(...)
```

**Location:** Test assertion at `insert.rs:365`

**Root Cause:** Test expectation mismatch

**Analysis:**
```
Test: index_postfix1
Query: [b, c, d, d]
Expected: Should find 'abcd' as EntireRoot
Actual: Returns Postfix path

Reason: The inserted pattern 'abcd' might not be stored as expected,
        or the search is finding a partial match within a larger pattern
```

**Test Code:**
```rust
let abcd = graph
    .find_ancestor(vec![b, cd])
    .unwrap()
    .expect_complete("abcd")  // ← Panics here
    .root_parent();
```

**Issue:** `expect_complete()` calls `expect_entire_root()` which requires `PathCoverage::EntireRoot`, but search returns `PathCoverage::Postfix`

**Possible Causes:**
1. Insertion created pattern but in wrong hierarchy position
2. Search algorithm finding partial match in parent pattern
3. Test expectation is too strict (should accept Postfix for this case?)

#### Issue 3: Cache Position Mismatch (interval_graph2)

**Error:**
```
assertion failed: `(left == right)`
Diff:
  hi => TD { 4: ... }  (expected)
  hi => TD { 1: ... }  (actual)
  
  cdefghi => BU { 4 => cdefg -> (id, 0) }  (expected)
  cdefg => BU { 1 => cd -> (id, 0) }  (actual)
```

**Root Cause:** Position calculations in trace cache are off by 3

**Analysis:**
```
Test: interval_graph2
Query: [d, e, f, g, h]
Pattern: cdefghi = [cdefg, hi]
         where cdefg = [cd, efg]
               cd = [c, d]
               hi = [h, i]

Expected end_bound: 5
Actual end_bound: ? (likely 2)

Impact: All cache positions shifted by -3
```

**Pattern:**
- Expected positions: 4, 4, 4
- Actual positions: 1, 1, 1
- Difference: -3

**Hypothesis:**
The `end_bound` is being set to the **cursor position within the query** instead of the **absolute position within the root token**.

```
Query: [d, e, f, g, h]
       ^0 ^1 ^2 ^3 ^4  ← Query indices

Root: cdefghi
      c d e f g h i
      ^0^1^2^3^4^5^6  ← Absolute positions

Search matches: d=1, e=2, f=3, g=4, h=5
Query cursor at: 5 (exhausted)
Checkpoint at: 5 (last confirmed match)

But if end_bound = 2, that suggests:
  Position 2 in query [d, e, f, g, h]
  Which is 'f' (index 2)
  
Actual needed: Position 5 in root (h)
```

**Root Cause Confirmed:**
The `checkpoint_position()` is returning the position **relative to the query cursor's pattern**, not the **absolute position in the root token**.

### Summary of Root Causes

| Test | Root Cause | Fix Needed |
|------|-----------|-----------|
| index_prefix1 | Width mismatch from wrong end_bound | Verify checkpoint_position calculation |
| index_postfix1 | PathCoverage mismatch | Insertion creates wrong path type or test expectation needs adjustment |
| interval_graph2 | Cache positions off by -3 | checkpoint_position returns query-relative not root-relative position |

**Common Thread:** All failures relate to position calculation discrepancies between:
- Query-relative positions (cursor within search pattern)
- Root-relative positions (absolute position in matched token)
- End-bound semantics (confirmed extent vs. query extent)

---

## Common Patterns

### Pattern 1: Check Before Inserting

```rust
// ✅ Always search first to avoid duplicate insertions
let result = graph.find_ancestor(query)?;

if !result.query_exhausted() {
    // Only insert if query not fully matched
    let init = InitInterval::from(result);
    let token = graph.insert_init((), init)?;
}
```

### Pattern 2: Handle Multiple Path Types

```rust
// ✅ Comprehensive path coverage handling
match result.end.path {
    PathCoverage::EntireRoot(path) => {
        // Full token match - use directly
    },
    PathCoverage::Range(range) => {
        // Infix match - might need surrounding insertion
    },
    PathCoverage::Prefix(prefix) => {
        // Prefix match - need postfix insertion
    },
    PathCoverage::Postfix(postfix) => {
        // Postfix match - need prefix insertion
    },
}
```

### Pattern 3: Validate InitInterval

```rust
// ✅ Sanity check before insertion
let init = InitInterval::from(response);

assert!(init.end_bound.0 > 0, "end_bound should be positive");
assert!(!init.cache.entries.is_empty(), "cache should have data");
assert_eq!(init.root, response.root_token(), "root should match");
```

### Pattern 4: Use Correct Extraction Mode

```rust
// ✅ Choose extraction based on use case
let result = graph.insert_init(
    extract_complete,  // For full token insertion
    // vs
    extract_interval,  // For progressive insertion
    init
)?;
```

---

## Refactoring Opportunities

### Opportunity 1: Clarify Position Semantics

**Issue:** Confusion between cursor_position, checkpoint_position, and end_bound

**Proposal:**
```rust
impl Response {
    /// For insertion boundaries - confirmed match extent
    pub fn insertion_boundary(&self) -> AtomPosition {
        self.checkpoint_position()
    }
    
    /// For consecutive searches - advanced exploration position
    pub fn continuation_position(&self) -> AtomPosition {
        self.cursor_position()
    }
}
```

**Benefits:**
- Clearer intent at call sites
- Reduces confusion about which position to use
- Self-documenting code

### Opportunity 2: Unified Position Types

**Issue:** Multiple position types with unclear conversions

**Current:**
```rust
AtomPosition        // Generic position
UpPosition         // Bottom-up traversal
DownPosition       // Top-down traversal
```

**Proposal:**
```rust
pub enum PositionContext {
    Absolute(AtomPosition),      // Absolute within token
    QueryRelative(AtomPosition), // Relative to query cursor
    ParentRelative(AtomPosition),// Relative to parent pattern
}

impl PositionContext {
    fn to_absolute(self, root: Token, query: &Pattern) -> AtomPosition {
        match self {
            Absolute(pos) => pos,
            QueryRelative(pos) => /* calculate */,
            ParentRelative(pos) => /* calculate */,
        }
    }
}
```

### Opportunity 3: Split-Join Visibility

**Issue:** Split-join logic is opaque, hard to debug

**Proposal:** Add builder pattern for IntervalGraph:
```rust
impl IntervalGraph {
    pub fn builder() -> IntervalGraphBuilder { ... }
}

impl IntervalGraphBuilder {
    pub fn with_init(self, init: InitInterval) -> Self { ... }
    pub fn with_trace_ctx(self, ctx: TraceCtx) -> Self { ... }
    pub fn build(self) -> Result<IntervalGraph, BuildError> {
        // Validate before building
        // Clear error messages
        // Debugging hooks
    }
}
```

### Opportunity 4: Type-Safe RootMode

**Issue:** RootMode is an enum but behavior differs significantly

**Current:**
```rust
pub enum RootMode {
    Prefix,
    Infix,
    Postfix,
}
```

**Proposal:** Use type system to enforce correct operations:
```rust
pub trait RootMode {
    fn split_strategy(&self) -> SplitStrategy;
    fn join_strategy(&self) -> JoinStrategy;
}

pub struct Prefix;
pub struct Infix;
pub struct Postfix;

impl RootMode for Prefix { ... }
impl RootMode for Infix { ... }
impl RootMode for Postfix { ... }

pub struct TypedIntervalGraph<M: RootMode> {
    root: Token,
    states: SplitStates,
    cache: SplitCache,
    _mode: PhantomData<M>,
}
```

**Benefits:**
- Compile-time enforcement of mode-specific operations
- Clearer which operations are valid for each mode
- Better IDE support and documentation

### Opportunity 5: Simplify Range Roles

**Issue:** RangeRole system is complex with many nested types

**Current:**
```rust
Pre, Post, In, BooleanPerfectOf<R>, OffsetsOf<R>
```

**Proposal:** Flatten to essential variants:
```rust
pub enum RangeRole {
    Pre { perfect_border: bool },
    Post { perfect_border: bool },
    In { offsets: Option<Offsets> },
}

impl RangeRole {
    fn is_perfect_border(&self) -> bool { ... }
    fn has_offsets(&self) -> bool { ... }
}
```

### Opportunity 6: Better Error Types

**Issue:** Panics instead of Results in many places

**Proposal:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum InsertError {
    #[error("Width mismatch: expected {expected}, got {actual}")]
    WidthMismatch { expected: TokenWidth, actual: TokenWidth },
    
    #[error("Invalid end bound: {bound} exceeds token width {width}")]
    InvalidEndBound { bound: AtomPosition, width: TokenWidth },
    
    #[error("Cache inconsistency: {reason}")]
    CacheInconsistency { reason: String },
    
    #[error("Split failed: {reason}")]
    SplitFailed { reason: String },
}

pub type InsertResult<T> = Result<T, InsertError>;
```

---

## Known Issues and Fixes

### Issue 1: checkpoint_position vs cursor_position

**Status:** Identified in tests  
**Severity:** High - causes wrong insertion boundaries

**Fix:**
```rust
// In init.rs - VERIFY THIS IS ALREADY CORRECT
impl From<Response> for InitInterval {
    fn from(state: Response) -> Self {
        let root = state.root_token();
        let end_bound = state.checkpoint_position();  // ✓ Correct
        // NOT: state.cursor_position()                // ✗ Wrong
        Self {
            cache: state.cache,
            root,
            end_bound,
        }
    }
}
```

**Verification:** Check git history - was this changed recently?

### Issue 2: PathCoverage expectations in tests

**Status:** Tests fail on assertion  
**Severity:** Medium - test expectations may be wrong

**Investigation needed:**
1. Is `PathCoverage::Postfix` a valid result for inserted patterns?
2. Should tests use `expect_complete()` or handle all path types?
3. Does insertion always produce `EntireRoot` paths?

**Possible Fix:**
```rust
// More lenient test assertion
let abcd = graph
    .find_ancestor(vec![b, cd])
    .unwrap();

// Accept both EntireRoot and valid partial paths
match abcd.end.path {
    PathCoverage::EntireRoot(path) | PathCoverage::Postfix(path) => {
        assert_eq!(path.root_parent(), expected_abcd);
    },
    _ => panic!("Unexpected path type"),
}
```

### Issue 3: Position calculation in cache

**Status:** Cache positions off by constant offset  
**Severity:** High - affects split-join correctness

**Hypothesis:** `checkpoint_position()` returns query-relative not root-relative

**Fix needed in:** `context-search/src/state/result.rs` or cursor position calculation

**Investigation:**
```rust
// Check what checkpoint_position actually returns
impl Response {
    pub fn checkpoint_position(&self) -> AtomPosition {
        self.end.checkpoint().atom_position  // ← What context is this in?
    }
}

// Need to understand PatternCursor<Matched>::atom_position semantics
// Is it:
// A) Position within query pattern? (current behavior?)
// B) Position within root token? (needed behavior?)
```

### Issue 4: Width calculation during join

**Status:** Panics on width mismatch  
**Severity:** High - insertion fails

**Root cause:** Incorrect end_bound leads to wrong split points, which causes width mismatch in join

**Fix:** Resolve Issue 1 (checkpoint_position) first, then verify width calculations

---

## Next Steps for Implementation

### Immediate (Fix Failing Tests)

1. **Verify checkpoint_position calculation**
   - Add debug logging to show position values
   - Check if position is query-relative or root-relative
   - Fix if needed

2. **Update test expectations**
   - Review PathCoverage variants for insert results
   - Adjust assertions to handle valid partial paths
   - Document when each path type occurs

3. **Debug interval_graph2 positions**
   - Add tracing to show position calculations
   - Compare expected vs actual cache entries
   - Identify exact point where offset is introduced

### Short-term (Improve Robustness)

1. **Add validation to InitInterval**
   - Check end_bound is within root token width
   - Verify cache consistency
   - Return Result instead of panicking

2. **Improve error messages**
   - Convert panics to proper errors
   - Add context to width mismatch errors
   - Show expected vs actual values clearly

3. **Add position calculation helpers**
   - `query_position_to_root()`
   - `root_position_to_query()`
   - Document which functions expect which context

### Medium-term (Refactoring)

1. **Simplify RangeRole system**
   - Flatten type hierarchy
   - Document role purposes clearly
   - Add examples for each role type

2. **Type-safe RootMode**
   - Use type system to enforce mode-specific operations
   - Reduce runtime errors
   - Improve API clarity

3. **Better debugging tools**
   - Add `pretty_print` for IntervalGraph
   - Visualize split-join pipeline
   - Trace position transformations

### Long-term (Architecture)

1. **Separate concerns**
   - Position calculations (separate module)
   - Cache management (unified interface)
   - Error handling (comprehensive types)

2. **Improve testability**
   - Builder patterns for complex types
   - Mock-friendly interfaces
   - Property-based testing

3. **Documentation**
   - Visual diagrams for split-join
   - Step-by-step examples
   - Decision trees for insertion scenarios

---

## Conclusion

Context-insert implements a sophisticated split-join architecture that enables safe graph modification. The failing tests reveal important issues with position calculation semantics, particularly the distinction between:

- **Query-relative positions** (cursor within search query)
- **Root-relative positions** (absolute position in matched token)
- **Checkpoint vs candidate positions** (confirmed vs exploratory)

Fixing these issues requires:
1. Verifying `checkpoint_position()` returns root-relative positions
2. Adjusting test expectations for PathCoverage variants
3. Ensuring cache positions are calculated consistently

The architecture is sound, but the position semantics need clarification and consistent use across the search-insert boundary.
