# Context Framework Cheat Sheet

> **Quick reference for common operations, API patterns, and gotchas when working with context-trace, context-search, and context-insert.**

## Table of Contents
1. [Type Quick Reference](#type-quick-reference)
2. [Common Patterns](#common-patterns)
3. [API Changes (Recent)](#api-changes-recent)
4. [Gotchas & Common Mistakes](#gotchas--common-mistakes)
5. [Testing Patterns](#testing-patterns)
6. [Debug Tips](#debug-tips)

---

## Type Quick Reference

### Graph Types
```rust
// Core graph types
Hypergraph<G: GraphKind>        // Base graph structure
HypergraphRef                    // Arc<RwLock<Hypergraph>> - thread-safe wrapper
BaseGraphKind                    // Default graph kind

// Token and indexing
Token { index: VertexIndex, width: TokenWidth }
VertexIndex                      // usize wrapper
AtomPosition                     // usize wrapper
PatternId                        // UUID for pattern identification
```

### Path Types (context-trace)
```rust
// Role-based paths (different roles for different traversal contexts)
RolePath<R: RangeRole>          // Generic path with role
IndexRangePath                   // Complete paths with indices
PatternRangePath                 // Pattern-level paths
RootedRolePath<R>               // Path with explicit root

// Path construction
RolePath::new(locations)         // From child locations
path.to_rooted(root)            // Add root to path
```

### Search Types (context-search)
```rust
// New unified API (since recent refactor)
Response {                       // Unified search result
    cache: TraceCache,          // Trace cache from search
    end: EndState               // Terminal state (complete/incomplete)
}

// EndState contains (private fields, use accessor methods):
PathEnum::Complete(IndexRangePath)  // Full match
PathEnum::Range / Postfix / Prefix  // Partial matches

// Result handling
response.is_complete()           // Check if search succeeded
response.expect_complete(msg)    // Unwrap complete path (panics if incomplete)
response.as_complete()           // Try get complete path (returns Option)
response.root_token()            // Get root token from path
response.query_pattern()         // Get pattern path from cursor
response.cursor_position()       // Get atom position from cursor

// Search execution
Searchable::search::<K>(query, graph) -> Result<Response, ErrorState>
graph.find_ancestor(query) -> Result<Response, ErrorReason>
```

### Insert Types (context-insert)
```rust
// Insertion context
InsertCtx<G>                     // Main insertion context
InitInterval {                   // Initialization for incomplete searches
    root: Token,
    cache: TraceCache,
    end_bound: AtomPosition
}

// Conversion from search results
InitInterval::from(response)     // Convert incomplete search to insert init

// Insertion methods
graph.insert(pattern)            // Insert foldable pattern
graph.insert_init(extract, init) // Insert with initialization interval
```

### Trace & Cache Types
```rust
// Tracing
TraceCache                       // Cache of traced vertices
TraceCtx<G>                     // Tracing context with graph reference
VertexCache                      // Per-vertex cache data

// Directions
Left, Right                      // Direction types
Direction                        // Trait for directional operations
```

---

## Common Patterns

### Pattern 1: Create and Search a Graph
```rust
use context_trace::{Hypergraph, HypergraphRef};
use context_search::{Searchable, Find};

// Create graph
let mut graph = Hypergraph::<BaseGraphKind>::default();

// Insert atoms (use test macros for convenience)
insert_atoms!(graph, {a, b, c, d});

// Insert patterns
insert_patterns!(graph,
    (ab, ab_id) => [a, b],
    (cd, cd_id) => [c, d],
    (abcd, abcd_id) => [ab, cd]
);

// Search for sequence
let query = vec![a, b, c, d];
let result = Searchable::search::<AncestorSearchTraversal>(query, graph.clone())?;

// Handle result
if result.is_complete() {
    let path = result.expect_complete("should be complete");
    println!("Found at: {:?}", path);
} else {
    // Search incomplete - can convert to InitInterval for insertion
    let init = InitInterval::from(result);
}
```

### Pattern 2: Insert Missing Patterns
```rust
use context_insert::{ToInsertCtx, InitInterval};

// Search first
let query = vec![a, b, c];
let result = graph.find_ancestor(query)?;

if !result.is_complete() {
    // Convert incomplete result to insertion interval
    let init = InitInterval::from(result);
    
    // Perform insertion
    let insert_result = graph.insert_init(extract, init);
}
```

### Pattern 3: Iterate Over Graph Vertices
```rust
// Get vertex by index
let vertex = graph.expect_vertex(token.index);
let vertex_data = graph.vertex_data(token.index);

// Access children
for child in vertex.children() {
    println!("Child: {:?}", child);
}

// Access parents
for parent in vertex.parents() {
    println!("Parent: {:?}", parent);
}
```

### Pattern 4: Work with Paths
```rust
use context_trace::path::*;

// Create rooted path
let role_path = RolePath::new(child_locations);
let rooted = role_path.to_rooted(root_token);

// Navigate path
let token = rooted.root_parent();      // Get root token
let position = path.start_position();   // Get start position
```

### Pattern 5: Test with Tracing
```rust
#[test]
fn my_test() {
    // Initialize test tracing (from context-trace)
    let _tracing = context_trace::init_test_tracing!();
    
    // Test code here
    // Logs will be written to target/test-logs/my_test.log on failure
}
```

---

## API Changes (Recent)

### ⚠️ Major Breaking Changes

**Old API (removed):**
```rust
// Separate types for complete/incomplete
CompleteState               // ❌ REMOVED
IncompleteState            // ❌ REMOVED

// Old search pattern
let result = searchable.search::<K>(trav)?;
match result {
    CompleteState::try_from(response) => {
        Ok(complete) => // handle complete
        Err(incomplete) => // handle incomplete
    }
}
```

**New API (current):**
```rust
// Unified Response type
Response {
    cache: TraceCache,      // Public field
    end: EndState          // Public field (but inner fields private)
}

// New search pattern
let result: Result<Response, ErrorState> = 
    Searchable::search::<K>(query, graph)?;

if result.is_complete() {
    let path = result.expect_complete("msg");
    // Use path.root_parent() to get Token
} else {
    // Use Response directly for incomplete handling
    let init = InitInterval::from(result);
}
```

### Key Accessor Methods (Response)
```rust
// Checking completeness
response.is_complete() -> bool
response.as_complete() -> Option<&IndexRangePath>

// Getting data (safe, returns copies/refs)
response.root_token() -> Token              // Root token from path
response.query_pattern() -> &PatternRangePath
response.query_cursor() -> &PatternCursor
response.cursor_position() -> AtomPosition

// Unwrapping (panics if incomplete)
response.unwrap_complete() -> IndexRangePath
response.expect_complete(msg) -> IndexRangePath
```

### Search Method Signature Changes
```rust
// Old (took traversal, returned bare result)
searchable.search::<K>(traversal)

// New (takes graph, returns Result)
Searchable::search::<K>(query, graph) -> Result<Response, ErrorState>
```

### Searchable Trait Location
```rust
// Now exported from context-search publicly
use context_search::Searchable;

// Found in: context-search/src/state/start.rs
// Re-exported in: context-search/src/lib.rs
```

### Compare Module Naming (Nov 2024)
```rust
// ❌ OLD (naming collision with cursor states)
enum TokenMatchState {           // Collided with PathCursor<_, Matched>
    Match(CompareState),
    Mismatch(EndState),
}
enum CompareResult {
    MatchState(TokenMatchState), // Ambiguous name
    Prefixes(CompareQueue),
}

// ✅ NEW (clearer, no collision)
enum CandidateResult {           // Describes what it is: result of comparing a candidate
    Match(CompareState),
    Mismatch(EndState),
}
enum CompareResult {
    CompareResult(CandidateResult),  // More descriptive
    Prefixes(CompareQueue),
}

// Import path unchanged:
use crate::compare::state::{CandidateResult, CompareResult};
```

---

## Gotchas & Common Mistakes

### 1. ❌ Direct Field Access on EndState
```rust
// ❌ WRONG - fields are private!
let path = response.end.path;      // Error: private field
let cursor = response.end.cursor;  // Error: private field

// ✅ CORRECT - use accessor methods
let token = response.root_token();
let cursor = response.query_cursor();
let pattern = response.query_pattern();
```

### 2. ❌ Forgetting .root_parent() After expect_complete()
```rust
// ❌ WRONG - expect_complete() returns IndexRangePath, not Token
let token: Token = response.expect_complete("msg");  // Type error!

// ✅ CORRECT - call root_parent() to get Token
let token: Token = response.expect_complete("msg").root_parent();
```

### 3. ❌ Incorrect InitInterval Conversion
```rust
// ❌ WRONG - old API pattern
let incomplete: IncompleteState = result.try_into().unwrap();  // Type doesn't exist

// ✅ CORRECT - use From<Response>
let init = InitInterval::from(response);
```

### 4. ❌ Moving Response Before Borrowing
```rust
// ❌ WRONG - borrow checker error
let cache = response.cache;         // Moves cache
let token = response.root_token();  // Error: response partially moved

// ✅ CORRECT - borrow before moving, or reorder
let token = response.root_token();  // Borrows first
let cache = response.cache;         // Then move
```

### 5. ❌ Wrong Traversal Type Parameter
```rust
// ❌ WRONG - using wrong traversal kind
Searchable::search::<BreadthFirst>(query, graph)  // BreadthFirst is not TraversalKind

// ✅ CORRECT - use actual traversal implementations
Searchable::search::<InsertTraversal>(query, graph)
Searchable::search::<AncestorSearchTraversal>(query, graph)
```

### 6. ❌ Forgetting to Check is_complete()
```rust
// ❌ WRONG - assuming search always succeeds
let path = response.expect_complete("failed");  // Panics on incomplete!

// ✅ CORRECT - check first
if response.is_complete() {
    let path = response.expect_complete("msg");
} else {
    // Handle incomplete case
    let init = InitInterval::from(response);
}
```

### 7. ❌ Using Old init_tracing() Function
```rust
// ❌ WRONG - function was removed
#[test]
fn my_test() {
    init_tracing();  // Error: not found

// ✅ CORRECT - use macro from context-trace
#[test]
fn my_test() {
    let _tracing = context_trace::init_test_tracing!();
```

---

## Testing Patterns

### Test Initialization
```rust
#[test]
fn my_test() {
    // Always initialize tracing for debugging
    let _tracing = context_trace::init_test_tracing!();
    
    // Create test graph
    let mut graph = HypergraphRef::default();
    
    // Use helper macros
    insert_atoms!(graph, {a, b, c});
    insert_patterns!(graph, (ab, ab_id) => [a, b]);
}
```

### Creating RootedRolePaths
```rust
use context_trace::rooted_path;

// IndexRangePath with simple entry/exit
let path: IndexRangePath = rooted_path!(Range: root, start: 0, end: 2);

// PatternRangePath
let pattern = Pattern::from(vec![a, b, c]);
let path: PatternRangePath = rooted_path!(Range: pattern, start: 0, end: 2);

// With nested child locations (supports multiple on each side)
let path: IndexRangePath = rooted_path!(Range: root,
    start: (0, [child_loc1, child_loc2]),
    end: (2, [child_loc3])
);

// Single-role paths
let start: IndexStartPath = rooted_path!(Start: root, 0);
let end: PatternEndPath = rooted_path!(End: pattern, 1);

// With children
let start: IndexStartPath = rooted_path!(Start: root, (0, [child_loc]));
```

**Before/After comparison:**
```rust
// ❌ OLD WAY - verbose and error-prone
let pattern_path = PatternRangePath::new(
    Pattern::from(vec![a, b, c]),
    RolePath::new_empty(0),
    RolePath::new_empty(2),
);

// ✅ NEW WAY - concise and clear
let pattern_path: PatternRangePath = rooted_path!(
    Range: Pattern::from(vec![a, b, c]),
    start: 0,
    end: 2
);
```

**Note:** Type annotations are usually required due to `Into<Root>` ambiguity. The macro is exported from `context_trace` and available in all tests.

### Assertion Patterns
```rust
use assert_matches::assert_matches;

// Check search results
assert_matches!(result, Ok(ref r) if r.is_complete());
assert!(!result.is_complete());

// Check specific values
assert_eq!(response.root_token(), expected_token);
assert_eq!(response.cursor_position(), AtomPosition(3));

// Pattern matching on Ok results
match result {
    Ok(ref response) if response.is_complete() => {
        assert_eq!(response.root_token(), expected);
    },
    _ => panic!("Expected complete result"),
}
```

### Building Test Caches
```rust
// Use build_trace_cache! macro for expected cache structures
let expected = build_trace_cache!(
    token => (
        BU { pos => child -> (pattern_id, sub_index) },
        TD {},
    ),
);
```

---

## Debug Tips

### 0. Tracing Configuration File (Preferred)

**Use TOML config files for tracing format configuration:**

```toml
# Create config/tracing.toml in workspace root
# See config/tracing.toml.example for full documentation

[span_enter]
show = true                # Show/hide span enter messages
show_fn_signature = true   # Show function signatures
show_fields = true         # Show span fields (parameters)

[span_close]
show = false               # Disable span close noise
show_timing = false        # Hide time.busy/time.idle

[panic]
show = true                # Show panic log events
show_message = true        # Include panic message details
show_stderr = true         # Print 🔥 PANIC: to stderr

enable_indentation = true  # Box-drawing chars (┬─, │, └─)
show_file_location = true  # Show file:line
enable_ansi = true         # Enable colors
```

**Config file search order:**
1. `TRACING_CONFIG` env var (absolute or relative to workspace root)
2. `{workspace_root}/config/tracing.toml`
3. `{workspace_root}/.tracing.toml` (legacy)
4. `{workspace_root}/tracing.toml` (legacy)
5. `./config/tracing.toml` (current directory)
6. `./.tracing.toml` (current directory, legacy)
7. `./tracing.toml` (current directory, legacy)
8. `~/.config/tracing.toml`
9. Falls back to environment variables (see below)

**Examples:**
```bash
# Use custom config file
TRACING_CONFIG=config/my-config.toml cargo test

# Override via environment (takes precedence over config file if no file found)
TRACING_SPAN_CLOSE_SHOW=0 cargo test

# Mix config file + env overrides
# Config file sets defaults, env vars can still override
```

**Benefits over environment variables:**
- Object notation (hierarchical structure)
- Easy to share/version control
- IDE-friendly (syntax highlighting, validation)
- Less environment variable clutter

### 1. View Test Logs
```bash
# Failed tests preserve logs automatically
cat target/test-logs/my_test_name.log

# Run with stdout logging
RUST_TEST_LOG_STDOUT=1 cargo test my_test -- --nocapture

# Adjust log level
RUST_LOG=debug cargo test my_test -- --nocapture
RUST_LOG=context_search::search=trace cargo test
```

### 2. Structured Tracing (Preferred Pattern)
```rust
// Use tracing crate with structured fields for better log analysis
use tracing::{trace, debug, info};

// ❌ DON'T: String formatting in logs
debug!("Comparing tokens: path={:?}, query={:?}", path_token, query_token);

// ✅ DO: Structured fields for parsing and filtering
tracing::debug!(
    path_token = path_token.index,
    query_token = query_token.index,
    cursor_pos = %cursor.atom_position,
    "comparing candidate tokens"
);

// Benefits:
// - Easy to filter: LOG_FILTER=context_search[cursor_pos=5]
// - Machine-readable logs
// - Better performance (no string allocation)

// Field syntax:
// field = value         // Display formatting
// field = ?value        // Debug formatting
// field = %value        // Display trait
// "message"            // Message string (always last)
```

### 3. Tracing in Tests
```rust
// Basic usage - add at start of test function (REQUIRED for tracing output)
let _tracing = init_test_tracing!();

// With test graph registration - tokens show string representations in logs
let graph = Hypergraph::default();
// ... build graph ...
let _tracing = init_test_tracing!(&graph);  // or just 'graph' for HypergraphRef

// With custom config
let config = TracingConfig::default().with_stdout_level("debug");
let _tracing = init_test_tracing!(config);

// With both graph and config
let _tracing = init_test_tracing!(&graph, config);

// Control log output with environment variables:
// LOG_STDOUT=1                 - Send logs to terminal (not just file)
// LOG_FILTER=trace             - Set global log level
// LOG_FILTER=debug             - Module-specific level
// LOG_FILTER=context_search::compare=trace,context_trace=debug

// Formatting control - use config/tracing.toml file (recommended) or environment variables:
// 
// Nested configuration (environment variables):
// TRACING_SPAN_ENTER_SHOW=0              - Hide all span enter messages
// TRACING_SPAN_ENTER_SHOW_FN_SIGNATURE=0 - Hide function signatures
// TRACING_SPAN_ENTER_SHOW_FIELDS=0       - Hide span fields
// TRACING_SPAN_CLOSE_SHOW=0              - Hide all span close messages
// TRACING_SPAN_CLOSE_SHOW_TIMING=0       - Hide timing info
// TRACING_PANIC_SHOW=0                   - Disable panic logging
// TRACING_PANIC_SHOW_MESSAGE=0           - Hide panic message details
// TRACING_PANIC_SHOW_STDERR=0            - Don't print 🔥 PANIC: to stderr
//
// Legacy flat names (still supported):
// TRACING_SHOW_FN_SIGNATURE=1     - Show function signatures for spans
// TRACING_SHOW_SPAN_FIELDS=1      - Show span fields (parameters, etc.)
// TRACING_SHOW_SPAN_TIMING=1      - Show time.busy/time.idle on span close
//
// Other options:
// TRACING_ENABLE_INDENTATION=1    - Use box-drawing chars (┬─, │, └─)
// TRACING_SHOW_FILE_LOCATION=1    - Show file:line for events
// TRACING_ENABLE_ANSI=1           - Enable color output

// Example: Minimal output (no decorations)
// TRACING_SPAN_ENTER_SHOW_FN_SIGNATURE=0 TRACING_ENABLE_INDENTATION=0 cargo test

// Example: Disable span close noise
// TRACING_SPAN_CLOSE_SHOW=0 cargo test

// Example: Disable panic logging
// TRACING_PANIC_SHOW=0 cargo test

// Example: Events only (no span messages)
// TRACING_SPAN_ENTER_SHOW=0 TRACING_SPAN_CLOSE_SHOW=0 cargo test

// Configure in .cargo/config.toml or ~/.cargo/config.toml (or use config/tracing.toml - preferred):
// [env]
// TRACING_SPAN_CLOSE_SHOW = "0"
// TRACING_PANIC_SHOW = "0"
// TRACING_ENABLE_INDENTATION = "1"

// Or in code:
let config = TracingConfig::default()
    .show_fn_signature(false)
    .enable_indentation(false)
    .show_span_timing(false);
let _tracing = init_test_tracing!(config);

// Example test run:
// LOG_STDOUT=1 LOG_FILTER=trace cargo test -p context-search test_name -- --nocapture

// Logs automatically saved to: target/test-logs/<test_name>.log
// Log files deleted on test success, preserved on failure
// Test graph automatically cleaned up on test end
```

### 4. Pretty-Print Debug Output
```rust
use context_trace::logging::pretty;

// Format structs for logging
debug!("Response: {}", pretty(&response));
println!("Cache: {:#?}", response.cache);  // Use {:#?} for pretty debug
```

### 3. Check Response State
```rust
// Add this to understand why a search stopped
if !response.is_complete() {
    eprintln!("Search incomplete at position: {:?}", 
              response.cursor_position());
    eprintln!("Current pattern: {:?}", 
              response.query_pattern());
}
```

### 4. Validate Cache Contents
```rust
// Check what's in the cache
for (token, vertex_cache) in response.cache.entries.iter() {
    println!("Token {}: {:?}", token.index, vertex_cache);
}
```

### 5. Compare Test Expectations
```rust
// When tests fail with complex structs, use pretty_assertions
use pretty_assertions::assert_eq;

assert_eq!(actual, expected);  // Shows diff in color
```

---

## Quick Command Reference

```bash
# Build and test specific crate
cargo build -p context-search
cargo test -p context-search

# Run specific test
cargo test -p context-insert index_prefix1

# Run with output
cargo test -p context-search -- --nocapture

# Check compilation without running
cargo check -p context-trace

# Generate and open docs
cargo doc -p context-search --open

# Format code
cargo fmt --package context-search

# Clippy lints
cargo clippy -p context-trace

# Run all tests in workspace
cargo test --workspace
```

---

## Common Import Patterns

```rust
// Context-trace (core types)
use context_trace::{
    Hypergraph, HypergraphRef,
    Token, VertexIndex, AtomPosition,
    TraceCache, TraceCtx,
    IndexRangePath, PatternRangePath,
    Direction, Left, Right,
};

// Context-search (search operations)
use context_search::{
    Searchable, Find,
    Response, ErrorState,
    AncestorPolicy,
    TraversalKind,
};

// Context-insert (insertion operations)
use context_insert::{
    ToInsertCtx, InsertCtx,
    InitInterval, IntervalGraph,
    InsertResult,
};

// Test utilities (in test modules)
#[cfg(test)]
use context_trace::{insert_atoms, insert_patterns, build_trace_cache};
```

---

## Architecture Reminder

```
context-trace (foundation)
    ↓ (provides graph & paths)
context-search (search & traversal)
    ↓ (provides search results)
context-insert (modifications)
    ↓ (provides high-level ops)
context-read (reading & expansion)
```

Each layer depends on the ones below it. When debugging:
1. Check context-trace types and graph structure first
2. Then context-search search logic and results
3. Finally context-insert insertion behavior
