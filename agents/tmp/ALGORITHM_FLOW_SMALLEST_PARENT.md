# Algorithm: Find Smallest Parent with Largest Contiguous Match

## Overview

The matching algorithm searches for the **smallest parent node that contains the largest contiguous match** of the query pattern. It explores the parent hierarchy by traversing from smaller to larger parents, stopping when it finds a partial match or exhausts all possibilities.

## Core Concept

### Match Types

1. **Partial Match** (SUCCESS case)
   - Pattern matches for some atoms, then mismatches within the same root
   - Detected by: Mismatch while searching for end of RootCursor
   - Action: **STOP and return immediately** - this is the smallest parent with largest match

2. **Complete Match** (CONTINUE exploring)
   - Pattern matches to the end of the current root
   - Detected by: `RootCursor.find_end()` returns `Err` (cannot advance, matched entire root)
   - Action: **Save as last_complete_match, add parents to queue, continue searching**

3. **Immediate Mismatch** (NO match)
   - Pattern mismatches immediately without any matches
   - Detected by: mismatch in candidate path when trying to create RootCursor
   - Action: Revert state, break, try other paths

## Algorithm Flow

```
Start with initial candidate
    ↓
┌─→ Pop next TraceNode from queue
│       ↓
│   RootCursor iterates through root:
│       - Compare each position
│       - On match: advance cursor, update checkpoint
│       - Continue until mismatch or end
│       ↓
│   ┌─────────────────────────────┐
│   │  find_end() Result?         │
│   └─────────────────────────────┘
│            ↓           ↓
│         Ok(end)     Err(root_cursor)
│            ↓           ↓
│   ┌──────────────┐   Matched to end of root
│   │ Check reason │   (Complete Match)
│   └──────────────┘   ↓
│            ↓         next_parents()?
│   Mismatch?         ↓          ↓
│   checkpoint≠0?    Ok(batch)  Err(end)
│       ↓               ↓          ↓
│   YES: Partial      Save to   Return last_complete_match
│   Match - RETURN   last_complete_match    or end state
│       ↓               ↓
│   NO: Continue     Add parents to queue
│                       ↓
└───────────────────────┘
    Continue iteration
```

## Key Data Structures

### MatchIterator Fields

```rust
pub(crate) struct MatchIterator<K: TraversalKind> {
    pub(crate) trace_ctx: TraceCtx<K::Trav>,
    pub(crate) match_ctx: MatchCtx,
    /// Tracks the largest complete match found during parent exploration
    pub(crate) last_complete_match: Option<EndState>,
}
```

- **trace_ctx**: Traversal context with graph access
- **match_ctx**: Queue of TraceNodes to explore (VecDeque)
- **last_complete_match**: Stores the best complete match found so far

### CompareState with Checkpoint

```rust
pub(crate) struct CompareState<S> {
    pub(crate) cursor: PathCursor<RootedRangePath, S>,
    pub(crate) token: CompareNext,
    /// Checkpoint tracks the last MATCHED cursor position
    /// Only updated on successful matches, never on mismatch
    pub(crate) checkpoint: RootedRangePath,
}
```

- **cursor**: Current position in graph (with state: Candidate/Matched/Mismatched)
- **token**: Next token to compare
- **checkpoint**: Last matched position (for determining partial vs immediate mismatch)

## Implementation Details

### Checkpoint Semantics (CRITICAL)

The checkpoint represents the **last position where a match occurred**:

1. **On Match**: 
   - Old matched cursor becomes new checkpoint
   - Cursor advances to next position
   - Handled in `into_next_candidate()`

2. **Never on Mismatch**:
   - Checkpoint stays at last matched position
   - Used to detect if we've made any progress (partial match detection)

### State Transitions

```
Candidate --[compare]--> Match or Mismatch
                          ↓
Match --[into_next_candidate()]--> Candidate (with updated checkpoint)
                                   or stays Matched (cannot advance)
```

The `into_next_candidate()` method:
```rust
fn into_next_candidate() -> Result<CandidateCompareState, MatchedCompareState>
```
- Converts old matched cursor to checkpoint (PrefixPath → RangePath)
- Advances cursor to next position
- Returns Ok(Candidate) if advancement succeeds
- Returns Err(Matched) if cannot advance (end of root)

### RootCursor Iteration

`RootCursor` is an iterator that compares candidates until match/mismatch:

```rust
impl Iterator for RootCursor {
    fn next(&mut self) -> Option<IteratorControl<EndReason>> {
        // Compare current position
        match self.state.compare(self.trav) {
            Match(matched) => {
                // Convert to next candidate with checkpoint update
                match matched.into_next_candidate(self.trav) {
                    Ok(candidate) => {
                        self.state = candidate;
                        Some(Continue) // Keep iterating
                    }
                    Err(matched) => {
                        // Cannot advance - matched to end of root
                        self.state = matched;
                        None // End iteration (signals complete match)
                    }
                }
            }
            Mismatch(_) => {
                // Check if partial match (had matches before)
                if checkpoint.atom_position != 0 {
                    Some(Break(Mismatch)) // Partial match - SUCCESS
                } else {
                    Some(Break(Mismatch)) // Immediate mismatch - NO MATCH
                }
            }
        }
    }
}
```

**Outcomes**:
- `Some(Continue)`: Matched and advanced, keep going
- `Some(Break(Mismatch))`: Mismatch found (check checkpoint for partial vs immediate)
- `None`: Matched to end of root (complete match, need parents)

### Iterator Logic (MatchIterator::next)

```rust
fn next(&mut self) -> Option<EndState> {
    match find_root_cursor() {
        Some(root_cursor) => {
            match root_cursor.find_end() {
                Ok(end) => {
                    // Partial match or immediate mismatch
                    if end.reason == Mismatch && end.cursor.atom_position != 0 {
                        return Some(end); // SUCCESS - found smallest parent
                    }
                    Some(end) // Other end reasons
                }
                Err(root_cursor) => {
                    // Complete match - matched to end of root
                    match root_cursor.next_parents() {
                        Ok((parent, batch)) => {
                            // Save this complete match
                            let complete_match = EndState::query_end(trav, parent);
                            self.last_complete_match = Some(complete_match);
                            
                            // Add parents to queue
                            self.match_ctx.nodes.extend(batch.into_iter().map(Parent));
                            
                            // Recursively continue
                            self.next()
                        }
                        Err(end) => {
                            // No more parents
                            // Return last complete match or end state
                            self.last_complete_match.take()
                                .or(Some(*end))
                        }
                    }
                }
            }
        }
        None => {
            // Queue exhausted
            self.last_complete_match.take()
        }
    }
}
```

## Parent Exploration Strategy

### Current Implementation
- Parents are added to queue in batches via `next_parents()`
- Queue is processed FIFO (VecDeque)

### Future Enhancement: Width-Based Priority
Parents should be explored in order of **root width** (smallest first):

**Rationale**: Smaller parents are more specific matches, so we want to find the smallest parent that contains the complete match.

**Implementation needed**:
- Sort queue by root width when inserting parents
- Or use priority queue (BinaryHeap) with width ordering
- Preserve insertion order for same-width parents

## Why This Design?

### Goal: Smallest Parent with Largest Match

The algorithm prioritizes **specificity** (smallest parent) while maximizing **match length** (largest contiguous match).

**Example**:
```
Query: "hello world"
Graph:
  - Token "hello" (small parent)
  - Phrase "hello world" (medium parent) 
  - Sentence "hello world today" (large parent)
```

Exploration order:
1. Try "hello" token → Complete match (only "hello"), continue to parents
2. Try "hello world" phrase → Complete match (both words), continue to parents
3. Try "hello world today" sentence → Partial match (mismatch at "today")
4. **Return**: sentence node with match up to "world" (smallest parent with complete query match)

### Why Partial Match = Success?

When we find a partial match (match followed by mismatch in same root), we know:
- This root contains the entire matched portion
- Any parent would be larger (less specific)
- This is the **smallest root** that contains our match
- **STOP here** - we've found the optimal result

### Why Continue on Complete Match?

When we match to the end of a root:
- The match might be contained in a smaller parent
- We need to explore parents to find the smallest containing node
- Save this as a candidate and keep exploring
- Only return it if we exhaust all smaller options

## Edge Cases

### Query Matches Single Token
- Complete match on initial token
- No parents available
- Return the token itself (could be in `last_complete_match`)

### Query Matches Nothing
- Immediate mismatch everywhere
- Queue exhausts with no matches
- Return None or last complete match (if any token matched partially)

### Query Longer Than All Paths
- Complete matches on all explored roots
- Parents exhausted
- Return `last_complete_match` (largest root explored)

## AtomPosition Comparison

**Important**: `AtomPosition` does not implement `PartialOrd`, only `PartialEq`.

Use:
```rust
✓ checkpoint.atom_position != AtomPosition::from(0)  // Check for any progress
✗ checkpoint.atom_position > AtomPosition::from(0)   // Compilation error
```

Or convert to usize:
```rust
✓ usize::from(checkpoint.atom_position) > 0
```

## Testing Scenarios

### Test: Partial Match Detection
1. Pattern: "hello world"
2. Root: "hello world today"
3. Expected: Match "hello world", mismatch at "today", return with checkpoint at end of "world"

### Test: Complete Match Parent Exploration  
1. Pattern: "hello"
2. Roots: token "hello" → phrase "hello world" → sentence
3. Expected: Match token completely, explore parents, find partial match in phrase

### Test: Queue Exhaustion
1. Pattern: "hello world"
2. Root: "hello world" (exact match, no parents)
3. Expected: Return token as last_complete_match when parents exhausted

## Related Files

- `context-search/src/match/iterator.rs` - Main iterator with algorithm logic
- `context-search/src/match/root_cursor.rs` - Root iteration and partial match detection
- `context-search/src/compare/state.rs` - State management and checkpoint updates
- `context-search/src/cursor/mod.rs` - MarkMatchState trait for state transitions
- `context-search/src/search/mod.rs` - FoldCtx using MatchIterator

## Migration Notes

### What Changed from Previous Implementation

**Old behavior**: 
- First match wins, stopped immediately
- Cleared queue on match
- No distinction between partial and complete matches

**New behavior**:
- Explores parent hierarchy to find smallest parent
- Keeps queue alive for continued exploration
- Distinguishes partial (stop) vs complete (continue) matches
- Tracks `last_complete_match` across iterations

### Breaking Changes
- MatchIterator changed from tuple struct to named fields
- Field access changed from `.0`/`.1` to `.trace_ctx`/`.match_ctx`
- Iterator may return different results (smallest parent vs first match)

## Future Work

1. **Queue Sorting**: Implement width-based parent exploration order
2. **Performance**: Consider caching parent widths to avoid repeated calculations
3. **Metrics**: Add tracing for exploration depth and parent count
4. **Tests**: Add comprehensive tests for all match scenarios
