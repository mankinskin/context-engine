---
confidence: 🟢
tags: `#context-search` `#EntireRoot` `#cursor-position` `#bug-fix` `#validation`
summary: Fixed EntireRoot cursor_position to equal root token width, added validation and tests
---

# EntireRoot Cursor Position Invariant Fix

## Summary

Fixed a bug where `EntireRoot` matches had `cursor_position` set to 0 instead of the root token's width. This caused incorrect behavior in context-insert when processing full token matches. Added validation via `MatchResult::new()` and tests to ensure the invariant holds.

## Problem

When searching for a pattern that doesn't exist in the graph (or finds a complete token match), the search creates an `EntireRoot` response. The "no matches found" case in `search/mod.rs` was incorrectly setting `atom_position: AtomPosition::default()` (0) instead of the root token's width.

This violated the invariant that for `EntireRoot` matches, the cursor position should equal the matched token's width, since the entire token was "traversed".

## Root Cause

In `crates/context-search/src/search/mod.rs` lines 140-162, the "no matches found" case created:
```rust
let raw_cursor = PatternCursor {
    atom_position: AtomPosition::default(),  // ❌ Wrong: was 0
    // ...
};
```

## Changes

| File | Change |
|------|--------|
| `crates/context-search/src/search/mod.rs` | Fixed "no matches found" case to use `AtomPosition::from(token_width)` instead of `AtomPosition::default()` |
| `crates/context-search/src/search/mod.rs` | Updated both `MatchResult` creation sites to use `MatchResult::new()` |
| `crates/context-search/src/state/matched/mod.rs` | Added `MatchResult::new()` constructor with invariant validation |
| `crates/context-search/src/state/matched/mod.rs` | Added `validate_entire_root_invariant()` with `debug_assert_eq!` |
| `crates/context-search/src/tests/search/mod.rs` | Added 2 new tests for the invariant |
| `crates/context-insert/src/interval/init.rs` | Changed to use `cursor_position()` instead of `matched_end_position()` |

## API

```rust
impl MatchResult {
    /// Create a new MatchResult, validating invariants
    ///
    /// For EntireRoot paths, validates that cursor_position equals root token width.
    pub fn new(path: PathCoverage, cursor: CheckpointedCursor) -> Self {
        let result = Self { path, cursor };
        result.validate_entire_root_invariant();
        result
    }

    /// Validate that EntireRoot cursor position equals root token width
    #[inline]
    fn validate_entire_root_invariant(&self) {
        if let PathCoverage::EntireRoot(_) = &self.path {
            let cursor_pos = *self.cursor.cursor().atom_position.as_ref();
            let root_width: usize = (*self.path.root_parent().width()).into();
            debug_assert_eq!(
                cursor_pos,
                root_width,
                "EntireRoot cursor position ({}) must equal root token width ({})",
                cursor_pos,
                root_width
            );
        }
    }
}
```

## Migration

- `matched_end_position()` is now deprecated - use `cursor_position()` instead
- Code creating `MatchResult` directly should use `MatchResult::new()` for validation

## Testing

- `test_entire_root_cursor_position_equals_token_width` - validates EntireRoot matches have correct cursor position
- `test_no_match_entire_root_cursor_position` - validates the "no matches found" case
- All 58 context-search tests pass
- All 16 context-insert tests pass (2 ignored for unrelated issues)

## Related Issues

This fix addresses part of the "roots not being found in trace cache" failures in context-read tests. The fix ensures that when context-insert receives an `EntireRoot` response, the `cursor_position()` correctly reflects the matched extent.
