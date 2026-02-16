---
tags: `#plan` `#context-trace` `#context-search` `#context-insert` `#testing` `#refactoring` `#api`
summary: Refactoring the graph from `Arc<RwLock<Hypergraph>>` to `Arc<Hypergraph>` with per-vertex interior mutability using DashMap and per-vertex RwLock.
status: 📋
---

# Interior Mutability Refactor - Status Report

## Summary
Refactoring the graph from `Arc<RwLock<Hypergraph>>` to `Arc<Hypergraph>` with per-vertex interior mutability using DashMap and per-vertex RwLock.

## Completed Work

### 1. context-trace (COMPILES ✓)
- Added DashMap dependency
- Created `VertexEntry` wrapper with per-vertex `RwLock`
- Refactored `Hypergraph` to use `DashMap<VertexIndex, VertexEntry>`
- Changed all mutation methods from `&mut self` to `&self`
- Simplified `HypergraphRef` from `Arc<RwLock<Hypergraph>>` to `Arc<Hypergraph>`
- Updated `VertexSet` trait for callbacks and owned returns
- Updated `HasGraph` trait

### 2. context-search (COMPILES ✓)
- Updated `expect_vertex` → `expect_vertex_data` calls
- Added type annotations where needed
- Fixed import paths

### 3. context-insert (8 ERRORS REMAINING)
Major changes completed:
- `ToInsertCtx` now requires `HasGraph` instead of `HasGraphMut`
- `LockedFrontierCtx` now holds `&'a Hypergraph` instead of write guard
- `interval/init.rs` uses `&G` instead of `&mut G`
- `NodeTraceCtx` and `PatternTraceCtx` now own their data (no lifetime params)
- `PatternJoinCtx` now owns its data
- `ModeCtx` trait updated with GATs for flexible lifetimes
- Updated all `expect_vertex` → `expect_vertex_data` calls
- Updated all `position_splits` calls to use `.iter()`

## Remaining Errors (8)

### 1. Type Mismatch: `&VertexData` vs `VertexData`
File: `split/vertex/mod.rs:232`
```rust
let output = self.global_splits::<N>(end_pos, node);
// node is VertexData (owned), but method expects &VertexData
```
**Fix:** Update `global_splits` signature or pass `&node`

### 2. Borrowed Value Doesn't Live Long Enough (2 occurrences)
Files: 
- `interval/partition/info/range/mod.rs:57`
- `join/partition/info/pattern_info.rs:98`

Pattern context is owned but we're trying to borrow `.pattern` across an expression boundary.
**Fix:** Clone or restructure to avoid cross-expression borrows

### 3. Parameter `R` May Not Live Long Enough (2 occurrences)
File: `join/partition/info/mod.rs:42,48`
```rust
pub fn into_joined_patterns<'a>(...) -> JoinedPatterns<R>
```
**Fix:** Add `R: 'static` bound or restructure

### 4. Use of Moved Value `ctx` (2 occurrences)
File: `split/cache/vertex.rs:70,113`

`NodeTraceCtx` doesn't implement `Copy` but is used multiple times in loops.
**Fix:** Already has `.clone()` in some places; add `.clone()` to remaining uses

### 5. Cannot Move Out of Closure Capture
File: `split/trace/states/mod.rs:45`
```rust
let node = graph.expect_vertex_data(index);
// Later: node.child_patterns() moves node inside closure
```
**Fix:** Use `node.child_patterns().clone()` or restructure

## Key Architectural Changes

### Before (External Mutability)
```rust
type HypergraphRef = Arc<RwLock<Hypergraph>>;

// Mutations required &mut self
fn insert_pattern(&mut self, pattern: impl IntoPattern) -> Token

// HasGraphMut provided mutable access
trait HasGraphMut: HasGraph {
    fn graph_mut(&mut self) -> Self::GuardMut<'_>;
}
```

### After (Interior Mutability)
```rust
type HypergraphRef = Arc<Hypergraph>;

// Mutations use &self (interior mutability via per-vertex locks)
fn insert_pattern(&self, pattern: impl IntoPattern) -> Token

// HasGraph is sufficient - mutations happen through &self
trait HasGraph {
    fn graph(&self) -> Self::Guard<'_>;
}
```

### Context Types Now Own Data
```rust
// Before: borrowed references
pub struct NodeTraceCtx<'p> {
    pub patterns: &'p ChildPatterns,
    pub index: Token,
}

// After: owned data
pub struct NodeTraceCtx {
    pub patterns: ChildPatterns,
    pub index: Token,
}
```

## Next Steps
1. Fix the 8 remaining compilation errors
2. Run all tests to verify behavior
3. Test that string representations show correctly during inserts (original issue)
4. Clean up any remaining warnings

## Benefits Achieved
- String representations will work during inserts (reads won't be blocked by writes)
- More granular locking (per-vertex instead of whole-graph)
- Simpler API (no more `&mut` required for mutations)
