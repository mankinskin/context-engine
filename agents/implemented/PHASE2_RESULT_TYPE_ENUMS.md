# Phase 2: Result Type Enum Refactoring

> **Completed**: 2025-11-23  
> **Status**: ✅ Complete  
> **Part of**: Phase 2 Week 3-4 (Days 11-13) - Type Simplification + Method Naming  
> **Issues**: #6 (complex type aliases) and #2 Part B (method naming) from PLAN_CODEBASE_NAMING_AND_DUPLICATION_REFACTOR.md

## Summary

Replaced 3 complex `Result` type aliases and renamed 1 misleading method with descriptive enums and clear names that better express operation semantics. This improves code clarity by making valid outcomes explicit rather than using Ok/Err for non-error cases.

## Changes Made

### 1. QueryAdvanceResult & IndexAdvanceResult Enums (Day 11)

**Replaced type aliases:**
```rust
// Before
pub(crate) type QueryAdvanceResult<EndNode = ...> = Result<
    CompareState<Candidate, Matched, EndNode>,
    CompareState<Matched, Matched, EndNode>,
>;

pub(crate) type IndexAdvanceResult<EndNode = ...> = Result<
    CompareState<Candidate, Candidate, EndNode>,
    CompareState<Candidate, Matched, EndNode>,
>;

// After
pub(crate) enum QueryAdvanceResult<EndNode: PathNode = ...> {
    Advanced(CompareState<Candidate, Matched, EndNode>),
    Exhausted(CompareState<Matched, Matched, EndNode>),
}

pub(crate) enum IndexAdvanceResult<EndNode: PathNode = ...> {
    Advanced(CompareState<Candidate, Candidate, EndNode>),
    Exhausted(CompareState<Candidate, Matched, EndNode>),
}
```

**Files modified:**
- `crates/context-search/src/compare/state.rs` - Enum definitions (2), return sites (2 functions)
- `crates/context-search/src/match/root_cursor.rs` - Call sites (4 match expressions)

**Call sites:** 4 matches updated from `Ok`/`Err` to `Advanced`/`Exhausted`

### 2. AdvanceCursorsResult Enum (Day 12)

**Replaced type alias:**
```rust
// Before
pub(crate) type AdvanceCursorsResult<K> = Result<
    RootCursor<K, Candidate, Candidate>,
    (EndReason, Option<RootCursor<K, Candidate, Matched>>),
>;

// After
pub(crate) enum AdvanceCursorsResult<K: SearchKind> {
    /// Both cursors advanced successfully
    BothAdvanced(RootCursor<K, Candidate, Candidate>),
    /// Query cursor exhausted - complete match found
    QueryExhausted,
    /// Child cursor exhausted - query continues, needs parent exploration
    ChildExhausted(RootCursor<K, Candidate, Matched>),
}
```

**Benefits:**
- Eliminates confusing tuple `(EndReason, Option<RootCursor>)` in Err case
- Makes three distinct outcomes explicit and named
- Removes need for match on tuple destructuring

**Files modified:**
- `crates/context-search/src/match/root_cursor.rs` - Enum definition, return site (1 function), call site (1 match)

**Call sites:** 1 complex nested match simplified to flat 3-variant match

### 3. AdvanceToEndResult Enum (Day 12)

**Replaced Result type:**
```rust
// Before
Result<MatchResult, (MatchResult, RootCursor<K, Candidate, Matched>)>

// After
pub(crate) enum AdvanceToEndResult<K: SearchKind> {
    /// Cursor completed with a match (QueryExhausted or partial match)
    Completed(MatchResult),
    /// Cursor needs parent exploration to continue matching
    NeedsParentExploration {
        checkpoint: MatchResult,
        cursor: RootCursor<K, Candidate, Matched>,
    },
}
```

**Benefits:**
- Named struct variant clearly documents the two pieces of data
- Eliminates tuple destructuring `(checkpoint, cursor)` 
- Makes parent exploration need explicit

**Functions updated:**
- `advance_to_end()` - returns `AdvanceToEndResult`
- `advance_to_matched()` - returns `AdvanceToEndResult`

**Files modified:**
- `crates/context-search/src/match/root_cursor.rs` - Enum definition, 2 function signatures, 3 return sites
- `crates/context-search/src/match/iterator.rs` - Import, 1 call site with named struct destructuring

**Call sites:** 1 complex match simplified, using named fields `checkpoint` and `cursor`

### 4. Method Rename: next_parents → get_parent_batch (Day 12)

**Issue:** Method name `next_parents` implied advancing/iteration, but it actually just fetches a batch of parent states without advancing anything.

**Renamed:**
```rust
// Before
pub(crate) fn next_parents(self, trav: &K::Trav) 
    -> Result<(ParentCompareState, CompareParentBatch), Box<EndState>>

// After  
pub(crate) fn get_parent_batch(self, trav: &K::Trav)
    -> Result<(ParentCompareState, CompareParentBatch), Box<EndState>>
```

**Files modified:**
- `crates/context-search/src/match/root_cursor.rs` - 2 function definitions (both overloads)
- `crates/context-search/src/match/iterator.rs` - 1 call site

**Rationale:** "get" clearly indicates retrieval without side effects, "batch" indicates it returns multiple items at once

## Pattern: When to Use Enums Over Result

Use descriptive enum variants instead of `Result<T, E>` when:

1. **Both outcomes are valid states** (not success vs failure)
   - ✓ Cursor exhausted is a normal outcome, not an error
   - ✓ Need for parent exploration is expected, not exceptional

2. **Err case contains structured data**
   - ✓ Tuples like `(EndReason, Option<Cursor>)` 
   - ✓ Multiple related pieces `(checkpoint, cursor)`

3. **Err case has multiple meanings**
   - ✓ `(QueryExhausted, None)` vs `(ChildExhausted, Some(cursor))`
   - Better: Separate enum variants with descriptive names

4. **Semantics are unclear**
   - ✗ "What does Ok mean here?" → Bad
   - ✓ "BothAdvanced means both cursors moved forward" → Good

## Benefits Achieved

### Code Clarity
**Before:**
```rust
match state.advance_cursors() {
    Ok(cursor) => { /* both advanced */ },
    Err((reason, cursor_opt)) => {
        match (reason, cursor_opt) {
            (EndReason::QueryExhausted, None) => { /* ... */ },
            (EndReason::ChildExhausted, Some(cursor)) => { /* ... */ },
            _ => unreachable!(),
        }
    }
}
```

**After:**
```rust
match state.advance_cursors() {
    AdvanceCursorsResult::BothAdvanced(cursor) => { /* ... */ },
    AdvanceCursorsResult::QueryExhausted => { /* ... */ },
    AdvanceCursorsResult::ChildExhausted(cursor) => { /* ... */ },
}
```

### Named Fields
**Before:**
```rust
Err((checkpoint_state, root_cursor)) => {
    let checkpoint = checkpoint_state; // What's first? What's second?
    let cursor = root_cursor;
}
```

**After:**
```rust
AdvanceToEndResult::NeedsParentExploration { checkpoint, cursor } => {
    // Clear! checkpoint is MatchResult, cursor is RootCursor
}
```

### Better Error Messages
Compiler now shows:
- `AdvanceCursorsResult::BothAdvanced(...)`
- Not: `Result<RootCursor<K, Candidate, Candidate>, (EndReason, Option<...>)>`

### Documentation
Enum variants are self-documenting with doc comments:
```rust
/// Query cursor exhausted (reached end of pattern)
Exhausted(CompareState<Matched, Matched, EndNode>),
```

## Testing

### Results
- ✅ All 29 passing tests still pass
- ✅ Same 6 pre-existing failures (unrelated)
- ✅ context-trace tests unaffected
- ✅ Zero new compilation errors or warnings

### Test Command
```bash
cargo test -p context-search --lib
# Result: 29 passed; 6 failed (same 6 as before)
```

## Files Modified Summary

| File | Changes |
|------|---------|
| `compare/state.rs` | +2 enums, 2 function returns |
| `match/root_cursor.rs` | +2 enums, 6 function signatures, 8 return sites, 4 call sites, 2 renames |
| `match/iterator.rs` | +1 import, 1 call site with named destructuring, 1 rename call |

**Total:**
- 4 new enum types
- 8 function signatures updated
- 11 return sites updated
- 6 call sites updated
- 1 method renamed (2 overloads + 1 call site)

## Migration Patterns

### Simple Result → Enum
```rust
// Before
match result {
    Ok(x) => handle_success(x),
    Err(y) => handle_failure(y),
}

// After
match result {
    EnumType::Success(x) => handle_success(x),
    EnumType::Failure(y) => handle_failure(y),
}
```

### Tuple in Err → Named Struct Variant
```rust
// Before
match result {
    Err((a, b)) => {
        // Which is which?
    }
}

// After
match result {
    EnumType::Variant { field_a: a, field_b: b } => {
        // Crystal clear!
    }
}
```

### Nested Match → Flat Enum Match
```rust
// Before
match outer {
    Err((reason, opt)) => match (reason, opt) {
        (Reason::A, None) => { /* ... */ },
        (Reason::B, Some(x)) => { /* ... */ },
        _ => unreachable!(),
    }
}

// After
match outer {
    EnumType::VariantA => { /* ... */ },
    EnumType::VariantB(x) => { /* ... */ },
}
```

## Lessons Learned

1. **Enums beat Result for non-error cases**: When both outcomes are valid, use descriptive variants
2. **Named struct variants are powerful**: Better than tuples for multi-field variants
3. **Internal APIs can evolve freely**: No breaking changes since all types are `pub(crate)`
4. **Pattern emerged organically**: Found 3 similar patterns, applied same solution
5. **Method names matter**: `next_` implies iteration, `get_` implies retrieval
6. **Small changes, big impact**: ~50 lines of enum definitions improved 15+ call sites

## Related Work

- **Phase 1**: Trait consolidation, CursorStateMachine (~270 lines removed)
- **Phase 2 Day 11**: QueryAdvanceResult, IndexAdvanceResult (this doc, part 1)
- **Phase 2 Day 12**: AdvanceCursorsResult, AdvanceToEndResult, get_parent_batch rename (this doc, parts 2-4)
- **Next**: Trait naming conventions (Issue #5), duplicated implementations (Issue #7)

## Future Opportunities

Other potential candidates for enum treatment:
- `SearchResult = Result<Response, ErrorReason>` - probably fine as-is (true error case)
- Any other `Result<T, (A, B)>` patterns in codebase

Could add helper methods to enums:
```rust
impl<K: SearchKind> AdvanceCursorsResult<K> {
    pub fn is_both_advanced(&self) -> bool {
        matches!(self, Self::BothAdvanced(_))
    }
    
    pub fn into_cursor(self) -> Option<RootCursor<K, Candidate, Candidate>> {
        match self {
            Self::BothAdvanced(cursor) => Some(cursor),
            _ => None,
        }
    }
}
```

## Conclusion

Successfully replaced 3 complex Result types and renamed 1 misleading method, improving code clarity with zero breaking changes or test regressions. This pattern of using descriptive enums for non-error outcomes significantly improves readability and maintainability.

**Time spent**: ~3-4 hours (Days 11-12)  
**Enums created**: 4 (QueryAdvanceResult, IndexAdvanceResult, AdvanceCursorsResult, AdvanceToEndResult)
**Methods renamed**: 1 (next_parents → get_parent_batch)  
**Lines changed**: ~90 (enum definitions + call sites + rename)  
**Tests passing**: 29/35 (maintained, 0 regressions)
