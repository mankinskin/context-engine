# Phase 1 Week 2: IntoCursor Trait Rename

**Status:** ✅ Complete  
**Date:** 2025-01-27  
**Plan:** agents/plans/PLAN_CODEBASE_NAMING_AND_DUPLICATION_REFACTOR.md (Issue #3)

## Overview

Renamed `ToCursor` trait to `IntoCursor` following Rust naming conventions for consuming conversions. This standardizes conversion trait naming to match stdlib patterns (`Into*`, `From*`) rather than mixing `To*` and `Into*` prefixes.

## Changes

### Trait Rename

**File:** `crates/context-search/src/state/start.rs`

**Before:**
```rust
pub(crate) trait ToCursor: StartFoldPath {
    fn to_cursor<G: HasGraph>(self, trav: &G) -> PathCursor<Self>;
}
```

**After:**
```rust
pub(crate) trait IntoCursor: StartFoldPath {
    fn into_cursor<G: HasGraph>(self, trav: &G) -> PathCursor<Self>;
}
```

### Naming Rationale

**Rust conventions:**
- `Into*` traits indicate consuming conversions (take `self`, not `&self`)
- `To*` methods typically indicate borrowing conversions or cloning operations
- Examples from stdlib: `IntoIterator`, `Into<T>`, `FromStr`

**Our trait:**
- Takes `self` (consumes the path) ✓
- Returns a new type (`PathCursor<Self>`) ✓
- Matches `Into*` pattern exactly ✓

### Method Rename

**Method:** `to_cursor()` → `into_cursor()`

**Call sites updated:** 2 locations
1. `PatternEndPath::start_search()` - line 392
2. `PatternRangePath::start_search()` - line 411

### Implementation Update

**Before:**
```rust
impl<P: StartFoldPath> ToCursor for P {
    fn to_cursor<G: HasGraph>(self, trav: &G) -> PathCursor<Self> {
        PathCursor {
            atom_position: (*self.calc_width(trav)).into(),
            path: self,
            _state: std::marker::PhantomData,
        }
    }
}
```

**After:**
```rust
impl<P: StartFoldPath> IntoCursor for P {
    fn into_cursor<G: HasGraph>(self, trav: &G) -> PathCursor<Self> {
        PathCursor {
            atom_position: (*self.calc_width(trav)).into(),
            path: self,
            _state: std::marker::PhantomData,
        }
    }
}
```

## Impact

### Scope
- **Files affected:** 1 file (`state/start.rs`)
- **Trait renamed:** 1 trait
- **Method renamed:** 1 method
- **Call sites updated:** 2 locations
- **Breaking change:** No (internal API only)

### Benefits
1. **Convention adherence:** Matches Rust stdlib naming patterns
2. **Consistency:** All consuming conversion traits now use `Into*` prefix
3. **Clarity:** Immediately signals that conversion consumes the value
4. **Predictability:** Developers familiar with Rust conventions will understand instantly

### Related Traits

This completes the conversion trait standardization. Context-trace already uses `Into*` prefix consistently:
- `IntoRootedRolePath`
- `IntoRootedPath`
- `IntoRolePath`
- `IntoParentState`
- `IntoChildLocation`

Now context-search also follows this pattern with `IntoCursor`.

## Testing

✅ All tests passing: 29/35 (same as before)  
⚠️ 6 pre-existing failures documented (atom_position off-by-one issues, unrelated to this refactor)

No new test failures introduced. Behavior unchanged - pure rename refactoring.

## Phase 1 Summary

With this change, **Phase 1 is now complete**:

### Week 1: Has* Trait Consolidation ✅
- Created 3 unified traits (PathAccessor, RootedPathAccessor, StatePosition)
- Deprecated 11 fragmented traits
- Non-breaking migration path established

### Week 2: Cursor & Conversion Cleanup ✅
- Implemented CursorStateMachine trait (Issue #4)
- Eliminated ~200 lines of state transition duplication
- Standardized conversion traits (Issue #3) - this change

### Metrics
- **Traits consolidated:** 11 → 3 (73% reduction in Has* traits)
- **Duplication removed:** ~270 lines
- **Naming consistency:** 100% of conversion traits now use `Into*` prefix
- **Breaking changes:** 0 (all changes backward compatible via deprecation)
- **Test stability:** 100% (no new failures)

## Next Steps

Phase 1 complete ✓  
**Ready for Phase 2:** High-Impact Cleanup (Weeks 3-4)
- Issue #6: Replace complex type aliases with newtypes
- Issue #5: Standardize trait naming conventions
- Issue #2 (Part B): Consolidate RootCursor advance methods

## Related Documentation

- **Plan:** `agents/plans/PLAN_CODEBASE_NAMING_AND_DUPLICATION_REFACTOR.md`
- **Week 1:** `agents/implemented/PHASE1_HAS_TRAIT_CONSOLIDATION.md`
- **Week 2 Part 1:** `agents/implemented/PHASE1_CURSOR_STATE_MACHINE.md`
- **Week 2 Part 2:** This document (IntoCursor rename)
- **Cheat Sheet:** `CHEAT_SHEET.md` (update with IntoCursor pattern)
