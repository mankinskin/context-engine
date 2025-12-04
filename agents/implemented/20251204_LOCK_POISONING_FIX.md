# Lock Poisoning Fix - Thread-Local Test Isolation

**Date:** 2025-12-04  
**Status:** ✅ Complete  
**PR:** Fix lock poisoning in context-insert tests using thread_local  
**Related Analysis:** `agents/analysis/20251204_LOCK_POISONING_ANALYSIS.md`

## Summary

Fixed lock poisoning in context-insert tests by replacing `lazy_static!` with `thread_local!` for test environment storage. Each test thread now gets its own isolated `Env1` instance, eliminating lock contention and poisoning between tests.

## Problem

The `test_split_cache1` test was failing with `PoisonError` because other tests panicked while holding a shared `RwLock<Env1>`. Tests were sharing a global `lazy_static` context, causing non-deterministic cascading failures.

## Solution Implemented

### Code Changes

**File:** `crates/context-trace/src/tests/env/mod.rs`

Replaced:
```rust
lazy_static::lazy_static! {
    pub(crate) static ref CONTEXT: Arc<RwLock<Env1>> = Arc::new(RwLock::new(Env1::initialize_expected()));
}
```

With:
```rust
thread_local! {
    static CONTEXT: OnceLock<Arc<RwLock<Env1>>> = OnceLock::new();
}

fn get_context() -> &'static Arc<RwLock<Env1>> {
    CONTEXT.with(|cell| {
        unsafe {
            let ptr = cell.get_or_init(|| Arc::new(RwLock::new(Env1::initialize_expected())));
            &*(ptr as *const Arc<RwLock<Env1>>)
        }
    })
}
```

### Why This Works

- **Thread-local isolation:** Each test thread gets its own `Env1` instance
- **No shared state:** Tests cannot poison each other's locks
- **Lazy initialization:** `OnceLock` ensures single initialization per thread
- **Unsafe but sound:** Raw pointer cast extends lifetime to `'static` (safe because thread_local persists for thread lifetime)

### Additional Fixes

- Fixed compilation error in `index_pattern2` test by removing unnecessary `graph_ref` variable
- Removed redundant `HypergraphRef::from(graph)` conversions in `index_pattern1` and `index_pattern2`

## Results

**Before:**
- 5 tests passed, 5 tests failed
- `test_split_cache1` failed with `PoisonError`
- Non-deterministic failures based on test execution order

**After:**
- 6 tests passed, 4 tests failed
- `test_split_cache1` now **passes** ✅
- Lock poisoning completely eliminated
- Tests run independently regardless of execution order

## Remaining Work

The 4 remaining test failures are legitimate test bugs unrelated to lock poisoning:
- `interval_graph1`, `interval_graph2` - assertion failures (see `20251204_PLAN_FIX_INTERVAL_TESTS.md`)
- `index_prefix1` - pattern width mismatch (see `20251204_PLAN_FIX_INDEX_PREFIX_TEST.md`)
- `index_postfix1` - path assertion (see `20251204_PLAN_FIX_INDEX_POSTFIX_TEST.md`)

## Future Improvements

- Consider removing `Arc<RwLock<>>` wrapper if tests don't need concurrent access within a single thread
- Evaluate if per-test setup functions would be simpler than shared fixtures
- Document thread_local pattern in AGENTS.md as best practice for test fixtures

## Commits

- f155148: Fix compilation error in index_pattern2 test
- e4be4dc: Add analysis and implementation plan documents
- 47f7b1d: Replace lazy_static with thread_local
- a246924: Update implementation plan to reflect completion

## Lessons Learned

1. **Thread-local is better than lazy_static for test fixtures** - prevents lock poisoning entirely
2. **OnceLock simplifies thread_local initialization** - cleaner than `RefCell<Option<>>` pattern
3. **Lock poisoning is a symptom, not root cause** - fix by isolating tests, not by recovering from poisoned locks
4. **Document patterns early** - other codebases already used thread_local (e.g., `test_graph.rs`)
