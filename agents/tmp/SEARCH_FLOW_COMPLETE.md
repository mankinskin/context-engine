# Complete Search Flow: From find_ancestor to Cache Entry

## Overview
This document traces the complete execution flow from `find_ancestor()` through the entire search algorithm, showing how `SearchIterator`, `RootFinder`, `NodeConsumer::consume`, and `RootCursor` work together.

## 1. Entry Point: `Find::find_ancestor()`

```rust
// In search/mod.rs
fn find_ancestor(&self, searchable: impl Searchable) -> SearchResult {
    searchable.search::<AncestorSearchTraversal<Self>>(self.ctx())
}
```

**Flow:**
1. User calls `graph.find_ancestor(pattern)`
2. Pattern is wrapped as `Searchable` (e.g., `PatternCursor`)
3. Calls `.search::<K>()` with traversal kind `K = AncestorSearchTraversal`

---

## 2. Start Search: `Searchable::search()`

```rust
// In state/start.rs
fn search<K: TraversalKind>(self, trav: K::Trav) -> Result<Response, ErrorState> {
    match self.start_search::<K>(trav) {
        Ok(ctx) => Ok(ctx.search()),  // <- Calls FoldCtx::search()
        Err(err) => Err(err),
    }
}
```
```rust
// In state/start.rs
fn search<K: TraversalKind>(self, trav: K::Trav) -> Result<Response, ErrorState> {
    match self.start_search::<K>(trav) {
        Ok(ctx) => Ok(ctx.search()),  // <- Calls SearchState::search()
        Err(err) => Err(err),
    }
}
```

**For PatternCursor:**

```rust
fn start_search<K: TraversalKind>(self, trav: K::Trav) -> Result<SearchState<K>, ErrorState> {
    let start_token = self.path.role_root_child_token::<End>(&trav);
    let start = StartCtx { cursor: self.clone() };
    
    match start.get_parent_batch::<K>(&trav) {
        Ok(p) => Ok(SearchState {
            last_match: EndState::init_fold(self),
            matches: SearchIterator::start_parent(trav, start_token, p),
        }),
        Err(err) => Err(err),
    }
}
```
## 3. Fold Search: `SearchState::search()`

```rust
// In search/mod.rs
impl<K: TraversalKind> SearchState<K> {
    pub(crate) fn search(mut self) -> Response {
        // Iterate through SearchIterator to find all matches
        while let Some(end_state) = self.matches.find_next() {
            // Update last_match if this is a better match
            if end_state.is_more_complete(&self.last_match) {
                self.last_match = end_state;
            }
        }
        
        // Convert final EndState to Response
        Response::from_end_state(self.last_match)
    }
}
```

**Flow:**
- Calls `SearchIterator::find_next()` repeatedly
- Each iteration produces one `EndState` (representing a match or mismatch)
- Keeps track of the best (most complete) match found
- Returns final `Response` with best match

---

## 4. Match Iteration: `SearchIterator::next()`

```rust
// In match/iterator.rs
impl<K: TraversalKind> Iterator for SearchIterator<K> {
    type Item = EndState;

    fn next(&mut self) -> Option<Self::Item> {
        // Step 1: Find a root cursor (initial match)
        match RootSearchIterator::<K>::new(&self.trace_ctx.trav, &mut self.match_ctx)
            .find_root_cursor()
        {
            Some(root_cursor) => {
                // Step 2: Process the root cursor to find end state
                Some(match root_cursor.find_end() {
                    Ok(end) => {
                        // Found end (query end, mismatch, etc.)
                        end
                    },
                    Err(root_cursor) => {
                        // Index ended but query continues - explore parents
                        match root_cursor.next_parents::<K>(&self.trace_ctx.trav) {
                            Err(end) => {
                                // No more parents - return saved complete match or this end
                                self.last_complete_match.take().unwrap_or(*end)
                            },
                            Ok((parent, batch)) => {
                                // Save complete match and queue parent batch
                                self.last_complete_match = Some(EndState::query_end(...));
                                self.match_ctx.nodes.extend(batch...);
                                
                                // Recursively call next() to explore parents
                                self.next().unwrap_or_else(|| 
                                    self.last_complete_match.take().unwrap()
                                )
                            },
                        }
                    },
                })
            },
            None => None,  // Queue exhausted
        }
    }
}
```

**Key State:**
```rust
struct SearchIterator<K> {
    trace_ctx: TraceCtx<K::Trav>,       // Graph traversal + cache
    match_ctx: SearchQueue,                 // Queue of nodes to process
    last_complete_match: Option<EndState>,  // Best complete match so far
}

struct SearchQueue {
    nodes: VecDeque<SearchNode>,  // Queue of ParentCandidate/PrefixQueue nodes
}

enum SearchNode {
    ParentCandidate(ParentCompareState),           // Parent candidate to explore
    PrefixQueue(ChildQueue<CandidateCompareState>),  // Child prefixes to compare
}
```

**Flow:**
1. Calls `RootFinder::find_root_cursor()` to find initial match
2. If found, calls `root_cursor.find_end()` to iterate through matches
3. Handles two cases:
   - **Ok(end)**: Found termination (query end, mismatch)
   - **Err(cursor)**: Index ended, query continues → explore parents
4. When exploring parents:
   - Saves current match as `last_complete_match`
   - Queues parent batch to `match_ctx.nodes`
   - Recursively calls `self.next()` to continue

---

## 5. Root Search: `RootFinder::find_root_cursor()`

```rust
// In match/mod.rs
impl<K: TraversalKind> RootFinder<'_, K> {
    fn find_root_cursor(mut self) -> Option<RootCursor<&'a K::Trav, Matched, Matched>> {
        // Uses Iterator::find_map to find first matched state
        self.find_map(|root| root).map(|matched_state| {
            RootCursor {
                trav: self.trav,
                state: Box::new(matched_state),
            }
        })
    }
}

impl<K: TraversalKind> Iterator for RootFinder<'_, K> {
    type Item = Option<MatchedCompareState>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.match_ctx.nodes.pop_front()?;
            
            match NodeConsumer(node, self.trav).consume() {
                Some(NodeResult::FoundMatch(matched_state)) => {
                    return Some(Some(matched_state));  // Found match!
                },
                Some(NodeResult::Skip) => continue,  // Mismatch, try next
                Some(NodeResult::QueueMore(new_nodes)) => {
                    self.match_ctx.nodes.extend(new_nodes);
                    continue;  // Added more nodes, keep searching
                },
                None => continue,  // Node exhausted, try next
            }
        }
    }
}
```

**Key Points:**
- Iterates through `match_ctx.nodes` queue
- Each node is processed by `NodeConsumer::consume()`
- Returns first `FoundMatch` found (as `MatchedCompareState`)
- Wraps in `RootCursor<Matched, Matched>` for further processing

---

## 6. Node Consumer: `NodeConsumer::consume()`

```rust
// In match/mod.rs
impl<K: TraversalKind> NodeConsumer<'_, K> {
    fn consume(self) -> Option<NodeResult> {
        match self.0 {
            ParentCandidate(parent) => {
                // Try to create initial compare state from parent
                match parent.into_advanced(&self.1) {
                    Ok(state) => {
                        // Successfully created initial state
                        Self::compare_next(self.1, ChildQueue::from_iter([state.token]))
                    },
                    Err(parent) => {
                        // Parent exhausted - get next level parents
                        Some(QueueMore(
                            K::Policy::next_batch(self.1, &parent)
                                .into_iter()
                                .flat_map(|batch| batch.parents)
                                .map(|parent_state| ParentCompareState { ... })
                                .map(ParentCandidate)
                                .collect(),
                        ))
                    },
                }
            },
            PrefixQueue(queue) => Self::compare_next(self.1, queue),
        }
    }

    fn compare_next(trav: &K::Trav, queue: ChildQueue<CandidateCompareState>) 
        -> Option<NodeResult> 
    {
        let mut compare_iter = CompareIterator::new(trav, queue);
        
        match compare_iter.next() {
            Some(Some(CompareResult::FoundMatch(matched_state))) => {
                Some(NodeResult::FoundMatch(matched_state))  // Found match!
            },
            Some(Some(CompareResult::Mismatch(_))) => {
                Some(Skip)  // Mismatch, try next node
            },
            Some(Some(CompareResult::Prefixes(_))) => unreachable!(),
            Some(None) => {
                // Generated prefixes - add as PrefixQueue nodes
                Some(QueueMore(vec![PrefixQueue(compare_iter.children.queue)]))
            },
            None => None,  // Iterator exhausted
        }
    }
}
```

**Node Processing:**

**For ParentCandidate Node:**
1. Try to advance parent to create initial `CompareState<Candidate, Candidate>`
2. If successful → call `compare_next()` with this state
3. If parent exhausted → get next level parents and append to queue

**For PrefixQueue Node:**
1. Create `CompareIterator` with child queue
2. Compare query pattern against graph
3. Return:
   - `FoundMatch(matched_state)` → Found initial match!
   - `Mismatch` → Skip, try next node
   - `None` with prefixes → QueueMore prefixes as new PrefixQueue nodes

---

## 7. Root Cursor Processing: `RootCursor<Matched, Matched>::find_end()`

```rust
// In match/root_cursor.rs
impl<G: HasGraph + Clone> RootCursor<G, Matched, Matched> {
    fn find_end(self) -> Result<EndState, RootCursor<G, Candidate, Matched>> {
        let matched_state = *self.state;
        let trav = self.trav;
        
        // Try to advance query cursor
        match matched_state.advance_query_cursor(&trav) {
            Ok(query_advanced) => {
                // Query advanced, now try index
                match query_advanced.advance_index_cursor(&trav) {
                    Ok(both_advanced) => {
                        // Both advanced - create candidate cursor and iterate
                        let candidate_cursor = RootCursor {
                            state: Box::new(both_advanced),
                            trav,
                        };
                        candidate_cursor.find_end().map_err(|_| 
                            panic!("Candidate RootCursor completed without Break")
                        )
                    },
                    Err(query_only_advanced) => {
                        // Index ended but query continues
                        // Return RootCursor<Candidate, Matched> for parent exploration
                        Err(RootCursor {
                            state: Box::new(query_only_advanced),
                            trav,
                        })
                    },
                }
            },
            Err(matched_state) => {
                // Query ended immediately - complete match!
                Ok(EndState { reason: EndReason::QueryEnd, ... })
            },
        }
    }
}
```

**Cases:**

**Case 1: Query Ends Immediately**
- `advance_query_cursor()` fails
- **Result:** `Ok(EndState)` with `QueryEnd` reason
- **Cache:** Entry added for this position

**Case 2: Both Cursors Advance**
- Both `advance_query_cursor()` and `advance_index_cursor()` succeed
- Creates `RootCursor<Candidate, Candidate>`
- Delegates to `RootCursor<Candidate, Candidate>::find_end()`
- **Continues iteration** through pattern

**Case 3: Index Ends, Query Continues** ← **This is our test case!**
- `advance_query_cursor()` succeeds
- `advance_index_cursor()` fails (graph path ended)
- **Result:** `Err(RootCursor<Candidate, Matched>)`
- **Handled by:** `SearchIterator::next()` → calls `next_parents()`
- **Cache:** Entry added for partial match (e.g., "xab")

---

## 8. Candidate Cursor Iteration: `RootCursor<Candidate, Candidate>::next()`

```rust
// In match/root_cursor.rs
impl<G: HasGraph + Clone> Iterator for RootCursor<G, Candidate, Candidate> {
    type Item = ControlFlow<EndReason>;

    fn next(&mut self) -> Option<Self::Item> {
        // Compare current candidate state
        match CompareIterator::new(&self.trav, *self.state.clone()).compare() {
            FoundMatch(matched_state) => {
                // Create Matched cursor and try to advance
                let matched_cursor = RootCursor {
                    state: Box::new(matched_state),
                    trav: self.trav.clone(),
                };
                
                match matched_cursor.advance_cursors() {
                    Ok(candidate_cursor) => {
                        // Both advanced - update self and continue
                        *self = candidate_cursor;
                        Some(Continue(()))
                    },
                    Err((EndReason::QueryEnd, None)) => {
                        // Query ended - complete match
                        Some(Break(EndReason::QueryEnd))
                    },
                    Err((EndReason::Mismatch, Some(_))) => {
                        // Index ended - need parent exploration
                        None  // Signals to return Err from find_end()
                    },
                    _ => unreachable!(),
                }
            },
            Mismatch(_) => {
                // Check checkpoint to see if partial match
                if self.state.checkpoint.atom_position != AtomPosition::from(0) {
                    Some(Break(EndReason::Mismatch))  // Partial match
                } else {
                    Some(Break(EndReason::Mismatch))  // Immediate mismatch
                }
            },
            Prefixes(_) => unreachable!(),
        }
    }
}

impl<G: HasGraph + Clone> RootCursor<G, Candidate, Candidate> {
    fn find_end(mut self) -> Result<EndState, Self> {
        // Uses Iterator::find_map to find first Break
        match self.find_map(|flow| match flow {
            Continue(()) => None,
            Break(reason) => Some(reason),
        }) {
            Some(reason) => {
                // Found end - construct EndState
                Ok(EndState { reason, cursor: self.state.checkpoint, ... })
            },
            None => {
                // Iterator completed without Break
                // This means we need parent exploration
                Err(self)
            },
        }
    }
}
```

**Iteration Loop:**
1. Compare current state: query pattern vs graph path
2. **If Match:**
   - Create `RootCursor<Matched, Matched>`
   - Try to advance both cursors
   - If both advance → update self, continue loop
   - If query ends → `Break(QueryEnd)`
   - If index ends → return `None` → signals parent exploration
3. **If Mismatch:**
   - Check if partial match (checkpoint > 0)
   - `Break(Mismatch)` → end iteration
4. `find_end()` collects first `Break` reason

---

## 9. Parent Exploration Flow

When `RootCursor::find_end()` returns `Err(cursor)`:

```rust
// Back in SearchIterator::next()
Err(root_cursor) => {
    // root_cursor is RootCursor<Candidate, Matched>
    match root_cursor.next_parents::<K>(&self.trace_ctx.trav) {
        Ok((parent, batch)) => {
            // 1. Save current match
            self.last_complete_match = Some(EndState::query_end(...));
            
            // 2. Add parent batch to queue
            self.match_ctx.nodes.extend(
                batch.into_compare_batch()
                    .into_iter()
                    .map(ParentCandidate)
            );
            
            // 3. Recursively search parents
            self.next().unwrap_or_else(|| 
                self.last_complete_match.take().unwrap()
            )
        },
        Err(end) => {
            // No more parents available
            self.last_complete_match.take().unwrap_or(*end)
        },
    }
}
```

**Flow:**
1. Call `next_parents()` to get parent candidates
2. Save current position as complete match
3. Queue parent nodes for exploration
4. Recursively call `next()` to process parent queue
5. If parent search succeeds → returns better match
6. If parent search fails → returns saved match

---

## 10. Complete Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. find_ancestor(pattern)                                       │
│    ↓                                                             │
│ 2. Searchable::search<K>()                                      │
│    ├─> start_search() → get initial parent batch                │
│    └─> SearchState::search()                                    │
│        ↓                                                         │
│ 3. SearchIterator::next() [LOOP]                                │
│    ├─> RootFinder::find_root_cursor()                           │
│    │   ├─> RootFinder::next() [LOOP]                            │
│    │   │   ├─> NodeConsumer::consume()                          │
│    │   │   │   ├─> ParentCandidate → into_advanced() → CompareIterator   │
│    │   │   │   └─> PrefixQueue → CompareIterator                │
│    │   │   └─> Returns FoundMatch/Skip/QueueMore                │
│    │   └─> Returns MatchedCompareState                          │
│    │       ↓                                                     │
│    └─> RootCursor<Matched, Matched>::find_end()                 │
│        ├─> advance_query_cursor()                               │
│        ├─> advance_index_cursor()                               │
│        ├─ Case 1: Query ends → Ok(EndState)                     │
│        ├─ Case 2: Both advance → RootCursor<Candidate, Candidate>│
│        │   └─> RootCursor::next() [LOOP]                        │
│        │       ├─> compare() → Match/Mismatch                   │
│        │       ├─> Match → advance_cursors()                    │
│        │       └─> Continue/Break                               │
│        │   └─> find_end() → Ok(EndState)                        │
│        └─ Case 3: Index ends → Err(RootCursor<Candidate,Matched>)│
│            └─> next_parents() → parent batch                    │
│                ├─> Save complete match                          │
│                ├─> Queue parent batch                           │
│                └─> Recursive next()                             │
│                    ↓                                             │
│                [Back to step 3 with parent nodes]               │
└─────────────────────────────────────────────────────────────────┘
```

---

## 11. Test Case: "xab" Pattern Flow

**Graph:**
```
Token 0: "x"     (atom)
Token 1: "a"     (atom)  
Token 2: "b"     (atom)
Token 3: "xab"   (pattern: [0,1,2])
```

**Query:** Find "xab" pattern

**Execution:**

### Initial Setup
```
start_search():
  - start_token = Token 3 ("xab")
  - cursor = PatternCursor for "xab"
  - parent_batch = parents of Token 3 in graph
```

### First Iteration
```
SearchIterator::next():
  └─> RootFinder::find_root_cursor()
      └─> NodeConsumer::consume(ParentCandidate(Token 3))
          └─> compare_next()
              └─> CompareIterator::next()
                  Query: "xab" vs Graph: "xab"
                  └─> FoundMatch! Returns MatchedCompareState
      
  └─> RootCursor<Matched, Matched> {
        cursor: "xab" at position 3 ✓
        index_cursor: "xab" at position 3 ✓
      }
  
  └─> find_end():
      ├─> advance_query_cursor() → Fails (query ended)
      └─> Returns Ok(EndState { reason: QueryEnd })
```

**Result:** Complete match found! Cache entry created for Token 3.

---

## 12. Cache Entry Creation

Cache entries are created by `TraceCache` when:
1. **Query ends** → `EndReason::QueryEnd`
2. **Index ends** → Parent exploration triggered
3. **Partial match** → `EndReason::Mismatch` with checkpoint > 0

The cache stores these as `VertexCache` entries mapping Token → Position → SubLocation.

---

## Summary

The search algorithm is a **breadth-first exploration** with key components:

1. **SearchIterator**: Main loop, manages queue of nodes to explore
2. **RootFinder**: Finds initial matches from node queue
3. **NodeConsumer::consume()**: Processes nodes (ParentCandidate/PrefixQueue), generates matches
4. **RootCursor**: Iterates through matched states, handles cursor advancement
5. **Parent Exploration**: When index ends, explores parent graph nodes
6. **Cache**: Stores successful match positions for reuse

**Key Innovation:** Type-state pattern with `RootCursor<G, Q, I>` where:
- `Q` = Query cursor state (Matched/Candidate)
- `I` = Index cursor state (Matched/Candidate)
- Enables independent cursor advancement
- Supports "index ends, query continues" case for parent exploration
