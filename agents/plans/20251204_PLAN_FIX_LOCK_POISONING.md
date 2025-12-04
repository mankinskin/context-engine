# Implementation Plan: Fix Lock Poisoning in context-insert Tests

**Date:** 2025-12-04  
**Status:** Ready for Execution  
**Related Analysis:** `agents/analysis/20251204_LOCK_POISONING_ANALYSIS.md`

## Objective

Fix the lock poisoning issue in context-insert tests by handling poisoned locks gracefully and fixing the underlying test failures that cause lock poisoning.

## Context

### Files Affected
- `crates/context-trace/src/tests/env/mod.rs` - Lock poisoning handling
- `crates/context-insert/src/tests/interval.rs` - Tests with assertion failures
- `crates/context-insert/src/tests/insert.rs` - Tests with assertion failures

### Dependencies
- All tests depend on shared `Env1::CONTEXT` static RwLock
- Lock poisoning is caused by 4 failing tests that panic while holding the lock
- `test_split_cache1` is blocked by poisoned lock

### Constraints
- Must maintain test isolation
- Cannot break existing passing tests
- Should preserve the benefits of shared test fixtures
- Must be backwards compatible with existing test patterns

## Analysis

### Current State

**Problem:** Tests share a global `RwLock<Env1>` that becomes poisoned when tests panic:

```rust
// crates/context-trace/src/tests/env/mod.rs:241-243
fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self> {
    CONTEXT.write().unwrap()  // Panics on PoisonError
}
```

**Failing Tests:**
1. `interval_graph1` - `assertion failed: !res.query_exhausted()`
2. `interval_graph2` - `assertion failed: !res.query_exhausted()`  
3. `index_prefix1` - Pattern width mismatch assertion
4. `index_postfix1` - Non-EntireRoot path assertion
5. `test_split_cache1` - Lock poisoning victim

### Desired State

- All tests should pass or fail independently
- Lock poisoning should not cascade failures
- Tests should either:
  - Recover from poisoned locks, OR
  - Not use shared mutable state (refactor to per-test fixtures)

### Key Changes Required

1. **Short-term fix:** Handle poisoned locks gracefully in `get_expected_mut()`
2. **Fix underlying test failures:** Investigate and fix the 4 tests causing poisoning
3. **Long-term (optional):** Refactor to use per-test fixtures instead of shared global state

## Execution Steps

### Phase 1: Handle Lock Poisoning

- [x] Create analysis document explaining the issue
- [x] Create this implementation plan
- [ ] Modify `get_expected_mut()` to handle PoisonError:
  ```rust
  fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self> {
      CONTEXT.write().unwrap_or_else(|poisoned| poisoned.into_inner())
  }
  ```
- [ ] Modify `get_expected()` similarly for read locks
- [ ] Verification: Run `cargo test -p context-insert` - `test_split_cache1` should no longer panic on PoisonError

### Phase 2: Fix Test Failures

#### Fix 2.1: interval_graph1 and interval_graph2
- [ ] Investigate why `!res.query_exhausted()` assertion fails
- [ ] Check if the query logic has changed or the test expectations are wrong
- [ ] Review test logs in `target/test-logs/interval_graph*.log`
- [ ] Fix the assertion or update test expectations
- [ ] Verification: Both tests should pass

#### Fix 2.2: index_prefix1
- [ ] Investigate pattern width mismatch (expects 4, got 6)
- [ ] Review the pattern being created and its expected width
- [ ] Check if token width calculation changed
- [ ] Fix either the width calculation or test expectations
- [ ] Verification: `index_prefix1` should pass

#### Fix 2.3: index_postfix1  
- [ ] Investigate "Complete response has non-EntireRoot path" assertion
- [ ] Understand why the path is Postfix instead of EntireRoot
- [ ] Fix the response path logic or update test expectations
- [ ] Verification: `index_postfix1` should pass

### Phase 3: Validation

- [ ] Run all tests: `cargo test -p context-insert`
- [ ] Verify all 10 tests pass
- [ ] Check no new warnings introduced
- [ ] Manual verification: Tests run in different orders without poisoning

### Phase 4: Documentation

- [ ] Update analysis document with resolution notes
- [ ] Create summary in `agents/implemented/20251204_LOCK_POISONING_FIX.md`
- [ ] Update `agents/analysis/INDEX.md` with new entry
- [ ] Update `agents/implemented/INDEX.md` with new entry
- [ ] Add lesson learned to AGENTS.md about shared test state risks

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Tests have complex dependencies not immediately visible | Medium | High | Review test logs carefully, trace query logic |
| Fixing one test breaks another | Low | Medium | Run full test suite after each fix |
| Lock recovery allows tests with bad state | Low | Low | Document that lock recovery is for graceful failure, not correctness |
| Underlying API changes make tests obsolete | Medium | High | Check git history, consult repository memories |

## Validation Criteria

**How to verify success:**
- [x] Analysis document created and accurate
- [x] Implementation plan created and complete
- [ ] Tests pass: `cargo test -p context-insert` shows 10/10 passing
- [ ] No lock poisoning errors in test output
- [ ] Tests can run in any order without failures
- [ ] Documentation updated with findings and resolution

## Notes

### Questions for User

- Are the failing tests expected to fail (known issues), or should they pass?
- Is there a preference for keeping shared test fixtures vs. refactoring to per-test setup?
- Should we add #[ignore] attributes to known-failing tests during investigation?

### Implementation Strategy

**Two-pronged approach:**

1. **Defensive:** Handle lock poisoning so it doesn't cascade
   - Pros: Immediate relief, isolated failures
   - Cons: Doesn't fix root cause

2. **Corrective:** Fix the tests that are failing
   - Pros: Addresses root cause, tests serve their purpose
   - Cons: May require deeper investigation

**Recommended:** Do both - handle poisoning first (quick win), then fix tests properly.

### Alternative Approaches Considered

#### Option A: Ignore Lock Poisoning (Current)
- Status: Failing 
- Issue: Cascading failures

#### Option B: Handle Poisoned Locks (Recommended)
- Use `unwrap_or_else(|poisoned| poisoned.into_inner())`
- Pros: Prevents cascading failures
- Cons: May hide issues in test data

#### Option C: Remove Shared State (Long-term)
- Refactor to per-test fixtures
- Pros: True test isolation
- Cons: Slower tests, more boilerplate

#### Option D: Use std::sync::Mutex::new per test
- Clone Env1 for each test instead of sharing
- Pros: No lock contention
- Cons: Expensive initialization

**Choice: Option B for immediate fix, Option C for future improvement**

### Deviations from Plan

- Fixed compilation error in `index_pattern2` test first
- Removed unnecessary `HypergraphRef::from` conversions

### Related Work

- Repository memory indicates 6 failing tests were expected in context-insert
- This plan addresses 5 of those failures
- Some failures may be known/acceptable - need clarification

