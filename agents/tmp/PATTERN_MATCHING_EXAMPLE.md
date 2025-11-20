# Pattern Matching Algorithm Example

This document provides a detailed walkthrough of how the context-search crate finds patterns in a hypergraph, using a concrete example with step-by-step algorithm execution.

## Example Setup

### Hypergraph Structure
Let's create a hypergraph representing the text "Hello World" with hierarchical patterns:

```
Atoms: H(0), e(1), l(2), l(3), o(4), W(5), o(6), r(7), l(8), d(9)

Patterns:
- "ell" → [e(1), l(2), l(3)]           // Pattern P1, Token T1
- "llo" → [l(2), l(3), o(4)]           // Pattern P2, Token T2  
- "Hello" → [H(0), T1, o(4)]           // Pattern P3, Token T3
- "World" → [W(5), o(6), r(7), l(8), d(9)]  // Pattern P4, Token T4
- "Hello World" → [T3, T4]             // Pattern P5, Token T5
```

### Search Query
**Query**: Find the pattern `[H, e, l, l, o]` (searching for "Hello" as individual atoms)

### Expected Result
The algorithm should find that this sequence can be completed by matching against Pattern P3 ("Hello").

## Algorithm Execution Step-by-Step

### Step 1: Input Processing and Folding

#### 1.1 Foldable Conversion
```rust
// Input: &[Token] = &[H(0), e(1), l(2), l(3), o(4)]
let query = &[atom_H, atom_e, atom_l1, atom_l2, atom_o];

// Foldable::to_fold_context() creates initial search context
let fold_ctx = FoldCtx {
    cursor: PatternCursor {
        path: RootedRolePath::new(query.to_vec(), RolePath::empty()),
        relative_pos: 0.into(),
    },
    traversal: BreadthFirst::new(graph_ref),
};
```

#### 1.2 Search Strategy Selection
```rust
// Using BreadthFirst strategy
// - Explores all possibilities at each position before advancing
// - Better for finding shortest/most direct matches
let traversal = BreadthFirst::new(graph_ref);
```

### Step 2: Search Initialization

#### 2.1 Create Start Context
```rust
let start_ctx = StartCtx {
    trav: graph_ref,
    root: atom_H,  // Starting from 'H' atom
    cache: TraceCache::new(atom_H),
};

// Initial cursor points to first element
let cursor = PatternCursor {
    path: RootedRolePath::new(query.clone(), RolePath::new(0, vec![])),
    relative_pos: 0.into(),
};
```

#### 2.2 Initialize Trace Cache
```rust
// Cache tracks visited vertices and their relationships
let mut cache = TraceCache::new(atom_H);
// Initial entry: H(0) → VertexCache { bottom_up: [], top_down: [] }
```

### Step 3: Position-by-Position Matching

#### 3.1 Position 0: Match 'H'
```rust
// Current: cursor.relative_pos = 0, target = H(0)
// Direct atom match - success
let current_match = atom_H;  // H(0)

// Update cursor to next position
cursor.relative_pos = 1.into();  // Move to position 1
```

**Cache State:**
```rust
cache.entries = {
    H(0) → VertexCache { 
        index: H(0),
        bottom_up: [],
        top_down: [],
    }
}
```

#### 3.2 Position 1: Match 'e'
```rust
// Current: cursor.relative_pos = 1, target = e(1)
// Check for direct match: e(1) exists
let current_match = atom_e;  // e(1)

// Update cursor
cursor.relative_pos = 2.into();  // Move to position 2
```

**Cache State:**
```rust
cache.entries = {
    H(0) → VertexCache { ... },
    e(1) → VertexCache {
        index: e(1),
        bottom_up: [],
        top_down: [],
    }
}
```

#### 3.3 Position 2: Match 'l' (First L)
```rust
// Current: cursor.relative_pos = 2, target = l(2)
// Direct atom match
let current_match = atom_l1;  // l(2)

// Also check for pattern opportunities
// Note: l(2) is part of pattern "ell" [e(1), l(2), l(3)]
// This creates a potential alternative path
```

**Cache State:**
```rust
cache.entries = {
    H(0) → VertexCache { ... },
    e(1) → VertexCache { ... },
    l(2) → VertexCache {
        index: l(2),
        bottom_up: [],
        top_down: [
            (2.into(), PositionCache::with_bottom({
                DirectedKey::down(e, 2) → SubLocation::new(ell_pattern_id, 1)
            }))
        ],
    }
}
```

#### 3.4 Position 3: Match 'l' (Second L) - Pattern Discovery
```rust
// Current: cursor.relative_pos = 3, target = l(3)
// Direct atom match: l(3)

// IMPORTANT: Algorithm detects pattern "ell" completion
// [e(1), l(2), l(3)] forms complete pattern P1 → Token T1
let ell_pattern = Token::T1;  // Represents "ell" pattern

// This creates alternative interpretation:
// Query could be: [H(0), T1, o(4)] (using ell pattern)
// This matches pattern P3 exactly!
```

**Cache State with Pattern Discovery:**
```rust
cache.entries = {
    // ... previous entries ...
    T1 → VertexCache {  // "ell" pattern token
        index: T1,
        bottom_up: [
            (1.into(), PositionCache::with_bottom({
                DirectedKey::up(e, 1) → SubLocation::new(ell_pattern_id, 0)
            }))
        ],
        top_down: [
            (1.into(), PositionCache::with_bottom({
                DirectedKey::down(l(2), 1) → SubLocation::new(ell_pattern_id, 1),
                DirectedKey::down(l(3), 1) → SubLocation::new(ell_pattern_id, 2)
            }))
        ],
    }
}
```

### Step 4: Pattern Completion Detection

#### 4.1 Recognize Higher-Level Pattern
```rust
// At position 3, algorithm recognizes potential pattern match:
// Current sequence: [H(0), e(1), l(2), l(3), ...]
// Can be reinterpreted as: [H(0), T1("ell"), ...]

// Check if this forms a known pattern:
// Pattern P3: "Hello" = [H(0), T1("ell"), o(4)]
let hello_pattern_match = graph.find_pattern(&[atom_H, ell_token, atom_o]);
```

#### 4.2 Position 4: Complete Pattern Match
```rust
// Current: cursor.relative_pos = 4, target = o(4)
// Direct atom match: o(4)

// PATTERN COMPLETION DETECTED!
// [H(0), T1("ell"), o(4)] exactly matches Pattern P3 ("Hello")
let complete_match = Token::T3;  // "Hello" pattern token
```

### Step 5: Result Construction

#### 5.1 Create Finished State
```rust
let finished_state = FinishedState {
    kind: FinishedKind::Complete(Box::new(CompleteEnd {
        cursor: PatternCursor {
            path: RootedRolePath::new(
                query.clone(),
                RolePath::new(4, vec![
                    ChildLocation::new(ell_token, ell_pattern_id, 1),
                ])
            ),
            relative_pos: 5.into(),  // End of pattern
        },
    })),
    cache: cache,
};
```

#### 5.2 Final Cache State
```rust
// Complete cache showing all discovered relationships
cache.entries = {
    H(0) → VertexCache { ... },
    e(1) → VertexCache { 
        bottom_up: [],
        top_down: [(1, DirectedKey::down(T1, 1) → ell_pattern_id.0)]
    },
    l(2) → VertexCache {
        bottom_up: [],
        top_down: [(2, DirectedKey::down(T1, 2) → ell_pattern_id.1)]
    },
    l(3) → VertexCache {
        bottom_up: [],
        top_down: [(3, DirectedKey::down(T1, 3) → ell_pattern_id.2)]
    },
    o(4) → VertexCache { ... },
    T1 → VertexCache {  // "ell" pattern
        bottom_up: [(1, DirectedKey::up(e, 1) → ell_pattern_id.0)],
        top_down: [(1, [l(2), l(3)] mappings)]
    },
    T3 → VertexCache {  // "Hello" pattern
        bottom_up: [(0, DirectedKey::up(H, 0) → hello_pattern_id.0)],
        top_down: [(0, [T1, o(4)] mappings)]
    }
}
```

## Algorithm Flow Summary

### Phase 1: Linear Matching
1. **Start** at position 0 with atom H(0)
2. **Match** each position sequentially: H(0) → e(1) → l(2) → l(3) → o(4)
3. **Track** each successful match in trace cache

### Phase 2: Pattern Recognition
1. **Detect** that [e(1), l(2), l(3)] forms pattern "ell" (T1)
2. **Reinterpret** sequence as [H(0), T1, o(4)]
3. **Recognize** this matches "Hello" pattern (T3)

### Phase 3: Completion
1. **Validate** complete pattern match
2. **Construct** result with success indicator
3. **Return** cache containing all discovered relationships

## Key Algorithm Features Demonstrated

### 1. **Hierarchical Pattern Discovery**
- Algorithm doesn't just match atoms linearly
- Recognizes when subsequences form higher-level patterns
- Can work with multiple levels of pattern hierarchy

### 2. **Efficient Caching**
- Tracks all explored paths and relationships
- Enables reuse of discovered patterns
- Provides detailed trace information for debugging

### 3. **Flexible Matching**
- Can find multiple valid interpretations of the same sequence
- Supports both direct atom matching and pattern-based matching
- Handles overlapping patterns gracefully

### 4. **State Preservation**
- Maintains exact cursor position throughout search
- Preserves all intermediate states for analysis
- Enables search continuation and resumption

## Alternative Search Strategies

### Depth-First vs Breadth-First

**Breadth-First (used above):**
- Explores all possibilities at each position
- Better for finding shortest/most direct matches
- Higher memory usage but more comprehensive

**Depth-First Alternative:**
- Would follow first viable path to completion
- Faster for simple cases but might miss optimal matches
- Lower memory usage but potentially less thorough

### Pattern-First vs Atom-First

**Atom-First (demonstrated):**
- Starts with individual atoms, builds up to patterns
- Good for finding exact sequence matches
- Natural for text-like sequential data

**Pattern-First Alternative:**
- Starts with known patterns, matches against query
- Better for structural/semantic matching
- More efficient when query likely contains known patterns

This example demonstrates the sophisticated pattern recognition capabilities of the context-search architecture, showing how it can seamlessly transition between atom-level and pattern-level matching to find optimal results in hierarchical graph structures.