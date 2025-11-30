# Test Ecosystem Improvements Plan

## Problem Statement

The `insert_scenarios::prefix1` test is failing because top-down path positions are traced with incorrect atom positions:
- **Expected (correct):** Position `2`
- **Actual (bug):** Position `1`

This is a **real bug** in the position caching logic during top-down traversal of hierarchical prefix matches with non-empty end paths. The test expectations are correct. A duplicate test (`search_prefix1_h_e_l_l`) had incorrect expectations and has been removed.

**Root Cause:** Position tracking during top-down traversal incorrectly calculates atom positions when navigating through pattern children in prefix matches.

## Analysis Notes

### Test Organization Overview

**Test Structure:**
```
context-search/src/tests/
├── examples.rs           (13 tests) - API usage examples
├── macros.rs             (1 macro)  - assert_not_indices
├── traversal.rs          (3 tests)  - Cache verification with Env1
├── search/
│   ├── ancestor.rs       (10 tests) - Env1-based ancestor search
│   ├── consecutive.rs    (1 test)   - Multi-step search with cursors
│   ├── insert_scenarios.rs (12 tests) - Inline graph construction
│   ├── parent.rs         (1 test)   - Env1-based parent search
│   └── mod.rs            (2 tests)  - find_sequence & find_pattern1
└── state_advance/
    ├── compare_state.rs       (2 tests) - Low-level state tests
    ├── integration.rs         (6 tests) - State advancement flows
    └── parent_compare_state.rs (5 tests) - Parent state transitions
```

**Total: 53 tests**

### Test Categories

#### 1. **High-Level Search Tests** (29 tests)
- `examples.rs` - API usage patterns, integration examples
- `search/ancestor.rs` - Uses Env1, focuses on complete/incomplete matches
- `search/parent.rs` - Uses Env1, tests parent finding
- `search/consecutive.rs` - Multi-step cursor-based search
- `search/insert_scenarios.rs` - **INLINE GRAPHS**, replicates insert scenarios
- `search/mod.rs` - Basic find operations

#### 2. **Cache Verification Tests** (3 tests)
- `traversal.rs` - **Detailed cache assertions with Env1**
  - `prefix1` - Matches [a, bc, d, e] in abcdef
  - `postfix1` - Matches [c, d, ef, ghi] in abcdefghi  
  - `range1` - Matches [bc, d, e] in abcdef

### Failing Test Analysis

**Test:** `insert_scenarios::prefix1`
```rust
Graph: [h, e, l, d] -> ld=[l,d], heldld=[h, e, ld, ld]
Query: [h, e, l, l]
Expected: Prefix match with positions at 2 (CORRECT)
Actual: Prefix match with positions at 1 (BUG)
```

**Error:** Top-down positions cached with wrong atom position:
- **Expected:** `AtomPosition(2)` ✅ Correct!
- **Actual:** `AtomPosition(1)` ❌ Bug!

This affects:
1. `ld` vertex - TD position should be 2, is 1
2. `heldld` vertex - TD position should be 2, is 1
3. `l` atom - TD position should be 2, is 1
4. All `DirectedKey` down positions are off by 1

**Root Cause:** Position tracking during top-down traversal in prefix matches with hierarchical end paths incorrectly calculates the atom position when the end path navigates through pattern children (RolePath with entries).
3. `l` atom - TD position
4. All `DirectedKey` down positions

**Root Cause:** Position tracking during top-down traversal in prefix matches with hierarchical end paths (navigating through pattern children).

### Test Duplication Analysis

#### Prefix Match Tests
1. `insert_scenarios::prefix1` - [h,e,l,l] in heldld=[[h,e,ld,ld]] (**FAILING - test expectations CORRECT**)
2. ~~`insert_scenarios::search_prefix1_h_e_l_l`~~ - **REMOVED** (was duplicate with incorrect expectations)
3. `insert_scenarios::search_infix2_a_b_c_d` - [a,b,c,d] prefix of abcdx
4. `traversal.rs::prefix1` - [a,bc,d,e] in abcdef (uses Env1) - **Need to verify this test**
5. `examples::example_incomplete_prefix` - [a,b] in abc

**KEY FINDING:** 
- `prefix1` test has **CORRECT** expectations (position `2`)
- Implementation has bug producing position `1`
- Duplicate test had wrong expectations and has been removed
- Need to check if `traversal.rs::prefix1` also encounters this bug or tests a different scenario

#### Postfix Match Tests
1. `insert_scenarios::search_pattern1_by_z` - [by,z] postfix
2. `insert_scenarios::search_pattern1_ab_y` - [ab,y] postfix
3. `insert_scenarios::search_postfix1_b_c_d_d` - [b,c,d,d] partial postfix
4. `traversal.rs::postfix1` - [c,d,ef,ghi] in abcdefghi
5. `ancestor.rs::find_ancestor2` - [by,z] postfix (duplicate of #1)
6. `ancestor.rs::find_ancestor3` - [ab,y] postfix (duplicate of #2)
7. `examples::example_hierarchical_ancestor_search` - [b,c,d] postfix
8. `examples::example_incomplete_postfix` - [b,c] postfix

**Duplicates:** Tests #1/#5 and #2/#6 test identical scenarios (postfix from insert scenarios)

#### Range Match Tests  
1. `insert_scenarios::search_pattern2_a_b_y` - [a,b,y] range
2. `insert_scenarios::search_infix1_a_b_y` - [a,b,y] infix/range
3. `insert_scenarios::search_infix1_a_b` - [a,b] infix
4. `traversal.rs::range1` - [bc,d,e] range in abcdef
5. `search/mod.rs::find_pattern1` - [a,b,y,x] range

#### Complete Match Tests
1. `insert_scenarios::search_pattern2_a_b` - [a,b] complete postfix
2. `insert_scenarios::search_infix2_a_b_c_d` - [a,b,c,d] complete prefix
3. `insert_scenarios::search_complete_token_b_c` - [b,c] complete token
4. `insert_scenarios::search_complete_token_a_bc` - [a,bc] complete token
5. `examples::example_basic_sequence_search` - [b,c] complete postfix
6. `ancestor.rs` - Multiple complete matches (b_c, a_bc, ab_c, a_bc_d, a_b_c, long pattern)

### Code Duplication Issues

#### 1. **Graph Setup Duplication**
- Env1 graph (in context-trace) used by ~15 tests
- Inline setup repeated in insert_scenarios.rs (~12 times)
- Simple patterns (abc, abcd) recreated multiple times
- No shared fixtures or builders

#### 2. **Assertion Patterns**
- Manual `assert_eq!` on full Response struct (verbose, brittle)
- Some use `assert_matches!` (better)
- Cache assertions repeated with `FromIterator::from_iter` boilerplate
- `build_trace_cache!` macro exists but underutilized in search tests

#### 3. **Setup Boilerplate**
```rust
let mut graph = HypergraphRef::default();
insert_atoms!(graph, {a, b, c, ...});
insert_patterns!(graph, ...);
let _tracing = init_test_tracing!(&graph);
let query = vec![...];
let response = graph.find_ancestor(&query).unwrap();
```
Repeated ~40 times with slight variations.

#### 4. **Expected Value Construction**
- Manual `Response { end: MatchResult { ... }, cache: TraceCache { ... } }`
- Deeply nested structures hard to read/maintain
- Pattern IDs extracted then used in assertions (verbose)

### Test Coverage Gaps

#### Covered Well ✅
- Complete token matches (atoms forming exact pattern)
- Simple postfix matches (query ends at parent's end)
- Simple prefix matches (query starts at parent's start)
- Basic range/infix matches
- Single-step search from atoms
- State advancement mechanics

#### Gaps/Weak Coverage ⚠️
1. **Hierarchical Prefix with Non-Empty End Paths** ⚠️
   - Only 1 test (`insert_scenarios::prefix1`) - **FAILING**
   - `search_prefix1_h_e_l_l` may have wrong expectations
   - Need tests with varying depths (2-level, 3-level hierarchy)

2. **Multi-Pattern Vertices (Width)**
   - Only one test mentions pattern width (`examples::example_pattern_width`)
   - No systematic testing of ambiguous matches
   - No verification of path selection in multi-pattern scenarios

3. **Edge Cases**
   - Empty patterns (0-width)
   - Single-token parents vs multi-token parents
   - Deeply nested hierarchies (>3 levels)
   - Mixed atom/pattern queries at different hierarchy levels

4. **Negative Cases**
   - Most tests expect success
   - Few tests verify specific error conditions
   - `examples::example_single_atom_error` is one exception

5. **Cache Consistency**
   - Only 3 tests (`traversal.rs`) verify full cache state
   - Most tests ignore cache or test partially
   - No systematic cache verification across scenarios

6. **Consecutive/Cursor-Based Search**
   - Only 1 test (`consecutive::find_consecutive1`)
   - No tests for cursor reuse across different queries
   - No multi-hop search chains

7. **Position Edge Cases**
   - Positions at hierarchy boundaries
   - Zero-width position tracking
   - Position propagation through multiple levels ⚠️ (FAILING AREA)

## Improvement Plan

### Phase 1: Fix Failing Test & Clarify Expectations

**Goal:** Resolve position bug and verify test expectations.

**Tasks:**
1. **Investigate position discrepancy:**
   - Compare `prefix1` vs `search_prefix1_h_e_l_l` expected positions
   - Determine correct position for the scenario
   - Root cause: Position tracking in top-down traversal with hierarchical end paths

2. **Document position calculation rules:**
   - When are positions `1` vs `2` for same query?
   - How do hierarchical paths affect position tracking?
   - Add to `agents/guides/` if not documented

3. **Fix the bug OR update test expectations:**
   - If position `2` is correct: Update `search_prefix1_h_e_l_l` to expect `2`
   - If position `1` is correct: Fix tracing logic in search implementation

4. **Add regression test:**
   - Ensure both inline and Env1-based tests cover this scenario
   - Document why this case is special

### Phase 2: Test Infrastructure Improvements

## Improvement Plan

### Phase 0: Bug Investigation & Fix Preparation

**Goal:** Understand the position caching bug before broader improvements.

**Tasks:**
1. ✅ **Confirmed test expectations are correct** (position should be `2`)
2. ⏭️ **Investigate the bug:**
   - Where in the code is the position calculated during top-down traversal?
   - Why is it producing `1` instead of `2`?
   - What's the rule for position calculation with hierarchical end paths?
   - Check if `traversal.rs::prefix1` also fails or tests different scenario

3. ⏭️ **Locate bug in codebase:**
   - Search for top-down position tracking code
   - Find where positions are cached during prefix matches
   - Identify the off-by-one error or incorrect calculation

4. ⏭️ **Create bug report:**
   - Document in `agents/bug-reports/BUG_PREFIX_POSITION_CACHING.md`
   - Include: scenario, expected behavior, actual behavior, code locations
   - Add to bug-reports INDEX.md

5. ⏭️ **Fix the bug:**
   - Implement correct position calculation
   - Verify all tests pass
   - Document the fix

6. ⏭️ **Document position rules:**
   - Add guide explaining position calculation in hierarchical matches
   - Include examples of when position is atom count vs pattern position
   - Link from CHEAT_SHEET.md

### Phase 1: Test Infrastructure Foundation

**Goal:** Create reusable test infrastructure to reduce duplication and improve readability.

**Tasks (no changes from original plan below):**
/// Macro to assert Response with readable syntax
/// 
/// Example:
/// ```
/// assert_response!(response,
///     path: Prefix { root: heldld, ... },
///     cache: { heldld => (TD: {2 => ...}), ... },
///     exhausted: true
/// );
/// ```
#[macro_export]
macro_rules! assert_response {
    // Implementation
}

/// Macro to assert only cache structure (ignore specific positions)
#[macro_export]
macro_rules! assert_cache_structure {
    // Implementation
}
```

#### 2B: Test Fixtures Module

**Create:** `crates/context-search/src/tests/fixtures/mod.rs`

```rust
/// Common graph structures for reuse
pub mod graphs {
    /// Simple linear: abc, abcd
    pub fn simple_linear() -> TestFixture { ... }
    
    /// Hierarchical: ab, cd, abcd=[ab,cd]
    pub fn simple_hierarchical() -> TestFixture { ... }
    
    /// Complex: Env1-like structure (subset)
    pub fn complex_hierarchical() -> TestFixture { ... }
    
    /// Multi-pattern width scenarios
    pub fn multi_pattern() -> TestFixture { ... }
}

pub struct TestFixture {
    pub graph: HypergraphRef,
    pub atoms: HashMap<&'static str, Token>,
    pub patterns: HashMap<&'static str, (Token, Vec<PatternId>)>,
}

impl TestFixture {
    pub fn search(&self, query: &[Token]) -> Response { ... }
    
    pub fn pattern(&self, name: &str) -> Token { ... }
    
    pub fn pattern_id(&self, name: &str, index: usize) -> PatternId { ... }
}
```

#### 2C: Assertion Helpers

**Create:** `crates/context-search/src/tests/assertions/mod.rs`

```rust
/// Assertion helpers for common patterns

pub fn assert_prefix_match(
    response: &Response,
    expected_root: Token,
    expected_end_pos: AtomPosition,
) { ... }

pub fn assert_postfix_match(
    response: &Response,
    expected_root: Token,
    expected_root_pos: AtomPosition,
) { ... }

pub fn assert_cache_entry(
    cache: &TraceCache,
    vertex: Token,
    expected_bu_positions: &[AtomPosition],
    expected_td_positions: &[AtomPosition],
) { ... }

pub fn assert_hierarchical_path(
    path: &RootedRolePath,
    expected_depth: usize,
    expected_root: Token,
) { ... }
```

### Phase 3: Expand Test Coverage

**Goal:** Fill identified gaps with systematic tests.

#### 3A: Hierarchical Prefix Tests

**Create:** `crates/context-search/src/tests/search/prefix_hierarchical.rs`

```rust
#[test]
fn prefix_2level_empty_end() { ... }

#[test]
fn prefix_2level_1step_end() { ... }  // Like failing test

#[test]
fn prefix_2level_2step_end() { ... }

#[test]
fn prefix_3level_nested() { ... }

#[test]
fn prefix_with_multi_pattern_ambiguity() { ... }
```

#### 3B: Width/Ambiguity Tests

**Create:** `crates/context-search/src/tests/search/multi_pattern.rs`

```rust
#[test]
fn multi_pattern_first_wins() { ... }

#[test]
fn multi_pattern_all_reachable() { ... }

#[test]
fn multi_pattern_different_depths() { ... }

#[test]
fn multi_pattern_cache_includes_all() { ... }
```

#### 3C: Position Tracking Tests

**Create:** `crates/context-search/src/tests/search/position_tracking.rs`

```rust
#[test]
fn position_at_hierarchy_boundary() { ... }

#[test]
fn position_zero_width_pattern() { ... }

#[test]
fn position_through_multiple_levels() { ... }

#[test]
fn position_consistency_bu_td() { ... }
```

#### 3D: Negative Case Tests

**Enhance:** `crates/context-search/src/tests/search/errors.rs`

```rust
#[test]
fn error_single_atom() { ... }  // Exists in examples

#[test]
fn error_pattern_not_found() { ... }

#[test]
fn error_empty_query() { ... }

#[test]
fn error_nonexistent_token() { ... }

#[test]
fn error_invalid_hierarchy() { ... }
```

### Phase 4: Refactor Existing Tests

**Goal:** Apply new infrastructure to existing tests.

#### Tasks:
1. **Migrate insert_scenarios.rs:**
   - Use `test_fixture!` macro for graph setup
   - Use `assert_response!` for cleaner assertions
   - Keep inline graphs (good for independence)
   - Deduplicate identical scenarios

2. **Enhance traversal.rs:**
   - Use `assert_cache_entry` helper
   - Add more cache verification tests
   - Cover all PathCoverage variants

3. **Consolidate ancestor.rs:**
   - Remove duplicates with insert_scenarios
   - Focus on Env1-specific scenarios
   - Use assertion helpers

4. **Update examples.rs:**
   - Keep as integration tests
   - Add more "how-to" scenarios
   - Link to API documentation

### Phase 5: Documentation & Organization

#### 5A: Test Documentation

**Create:** `crates/context-search/src/tests/README.md`

```markdown
# Context-Search Test Suite

## Organization
- `examples/` - Integration tests and API usage
- `search/` - High-level search behavior tests
  - `ancestor.rs` - Ancestor finding
  - `prefix_hierarchical.rs` - Hierarchical prefix matches
  - `multi_pattern.rs` - Width and ambiguity
  - `position_tracking.rs` - Position calculation
  - `errors.rs` - Error conditions
- `traversal/` - Cache verification
- `state_advance/` - State machine unit tests
- `fixtures/` - Reusable graph structures
- `assertions/` - Test helpers

## Writing New Tests
1. Use `test_fixture!` for graph setup
2. Use `assert_response!` for result verification
3. Use helpers from `assertions/` for specific checks
4. Add tracing: `let _tracing = init_test_tracing!(&graph);`

## Coverage Matrix
[Table showing what's tested per PathCoverage variant × hierarchy depth × width]
```

#### 5B: Test Naming Convention

Establish consistent naming:
- `test_<scenario>_<variant>` for unit tests
- `<action>_<query_pattern>_<expected_result>` for search tests
- `error_<condition>` for negative tests

#### 5C: Test Organization

Reorganize files:
```
tests/
├── README.md
├── macros.rs (keep existing)
├── macros/
│   ├── mod.rs
│   ├── builders.rs (new)
│   └── assertions.rs (new)
├── fixtures/
│   ├── mod.rs
│   └── graphs.rs
├── assertions/
│   ├── mod.rs
│   ├── path.rs
│   └── cache.rs
├── search/
│   ├── mod.rs
│   ├── ancestor.rs (refactor)
│   ├── parent.rs (keep)
│   ├── consecutive.rs (keep)
│   ├── insert_scenarios.rs (refactor)
│   ├── prefix_hierarchical.rs (new)
│   ├── multi_pattern.rs (new)
│   ├── position_tracking.rs (new)
│   └── errors.rs (new)
├── traversal/ (or cache/)
│   ├── mod.rs
│   └── verification.rs (from traversal.rs)
├── state_advance/ (keep as-is)
└── examples/ (refactor from examples.rs)
    ├── mod.rs
    ├── basic.rs
    ├── hierarchical.rs
    └── advanced.rs
```

## Implementation Order

### Sprint 1: Fix & Understand (Week 1)
1. ✅ Analyze failing test
2. ✅ Identify duplicate test scenarios
3. ✅ Document findings
4. ⏭️ Fix position bug or clarify expectations
5. ⏭️ Add regression test
6. ⏭️ Document position rules in guides

### Sprint 2: Infrastructure (Week 2)
1. Create test macros (`test_fixture!`, `assert_response!`)
## Implementation Order

### Sprint 0: Bug Fix (Days 1-2) **← START HERE**
1. ✅ Analyze failing test
2. ✅ Identify that test expectations are correct
3. ✅ Remove duplicate test with wrong expectations
4. ⏭️ Investigate position calculation bug in code
5. ⏭️ Create bug report document
6. ⏭️ Fix the position caching bug
7. ⏭️ Verify fix with all tests
8. ⏭️ Document position calculation rules

### Sprint 1: Infrastructure Foundation (Week 1)

### Sprint 4: Refactor (Week 4)
1. Migrate insert_scenarios.rs
2. Enhance traversal.rs
3. Consolidate ancestor.rs
4. Update examples.rs

### Sprint 5: Polish (Week 5)
1. Reorganize file structure
2. Update documentation
3. Create coverage matrix
4. Final cleanup

## Success Metrics

1. **Bug Fixed:** ✅ All tests pass
2. **Coverage Improved:** +20 new tests covering gaps
3. **Duplication Reduced:** <10% duplicate test logic
4. **Readability:** New tests 50% shorter on average
5. **Maintainability:** <5 min to add new scenario test
6. **Documentation:** Complete README with coverage matrix

## Open Questions

1. **Position calculation:** When is position `1` vs `2` for hierarchical prefix with end path?
   - Need to trace through code and document rules
   - May require fix in tracing logic

2. **Test organization:** Keep Env1 in context-trace or move to context-search?
   - Pro move: Better encapsulation, easier to extend
## Open Questions

1. ✅ **Position calculation:** ~~When is position `1` vs `2`?~~ → Position should be `2`, implementation has bug
   - ⏭️ Need to find where the bug is in the code
   - ⏭️ Document the correct calculation rule after fix

2. **Test organization:** Keep Env1 in context-trace or move to context-search?
   - Current approach: Keep as integration, split by theme
   - Alternative: Merge into search/ directory

5. **Cache testing:** How much cache verification per test?
   - Full cache: Only in dedicated cache tests
   - Partial: Use assertions helpers
   - None: Simple behavior tests

## Related Documents

- `agents/guides/INDEX.md` - Check for position tracking guide
- `agents/bug-reports/INDEX.md` - May need bug report for position issue
- `CHEAT_SHEET.md` - Update with test patterns
- `crates/context-search/HIGH_LEVEL_GUIDE.md` - Add test architecture section
