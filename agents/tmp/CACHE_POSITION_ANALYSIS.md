# Cache Position Analysis for range1 Test

## Test Setup
- Query: `[bc, d, e]` with widths `[2, 1, 1]`
- Expected final atom_position: 4
- Expected cache entry for token 'e' (index 4): AtomPosition(3)
- Actual cache entry: AtomPosition(7)

## Expected Matching Flow

### Iteration 1: Match "bc"
- Start: cursor.atom_position = 0, cursor.path.end.root_entry = 0
- Advance cursor: path.end.root_entry = 1, atom_position = ?
- **matched_cursor should be set to 0** (where we started matching "bc")
- Compare and match "bc"
- Cache entries for "bc" matching should use position 0

### Iteration 2: Match "d"
- Start: cursor from prev iteration (after matching "bc")
- Expected cursor.atom_position = 2 (after consuming "bc" width=2)
- Advance cursor: path.end.root_entry = 2
- **matched_cursor should be set to 2** (where we started matching "d")
- Compare and match "d"
- Cache entries for "d" matching should use position 2

### Iteration 3: Match "e"
- Start: cursor from prev iteration (after matching "d")
- Expected cursor.atom_position = 3 (2 + 1)
- Advance cursor: path.end.root_entry = 3 (past end!)
- **matched_cursor should be set to 3** (where we started matching "e")
- Compare and match "e"
- **Cache entries for "e" matching should use position 3** ← This is failing!
- Final cursor.atom_position = 4 (3 + 1)

## Problem: Getting Position 7 Instead of 3

Position 7 doesn't match any obvious calculation from the query structure:
- Not 0, 2, 3, or 4 (the expected positions)
- Not 6 (2+1+1+2) or 5 (2+1+1+1)
- Specifically 7

### Hypothesis 1: Double Advancement
Maybe the cursor is being advanced twice:
- Once in `advanced()` → position 4?
- Again somewhere else → position 7?
But this doesn't match the pattern.

### Hypothesis 2: Prefix Matching Artifact
During prefix matching in iteration 3, if token "e" has sub-structure, `prefix_states()` creates child CompareStates. These might be using a cursor position that was advanced too far.

### Hypothesis 3: Incorrect matched_cursor Propagation
Even though we set `self.state.matched_cursor = prev_state.cursor` before calling `compare()`, the CompareState instances created inside `compare()` might not be getting the correct matched_cursor value.

## Current Implementation

In `root_cursor.rs::next()`:
```rust
fn next(&mut self) -> Option<Self::Item> {
    let prev_state = self.state.clone();  // cursor at position A
    match self.advanced() {                // advances cursor to position B
        Continue(_) => {
            // Set matched_cursor to prev_state.cursor (position A)
            self.state.matched_cursor = prev_state.cursor.clone();
            
            Some(match CompareIterator::new(&self.trav, *self.state.clone()).compare() {
                Match(c) => {
                    *self.state = c;  // c has cursor at position C
                    Continue(())
                },
                ...
            })
        },
        ...
    }
}
```

In `compare/state.rs::prefix_states()`:
```rust
.map(|(sub, cursor)| Self {
    target: DownKey::new(
        sub.token(),
        (*self.matched_cursor.cursor_pos()).into(),  // Uses matched_cursor!
    ),
    ...
    matched_cursor: self.matched_cursor.clone(),  // Propagates matched_cursor
})
```

## Debugging Strategy

1. **Verify prev_state.cursor values**: Add logging to see what prev_state.cursor.atom_position is at each iteration
2. **Track matched_cursor propagation**: Log matched_cursor at each CompareState creation
3. **Identify position 7 source**: Search for where AtomPosition(7) could be calculated
4. **Check advancement logic**: Verify cursor.advance() is only called once per iteration

## Integration with context-insert

The cache positions are critical for context-insert because:
- `InitInterval::end_bound` uses the cursor position from EndState (fixed to use matched_cursor)
- `TraceCache` entries are used in split calculations via `top_down_splits()` and `bottom_up_splits()`
- The `outer_offset` in split calculations comes from cache entry positions
- Wrong positions → wrong split offsets → incorrect pattern insertion

## Resolution Path

The phantom type state infrastructure is correct, but we need to ensure:
1. `matched_cursor` accurately represents "position where current token matching started"
2. Cache entries use `matched_cursor` consistently throughout prefix matching
3. No hidden cursor advancements between setting matched_cursor and calling compare()
