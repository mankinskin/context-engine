---
confidence: 🟢
tags: `#context-insert` `#context-read` `#edge-cases` `#testing` `#validation`
summary: Analysis of edge case failures discovered through context-read testing, with new test coverage for context-insert
---

# Context-Insert Edge Cases Analysis

## Overview

This analysis documents edge case failures discovered through context-read testing that expose bugs in context-read's algorithm and context-insert's input validation. New tests have been added to context-insert to reproduce these failures and guide the implementation of proper error handling.

**Key Finding**: The failures in context-read are caused by context-read producing invalid inputs, not by context-insert failing to handle valid inputs. The fix should be in context-read to avoid generating these edge cases.

## Test Summary

| context-read Test | Status | Root Cause |
|-------------------|--------|------------|
| `validate_single_char` | ✅ PASSING | Empty pattern passed to `read_known()` - FIXED |
| `validate_two_chars` | ✅ PASSING | Empty pattern passed to `read_known()` - FIXED |
| `validate_triple_repeat` | FAILED | Cache missing root token entry |
| `validate_three_repeated` | ✅ PASSING | EntireRoot cursor position fix applied |
| `sync_read_text1` | ✅ PASSING | Empty pattern root access - FIXED |
| `sync_read_text2` | FAILED | Wrong pattern decomposition |
| `read_infix1` | FAILED | Empty pattern root access |
| `read_infix2` | FAILED | Wrong pattern decomposition |
| `read_multiple_overlaps1` | FAILED | Empty pattern root access |
| `read_repeating_known1` | FAILED | Prefix path instead of EntireRoot |
| `read_sequence1` | FAILED | Wrong pattern decomposition |
| `read_loose_sequence1` | FAILED | Wrong pattern decomposition |

## Failure Modes

### Failure Mode 1: Empty Pattern in context-read

**Location:** `context-read/src/context/mod.rs:82` - `read_known(known)` called unconditionally

**Trigger:** `BlockIter::next()` returns `NextBlock { unknown: [...], known: [] }` (empty known pattern)

**Chain of events:**
1. `BlockIter::next()` produces blocks with potentially empty `known` pattern
2. `read_block()` unconditionally calls `read_known(known)` 
3. `read_known(Pattern([]))` creates `PatternEndPath` with empty root
4. `ExpansionCtx::new()` tries to access `cursor.path_root()[0]`
5. **PANIC**: `unwrap()` on `None` because path_root is empty

**Tests affected:** `validate_single_char`, `validate_two_chars`, `sync_read_text1`, `read_infix1`, `read_multiple_overlaps1`

**Status: ✅ FIXED**

Fix applied in `context-read/src/context/mod.rs`:
```rust
fn read_block(&mut self, block: NextBlock) {
    let NextBlock { unknown, known } = block;
    self.append_pattern(unknown);
    if !known.is_empty() {  // <-- Added check
        self.read_known(known);
    }
}
```

Tests now passing: `validate_single_char`, `validate_two_chars`, `sync_read_text1`

### Failure Mode 2: EntireRoot Cursor Position Bug

**Location:** `context-search/src/search/mod.rs` lines 140-162 ("no matches found" case)

**Trigger:** Search finds an `EntireRoot` match but `cursor_position` is 0 instead of token width

**Status: ✅ FIXED** (see `20260205_ENTIRE_ROOT_CURSOR_POSITION_FIX.md`)

**Chain of events:**
1. Search finds a complete token match or no match
2. Creates `EntireRoot` path coverage with `atom_position: AtomPosition::default()` (0)
3. context-insert receives Response with `cursor_position() = 0`
4. `InitInterval::from(Response)` creates interval with `end_bound = 0`
5. Downstream processing fails due to invalid bounds

**Fix applied:**
- `search/mod.rs`: Set `atom_position: AtomPosition::from(token_width)` for EntireRoot
- `state/matched/mod.rs`: Added `MatchResult::new()` with `debug_assert_eq!` validation
- `interval/init.rs`: Now uses `cursor_position()` which returns correct value

Tests now passing: `validate_three_repeated`

### Failure Mode 3: Cache Missing Root Token Entry

**Location:** `context-insert/src/interval/partition/info/range/splits.rs:63`

**Trigger:** `InitInterval` created where cache doesn't contain the root token's vertex

**Debug Output:**
```
[DEBUG] insert_init: root=T2w2 (index=2), end_bound=AtomPosition(2)
[DEBUG] cache entries: [0]  // Cache only has vertex 0, not vertex 2!
```

**Chain of events:**
1. context-read calls `insert_or_get_complete` with some pattern
2. Search traverses graph, caching vertices it visits
3. `Response` is created with `root_token()` from final path
4. BUT: The root token (T2) is different from cached vertices (only 0)
5. `InitInterval::from(Response)` takes cache that doesn't contain root
6. `SplitCacheCtx::init` tries to get splits for root token
7. `completed_splits` returns empty because root not in cache
8. **PANIC**: `get_splits(&(0..1), self)` on empty positions

**Tests affected:** `validate_triple_repeat`

**Required fix:** Either:
1. Ensure search cache always contains the root token, OR
2. Validate in context-insert that cache contains root token entry

### Failure Mode 4: Missing Intermediate Tokens

**Location:** context-read algorithm

**Trigger:** Input "aaa" should produce `{a, aa, aaa}` but context-read only produces `{a, aaa}`

**Tests affected:** Previously `validate_three_repeated`, but now passing after EntireRoot fix

**Required fix:** The context-read expansion algorithm needs to identify and create all repeated substrings, not just the final result.

### Failure Mode 5: Wrong Pattern Decomposition

**Tests affected:** `read_sequence1`, `read_infix1`, `read_infix2`, `sync_read_text2`, `read_loose_sequence1`, `read_multiple_overlaps1`, `read_repeating_known1`

**Issue:** context-read produces different token decompositions than expected. This may be an algorithm issue or test expectation issue.

## New Test Coverage in context-insert

| Test Name | Status | Failure Mode | Purpose |
|-----------|--------|--------------|---------|
| `reject_init_interval_with_zero_end_bound` | ✅ passing | end_bound = 0 | Validates error returned |
| `reject_empty_pattern_search` | ✅ passing | empty pattern | Validates error returned |
| `reject_empty_pattern_insert` | ✅ passing | empty pattern | Validates error returned |
| `integration_partial_match_no_checkpoint` | ✅ passing | integration | No panic on partial match |
| `single_token_mismatch_at_start` | ✅ passing | boundary | Graceful handling |
| `reject_init_interval_with_missing_root_entry` | 🔄 ignored | missing root | Needs fix first |
| `triple_repeat_pattern_scenario` | 🔄 ignored | scenario | Needs fix first |
| `repeated_pattern_intermediate_tokens` | 🔄 ignored | algorithm | Needs fix first |

## Required Fixes Summary

### context-read Fixes (Primary)

1. **Check for empty `known` pattern before calling `read_known()`** ✅ FIXED
   - Location: `context/mod.rs:read_block()`
   - Fix: `if !known.is_empty() { self.read_known(known); }`

2. **Ensure cache contains root token before creating InitInterval**
   - Location: Where `insert_or_get_complete` is called
   - May need to validate Response before conversion

3. **Algorithm fix for intermediate token discovery**
   - The expansion algorithm should identify all repeated substrings
   - Reference: ngrams algorithm produces correct output

### context-search Fixes

1. **EntireRoot cursor_position must equal root token width** ✅ FIXED
   - Location: `search/mod.rs` - "no matches found" case
   - Fix: Set `atom_position: AtomPosition::from(token_width)` instead of default
   - Added validation in `MatchResult::new()` with `debug_assert_eq!`
   - See: `20260205_ENTIRE_ROOT_CURSOR_POSITION_FIX.md`

### context-insert Fixes (Secondary/Defensive)

Already implemented:
- ✅ `end_bound = 0` validation returns `InvalidEndBound` error
- ✅ Empty pattern validation returns `EmptyPatterns` error
- ✅ Uses `cursor_position()` in `InitInterval::from()` (works with EntireRoot fix)

Still needed:
- Validate cache contains root token entry (defensive)

## Conclusions

The root cause of context-read failures is in context-read itself, not context-insert:

1. **Empty patterns**: ✅ FIXED - context-read now checks for empty patterns
2. **EntireRoot cursor position**: ✅ FIXED - context-search now sets correct position
3. **Cache mismatch**: context-read should ensure search cache contains root token
4. **Missing tokens**: context-read algorithm needs refinement to find all repeated substrings

The validation in context-insert (end_bound=0, empty patterns) works correctly. Additional defensive validation for cache-root mismatch would be helpful but the primary fix should be in context-read.

## References

- Test file: `crates/context-insert/src/tests/cases/insert/edge_cases.rs`
- context-read source: `crates/context-read/src/context/mod.rs`
- context-search fix: `crates/context-search/src/search/mod.rs`
- Panic location 1: `crates/context-insert/src/interval/partition/info/range/splits.rs:63`
- Panic location 2: `crates/context-trace/src/path/structs/rooted/pattern_range.rs:175`
- Related docs:
  - `20251204_CONTEXT_INSERT_ARCHITECTURE.md`
  - `20260205_ENTIRE_ROOT_CURSOR_POSITION_FIX.md`
