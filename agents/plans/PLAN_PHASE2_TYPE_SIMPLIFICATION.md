# Phase 2: Type Simplification Plan

> **Created**: 2025-11-23  
> **Status**: In Progress  
> **Phase**: Week 3 (Days 11-15) of main refactoring plan  
> **Goal**: Simplify type system and reduce complex type aliases

## Context

Phase 1 successfully consolidated traits and removed ~270 lines of duplication. Phase 2 focuses on simplifying the type system, particularly around CompareState and Result types.

## Pre-Phase 2 State

- ✅ 6 pre-existing test failures in context-search (known issues, unrelated to refactoring)
- ✅ Phase 1 complete: traits consolidated, CursorStateMachine implemented
- ✅ All new code backward compatible via deprecation

## Objectives (Week 3: Days 11-15)

### Day 11-13: Replace Complex Type Aliases (Issue #6)

**Current problematic aliases:**

```rust
// context-search/src/compare/state.rs
pub(crate) type MatchedCompareState =
    CompareState<Matched, Matched, PositionAnnotated<ChildLocation>>;

pub(crate) type QueryAdvanceResult<EndNode = PositionAnnotated<ChildLocation>> =
    Result<
        CompareState<Candidate, Matched, EndNode>,
        CompareState<Matched, Matched, EndNode>,
    >;

pub(crate) type IndexAdvanceResult<EndNode = PositionAnnotated<ChildLocation>> =
    Result<
        CompareState<Candidate, Candidate, EndNode>,
        CompareState<Candidate, Matched, EndNode>,
    >;
```

**Problems:**
1. `QueryAdvanceResult` and `IndexAdvanceResult` hide Ok/Err semantics in generic Result
2. Default generic parameter `EndNode = PositionAnnotated<ChildLocation>` is unclear
3. Type aliases don't allow adding methods or documentation
4. Error messages show full generic expansion instead of meaningful name

**Proposed solution:**

Create enum wrappers with descriptive variant names:

```rust
/// Result of advancing the query cursor
pub(crate) enum QueryAdvanceResult<EndNode = PositionAnnotated<ChildLocation>> {
    /// Query cursor advanced to next token
    Advanced(CompareState<Candidate, Matched, EndNode>),
    /// Query cursor exhausted (reached end of pattern)
    Exhausted(CompareState<Matched, Matched, EndNode>),
}

/// Result of advancing the index (child) cursor  
pub(crate) enum IndexAdvanceResult<EndNode = PositionAnnotated<ChildLocation>> {
    /// Index cursor advanced to next position
    Advanced(CompareState<Candidate, Candidate, EndNode>),
    /// Index cursor exhausted (no more positions)
    Exhausted(CompareState<Candidate, Matched, EndNode>),
}
```

**Note:** Keeping `MatchedCompareState` as type alias for now since it's just a convenience shorthand used in many places. Can revisit if it causes issues.

### Day 14-15: Update Usage Sites

**Estimated call sites:**
- `QueryAdvanceResult`: ~8-10 uses
- `IndexAdvanceResult`: ~5-7 uses

**Migration approach:**
1. Add enums alongside Result type aliases (no breaking change)
2. Update internal functions to return enums
3. Update call sites to use match on enum variants instead of Ok/Err
4. Deprecate type aliases
5. Later: remove type aliases

## Alternative Considered: Keep as Result

**Pros of Result:**
- Familiar error handling semantics
- Works with `?` operator
- Standard library type

**Cons of Result:**
- "Advanced" is not really Ok and "Exhausted" is not really Err
- Semantically these are both valid outcomes, not success/failure
- Less clear API (what does Ok mean vs Err?)

**Decision:** Use enums for better semantics

## Non-Goals for Phase 2

- ❌ Not replacing `PositionAnnotated<ChildLocation>` - already well-designed generic struct
- ❌ Not replacing `MatchedCompareState` - simple convenience alias
- ❌ Not touching context-trace path types yet (PatternRangePath, etc.) - save for later

## Success Metrics

- [ ] QueryAdvanceResult enum implemented
- [ ] IndexAdvanceResult enum implemented  
- [ ] All call sites updated to use enums
- [ ] Old type aliases deprecated
- [ ] All existing tests still pass (29/35 passing maintained)
- [ ] Documentation updated
- [ ] Zero new test failures

## Files to Modify

1. `crates/context-search/src/compare/state.rs` - Define enums
2. Update callers (to be identified during implementation):
   - Methods returning these types
   - Match expressions using Ok/Err
   - Any `?` operators that need changing

## Testing Strategy

- Run full test suite after each change
- Ensure 29/35 tests still passing (same 6 known failures)
- Check that enum variants make code more readable

## Risks

- Low risk: Changes are internal to context-search
- May need to adjust match patterns at call sites
- Enum doesn't work with `?` operator (need explicit match)

## Timeline

- **Day 11** (Today): Create enums, update ~5 call sites
- **Day 12**: Update remaining call sites, run tests
- **Day 13**: Buffer day for issues, documentation
- **Day 14-15**: Reserved for next task or overflow

## Next Steps After This

- Issue #5: Standardize trait naming (Week 4, Days 16-17)
- Issue #2 Part B: Consolidate RootCursor methods (Week 4, Days 18-19)
- Issue #7: Remove duplicated implementations (Week 4, Day 20)
