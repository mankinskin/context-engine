---
tags: `#guide` `#context-trace` `#context-search` `#debugging` `#testing` `#api`
summary: When running tests, tokens in log output appear as cryptic identifiers like `T0w1`, `T1w1`, `T3w1` instead of meaningful string representations lik...
---

# Token Test Labeling Guide

## Problem Description

When running tests, tokens in log output appear as cryptic identifiers like `T0w1`, `T1w1`, `T3w1` instead of meaningful string representations like `"a"`, `"b"`, `"y"`. This makes debugging and understanding test logs significantly more difficult.

### Example of the Issue

**Bad output (what we see now):**
```
pattern: [T0w1, T1w1, T3w1, T2w1]
start_token=T0w1
popped_token=T6w3
```

**Good output (what we should see):**
```
pattern: ["a", "b", "y", "x"]
start_token="a"
popped_token="xaby"
```

## Root Cause

The token labeling system requires **explicit registration** of the hypergraph with the test tracing infrastructure. When tests create their own graph instances but fail to pass them to `init_test_tracing!()`, the Display implementation for `Token` cannot access the graph data to look up string representations.

### How Token Display Works

1. **Token Display Implementation** (`context-trace/src/graph/vertex/token.rs:275-289`):
```rust
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(any(test, feature = "test-api"))]
        {
            if self.get_string_repr().is_some() {
                // Uses VertexIndex's Display which shows string representation
                return write!(f, "{}", self.index);
            }
        }
        // Fallback: shows T{index}w{width} format
        write!(f, "T{}w{}", self.index, self.width.0)
    }
}
```

2. **String Representation Lookup** (`context-trace/src/graph/vertex/token.rs:114-116`):
```rust
#[cfg(any(test, feature = "test-api"))]
pub fn get_string_repr(&self) -> Option<String> {
    crate::graph::test_graph::get_token_string_from_test_graph(*self.index)
}
```

3. **Thread-Local Graph Registry** (`context-trace/src/graph/test_graph.rs:65-92`):
```rust
thread_local! {
    static TEST_GRAPH: RefCell<Option<Box<dyn GraphStringGetter>>> = RefCell::new(None);
}

pub fn register_test_graph<G: GraphKind + 'static>(graph: &Hypergraph<G>)
where
    G::Atom: std::fmt::Display,
{
    let graph_clone = graph.clone();
    TEST_GRAPH.with(|tg| {
        *tg.borrow_mut() = Some(Box::new(graph_clone));
    });
}

pub fn get_token_string_from_test_graph(index: usize) -> Option<String> {
    TEST_GRAPH.with(|tg| {
        tg.borrow()
            .as_ref()
            .and_then(|graph| graph.get_token_string(index))
    })
}
```

## Solution: Proper Graph Registration

### Option 1: Pass Graph to `init_test_tracing!()` (Recommended)

When creating a graph in your test, pass it to the tracing initialization:

```rust
#[test]
fn my_test() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, c});
    // ... build graph ...
    
    // CORRECT: Pass graph to init_test_tracing
    let _tracing = init_test_tracing!(&graph);
    
    // OR: If using HypergraphRef
    let graph_ref = HypergraphRef::from(graph);
    let _tracing = init_test_tracing!(&graph_ref);
    
    // Now all tokens will show string representations in logs
}
```

### Option 2: Use Pre-Built Test Environments (Already Configured)

Test environments like `Env1` automatically register their graphs:

```rust
#[test]
fn my_test() {
    let Env1 { graph, a, b, c, .. } = &*Env1::get_expected();
    
    // CORRECT: Pass the env's graph reference
    let _tracing = init_test_tracing!(graph);
    
    // Tokens will show string representations
}
```

The `Env1::initialize_expected()` method already calls `register_test_graph(&graph)` internally (see `context-trace/src/tests/env/mod.rs:178`).

### Option 3: Manual Registration (For Special Cases)

If you need more control:

```rust
#[test]
fn my_test() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, c});
    
    // Initialize tracing first
    let _tracing = init_test_tracing!();
    
    // Manually register the graph
    use context_trace::graph::test_graph::register_test_graph;
    register_test_graph(&graph);
    
    // Or use the macro
    register_test_graph!(&graph);
}
```

## Common Mistakes

### ❌ Mistake 1: Not Passing Graph to Tracing
```rust
#[test]
fn find_pattern1() {
    let _tracing = init_test_tracing!();  // ❌ No graph!
    let mut base_graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(base_graph, {a, b, x, y, z});
    // Tokens will show as T0w1, T1w1, etc.
}
```

**Fix:**
```rust
#[test]
fn find_pattern1() {
    let mut base_graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(base_graph, {a, b, x, y, z});
    let _tracing = init_test_tracing!(&base_graph);  // ✅ Pass graph!
    // Tokens will show as "a", "b", "x", "y", "z"
}
```

### ❌ Mistake 2: Wrong Order (Graph After Tracing)
```rust
#[test]
fn my_test() {
    let _tracing = init_test_tracing!();
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, c});
    // ❌ Graph created after tracing init
}
```

**Fix:**
```rust
#[test]
fn my_test() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, c});
    let _tracing = init_test_tracing!(&graph);  // ✅ Correct order
}
```

### ❌ Mistake 3: Forgetting to Register After Graph Modifications
```rust
#[test]
fn my_test() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, c});
    let _tracing = init_test_tracing!(&graph);
    
    // Add more patterns
    let abc = graph.insert_pattern(vec![a, b, c]);
    // ❌ New pattern won't have string repr!
}
```

**Fix:**
```rust
#[test]
fn my_test() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, c});
    
    // Add all patterns before registration
    let abc = graph.insert_pattern(vec![a, b, c]);
    
    let _tracing = init_test_tracing!(&graph);  // ✅ Register complete graph
}
```

## How `init_test_tracing!` Macro Works

The macro has multiple variants (`context-trace/src/logging/tracing_utils/mod.rs:111-185`):

1. **No arguments** - Basic initialization, no graph registration:
```rust
init_test_tracing!()
```

2. **With graph** - Automatically registers graph:
```rust
init_test_tracing!(&graph)
init_test_tracing!(&graph_ref)
```

3. **With graph and config** - Custom tracing config plus graph:
```rust
let config = TracingConfig::default().with_stdout_level("debug");
init_test_tracing!(&graph, config)
```

The macro internally calls:
- `TestTracing::init_with_graph(test_name, graph)` which calls
- `graph.register_test_graph()` (via `AsGraphRef` trait) which calls
- `crate::graph::test_graph::register_test_graph(&graph)`

## Testing Your Fix

After fixing a test, verify the output:

```bash
# Run with trace logging to see token representations
LOG_STDOUT=1 LOG_FILTER=trace cargo test -p context-search find_pattern1 -- --nocapture
```

**Check the log file:**
```bash
cat target/test-logs/find_pattern1.log | head -50
```

You should see:
- ✅ `pattern: ["a", "b", "y", "x"]` instead of `pattern: [T0w1, T1w1, T3w1, T2w1]`
- ✅ `start_token="a"` instead of `start_token=T0w1`
- ✅ `popped_token="xaby"` instead of `popped_token=T6w3`

## Thread Safety and Parallel Tests

The test graph registry uses **thread-local storage** (`thread_local!` macro), which means:
- ✅ Each test thread has its own graph registry
- ✅ Tests can run in parallel without interference
- ✅ No need for locking or synchronization
- ✅ Graph is automatically cleaned up when thread exits

The `TestTracing` struct optionally calls `clear_test_graph()` on drop to ensure cleanup.

## Related Files

- **Token Display:** `context-trace/src/graph/vertex/token.rs:275-289`
- **Graph Registry:** `context-trace/src/graph/test_graph.rs`
- **Tracing Macro:** `context-trace/src/logging/tracing_utils/mod.rs:111-185`
- **Test Trait:** `context-trace/src/logging/tracing_utils/test_tracing.rs:28-60` (AsGraphRef trait)
- **Example Tests:** `context-search/src/tests/search/ancestor.rs` (correct usage)
- **Test String Repr:** `context-trace/src/tests/test_string_repr.rs` (comprehensive examples)

## Migration Checklist

When you encounter a test with poor token labeling:

1. ☐ Locate the `init_test_tracing!()` call
2. ☐ Find the graph variable (e.g., `base_graph`, `graph`, `graph_ref`)
3. ☐ Ensure graph is created and populated BEFORE tracing init
4. ☐ Pass graph to macro: `init_test_tracing!(&graph)`
5. ☐ Run test with `LOG_STDOUT=1` to verify output
6. ☐ Check log file in `target/test-logs/` for readable token names
7. ☐ If using `HypergraphRef`, you can pass either `&graph` or `&graph_ref`

## Key Insight

The token labeling system is **opt-in by design**. Tests must explicitly register their graphs to enable readable token output. This design:
- Keeps the token Display implementation fast (no global state lookup)
- Allows parallel test execution without conflicts (thread-local storage)
- Works with any `GraphKind` (type-erased trait object)
- Automatically handles graph cloning for safe storage
