# Analysis: Why Cache Entries Are Missing

> **NOTE:** This is historical analysis from before the `CompareState<Q, I>` refactoring.
> The bug described here was fixed by introducing separate query and index cursor states.
> Type names in this document reflect the old implementation (e.g., MatchIterator, TraceNode).
> See SEARCH_FLOW_COMPLETE.md for current implementation with updated names.

## Problem Summary
The test `find_pattern1` expects cache entries for tokens `xabyz` (9), `xab` (6), and `yz` (7), but only gets entries for tokens 6 and 0.

## Query Pattern
- Query: `[a, b, y, x]` (tokens 0, 1, 3, 2)
- First parent found: `xab` (token 6) at position 1

## What Happened

### 1. Search Initialization
- Query pattern cursor starts at token 0 (a), atom_position=1
- Gets parent batch: finds `xab` (token 6) containing `a` at sub_index=1
- SearchIterator starts with this parent

### 2. First Match Attempt (Only One!)
```
Comparing: path_leaf=1 (b) vs query_leaf=3 (y)
Result: MISMATCH (atoms differ)
```

**Key observation:** The comparison happened at:
- Graph path position 2 (trying to match 'b' from xab)
- Query cursor position 2 (trying to match 'y' from query)
- These don't match → immediate Mismatch

### 3. What Should Have Happened
According to the test expectations and the algorithm flow document:

1. **Should match 'a' → 'b' path**:
   - Start at 'a' (position 0), find parent `xab`
   - Match continues: 'a' matches, advance to 'b'
   - Now at position 1 in graph (b), position 1 in query (b)
   - **This should MATCH**
   
2. **Then explore parents of xab**:
   - When 'a'→'b' completes, should explore `xabyz` (parent of xab)
   - This would create cache entry for `xabyz`
   
3. **Continue matching in xabyz**:
   - Match 'y' from query at position 2
   - This would create cache entry for `yz`

## The Bug

The matching stopped after only ONE comparison that resulted in mismatch. This suggests:

**The RootCursor is comparing at the wrong positions!**

Looking at the comparison state:
```
cursor: atom_position=3 (should be at 'x' in query)
checkpoint: atom_position=2 (last confirmed match at 'b')
path: root_entry=2 (pointing to token index 2 in xab parent pattern)
```

Wait - the path is `xab` (token 6), and we're at root_entry=2?
- Token 6 (xab) has 3 sub-tokens: positions 0, 1, 2
- root_entry=2 would be the 3rd position - that's past 'b', trying to access non-existent position!

Actually looking more carefully:
- Graph path root: PatternLocation { parent: Token 6 (xab), pattern_id, sub_path at root_entry=2 }
- root_entry=2 in a 3-token pattern (xab) is accessing position 2 (the 'b')
- Query path: [a, b, y, x] at root_entry=2 is the 'y'

So it's comparing:
- path_leaf = Token 1 (which is 'b' - but why token 1?)
- query_leaf = Token 3 (which is 'y')

**The issue:** After finding the first parent (`xab`), the algorithm is starting the comparison at position 2 instead of position 1!

## Root Cause Hypothesis

Looking at the parent batch creation:
```
cursor: PathCursor {
    path: [a, b, y, x],
    end: RolePath { root_entry: 1 },  // Points to position 1
    atom_position: 2
}
```

The cursor was already advanced to position 1 (atom_position=2 means 2 atoms consumed, so at token index 1).

But the ChildState for comparison starts at:
```
root_pos: AtomPosition(1),
path: root_entry: 1  // Also at position 1
```

Then when creating CompareState, it's comparing at root_entry=2!

**The advancement is happening in the wrong place or at the wrong time.**

## Why No Cache Entries?

1. **Only one match iteration occurs** - it immediately finds a mismatch
2. **No parent exploration** - because there's no complete match to trigger parent search
3. **Only traces the initial path** - from 'a' to 'xab' (which creates entries for tokens 0 and 6)
4. **Missing tokens**:
   - Token 9 (xabyz): Never explored because xab match failed
   - Token 7 (yz): Never explored because parent exploration didn't happen

## Root Cause Identified!

### The Bug is in ParentCompareState::into_advanced

When converting `ParentCompareState` to `CompareState`, the code does:

1. **Graph side advancement** (correct):
   - `ParentState::into_advanced()` calls `path.into_range(next_i)`
   - This creates a range from current_index to next_index
   - For parent at sub_index=1 ('a' in xab), this creates range [1, 2], pointing at position 2 ('b')

2. **Query side** (BUG):
   - The cursor (PatternCursor) is passed through with the SAME atom_position
   - But the graph has already advanced by 1 position!
   - This creates a mismatch in synchronization

### Example from Test

**Initial state:**
- Query: [a, b, y, x] at position 0 (a)
- Parent found: xab at sub_index=1 (pointing to 'a')
- Cursor after getting parent: atom_position=2 (already consumed 'a' and pointing to 'b')

**After into_advanced:**
- Graph path: xab range [1, 2] → points to position 2 which is 'b'
- Query cursor: still at atom_position=2 but now converted to prefix path
  - Wait, the conversion to rooted_role_path creates an End path at position 2
  - Position 2 in query [a, b, y, x] is 'y' (index 3)!

**So the comparison becomes:**
- Graph: position 2 in xab = 'b' (token 1)
- Query: position 2 in [a,b,y,x] = 'y' (token 3)
- Result: MISMATCH (correct, but we're comparing the wrong positions!)

### The Real Problem

The query cursor should still be at position 1 (pointing to 'b'), not position 2 (pointing to 'y').

When we got the parent batch, the cursor was advanced to prepare for the next comparison, but `into_advanced` doesn't account for the fact that:
1. The graph position is being advanced by 1 (from entry to next_i)
2. The query cursor was ALREADY advanced when the parent batch was created

So there's a **double advancement** of the query cursor but only **single advancement** of the graph position.

### FINAL ROOT CAUSE IDENTIFIED

Looking at the actual comparison logs, there are **TWO** comparisons:

**First Comparison** (MATCH):
```
path_leaf: Token 1 (b), query_leaf: Token 1 (b)
cursor_pos: 2, checkpoint_pos: 2
Result: MATCH ✓
```

**Second Comparison** (MISMATCH):
```
path_leaf: Token 1 (b), query_leaf: Token 3 (y)  
cursor_pos: 3, checkpoint_pos: 2
Result: MISMATCH ❌
```

### The Bug

After the first match:
1. Query cursor advances: cursor_pos goes from 2 → 3 ✓
2. Graph path DOES NOT advance: path_leaf stays at Token 1 (b) ❌

**The graph path position is not advancing after a match!**

The `ChildState` (which contains the graph path) is not being updated when `into_next_candidate` is called. Only the query cursor is advancing.

### Where the Bug Is

In `CompareState<Matched>::into_next_candidate()`:
- The method advances the query cursor: `self.cursor.advance(trav)`
- But the `child_state` (which contains the graph path position) is passed through unchanged!

```rust
fn into_next_candidate(&self, trav: &G) -> Result<CompareState<Candidate>, CompareState<Matched>> {
    // ...
    match self.cursor.advance(trav) {
        Continue(_) => {
            Ok(CompareState {
                child_state: self.child_state,  // ❌ NOT ADVANCED!
                cursor: candidate_cursor,        // ✓ Advanced
                checkpoint: new_checkpoint,
                target: self.target,
                mode: self.mode,
            })
        },
        // ...
    }
}
```

The `child_state.base.path` needs to advance to the next position in the graph after a successful match, but it's not happening.

### Solution

`CompareState<Matched>::into_next_candidate()` needs to advance the `child_state` as well as the cursor. This likely means calling `child_state.into_advanced(trav)` to move to the next position in the graph path.
