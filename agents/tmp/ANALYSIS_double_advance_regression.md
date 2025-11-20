# Analysis: Double Advance Regression in root_cursor.rs

## Issue

The `range1` test fails with panic: "query_advanced returned Break when can_advance was true"

This panic was **introduced** by commit `c483f69` which claimed to "Fix issue with double advance of query".

## What Changed in c483f69

### Problem the commit tried to fix: Double advancement in `next_parents()`

**Before c483f69**: `next_parents()` would advance the cursor:
```rust
pub(crate) fn next_parents<K: TraversalKind>(
    self,
    trav: &K::Trav,
) -> Result<(ParentCompareState, CompareParentBatch), Box<EndState>> {
    let mut parent = self.state.parent_state();
    let prev_cursor = parent.cursor.clone();
    if parent.cursor.advance(trav).is_continue() {  // ← Advanced cursor here
        if let Some(batch) = K::Policy::next_batch(trav, &parent.parent_state) {
            let batch = CompareParentBatch {
                batch,
                cursor: parent.cursor.clone(),
            };
            Ok((parent, batch))
        } else {
            parent.cursor = prev_cursor;
            Err(Box::new(EndState::mismatch(trav, parent)))
        }
    } else {
        Err(Box::new(EndState::query_end(trav, parent)))
    }
}
```

**After c483f69**: `next_parents()` assumes cursor is already advanced:
```rust
pub(crate) fn next_parents<K: TraversalKind>(
    self,
    trav: &K::Trav,
) -> Result<(ParentCompareState, CompareParentBatch), Box<EndState>> {
    let parent = self.state.parent_state();
    // Note: The cursor should already be advanced by the `advanced()` method
    // before this function is called. We don't advance it again here.
    if let Some(batch) = K::Policy::next_batch(trav, &parent.parent_state) {
        let batch = CompareParentBatch {
            batch,
            cursor: parent.cursor.clone(),
        };
        Ok((parent, batch))
    } else {
        Err(Box::new(EndState::mismatch(trav, parent)))
    }
}
```

### New logic in `advanced()`:

The commit changed the `else` branch (when `can_advance == false`) to always advance the query cursor:

**Before**: Handled case where query could advance:
```rust
} else {
    // Child state cannot advance further in the graph
    tracing::debug!("RootCursor::advanced - child_state cannot advance, attempting to advance query");

    match self.query_advanced() {
        Continue(_) => {
            // Check if past pattern end
            if cursor_end_index >= cursor_pattern_len {
                Break(Some(EndReason::QueryEnd))
            } else {
                Break(None)  // ← Search in parents
            }
        },
        Break(_) => {
            Break(None)  // ← Both exhausted
        },
    }
}
```

**After**: Always advances query, expects it to succeed:
```rust
} else {
    // Child state cannot advance further in the graph
    tracing::debug!("RootCursor::advanced - child_state cannot advance, attempting to advance query");

    match self.query_advanced() {
        Continue(_) => {
            // Query advanced successfully but child_state could not
            // We need to search in parents for the next query token
            
            debug_assert!(
                cursor_end_index < cursor_pattern_len,
                "Query advanced but is past pattern end"
            );

            // Signal to search in parents (next_parents will be called)
            // The cursor is already advanced for the parent search
            Break(None)
        },
        Break(_) => {
            // Query cannot advance - both query and child_state are exhausted
            // This means we've matched the entire query (QueryEnd)
            Break(Some(EndReason::QueryEnd))
        },
    }
}
```

### Most problematic change: Added panic in `if can_advance` branch

```rust
if can_advance {
    // Both query and child_state can advance together
    // Query advance is guaranteed to succeed since can_advance returned true
    let advance_result = self.query_advanced();
    if advance_result.is_break() {
        panic!(
            "query_advanced returned Break when can_advance was true"
        );
    }
    // ...
}
```

## The Bug

The panic assumes: **"If child_state can advance, then cursor must also be able to advance"**

**This assumption is WRONG** because:
1. `can_advance` is called on `rooted_path` (the graph traversal path)
2. But `query_advanced()` advances the `cursor` (the query pattern path)
3. These are **independent paths** that can be in different states!

## Evidence from range1 Test

From logs:
```
child_state can_advance=true, child_state=RootedRangePath { ... end: root_entry: 1 }
query cursor=PathCursor { ... end: root_entry: 2, atom_position: AtomPosition(4) }
PatternRangePath::move_root_index - current_index=2, pattern_len=3
PatternRangePath::move_root_index - reached end of pattern, returning Break
```

- **child_state** can advance: `end.root_entry=1`, more graph to explore
- **cursor** cannot advance: `end.root_entry=2`, pattern length is 3 (at last element)

## Root Cause Analysis

### Before c483f69 (OLD behavior - possibly correct):

1. **When both can advance** (`if can_advance`):
   - Advance cursor with `query_advanced()`
   - Handle both success and failure gracefully
   - If cursor succeeds, also advance child_state

2. **When child cannot advance** (`else`):
   - Try to advance cursor anyway
   - If cursor advances: search in parents (`Break(None)`)
   - If cursor cannot advance: both exhausted, query done

3. **In `next_parents()`**:
   - Advance cursor again (this was the double advance bug)
   - But it had fallback logic if advance failed

### After c483f69 (NEW behavior - introduced bug):

1. **When both can advance** (`if can_advance`):
   - **ASSUMES** cursor will advance successfully
   - **PANICS** if cursor cannot advance
   - This assumption is architecturally wrong!

2. **When child cannot advance** (`else`):
   - Try to advance cursor
   - If cursor advances: search in parents (`Break(None)`)
   - If cursor cannot advance: query done

3. **In `next_parents()`**:
   - Assumes cursor is already advanced
   - No longer advances it (fixes double advance)
   - But breaks when cursor wasn't advanced!

## The Correct Fix

The commit correctly identified that `next_parents()` was double-advancing, but **incorrectly** tried to pre-advance the cursor in all cases in `advanced()`.

### Option 1: Remove the panic, handle cursor end gracefully

```rust
if can_advance {
    // Both query and child_state can advance together
    // Try to advance cursor
    match self.query_advanced() {
        Continue(()) => {
            let cursor_end_index = self.state.cursor.role_root_child_index::<End>();
            let cursor_pattern_len = {
                let graph = self.trav.graph();
                self.state.cursor.path.root_pattern::<G>(&graph).len()
            };

            if cursor_end_index >= cursor_pattern_len {
                // Query pattern is complete
                Break(Some(EndReason::QueryEnd))
            } else {
                // Both advanced successfully, continue
                let _ = self.path_advanced();
                Continue(())
            }
        },
        Break(()) => {
            // Cursor reached end of pattern, but child_state can still advance
            // This means query is complete
            Break(Some(EndReason::QueryEnd))
        }
    }
}
```

### Option 2: Check both paths before assuming

```rust
if can_advance {
    // Child path can advance, check if cursor can too
    let cursor_can_advance = self.state.cursor.can_advance(&self.trav);
    
    if cursor_can_advance {
        // Both can advance
        let _ = self.query_advanced();
        let _ = self.path_advanced();
        Continue(())
    } else {
        // Child can advance but cursor is at end: Query complete
        Break(Some(EndReason::QueryEnd))
    }
}
```

### Option 3: Separate concerns - don't tie child and cursor advancement

The real issue is that the code assumes child advancement and cursor advancement are synchronized. They shouldn't be!

**Child advancement** = moving through the graph structure  
**Cursor advancement** = moving through the query pattern being matched

These are orthogonal concerns. The query ending should be determined by cursor state alone, not by whether the child can advance.

## Recommendation

**Use Option 1** - Remove the panic and handle the cursor reaching the end of the pattern as a normal termination condition (`QueryEnd`), regardless of whether the child path can continue advancing.

The query is complete when the cursor has matched all tokens in the query pattern. Whether the graph has more structure to explore is irrelevant to query completion.

## Related Files

- Bug location: `context-search/src/match/root_cursor.rs:91-110`
- Regression introduced: commit `c483f69` (Nov 11, 2025)
- Previous working version: commit `6c9cedd`
- Test demonstrating bug: `context-search/src/tests/traversal.rs:232` (`range1` test)
