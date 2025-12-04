# Plan: Fix interval_graph1 and interval_graph2 Test Failures

**Date:** 2025-12-04  
**Status:** Planning  
**Related:** Lock poisoning fix (now complete)

## Objective

Fix `interval_graph1` and `interval_graph2` tests which are failing with `assertion failed: !res.query_exhausted()`.

## Context

### Failing Tests

Both tests in `crates/context-insert/src/tests/interval.rs`:
- `interval_graph1` (line 206-235)
- `interval_graph2` (line 237-348)

### Current Failures

```
assertion failed: !res.query_exhausted()
```

Both tests expect the search result to **not** be query-exhausted, but the query is being fully matched.

### Files Affected

- `crates/context-insert/src/tests/interval.rs` - Test implementations
- Potentially query/search logic if expectations are correct

## Analysis

### What These Tests Do

These tests verify interval graph construction for partial query matches:
- Set up complex graph structures with patterns
- Search for partial sequences that should match within larger patterns
- Verify the search correctly identifies the partial match (non-exhausted query)

### Possible Causes

1. **Query logic changed** - Search now exhausts queries that previously didn't
2. **Test expectations wrong** - Tests may have been written against incorrect behavior
3. **Graph construction issue** - Patterns not being inserted as expected
4. **API changes** - `query_exhausted()` semantics changed

## Execution Steps

### Phase 1: Investigation

- [ ] Read test logs: `target/test-logs/interval_graph*.log`
- [ ] Understand what query is being searched
- [ ] Trace through search logic to see why query is exhausted
- [ ] Check git history for changes to search behavior
- [ ] Compare with similar passing tests

### Phase 2: Diagnosis

- [ ] Determine if query **should** be exhausted or not
- [ ] Check if graph is constructed correctly (print structure)
- [ ] Verify InitInterval expectations match actual behavior
- [ ] Consult repository memories for context-insert test expectations

### Phase 3: Fix

**Option A: Update test expectations**
- [ ] If query should be exhausted, change assertion to `assert!(res.query_exhausted())`
- [ ] Update test documentation to explain why

**Option B: Fix search logic**
- [ ] If query shouldn't be exhausted, identify bug in search/traversal
- [ ] Fix the logic that's incorrectly exhausting the query
- [ ] Add regression test

**Option C: Fix graph construction**
- [ ] If patterns aren't being inserted correctly, fix insertion logic
- [ ] Verify with additional assertions

### Phase 4: Validation

- [ ] Both tests pass: `cargo test -p context-insert interval_graph`
- [ ] No regressions: `cargo test -p context-insert`
- [ ] Update documentation if behavior changed

## Questions for Investigation

- What query is being searched in each test?
- What pattern should it match?
- Why does the search exhaust the query when test expects it not to?
- Have search semantics changed in recent commits?

## Related Work

- Lock poisoning fix (complete) - these tests no longer poison the lock
- Consider adding more granular assertions to understand intermediate state
