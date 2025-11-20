# Queue Clearing Implementation Options Analysis

## Context: The Problem

When the query advances past the root token (finds first match in a root), we need to:
1. **Clear the queue** - Remove unmatched candidate parents (on unrelated branches)
2. **Generate parent states** - Add only parents of the matched root for continued exploration

**Current flow** (no queue clearing):
```
SearchIterator.next() → RootFinder → find_root_cursor() → RootCursor.find_end()
                                                              ↓
                                                    (if needs parents)
                                                              ↓
                                                    next_parents() → returns CompareParentBatch
                                                              ↓
                                                    Add to queue, continue iterating
```

**Problem locations:**
- `SearchState.next()` (search/mod.rs:120-180) - Tracks `last_match`, decides when to update
- `SearchIterator.next()` (match/iterator.rs:90-158) - Processes queue, calls RootCursor
- `RootCursor.find_end()` (match/root_cursor.rs:136-181) - Iterates until match or needs parents

## Option 1: Clear Queue in SearchState.next() (High-Level)

**Location:** `context-search/src/search/mod.rs` lines 120-180

**Concept:** When `SearchState.next()` detects first match, clear the queue immediately.

### Implementation

```rust
impl<K: TraversalKind> Iterator for SearchState<K> {
    type Item = MatchedEndState;
    fn next(&mut self) -> Option<Self::Item> {
        match self.matches.find_next() {
            Some(matched_state) => {
                let is_first_match = matches!(&self.last_match, MatchState::Query(_));
                
                let should_update = match &self.last_match {
                    MatchState::Located(prev_match) => {
                        let current_width = matched_state.root_parent().width.0;
                        let prev_width = prev_match.root_parent().width.0;
                        current_width < prev_width
                    },
                    MatchState::Query(_) => true,
                };

                if should_update {
                    if is_first_match {
                        // QUEUE CLEARING: Remove all unmatched candidate parents
                        debug!("First match found - clearing queue of candidate parents");
                        self.matches.queue.nodes.clear();
                        
                        // PARENT GENERATION: Add parents of matched root
                        if let Some(parent_nodes) = self.extract_parent_nodes(&matched_state) {
                            debug!("Adding {} parent nodes", parent_nodes.len());
                            self.matches.queue.nodes.extend(parent_nodes);
                        }
                    }
                    
                    self.last_match = MatchState::Located(matched_state.clone());
                }

                Some(matched_state)
            },
            None => None,
        }
    }
}

impl<K: TraversalKind> SearchState<K> {
    fn extract_parent_nodes(&self, matched_state: &MatchedEndState) -> Option<Vec<SearchNode>> {
        // Get PathEnum from MatchedEndState
        let path_enum = matched_state.path();
        
        // Extract IndexRangePath (works for Complete paths)
        let index_path = match path_enum {
            PathEnum::Complete(p) => p,
            PathEnum::Range(r) => &r.path,
            PathEnum::Postfix(p) => return None,  // Postfix uses different path type
            PathEnum::Prefix(p) => return None,   // Prefix doesn't have clear parents
        };
        
        // Get cursor from matched state
        let cursor = matched_state.cursor().clone();
        
        // Create ParentState from index path
        let parent_state = index_path.as_ref().clone().into();  // IndexRangePath → ChildState → ParentState
        
        // Use traversal policy to get parent batch
        K::Policy::next_batch(&self.matches.trace_ctx.trav, &parent_state)
            .map(|batch| {
                batch.parents
                    .into_iter()
                    .map(|ps| SearchNode::ParentCandidate(ParentCompareState {
                        parent_state: ps,
                        cursor: cursor.clone(),
                    }))
                    .collect()
            })
    }
}
```

### Pros
- ✅ **Clear separation** - Queue clearing logic isolated in SearchState
- ✅ **Simple** - Single place to modify
- ✅ **Correct timing** - Happens immediately when first match detected
- ✅ **Uses existing infrastructure** - Leverages K::Policy::next_batch

### Cons
- ⚠️ **Type complexity** - Need to extract different path types from MatchedEndState
- ⚠️ **Path conversion** - IndexRangePath → ChildState → ParentState (may need helpers)
- ⚠️ **Postfix/Prefix paths** - Different structure, may not have clear parent extraction

### Key Challenge: Path Type Extraction

**Problem:** `MatchedEndState` wraps `PathEnum` which has 4 variants:
- `Complete(IndexRangePath)` - ✅ Easy to extract
- `Range(RangeEnd)` - Contains `IndexRangePath` inside
- `Postfix(PostfixEnd)` - Uses `RootedRolePath<End>` (different structure)
- `Prefix(PrefixEnd)` - Uses `PatternPrefixPath` (different structure)

**Solution:** Add helper method to extract parent state from PathEnum:
```rust
impl PathEnum {
    pub(crate) fn to_parent_state(&self) -> Option<ParentState> {
        match self {
            PathEnum::Complete(p) => Some(/* convert p to ParentState */),
            PathEnum::Range(r) => Some(/* convert r.path to ParentState */),
            PathEnum::Postfix(_) => None,  // Complex, defer
            PathEnum::Prefix(_) => None,   // Complex, defer
        }
    }
}
```

---

## Option 2: Clear Queue in SearchIterator.next() (Mid-Level)

**Location:** `context-search/src/match/iterator.rs` lines 90-158

**Concept:** When `RootCursor.find_end()` returns a match, check if it's the first match and clear queue.

### Implementation

```rust
impl<K: TraversalKind> Iterator for SearchIterator<K> {
    type Item = MatchedEndState;

    fn next(&mut self) -> Option<Self::Item> {
        match RootFinder::<K>::new(&self.trace_ctx.trav, &mut self.queue)
            .find_root_cursor()
        {
            Some(root_cursor) => {
                Some(match root_cursor.find_end() {
                    Ok(matched_state) => {
                        // RootCursor found a match
                        // NOTE: We can't determine if it's "first match" here
                        // That logic lives in SearchState.next()
                        
                        matched_state
                    },
                    Err(root_cursor) => {
                        // Need to explore parents
                        match root_cursor.next_parents::<K>(&self.trace_ctx.trav) {
                            Err(_end_state) => {
                                return self.next();
                            },
                            Ok((parent, batch)) => {
                                // Add parent batch to queue
                                self.queue.nodes.extend(
                                    batch.into_compare_batch()
                                        .into_iter()
                                        .map(ParentCandidate),
                                );
                                return self.next();
                            },
                        }
                    },
                })
            },
            None => None,
        }
    }
}
```

### Pros
- ✅ **Already iterates over matches** - Natural place to intercept
- ✅ **Has access to queue** - Can clear directly

### Cons
- ❌ **Can't determine "first match"** - That state lives in SearchState.last_match
- ❌ **Wrong abstraction level** - SearchIterator shouldn't know about match tracking logic
- ❌ **Breaks separation** - Mixes iteration with match selection

**Verdict:** ❌ Not recommended - wrong abstraction level

---

## Option 3: Return Signal from RootCursor.find_end() (Low-Level)

**Location:** `context-search/src/match/root_cursor.rs` lines 136-181

**Concept:** Make `RootCursor.find_end()` return a signal indicating "first match in this root".

### Implementation

```rust
pub(crate) enum MatchResult {
    FirstMatchInRoot(MatchedEndState),  // NEW: Signal for queue clearing
    SubsequentMatch(MatchedEndState),    // Subsequent match in same root
}

impl<G: HasGraph + Clone> RootCursor<G, Matched, Matched> {
    pub(crate) fn find_end(self) -> Result<MatchResult, RootCursor<G, Candidate, Matched>> {
        // Determine if this is first match in this root
        // Problem: How? We don't have access to SearchState.last_match
        
        match self.advance_to_candidate() {
            Ok(candidate_cursor) => {
                match candidate_cursor.advance_to_matched() {
                    Ok(matched_cursor) => {
                        // Recurse to continue
                        matched_cursor.find_end()
                    },
                    Err(Ok(end_state)) => {
                        // Found match - but is it first in root?
                        // We can't tell from here!
                        Ok(MatchResult::FirstMatchInRoot(end_state))
                    },
                    Err(Err(candidate_cursor)) => {
                        Err(candidate_cursor)
                    },
                }
            },
            Err(Ok(end_state)) => Ok(MatchResult::FirstMatchInRoot(end_state)),
            Err(Err(candidate_cursor)) => Err(candidate_cursor),
        }
    }
}
```

### Pros
- ✅ **Low-level control** - Precise point where match occurs

### Cons
- ❌ **Can't determine "first"** - RootCursor doesn't have SearchState context
- ❌ **Wrong abstraction** - RootCursor shouldn't know about global match state
- ❌ **Breaks type system** - Return type becomes more complex

**Verdict:** ❌ Not recommended - wrong abstraction level

---

## Option 4: Hybrid Approach with Helper Method (RECOMMENDED)

**Concept:** Combine Option 1 with better helper infrastructure.

### Phase 1: Add Parent Extraction to MatchedEndState

```rust
// In context-search/src/state/matched/mod.rs
impl MatchedEndState {
    /// Extract parent state for queue repopulation
    /// Returns ParentState if this match has explorable parents
    pub(crate) fn to_parent_state(&self) -> Option<(IndexRangePath, PatternCursor)> {
        match self {
            MatchedEndState::Complete(state) => {
                match &state.path {
                    PathEnum::Complete(p) => Some((p.clone(), state.cursor.clone())),
                    PathEnum::Range(r) => Some((r.path.clone(), state.cursor.clone())),
                    PathEnum::Postfix(_) => None,  // Complex path type
                    PathEnum::Prefix(_) => None,   // Complex path type
                }
            },
            MatchedEndState::Partial(state) => {
                match &state.path {
                    PathEnum::Complete(p) => Some((p.clone(), state.cursor.clone())),
                    PathEnum::Range(r) => Some((r.path.clone(), state.cursor.clone())),
                    _ => None,
                }
            },
        }
    }
}
```

### Phase 2: Clear Queue in SearchState with Helper

```rust
impl<K: TraversalKind> SearchState<K> {
    fn extract_parent_batch(
        &self,
        matched_state: &MatchedEndState,
    ) -> Option<Vec<SearchNode>> {
        // Extract index path and cursor
        let (index_path, cursor) = matched_state.to_parent_state()?;
        
        // Convert to ChildState
        let child_state: ChildState = index_path.into();
        
        // Get parent state
        let parent_state = child_state.parent_state();
        
        // Use policy to get parent batch
        let batch = K::Policy::next_batch(&self.matches.trace_ctx.trav, &parent_state)?;
        
        // Convert to SearchNode
        Some(
            batch.parents
                .into_iter()
                .map(|ps| SearchNode::ParentCandidate(ParentCompareState {
                    parent_state: ps,
                    cursor: cursor.clone(),
                }))
                .collect()
        )
    }
}

impl<K: TraversalKind> Iterator for SearchState<K> {
    type Item = MatchedEndState;
    fn next(&mut self) -> Option<Self::Item> {
        match self.matches.find_next() {
            Some(matched_state) => {
                let is_first_match = matches!(&self.last_match, MatchState::Query(_));
                
                let should_update = match &self.last_match {
                    MatchState::Located(prev_match) => {
                        let current_width = matched_state.root_parent().width.0;
                        let prev_width = prev_match.root_parent().width.0;
                        current_width < prev_width
                    },
                    MatchState::Query(_) => true,
                };

                if should_update {
                    if is_first_match {
                        debug!("First match - clearing queue and adding parents");
                        self.matches.queue.nodes.clear();
                        
                        if let Some(parents) = self.extract_parent_batch(&matched_state) {
                            debug!("Adding {} parent nodes", parents.len());
                            self.matches.queue.nodes.extend(parents);
                        } else {
                            debug!("No parents to add (Postfix/Prefix path or root has no parents)");
                        }
                    }
                    
                    self.last_match = MatchState::Located(matched_state.clone());
                }

                Some(matched_state)
            },
            None => None,
        }
    }
}
```

### Pros
- ✅ **Clear separation** - Path extraction in MatchedEndState, queue clearing in SearchState
- ✅ **Type-safe** - Helper method handles all PathEnum variants
- ✅ **Reusable** - to_parent_state() could be useful elsewhere
- ✅ **Handles edge cases** - Returns None for Postfix/Prefix (deferred complexity)
- ✅ **Correct timing** - Queue cleared exactly when first match detected
- ✅ **Minimal changes** - Builds on existing infrastructure

### Cons
- ⚠️ **Postfix/Prefix deferred** - Need separate handling later (acceptable for now)
- ⚠️ **Type conversion chain** - IndexRangePath → ChildState → ParentState (unavoidable)

---

## Option 5: Lazy Queue Clearing (Alternative Strategy)

**Concept:** Instead of clearing queue immediately, mark nodes as "stale" and skip them during iteration.

### Implementation

```rust
// Add generation counter to SearchQueue
#[derive(Debug, Default)]
pub(crate) struct SearchQueue {
    pub(crate) nodes: BinaryHeap<SearchNode>,
    pub(crate) generation: usize,  // NEW: Increments on each match
}

// Modify SearchNode to track generation
#[derive(Debug)]
pub(crate) enum SearchNode {
    ParentCandidate {
        state: ParentCompareState,
        generation: usize,  // NEW: When this node was added
    },
    PrefixQueue(ChildQueue<CompareState<Candidate, Candidate>>),
}

// In SearchState.next():
if should_update && is_first_match {
    // Don't clear queue - just increment generation
    self.matches.queue.generation += 1;
    
    // Add new parents with current generation
    if let Some(parents) = self.extract_parent_batch(&matched_state) {
        self.matches.queue.nodes.extend(
            parents.into_iter().map(|p| SearchNode::with_generation(p, self.matches.queue.generation))
        );
    }
}

// In RootFinder (queue processing):
fn next(&mut self) -> Option<Self::Item> {
    loop {
        match self.ctx.nodes.pop() {
            Some(node) => {
                // Skip stale nodes (old generation)
                if node.generation() < self.ctx.generation {
                    continue;  // Skip this node
                }
                // Process node...
            }
            None => return None,
        }
    }
}
```

### Pros
- ✅ **No queue mutation** - Avoids clearing BinaryHeap (which is O(n))
- ✅ **Lazy evaluation** - Nodes filtered during iteration
- ✅ **Simpler parent extraction** - Just add new nodes, don't touch old ones

### Cons
- ❌ **More complex** - Adds generation tracking to multiple types
- ❌ **Memory overhead** - Keeps stale nodes in heap until popped
- ❌ **Unclear semantics** - "Stale" vs "fresh" nodes less obvious than clear()
- ❌ **More code changes** - Touches SearchNode, SearchQueue, RootFinder

**Verdict:** ⚠️ Over-engineered for this use case

---

## Comparison Matrix

| Option | Complexity | Correctness | Maintainability | Performance |
|--------|------------|-------------|-----------------|-------------|
| **1. SearchState** | Medium | ✅ High | ✅ High | ✅ Good |
| **2. SearchIterator** | Low | ❌ Can't track first | ❌ Wrong level | ✅ Good |
| **3. RootCursor** | Medium | ❌ Can't track first | ❌ Wrong level | ✅ Good |
| **4. Hybrid (BEST)** | Medium | ✅ High | ✅ Excellent | ✅ Good |
| **5. Lazy Clearing** | High | ✅ High | ⚠️ Medium | ⚠️ Memory overhead |

---

## Recommended Approach: Option 4 (Hybrid)

### Implementation Steps

1. **Add helper to MatchedEndState** (state/matched/mod.rs)
   ```rust
   pub(crate) fn to_parent_state(&self) -> Option<(IndexRangePath, PatternCursor)>
   ```

2. **Add extraction method to SearchState** (search/mod.rs)
   ```rust
   fn extract_parent_batch(&self, matched_state: &MatchedEndState) -> Option<Vec<SearchNode>>
   ```

3. **Modify SearchState::next()** (search/mod.rs)
   ```rust
   if should_update && is_first_match {
       self.matches.queue.nodes.clear();
       if let Some(parents) = self.extract_parent_batch(&matched_state) {
           self.matches.queue.nodes.extend(parents);
       }
   }
   ```

4. **Test incrementally**
   - Verify queue clearing doesn't break existing passing tests
   - Check that find_ancestor1_a_b_c_c now passes
   - Validate cache structure matches expected

### Edge Cases to Handle

1. **Postfix/Prefix paths** - Return None from to_parent_state() (acceptable for now)
2. **Root has no parents** - next_batch returns None (acceptable - search ends)
3. **Multiple matches in same root** - Only clear on first (is_first_match guard)
4. **Width comparison** - Already implemented ✅

### Testing Strategy

```rust
#[test]
fn test_queue_clearing() {
    // Setup graph with abc (3) and xyz (3) as competing roots
    // Query [a,b,c]
    // Expected: Find abc, clear queue, don't process xyz
    
    // Add instrumentation to count queue size before/after
    // Verify queue.nodes.len() == 0 after first match
    // Verify only abc's parents in queue afterwards
}
```

---

## Conclusion

**Recommended:** Option 4 (Hybrid Approach)

**Rationale:**
- ✅ Correct abstraction level (SearchState tracks matches)
- ✅ Clear separation of concerns (helper methods for extraction)
- ✅ Type-safe (handles all PathEnum variants)
- ✅ Minimal code changes (builds on existing infrastructure)
- ✅ Handles edge cases gracefully (Postfix/Prefix deferred)

**Key insight:** The "first match" detection belongs in SearchState because that's where `last_match` tracking lives. Queue clearing is a consequence of finding the first match, not a property of the match itself.

**Next steps:**
1. Implement to_parent_state() helper on MatchedEndState
2. Implement extract_parent_batch() on SearchState
3. Add queue clearing logic to SearchState::next()
4. Test with find_ancestor1_a_b_c_c
5. Verify all tests pass
