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

The search algorithm appears to terminate early when it finds a complete match within a shorter parent pattern (`abab`) rather than continuing to find the exact pattern (`ababab`).

Likely locations:
1. `context-search/src/search/context/mod.rs` - AncestorSearchTraversal
2. `context-search/src/search/mod.rs` - Main search loop
3. State machine transition logic for EntireRoot matches

The algorithm should:
1. Continue searching when there's more query to consume
2. Prefer exact-length matches over embedded matches

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
