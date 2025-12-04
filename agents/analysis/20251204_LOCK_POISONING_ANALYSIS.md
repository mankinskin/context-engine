# Lock Poisoning Analysis: context-insert Test Failures

**Date:** 2025-12-04  
**Status:** ✅ RESOLVED - Fixed with thread_local!  
**Severity:** High - Blocked 5 tests from running (now fixed)

## Executive Summary

The `test_split_cache1` test in the context-insert crate failed with a lock poisoning error because other tests (`interval_graph1`, `interval_graph2`, `index_prefix1`, `index_postfix1`) panicked while holding a shared `RwLock`, causing it to become poisoned. This prevented subsequent tests from acquiring the lock.

**Resolution:** Replaced `lazy_static!` with `thread_local!` for the test environment, eliminating lock poisoning by giving each test thread its own isolated environment instance.

## Problem Description

### Symptoms

When running `cargo test -p context-insert`, 5 tests fail:
1. **`test_split_cache1`** - Fails with `PoisonError` at `crates/context-trace/src/tests/env/mod.rs:242:25`
2. **`interval_graph1`** - Panics with assertion failure
3. **`interval_graph2`** - Panics with assertion failure  
4. **`index_prefix1`** - Panics with pattern width mismatch assertion
5. **`index_postfix1`** - Panics with non-EntireRoot path assertion

### Root Cause

The tests share a global `lazy_static` `RwLock<Env1>` defined in `crates/context-trace/src/tests/env/mod.rs`:

```rust
lazy_static::lazy_static! {
    pub(crate) static ref
        CONTEXT: Arc<RwLock<Env1>> = Arc::new(RwLock::new(Env1::initialize_expected()));
}
```

The trait methods that access this lock use `.unwrap()`:

```rust
fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self> {
    CONTEXT.write().unwrap()  // Line 242 - panics on PoisonError
}
```

### Failure Sequence

1. Test execution order varies (non-deterministic)
2. Tests like `interval_graph1` or `index_prefix1` run first
3. These tests call `Env1::get_expected_mut()` which acquires the write lock
4. The test encounters an assertion failure and panics **while holding the lock**
5. Rust's `RwLock` marks the lock as "poisoned" when a panic occurs while locked
6. Subsequent tests (like `test_split_cache1`) try to acquire the lock
7. The lock acquisition returns `Err(PoisonError { .. })`
8. The `.unwrap()` panics with the PoisonError message

### Why Lock Poisoning Happens

Rust's `RwLock` uses lock poisoning as a safety mechanism. When a thread panics while holding a lock, the lock is marked as poisoned because:
- The data protected by the lock may be in an inconsistent state
- The panicking code may have left the data partially modified
- Rust prevents other threads from accessing potentially corrupted data

## Evidence

### Error Message
```
🔥 PANIC: panicked at crates/context-trace/src/tests/env/mod.rs:242:25:
called `Result::unwrap()` on an `Err` value: PoisonError { .. }
```

### Code Location
```rust
// File: crates/context-trace/src/tests/env/mod.rs
// Lines 241-243
fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self> {
    CONTEXT.write().unwrap()  // ← This unwrap() panics
}
```

### Test Usage Pattern
```rust
// File: crates/context-insert/src/tests/interval.rs
// Line 71
let env @ Env1 { ... } = &*Env1::get_expected_mut();  // Holds lock during test
```

## Impact Analysis

### Current Impact
- **5 tests blocked** from running in context-insert
- **Non-deterministic failures** - execution order affects which tests fail
- **Cascading failures** - one failing test causes others to fail
- **Development velocity** - developers cannot trust test results

### Affected Files
1. `crates/context-trace/src/tests/env/mod.rs` - Lock implementation
2. `crates/context-insert/src/tests/interval.rs` - Tests using the shared context
3. Test execution framework - non-deterministic ordering

## Underlying Issues

### Issue 1: Shared Mutable State in Tests
Tests share a global mutable state via `CONTEXT`, violating test isolation principles.

### Issue 2: Panic-Unsafe Lock Usage
The code uses `.unwrap()` on lock acquisition, which doesn't handle poisoned locks gracefully.

### Issue 3: Long-Lived Lock Guards
Tests hold write locks for their entire duration (via reference binding), increasing the window for poisoning.

### Issue 4: Pre-existing Test Failures
The tests that poison the lock have their own bugs:
- `interval_graph1`: `assertion failed: !res.query_exhausted()`
- `interval_graph2`: `assertion failed: !res.query_exhausted()`
- `index_prefix1`: Pattern width mismatch assertion
- `index_postfix1`: Complete response has non-EntireRoot path assertion

## Design Considerations

### Why Global State Was Used
The `Env1` struct contains a complex pre-built `HypergraphRef` with many tokens and patterns. Creating this graph for every test is:
- **Expensive**: Many graph insertions
- **Repetitive**: Same setup code across tests
- **Error-prone**: Hard to maintain consistency

### Trade-offs
| Approach | Pros | Cons |
|----------|------|------|
| Shared global state (current) | Fast, DRY | Lock poisoning, no isolation |
| Per-test setup | Isolated, no locks | Slow, repetitive |
| Test fixtures/builders | Reusable, isolated | More upfront work |
| Lazy per-test init | Isolated, lazy | Still repetitive |

## Next Steps

See implementation plan: `agents/plans/20251204_PLAN_FIX_LOCK_POISONING.md`

### Immediate Actions
1. Fix the 4 tests that are panicking and causing lock poisoning
2. Add lock poisoning recovery to `get_expected_mut()`
3. Minimize lock hold duration in tests

### Long-term Solutions
1. Refactor to use per-test fixtures instead of shared global state
2. Implement test builders for common graph setups
3. Add test isolation guidelines to AGENTS.md

## References

- **Rust RwLock documentation**: https://doc.rust-lang.org/std/sync/struct.RwLock.html#poisoning
- **Test file**: `crates/context-insert/src/tests/interval.rs`
- **Lock implementation**: `crates/context-trace/src/tests/env/mod.rs`
- **Related memory**: Repository memory notes test status showing 6 expected failures

## Appendix: Full Test Output

```
running 10 tests
test tests::atom_pos_split ... ok
test tests::insert::index_infix2 ... ok
test tests::insert::index_infix1 ... ok
test tests::insert::index_pattern1 ... ok
test tests::insert::index_pattern2 ... ok
test tests::insert::index_prefix1 ... FAILED
test tests::insert::index_postfix1 ... FAILED
test tests::interval::interval_graph2 ... FAILED
test tests::interval::interval_graph1 ... FAILED
test tests::interval::test_split_cache1 ... FAILED
```

### Key Observation
The lock poisoning is a **symptom**, not the root cause. The real issues are:
1. The 4 tests have legitimate bugs causing assertions to fail
2. Shared mutable state creates dependencies between tests
3. Inadequate error handling for poisoned locks

Fixing the lock poisoning requires addressing both the immediate test failures and the architectural issue of shared mutable test state.

## Resolution

### Solution Implemented

Replaced `lazy_static!` with `thread_local!` in `crates/context-trace/src/tests/env/mod.rs` to give each test thread its own isolated environment:

**Before (lazy_static - shared across all test threads):**
```rust
lazy_static::lazy_static! {
    pub(crate) static ref
        CONTEXT: Arc<RwLock<Env1>> = Arc::new(RwLock::new(Env1::initialize_expected()));
}
```

**After (thread_local - per-thread instances):**
```rust
thread_local! {
    static CONTEXT: RefCell<Option<&'static Arc<RwLock<Env1>>>> = RefCell::new(None);
}

impl TestEnv for Env1 {
    fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self> {
        CONTEXT.with(|cell| {
            let mut borrow = cell.borrow_mut();
            if borrow.is_none() {
                *borrow = Some(Box::leak(Box::new(Arc::new(RwLock::new(Env1::initialize_expected())))));
            }
            borrow.unwrap().write().unwrap()
        })
    }
}
```

### Key Changes

1. **Thread-local storage:** Uses `thread_local!` instead of `lazy_static!`
2. **Lazy initialization:** Each thread initializes its own `Env1` on first access
3. **Box::leak for 'static lifetime:** Required because `thread_local!` `with()` closure can't return guards directly

### Results

- **Before:** 5 passed, 5 failed (including `test_split_cache1` with `PoisonError`)
- **After:** 6 passed, 4 failed (`test_split_cache1` now passes, no more lock poisoning)

The 4 remaining failures are legitimate test bugs unrelated to lock poisoning.

## Original Analysis

Fixing the lock poisoning requires addressing both the immediate test failures and the architectural issue of shared mutable test state.
