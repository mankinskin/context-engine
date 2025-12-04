# Plan: Fix index_prefix1 Test Pattern Width Mismatch

**Date:** 2025-12-04  
**Status:** Planning  
**Related:** Lock poisoning fix (now complete)

## Objective

Fix `index_prefix1` test which is failing with a pattern width mismatch assertion.

## Context

### Failing Test

Test in `crates/context-insert/src/tests/insert.rs`:
- `index_prefix1` (lines 277-327)

### Current Failure

```
assertion `left == right` failed: Pattern width mismatch in index 5 token pattern
  left: TokenWidth(4)
 right: TokenWidth(6)
```

The test is creating a token with width 4, but the pattern being inserted expects width 6.

### Files Affected

- `crates/context-insert/src/tests/insert.rs` - Test implementation
- Potentially token width calculation logic
- Pattern insertion logic

## Analysis

### What This Test Does

The `index_prefix1` test:
- Creates atoms for 'h', 'e', 'l', 'd'
- Inserts patterns: `ld`, `heldld`
- Searches for partial query `[h, e, l, l]`
- Expects to create new token `hel` as a prefix

### The Issue

When inserting `hel`, a pattern is being created with width 6 but the token has width 4. This suggests:
- The token `hel` should have width 3 (h=1, e=1, l=1)
- Something is calculating width as 4 or 6 incorrectly
- Or pattern is including extra tokens

### Possible Causes

1. **Width calculation bug** - Token width not computed correctly
2. **Pattern includes wrong tokens** - Pattern has extra elements
3. **Test expectations wrong** - Test setup creates unexpected structure
4. **Double-counting** - Width calculated multiple times or overlapping

## Execution Steps

### Phase 1: Investigation

- [ ] Read test log: `target/test-logs/index_prefix1.log`
- [ ] Print the actual pattern that's causing the mismatch
- [ ] Trace token width calculation for the failing token
- [ ] Check what token is at "index 5" in the graph
- [ ] Review pattern insertion logic for prefix scenarios

### Phase 2: Diagnosis

- [ ] Identify which token/pattern has the mismatch
- [ ] Determine expected vs actual width
- [ ] Understand why width is 4 vs 6
- [ ] Check if related to the pattern `heldld` = `[h, e, ld, ld]` (width 6)
- [ ] Verify `hel` token creation logic

### Phase 3: Fix

**Option A: Fix width calculation**
- [ ] Correct token width computation
- [ ] Ensure width is sum of child widths
- [ ] Add assertions to catch mismatches earlier

**Option B: Fix pattern structure**
- [ ] Correct pattern insertion to use proper tokens
- [ ] Verify pattern children match expected structure
- [ ] Update test if pattern structure changed intentionally

**Option C: Update test expectations**
- [ ] If behavior is correct, update test assertions
- [ ] Document why width is different than originally expected

### Phase 4: Validation

- [ ] Test passes: `cargo test -p context-insert index_prefix1`
- [ ] No regressions: `cargo test -p context-insert`
- [ ] Width assertions work correctly for all patterns

## Questions for Investigation

- What is the token at index 5?
- What pattern is being inserted that has width mismatch?
- Is the width calculation logic correct?
- Did recent changes affect how patterns calculate width?

## Related Work

- Lock poisoning fix (complete) - this test no longer poisons the lock
- May need to review token width calculation across all tests
