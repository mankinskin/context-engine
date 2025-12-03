# Context-Insert Usage Guide

> **Practical guide for using context-insert to add patterns to hypergraphs**

**Confidence:** 🟢 High - Verified patterns and examples

**Tags:** `#insert` `#patterns` `#split-join` `#initialization`

## Quick Start

### Basic Pattern Insertion

```rust
use context_insert::{ToInsertCtx, InitInterval};
use context_search::Searchable;
use context_trace::*;

// Setup graph
let mut graph = Hypergraph::default();
insert_atoms!(graph, {a, b, c, d});
let graph = HypergraphRef::from(graph);

// Insert pattern
let query = vec![a, b, c, d];
let abcd: Token = graph.insert(query.clone())?;

// Verify insertion
println!("Inserted token: {:?}, width: {}", abcd, abcd.width());
```

### Insert Only If Needed

```rust
// Search first to avoid unnecessary insertion
let query = vec![a, b, c];
let result = graph.find_ancestor(&query)?;

if result.query_exhausted() && result.is_full_token() {
    // Pattern already exists
    println!("Pattern exists: {:?}", result.root_token());
} else {
    // Need to insert
    let token = graph.insert(query)?;
    println!("Inserted: {:?}", token);
}
```

### Insert or Get Existing

```rust
// Convenience method - inserts only if needed
let token = graph.insert_or_get_complete(vec![a, b, c])?;
match token {
    Some(t) => println!("Token: {:?}", t),
    None => println!("Insertion incomplete"),
}
```

---

## Common Patterns

### Pattern 1: Build Incrementally

```rust
// Start with atoms
let mut graph = Hypergraph::default();
insert_atoms!(graph, {a, b, c, d, e, f});
let graph = HypergraphRef::from(graph);

// Build up patterns
let ab = graph.insert(vec![a, b])?;      // ab
let cd = graph.insert(vec![c, d])?;      // cd
let abcd = graph.insert(vec![ab, cd])?;  // abcd using existing patterns
let ef = graph.insert(vec![e, f])?;      // ef
let abcdef = graph.insert(vec![abcd, ef])?; // Final pattern

println!("Built: {:?}", abcdef);
```

### Pattern 2: Handle Partial Matches

```rust
// Setup existing structure
let mut graph = Hypergraph::default();
insert_atoms!(graph, {h, e, l, o});
insert_patterns!(graph,
    hel => [[h, e, l]]
);
let graph = HypergraphRef::from(graph);

// Try to find longer pattern
let query = vec![h, e, l, l, o];
let result = Searchable::search::<InsertTraversal>(
    query.clone(),
    graph.clone()
)?;

// Result matches "hel" but not complete
if !result.query_exhausted() {
    // Convert to initialization interval
    let init = InitInterval::from(result);
    
    println!("Partial match at: {:?}", init.root);
    println!("Need to extend from: {:?}", init.end_bound);
    
    // Perform insertion from this point
    let hello = graph.insert_init(extract_complete, init)?;
    println!("Completed: {:?}", hello);
}
```

### Pattern 3: Multiple Representations

```rust
// Same pattern can have multiple child structures
let mut graph = Hypergraph::default();
insert_atoms!(graph, {a, b, c});
let graph = HypergraphRef::from(graph);

// First representation: [a, b, c]
let abc1 = graph.insert(vec![a, b, c])?;

// Second representation via subpatterns
let ab = graph.insert(vec![a, b])?;
let abc2 = graph.insert(vec![ab, c])?;

// Same token!
assert_eq!(abc1, abc2);

// But now has multiple child patterns
let g = graph.graph();
let vertex = g.expect_vertex(abc1);
assert_eq!(vertex.child_patterns().len(), 2);
```

### Pattern 4: Check Before Insert

```rust
fn insert_if_missing(
    graph: &HypergraphRef,
    pattern: Vec<Token>
) -> Result<Token> {
    // Always search first
    match graph.find_ancestor(&pattern) {
        Ok(found) if found.query_exhausted() && found.is_full_token() => {
            // Already exists
            Ok(found.root_token())
        },
        Ok(_incomplete) | Err(_) => {
            // Need to insert
            graph.insert(pattern)
        }
    }
}
```

### Pattern 5: Batch Operations

```rust
// Insert multiple related patterns
let patterns = vec![
    vec![a, b],
    vec![c, d],
    vec![a, b, c, d],
];

let mut tokens = Vec::new();
for pattern in patterns {
    let token = graph.insert(pattern)?;
    tokens.push(token);
}

println!("Inserted {} patterns", tokens.len());
```

---

## Insertion Modes

### Mode 1: Direct Insert

```rust
// Simple - just insert the pattern
let token = graph.insert(vec![a, b, c])?;
```

**When to use:**
- Straightforward insertion
- Don't need fine control
- Pattern is complete

### Mode 2: Insert with InitInterval

```rust
// Advanced - control initialization
let search_result = graph.find_ancestor(&query)?;
let init = InitInterval::from(search_result);

// Specify extraction mode
let token = graph.insert_init(extract_complete, init)?;
```

**When to use:**
- Reusing search results
- Need to inspect InitInterval
- Controlling extraction mode
- Debugging insertion process

### Mode 3: Insert or Get

```rust
// Convenience - handles both cases
let token = graph.insert_or_get_complete(vec![a, b, c])?;
```

**When to use:**
- Don't care if exists or not
- Want single call
- Simplified error handling

---

## Understanding InitInterval

### What It Contains

```rust
pub struct InitInterval {
    pub root: Token,           // Where search stopped
    pub cache: TraceCache,     // Search trace data
    pub end_bound: AtomPosition, // Where to continue from
}
```

### Creating InitInterval

**From Search Result:**
```rust
let search_result = graph.find_ancestor(&query)?;
let init = InitInterval::from(search_result);
```

**Manual Construction (rare):**
```rust
let init = InitInterval {
    root: some_token,
    cache: existing_cache,
    end_bound: AtomPosition(5),
};
```

### Inspecting InitInterval

```rust
let init = InitInterval::from(search_result);

println!("Matched up to: {:?}", init.root);
println!("Match ended at position: {:?}", init.end_bound);
println!("Cache has {} entries", init.cache.entries.len());

// Check cache contents
for (token, vertex_cache) in init.cache.entries.iter() {
    println!("  Cached token: {:?}", token);
}
```

### Common InitInterval Patterns

**Pattern: Verify before use**
```rust
let init = InitInterval::from(result);

// Sanity checks
assert!(init.end_bound.0 > 0, "Invalid end bound");
assert!(!init.cache.entries.is_empty(), "Empty cache");
assert!(graph.contains(init.root), "Root not in graph");
```

**Pattern: Log for debugging**
```rust
let init = InitInterval::from(result);
tracing::debug!("InitInterval: root={:?}, end_bound={:?}, cache_size={}",
    init.root, init.end_bound, init.cache.entries.len());
```

---

## Testing Patterns

### Basic Test Structure

```rust
#[test]
fn test_insert_pattern() {
    // 1. Enable tracing (pass graph for readable labels!)
    let mut graph = Hypergraph::default();
    let _tracing = context_trace::init_test_tracing!(&graph);
    
    // 2. Setup graph
    insert_atoms!(graph, {a, b, c});
    let graph = HypergraphRef::from(graph);
    
    // 3. Perform insertion
    let abc = graph.insert(vec![a, b, c]).unwrap();
    
    // 4. Verify result
    assert_eq!(abc.width(), 3);
    
    // 5. Verify searchable
    let found = graph.find_ancestor(&vec![a, b, c]).unwrap();
    assert!(found.query_exhausted() && found.is_full_token());
}
```

### Testing Partial Matches

```rust
#[test]
fn test_partial_match_insert() {
    let mut graph = Hypergraph::default();
    let _tracing = context_trace::init_test_tracing!(&graph);
    
    insert_atoms!(graph, {a, b, c, d});
    insert_patterns!(graph,
        ab => [[a, b]]
    );
    let graph = HypergraphRef::from(graph);
    
    // Search for longer sequence
    let query = vec![a, b, c, d];
    let result = Searchable::search::<InsertTraversal>(
        query.clone(),
        graph.clone()
    ).unwrap();
    
    // Should match 'ab' but not complete
    assert!(!result.query_exhausted());
    assert_eq!(result.root_token(), ab);
    
    // Create InitInterval
    let init = InitInterval::from(result);
    assert_eq!(init.root, ab);
    assert_eq!(init.end_bound, AtomPosition(2));
    
    // Complete insertion
    let abcd = graph.insert(query).unwrap();
    assert_eq!(abcd.width(), 4);
}
```

### Testing Multiple Representations

```rust
#[test]
fn test_multiple_representations() {
    let mut graph = Hypergraph::default();
    let _tracing = context_trace::init_test_tracing!(&graph);
    
    insert_atoms!(graph, {a, b, c});
    let graph = HypergraphRef::from(graph);
    
    // Insert with different structures
    let abc1 = graph.insert(vec![a, b, c]).unwrap();
    
    let ab = graph.insert(vec![a, b]).unwrap();
    let abc2 = graph.insert(vec![ab, c]).unwrap();
    
    // Should be same token
    assert_eq!(abc1, abc2);
    
    // But has multiple child patterns
    let g = graph.graph();
    let vertex = g.expect_vertex(abc1);
    assert!(vertex.child_patterns().len() >= 2);
}
```

### Testing Edge Cases

```rust
#[test]
fn test_single_atom_pattern() {
    let mut graph = Hypergraph::default();
    insert_atoms!(graph, {a});
    let graph = HypergraphRef::from(graph);
    
    // Single atom is special case
    let result = graph.find_sequence("a".chars());
    assert!(matches!(result, Err(_))); // Single atoms are errors
}

#[test]
fn test_duplicate_insert() {
    let mut graph = Hypergraph::default();
    insert_atoms!(graph, {a, b});
    let graph = HypergraphRef::from(graph);
    
    // Insert twice
    let ab1 = graph.insert(vec![a, b]).unwrap();
    let ab2 = graph.insert(vec![a, b]).unwrap();
    
    // Should be idempotent
    assert_eq!(ab1, ab2);
}
```

---

## Common Issues & Solutions

### Issue 1: Forgetting to Convert to HypergraphRef

```rust
// ❌ Wrong - can't insert into owned graph directly
let mut graph = Hypergraph::default();
let token = graph.insert(vec![a, b])?; // Error!

// ✅ Correct - convert to ref first
let graph = HypergraphRef::from(graph);
let token = graph.insert(vec![a, b])?;
```

**Why:** Insert operations need interior mutability provided by HypergraphRef

### Issue 2: Not Checking Search Result

```rust
// ❌ Wrong - assuming search always incomplete
let result = graph.find_ancestor(&query)?;
let init = InitInterval::from(result);  // Might be complete!

// ✅ Correct - check first
let result = graph.find_ancestor(&query)?;
if !result.query_exhausted() {
    let init = InitInterval::from(result);
    // Now insert...
}
```

**Why:** If pattern exists, no need to insert

### Issue 3: Wrong end_bound Expectations

```rust
// ❌ Wrong - end_bound might not match cursor exactly
let init = InitInterval::from(result);
assert_eq!(init.end_bound, result.cursor_position()); // Might fail!

// ✅ Correct - use end_bound as-is
let init = InitInterval::from(result);
// Trust the end_bound value
```

**Why:** Cursor position semantics may differ from end_bound

### Issue 4: Modifying Graph During Insertion

```rust
// ❌ Wrong - don't modify graph during insert
let token = graph.insert(vec![a, b])?;
insert_atoms!(graph, {x, y});  // Dangerous!

// ✅ Correct - complete insert first
let token = graph.insert(vec![a, b])?;
// Now safe to modify
insert_atoms!(graph, {x, y});
```

**Why:** Insertion is multi-phase, don't disturb graph state

### Issue 5: Not Enabling Tracing in Tests

```rust
// ❌ Wrong - hard to debug without tracing
#[test]
fn test_something() {
    let mut graph = Hypergraph::default();
    // ... test code
}

// ✅ Correct - always enable tracing
#[test]
fn test_something() {
    let mut graph = Hypergraph::default();
    let _tracing = context_trace::init_test_tracing!(&graph);
    // ... test code
    // Check target/test-logs/test_something.log if it fails
}
```

**Why:** Tracing output is essential for debugging failures

---

## Performance Tips

### Tip 1: Reuse Search Results

```rust
// ✅ Good - search once, use result
let result = graph.find_ancestor(&query)?;
if !result.query_exhausted() {
    let init = InitInterval::from(result);
    graph.insert_init(extract_complete, init)?;
}

// ❌ Bad - search twice
let result = graph.find_ancestor(&query)?;
if !result.query_exhausted() {
    graph.insert(query)?;  // Searches again internally!
}
```

### Tip 2: Build Hierarchically

```rust
// ✅ Good - build from bottom up
let ab = graph.insert(vec![a, b])?;
let cd = graph.insert(vec![c, d])?;
let abcd = graph.insert(vec![ab, cd])?;  // Reuses existing

// ❌ Less efficient - flat insertion
let abcd = graph.insert(vec![a, b, c, d])?;  // Must build everything
```

### Tip 3: Batch Related Inserts

```rust
// ✅ Good - insert related patterns together
let patterns = vec![vec![a, b], vec![c, d], vec![a, b, c, d]];
for pattern in patterns {
    graph.insert(pattern)?;
}

// Graph can optimize internally
```

### Tip 4: Check Existence First

```rust
// ✅ Good - avoid unnecessary work
if !graph.find_ancestor(&query)?.is_complete() {
    graph.insert(query)?;
}

// ❌ Wasteful - always insert
graph.insert(query)?;  // Might be redundant
```

---

## Debugging Insertion

### Enable Detailed Logging

```bash
# All insert logging
RUST_LOG=context_insert=debug cargo test test_name -- --nocapture

# Specific module
RUST_LOG=context_insert::split=trace cargo test -- --nocapture

# Multiple modules
RUST_LOG=context_insert::split=trace,context_insert::join=debug cargo test
```

### Inspect InitInterval

```rust
let init = InitInterval::from(result);

eprintln!("=== InitInterval ===");
eprintln!("Root: {:?}", init.root);
eprintln!("End bound: {:?}", init.end_bound);
eprintln!("Cache size: {}", init.cache.entries.len());
eprintln!("Cache entries:");
for (token, cache) in init.cache.entries.iter() {
    eprintln!("  Token {:?}: {:?}", token, cache);
}
```

### Check Test Logs

```bash
# After test failure
cat target/test-logs/your_test_name.log

# Look for:
# - Split operations
# - Join operations  
# - Cache building
# - Error messages
```

### Add Debug Assertions

```rust
#[test]
fn test_with_assertions() {
    let result = graph.find_ancestor(&query)?;
    
    // Verify search state
    assert!(!result.query_exhausted(), "Should be incomplete");
    assert_eq!(result.cursor_position(), expected_position);
    
    let init = InitInterval::from(result);
    
    // Verify init state
    assert_eq!(init.root, expected_root);
    assert_eq!(init.end_bound, expected_bound);
    assert!(!init.cache.entries.is_empty());
    
    // Perform insertion
    let token = graph.insert_init(extract_complete, init)?;
    
    // Verify result
    assert_eq!(token.width(), expected_width);
}
```

---

## Advanced Usage

### Custom Extraction

```rust
// Different extraction modes for different use cases
let result1 = graph.insert_init(extract_complete, init.clone())?;
let result2 = graph.insert_init(extract_interval, init)?;

// Handle based on what you need
```

### Progressive Insertion

```rust
// Insert in stages for complex patterns
let result1 = graph.find_ancestor(&partial_query)?;
let init1 = InitInterval::from(result1);
let token1 = graph.insert_init(extract_complete, init1)?;

// Continue with more
let result2 = graph.find_ancestor(&extended_query)?;
let init2 = InitInterval::from(result2);
let token2 = graph.insert_init(extract_complete, init2)?;
```

### Conditional Insertion

```rust
fn insert_if_needed(
    graph: &HypergraphRef,
    query: Vec<Token>,
    condition: impl Fn(&Response) -> bool
) -> Result<Option<Token>> {
    let result = graph.find_ancestor(&query)?;
    
    if condition(&result) {
        Ok(Some(graph.insert(query)?))
    } else {
        Ok(None)
    }
}

// Usage
let token = insert_if_needed(
    &graph,
    vec![a, b, c],
    |r| !r.query_exhausted()  // Only if incomplete
)?;
```

---

## Integration with Other Crates

### With context-search

```rust
use context_search::Searchable;
use context_insert::{ToInsertCtx, InitInterval};

// Search, then insert if needed
let result = graph.find_ancestor(&query)?;
if !result.query_exhausted() {
    let init = InitInterval::from(result);
    graph.insert_init(extract_complete, init)?;
}
```

### With context-trace

```rust
use context_trace::*;
use context_insert::ToInsertCtx;

// Build graph with context-trace
let mut graph = Hypergraph::default();
insert_atoms!(graph, {a, b, c});

// Insert with context-insert
let graph = HypergraphRef::from(graph);
let abc = graph.insert(vec![a, b, c])?;

// Query back with context-trace
let g = graph.graph();
let vertex = g.expect_vertex(abc);
println!("Children: {:?}", vertex.children());
```

---

## Best Practices Summary

1. **Always search first** - Check if pattern exists
2. **Convert to HypergraphRef** - Required for insertion
3. **Enable tracing in tests** - Essential for debugging
4. **Check query_exhausted()** - Don't insert if complete
5. **Verify InitInterval** - Sanity check before using
6. **Build hierarchically** - Reuse existing patterns
7. **Handle errors properly** - Don't unwrap blindly
8. **Test edge cases** - Single atoms, duplicates, etc.
9. **Use appropriate mode** - insert() vs insert_init()
10. **Check test logs** - target/test-logs/ for failures

---

## Related Documentation

- **High-level overview**: `crates/context-insert/HIGH_LEVEL_GUIDE.md`
- **Algorithm details**: `agents/analysis/20251203_CONTEXT_INSERT_ANALYSIS.md`
- **API reference**: `cargo doc --open -p context-insert`
- **Test examples**: `crates/context-insert/src/tests/`
- **Questions**: `QUESTIONS_FOR_AUTHOR.md`

---

## Next Steps

After understanding insertion:
1. **Try examples** - Run tests to see it work
2. **Read HIGH_LEVEL_GUIDE.md** - Deeper concepts
3. **Study split-join** - Core algorithm details
4. **Experiment** - Build your own patterns
5. **Check analysis** - CONTEXT_INSERT_ANALYSIS.md for theory

## Quick Reference

```rust
// Basic insertion
let token = graph.insert(vec![a, b, c])?;

// From search result
let result = graph.find_ancestor(&query)?;
if !result.query_exhausted() {
    let init = InitInterval::from(result);
    let token = graph.insert_init(extract_complete, init)?;
}

// Insert or get
let token = graph.insert_or_get_complete(vec![a, b, c])?;

// Test template
#[test]
fn test_name() {
    let mut graph = Hypergraph::default();
    let _tracing = context_trace::init_test_tracing!(&graph);
    insert_atoms!(graph, {a, b, c});
    let graph = HypergraphRef::from(graph);
    
    let token = graph.insert(vec![a, b, c]).unwrap();
    assert_eq!(token.width(), 3);
}
```
