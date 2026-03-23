---
tags: `#plan` `#context-insert` `#debugging` `#testing` `#api`
summary: Fix `index_postfix1` test which is failing with a path type assertion.
status: 📋
---

# Plan: Fix index_postfix1 Test Path Assertion

**Date:** 2025-12-04  
**Status:** Planning  
**Related:** Lock poisoning fix (now complete)

## Objective

Fix `index_postfix1` test which is failing with a path type assertion.

## Context

### Failing Test

Test in `crates/context-insert/src/tests/insert.rs`:
- `index_postfix1` (lines 330-375)

### Current Failure

```
abcd: Complete response has non-EntireRoot path: Postfix(...)
```

The test expects a complete response to have an `EntireRoot` path, but it's returning a `Postfix` path instead.

### Files Affected

- `crates/context-insert/src/tests/insert.rs` - Test implementation
- Search/response path logic
- Pattern insertion and search traversal

## Analysis

### What This Test Does

The `index_postfix1` test:
- Creates atoms for 'a', 'b', 'c', 'd'
- Inserts patterns: `ab`, `ababcd`
- Searches for partial query `[b, c, d, d]`
- Expects to insert `bcd` and `abcd` tokens
- Asserts `abcd` should have `EntireRoot` path when found

### The Issue

When searching for or inserting `abcd`, the response path is `Postfix` rather than `EntireRoot`. This suggests:
- The token `abcd` is being found as a postfix of `ababcd` rather than as a complete token
- Path classification logic may be incorrect
- Or test expectations are wrong about how `abcd` should be represented

### Possible Causes

1. **Path classification bug** - `EntireRoot` not being set when it should be
2. **Pattern structure** - `abcd` exists but not as a root-level token
3. **Search logic** - Finding `abcd` through `ababcd` creates Postfix path
4. **Test expectations wrong** - Maybe `abcd` should be Postfix in this context

## Execution Steps

### Phase 1: Investigation

- [ ] Read test log: `target/test-logs/index_postfix1.log`
- [ ] Examine the response structure that's returned
- [ ] Check how `abcd` token is inserted into the graph
- [ ] Verify parent-child relationships: `ababcd` -> `abcd`
- [ ] Understand when path should be `EntireRoot` vs `Postfix`

### Phase 2: Diagnosis

- [ ] Determine if `abcd` is a standalone token or just part of `ababcd`
- [ ] Check if insert logic creates `abcd` correctly
- [ ] Verify search finds the right token representation
- [ ] Review path type assignment logic
- [ ] Check assertion macro expectations

### Phase 3: Fix

**Option A: Fix path classification**
- [ ] Correct logic that determines path type
- [ ] Ensure `EntireRoot` is set when appropriate
- [ ] Update path construction in search/insert

**Option B: Fix token structure**
- [ ] Ensure `abcd` is inserted as independent token, not just suffix
- [ ] Verify pattern `[[a, bcd], [ab, cd]]` creates proper structure
- [ ] Check parent relationships are correct

**Option C: Update test expectations**
- [ ] If `Postfix` is correct behavior, update assertion
- [ ] Change to check for complete match regardless of path type
- [ ] Document why path is Postfix

### Phase 4: Validation

- [ ] Test passes: `cargo test -p context-insert index_postfix1`
- [ ] No regressions: `cargo test -p context-insert`
- [ ] Path types are consistent across similar tests

## Questions for Investigation

- When should a path be `EntireRoot` vs `Postfix`?
- Is `abcd` inserted as its own token or only as part of `ababcd`?
- What does the `assert_indices!` macro expect?
- Did path classification logic change recently?

## Related Work

- Lock poisoning fix (complete) - this test no longer poisons the lock
- May be related to response API changes (check recent commits)
- Consider reviewing PathCoverage enum and its semantics
