# Search Flow Explanation & Test Summary

## Search Flow with New Naming

### High-Level Architecture

```
User Query → Searchable::search() → SearchState → SearchIterator → Response
                                         ↓
                                    RootFinder → NodeConsumer → SearchNode
                                         ↓
                                    RootCursor (type-state iterator)
                                         ↓
                                    CompareState<Q, I>
```

### Component Responsibilities

#### 1. **SearchIterator** - Main Orchestrator
- **Purpose**: Manages the overall search process
- **Structure**:
  ```rust
  struct SearchIterator<K> {
      trace_ctx: TraceCtx<K::Trav>,  // Graph + cache
      match_ctx: SearchQueue,         // Work queue (VecDeque<SearchNode>)
      last_complete_match: Option<EndState>,  // Best match so far
  }
  ```
- **Flow**:
  1. Calls `RootFinder` to find initial match from queue
  2. Processes match through `RootCursor` iteration
  3. Handles parent exploration when index ends but query continues
  4. Caches results for reuse

#### 2. **RootFinder** - Initial Match Finder
- **Purpose**: Finds first match from the search queue
- **Structure**: Iterator that processes `SearchQueue` nodes
- **Flow**:
  1. Pops nodes from `SearchQueue`
  2. Passes each to `NodeConsumer::consume()`
  3. Returns first `FoundMatch` as `MatchedCompareState`
  4. Wraps result in `RootCursor<Matched, Matched>`

#### 3. **SearchNode** - Work Items
- **Purpose**: Represents units of work in the search queue
- **Variants**:
  - `ParentCandidate(ParentCompareState)`: Parent token to explore
  - `PrefixQueue(ChildQueue)`: Queue of child prefixes to compare

#### 4. **NodeConsumer** - Node Processor
- **Purpose**: Processes `SearchNode` items and generates results
- **Structure**: Tuple struct `NodeConsumer(SearchNode, &Traversal)`
- **Returns**: `Option<NodeResult>`
  - `Some(FoundMatch(..))`: Match found
  - `Some(Skip)`: Mismatch, try next
  - `Some(QueueMore(..))`: Add more nodes to explore
  - `None`: Node exhausted

#### 5. **NodeResult** - Processing Outcomes
- **Purpose**: Represents result of processing a `SearchNode`
- **Variants**:
  - `FoundMatch(MatchedCompareState)`: Successfully found a match
  - `Skip`: Mismatch occurred, continue to next node
  - `QueueMore(Vec<SearchNode>)`: Generated more nodes to process

#### 6. **SearchQueue** - Work Queue
- **Purpose**: FIFO queue for breadth-first search
- **Structure**: `struct SearchQueue { nodes: VecDeque<SearchNode> }`
- **Operations**: `push_back()` to add, `pop_front()` to get next

#### 7. **RootCursor** - Type-State Iterator
- **Purpose**: Iterates through matched positions with compile-time safety
- **Type States**: `RootCursor<G, Q, I>` where:
  - `G`: Graph type
  - `Q`: Query cursor state (`Matched` / `Candidate`)
  - `I`: Index cursor state (`Matched` / `Candidate`)

**Three Valid States**:
1. `RootCursor<G, Matched, Matched>`: Both cursors on matched positions (initial state)
2. `RootCursor<G, Candidate, Matched>`: Query advanced, index at match (index ended)
3. `RootCursor<G, Candidate, Candidate>`: Both at candidates (iterating)

**State Transitions**:
```
Matched, Matched
    │ advance_query_cursor()
    ├─ Ok ────────────────────────────────┐
    │                                     ▼
    │                         Candidate, Matched
    │                                     │ advance_index_cursor()
    │                                     ├─ Ok ──────┐
    │                                     │           ▼
    │                                     │   Candidate, Candidate
    │                                     │           │ Iterator::next()
    │                                     │           └─ Continue/Break
    │                                     │
    │                                     └─ Err → Index ended (parent exploration)
    │
    └─ Err → Query ended (complete match)
```

#### 8. **CompareState<Q, I>** - Cursor State Management
- **Purpose**: Manages query and index cursor positions independently
- **Type Parameters**:
  - `Q`: Query cursor state (`Matched` / `Candidate`)
  - `I`: Index cursor state (`Matched` / `Candidate`)
- **Key Innovation**: Separate advancement of query and index cursors
- **Methods**:
  - `advance_query_cursor()`: Advances Q from Matched to Candidate
  - `advance_index_cursor()`: Advances I from Matched to Candidate
- **Why**: Enables handling "index ends, query continues" case for parent exploration

#### 9. **Response** - Unified Result API
- **Purpose**: Provides consistent API for search results
- **Methods**:
  - `is_complete()`: Check if search succeeded
  - `expect_complete(msg)`: Unwrap complete match or panic
  - `root_token()`: Get matched token
  - `query_cursor()`: Get query cursor position
  - `query_pattern()`: Get query pattern

---

## Complete Flow Example

### Scenario: Search for "xab" pattern

**Graph**:
```
Token: "x" (atom)
Token: "a" (atom)
Token: "b" (atom)
Token: "xab" (pattern: [x, a, b])
```

**Execution**:

1. **Entry**: `graph.find_ancestor(&query)` where query = "xab"

2. **SearchState Creation**:
   - Creates cursor for query pattern
   - Gets parent batch (tokens containing 'x')
   - Initializes `SearchIterator` with queue

3. **SearchIterator Loop**:
   ```
   SearchIterator::next()
   └─> RootFinder::find_root_cursor()
       └─> RootFinder::next() [Iterator]
           └─> Pop ParentCandidate(xab) from queue
           └─> NodeConsumer::consume()
               └─> Process ParentCandidate
               └─> Run comparison: "xab" vs "xab"
               └─> Return FoundMatch(matched_state)
       └─> Wrap in RootCursor<Matched, Matched>
   ```

4. **RootCursor Processing**:
   ```
   RootCursor<Matched, Matched>::find_end()
   └─> advance_query_cursor()
       └─> Fails (query ended at position 3)
   └─> Return Ok(EndState { reason: QueryEnd, ... })
   ```

5. **Cache Entry**: Store match for token "xab" at position 3

6. **Response**: Return `Response { end: EndState, ... }`

---

## Parent Exploration Flow

### Scenario: Query "xabyz", Graph path "xab"

**What happens**:

1. **Initial Match**: Find "xab" token, match x→a→b ✓

2. **Index Ends**:
   ```
   RootCursor<Matched, Matched>::find_end()
   └─> advance_query_cursor() → Ok (query has 'y', 'z' left)
   └─> advance_index_cursor() → Err (index ended after 'b')
   └─> Return Err(RootCursor<Candidate, Matched>)
   ```

3. **Parent Exploration Triggered**:
   ```
   SearchIterator::next()
   └─> Got Err(cursor) from find_end()
   └─> Call cursor.next_parents()
       └─> Get parents of "xab" → ["xabyz"]
   └─> Save current match as last_complete_match
   └─> Queue ParentCandidate("xabyz") nodes
   └─> Recursively call self.next()
   ```

4. **Process Parent**:
   - RootFinder processes "xabyz" node
   - Matches x→a→b→y→z ✓
   - Returns complete match

5. **Result**: Found "xabyz" through parent exploration

---

## Unit Test Structure

### Created Test Files

#### 1. `context-search/src/tests/search/core_types.rs`
**Purpose**: Document core internal types and concepts

**Test Modules**:
- `search_node_concepts`: SearchNode variants and usage
- `node_result_concepts`: NodeResult variants and meaning
- `search_queue_concepts`: Queue management
- `root_cursor_type_states`: Type-state pattern documentation
- `compare_state_type_params`: CompareState<Q, I> innovation
- `public_api_tests`: Tests using only public APIs
- `test_utilities`: Documentation of test patterns

**Test Results**: ✅ 9 tests passed

#### 2. `context-search/src/tests/search/iterator.rs`
**Purpose**: Document search orchestration components

**Test Modules**:
- `search_iterator_concepts`: SearchIterator flow and responsibilities
- `root_finder_concepts`: RootFinder iterator behavior
- `node_consumer_concepts`: Node processing logic
- `root_cursor_iteration`: RootCursor iteration behavior
- `response_api_concepts`: Response API usage

**Test Results**: ✅ 11 tests passed

### Test Philosophy

These tests are **documentation tests** that:
- Explain the architecture and design
- Document type relationships and state transitions
- Provide conceptual understanding
- Avoid implementation details (many types are internal)
- Focus on the "why" and "how" of the design

### Running Tests

```bash
# Run all new tests
cargo test -p context-search --lib core_types
cargo test -p context-search --lib iterator

# Combined
cargo test -p context-search --lib core_types iterator

# All search tests
cargo test -p context-search --lib search
```

---

## Key Insights

### 1. Type-State Pattern
`RootCursor<G, Q, I>` uses Rust's type system to prevent invalid state transitions at compile time. The three type parameters ensure that cursor advancement is tracked in the type signature.

### 2. Independent Cursor Advancement
`CompareState<Q, I>` separates query and index cursor states, enabling the critical "index ends, query continues" case that triggers parent exploration.

### 3. Breadth-First Search
`SearchQueue` (VecDeque) ensures breadth-first traversal of the graph, processing all nodes at one level before moving to the next.

### 4. Iterator-Based Design
Both `RootFinder` and `RootCursor<Candidate, Candidate>` implement `Iterator`, allowing use of standard Rust iterator combinators and ensuring idiomatic code.

### 5. Unified Response API
`Response` provides a consistent interface for search results, hiding internal complexity and providing safe accessor methods.

---

## Next Steps

For full integration testing with real graph data, see:
- `context-search/src/tests/search/ancestor.rs` - Ancestor finding tests
- `context-search/src/tests/search/parent.rs` - Parent exploration tests
- `context-search/src/tests/search/consecutive.rs` - Consecutive pattern tests

These files contain tests with actual graph construction and search execution.
