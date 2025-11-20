# Search Algorithm Guide

> **Quick reference for understanding how pattern search works in context-search**

## Overview

The context-search engine finds patterns in a **hierarchical hypergraph** where sequences of tokens can exist at multiple levels of abstraction. Think of it like text that can be represented as both individual characters and as higher-level words/phrases.

## Core Concept: Hierarchical Pattern Matching

The key insight is that a sequence like `[H, e, l, l, o]` might exist in the graph both as:
- **Individual atoms**: `H`, `e`, `l`, `l`, `o`
- **Higher-level patterns**: `"ell"` = `[e, l, l]`, `"Hello"` = `[H, "ell", o]`

The algorithm discovers and navigates these hierarchies automatically.

## Search Result States

### Two Orthogonal Properties

Search results have two independent boolean properties:

#### 1. Query Exhaustion (`is_exhausted()`)
**Question:** Has the entire query been matched?

```rust
// Query: [h, e, l, l, o]
// Found: "Hello" pattern
response.is_exhausted() == true  // ✓ All query tokens matched

// Query: [h, e, l, l, o, x]
// Found: "Hello" pattern but 'x' doesn't match
response.is_exhausted() == false  // ✗ Query has remaining tokens
```

#### 2. Full Token Match (`is_full_token()`)
**Question:** Is the result a complete pre-existing token in the graph?

```rust
// Found: Token for "Hello" pattern (exists in graph)
response.is_full_token() == true  // ✓ Complete token

// Found: Intersection path [h, e, l] within "Hello"
response.is_full_token() == false  // ✗ Partial path within token
```

### Four Possible Combinations

| is_exhausted() | is_full_token() | Meaning |
|----------------|------------------|---------|
| true | true | **Perfect match**: Query fully matched to existing token |
| true | false | **Exhausted on path**: Query matched but ends within a token |
| false | true | **Prefix match**: Found complete token but query continues |
| false | false | **Partial match**: Neither query exhausted nor on complete token |

### Examples

```rust
// Setup: Graph has "Hello" = [h, e, l, l, o]

// Case 1: Perfect match
let result = search([h, e, l, l, o]);
// ✓ is_exhausted() && is_full_token()
// Found complete "Hello" token, entire query matched

// Case 2: Exhausted on path  
let result = search([h, e, l]);
// ✓ is_exhausted() && !is_full_token()
// Matched up to 'l' within "Hello", but "Hello" continues

// Case 3: Prefix match
let result = search([h, e, l, l, o, x]);
// !is_exhausted() && ✓ is_full_token()
// Found complete "Hello" token, but query has 'x' remaining

// Case 4: Partial match
let result = search([h, e, x]);
// !is_exhausted() && !is_full_token()
// Stopped at 'e' within "Hello", query has 'x' remaining
```

## Algorithm Flow (Step-by-Step)

### **1. Input Processing**

```rust
// You provide a query (various formats supported)
let query = vec![a, b, c, d];  // Token array
// or
let query = some_pattern;       // Existing pattern
// or  
let query = cursor;             // Cursor at specific position
```

All queries implement the `Searchable` trait, which converts them into a unified search context with a **PatternCursor** (tracks position in the pattern).

### **2. Search Initialization**

The algorithm creates:
- **SearchIterator**: Manages the search process with a queue of candidates to explore
- **TraceCache**: Records discovered relationships between vertices
- **PatternCursor**: Tracks current position in the query pattern
- **SearchQueue**: Queue of candidate matches to explore (breadth-first or depth-first)

### **3. Hierarchical Traversal**

This is where the magic happens. The algorithm:

**A. Generates Parent Candidates**
- For the current token, find all parent patterns that contain it
- Example: `l` might be in patterns `"ll"`, `"ell"`, `"llo"`, `"hello"`

**B. Compares Each Candidate**
- For each parent pattern, check if it matches the query at this position
- Uses **token-by-token comparison** within the pattern structure

**C. Updates Best Checkpoint**
- Tracks the **deepest match found** (most query tokens matched)
- Even partial matches are saved for later use

**D. Explores Parents Recursively**
- If a pattern matches, queue its parents for exploration
- This allows finding `"hello"` even when searching `[h, e, l, l, o]`

### **4. Match Detection**

The algorithm tracks the best match found using a checkpoint system:

**Checkpoint Selection Criteria:**
1. **Most query tokens matched** (highest `atom_position`)
2. If tied, prefer **query exhausted** over partial
3. If still tied, prefer **exact match** over intersection path

**Result Types:**
- **PathCoverage::EntireRoot**: Matched a complete existing token
- **PathCoverage::Range/Prefix/Postfix**: Matched a path within a token

### **5. Result Construction**

Returns a **Response** containing:
```rust
Response {
    cache: TraceCache,           // All discovered relationships
    end: MatchedEndState {       // Final state
        path: PathCoverage,      // What pattern matched
        cursor: PatternCursor,   // Position reached
    }
}

// Check the result
response.is_exhausted();    // Query fully matched?
response.is_full_token();   // Result is complete token?
```

## Key Features

### **Pattern Recognition**

The algorithm can recognize that `[e, l, l]` forms the pattern `"ell"`, then use that knowledge to match `[H, "ell", o]` as `"Hello"`.

### **Cache-Aware**

Builds a **TraceCache** recording:
- **Bottom-up relationships**: Which tokens are children of which patterns
- **Top-down relationships**: Which patterns contain which tokens
- Reusable across searches for performance

### **Flexible Matching**

Supports multiple search strategies:
- **AncestorSearch**: Finds the largest containing pattern
- **ParentSearch**: Finds immediate parent patterns
- Custom traversal policies for specific needs

### **State Preservation**

Even failed/partial searches return useful data:
- Exact position where matching stopped
- Cache of what was explored
- Can be used to guide insertions (see `context-insert`)

## Example Walkthrough

```rust
// Graph contains:
// Atoms: h, e, l, l, o
// Patterns: "ell" = [e,l,l], "Hello" = [h,"ell",o]

let query = vec![h, e, l, l, o];
let result = query.search::<AncestorSearchTraversal>(graph)?;

// Algorithm steps:
// 1. Start at 'h' → match 'h'
// 2. Advance to 'e' → match 'e'
// 3. Advance to 'l' → match 'l'
// 4. Advance to 'l' → match 'l'
//    → Recognize [e,l,l] forms "ell" pattern!
//    → Reinterpret as [h, "ell", ...]
// 5. Advance to 'o' → match 'o'
//    → Recognize [h,"ell",o] forms "Hello" pattern!
// 6. Complete! Found "Hello"

result.is_exhausted();    // true - entire query matched
result.is_full_token();   // true - found complete "Hello" token
result.root_token();      // Token for "Hello" pattern
```

## Checkpoint System Deep Dive

### Why Checkpoints?

During hierarchical traversal, the algorithm explores multiple candidate patterns. Not all lead to better matches. The checkpoint system tracks the **best match found so far**.

### Checkpoint Updates

```rust
// Checkpoint is updated when:
1. First match found (no previous checkpoint)
2. New match has higher atom_position (more query matched)
3. Same atom_position but new match is exhausted (previous wasn't)
```

### Checkpoint vs Final Result

- **Checkpoint**: Best match found during traversal
- **Final Result**: The checkpoint when search completes (no more candidates)

If no match found, returns empty mismatch at position 0.

## Common Patterns

### Check Before Using

```rust
// ✅ Safe pattern - check both properties
if response.is_exhausted() {
    if response.is_full_token() {
        // Perfect match: use as complete token
        let token = response.expect_complete("checked").root_parent();
    } else {
        // Exhausted on path: query matched but within a token
        let path = response.query_pattern();
    }
} else {
    // Query not fully matched: use for insertion
    let init = InitInterval::from(response);
}

// ❌ Unsafe - assuming exhausted means exact match
let token = response.expect_complete("hope it's exact!");
```

### Handle All Cases

```rust
match (response.is_exhausted(), response.is_full_token()) {
    (true, true) => {
        // Perfect match: complete token, entire query
        println!("Found exact: {:?}", response.root_token());
    },
    (true, false) => {
        // Exhausted on path: query done, but inside token
        println!("Query exhausted within: {:?}", response.root_token());
    },
    (false, true) => {
        // Prefix match: found complete token, query continues
        println!("Token complete, query has more: {:?}", response.cursor_position());
    },
    (false, false) => {
        // Partial match: neither exhausted nor full token
        println!("Partial match at: {:?}", response.cursor_position());
    }
}
```

## Why This Design?

**Hierarchical matching** allows:
- **Efficient storage**: Reuse patterns instead of duplicating sequences
- **Semantic understanding**: Patterns can represent meaningful units
- **Flexible queries**: Search with atoms or patterns interchangeably
- **Incremental discovery**: Find higher-level structures during search

The algorithm essentially performs **bottom-up pattern discovery** while **top-down validation**, maintaining the best match found so far as a checkpoint for robust handling of partial matches.

## Related Documentation

- **CHEAT_SHEET.md**: Quick API reference for Response methods
- **crates/context-search/HIGH_LEVEL_GUIDE.md**: Comprehensive search concepts
- **agents/tmp/PATTERN_MATCHING_EXAMPLE.md**: Detailed step-by-step example
- **agents/guides/UNIFIED_API_GUIDE.md**: Response type and unified API

## Tags

#search #algorithm #hierarchical #pattern-matching #response-api #checkpoint #query-exhaustion #exact-match
