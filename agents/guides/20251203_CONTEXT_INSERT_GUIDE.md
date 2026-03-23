---
tags: `#guide` `#context-insert` `#algorithm` `#debugging` `#testing` `#api` `#performance`
summary: ```rust
---

# Context-Insert Guide

**Practical guide for safe pattern insertion via split-join architecture**

---

## Quick Start

```rust
// Basic insertion
let result = graph.insert(vec![a, b, c])?;

// Insert if needed (search first)
let response = graph.find_ancestor(query)?;
if !response.query_exhausted() {
    let token = graph.insert_init((), InitInterval::from(response))?;
}

// Insert or get existing
let token = if let Ok(r) = graph.find_ancestor(query) {
    r.expect_complete("should exist").root_parent()
} else {
    graph.insert(query)?
};
```

---

## Common Patterns

| Pattern | Code |
|---------|------|
| Build incrementally | `graph.insert(vec![a,b]); graph.insert(vec![ab,c])` |
| Handle partial match | `if !r.query_exhausted() { graph.insert_init((), InitInterval::from(r))? }` |
| Multiple representations | Multiple `insert()` calls with same token, different patterns |
| Check before insert | `graph.find_ancestor(q).is_err()` → insert |
| Batch operations | Loop with `insert()` reusing intermediate results |

---

## Insertion Modes

| Mode | When to Use | API |
|------|-------------|-----|
| Direct insert | New pattern, no search | `graph.insert(pattern)` |
| InitInterval | After incomplete search | `graph.insert_init((), InitInterval::from(response))` |
| Insert or get | Idempotent operations | Check with `find_ancestor()` first |

---

## InitInterval

```rust
InitInterval {
    root: Token,              // Partially matched token
    cache: TraceCache,        // Reusable search data
    end_bound: AtomPosition   // Where to extend from (checkpoint!)
}

// Creation
InitInterval::from(response)  // ⚠️ Uses checkpoint_position(), not cursor_position()

// Inspection
println!("Root: {:?}, Bound: {:?}", init.root, init.end_bound);
assert!(init.end_bound.0 < root_width);
```

---

## Testing

```rust
#[test]
fn test_insertion() {
    let _tracing = init_test_tracing!(&graph);
    insert_atoms!(graph, {a, b, c});
    
    // Test partial match + insert
    let r = graph.find_ancestor(vec![a, b, c, d]).unwrap();
    assert!(!r.query_exhausted());
    let abcd = graph.insert_init((), InitInterval::from(r))?;
    
    // Verify
    let r2 = graph.find_ancestor(vec![a, b, c, d]).unwrap();
    assert!(r2.is_complete());
}
```

---

## Common Issues

| Issue | Symptom | Fix |
|-------|---------|-----|
| Not using HypergraphRef | Borrow errors | Wrap: `HypergraphRef::from(graph)` |
| Unchecked search result | Unwrap panic | Check `is_complete()` or `query_exhausted()` |
| Wrong end_bound | Width mismatch | Use `checkpoint_position()` not `cursor_position()` |
| Graph modification during insert | Inconsistent state | Don't modify graph between search and insert |
| No tracing in tests | Hard to debug | `init_test_tracing!(&graph)` |

---

## Performance Tips

1. **Reuse search results** - `InitInterval` contains cached trace data
2. **Build hierarchically** - Insert atoms first, then compounds
3. **Batch related inserts** - Group by shared prefixes
4. **Check existence** - `find_ancestor()` before `insert()` to avoid duplicates

---

## Debugging

```bash
# Full tracing
LOG_STDOUT=1 LOG_FILTER=trace cargo test <name> -- --nocapture

# Check logs
cat target/test-logs/<test_name>.log

# Debug InitInterval
println!("Init: root={:?} bound={:?}", init.root, init.end_bound);
assert!(init.end_bound.0 > 0 && init.end_bound.0 <= root_width);
```

---

## Advanced Usage

**Custom extraction:**
```rust
graph.insert_init(
    |state| state.extract_interval(),  // Custom extractor
    init
)?
```

**Directional insertion:**
```rust
use context_insert::insert::Direction;
graph.insert_with_direction(pattern, Direction::Prefix)?
```

**See also:** CONTEXT_INSERT_ARCHITECTURE.md (algorithm details), CONTEXT_INSERT_SEARCH_INTEROP.md (search integration)
