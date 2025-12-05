# Context Framework Cheat Sheet

**Quick API reference for context-trace, context-search, and context-insert**

---

## Essential Types

```rust
// Graph
Hypergraph<G>, HypergraphRef (Arc<RwLock<Hypergraph>>), Token{index, width}

// Paths (context-trace)
RolePath<R>, IndexRangePath, PatternRangePath, RootedRolePath<R>

// Search (context-search) - CURRENT API
Response { cache: TraceCache, end: EndState }  // Unified result type
response.is_complete() -> bool
response.expect_complete(msg) -> IndexRangePath  // Panics if incomplete
response.root_token() -> Token
response.cursor_position() -> AtomPosition

Searchable::search::<K>(query, graph) -> Result<Response, ErrorState>
graph.find_ancestor(query) -> Result<Response, ErrorReason>

// Insert (context-insert)
InsertCtx<G>, InitInterval{root, cache, end_bound}
InitInterval::from(response)  // Convert search result
graph.insert(pattern), graph.insert_init(extract, init)
```

---

## Top 10 Patterns

| Pattern | Code |
|---------|------|
| Create + search | `graph.insert_atoms(atoms); graph.find_ancestor(query)` |
| Check complete | `response.is_complete()` or `response.expect_complete("msg")` |
| Insert if needed | `if !r.query_exhausted() { graph.insert_init((), InitInterval::from(r))? }` |
| Get root token | `response.expect_complete("msg").root_parent()` |
| Iterate vertices | `graph.iter_vertex_data().for_each(\|(token, data)\| ...)` |
| Rooted path | `path.to_rooted(root)` or `rooted_path!(graph, root => [child_locs])` |
| Test tracing | `let _tracing = init_test_tracing!(&graph);` |
| Traverse patterns | `data.expect_any_child_pattern()` or `data.iter_all_child_patterns()` |
| Path children | `path.iter_child_locations()` or `path.locations()` |
| Position check | `response.checkpoint_position()` (confirmed) vs `cursor_position()` (speculative) |

---

## Critical Gotchas

1. **Response fields are private** - Use `.is_complete()`, `.expect_complete()`, NOT `.end.path`
2. **Always call `.root_parent()`** after `.expect_complete()` - Path ≠ Token
3. **InitInterval from Response** - Use `checkpoint_position()` not `cursor_position()`
4. **Check before insert** - `!query_exhausted()` AND `!is_full_token()` before inserting
5. **Test tracing** - Use `init_test_tracing!(&graph)` NOT old `init_tracing()`

---

## Testing Essentials

```rust
// Setup
let _tracing = init_test_tracing!(&graph);  // Pass graph for readable tokens!
insert_atoms!(graph, {a, b, c});
insert_patterns!(graph, abc => [[a, b, c]]);

// Assertions
assert!(response.is_complete());
assert_eq!(path.root_parent(), expected_token);

// RootedRolePath creation
rooted_path!(graph, root => [child_locs])
```

---

## Debug Commands

```bash
# Run with full tracing
LOG_STDOUT=1 LOG_FILTER=trace cargo test <name> -- --nocapture

# Check logs
cat target/test-logs/<test_name>.log

# Single test
cargo test -p <crate> <test_name> -- --nocapture
```

**Tracing config:** `config/tracing.toml` - Set `log_to_file = true` and adjust filters

---

## Common Imports

```rust
// context-trace
use context_trace::{*, hypergraph::*, path::*};

// context-search
use context_search::{*, search::Searchable};

// context-insert
