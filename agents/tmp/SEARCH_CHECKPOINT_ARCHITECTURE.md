# Search Checkpoint Architecture Issue

## Current Problem

When a root cursor matches successfully but needs parent exploration (pattern exhausted, query continues):
- `find_end()` returns `Err(root_cursor)` 
- Parents are added to queue
- NO MatchedEndState is yielded
- `last_match` is NEVER updated
- The successful match is lost!

Example: Query `[a,b,c,c]` with root "abc":
- Matches all 3 tokens successfully → checkpoint at position 3
- Pattern exhausted, query continues with 'c'
- Needs parents → returns Err, adds parents like "abcd", "abcdef"
- Those parents get explored and produce Mismatches
- But "abc"'s successful match is never saved!

## Root Cause

The checkpoint (best match) is only updated when the iterator yields a MatchedEndState.
Roots that need parent exploration don't yield anything, so their matches are lost.

## Proposed Fix

**Track checkpoint separately from iteration:**

1. Add `best_checkpoint: Option<MatchedEndState>` to SearchIterator or SearchState
2. When a root completes successfully (even if needing parents), create a QueryExhausted state and update best_checkpoint
3. Continue exploring parents to try to extend the match
4. When queue exhausted, return best_checkpoint

## Implementation Approach

### Option A: Update in Iterator (when needing parents)
```rust
Err(root_cursor) => {
    // Create QueryExhausted state from checkpoint
    let checkpoint_state = create_query_exhausted_from_checkpoint(&root_cursor);
    
    // Update internal best_checkpoint if better
    if should_update_checkpoint(&self.best_checkpoint, &checkpoint_state) {
        self.best_checkpoint = Some(checkpoint_state);
    }
    
    // Add parents and continue searching
    match root_cursor.next_parents() {
        Ok((parent, batch)) => {
            self.queue.nodes.extend(batch);
            return self.next(); // Continue iteration
        }
        Err(_) => return self.next(), // No parents, try next candidate
    }
}
```

### Option B: Refactor Search Loop
Don't iterate over matches - just exhaust queue while tracking best_checkpoint:
```rust
fn search(mut self) -> Response {
    let mut best_checkpoint = None;
    
    while let Some(node) = self.queue.pop() {
        match process_node(node) {
            NodeResult::CompleteMatch(state) => {
                if better_than(&best_checkpoint, &state) {
                    best_checkpoint = Some(state);
                }
            }
            NodeResult::NeedParents(root, parents) => {
                // Extract checkpoint from root
                let checkpoint = extract_checkpoint(root);
                if better_than(&best_checkpoint, &checkpoint) {
                    best_checkpoint = Some(checkpoint);
                }
                // Add parents for exploration
                self.queue.extend(parents);
            }
            NodeResult::Failed => continue,
        }
    }
    
    Response {
        end: best_checkpoint.unwrap_or(create_no_match_state()),
        cache: self.cache,
    }
}
```

## Key Insight

The checkpoint represents "best match found so far during exploration".
It should be updated whenever we successfully match a root, regardless of whether:
- We find a Mismatch after the match (query continues, pattern doesn't)
- We need parent exploration (pattern exhausted, query continues)
- We find a complete QueryExhausted match

All three cases represent successful matches at some level!
