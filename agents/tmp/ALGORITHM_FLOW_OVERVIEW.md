# Search Algorithm Flow - Preferred Architecture

> **Overview of the intended algorithm flow for context-search pattern matching**
>
> **Note:** This describes the INTENDED design based on refactoring analysis, not necessarily the current broken implementation.

## Table of Contents
1. [High-Level Search Flow](#high-level-search-flow)
2. [State Machine Architecture](#state-machine-architecture)
3. [Iteration Layers](#iteration-layers)
4. [Token Comparison Logic](#token-comparison-logic)
5. [Cursor State Transitions](#cursor-state-transitions)
6. [Prefix Decomposition Strategy](#prefix-decomposition-strategy)
7. [Key Design Principles](#key-design-principles)

---

## High-Level Search Flow

### The Three-Phase Model

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: START SEARCH                                        │
│ - Initialize from query pattern                              │
│ - Find starting positions in graph                           │
│ - Set up initial candidate states                            │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Phase 2: MATCH ITERATION (Fold)                              │
│ - Iterate through parent patterns (RootSearchIterator)       │
│ - For each root, iterate through match candidates            │
│ - Compare tokens, decompose when needed                      │
│ - Build trace cache as we go                                 │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Phase 3: RESULT CONSTRUCTION                                 │
│ - Complete match: Return full pattern path                   │
│ - Incomplete: Return partial match with reason               │
│ - Error: Return error state with context                     │
└─────────────────────────────────────────────────────────────┘
```

### Entry Points

1. **Searchable Trait** - Unified interface for all search types:
   ```rust
   trait Searchable {
       fn search<K: TraversalKind>(self, graph) -> Result<Response, ErrorState>;
   }
   ```

2. **Convenience Functions** - User-facing API:
   ```rust
   find_ancestor(query, graph)  // Find containing pattern
   find_pattern(query, graph)   // Find exact match
   ```

---

## State Machine Architecture

### Cursor States (Type-Safe Phases)

The cursor moves through distinct states during matching:

```rust
// Three main cursor states
enum CursorState {
    Candidate,    // Token proposed for matching
    Matched,      // Token confirmed as matched
    Mismatched,   // Token failed to match
}

// Additionally:
enum CursorEnd {
    Exhausted,    // Pattern fully consumed
}
```

### State Transition Diagram

```
    ┌──────────────┐
    │ Initial      │
    │ (Matched)    │
    └──────┬───────┘
           │ advance()
           ↓
    ┌──────────────┐
    │  Candidate   │ ← Start here for comparison
    └──┬────────┬──┘
       │        │
       │compare │
       ↓        ↓
┌─────────┐  ┌──────────┐
│ Matched │  │Mismatched│
└────┬────┘  └────┬─────┘
     │            │
     │advance()   │revert()
     ↓            ↓
   (next)      (backtrack)
```

### CompareState - The Core State Type

```rust
pub struct CompareState<S: CursorState> {
    /// Graph path being matched against (ChildState)
    child_state: ChildState,
    
    /// Query cursor in state S
    cursor: PathCursor<RootedRolePath, S>,
    
    /// Checkpoint: last confirmed match position (always Matched)
    checkpoint: PathCursor<PatternRangePath, Matched>,
    
    /// Target position in graph
    target: DownKey,
    
    /// Comparison mode (GraphMajor vs QueryMajor)
    mode: PathPairMode,
}

// Type aliases for clarity
type CandidateState = CompareState<Candidate>;
type MatchedState = CompareState<Matched>;
type MismatchedState = CompareState<Mismatched>;
```

**Key Design Points:**
- **Checkpoint is always Matched** - represents the "safe position" to return to
- **Cursor changes state** - goes from Candidate → Matched → (advance) → Candidate
- **Mode determines decomposition** - which token to decompose into prefixes

---

## Iteration Layers

The search uses a **nested iteration architecture** with three distinct layers:

### Layer 1: RootSearchIterator (Match Level)

**Purpose:** Iterate through different parent patterns at the root level

```rust
RootSearchIterator {
    nodes: VecDeque<TraceNode>,  // Queue of parent patterns to try
}
```

**Iteration logic:**
1. Pop next parent pattern from queue
2. Create initial CompareState for that parent
3. Yield to Layer 2 (RootCursor) for matching
4. If mismatch, try next parent in queue
5. If match, complete successfully

**What it yields:** `CompareState<Matched>` - a successful match in a parent pattern

### Layer 2: RootCursor (Token Sequence Level)

**Purpose:** Match sequence of tokens within a single parent pattern

```rust
RootCursor {
    state: CompareState<Candidate>,  // Current comparison state
    trav: GraphTraversal,            // Graph context
}
```

**Iteration logic:**
1. Start with candidate state
2. Compare candidate token (via Layer 3)
3. If match: Update checkpoint, advance to next token, repeat
4. If mismatch: Return to Layer 1 to try different parent
5. If exhausted (all tokens matched): Success!

**What it yields:** `ControlFlow<EndReason>` - Continue or Break with reason

**Key method: `next()`**
```rust
fn next(&mut self) -> Option<ControlFlow<EndReason>> {
    // 1. Compare current candidate
    match CompareIterator::new(&self.trav, self.state.clone()).compare() {
        Match(matched_state) => {
            // 2. Try to advance to next candidate
            match matched_state.into_next_candidate(&self.trav) {
                Ok(next_candidate) => {
                    self.state = next_candidate;
                    Some(Continue(()))  // Keep going
                },
                Err(_) => {
                    // Pattern exhausted - success!
                    Some(Break(EndReason::Complete))
                }
            }
        },
        Mismatch(mismatched_state) => {
            // Matching failed
            Some(Break(EndReason::Mismatch))
        }
    }
}
```

### Layer 3: CompareIterator (Prefix Decomposition Level)

**Purpose:** Handle token comparison with prefix decomposition when needed

```rust
CompareIterator {
    children: ChildIterator<CompareState<Candidate>>,  // Queue of comparison states
}
```

**Iteration logic:**
1. Pop next candidate from queue
2. Compare tokens (call `next_match()`)
3. If direct match/mismatch: Return result
4. If tokens need decomposition: Generate prefix states, add to queue, continue
5. Repeat until match/mismatch determined

**What it yields:** `CompareNext` enum
```rust
enum CompareNext {
    Match(CompareState<Matched>),      // Tokens matched!
    Mismatch(CompareState<Mismatched>), // Tokens don't match
    Prefixes(ChildQueue<CompareState<Candidate>>), // Needs decomposition (internal only)
}
```

**Key method: `compare()`**
```rust
fn compare(self) -> CompareNext {
    // Uses Iterator::find_map to process queue until definitive result
    self.find_map(|result| result).unwrap()
}
```

---

## Token Comparison Logic

### The Three Comparison Outcomes

Located in `CompareState::next_match()`:

```rust
fn next_match<G: HasGraph>(self, trav: &G) -> CompareNext {
    let path_token = self.child_state.leaf_token(trav);
    let query_token = self.cursor.leaf_token(trav);
    
    if path_token == query_token {
        // ✓ Exact match - tokens are identical
        CompareNext::Match(self.mark_match())
        
    } else if path_token.width() == 1 && query_token.width() == 1 {
        // ✗ Atom mismatch - both are atoms (width=1) but different
        CompareNext::Mismatch(self.mark_mismatch())
        
    } else {
        // ? Need decomposition - at least one is composite
        CompareNext::Prefixes(self.generate_prefixes(trav))
    }
}
```

### Why Decomposition?

**Problem:** Tokens at different abstraction levels

Example:
```
Graph path:  [Token 6: "xab" (width=3)]
Query path:  [Token 0: "x", Token 1: "a", Token 2: "b"]
```

These represent the same sequence but at different granularities!

**Solution:** Decompose the composite token into its constituent sub-tokens:
```
Token 6 "xab" → [Token 2: "x", Token 0: "a", Token 1: "b"]
```

Now we can compare at the same level.

---

## Cursor State Transitions

### The Canonical Transition Flow

```rust
// 1. Start: Last confirmed match position
checkpoint: Matched { path: [a, b], position: 2 }
cursor:     Matched { path: [a, b], position: 2 }

// 2. Advance cursor to candidate
cursor: Candidate { path: [a, b, c], position: 3 }
checkpoint: Matched { path: [a, b], position: 2 }  // Unchanged!

// 3a. If match succeeds:
cursor: Matched { path: [a, b, c], position: 3 }
checkpoint: Matched { path: [a, b, c], position: 3 }  // Updated!

// 3b. If match fails:
cursor: Mismatched { path: [a, b, c], position: 3 }
// Revert to checkpoint:
cursor: Matched { path: [a, b], position: 2 }
checkpoint: Matched { path: [a, b], position: 2 }  // Unchanged
```

### Key State Transition Methods

#### On CompareState<Candidate>
```rust
impl CompareState<Candidate> {
    /// After successful comparison
    fn mark_match(self) -> CompareState<Matched> {
        CompareState {
            cursor: self.cursor.confirm_match(),
            checkpoint: self.checkpoint,  // Keep old checkpoint
            // ... rest unchanged
        }
    }
    
    /// After failed comparison  
    fn mark_mismatch(self) -> CompareState<Mismatched> {
        CompareState {
            cursor: self.cursor.mark_mismatch(),
            checkpoint: self.checkpoint,  // Keep checkpoint for revert
            // ... rest unchanged
        }
    }
}
```

#### On CompareState<Matched>
```rust
impl CompareState<Matched> {
    /// Prepare for next comparison
    fn into_next_candidate<G: HasGraph>(
        self, 
        trav: &G
    ) -> Result<CompareState<Candidate>, CompareState<Matched>> {
        // 1. Update checkpoint to current position
        let new_checkpoint = self.cursor.clone().into();
        
        // 2. Advance cursor
        match self.cursor.advance(trav) {
            Continue(_) => {
                Ok(CompareState {
                    cursor: self.cursor.as_candidate(),
                    checkpoint: new_checkpoint,  // Updated!
                    // ... rest unchanged
                })
            },
            Break(_) => {
                // Cannot advance - pattern exhausted
                Err(self)
            }
        }
    }
}
```

---

## Prefix Decomposition Strategy

### When to Decompose

The `mode` field determines which token to decompose:

```rust
enum PathPairMode {
    GraphMajor,  // Graph token is larger → decompose graph token
    QueryMajor,  // Query token is larger → decompose query token
}
```

**Decision logic in `next_match()`:**
```rust
match path_token.width().cmp(&query_token.width()) {
    Equal => {
        // Both same size → try decomposing both (try graph first)
        let prefixes = self.mode_prefixes(trav, GraphMajor)
            .chain(self.mode_prefixes(trav, QueryMajor));
        CompareNext::Prefixes(prefixes)
    },
    Greater => {
        // Graph token larger → decompose graph token
        CompareNext::Prefixes(self.mode_prefixes(trav, GraphMajor))
    },
    Less => {
        // Query token larger → decompose query token
        CompareNext::Prefixes(self.mode_prefixes(trav, QueryMajor))
    }
}
```

### How Decomposition Works

#### GraphMajor Mode (Decompose Graph Path)
```rust
fn prefix_states<G: HasGraph>(self, trav: &G) -> ChildQueue<...> {
    if self.mode == GraphMajor {
        // Get prefix sub-tokens of the graph token
        let prefixes = self.child_state.prefix_states(trav);
        
        // Create new candidate states for each prefix
        prefixes.map(|(sub_token, new_child_state)| {
            CompareState {
                child_state: new_child_state,  // Advanced path
                cursor: self.cursor.clone(),    // Same query position
                checkpoint: self.checkpoint.clone(),
                target: DownKey::new(sub_token, checkpoint_pos),
                mode: GraphMajor,
            }
        })
    }
}
```

**Example:**
```
Before: Compare Token 6 "xab" (width=3) vs Token 0 "x" (width=1)
After:  Compare Token 2 "x"   (width=1) vs Token 0 "x" (width=1) ✓
        Compare Token 0 "a"   (width=1) vs Token 0 "x" (width=1) ✗
        ...
```

#### QueryMajor Mode (Decompose Query Cursor)
```rust
fn prefix_states<G: HasGraph>(self, trav: &G) -> ChildQueue<...> {
    if self.mode == QueryMajor {
        // Get prefix sub-tokens of the query token
        let prefixes = self.cursor.prefix_states_from(trav, checkpoint_pos);
        
        // Create new candidate states for each prefix
        prefixes.map(|(sub_token, new_cursor)| {
            CompareState {
                child_state: self.child_state.clone(),  // Same graph position
                cursor: new_cursor,                      // Advanced cursor
                checkpoint: self.checkpoint.clone(),
                target: DownKey::new(sub_token, checkpoint_pos),
                mode: QueryMajor,
            }
        })
    }
}
```

### Prefix Generation - The PrefixStates Trait

**Trait definition:**
```rust
pub trait PrefixStates {
    fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> VecDeque<(SubToken, Self)>;
}
```

**Implementation for paths:**
```rust
impl<T: RootedLeafToken<End> + PathAppend> PrefixStates for T {
    fn prefix_states<G: HasGraph>(&self, trav: &G) -> VecDeque<...> {
        let leaf_token = self.leaf_token(trav);
        let prefix_children = trav.graph()
            .expect_vertex(leaf_token)
            .prefix_children::<G>();
        
        // For each prefix child, create new path
        prefix_children.map(|sub_token| {
            let mut new_path = self.clone();
            new_path.append(sub_token.location());
            (sub_token, new_path)
        })
    }
}
```

**Special handling for PathCursor (tracks atom_position):**
```rust
impl<P, S> PathCursor<P, S> {
    fn prefix_states_from<G: HasGraph>(
        &self,
        trav: &G,
        base_position: AtomPosition,  // Position to start from
    ) -> VecDeque<...> {
        let prefixes = self.path.prefix_states(trav);
        
        // Calculate atom_position for each prefix
        let mut accumulated_pos = base_position;
        prefixes.map(|(sub, mut cursor)| {
            cursor.atom_position = accumulated_pos;
            accumulated_pos += sub.width();
            (sub, cursor)
        })
    }
}
```

---

## Key Design Principles

### 1. Type-Safe State Transitions

**Principle:** Use phantom types to enforce correct state machine behavior

```rust
// ✓ Compile-time safe
CompareState<Candidate>.mark_match() -> CompareState<Matched>

// ✗ Compile error - can't mark non-candidate as matched
CompareState<Matched>.mark_match()  // Method doesn't exist!
```

### 2. Checkpoint Pattern

**Principle:** Always maintain a "last known good state" to revert to

```rust
CompareState {
    cursor: Candidate,      // Current trial
    checkpoint: Matched,    // Last confirmed position
}
```

**Why:** Enables backtracking without losing progress

### 3. Lazy Prefix Generation

**Principle:** Only decompose tokens when necessary

```rust
// First try direct comparison
if tokens_equal { return Match; }
if both_atoms { return Mismatch; }

// Only decompose if neither of above
return Prefixes(generate_prefixes());
```

**Why:** Performance - most comparisons don't need decomposition

### 4. Queue-Based Iteration

**Principle:** Use queues for breadth-first exploration of prefix possibilities

```rust
CompareIterator {
    queue: VecDeque<CompareState<Candidate>>
}

// Add prefixes to back of queue
queue.extend(prefix_states);

// Process from front
while let Some(candidate) = queue.pop_front() { ... }
```

**Why:** Ensures all prefix combinations are tried before giving up

### 5. Bidirectional Cache Building

**Principle:** Build TraceCache as search progresses for future use

```rust
// As we match tokens, record in cache
cache.add_bottom_up_entry(child_token, parent_token);
cache.add_top_down_entry(parent_token, child_token);

// Future searches can use this cached information
```

**Why:** Speeds up subsequent searches and enables insertion operations

### 6. Response Unification

**Principle:** All search outcomes return the same `Response` type

```rust
Response {
    cache: TraceCache,     // Always present
    end: EndState {        // Describes outcome
        reason: Complete | Mismatch | Exhausted | ...,
        cursor: PatternCursor,
        path: PathEnum,
    }
}
```

**Why:** 
- Consistent API regardless of search outcome
- Can always extract useful information (cache, cursor position)
- Enables resuming or transforming searches

---

## Common Patterns and Idioms

### Pattern 1: Advancing with State Transition

```rust
// Start with matched state
let matched: CompareState<Matched> = ...;

// Try to advance
match matched.into_next_candidate(trav) {
    Ok(candidate) => {
        // Successfully advanced - now compare
        compare(candidate)
    },
    Err(still_matched) => {
        // Cannot advance - pattern complete
        return Break(EndReason::Complete);
    }
}
```

### Pattern 2: Compare with Backtracking

```rust
let prev_state = current_state.clone();

match compare_current() {
    Match(matched) => {
        // Progress - keep going
        current_state = matched;
    },
    Mismatch(_) => {
        // Failed - revert to previous
        current_state = prev_state;
        return Break(EndReason::Mismatch);
    }
}
```

### Pattern 3: Iterator Chaining for Prefix Modes

```rust
// Try both decomposition modes when tokens equal width
let prefixes = self.mode_prefixes(trav, GraphMajor)
    .into_iter()
    .chain(self.mode_prefixes(trav, QueryMajor))
    .collect();

CompareNext::Prefixes(prefixes)
```

### Pattern 4: Checkpoint Update on Success

```rust
impl CompareState<Matched> {
    fn into_next_candidate(self, trav: &G) -> Result<...> {
        // Key: Convert current matched cursor to checkpoint
        let new_checkpoint: PatternCursor = self.cursor.clone().into();
        
        // Then advance for next comparison
        let advanced_cursor = self.cursor.advance(trav)?;
        
        Ok(CompareState {
            cursor: advanced_cursor.as_candidate(),
            checkpoint: new_checkpoint,  // Updated!
            ...
        })
    }
}
```

---

## Critical Implementation Details

### AtomPosition Tracking

**The Challenge:** AtomPosition must be correct for each state

**The Rule:**
- **Checkpoint.atom_position** = position where last token was matched
- **Cursor.atom_position** = position where current token should be found
- **After advance:** cursor.atom_position += token.width()

**Example:**
```
Pattern: [a(1), b(1), c(1)]

State 0: checkpoint.pos=0, cursor.pos=0  (before matching 'a')
Match 'a' → State 1: checkpoint.pos=1, cursor.pos=1  (after matching 'a')
Match 'b' → State 2: checkpoint.pos=2, cursor.pos=2  (after matching 'b')
Match 'c' → State 3: checkpoint.pos=3, cursor.pos=3  (complete)
```

### Prefix Position Calculation

**For QueryMajor mode:**
```rust
fn prefix_states_from(&self, trav: &G, base_position: AtomPosition) {
    let mut position = base_position;
    
    prefixes.map(|(sub, cursor)| {
        cursor.atom_position = position;  // Position for THIS prefix
        position += sub.width();          // Advance for next prefix
        (sub, cursor)
    })
}
```

**Example:**
```
Token "abc" (width=3) at position 5:
  Prefix "a" (width=1): position = 5
  Prefix "b" (width=1): position = 6
  Prefix "c" (width=1): position = 7
```

### Mode Switching

When decomposing equal-width tokens, try GraphMajor first:

**Rationale:** Graph structure is typically more stable than query order

```rust
match width_comparison {
    Equal => {
        // Try graph decomposition first
        graph_prefixes.chain(query_prefixes)
    },
    Greater => graph_prefixes,
    Less => query_prefixes,
}
```

---

## Testing Strategy

### Unit Test Levels

1. **State Transitions** - Test each state transition method in isolation
2. **Token Comparison** - Test next_match() with various token combinations
3. **Prefix Generation** - Test prefix_states() produces correct sequences
4. **Iterator Layers** - Test each iterator independently
5. **Integration** - Test full search flows end-to-end

### Key Test Scenarios

#### Simple Exact Match
```rust
Graph: [a, b, c]
Query: [a, b, c]
Expected: Complete match
```

#### Hierarchical Match
```rust
Graph: [abc]  (composite token width=3)
Query: [a, b, c]  (three atoms)
Expected: Complete match via prefix decomposition
```

#### Partial Match
```rust
Graph: [a, b]
Query: [a, b, c]
Expected: Incomplete - c not found
```

#### Width Mismatch
```rust
Graph: [ab]  (width=2)
Query: [a]   (width=1)
Expected: Match via prefix decomposition
```

---

## Future Enhancements

### Optimization Opportunities

1. **Shared Path Storage** - Checkpoint and cursor share base path
2. **Lazy Checkpoint Updates** - Only update when needed
3. **Prefix Caching** - Cache prefix decompositions
4. **Early Termination** - Stop as soon as mismatch certain

### Architectural Improvements

1. **Separate Iteration Traits** - Make each layer independently testable
2. **Mode Strategies** - Use strategy pattern for GraphMajor/QueryMajor
3. **Position Automation** - Automatically track positions via trait
4. **Result Builders** - Fluent API for constructing responses

---

## Summary

The context-search algorithm implements a **three-layer iterator architecture** with **type-safe state machines** for pattern matching in hierarchical hypergraphs:

1. **RootSearchIterator** - Tries different parent patterns
2. **RootCursor** - Matches token sequences within a pattern
3. **CompareIterator** - Handles token comparison with prefix decomposition

Key innovations:
- **Checkpoint pattern** for safe backtracking
- **Type-level states** for compile-time correctness
- **Lazy prefix decomposition** for performance
- **Unified Response** for consistent API

The algorithm gracefully handles:
- Exact matches at same abstraction level
- Hierarchical matches via prefix decomposition
- Partial matches with detailed failure information
- Complex graph structures with bidirectional caching
