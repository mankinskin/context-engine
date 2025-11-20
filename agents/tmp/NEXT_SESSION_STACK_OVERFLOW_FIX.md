# Stack Overflow Fix - Next Session Briefing

## Problem Summary

All tests in `context-search` were failing with **stack overflow** (STATUS_STACK_OVERFLOW 0xc00000fd). The stack overflow was NOT a pre-existing issue - it was a new regression.

## Root Cause Identified

**Location:** `context-search/src/cursor/mod.rs` line 54-62

**The Bug:** Infinite recursion in `From<PatternPrefixCursor> for PatternCursor` implementation:

```rust
impl From<PatternPrefixCursor> for PatternCursor {
    fn from(value: PatternPrefixCursor) -> Self {
        let value: PatternCursor = value.into();  // ← CALLS ITSELF INFINITELY!
        Self {
            path: value.path.into(),
            atom_position: value.atom_position,
            _state: PhantomData,
        }
    }
}
```

**The Fix Applied:**

```rust
impl From<PatternPrefixCursor> for PatternCursor {
    fn from(value: PatternPrefixCursor) -> Self {
        Self {
            path: value.path.into(),  // Convert path directly, no recursion
            atom_position: value.atom_position,
            _state: PhantomData,
        }
    }
}
```

## Investigation Process

### How We Found It

1. **Initial symptoms:** All tests crashed immediately with stack overflow
2. **Ran tests with tracing:** `LOG_STDOUT=1 LOG_FILTER=debug cargo test -p context-search find_pattern1 -- --nocapture`
3. **Examined log file:** `target/test-logs/find_pattern1.log`
4. **Key finding:** Last log message was "tokens matched, token: 1, width: 1" before crash
5. **Added detailed tracing:** Added debug statements throughout iteration chain
6. **Pinpointed crash location:** Stack overflow occurred during `cursor.clone().into()` conversion in `into_next_candidate()`
7. **Traced conversion chain:** Found the infinite recursion in the `From` implementation

### Execution Flow Before Crash

```
Test starts → find_ancestor → start_search → MatchIterator
  → RootSearchIterator processes queue
  → CompareIterator::next called
  → next_match() compares tokens
  → Tokens match! → CompareNext::Match returned
  → RootCursor receives Match
  → Calls matched_state.into_next_candidate()
  → Attempts to convert cursor to PatternCursor
  → STACK OVERFLOW in From<PatternPrefixCursor> for PatternCursor
```

### Key Code Locations

- **Test:** `context-search/src/tests/search/mod.rs::find_pattern1`
- **Crash trigger:** `context-search/src/compare/state.rs::into_next_candidate()` line 416
- **Bug location:** `context-search/src/cursor/mod.rs::From<PatternPrefixCursor> for PatternCursor` line 56
- **Iteration chain:** 
  - `context-search/src/match/iterator.rs::MatchIterator::next()`
  - `context-search/src/match/root_cursor.rs::RootCursor::next()`
  - `context-search/src/compare/iterator.rs::CompareIterator::next()`
  - `context-search/src/compare/state.rs::next_match()`

## Current Status

### ✅ Fixed
- Stack overflow eliminated
- All 21 tests now **run to completion** (no crashes)

### ❌ Remaining Issues
- **2 tests pass**
- **19 tests fail** with assertion errors (NOT stack overflows)

### Common Failure Patterns

Looking at test output:
```
assertion failed: !response.is_complete()
assertion failed: `(left == right)`: Query should be fully matched
assertion failed: `(left matches right)`: b_c
assertion failed: `(left == right)`: Query should be fully matched. Got: Mismatch
```

These are **logic errors** in the search/matching implementation, not crashes.

## Debug Tracing Added

**NOTE:** Extensive debug tracing was added during investigation. You may want to remove or reduce it:

### Files with Added Tracing
1. `context-search/src/compare/state.rs`
   - `prefix_states()` - entry/exit, mode tracking
   - `next_match()` - token comparison details
   - `into_next_candidate()` - conversion steps
   - `mode_prefixes()` - mode switching

2. `context-search/src/compare/iterator.rs`
   - `CompareIterator::next()` - queue processing

3. `context-search/src/match/root_cursor.rs`
   - `RootCursor::next()` - match handling

4. Trait implementation in `compare/state.rs`
   - `PrefixStates::prefix_states()` - prefix generation

### Cleanup Recommendation
Consider removing debug tracing or changing to `trace!` level after understanding the logic flow.

## Next Steps for Fixing Test Failures

### 1. Understand Test Expectations
- Review `context-search/src/tests/search/mod.rs::find_pattern1`
- Understand what cache entries should contain
- Check what `EndState::reason` should be

### 2. Analyze Failure Patterns
Run specific failed tests with tracing:
```bash
LOG_STDOUT=1 LOG_FILTER=debug cargo test -p context-search <test_name> -- --nocapture
tail -200 target/test-logs/<test_name>.log
```

### 3. Key Questions to Answer
- **Why does search return incomplete when it should be complete?**
- **Why are cache entries missing?** (Test expects `cache.entries[&xabyz.index]` but key not found)
- **Why are matches not being found?** ("Query should be fully matched" but got "Mismatch")
- **Is the issue in:**
  - Token comparison logic (`next_match()`)?
  - Cursor advancement (`into_next_candidate()`)?
  - Cache population (`TraceCache` not being updated)?
  - State transitions (Candidate → Matched → Candidate)?
  - Prefix generation (`prefix_states()`)?

### 4. Debugging Strategy
1. Pick ONE simple failing test (e.g., `find_pattern1`)
2. Run with full debug tracing
3. Trace execution through log file step-by-step
4. Compare expected vs actual at each step:
   - What tokens are being compared?
   - What prefixes are generated?
   - What cache entries are created?
   - Where does the logic diverge from expected?
5. Identify the specific incorrect behavior
6. Fix that behavior
7. Verify fix doesn't break passing tests
8. Move to next failing test

### 5. Important Context
- **Response API** was recently unified (see CHEAT_SHEET.md)
- **Split-join architecture** for insertions (see context-insert/HIGH_LEVEL_GUIDE.md)
- **Graph tracing** is bidirectional with cache (see context-trace/HIGH_LEVEL_GUIDE.md)
- Check `QUESTIONS_FOR_AUTHOR.md` for known unclear behavior

## Files to Review

### Core Logic
- `context-search/src/compare/state.rs` - State machine, matching logic
- `context-search/src/compare/iterator.rs` - Prefix queue processing
- `context-search/src/match/root_cursor.rs` - Root-level matching
- `context-search/src/cursor/mod.rs` - Cursor types and conversions (FIXED)

### Tests
- `context-search/src/tests/search/mod.rs` - Main search tests
- `context-search/src/tests/search/ancestor.rs` - Ancestor finding tests

### Documentation
- `CHEAT_SHEET.md` - Quick API reference
- `context-search/HIGH_LEVEL_GUIDE.md` - Search concepts and Response API
- `QUESTIONS_FOR_AUTHOR.md` - Known issues and unclear behavior

## Commands Reference

### Run Specific Test with Tracing
```bash
LOG_STDOUT=1 LOG_FILTER=debug cargo test -p context-search find_pattern1 -- --nocapture
```

### Check Log File
```bash
tail -200 target/test-logs/find_pattern1.log
```

### Run All Tests (Summary)
```bash
cargo test -p context-search 2>&1 | grep -E "(test |passed|failed)"
```

### Check Test Status Count
```bash
cargo test -p context-search 2>&1 | grep "test result:"
```

## Critical Insight

The infinite recursion was **obvious in hindsight** once we looked at the actual code:
```rust
let value: PatternCursor = value.into();  // Converts PatternPrefixCursor to PatternCursor
                                           // But we're IN the From impl for that conversion!
```

The line was trying to convert the input to the output type *inside* the function that performs that conversion. Classic infinite recursion pattern.

## Success Criteria for Next Session

- [ ] Understand why 19 tests fail
- [ ] Fix at least 5-10 failing tests
- [ ] Document the logic errors discovered
- [ ] Update CHEAT_SHEET.md with findings
- [ ] Consider updating HIGH_LEVEL_GUIDE.md if architectural issues found
- [ ] Clean up excessive debug tracing (optional)
