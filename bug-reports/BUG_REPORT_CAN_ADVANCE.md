# Bug Report: can_advance/advance Inconsistency in context-search

## Summary

The `range1` test in `context-search` fails with the error:
```
query_advanced returned Break when can_advance was true
```

## Root Cause

The bug is in `context-search/src/match/root_cursor.rs:91-108`. The code checks if one path can advance, but then tries to advance a *different* path:

```rust
let rooted_path = self.state.rooted_path();
let can_advance = rooted_path.can_advance(&self.trav);  // ← Checks CHILD path

if can_advance {
    // Query advance is guaranteed to succeed since can_advance returned true
    let advance_result = self.query_advanced();  // ← Advances CURSOR path
    if advance_result.is_break() {
        panic!("query_advanced returned Break when can_advance was true");
    }
}
```

Where:
- `rooted_path` = `self.state.child_state.base.path` - the path through the graph being explored
- `cursor` = `self.state.cursor` - the query pattern being matched

These are **independent paths** tracking different things:
1. The `rooted_path` (child_state) tracks the current position while traversing the graph structure
2. The `cursor` tracks the current position in the query pattern being matched against the graph

## Evidence from range1 Test

From the preserved log file `target/test-logs/range1.log`:

```
child_state can_advance=true, child_state=RootedRangePath { ... end: RolePath { sub_path: SubPath { root_entry: 1, ... } } }
query cursor=PathCursor { ... end: RolePath { sub_path: SubPath { root_entry: 2, ... } }, atom_position: AtomPosition(4) }
PatternRangePath::move_root_index - current_index=2, pattern_len=3
PatternRangePath::move_root_index - reached end of pattern, returning Break
```

- **child_state can advance** because `end.root_entry=1` with more graph to explore
- **cursor cannot advance** because `end.root_entry=2` and `pattern_len=3` (already at the last element)

## The Contract Violation

The `can_advance` trait method has an implicit contract (as implemented in `context-trace/src/path/mutators/move_path/advance.rs:11-16`):

```rust
pub trait CanAdvance: Advance + Clone {
    fn can_advance<G: HasGraph>(&self, trav: &G) -> bool {
        self.clone().move_path(trav).is_continue()
    }
}
```

This checks if *the same object* can advance by cloning and trying. The violation is checking `can_advance` on one object but calling `advance` on a different object.

## Test Case

Created `context-trace/src/tests/path_advance.rs` with:
- `test_pattern_cursor_at_end_cannot_advance` - verifies that for a single path, can_advance accurately predicts advance success
- `test_can_advance_advance_consistency` - iteratively tests the invariant that `can_advance() == true` implies `advance()` will succeed

Both tests pass, confirming that the implementation of `can_advance`/`advance` is correct for a single path. The bug is architectural - checking the wrong path.

## Fix Options

### Option 1: Check the cursor's can_advance (Recommended)

```rust
let cursor_can_advance = self.state.cursor.can_advance(&self.trav);
let rooted_path = self.state.rooted_path();
let path_can_advance = rooted_path.can_advance(&self.trav);

if cursor_can_advance && path_can_advance {
    // Both can advance
    let advance_result = self.query_advanced();
    assert!(advance_result.is_continue());
    // ... advance both ...
}
```

### Option 2: Don't check can_advance (Simpler)

```rust
let rooted_path = self.state.rooted_path();
let path_can_advance = rooted_path.can_advance(&self.trav);

if path_can_advance {
    // Try to advance both, handle cursor reaching end gracefully
    match self.query_advanced() {
        Continue(()) => {
            // Both advanced successfully
            // ...
        },
        Break(()) => {
            // Query pattern is exhausted, this is actually QueryEnd not a bug
            return Break(Some(EndReason::QueryEnd));
        }
    }
}
```

### Option 3: Remove the panic

The panic might be incorrect - reaching the end of the query pattern while the graph path can still advance is actually a valid end condition (QueryEnd), not a bug.

## Recommendation

Use **Option 2** - Remove the assertion and handle the cursor reaching its end as a normal termination condition. When the query pattern is fully matched, that's `EndReason::QueryEnd`, regardless of whether the graph traversal could continue.

The current code incorrectly assumes that if the graph path can advance, the query cursor must also be able to advance. This is wrong - they are independent and the query ending first is a valid (and expected) outcome.

## Related Files

- Bug location: `context-search/src/match/root_cursor.rs:91-108`
- Test demonstrating bug: `context-search/src/tests/traversal.rs:232` (`range1` test)
- Test cases for can_advance invariant: `context-trace/src/tests/path_advance.rs`
- Tracing implementation: `context-trace/src/path/mutators/move_path/advance.rs`
