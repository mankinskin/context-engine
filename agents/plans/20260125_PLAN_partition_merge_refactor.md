---
tags: `#plan` `#context-insert` `#debugging` `#testing` `#refactoring`
summary: The current partition merge implementation has several design issues:
status: 📋
---

# Partition Merge Refactoring Plan

**Created:** 2026-01-25  
**Status:** Ready for Implementation  
**Objective:** Refactor partition merging to use generic position types and a unified MergePartitionCtx

---

## Background

The current partition merge implementation has several design issues:

1. **Duplicate position methods in PatternSplits trait:** `atom_pos()` and `atom_pos_pair()` are two separate Option methods, leading to awkward handling in `info_border_with_pos`
2. **`info_border_with_pos` exists as a workaround:** This method takes both single and pair positions as separate parameters
3. **MergePartition trait is awkward:** Uses trait-based dispatch instead of a context struct

### Current Issues (from debugging session)

During insert_prefix1 and insert_postfix1 test debugging, these issues were identified:

1. ✅ Fixed: `new_from_atom_pos` was using parameter inner_offset instead of recalculated one
2. ✅ Fixed: Deltas should only apply to offsets AFTER the merged partition
3. ✅ Fixed: Root mode shouldn't add sub-merge patterns for Full partition type  
4. ⚠️ Pending: Pattern replacement not happening correctly for postfix merge - architecture needs cleanup

---

## Design

### 1. Generic Position Type in PatternSplits

**Current:**
```rust
pub trait PatternSplits: Debug + Clone {
    type Pos;
    type Offsets;
    fn atom_pos(&self) -> Option<NonZeroUsize>;
    fn atom_pos_pair(&self) -> Option<(NonZeroUsize, NonZeroUsize)> { None }
    // ...
}
```

**Proposed:**
```rust
pub trait PatternSplits: Debug + Clone {
    type Pos;
    type Offsets;
    /// The atom position type - NonZeroUsize for Pre/Post, (NonZeroUsize, NonZeroUsize) for In
    type AtomPos: Clone + Debug;
    
    fn atom_pos(&self) -> Self::AtomPos;
    // ...
}

// Implementations:
impl PatternSplits for VertexSplits {
    type AtomPos = NonZeroUsize;
    fn atom_pos(&self) -> Self::AtomPos { self.pos }
}

impl<A: PatternSplits, B: PatternSplits> PatternSplits for (A, B) {
    type AtomPos = (A::AtomPos, B::AtomPos);
    fn atom_pos(&self) -> Self::AtomPos { (self.0.atom_pos(), self.1.atom_pos()) }
}
```

### 2. Unified VisitBorders::info_border

**Current:**
```rust
fn info_border(pattern: &Pattern, splits: &Self::Splits, atom_pos: Option<NonZeroUsize>) -> Self;
fn info_border_with_pos(pattern: &Pattern, splits: &Self::Splits, 
    atom_pos: Option<NonZeroUsize>, 
    atom_pos_pair: Option<(NonZeroUsize, NonZeroUsize)>) -> Self;
```

**Proposed:**
```rust
fn info_border<P: PatternSplits>(pattern: &Pattern, splits: &P) -> Self
where
    P::AtomPos: Into<Self::AtomPosInput>;
```

Or simpler - have the Splits type carry the position:
```rust
fn info_border(pattern: &Pattern, splits: &Self::Splits) -> Self;
// Where Self::Splits already contains the atom position
```

### 3. MergePartitionCtx Struct

**Current (trait-based):**
```rust
pub trait MergePartition<R: RangeRole<Mode = Join>>: Sized + ToPartition<R> {
    fn merge_partition(&mut self, ctx: &mut MergeCtx, range_map: &RangeMap, range: &PartitionRange) 
        -> (Token, Option<PatternSubDeltas>);
    // ...
}
```

**Proposed (context struct):**
```rust
pub struct MergePartitionCtx<'a, R: RangeRole<Mode = Join>> {
    pub merge_ctx: &'a mut MergeCtx<'a>,
    pub range_map: &'a RangeMap,
    pub partition_range: &'a PartitionRange,
    pub offsets: R::Offsets,  // Generic offsets type from RangeRole
}

impl<'a, R: RangeRole<Mode = Join>> MergePartitionCtx<'a, R> {
    /// Merge the partition, returning (token, delta)
    pub fn merge(self) -> (Token, Option<PatternSubDeltas>) { ... }
    
    /// Get partition info for all patterns
    pub fn info_partition(&self) -> Result<PartitionInfo<R>, Token> { ... }
    
    /// Create JoinedPartition from partition info
    pub fn join_partition(&mut self, info: PartitionInfo<R>) -> JoinedPartition<R> { ... }
}
```

---

## Implementation Plan

### Phase 1: Refactor PatternSplits (1-2 hours)

1. Add `type AtomPos` associated type to `PatternSplits` trait
2. Change `atom_pos()` to return `Self::AtomPos` (not Option)
3. Remove `atom_pos_pair()` method
4. Update implementations:
   - `VertexSplits`: `type AtomPos = NonZeroUsize`
   - `(A, B)` tuple: `type AtomPos = (A::AtomPos, B::AtomPos)`
5. Update all call sites

**Files to modify:**
- `crates/context-insert/src/split/pattern.rs`
- `crates/context-insert/src/split/vertex/mod.rs`

### Phase 2: Simplify VisitBorders (1-2 hours)

1. Remove `info_border_with_pos` method
2. Modify `info_border` to accept generic position type
3. Update `BorderInfo::new_from_atom_pos` to accept generic position
4. Update all implementations of `VisitBorders`

**Files to modify:**
- `crates/context-insert/src/interval/partition/info/border/mod.rs`
- `crates/context-insert/src/interval/partition/info/border/visit.rs`

### Phase 3: Create MergePartitionCtx (2-3 hours)

1. Create new `MergePartitionCtx<R>` struct in `merge/partition.rs`
2. Move partition-specific logic from trait methods to struct methods
3. Remove or simplify `MergePartition` trait
4. Update `MergeCtx::merge_sub_partitions` to use new context
5. Simplify the match on `PartitionType` in merge loop

**Files to modify:**
- `crates/context-insert/src/join/context/node/merge/partition.rs`
- `crates/context-insert/src/join/context/node/merge/context.rs`
- `crates/context-insert/src/interval/partition/mod.rs`

### Phase 4: Verify & Clean Up (1 hour)

1. Run all insert tests: `cargo test -p context-insert`
2. Fix any remaining issues
3. Remove dead code
4. Update documentation

---

## Test Commands

```bash
# Run all context-insert tests
cargo test -p context-insert

# Run specific insert tests
cargo test -p context-insert insert_prefix1 -- --nocapture
cargo test -p context-insert insert_postfix1 -- --nocapture

# Run with logging
LOG_STDOUT=1 LOG_FILTER=debug cargo test -p context-insert insert_prefix1 -- --nocapture
```

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `split/pattern.rs` | PatternSplits trait definition |
| `split/vertex/mod.rs` | VertexSplits implementation |
| `interval/partition/info/border/mod.rs` | BorderInfo struct |
| `interval/partition/info/border/visit.rs` | VisitBorders trait |
| `join/context/node/merge/partition.rs` | MergePartition trait (to refactor) |
| `join/context/node/merge/context.rs` | MergeCtx and merge loop |

---

## Notes from Previous Session

- The `new_from_atom_pos` function now correctly recalculates both `sub_index` AND `inner_offset` from the atom position
- Delta application now only affects offsets AFTER the current partition (using `after_offset_index` parameter)
- Root mode skips adding sub-merge patterns for Full partition type
- `insert_prefix1` test passes; `insert_postfix1` has pattern structure issues that this refactoring should help resolve
