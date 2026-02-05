# Context Framework

A comprehensive Rust framework for graph-based data structures with 
advanced search, insertion, and traversal capabilities. Built around 
hypergraph data structures with sophisticated path operations and 
bidirectional tracing.

## Framework Components

The context framework consists of four interconnected crates:

- **[context-trace](context-trace/)**: Core graph and tracing functionality
  - Foundational hypergraph data structures
  - Thread-safe graph references (Arc/RwLock)
  - Path operations with accessors and mutators
  - Bidirectional tracing (bottom-up and top-down)
  - Comprehensive cache management

- **[context-search](context-search/)**: Search and traversal operations
  - Policy-driven search operations
  - Configurable traversal strategies (BFT, DFT)
  - Early-terminating foldable operations
  - Pattern matching with partial match handling
  - Resumable search operations

- **[context-insert](context-insert/)**: Graph insertion operations
  - Complex pattern insertion into existing structures
  - Split-join architecture for safe modifications
  - Multi-phase processing (pre/in/post visit modes)
  - Interval management for insertion state tracking
  - Sophisticated caching for split and join operations

- **[context-read](context-read/)**: Reading and expansion operations
  - Ordered recursive hypergraph operations
  - Sequenced tokenized data handling
  - Graph complement operations
  - Expansion chain management
  - Block iteration for sequence processing

## Architecture Overview

The framework follows a layered architecture where each crate builds 
upon the previous ones:

1. **context-trace** provides the foundational graph structures
2. **context-search** adds search and traversal capabilities
3. **context-insert** enables complex graph modifications
4. **context-read** provides high-level reading and expansion operations

```
┌─────────────────────────────────────────────────────────┐
│                    context-read                         │
│         (Builds largest token decompositions)           │
├─────────────────────────────────────────────────────────┤
│                   context-insert                        │
│      (Inserts new nodes maintaining invariants)         │
├─────────────────────────────────────────────────────────┤
│                   context-search                        │
│         (Traverses hierarchy to find matches)           │
├─────────────────────────────────────────────────────────┤
│                   context-trace                         │
│    (Foundational types, graph structure, tracing)       │
└─────────────────────────────────────────────────────────┘
```

### The Reachability Invariant

A crucial property maintained throughout the framework:

> Two nodes have a path between them **if and only if** one is a substring of the other.

This creates a **containment hierarchy** where larger patterns contain smaller 
ones. The graph stores only edges to closest neighbors (transitive reduction), 
yet all substring relationships remain reachable through traversal.

This invariant enables:
- **context-search** to traverse upward from small to large patterns
- **context-insert** to find correct insertion points
- **context-read** to build optimal token decompositions

See [Crate Architecture](../doc/hypergraph-context-model/crate-architecture.md) 
for detailed documentation.

## Key Features

- **Hypergraph Data Structures**: Advanced graph representation with 
  vertex and token management
- **Policy-Based Design**: Configurable behavior through policy objects
- **Thread Safety**: Safe concurrent access through Arc/RwLock wrappers
- **State Management**: Comprehensive caching and state continuation
- **Type Safety**: Strong typing throughout the framework
- **Performance**: Optimized for large-scale graph operations

## Getting Started

Add the required crates to your `Cargo.toml`:

```toml
[dependencies]
context-trace = { path = "context-engine/context-trace" }
context-search = { path = "context-engine/context-search" }
context-insert = { path = "context-engine/context-insert" }
context-read = { path = "context-engine/context-read" }
```

Basic usage example:

```rust
use context_trace::Hypergraph;
use context_search::Searchable;
use context_insert::ToInsertCtx;

// Create a hypergraph
let mut graph = Hypergraph::new();

// Insert patterns
let result = graph.insert(pattern)?;

// Search for sequences
let search_result = graph.find_sequence(vec!["hello", "world"])?;
```

## Development

### Prerequisites

This project requires Rust nightly due to the use of unstable language features
such as `test`, `assert_matches`, `try_blocks`, `slice_pattern`, 
`exact_size_is_empty`, `associated_type_defaults`, and `type_changing_struct_update`.
The `rust-toolchain.toml` file in the repository root automatically selects the 
nightly toolchain when you work in this directory.

### Building and Testing

Each crate can be developed and tested independently:

```bash
# Run all tests
cargo test --workspace

# Generate documentation
cargo doc --workspace --open

# Run specific crate tests
cargo test -p context-trace
cargo test -p context-search
cargo test -p context-insert
cargo test -p context-read
```

## Features

- **test-api**: Enables testing utilities across all crates
- **logging**: Comprehensive tracing and debugging support

## Contributing

Please refer to individual crate READMEs for specific implementation 
details and contribution guidelines.

## License

This project is part of the graph_app repository.