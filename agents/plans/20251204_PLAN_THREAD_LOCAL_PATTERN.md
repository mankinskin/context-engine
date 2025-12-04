# Plan: Thread-Local Test Environment Pattern

**Date:** 2025-12-04  
**Status:** ✅ Implemented  
**Implementation:** `agents/implemented/20251204_LOCK_POISONING_FIX.md`

## Objective

Establish thread-local test environment pattern to eliminate lock poisoning and enable parallel test execution.

## What Was Implemented

Replaced `lazy_static!` with `thread_local!` in `crates/context-trace/src/tests/env/mod.rs` to give each test thread its own isolated environment.

**Result:** Lock poisoning completely eliminated. Tests run independently.

## Future Thread Synchronization Improvements

### Optional Enhancement 1: Remove RwLock If Not Needed

**Current:**
```rust
thread_local! {
    static CONTEXT: OnceLock<Arc<RwLock<Env1>>> = OnceLock::new();
}
```

**If tests don't need concurrent access within a thread, simplify to:**
```rust
thread_local! {
    static CONTEXT: OnceLock<Env1> = OnceLock::new();
}
```

**Benefits:**
- Simpler code
- No lock overhead
- Direct access to Env1

**Trade-off:** Can't share Env1 between functions in same thread if they need mutable access

### Optional Enhancement 2: Document Pattern in AGENTS.md

Add to testing best practices:
- When to use `thread_local!` vs `lazy_static!` for test fixtures
- How to use `OnceLock` for lazy thread-local initialization
- Why unsafe pointer cast is safe for thread-local storage
- Reference `test_graph.rs` as existing example

### Optional Enhancement 3: Apply Pattern to Other Test Modules

Check if other test modules could benefit from thread-local isolation:
- `crates/context-search/src/tests/` - already uses different pattern
- Other crates with shared test fixtures

## Validation

- [x] No lock poisoning in context-insert tests
- [x] Tests pass/fail independently
- [x] Pattern documented in implementation summary
- [ ] Pattern added to AGENTS.md (optional)
- [ ] Applied to other modules if needed (optional)

## Related Plans

Test failures that were previously hidden by lock poisoning now have dedicated fix plans:
- `20251204_PLAN_FIX_INTERVAL_TESTS.md` - interval_graph1, interval_graph2
- `20251204_PLAN_FIX_INDEX_PREFIX_TEST.md` - index_prefix1
- `20251204_PLAN_FIX_INDEX_POSTFIX_TEST.md` - index_postfix1
