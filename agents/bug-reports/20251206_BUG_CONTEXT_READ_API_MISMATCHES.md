# Bug Report: context-read API Mismatches (28 Compilation Errors)

**Date:** 2025-12-06  
**Component:** context-read  
**Severity:** Critical - Crate does not compile  
**Status:** Investigating

## Summary

The context-read crate has 28 compilation errors indicating it's significantly out of sync with the current context-trace API. Errors span missing types, renamed methods, private field access, and removed functionality.

## Error Categories

### 1. Missing Imports (2 errors)
**File:** `crates/context-read/src/complement.rs:48-50`

```rust
use context_trace::{
    path::mutators::move_path::retract::Retract,  // ❌ retract module not found
    trace::{
        command::PrefixCommand,  // ❌ command module not found
```

**Error messages:**
- `could not find 'retract' in 'move_path'`
- `could not find 'command' in 'trace'`

### 2. Type Name Errors (7 errors)
**Files affected:**
- `crates/context-read/src/sequence/block_iter.rs`
- `crates/context-read/src/sequence/mod.rs`
- `crates/context-read/src/expansion/chain/expand.rs`

**Missing types:**
- `NewAtomndex` (typo for `NewAtomIndex`?) - line 12
- `NewAtomIndex` - lines 12, 41
- `NewAtomIndices` - lines 14, 17, 21, 29, 34
- `PostfixIterator` - `expansion/chain/expand.rs:22`

**Error:** `E0412: cannot find type`

### 3. Method Naming - Capitalization (3 errors)
**File:** `crates/context-read/src/sequence/mod.rs:18, 26`

**Issue:** Trait methods use `to_new_atom_indices` but should be `to_new_Atom_indices` (capital A)

```rust
// ❌ Current (incorrect)
fn to_new_atom_indices<'a: 'g, 'g, G: HasGraphMut<Kind = BaseGraphKind>>(
    self,
    graph: &'a mut G,
) -> NewAtomIndices

// ✅ Expected
fn to_new_Atom_indices(...)  // Capital 'A'
```

**Error:** `E0407: method 'to_new_atom_indices' is not a member of trait 'ToNewAtomIndices'`

### 4. Removed/Renamed Methods (9 errors)

#### `root_child` → `graph_root_child`
**File:** `crates/context-read/src/complement.rs:20`
```rust
let root = self.link.root_postfix.root_child(trav);  // ❌
// Should be: graph_root_child(trav)
```

#### `vertex_mut` → `vertex` (no mut variant?)
**Files:** `crates/context-read/src/context/root.rs:37, 64`
```rust
let vertex = (*root).vertex_mut(&mut graph);  // ❌
// Should be: vertex(&mut graph)? Or different API?
```

#### `retract` method removed
**File:** `crates/context-read/src/complement.rs:64`
```rust
std::iter::repeat_with(|| complement_path.retract(trav))  // ❌
```
**Type:** `RootedRolePath<context_trace::End, IndexRoot>`

#### `postfix_iter` removed
**File:** `crates/context-read/src/expansion/chain/expand.rs:28`
```rust
let mut postfix_iter = last_end.postfix_iter(ctx.ctx.clone());  // ❌
```
**Type:** `context_trace::Token`

#### `start_index` removed
**File:** `crates/context-read/src/expansion/mod.rs:71`
```rust
index: cursor.start_index(&trav),  // ❌
```
**Type:** `&'a mut RootedRangePath<context_trace::Pattern>`

#### `prefix_path` removed
**File:** `crates/context-read/src/expansion/mod.rs:141`
```rust
let prefix_path = expansion.prefix_path(&self.cursor.ctx, overlap);  // ❌
```
**Type:** `&context_trace::Token`

#### `new_directed` removed
**File:** `crates/context-read/src/context/mod.rs:62`
```rust
match PatternEndPath::new_directed::<Right>(known.clone()) {  // ❌
```
**Type:** `RootedRolePath<context_trace::End, context_trace::Pattern>`

#### `to_new_atom_indices` (lowercase version)
**File:** `crates/context-read/src/context/mod.rs:47`
```rust
let new_indices = seq.to_new_atom_indices(&mut graph.graph_mut());  // ❌
// Should be: to_new_Atom_indices (capital A)
```

### 5. Private API Access (2 errors)

#### Private field: `root_entry`
**File:** `crates/context-read/src/complement.rs:23`
```rust
let intersection_start = self.link.root_postfix.root_entry;  // ❌
```
**Type:** `SubPath`
**Error:** `E0616: field 'root_entry' of struct 'SubPath' is private`

#### Private method: `new_atom_indices`
**File:** `crates/context-read/src/sequence/mod.rs:30`
```rust
graph.graph_mut().new_atom_indices(self)  // ❌
```
**Error:** `E0624: method 'new_atom_indices' is private`
**Note:** Defined as `pub(crate)` in `context-trace/src/graph/insert/atom.rs:76`

### 6. Type Inference Failures (5 errors)

**Files:**
- `complement.rs:65` - closure parameter type in `take_while`
- `context/mod.rs:62, 64` - `new_directed` result type, `into_range` parameter
- `expansion/chain/expand.rs:44` - tuple destructuring `(postfix_location, postfix)`
- `sequence/block_iter.rs:23, 24` - closure parameters in `next_pattern_where`

## Files Affected

1. `crates/context-read/src/complement.rs` - 6 errors
2. `crates/context-read/src/context/root.rs` - 2 errors
3. `crates/context-read/src/context/mod.rs` - 3 errors
4. `crates/context-read/src/expansion/chain/expand.rs` - 3 errors
5. `crates/context-read/src/expansion/mod.rs` - 2 errors
6. `crates/context-read/src/sequence/block_iter.rs` - 5 errors
7. `crates/context-read/src/sequence/mod.rs` - 6 errors

## Root Cause Analysis

**Likely causes:**
1. **API evolution** - context-trace underwent significant refactoring (trait consolidation, method renames)
2. **Missing migration** - context-read was not updated when context-trace API changed
3. **Type system changes** - New type names or moved types (NewAtomIndex vs NewAtomndex)
4. **Visibility changes** - Methods/fields made private that were previously public

## Investigation Needed

1. ✅ Document all errors (this file)
2. ⏳ Research current context-trace API for correct alternatives
3. ⏳ Check agents/guides/ and agents/implemented/ for migration documentation
4. ⏳ Determine if context-read is still in active use
5. ⏳ Create fix plan or deprecation plan

## Related Files

- `agents/implemented/` - Check for trait consolidation, method renames
- `CHEAT_SHEET.md` - Current API reference
- `crates/context-trace/HIGH_LEVEL_GUIDE.md` - API design
- `crates/context-trace/src/` - Source of truth for current API

## Next Steps

1. Research current context-trace API (Step 2)
2. Check for existing migration guides (Step 3)
3. Create fix plan or mark crate as deprecated
4. Update CHEAT_SHEET.md if needed

---

**Tags:** context-read, api-mismatch, compilation-errors, migration-needed, critical
