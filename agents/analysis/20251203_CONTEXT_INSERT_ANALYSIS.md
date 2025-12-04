# Context-Insert Algorithm Analysis

> **Deep analysis of the insertion algorithm, dependencies, and implementation details**

## Executive Summary

**Purpose:** Context-insert enables safe insertion of new patterns into the hypergraph by splitting existing structures at query boundaries, merging components, and creating new tokens that reach all valid substrings.

**Key Innovation:** Split-join architecture allows adding new patterns without modifying existing ones, preserving all references and maintaining graph invariants.

**Algorithm Phases:**
1. **Search** - Find partial match in existing graph
2. **Initialize** - Convert search result to InitInterval
3. **Split** - Break patterns at boundary positions
4. **Join** - Merge split components with new content
5. **Result** - Extract and return new token

**Performance:** O(d * p + k + m) where:
- d = pattern depth
- p = parents per level  
- k = split points
- m = merge operations

---

## Algorithm Flow

### 1. Initialization from Search

When a search doesn't exhaust the query, create an InitInterval:

```rust
pub struct InitInterval {
    pub root: Token,           // Partially matched token
    pub cache: TraceCache,     // Search trace data
    pub end_bound: AtomPosition, // Where to extend from
}

impl From<Response> for InitInterval {
    fn from(state: Response) -> Self {
        Self {
            root: state.root_token(),
            cache: state.cache,
            end_bound: state.cursor_position(),
        }
    }
}
```

**What this means:**
- `root`: The token that matched as far as possible
- `cache`: Reusable trace information from search
- `end_bound`: Position in query where search stopped

**Example:**
```
Graph has: "abcdef"
Query: [a, b, c, d, e, f, g]
Search matches up to "abcdef" at position 6
InitInterval:
  root = Token("abcdef")
  cache = TraceCache with search data
  end_bound = AtomPosition(6)
```

### 2. IntervalGraph Creation

Transform InitInterval into processing structure:

```rust
impl From<(&mut G, InitInterval)> for IntervalGraph {
    fn from((trav, init): (&mut G, InitInterval)) -> Self {
        let InitInterval { root, cache, end_bound, .. } = init;
        let ctx = TraceCtx { trav, cache };
        let iter = SplitTraceStatesCtx::new(ctx, root, end_bound);
        Self::from(SplitCacheCtx::init(iter))
    }
}
```

**Components:**
- **TraceCtx**: Combines graph traversal and cache
- **SplitTraceStatesCtx**: Manages split state iteration
- **SplitCacheCtx**: Initializes split caching
- **IntervalGraph**: Final processing structure

### 3. Split Phase

**Goal:** Break patterns at insertion boundaries without modifying originals

**Key Structure:**
```rust
pub struct SplitCache {
    pub root_mode: RootMode,  // Prefix/Postfix/Infix
    pub entries: HashMap<VertexIndex, SplitVertexCache>,
}

pub struct SplitVertexCache {
    pub positions: BTreeMap<NonZeroUsize, SplitPositionCache>,
    pub complete: Option<CompleteLocations>,
}

pub struct SplitPositionCache {
    pub parent: BTreeMap<PatternId, SubSplitLocation>,
}
```

**Split Algorithm:**

1. **Identify Split Points**
   - Start from InitInterval.end_bound
   - Find all patterns containing this position
   - Mark split positions in each pattern

2. **Create Split States**
   ```rust
   pub struct SplitStates {
       pub leaves: BTreeSet<PosKey>,    // Leaf positions to split
       pub queue: VecDeque<...>,        // Processing queue
   }
   ```

3. **Build Split Cache**
   - For each split position:
     - Record parent patterns
     - Track offset within pattern
     - Store location information

**Example:**
```
Pattern: abcdef = [ab, cdef]
Split at position 4 (within cdef):
  
Split Cache:
  cdef -> position 1 -> {
    parent: abcdef at entry 1,
    offset: 2 (c and d before split)
  }

Result: Can now access [c, d] and [e, f] separately
```

### 4. Split Tracing

**Purpose:** Track which patterns need splitting as algorithm progresses

**Key Types:**
```rust
pub trait TraceSide {}
pub struct TraceFront;  // Forward tracing
pub struct TraceBack;   // Backward tracing

pub struct PosKey {
    pub token: Token,
    pub position: usize,
}
```

**Tracing Process:**
1. Start from end_bound position
2. Trace through parent patterns
3. Identify all split points
4. Cache split information

**Why Both Directions?**
- **TraceFront**: Follow pattern children forward
- **TraceBack**: Follow pattern parents backward
- Together: Build complete split map

### 5. Join Phase

**Goal:** Merge split components with new content to create new pattern

**Key Structure:**
```rust
pub struct JoinContext {
    frontier: Frontier,      // Active join boundary
    nodes: Vec<JoinNode>,    // Nodes to merge
}

pub struct JoinedPartitions {
    patterns: Vec<Pattern>,  // Resulting patterns
    info: PartitionInfo,     // Metadata
}
```

**Join Algorithm:**

1. **Collect Split Components**
   - Gather all pieces from split cache
   - Include new content to insert
   - Order by position

2. **Merge Overlapping Pieces**
   ```rust
   // If pieces overlap, merge them
   [a, b] + [b, c] -> [a, b, c]
   ```

3. **Create New Patterns**
   - Build pattern from merged components
   - Register in graph
   - Update parent-child relationships

4. **Handle Different Roles**
   - **Pre**: Content before insertion
   - **In**: Insertion content itself  
   - **Post**: Content after insertion

**Example:**
```
Split pieces: [a, b], [c, d]
New content: [e, f]
Join:
  Pre: [a, b]
  In: [c, d, e, f]
  Post: []
Result: abcdef = [ab, cdef] where cdef = [c, d, e, f]
```

### 6. Range Roles System

**Purpose:** Different insertion scenarios need different boundary handling

**Role Types:**
```rust
pub trait RangeRole {
    type Mode;  // Pre, In, Post
}

pub struct Pre;     // Before insertion
pub struct In;      // At insertion
pub struct Post;    // After insertion

// Wrapper roles
pub struct BooleanPerfectOf<R>;  // Perfect boundary
pub struct OffsetsOf<R>;         // With offset info
```

**Why Multiple Roles?**

Different insertion positions require different logic:

| Role | Use Case | Boundaries Needed |
|------|----------|-------------------|
| Pre | Prefix insertion | Start of pattern |
| Post | Postfix insertion | End of pattern |
| In | Infix insertion | Both start and end |
| BooleanPerfectOf | Perfect boundary check | Clean splits |
| OffsetsOf | Partial splits | Offset calculations |

**Example:**
```
Pattern: abcdef = [ab, cd, ef]

Insert "xy" at position 2 (after "ab"):
  Pre: ab (completed)
  In: xy (inserting)
  Post: cdef (remaining)
```

---

## Dependency Analysis

### External Crate Dependencies

```toml
[dependencies]
# Macro helpers
derive-new = "0.7.0"          # Constructor macros
derive_more = "2"             # Additional derives
derivative = "2.2"            # Custom derives

# Collections
itertools = "0.14"            # Iterator utilities
linked-hash-map = "0.5"       # Ordered maps
linked_hash_set = "0.1"       # Ordered sets
maplit = "1.0"                # Collection literals

# Testing
pretty_assertions = "1.4"     # Better test output

# Logging
tracing = "^0.1"              # Structured logging
tracing-subscriber = "0.3"    # Log subscription
```

**Dependency Rationale:**

| Crate | Usage | Can Remove? |
|-------|-------|-------------|
| derive-new | `#[derive(new)]` constructors | No - pervasive |
| derive_more | Additional trait derives | Yes - manual impl |
| derivative | Custom derives | Yes - manual impl |
| itertools | `Itertools::sorted()`, grouping | Partial - some uses essential |
| linked-hash-map | Ordered iteration | Yes - use BTreeMap |
| linked_hash_set | Ordered sets | Yes - use BTreeSet |
| maplit | Test readability | Yes - verbose init |
| pretty_assertions | Test debugging | Yes - convenience only |
| tracing | Debugging/logging | No - critical for debugging |

**Reduction Recommendations:**

1. **Keep:**
   - derive-new (too pervasive)
   - tracing (essential for debugging)
   - itertools (key algorithms)

2. **Consider Removing:**
   - linked-hash-map/set → BTreeMap/Set
   - maplit → manual initialization
   - pretty_assertions → std assertions
   - derivative → manual derives

3. **Maybe Remove:**
   - derive_more → manual impls
   - Some itertools uses → manual loops

### Internal Crate Dependencies

```toml
context-search = { path = "../context-search", features = ["test-api"] }
context-trace = { path = "../context-trace", features = ["test-api"] }
```

**From context-trace:**
- Graph structures (Hypergraph, Token, Pattern)
- Path types (IndexRangePath, PatternRangePath)
- Position types (AtomPosition, DownPosition)
- Cache types (TraceCache, VertexCache)
- Parent/child relationships

**From context-search:**
- Response type (search results)
- Searchable trait (find_ancestor, etc.)
- Error types (ErrorReason)
- Foldable patterns

**Dependency Direction:**
```
context-trace (foundation)
    ↓
context-search (query)
    ↓
context-insert (modify)
```

**Interface Points:**

1. **Response → InitInterval**
   ```rust
   impl From<Response> for InitInterval
   ```

2. **TraceCache Reuse**
   - Search cache passed to insertion
   - Avoids redundant tracing

3. **Token Creation**
   - Uses graph insertion APIs
   - Maintains graph invariants

---

## Testing Patterns

### Test Structure

```rust
#[test]
fn test_name() {
    // 1. Setup graph
    let mut graph = Hypergraph::default();
    insert_atoms!(graph, {a, b, c, d});
    insert_patterns!(graph,
        ab => [[a, b]],
        cd => [[c, d]]
    );
    
    // 2. Convert to ref for insertion
    let graph_ref = HypergraphRef::from(graph);
    
    // 3. Perform insertion
    let query = vec![a, b, c, d];
    let abcd: Token = graph_ref.insert(query.clone())
        .expect("Indexing failed");
    
    // 4. Verify result
    assert_eq!(abcd.width(), 4);
    
    // 5. Verify searchable
    let found = graph_ref.find_ancestor(&query);
    assert_matches!(found,
        Ok(ref r) if r.query_exhausted() && r.is_full_token()
    );
}
```

### Key Test Helpers

**Graph Construction:**
```rust
insert_atoms!(graph, {a, b, c});
insert_patterns!(graph,
    ab => [[a, b]],           // Single pattern
    abc => [[ab, c], [a, b, c]] // Multiple patterns
);
```

**Split Cache Building:**
```rust
build_split_cache!(
    RootMode::Prefix,
    token => {
        { parent1: pos1, parent2: pos2 } -> split_pos => {
            pattern_id => (entry, offset)
        }
    }
)
```

**Assertions:**
```rust
// Check insertion succeeded
assert_eq!(result.width(), expected_width);

// Check searchable afterward
let found = graph.find_ancestor(&query)?;
assert!(found.query_exhausted() && found.is_full_token());

// Check pattern structure
assert_eq!(
    vertex.child_pattern_set(),
    expected_patterns
);
```

### Test Categories

**1. Basic Insertion (insert.rs)**
- Single pattern insertion
- Multi-pattern insertion
- Overlapping pattern insertion

**2. Interval Construction (interval.rs)**
- InitInterval from Response
- IntervalGraph creation
- Split cache building
- Split state management

**3. Edge Cases**
- Empty patterns
- Single atom patterns
- Fully nested patterns
- Overlapping boundaries

---

## Performance Characteristics

### Time Complexity

| Phase | Complexity | Explanation |
|-------|------------|-------------|
| Search | O(d * p) | d=depth, p=parents per level |
| InitInterval | O(1) | Simple conversion |
| Split Trace | O(k * p) | k=split points, p=parents |
| Split Cache | O(k) | Store split information |
| Join | O(m * c) | m=merges, c=components |
| **Total** | **O(d*p + k*p + m*c)** | Dominated by search+split |

### Space Complexity

| Structure | Size | Explanation |
|-----------|------|-------------|
| InitInterval | O(cache) | Reuses search cache |
| SplitCache | O(k * p) | Split points × parents |
| SplitStates | O(k + q) | Leaves + queue |
| Join Nodes | O(c) | Components to merge |
| **Total** | **O(cache + k*p)** | Dominated by split cache |

### Optimization Opportunities

**1. Cache Reuse**
```rust
// ✅ Good - reuse search cache
let init = InitInterval::from(search_response);
// Cache already populated

// ❌ Bad - rebuild cache
let init = InitInterval { cache: TraceCache::new(), .. };
// Must rebuild from scratch
```

**2. Lazy Splitting**
```rust
// Only split patterns that need it
// Don't split entire parent hierarchy
```

**3. Split Cache Sharing**
```rust
// Multiple insertions at same position
// Could share split cache
```

**4. Join Batching**
```rust
// Batch multiple joins
// Single graph update
```

---

## Design Insights

### Why Split-Join?

**Alternative 1: In-Place Modification**
```rust
// ❌ Breaks existing references
pattern.children.push(new_child);
```

**Alternative 2: Copy-On-Write**
```rust
// ❌ Expensive, duplicates entire patterns
let new_pattern = pattern.clone();
new_pattern.modify();
```

**Split-Join Approach:**
```rust
// ✅ Create new without modifying old
Split: pattern -> [pieces]
Join: [pieces] + new -> new_pattern
// Both old and new exist
```

### Why Interval Management?

**Problem:** Insertion is complex multi-phase process

**Solution:** IntervalGraph tracks state:
```rust
pub struct IntervalGraph {
    pub root: Token,         // Where we're inserting
    pub states: SplitStates, // What we're splitting
    pub cache: SplitCache,   // How to split it
}
```

Enables:
- Incremental processing
- Error recovery
- Debugging/inspection
- Resumable operations

### Why Multiple Roles?

**Problem:** Different insertion positions need different logic

**Example:**
```
Prefix:  [new] + [existing]  -> need end boundary of 'new'
Postfix: [existing] + [new]  -> need start boundary of 'new'
Infix:   [before] + [new] + [after]  -> need both boundaries
```

**Solution:** Role system provides correct boundary info per scenario

---

## Known Issues & Questions

### 1. end_bound Semantics

**Current:**
```rust
let end_bound = response.cursor_position();
```

**Question:** Is this correct? Should it be `cursor_position() + 1`?

**Impact:** Affects where insertion starts

### 2. PathCoverage Handling

When Response has different PathCoverage variants:
- EntireRoot (full token)
- Range (intersection)
- Prefix (partial match from start)
- Postfix (partial match at end)

**Question:** How should InitInterval handle each case?

### 3. Multiple Pattern Representations

Token can have multiple child patterns:
```rust
abc => [[a, b, c], [ab, c], [a, bc]]
```

**Question:** Which pattern(s) to use when splitting?

### 4. Dependency Bloat

Many dependencies for relatively simple operations:
- linked-hash-map/set for ordering
- maplit for test readability
- pretty_assertions for test output

**Question:** Can we reduce external dependencies?

---

## Algorithm Specification

### Inputs

```rust
fn insert<P: Into<FoldablePattern>>(
    graph: &mut Hypergraph,
    pattern: P
) -> Result<Token, InsertError>
```

**Parameters:**
- `graph`: Mutable graph reference
- `pattern`: Pattern to insert (token sequence)

**Preconditions:**
- Graph is valid hypergraph
- Pattern atoms exist in graph
- Pattern is non-empty

### Outputs

**Success:**
```rust
Token  // Represents inserted pattern
```

**Properties of returned token:**
- Width equals pattern length
- Searchable via find_ancestor
- Has child pattern(s) representing structure
- Reachable from all constituent atoms

**Failure:**
```rust
InsertError  // Describes what went wrong
```

### Algorithm Guarantees

**1. Idempotency**
```rust
// Inserting same pattern twice returns same token
let t1 = graph.insert(pattern)?;
let t2 = graph.insert(pattern)?;
assert_eq!(t1, t2);
```

**2. Searchability**
```rust
// After insertion, pattern is searchable
let token = graph.insert(pattern)?;
let found = graph.find_ancestor(&pattern)?;
assert_eq!(found.root_token(), token);
```

**3. Substring Reachability**
```rust
// Token reaches all valid substrings
let abcd = graph.insert([a, b, c, d])?;
// Can reach: a, b, c, d, ab, bc, cd, abc, bcd
```

**4. Reference Preservation**
```rust
// Existing patterns unchanged
let existing = graph.get_token(some_id)?;
graph.insert(new_pattern)?;
let still_exists = graph.get_token(some_id)?;
assert_eq!(existing, still_exists);
```

### Postconditions

After successful insertion:

1. **Token exists in graph**
   ```rust
   assert!(graph.contains(result_token));
   ```

2. **Has valid child pattern(s)**
   ```rust
   let vertex = graph.vertex(result_token);
   assert!(!vertex.child_patterns().is_empty());
   ```

3. **Searchable with original query**
   ```rust
   let found = graph.find_ancestor(&original_query)?;
   assert_eq!(found.root_token(), result_token);
   ```

4. **Preserves graph invariants**
   - All parent-child relationships valid
   - No dangling references
   - Width calculations correct

---

## Future Enhancements

### 1. Batch Insertion

Support inserting multiple patterns efficiently:
```rust
fn insert_batch(&mut self, patterns: &[Pattern]) -> Result<Vec<Token>>
```

Benefits:
- Share split cache across insertions
- Batch graph updates
- Better performance

### 2. Incremental Splitting

Only split what's absolutely necessary:
```rust
// Current: Split entire parent hierarchy
// Future: Lazy split only when accessed
```

### 3. Split Cache Persistence

Cache split information across operations:
```rust
struct PersistentSplitCache {
    cache: HashMap<Token, SplitVertexCache>,
    // Reuse across multiple insertions
}
```

### 4. Parallel Join

Join independent components in parallel:
```rust
// If join nodes don't overlap
// Process simultaneously
```

### 5. Dependency Reduction

Minimize external dependencies:
- Replace linked-hash-map with BTreeMap
- Remove maplit (use explicit construction)
- Make pretty_assertions optional

---

## Comparison with Other Approaches

### Traditional Trie Insertion

**Trie Approach:**
```
Walk down tree, create node if missing
```

**Context-Insert Approach:**
```
Search for match, split at boundaries, join components
```

**Differences:**
- Trie: Simple top-down walk
- Context-Insert: Complex split-join with caching
- Trie: Direct modification
- Context-Insert: Immutable existing structures

**Why More Complex?**
- Hypergraph not tree (multiple parents)
- Need to preserve all existing references
- Support overlapping pattern representations
- Enable incremental construction

### Immutable Data Structure Insertion

**Persistent Data Structure:**
```
Copy path from root, modify copy, return new root
```

**Context-Insert:**
```
Split existing, add new alongside, join together
```

**Similarities:**
- Both preserve originals
- Both create new structures

**Differences:**
- Persistent: Copy entire path
- Context-Insert: Split only boundaries
- Persistent: Return new root
- Context-Insert: Add to existing graph

---

## Conclusion

Context-insert provides sophisticated pattern insertion through split-join architecture. Key insights:

1. **Safe Modification**: Create new without breaking existing
2. **Reuse Search Data**: Leverage trace cache from search
3. **Boundary Splitting**: Split only at insertion boundaries
4. **Component Merging**: Join split pieces with new content
5. **Multiple Representations**: Support various pattern structures

The algorithm balances complexity with correctness, ensuring graph invariants while enabling efficient pattern construction.

**Performance:** Reasonable for hierarchical graph construction (dominated by search phase)

**Correctness:** Maintains all graph invariants, preserves references

**Flexibility:** Supports various insertion modes (prefix, postfix, infix)

**Dependencies:** Could be reduced but currently acceptable

For practical usage, see HIGH_LEVEL_GUIDE.md. For implementation details, see source code. For questions and unclear behavior, see QUESTIONS_FOR_AUTHOR.md.
