---
confidence: 🟢
tags: `#context-search` `#bug` `#repeat-patterns` `#testing`
summary: Critical bug: search for [ab,ab,ab] returns abab (4 atoms) instead of ababab (6 atoms)
---

# Search Algorithm Returns Partial Match for Repeated Patterns

## Overview

Critical bug discovered in context-search: when a graph contains patterns that share repeated sub-patterns (e.g., `ab`, `abab`, `ababab`), searching for the longer pattern incorrectly returns the shorter one.

This bug was discovered through test isolation following proper test environment patterns in context-search.

## Bug Description

### Graph Structure
```
atoms: a, b
patterns:
  - ab = [a, b]       (width 2)
  - abab = [ab, ab]   (width 4)
  - ababab = [ab, ab, ab]  (width 6)
```

### Expected Behavior
- Search for `[ab, ab, ab]` → returns `ababab` (width 6) as EntireRoot

### Actual Behavior
- Search for `[ab, ab, ab]` → returns `abab` (width 4) as EntireRoot

## Test Evidence

### Test File Location
`crates/context-search/src/tests/cases/ababab.rs`

### Test Environment
`crates/context-search/src/tests/env/ababab.rs` - `EnvAbabab`

### Test Output
```
assertion failed: `(left == right)`: Should find ababab (width 6), not abab (width 4)
Diff < left / right > :
<4
>6
```

### Working Tests
- `test_search_ab_from_atoms` ✅ - Search for [a, b] finds ab (width 2) correctly
- `test_search_abab_exact` ❌ - Has cache differences but correct root match
- `test_search_ababab_exact` ❌ - **Critical failure**: returns abab instead of ababab

## Root Cause Analysis

The search algorithm has a **flawed assumption** about parent exploration.

### Bug Location

**File:** `crates/context-search/src/match/iterator.rs`  
**Lines:** 102-106

```rust
// Clear the queue - all better matches are explored via this root cursor and its parent exploration
debug!("Found matching root - clearing search queue (will explore via parents)");
self.queue.nodes.clear();
```

### The Flawed Assumption

The comment claims: "all better matches are explored via this root cursor and its parent exploration"

This is **incorrect** when candidates are siblings rather than ancestors.

### Graph Structure

```
      ab (width 2)
     /  \
  abab   ababab
 (w:4)   (w:6)
```

Both `abab=[ab,ab]` and `ababab=[ab,ab,ab]` are **parents of `ab`**, but neither is an ancestor of the other.

### Execution Flow (from logs)

1. **Initial queue** contains both `abab` and `ababab` as parent candidates:
   ```
   [0] ParentCandidate(vertex:"abab"(3), pos:4),
   [1] ParentCandidate(vertex:"abab"(3), pos:4),
   [2] ParentCandidate(vertex:"ababab"(4), pos:4),
   [3] ParentCandidate(vertex:"ababab"(4), pos:4),
   [4] ParentCandidate(vertex:"ababab"(4), pos:4),
   ```

2. **First match found** - `abab` is processed first (queue order):
   ```
   Found matching root - creating RootCursor
   root_parent="abab"(3), root_width=4
   ```

3. **Queue cleared** - `ababab` candidates are discarded!

4. **Query advanced** - cursor moves from position 4 to 6 (consuming third `ab`)

5. **Child exhausted** - `abab` only has 2 children, we need a third

6. **Parent exploration fails** - `abab` has no parents, so:
   ```
   No parents available - search exhausted
   ```

7. **Result** - `abab` returned as best match (width 4), not `ababab` (width 6)

### Why the Assumption Fails

The algorithm assumes: "If we find `abab`, we can find `ababab` by exploring `abab`'s parents."

Reality: `ababab` is NOT a parent of `abab`. They are siblings (both parents of `ab`).

To find `ababab`, we must NOT clear the queue - we must continue processing all candidates.

### Fix Required

In `SearchIterator::next()` (iterator.rs:97-117):

**Option A**: Don't clear the queue. Let the iterator yield all matches and pick the best one.

**Option B**: Sort candidates by width descending and process longest first (may need priority queue).

**Option C**: Filter candidates - only clear those that are ancestors of the found match.

The cleanest fix is likely **Option A** - continue processing all queue items and track the best match by query exhaustion status or match length.

## Related Tests

### Also Affected: xyyxy scenario
- `test_search_y_alone` - Search for single atom 'y' fails completely
- `test_search_xy_exact` - Works but has cache differences

### Test Files
- `crates/context-search/src/tests/cases/ababab.rs`
- `crates/context-search/src/tests/cases/xyyxy.rs`
- `crates/context-search/src/tests/env/ababab.rs`
- `crates/context-search/src/tests/env/xyyxy.rs`

## Impact

This bug affects:
1. context-read: `validate_triple_repeat` test fails due to this search bug
2. context-insert: Any insert operation involving repeated patterns may get wrong results
3. Graph consistency: Incorrect token identification leads to wrong graph structure

## Fix Priority

**HIGH** - This is a fundamental search correctness issue that cascades through the entire system.

## Conclusions

The context-search algorithm has a bug where it returns partial matches embedded in shorter parent patterns instead of finding the exact pattern being searched for. This needs to be fixed in the search state machine to continue searching when the query is not yet exhausted.

## References

- Bug discovery: Session refactoring tests to use proper test environments
- Previous doc: `20260205_CONTEXT_INSERT_EDGE_CASES.md` - references `scenario_triple_repeat_ababab`
- Test pattern: Based on `EnvInsertPrefix1` and `EnvInsertPostfix1` environment patterns
