---
tags: `#bug-report` `#context-trace` `#context-read` `#debugging` `#testing` `#api`
summary: Research findings for fixing the 28 compilation errors in context-read. The crate is out of sync with context-trace API changes from trait consolid...
---

# Context-Read API Research - Migration Guide

**Date:** 2025-12-06  
**Component:** context-read, context-trace  
**Related:** 20251206_BUG_CONTEXT_READ_API_MISMATCHES.md  
**Status:** Research complete, fixes needed

## Summary

Research findings for fixing the 28 compilation errors in context-read. The crate is out of sync with context-trace API changes from trait consolidation and method renames. This document provides the correct API alternatives for all missing/renamed items.

## API Migration Map

### 1. Type Corrections

| context-read (Wrong) | context-trace (Correct) | Location |
|---------------------|------------------------|----------|
| `NewAtomndex` | `NewAtomIndex` | `crates/context-trace/src/graph/vertex/atom.rs:85` |
| `NewAtomIndex` | ✅ EXISTS | `pub(crate) enum` |
| `NewAtomIndices` | ✅ EXISTS | `pub(crate) type NewAtomIndices = Vec<NewAtomIndex>` (line 134) |
| `PostfixIterator` | ✅ EXISTS | `crates/context-trace/src/trace/child/bands/mod.rs` |

**Fix for NewAtomIndex/NewAtomIndices:**
```rust
// Types are pub(crate) in context-trace, need to be re-exported or context-read needs access
use context_trace::graph::vertex::atom::{NewAtomIndex, NewAtomIndices};
```

**Issue:** These types are `pub(crate)` - context-read cannot access them directly!

**Solution Options:**
1. Make types `pub` in context-trace (breaks encapsulation)
2. Provide public wrapper API in context-trace
3. Move context-read into context-trace workspace (same crate)
4. Redesign context-read to not use internal types

### 2. Method Renames

| Old Method | New Method | Type | Location |
|-----------|-----------|------|----------|
| `root_child()` | `graph_root_child()` | Trait method | `path/accessors/child/root.rs:46` |
| `vertex_mut()` | `vertex()` | Token → &VertexData | Token doesn't have mut variant |
| `to_new_atom_indices` | `to_new_Atom_indices` | Trait method | Capital 'A' |
| `new_directed()` | **REMOVED** | RootedRolePath | Use `new()` instead |
| `start_index()` | **REMOVED** | RootedRangePath | Access directly? |

#### `root_child()` → `graph_root_child()`
**Context-trace API:**
```rust
// trait GraphRootChild<R: PathRole>
fn graph_root_child<G: HasGraph>(&self, trav: &G) -> Token {
    *trav.graph().expect_child_at(
        <_ as GraphRootChild<R>>::graph_root_child_location(self),
    )
}
```

**Fix:**
```rust
// OLD (context-read)
let root = self.link.root_postfix.root_child(trav);

// NEW
let root = self.link.root_postfix.graph_root_child(trav);
```

#### `vertex_mut()` → Need Alternative
**Context-trace has:**
- `Token::vertex(&self, graph) -> &VertexData` (immutable)
- `TraceCache::get_vertex_mut(&mut self, key) -> Option<&mut VertexEntry>` (private)

**Context-read uses:**
```rust
let vertex = (*root).vertex_mut(&mut graph);  // ❌ Doesn't exist
```

**Fix Options:**
1. Use `vertex()` if only reading
2. Get mutable access via graph directly: `graph.get_vertex_mut(root.index)?`
3. Use TraceCache if in search context

#### `to_new_atom_indices` → `to_new_Atom_indices`
**Issue:** Capitalization error - trait expects capital 'A'

**Fix:**
```rust
// Trait definition
pub trait ToNewAtomIndices: Debug {
    fn to_new_Atom_indices<'a: 'g, 'g, G: HasGraphMut<Kind = BaseGraphKind>>(
        //       ^^^^^ Capital A
        self,
        graph: &'a mut G,
    ) -> NewAtomIndices;
}

// Implementation in context-read
impl ToNewAtomIndices for NewAtomIndices {
    fn to_new_Atom_indices<'a: 'g, 'g, G: HasGraphMut<Kind = BaseGraphKind>>(
        //       ^^^^^ Match trait signature
        self,
        _graph: &'a mut G,
    ) -> NewAtomIndices {
        self
    }
}
```

#### `new_directed()` → Use `new()`
**Old API (removed):**
```rust
PatternEndPath::new_directed::<Right>(known.clone())
```

**Current API:**
```rust
// RootedRolePath::new
pub fn new(
    root: impl Into<Root>,
    role_path: RolePath<R, N>,
) -> Self
```

**Fix:**
```rust
// Need to construct RolePath first, then call new()
// May need to use rooted_path! macro instead
```

### 3. Private API Access

#### `root_entry` field
**Issue:** `SubPath::root_entry` is `pub(crate)`

**Context-trace API:**
```rust
pub struct SubPath<N = ChildLocation> {
    pub(crate) root_entry: usize,  // ❌ Private
    pub(crate) path: Vec<N>,
}

// Public accessor via trait
impl<R: PathRole, N> HasRootChildIndex<R> for SubPath<N> {
    fn root_child_index(&self) -> usize {
        self.root_entry
    }
}
```

**Fix:**
```rust
// OLD (context-read)
let intersection_start = self.link.root_postfix.root_entry;

// NEW - Use trait method
use context_trace::path::structs::rooted::role_path::HasRootChildIndex;
let intersection_start = self.link.root_postfix.root_child_index();
```

#### `new_atom_indices()` method
**Issue:** Method is `pub(crate)`, not accessible from context-read

**Context-trace API:**
```rust
impl<G: GraphKind> Hypergraph<G> {
    pub(crate) fn new_atom_indices(
        &mut self,
        sequence: impl IntoIterator<Item = G::Atom>,
    ) -> NewAtomIndices {
        // ...
    }
}
```

**Fix Options:**
1. Make method `pub` in context-trace
2. Use `insert_atoms()` instead (public API)
3. Redesign context-read to not need this

### 4. Removed Functionality

#### `retract()` method and module
**Missing:** `path::mutators::move_path::retract::Retract`

**Status:** Module/method appears to be removed entirely from context-trace

**Investigation needed:** Was this functionality replaced? Is it in a different module?

#### `PrefixCommand`
**Missing:** `trace::command::PrefixCommand`

**Status:** Module `command` not found in trace/

**Investigation needed:** Was command system removed or renamed?

#### Iterator methods on Token
**Missing from Token:**
- `postfix_iter()`
- `prefix_path()`

**Found in:** `trace::child::bands::HasTokenRoleIters` trait

**Fix:**
```rust
// trait HasTokenRoleIters: ToToken
fn postfix_iter<'a, G: HasGraph + 'a>(
    &self,
    trav: G,
) -> PostfixIterator<'a, G>

fn prefix_path<G>(
    &self,
    trav: &G,
    prefix: Token,
) -> IndexStartPath
```

**Context-read needs:**
```rust
// Import trait
use context_trace::trace::child::bands::HasTokenRoleIters;

// Then can call:
let iter = token.postfix_iter(ctx);
let path = token.prefix_path(&ctx, overlap);
```

#### `start_index()` method
**Missing from:** `RootedRangePath<Pattern>`

**Fix:** Need to investigate how to get start index from range path. May be:
- `path.range.start`
- `path.start_location()`
- Different API pattern

### 5. Type Inference Failures

Most inference failures are due to missing iterator/closure types. Once the API is fixed, these should resolve naturally. If not:

```rust
// Example fix for closure type inference
.take_while(|result: &Result<_, _>| result.is_continue())

// Example for tuple destructuring
.map(|(postfix_location, postfix): (ChildLocation, Token)| { ... })
```

## Migration Priority

### Critical (Blocks compilation):
1. ✅ Fix type names: `NewAtomndex` → `NewAtomIndex`
2. ✅ Fix method name: `to_new_atom_indices` → `to_new_Atom_indices`
3. ✅ Replace `root_child()` → `graph_root_child()`
4. ✅ Replace `root_entry` → `root_child_index()`

### High (API changes):
5. ⚠️ Fix `vertex_mut()` calls - determine correct alternative
6. ⚠️ Fix `new_directed()` - use `new()` or macro
7. ⚠️ Fix `new_atom_indices()` - make public or use alternative
8. ⚠️ Import `HasTokenRoleIters` trait for iterator methods

### Medium (Removed features):
9. ❓ Investigate `retract` replacement
10. ❓ Investigate `PrefixCommand` replacement
11. ❓ Find `start_index()` alternative

### Low (Should auto-resolve):
12. Type inference failures (fix after above)

## Architectural Questions

### Question 1: Is context-read still needed?
**Evidence:**
- Not tested (no test runs)
- Significantly outdated
- Many private API dependencies
- No recent commits

**Options:**
1. Update and maintain
2. Mark as deprecated
3. Merge into context-trace
4. Remove entirely

### Question 2: Should internal types be exposed?
**Current:** `NewAtomIndex`, `NewAtomIndices` are `pub(crate)`

**Context-read needs:** These types in public API

**Options:**
1. Make types `pub` (breaks encapsulation)
2. Create public wrapper types
3. Redesign context-read to avoid need

### Question 3: What happened to retract/command?
**Missing modules:**
- `path::mutators::move_path::retract`
- `trace::command`

**Investigation needed:**
- Were these removed intentionally?
- Were they moved to different modules?
- Were they replaced by different APIs?

## Next Steps

1. **Decide on context-read status**
   - Still actively developed? → Full migration
   - Deprecated? → Mark and archive
   - Remove? → Delete crate

2. **If migrating, prioritize:**
   - Type visibility changes in context-trace (NewAtomIndex, etc.)
   - Simple renames (root_child, to_new_Atom_indices)
   - API replacements (vertex_mut, new_directed, start_index)
   - Missing functionality (retract, command modules)

3. **Document decisions in:**
   - `QUESTIONS_FOR_AUTHOR.md` - architectural questions
   - `CHEAT_SHEET.md` - API updates
   - `agents/guides/` - migration patterns

## Related Files

- **Bug report:** `agents/bug-reports/20251206_BUG_CONTEXT_READ_API_MISMATCHES.md`
- **Trait consolidation:** `agents/implemented/20251122_PHASE1_HAS_TRAIT_CONSOLIDATION.md`
- **Method naming:** `agents/implemented/20251123_PHASE3_WEEK5_METHOD_NAMING.md`
- **API reference:** `agents/CHEAT_SHEET.md`

---

**Tags:** context-read, api-migration, research, visibility, trait-methods
