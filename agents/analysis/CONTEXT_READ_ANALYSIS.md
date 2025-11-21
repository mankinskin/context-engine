# Context-Read Analysis

> **Analysis of context-read layer - the high-level graph reading and expansion system**

## Executive Summary

**Current State:** Partially implemented - core expansion infrastructure in place, reading operations under development

**Purpose:** Provide high-level operations for reading and expanding patterns in the hypergraph through recursive decomposition and complement operations.

**Key Innovation:** **Block iteration** + **expansion chains** + **complement operations** enable efficient context-aware reading of hierarchical patterns.

**Architecture:** Ordered recursive hypergraph processing with expansion cursors, band chains, and complement calculations.

---

## What Context-Read Will Do

Based on the implementation and structure, context-read is designed to:

### 1. Read Sequences as Hierarchical Patterns

**Goal:** Take a sequence of atoms (characters, tokens) and build/discover hierarchical patterns efficiently.

**Process:**
```
Input: [h, e, l, l, o, w, o, r, l, d]
       ↓ Classify atoms (check/insert as we go, initially empty graph)
Classified: [h(new), e(new), l(new), l(known), o(new), w(new), o(known), r(new), l(known), d(new)]
            ^insert  ^insert ^insert  ^exists   ^insert ^insert ^exists   ^insert ^exists   ^insert
       ↓ Block Iteration (group consecutive new, then consecutive known)
Blocks:
  Block 1: unknown:[h,e,l], known:[l]       (h,e,l inserted; second l found)
  Block 2: unknown:[o,w], known:[o]         (o,w inserted; second o found)
  Block 3: unknown:[r], known:[l]           (r inserted; third l found)
  Block 4: unknown:[d], known:[]            (d inserted; no known after)
       ↓ Process each block
  - Append unknown atoms directly (already inserted during classification)
  - Expand known patterns (find larger patterns containing them)
       ↓ Read Context
Output: Token representing "helloworld" with optimal decomposition
```

**Use Cases:**
- Building patterns from raw input
- Discovering efficient decompositions
- Context expansion for reading comprehension

### 2. Expand Patterns Incrementally

**Goal:** Given a partial pattern, expand it to find the largest enclosing context.

**Process:**
```
Start: Pattern "bc" in context "abcdef"
       ↓ Expansion Cursor
Find: Prefix "a", Postfix "def"
       ↓ Band Chain
Build: Complete context "abcdef"
       ↓ Complement
Output: Optimal representation
```

**Use Cases:**
- Context expansion
- Pattern completion
- Hierarchical discovery

### 3. Handle Multiple Representations

**Goal:** Work with tokens that have multiple child patterns.

**Process:**
```
Token "abc" has:
  - Pattern [a, b, c]
  - Pattern [ab, c]
  - Pattern [a, bc]
       ↓ Expansion
Find largest bundle across all representations
       ↓ Selection
Choose optimal decomposition for current context
```

**Use Cases:**
- Efficient pattern selection
- Context-aware decomposition
- Multi-representation management

### 4. Calculate Complements

**Goal:** Find the "missing piece" when expanding patterns.

**Process:**
```
Root: "abcdef"
Expansion: "def" (at position 3)
       ↓ Complement
Calculate: "abc" (positions 0-3)
       ↓ Insert
Create: Connection between complement and expansion
```

**Use Cases:**
- Bridge building between patterns
- Gap filling
- Complete context construction

---

## Core Concepts

### ReadCtx - Reading Context

**Purpose:** Main orchestrator for reading sequences.

```rust
pub struct ReadCtx {
    pub root: RootManager,      // Manages root token
    pub blocks: BlockIter,       // Iterates over known/unknown blocks
}

impl Iterator for ReadCtx {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.next().map(|block| self.read_block(block))
    }
}
```

**What it does:**
1. Takes a sequence of atoms
2. Splits into known/unknown blocks
3. Processes each block
4. Builds up hierarchical representation

**Example:**
```rust
let ctx = ReadCtx::new(graph, "helloworld".chars());
let token = ctx.read_sequence();  // Returns token for "helloworld"
```

### BlockIter - Block Iteration

**Purpose:** Split input into alternating known/unknown blocks.

```rust
pub struct BlockIter {
    iter: Peekable<IntoIter<NewAtomIndex>>,
}

pub struct NextBlock {
    pub known: Pattern,    // Tokens already in graph
    pub unknown: Pattern,  // New atoms need insertion
}
```

**What it does:**
```
Input: [a(new), b(new), c(new), d(new), e(new)]  (assuming initially empty graph)
       ↓ During classification: insert a, insert b, insert c, insert d, insert e
       ↓ Group consecutive new, then consecutive known
Blocks: 
  1. unknown:[a,b,c,d,e], known:[]  (all new, no known after)

Input: [a(new), b(new), a(known), b(known)]  (a,b repeat after insertion)
       ↓ During classification: insert a, insert b, find a, find b
       ↓ Group consecutive new, then consecutive known
Blocks:
  1. unknown:[a,b], known:[a,b]  (first a,b are new; second a,b are known)
```

**Why blocks?**
- **Unknown atoms**: Already inserted during classification phase
- **Known atoms**: Can be expanded to find larger patterns they belong to
- **Alternating**: Process incrementally - append new, then expand known

**Example flow:**
```
Input: [h, e, l, l, o]  (empty graph initially)
→ Classify: h(new), e(new), l(new), l(known), o(new)
→ Block 1: unknown:[h,e,l], known:[l]   (first 3 inserted; 4th l found)
→ Block 2: unknown:[o], known:[]         (o inserted; nothing after)
```

### ExpansionCtx - Pattern Expansion

**Purpose:** Expand a cursor position to find largest enclosing pattern.

```rust
pub struct ExpansionCtx<'a> {
    cursor: CursorCtx<'a>,      // Current position in pattern
    chain: BandChain,            // Chain of overlapping bands
}

impl Iterator for ExpansionCtx<'_> {
    type Item = Token;  // Next expansion
}
```

**What it does:**
1. Start at cursor position
2. Try to expand (find larger patterns)
3. Build chain of expansions
4. Return largest bundle

**Example:**
```
Cursor at "c" in "abcdef":
  → Try expand: find "cd"
  → Try expand: find "cdef"
  → Try expand: find "abcdef"
  → Return: Token("abcdef")
```

### BandChain - Expansion Chain

**Purpose:** Track series of overlapping patterns during expansion.

```rust
pub struct BandChain {
    bands: LinkedList<BandLink>,  // Ordered bands
    first: Token,                  // Starting token
}

pub struct Band {
    pattern: Pattern,    // Current pattern
    start_bound: usize,  // Start position
    end_bound: usize,    // End position (key for ordering)
}
```

**What it does:**
```
Initial: Band["c"] (start:2, end:3)
Expand:  Band["cd"] (start:2, end:4)
Expand:  Band["abcdef"] (start:0, end:6)
         ↑ Chain tracks progression
```

**Why chains?**
- Track expansion history
- Manage overlapping regions
- Enable backtracking
- Support complement calculations

### Complement Operations

**Purpose:** Calculate the "missing piece" when patterns don't align.

```rust
pub struct ComplementBuilder {
    link: ExpansionLink,  // Link between patterns
}

impl ComplementBuilder {
    pub fn build(&self, trav: &mut ReadCtx) -> Token {
        // Calculate complement
        // Insert if needed
        // Return complement token
    }
}
```

**What it does:**
```
Root: "abcdef" (positions 0-6)
Expansion: "def" (positions 3-6)
Complement: "abc" (positions 0-3)  ← The missing piece
```

**Example:**
```rust
// When expanding "def" in context of "abcdef"
let link = ExpansionLink {
    start_bound: 3,
    root_postfix: path_to("def"),
    expansion_prefix: path_from("abc"),
};
let complement = ComplementBuilder::new(link).build(&mut ctx);
// complement = Token("abc")
```

---

## Algorithm Flow

### High-Level Read Sequence

```
1. ReadCtx::new(graph, sequence)
   ├─ Convert sequence to NewAtomIndices
   ├─ Create BlockIter
   └─ Initialize RootManager

2. ReadCtx::read_sequence()
   ├─ Iterate over blocks
   │  ├─ Process unknown block (insert new atoms)
   │  └─ Process known block (expand existing patterns)
   │     ├─ Try insert_or_get_complete
   │     ├─ If incomplete: create ExpansionCtx
   │     ├─ Expand to find largest bundle
   │     └─ Update cursor
   └─ Return root token

3. ExpansionCtx::find_largest_bundle()
   ├─ Initialize with first token
   ├─ Iterate expansions
   │  ├─ Try expand pattern
   │  ├─ If expansion found:
   │  │  ├─ Calculate complement
   │  │  ├─ Insert complement if needed
   │  │  └─ Append to chain
   │  └─ If cap (boundary) reached: finalize
   └─ Return largest token from chain

4. ComplementBuilder::build()
   ├─ Get root from postfix path
   ├─ Calculate complement range (0 to intersection)
   ├─ Build trace cache for complement
   ├─ Create InitInterval
   └─ Insert complement token
```

### Detailed Expansion Process

```
Given: Pattern "cd" in "abcdef"

Step 1: Initialize ExpansionCtx
  cursor = pattern("cd")
  chain = BandChain::new(Token("cd"))
  
Step 2: Try to expand
  ExpandCtx finds parent patterns containing "cd"
  Options: "cdef", "abcdef", ...
  
Step 3: For each expansion candidate
  Create BandExpansion:
    postfix_path = path to current pattern
    expansion = larger pattern found
    start_bound = position in larger pattern
  
Step 4: Apply expansion
  Calculate complement:
    root = "abcdef"
    expansion = "def" (positions 3-6)
    complement = "abc" (positions 0-3)
  
  Insert complement if needed
  
  Create ExpansionLink:
    root_postfix = path from root to "def"
    expansion_prefix = path from expansion to "abc"
    start_bound = 3
  
  Append to chain:
    Band["cd"] → Band["cdef"] → Band["abcdef"]
  
Step 5: Return largest
  chain.last() = Token("abcdef")
```

---

## Key Components Deep Dive

### RootManager

**Purpose:** Manage the root token being constructed.

```rust
pub struct RootManager {
    pub graph: HypergraphRef,
    pub root: Option<Token>,  // Current root
}
```

**Operations:**
- `append_pattern(pattern)` - Add pattern to root
- `append_index(token)` - Add single token
- Track construction progress

### CursorCtx

**Purpose:** Wrapper combining context and cursor for expansion.

```rust
pub struct CursorCtx<'a> {
    pub ctx: ReadCtx,                    // Reading context
    pub cursor: &'a mut PatternRangePath, // Current position
}
```

**Why separate?**
- Cursor is mutable reference
- Context provides graph access
- Clean separation of concerns

### ExpansionLink

**Purpose:** Connect two patterns during expansion.

```rust
pub struct ExpansionLink {
    pub start_bound: usize,              // Position in root
    pub root_postfix: RootedRolePath,   // Path from root
    pub expansion_prefix: RootedRolePath, // Path to expansion
}
```

**Use case:**
```
When expanding "def" in "abcdef":
  start_bound = 3  (where "def" starts in "abcdef")
  root_postfix = path from "abcdef" to "def"
  expansion_prefix = path from "def" back to "abc" (complement)
```

### BandExpansion

**Purpose:** Represent a single expansion operation.

```rust
pub struct BandExpansion {
    pub postfix_path: RootedRolePath,  // Current position
    pub expansion: IndexWithPath,       // New larger pattern
    pub start_bound: usize,             // Position
}
```

**What it represents:**
```
Current: Band["cd"] at position 2
Expansion: Token("cdef") at position 2
Result: New band ["cdef"] (2-6)
```

### BandCap

**Purpose:** Mark the boundary of expansion (can't expand further).

```rust
pub struct BandCap {
    pub expansion: Token,      // Final token to append
    pub start_bound: usize,    // Boundary position
}
```

**When used:**
```
Reached end of available expansions
Cap marks where to stop
Finalize current band chain
```

---

## Future Capabilities

Based on the structure, context-read will enable:

### 1. Intelligent Pattern Discovery

**Concept:** Automatically find optimal decompositions.

```rust
// Future API (conceptual)
let sequence = "the quick brown fox";
let ctx = ReadCtx::new(graph, sequence.chars());
let optimal = ctx.discover_patterns();
// Finds: "the", "quick", "brown", "fox" as existing patterns
// Or creates: "the quick brown fox" with best decomposition
```

### 2. Context-Aware Reading

**Concept:** Read patterns with understanding of context.

```rust
// Future API (conceptual)
let pattern = "quick brown";
let context = read_ctx.expand_context(pattern);
// Returns: "the quick brown fox" (full context)
// With: Efficient decomposition based on existing patterns
```

### 3. Streaming Pattern Processing

**Concept:** Process long sequences incrementally.

```rust
// Future API (conceptual)
let mut ctx = ReadCtx::new(graph, long_sequence);
for block in ctx {
    // Process each block incrementally
    // Build up hierarchical representation
    // Memory efficient
}
```

### 4. Pattern Compression

**Concept:** Find most efficient representation.

```rust
// Future API (conceptual)
let token = graph.token_for("abcdefabcdef");
let compressed = read_ctx.compress(token);
// Finds: Pattern ["abcdef", "abcdef"]
// Instead of: ["a","b","c","d","e","f","a","b","c","d","e","f"]
```

---

## Dependencies

### Internal Dependencies

**From context-trace:**
- Hypergraph structure
- Token, Pattern types
- Path types (PatternRangePath, RootedRolePath)
- Traversal operations

**From context-search:**
- Search operations (find_ancestor)
- Response type
- Error handling

**From context-insert:**
- ToInsertCtx trait
- InsertCtx operations
- InitInterval
- Insertion for complements

**Dependency Flow:**
```
context-trace (foundation)
    ↓
context-search (query)
    ↓
context-insert (modify)
    ↓
context-read (high-level read/expand)
```

### External Dependencies

Major external deps:
- `itertools` - Block iteration, peekable operations
- `linked-hash-map`, `linked_hash_set` - Ordered collections
- `indexmap` - Ordered maps
- `async-std`, `tokio` - Async operations (future use)
- `rayon` - Parallel processing (future use)

**Note:** Heavy dependency list suggests future async/parallel features.

---

## What's Missing (TODO)

Based on incomplete implementation:

### 1. Complete Band Operations

**Current:** Basic band chain management
**Missing:** Full band merging, overlap resolution, optimal chain selection

### 2. Async Reading

**Current:** Synchronous operations
**Missing:** Async pattern reading, streaming support (deps suggest this is planned)

### 3. Advanced Complement Operations

**Current:** Basic complement calculation
**Missing:** Complex complement scenarios, multi-level complements

### 4. Pattern Optimization

**Current:** Find largest bundle
**Missing:** Optimize for memory, processing time, specific criteria

### 5. Read Strategies

**Current:** Single strategy (expand to largest)
**Missing:** Multiple reading strategies (greedy, optimal, balanced, etc.)

---

## Integration Vision

Context-read will be the **high-level API** that orchestrates:

```
User Input (sequence)
       ↓
context-read (ReadCtx)
       ├─ Block iteration (split known/unknown)
       ├─ Search (find existing patterns) ← context-search
       ├─ Insert (add new patterns) ← context-insert
       ├─ Expand (find contexts) ← expansion operations
       └─ Trace (build paths) ← context-trace
       ↓
Hierarchical Token
```

**Example complete workflow:**
```rust
// User inputs sequence
let input = "the quick brown fox jumps";

// Create reading context
let mut ctx = ReadCtx::new(graph, input.chars());

// Read sequence (orchestrates everything)
let token = ctx.read_sequence().unwrap();

// Result:
// Token("the quick brown fox jumps") with optimal decomposition:
//   - Reuses existing patterns: "the", "quick", "brown", "fox"
//   - Inserts new: "jumps"
//   - Creates hierarchy: ["the quick", "brown fox", "jumps"]
//   - Efficient: Minimal new allocations, maximal reuse
```

---

## Design Insights

### Why Block Iteration?

**Problem:** Sequences mix known and unknown patterns.

**Solution:** Process in blocks:
- **Known blocks**: Fast lookup and expansion
- **Unknown blocks**: Efficient batch insertion
- **Alternating**: Incremental progress

**Benefits:**
- Memory efficient (process in chunks)
- Optimal reuse (leverage existing patterns)
- Incremental (can stop/resume)

### Why Expansion Chains?

**Problem:** Pattern expansion isn't single-step.

**Solution:** Chain of bands:
- Track expansion history
- Handle overlapping patterns
- Support backtracking
- Enable complement calculations

**Benefits:**
- Flexible expansion strategies
- Correct complement calculation
- Efficient pattern selection
- Debugging visibility

### Why Complement Operations?

**Problem:** Expanding pattern doesn't give you the "other part".

**Solution:** Calculate complement:
- Find missing piece
- Insert if needed
- Connect to expansion

**Benefits:**
- Complete context construction
- Bridge pattern gaps
- Enable hierarchical assembly

---

## Performance Considerations

### Time Complexity

| Operation | Complexity | Explanation |
|-----------|------------|-------------|
| Block iteration | O(n) | Single pass over sequence |
| Known block processing | O(k * s) | k blocks × s search time |
| Unknown block processing | O(u * i) | u blocks × i insert time |
| Expansion | O(e * p) | e expansions × p parents |
| Complement | O(c * t) | c complements × t trace time |
| **Overall** | **O(n * (s + i + e*p))** | Dominated by search+insert+expand |

### Space Complexity

| Structure | Size | Explanation |
|-----------|------|-------------|
| BlockIter | O(n) | Stores input sequence |
| BandChain | O(e) | Expansion chain depth |
| ReadCtx | O(1) + graph | Context overhead minimal |
| Complements | O(c) | Temporary during insertion |
| **Overall** | **O(n + e + c)** | Linear in input + expansions |

### Optimization Opportunities

1. **Parallel block processing** - Independent blocks can be processed concurrently
2. **Lazy expansion** - Only expand when needed
3. **Expansion caching** - Cache expansion results
4. **Complement reuse** - Share complements across expansions

---

## Use Cases

### 1. Natural Language Processing

**Scenario:** Process text into hierarchical representations.

```
Input: "the quick brown fox"
Process:
  - Block 1: ["the"] (known)
  - Block 2: ["quick", "brown", "fox"] (expand to phrases)
Output: Hierarchical token with word/phrase structure
```

### 2. Code Analysis

**Scenario:** Build AST from token stream.

```
Input: "fn main() { println!("hello"); }"
Process:
  - Identify known patterns (keywords, operators)
  - Expand to larger constructs (expressions, statements)
  - Build hierarchical representation
Output: Structured code representation
```

### 3. Data Compression

**Scenario:** Find repeated patterns for compression.

```
Input: "abcabcabcdef"
Process:
  - Detect "abc" repetition
  - Create pattern ["abc", "abc", "abc", "def"]
  - Optimize to [repeat("abc", 3), "def"]
Output: Compressed representation
```

### 4. Pattern Learning

**Scenario:** Discover common patterns in data.

```
Input: Multiple sequences with common substrings
Process:
  - Build each sequence
  - Expansion finds shared patterns
  - Reuse increases for common patterns
Output: Automatically learned vocabulary
```

---

## Conclusion

Context-read is the **high-level orchestration layer** that:

1. **Reads sequences** incrementally with block iteration
2. **Expands patterns** to find largest contexts
3. **Calculates complements** to bridge gaps
4. **Builds hierarchies** efficiently through reuse

**Current state:** Core infrastructure in place (blocks, expansion, complements)

**Future state:** Complete reading system with async support, parallel processing, and advanced optimization

**Key innovation:** Combines **block iteration** + **expansion chains** + **complement operations** for efficient hierarchical pattern construction.

**Design goal:** Make it **easy** to read sequences and **automatic** to discover optimal hierarchical representations.

The layer completes the context-engine stack:
- **context-trace**: Graph foundation
- **context-search**: Pattern finding  
- **context-insert**: Safe modification
- **context-read**: High-level orchestration ← This layer

Together, they form a complete system for **hierarchical pattern management** in substring-aware hypergraphs.
