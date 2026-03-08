---
tags: `#guide` `#context-search` `#context-insert` `#debugging` `#testing` `#api`
summary: This guide explains how context-insert interacts with context-search, focusing on the critical concepts of position semantics and trace cache usage...
---

# Context-Insert and Context-Search Interoperability Guide

**Tags:** `#context-insert` `#context-search` `#interoperability` `#position-semantics` `#trace-cache`

## Overview

This guide explains how context-insert interacts with context-search, focusing on the critical concepts of position semantics and trace cache usage. Understanding these concepts is essential for correctly creating InitInterval instances and debugging insertion failures.

## Position Semantics

### Two Types of Positions

The context-search API provides two different position accessors, each with distinct semantics:

#### 1. Cursor Position - `cursor_position()`
```rust
pub fn cursor_position(&self) -> &TokenPosition
```

**Semantics:**
- Current position of the search cursor
- May point **inside** a pattern during matching
- Represents "where we are now" in the search
- Can be mid-match (not at a pattern boundary)

**Example:**
```rust
// Searching for pattern [0, 1, 2] in token stream [5, 0, 1, 2, 6]
// When matching the '2':
let cursor = response.cursor_position();  
// cursor points at index 4 (inside the pattern)
```

**Use Cases:**
- Resuming interrupted searches
- Debugging search state
- Understanding current search position

**❌ DON'T use for:** Insertion boundaries (may split patterns)

#### 2. Checkpoint Position - `checkpoint_position()`
```rust
pub fn checkpoint_position(&self) -> &TokenPosition
```

**Semantics:**
- Position of the **last confirmed complete match**
- Always points at a pattern boundary
- Represents "last known good state"
- Safe for insertion/deletion operations

**Example:**
```rust
// After matching pattern [0, 1, 2]:
let checkpoint = response.checkpoint_position();
// checkpoint points at index 1 (start of matched pattern)
// or index 4 (end of matched pattern after advancing)
```

**Use Cases:**
- ✅ Creating InitInterval for insertion
- ✅ Calculating insertion boundaries
- ✅ Determining split points in graph

**✅ ALWAYS use for:** Insertion boundaries

### Common Mistake: Using cursor_position() for Insertion

**❌ WRONG:**
```rust
let response = search(pattern, context)?;
let start = response.cursor_position().clone();  // May be mid-match!
let interval = InitInterval::new(start, end, ...);
```

**✅ CORRECT:**
```rust
let response = search(pattern, context)?;
let start = response.checkpoint_position().clone();  // Pattern boundary
let interval = InitInterval::new(start, end, ...);
```

**Why it matters:**
- Using cursor position can create intervals that start/end inside patterns
- This violates graph invariants (patterns must be atomic)
- Leads to cache misalignment and assertion failures
- Tests fail with width mismatches or unexpected pattern boundaries

## Trace Cache Structure

### What is the Trace Cache?

The trace cache stores the path taken through the graph during pattern matching. It maps positions to the hyperedges visited at each step.

**Type:**
```rust
HashMap<TokenPosition, IndexAxis<Hyperedge>>
```

**Entry Format:**
- **Key:** `TokenPosition` - Position in the matched token stream
- **Value:** `IndexAxis<Hyperedge>` - Hyperedge (pattern) traversed at that position

### Accessing the Trace Cache

```rust
// Get match result from response
let match_result = response.match_result()
    .expect("Search incomplete");

// Extract trace cache
let trace_cache = match_result.trace_cache();

// Iterate over matched path
for (position, hyperedge) in trace_cache {
    println!("At position {:?}, traversed pattern: {:?}", position, hyperedge);
}
```

### Trace Cache Usage in InitInterval

The trace cache is used to reconstruct the pattern sequence that was matched:

```rust
pub fn new(
    start: TokenPosition,
    end: TokenPosition,
    new_pattern: VertexIndex,
    trace: TraceCache,  // From match_result.trace_cache()
    root: VertexIndex,
) -> Self {
    // Use trace to build pattern sequence for split-join
    InitInterval {
        start,
        end,
        new_pattern,
        trace,
        root,
    }
}
```

**What the trace enables:**
- Reconstruct the exact pattern path matched
- Identify split points in nested patterns
- Determine vertical vs horizontal splits
- Build pattern_info for join phase

## Response API Integration

### Complete Match Workflow

```rust
use context_search::{search, Response};
use context_insert::InitInterval;

// 1. Perform search
let response = search(&pattern, &context)?;

// 2. Check if search completed
if !response.is_complete() {
    return Err("Search incomplete");
}

// 3. Get match result
let match_result = response.match_result()
    .expect("is_complete() returned true");

// 4. Extract insertion boundaries (use checkpoint!)
let start = match_result.checkpoint_position().clone();
let end = calculate_end_position(&start, &pattern);  // Based on pattern width

// 5. Extract trace cache
let trace_cache = match_result.trace_cache().clone();

// 6. Create InitInterval
let interval = InitInterval::new(
    start,
    end,
    new_pattern_vertex,
    trace_cache,
    root_vertex,
);

// 7. Perform insertion
let result = insert_context(interval, &mut graph)?;
```

### Handling Incomplete Searches

```rust
let response = search(&pattern, &context)?;

match response.is_complete() {
    true => {
        // Safe to extract match_result and create InitInterval
        let match_result = response.match_result().unwrap();
        let interval = create_interval_from_match(match_result);
        insert_context(interval, &mut graph)?;
    }
    false => {
        // Search didn't complete - can't insert yet
        // Options:
        // 1. Continue search: let cursor = response.resume();
        // 2. Handle partial match: handle_incomplete(response);
        // 3. Return error: Err("Pattern not found")
    }
}
```

## Common Patterns

### Pattern 1: Simple Insertion After Match

```rust
// Search for pattern and insert replacement
fn replace_pattern(
    pattern: &[Token],
    replacement: VertexIndex,
    graph: &mut Graph,
) -> Result<(), Error> {
    let context = SearchContext::new(graph);
    let response = search(pattern, &context)?;
    
    if !response.is_complete() {
        return Err(Error::PatternNotFound);
    }
    
    let match_result = response.match_result().unwrap();
    
    // Use checkpoint position for boundaries
    let start = match_result.checkpoint_position().clone();
    let pattern_width = pattern.len();
    let end = start.advance_by(pattern_width);
    
    let interval = InitInterval::new(
        start,
        end,
        replacement,
        match_result.trace_cache().clone(),
        graph.root(),
    );
    
    insert_context(interval, graph)?;
    Ok(())
}
```

### Pattern 2: Multiple Insertions from Same Search

```rust
// Insert at multiple match positions
fn insert_at_all_matches(
    pattern: &[Token],
    replacement: VertexIndex,
    graph: &mut Graph,
) -> Result<Vec<InsertResult>, Error> {
    let mut results = Vec::new();
    let context = SearchContext::new(graph);
    
    // Get all matches
    let matches = find_all_matches(pattern, &context)?;
    
    for match_result in matches {
        // Each match has its own checkpoint and trace
        let start = match_result.checkpoint_position().clone();
        let end = calculate_end(&start, pattern);
        let trace = match_result.trace_cache().clone();
        
        let interval = InitInterval::new(start, end, replacement, trace, graph.root());
        let result = insert_context(interval, graph)?;
        results.push(result);
    }
    
    Ok(results)
}
```

### Pattern 3: Conditional Insertion Based on Context

```rust
// Insert only if context matches additional criteria
fn conditional_insert(
    pattern: &[Token],
    replacement: VertexIndex,
    graph: &mut Graph,
    predicate: impl Fn(&TraceCache) -> bool,
) -> Result<Option<InsertResult>, Error> {
    let context = SearchContext::new(graph);
    let response = search(pattern, &context)?;
    
    if !response.is_complete() {
        return Ok(None);
    }
    
    let match_result = response.match_result().unwrap();
    
    // Check trace cache against predicate
    if !predicate(match_result.trace_cache()) {
        return Ok(None);  // Context doesn't match
    }
    
    // Proceed with insertion
    let start = match_result.checkpoint_position().clone();
    let end = calculate_end(&start, pattern);
    
    let interval = InitInterval::new(
        start,
        end,
        replacement,
        match_result.trace_cache().clone(),
        graph.root(),
    );
    
    let result = insert_context(interval, graph)?;
    Ok(Some(result))
}
```

## Debugging Tips

### Issue: Width Mismatch Errors

**Symptom:** Assertions fail with unexpected pattern widths

**Likely Cause:** Using `cursor_position()` instead of `checkpoint_position()`

**Fix:**
```rust
// ❌ Before
let start = response.cursor_position().clone();

// ✅ After
let start = response.checkpoint_position().clone();
```

### Issue: Cache Position Off by N

**Symptom:** Trace cache positions don't align with expected positions

**Likely Cause:** Position calculation doesn't account for pattern boundaries

**Debug Steps:**
1. Log both cursor and checkpoint positions
2. Verify checkpoint is at pattern boundary
3. Check end position calculation includes full pattern width
4. Validate trace cache entries align with [start, end) interval

**Example Debug Output:**
```rust
tracing::debug!(
    cursor = ?response.cursor_position(),
    checkpoint = ?response.checkpoint_position(),
    "Position comparison"
);

// Expected: checkpoint at start or end of pattern
// cursor may be anywhere in pattern
```

### Issue: Incomplete Search State

**Symptom:** `match_result()` returns None or panics

**Likely Cause:** Search didn't complete matching

**Fix:**
```rust
// Always check completion before accessing match_result
if !response.is_complete() {
    tracing::warn!("Search incomplete, cannot create InitInterval");
    return Err(Error::IncompleteSearch);
}

let match_result = response.match_result()
    .expect("is_complete() guarantees Some");
```

## Testing Guidelines

### Test Setup with Correct Positions

```rust
#[test]
fn test_insertion_boundaries() {
    let graph = setup_test_graph();
    let pattern = vec![token_a, token_b, token_c];
    
    let response = search(&pattern, &graph).unwrap();
    assert!(response.is_complete(), "Pattern should match");
    
    let match_result = response.match_result().unwrap();
    
    // Use checkpoint for boundaries
    let start = match_result.checkpoint_position().clone();
    let end = start.advance_by(pattern.len());
    
    // Verify boundaries are at pattern edges
    assert_eq!(start.token_at(start.offset()), token_a);
    assert_eq!(end.token_at(end.offset() - 1), token_c);
    
    // Create interval and insert
    let interval = InitInterval::new(start, end, replacement, trace, root);
    let result = insert_context(interval, &mut graph);
    
    assert!(result.is_ok());
}
```

### Test Tracing for Position Debugging

```rust
#[test]
fn test_with_position_tracing() {
    let _tracing = init_test_tracing!(&graph);
    
    let response = search(&pattern, &graph).unwrap();
    
    // Log positions for debugging
    tracing::info!(
        cursor = ?response.cursor_position(),
        checkpoint = ?response.checkpoint_position(),
        "Search positions"
    );
    
    let match_result = response.match_result().unwrap();
    
    // Log trace cache entries
    for (pos, edge) in match_result.trace_cache() {
        tracing::debug!(
            position = ?pos,
            edge = ?edge,
            "Trace entry"
        );
    }
    
    // Rest of test...
}
```

## Related Documentation

- **Architecture:** `agents/analysis/20251204_CONTEXT_INSERT_ARCHITECTURE.md`
- **API Reference:** `agents/CHEAT_SHEET.md` (Position Semantics section)
- **Crate Guide:** `crates/context-insert/HIGH_LEVEL_GUIDE.md`
- **Search API:** `crates/context-search/HIGH_LEVEL_GUIDE.md`

## Summary

**Key Takeaways:**
1. Always use `checkpoint_position()` for insertion boundaries
2. `cursor_position()` is for search state, not insertion
3. Trace cache maps positions to hyperedges traversed
4. Check `is_complete()` before accessing `match_result()`
5. Test with tracing enabled to debug position issues

**Common Mistakes:**
- Using cursor position instead of checkpoint
- Not checking search completion
- Incorrect end position calculation
- Misinterpreting trace cache structure

**Best Practices:**
- Extract checkpoint position first
- Calculate end based on pattern width
- Clone trace cache for InitInterval
- Enable tracing in tests
- Validate positions at pattern boundaries
