# Implementation Plan: Root Node Join Refactoring

**Date:** 2025-01-04  
**Status:** READY FOR REVIEW  
**Based on:** `20250103_SPEC_root_join_refactoring.md`

## 1. Key Insight from Examples

The examples reveal that the algorithm is fundamentally simple:

**"Merge all partitions from smallest to largest until all wrapper boundaries are reached."**

All partition types (inner, target, wrapper) are handled uniformly:
- **Inner partitions**: Built first (smallest), used as building blocks
- **Target partition**: Built using inner partitions, this is what we return
- **Wrapper partitions**: Built last (largest), contain the target plus complements

There is NO need to:
- Classify offsets by role (target/wrapper/inner) during the merge loop
- Have separate join functions for inner/target/wrapper partitions

The only special handling needed:
1. **Store the target token** when the target partition is created (to return it)
2. **Replace root pattern children** with partition tokens when partitions span child boundaries
   - This applies uniformly to ALL partitions (inner, target, and wrapper)

---

## 2. Algorithm Overview

### 2.1 Partition Types (from Spec Section 2.2)

| Partition Type | Definition | Ownership |
|---------------|------------|-----------|
| **Inner Partition** | Largest range of perfect boundaries inside target | Belongs to exactly one pattern |
| **Target Partition** | Partition defined by target offset(s) | Replaced in pattern only if both offsets perfect |
| **Wrapper Partition** | Partition from perfect boundaries around target | Belongs to exactly one pattern |

**Key insight**: Each partition belongs to **at most one pattern**. A partition IS its offset range - the same object may be an "intermediate partition" globally and "the inner partition of P1" simultaneously.

### 2.2 Algorithm: Two Phases

#### Phase 1: Augmentation
Add inner and wrapper offsets to all pattern nodes:
- For each pattern, calculate closest perfect boundaries inside/around target
- This determines which partitions each pattern owns

#### Phase 2: Join (smallest-to-largest merge)
Merge all k-part partitions from smaller to larger:

```
join_root_partitions(ctx, root_mode):
    
    1. Merge partitions from smallest to largest:
       For each partition (by increasing size):
         → Join partition and cache result
         
         Then check if replacement needed:
         
         1.1. If this is an INNER partition for some pattern:
              → Replace in its owning pattern
         
         1.2. If this is the TARGET partition:
              → Store the resulting child token
              → If BOTH target offsets are perfect: replace in pattern
         
         1.3. If this is a WRAPPER partition for some pattern:
              → Replace in its owning pattern
         
         (Intermediate partitions: no replacement, just cached for building larger partitions)
    
    2. Return stored target child
```

**Key points**:
- **All** partitions are joined and cached (including intermediate ones)
- Only special partitions (inner, target, wrapper) check for replacement
- Intermediate partitions exist as building blocks for larger partitions
- Each special partition has at most one owning pattern → at most one replacement
- Merge order ensures dependencies are satisfied (smaller partitions built first)

### 2.3 Why This Works

From the examples:
- All partitions between any two offsets are built during the merge
- The target partition is just ONE of these partitions (at a specific size)
- Wrapper partitions are ALSO just partitions (at larger sizes)
- Intermediate partitions are cached so larger partitions can reference them
- Delta tracking happens naturally because smaller partitions are built first

---

## 3. Data Structures

### 3.1 Minimal Context

```rust
/// Context for root join
pub struct RootJoinContext<'a> {
    /// Reference to node join context
    ctx: &'a mut NodeJoinCtx<'_>,
    
    /// All offsets sorted by position (from vertex cache)
    offsets: Vec<NonZeroUsize>,
    
    /// Target offset range (left, right) - right is None for Prefix/Postfix
    target_range: (NonZeroUsize, Option<NonZeroUsize>),
    
    /// Inner partition per pattern: pattern_id -> (left_bound, right_bound)
    /// Calculated during augmentation as largest range of perfect boundaries inside target
    inner_bounds: HashMap<PatternId, (usize, usize)>,
    
    /// Wrapper partition per pattern: pattern_id -> (left_bound, right_bound)
    /// Calculated during augmentation as closest perfect boundaries around target
    wrapper_bounds: HashMap<PatternId, (usize, usize)>,
    
    /// Created partitions: (start_pos, end_pos) -> Token
    partitions: HashMap<(usize, usize), Token>,
    
    /// Target token (set when target partition is created)
    target_token: Option<Token>,
}
```

### 3.2 Partition Ownership

Each partition belongs to **at most one pattern**. Ownership is determined during augmentation:

```rust
/// Maps partition ranges to their owning pattern and role
struct PartitionOwnership {
    /// Inner partitions: range -> owning pattern
    inner: HashMap<(usize, usize), PatternId>,
    
    /// Wrapper partitions: range -> owning pattern  
    wrapper: HashMap<(usize, usize), PatternId>,
    
    /// Target range (no owner unless both offsets perfect)
    target: (usize, usize),
    target_owner: Option<PatternId>,  // Set only if both offsets perfect
}
```

### 3.3 Delta Tracking

**Key finding**: Delta tracking IS still needed, but it's **handled automatically** by `JoinPartition`.

```rust
// JoinedPartition stores computed delta
pub struct JoinedPartition<R: RangeRole> {
    pub index: Token,
    pub perfect: R::Perfect,
    pub delta: PatternSubDeltas,  // Auto-computed during join
}
```

The current code uses `roffset.split.clone() - part.delta` to adjust offsets after joining. When using `JoinPartition::join_partition()`, this happens automatically.

**Implication**: Use the existing `JoinPartition` trait - no manual delta tracking needed.

---

## 4. Implementation Steps

The implementation leverages the existing `JoinPartition` infrastructure
rather than building everything from scratch.

### Step 1: Augmentation - Calculate Inner and Wrapper Bounds

During augmentation, we calculate inner and wrapper bounds for each pattern.
This determines partition ownership.

```rust
fn augment_patterns(
    ctx: &NodeJoinCtx,
    target_range: (usize, Option<usize>),
    root_mode: RootMode,
) -> (HashMap<PatternId, (usize, usize)>, HashMap<PatternId, (usize, usize)>) {
    let mut inner_bounds = HashMap::new();
    let mut wrapper_bounds = HashMap::new();
    let root_width = ctx.root_width();
    
    for pattern_id in ctx.pattern_ids() {
        // Calculate inner bounds: largest range of perfect boundaries inside target
        let (target_left, target_right) = match root_mode {
            RootMode::Prefix => (0, target_range.0),
            RootMode::Postfix => (target_range.0, root_width),
            RootMode::Infix => (target_range.0, target_range.1.unwrap()),
        };
        
        let left_inner = find_first_perfect_boundary_gte(ctx, pattern_id, target_left);
        let right_inner = find_first_perfect_boundary_lte(ctx, pattern_id, target_right);
        
        // Only store inner bounds if there's actually a range
        if left_inner < right_inner {
            inner_bounds.insert(pattern_id, (left_inner, right_inner));
        }
        
        // Calculate wrapper bounds: closest perfect boundaries around target
        let (left_wrapper, right_wrapper) = match root_mode {
            RootMode::Prefix => {
                // Wrapper: from 0 to first perfect boundary >= target
                let right = find_first_perfect_boundary_gte(ctx, pattern_id, target_right);
                (0, right)
            }
            RootMode::Postfix => {
                // Wrapper: from first perfect boundary <= target to end
                let left = find_first_perfect_boundary_lte(ctx, pattern_id, target_left);
                (left, root_width)
            }
            RootMode::Infix => {
                // Wrapper: perfect boundaries on both sides
                let left = find_first_perfect_boundary_lte(ctx, pattern_id, target_left);
                let right = find_first_perfect_boundary_gte(ctx, pattern_id, target_right);
                (left, right)
            }
        };
        
        // Only store wrapper bounds if different from target (i.e., target offset is imperfect)
        if (left_wrapper, right_wrapper) != (target_left, target_right) {
            wrapper_bounds.insert(pattern_id, (left_wrapper, right_wrapper));
        }
    }
    
    (inner_bounds, wrapper_bounds)
}
```

### Step 2: Merge Loop with Inline Replacement

**All partitions are joined and cached**. Only special partitions (inner, target, wrapper) check for replacement. Intermediate partitions are building blocks for constructing larger partitions.

```rust
fn merge_all_partitions(root_ctx: &mut RootJoinContext) {
    let offsets: Vec<usize> = root_ctx.offsets.iter().map(|o| o.get()).collect();
    let max_offset = *offsets.last().unwrap();
    
    // Include 0 as implicit start boundary
    let all_positions: Vec<usize> = std::iter::once(0)
        .chain(offsets.iter().copied())
        .collect();
    
    // Build partitions from smallest to largest
    for size in 1..=max_offset {
        for &start in &all_positions {
            let end = start + size;
            if !is_valid_boundary(end, &all_positions, max_offset) {
                continue;
            }
            
            // Skip if already created
            if root_ctx.partitions.contains_key(&(start, end)) {
                continue;
            }
            
            // JOIN: All partitions are joined and cached
            let token = join_partition_range(root_ctx, start, end);
            root_ctx.partitions.insert((start, end), token);
            
            // REPLACE: Only special partitions check for replacement
            let partition_type = classify_partition(root_ctx, start, end);
            
            match partition_type {
                PartitionType::Inner { owner_pattern } => {
                    // 1.1: Replace inner partition in its owning pattern
                    replace_in_pattern(root_ctx, owner_pattern, token, start, end);
                }
                PartitionType::Target { both_offsets_perfect, owner_pattern } => {
                    // 1.2: Store target child, replace only if both offsets perfect
                    root_ctx.target_token = Some(token);
                    if both_offsets_perfect {
                        if let Some(pattern) = owner_pattern {
                            replace_in_pattern(root_ctx, pattern, token, start, end);
                        }
                    }
                }
                PartitionType::Wrapper { owner_pattern } => {
                    // 1.3: Replace wrapper partition in its owning pattern
                    replace_in_pattern(root_ctx, owner_pattern, token, start, end);
                }
                PartitionType::Intermediate => {
                    // No replacement - intermediate partitions are just cached
                    // for use as building blocks in larger partitions
                }
            }
        }
    }
}

/// Replace a partition's children in a specific pattern.
/// 
/// Each partition has at most one owning pattern, so this is called
/// at most once per partition.
fn replace_in_pattern(
    root_ctx: &mut RootJoinContext,
    pattern_id: PatternId,
    token: Token,
    start: usize,
    end: usize,
) {
    if let Some((start_idx, end_idx)) = get_spanning_child_range(root_ctx.ctx, pattern_id, start, end) {
        let loc = root_ctx.ctx.index.to_pattern_location(pattern_id);
        root_ctx.ctx.trav.replace_in_pattern(loc, start_idx..end_idx, vec![token]);
    }
}

/// Classify a partition by its role and owning pattern (if any)
enum PartitionType {
    /// Inner partition - belongs to exactly one pattern
    Inner { owner_pattern: PatternId },
    /// Target partition - may belong to a pattern if both offsets perfect
    Target { both_offsets_perfect: bool, owner_pattern: Option<PatternId> },
    /// Wrapper partition - belongs to exactly one pattern
    Wrapper { owner_pattern: PatternId },
    /// Intermediate partition - building block, no owner
    Intermediate,
}

/// Join a partition using existing Infix infrastructure
///
/// 3+ part patterns are created automatically by this infrastructure:
/// - Root child patterns: e.g., P1's view `[a, bc, defg, h]` from root's children + split borders
/// - Inner range joining: Target spanning multiple boundaries creates recursive joins
/// - Composite patterns: The merge creates trigrams like `[left_border, inner, right_border]`
///
/// The merge loop only needs to track 2-way boundary combinations - `JoinPartition` handles
/// the recursive inner range joining that produces 3+ part patterns.
fn join_partition_range(root_ctx: &mut RootJoinContext, start: usize, end: usize) -> Token {
    // Get offset contexts for start and end positions
    let start_offset = position_splits(root_ctx.ctx.patterns(), NonZeroUsize::new(start).unwrap());
    let end_offset = position_splits(root_ctx.ctx.patterns(), NonZeroUsize::new(end).unwrap());
    
    // Use existing JoinPartition trait - handles:
    // - Existing token detection (returns Err(token))
    // - Pattern extraction from root children
    // - Inner range joining (3+ part patterns via recursive JoinInnerRangeInfo)
    // - Delta computation (automatic via JoinedPartition.delta)
    match Infix::new(&start_offset, &end_offset).join_partition(root_ctx.ctx) {
        Ok(joined) => joined.index,
        Err(existing) => existing,
    }
}
```

### Step 3: Helper Functions

```rust
/// Get the child index range that a partition spans in a given pattern.
/// Returns None if the partition doesn't span any children in this pattern.
fn get_spanning_child_range(
    ctx: &NodeJoinCtx,
    pattern_id: PatternId,
    start: usize,
    end: usize,
) -> Option<(usize, usize)> {
    let boundaries = ctx.get_pattern_boundaries(pattern_id);
    
    // Find first child that starts at or after `start`
    let start_idx = boundaries.iter().position(|&b| b >= start)?;
    
    // Find first child that ends at or after `end`
    let end_idx = boundaries.iter().position(|&b| b >= end)?;
    
    // Only return if we actually span something
    if end_idx > start_idx {
        Some((start_idx, end_idx))
    } else {
        None
    }
}

/// Check if a partition partially covers a child (i.e., splits it)
fn is_partial_child(
    ctx: &NodeJoinCtx,
    pattern_id: PatternId,
    start: usize,
    end: usize,
) -> bool {
    let boundaries = ctx.get_pattern_boundaries(pattern_id);
    // Partition is partial if start or end is NOT at a child boundary
    !boundaries.contains(&start) || !boundaries.contains(&end)
}
```

### Step 4: Entry Point

```rust
pub fn join_root_partitions(ctx: &mut NodeJoinCtx, root_mode: RootMode) -> Token {
    // Extract target range from root_mode
    let target_range = get_target_range(ctx, root_mode);
    
    // Calculate wrapper bounds for each pattern
    let wrapper_bounds = calculate_wrapper_bounds(ctx, target_range, root_mode);
    
    // Collect offsets from vertex cache
    let offsets: Vec<NonZeroUsize> = ctx.vertex_cache().keys().copied().collect();
    
    // Initialize context
    let mut root_ctx = RootJoinContext {
        ctx,
        offsets,
        target_range,
        wrapper_bounds,
        partitions: HashMap::new(),
        target_token: None,
    };
    
    // Run merge
    merge_all_partitions(&mut root_ctx);
    
    // Return target
    root_ctx.target_token.expect("Target partition should have been created")
}
```

---

## 5. Current Implementation Analysis & Cleanup Plan

### 5.1 Current File Structure

```
crates/context-insert/src/join/
├── mod.rs                           # Module exports only
├── context/
│   ├── mod.rs                       # Module exports
│   ├── frontier.rs                  # FrontierSplitIterator - JOIN DRIVER
│   ├── node/
│   │   ├── mod.rs                   # Module exports
│   │   ├── context.rs               # NodeJoinCtx + join_root_partitions (MAIN TARGET)
│   │   └── merge.rs                 # NodeMergeCtx - non-root merge
│   └── pattern/
│       ├── mod.rs                   # PatternJoinCtx
│       └── borders.rs               # JoinBorders trait impls
├── joined/
│   ├── mod.rs                       # Module exports
│   ├── partition.rs                 # JoinedPartition struct
│   └── patterns.rs                  # JoinedPatterns struct
└── partition/
    ├── mod.rs                       # JoinPartition trait
    └── info/
        ├── mod.rs                   # JoinPartitionInfo
        ├── inner_range.rs           # JoinInnerRangeInfo
        └── pattern_info.rs          # JoinPatternInfo
```

### 5.2 Code to Remove (Old Implementation)

**File: `context/node/context.rs`** (lines ~212-551)

| Function | Lines | Purpose | Why Remove |
|----------|-------|---------|------------|
| `join_incomplete_infix` | 212-350 | Handle imperfect infix target | Complex, imperative, handles cases individually |
| `join_incomplete_postfix` | 352-457 | Handle imperfect postfix target | Duplicates logic from infix with slight variations |
| `join_incomplete_prefix` | 459-551 | Handle imperfect prefix target | Duplicates logic from infix with slight variations |

**Total: ~340 lines to remove**

### 5.3 Code to Keep (Reusable Infrastructure)

| Component | Location | Why Keep |
|-----------|----------|----------|
| `JoinPartition` trait | `partition/mod.rs` | Core partition joining logic |
| `JoinedPartition` | `joined/partition.rs` | Result type with delta tracking |
| `JoinedPatterns` | `joined/patterns.rs` | Pattern collection for insertion |
| `JoinPartitionInfo` | `partition/info/mod.rs` | Partition metadata |
| `JoinInnerRangeInfo` | `partition/info/inner_range.rs` | Inner range handling |
| `JoinPatternInfo` | `partition/info/pattern_info.rs` | Per-pattern join info |
| `JoinBorders` trait | `context/pattern/borders.rs` | Border split access |
| `NodeMergeCtx` | `context/node/merge.rs` | Non-root merge (keep as-is) |
| `FrontierSplitIterator` | `context/frontier.rs` | Join driver (keep as-is) |
| `NodeJoinCtx` | `context/node/context.rs` | Context struct (keep, modify methods) |
| `LockedFrontierCtx` | `context/node/context.rs` | Lock wrapper (keep) |

### 5.4 Code to Modify

**File: `context/node/context.rs`**

| Function | Current State | New State |
|----------|--------------|-----------|
| `join_root_partitions` | Dispatches to `join_incomplete_*` | New unified algorithm |
| `join_partitions` | Non-root join (keep) | No change |

---

## 6. Implementation Phases

### Phase 1: Create New Root Join Module

**Create new file: `context/node/root.rs`**

This isolates the new implementation from the old code, allowing parallel development and easy rollback.

```rust
// crates/context-insert/src/join/context/node/root.rs

use std::collections::HashMap;
use std::num::NonZeroUsize;

use crate::{
    interval::partition::{Infix, Postfix, Prefix},
    join::{
        context::node::context::NodeJoinCtx,
        partition::JoinPartition,
    },
    split::vertex::output::RootMode,
};
use context_trace::*;

/// Context for root node joining
pub struct RootJoinContext<'a> {
    pub ctx: &'a mut NodeJoinCtx<'a>,
    pub target_range: (usize, usize),
    pub inner_bounds: HashMap<PatternId, (usize, usize)>,
    pub wrapper_bounds: HashMap<PatternId, (usize, usize)>,
    pub partitions: HashMap<(usize, usize), Token>,
    pub target_token: Option<Token>,
}

/// Partition classification for replacement logic
pub enum PartitionType {
    Inner { owner: PatternId },
    Target { both_perfect: bool, owner: Option<PatternId> },
    Wrapper { owner: PatternId },
    Intermediate,
}

// ... implementation functions
```

### Phase 2: Implement Core Algorithm

**Add to `root.rs`:**

1. `augment_patterns()` - Calculate inner/wrapper bounds per pattern
2. `merge_all_partitions()` - Main merge loop
3. `classify_partition()` - Determine partition type and owner
4. `replace_in_pattern()` - Single replacement helper
5. `join_partition_range()` - Wrapper around `JoinPartition`

### Phase 3: Wire Up Entry Point

**Modify `context/node/context.rs`:**

```rust
// Before (current):
pub fn join_root_partitions(&mut self) -> Token {
    let root_mode = self.interval.cache.root_mode;
    // ... dispatch to join_incomplete_*
}

// After (new):
pub fn join_root_partitions(&mut self) -> Token {
    root::join_root_partitions(self)
}
```

### Phase 4: Test & Verify

1. Run existing tests - ensure no regressions
2. Add new tests matching spec examples (Prefix, Postfix, Infix)
3. Verify delta tracking works correctly

### Phase 5: Cleanup

1. Remove `join_incomplete_prefix`
2. Remove `join_incomplete_postfix`
3. Remove `join_incomplete_infix`
4. Remove any dead code (unused helpers, commented code)

---

## 7. Differences from Original Spec

| Original Spec | Revised Implementation |
|--------------|----------------------|
| `TargetRole` trait with 3 implementations | Simple `match root_mode` statements |
| `SplitOffset` with role classification | Just use position values |
| Separate `join_inner`, `join_target`, `join_wrapper` | Single merge loop using `Infix::join_partition` |
| Separate replacement logic per partition type | Unified `replace_spanning_children` for all |
| Complex offset iterator patterns | Simple nested loops over positions |
| `PartitionResult` enum | Use existing `JoinedPartition` from `JoinPartition` trait |
| `PatternPartitionBounds` struct | Simple `(usize, usize)` tuples |
| Manual pattern building | Use existing `JoinPartition` infrastructure |
| Custom delta tracking | Automatic via `JoinPartition` |
| Manual existing token lookup | `JoinPartition` returns `Err(token)` for existing |
| Manual 3+ part pattern generation | Handled by `JoinPartition` inner range joining |

---

## 8. Implementation Checklist (Phased)

### Phase 1: Create New Module
- [ ] Create `crates/context-insert/src/join/context/node/root.rs`
- [ ] Add module export in `context/node/mod.rs`
- [ ] Define `RootJoinContext` struct
- [ ] Define `PartitionType` enum

### Phase 2: Implement Core Algorithm
- [ ] `augment_patterns()` - Calculate inner/wrapper bounds
- [ ] `merge_all_partitions()` - Main merge loop
- [ ] `classify_partition()` - Partition type classification
- [ ] `replace_in_pattern()` - Single replacement helper
- [ ] `join_partition_range()` - Wrapper around `JoinPartition`

### Phase 3: Helper Functions
- [ ] `find_first_perfect_boundary_gte(ctx, pattern_id, pos) -> usize`
- [ ] `find_first_perfect_boundary_lte(ctx, pattern_id, pos) -> usize`
- [ ] `get_child_boundaries(ctx, pattern_id) -> Vec<usize>`
- [ ] `get_spanning_child_range(ctx, pattern_id, start, end) -> Option<(usize, usize)>`
- [ ] `get_target_range(ctx, root_mode) -> (usize, usize)`

### Phase 4: Wire Up & Test
- [ ] Replace `join_root_partitions` in `context.rs` to call new module
- [ ] Run existing test suite - ensure no regressions
- [ ] Add unit tests for Prefix example from spec
- [ ] Add unit tests for Postfix example from spec
- [ ] Add unit tests for Infix example from spec
- [ ] Test edge cases: perfect target boundaries, single-pattern roots

### Phase 5: Cleanup
- [ ] Remove `join_incomplete_prefix` (~100 lines)
- [ ] Remove `join_incomplete_postfix` (~100 lines)
- [ ] Remove `join_incomplete_infix` (~140 lines)
- [ ] Remove commented-out code in `context.rs`
- [ ] Update module documentation

**Estimated LOC changes:**
- New code: ~200-250 lines (in `root.rs`)
- Removed code: ~340 lines (3 `join_incomplete_*` functions)
- Net reduction: ~90-140 lines

---

## 9. Examples

*[Copied verbatim from 20250103_SPEC_root_join_refactoring.md Section 8]*

This section provides detailed worked examples for each target type (Prefix, Postfix, Infix),
demonstrating the smallest-to-largest merge algorithm with delta tracking.

**Key constraint**: During joining, the target partition spans **at least one child boundary 
in every pattern**. This is because the search phase would have stopped at a smaller 
containing parent otherwise.

### 9.1 Prefix Example

```
Prefix target with NO perfect boundary, spanning inner boundaries in all patterns:

Root patterns BEFORE join (width=12, "abcdefghijkl"):
  P1: [a][bc][defg][hijkl]   → boundaries at 1, 3, 7, 12
  P2: [ab][cde][fghij][kl]   → boundaries at 2, 5, 10, 12
  P3: [abcd][ef][ghi][jkl]   → boundaries at 4, 6, 9, 12

Boundary uniqueness check: 1, 2, 3, 4, 5, 6, 7, 9, 10, 12 ✓ (all unique)

Target: Prefix ending at pos=8 ("abcdefgh")

Analysis - target offset at pos=8:
  P1: imperfect (inside 'hijkl', offset 1 from start)
      Target spans boundaries at pos=1, pos=3, and pos=7
      Children covered: [a, bc, defg, h] where 'h' is left-split of 'hijkl'
  P2: imperfect (inside 'fghij', offset 3 from start)
      Target spans boundaries at pos=2 and pos=5
      Children covered: [ab, cde, fgh] where 'fgh' is left-split of 'fghij'
  P3: imperfect (inside 'ghi', offset 2 from start)
      Target spans boundaries at pos=4 and pos=6
      Children covered: [abcd, ef, gh] where 'gh' is left-split of 'ghi'

No perfect boundary at pos=8 → all patterns need wrappers

Target spans inner boundaries in ALL patterns:
  P1: pos=1, 3, 7 (3 boundaries)
  P2: pos=2, 5 (2 boundaries)
  P3: pos=4, 6 (2 boundaries)

Wrapper boundaries (first perfect boundary ≥ pos=8 in each pattern):
  P1: pos=12 (end - no boundary between 8 and 12)
  P2: pos=10
  P3: pos=9

Step 1: Join all partitions smallest to largest

  All partition boundaries:
    - pos=1 (P1 boundary)
    - pos=2 (P2 boundary)
    - pos=3 (P1 boundary)
    - pos=4 (P3 boundary)
    - pos=5 (P2 boundary)
    - pos=6 (P3 boundary)
    - pos=7 (P1 boundary)
    - pos=8 (TARGET offset)
    - pos=9 (P3 wrapper boundary)
    - pos=10 (P2 wrapper boundary)
  
  Sorted offsets: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
  
  === 1-part partitions (between adjacent boundaries) ===
  
    pos=0 to pos=1: "a"      (P1 child, use directly)
    pos=1 to pos=2: "b"      (trivial/atom)
    pos=2 to pos=3: "c"      (trivial/atom)
    pos=3 to pos=4: "d"      (trivial/atom)
    pos=4 to pos=5: "e"      (trivial/atom)
    pos=5 to pos=6: "f"      (trivial/atom)
    pos=6 to pos=7: "g"      (trivial/atom)
    pos=7 to pos=8: "h"      (trivial/atom)
    pos=8 to pos=9: "i"      (trivial/atom)
    pos=9 to pos=10: "j"     (trivial/atom)
  
  === 2-part partitions ===
  
    pos=0 to pos=2: "ab"     - matches P2 child [ab], use directly
    pos=1 to pos=3: "bc"     - matches P1 child [bc], use directly
    pos=2 to pos=4: "cd"     - create from [c, d]
    pos=3 to pos=5: "de"     - create from [d, e]
    pos=4 to pos=6: "ef"     - matches P3 child [ef], use directly
    pos=5 to pos=7: "fg"     - create from [f, g]
    pos=6 to pos=8: "gh"     - create from [g, h]
    pos=7 to pos=9: "hi"     - create from [h, i]
    pos=8 to pos=10: "ij"    - create from [i, j]
  
  === 3-part partitions ===
  
    pos=0 to pos=3: "abc"    - create from [a, bc], [ab, c]
    pos=1 to pos=4: "bcd"    - create from [bc, d], [b, cd]
    pos=2 to pos=5: "cde"    - matches P2 child [cde], use directly
    pos=3 to pos=6: "def"    - create from [de, f], [d, ef]
    pos=4 to pos=7: "efg"    - create from [ef, g], [e, fg]
    pos=5 to pos=8: "fgh"    - create from [fg, h], [f, gh]
    pos=6 to pos=9: "ghi"    - matches P3 child [ghi], use directly
    pos=7 to pos=10: "hij"   - create from [hi, j], [h, ij]
  
  === 4-part partitions ===
  
    pos=0 to pos=4: "abcd"   - matches P3 child [abcd], use directly
    pos=1 to pos=5: "bcde"   - create from [bcd, e], [bc, de]
    pos=2 to pos=6: "cdef"   - create from [cde, f], [cd, ef]
    pos=3 to pos=7: "defg"   - matches P1 child [defg], use directly
    pos=4 to pos=8: "efgh"   - create from [efg, h], [ef, gh]
    pos=5 to pos=9: "fghi"   - create from [fgh, i], [fg, hi]
    pos=6 to pos=10: "ghij"  - create from [ghi, j], [gh, ij]
  
  === 5-part partitions ===
  
    pos=0 to pos=5: "abcde"  - create from [abcd, e], [abc, de], [ab, cde]
    pos=1 to pos=6: "bcdef"  - create from [bcde, f], [bcd, ef]
    pos=2 to pos=7: "cdefg"  - create from [cdef, g], [cde, fg]
    pos=3 to pos=8: "defgh"  - create from [defg, h], [def, gh]
    pos=4 to pos=9: "efghi"  - create from [efgh, i], [efg, hi]
    pos=5 to pos=10: "fghij" - matches P2 child [fghij], use directly
  
  === 6-part partitions ===
  
    pos=0 to pos=6: "abcdef" - create from [abcde, f], [abcd, ef], [abc, def]
    pos=1 to pos=7: "bcdefg" - create from [bcdef, g], [bcde, fg], [bc, defg]
    pos=2 to pos=8: "cdefgh" - create from [cdefg, h], [cdef, gh], [cde, fgh]
    pos=3 to pos=9: "defghi" - create from [defgh, i], [defg, hi], [def, ghi]
    pos=4 to pos=10: "efghij" - create from [efghi, j], [efgh, ij], [ef, ghij]
  
  === 7-part partitions ===
  
    pos=0 to pos=7: "abcdefg" - create from [abcdef, g], [abcde, fg], [abcd, efg], [a, bcdefg]
    pos=1 to pos=8: "bcdefgh" - create from [bcdefg, h], [bcdef, gh], [bc, defgh]
    pos=2 to pos=9: "cdefghi" - create from [cdefgh, i], [cdefg, hi], [cde, fghi]
    pos=3 to pos=10: "defghij" - create from [defghi, j], [defgh, ij], [defg, hij]
  
  === 8-part partitions ===
  
    pos=0 to pos=8: "abcdefgh" ← TARGET PARTITION
      
      Built from 7-part partitions + 1-part partitions:
        - [abcdefg, h] where 'abcdefg' was created at 7-part stage
        - [a, bcdefgh] where 'bcdefgh' was created at 7-part stage
      
      Plus patterns derived from each root pattern's child structure:
        - P1 view: [a, bc, defg, h] 
            Uses: 'a' (P1 child), 'bc' (P1 child), 'defg' (P1 child), 
                  'h' (left-split of 'hijkl')
        - P2 view: [ab, cde, fgh]
            Uses: 'ab' (P2 child), 'cde' (P2 child),
                  'fgh' (left-split of 'fghij')
        - P3 view: [abcd, ef, gh]
            Uses: 'abcd' (P3 child), 'ef' (P3 child),
                  'gh' (left-split of 'ghi')
      
      All merge combinations from smaller partitions:
        [abcdefg, h]      - 7-part + 1-part
        [abcdef, gh]      - 6-part + 2-part
        [abcde, fgh]      - 5-part + 3-part
        [abcd, efgh]      - 4-part + 4-part (uses P3 child 'abcd')
        [abc, defgh]      - 3-part + 5-part
        [ab, cdefgh]      - 2-part + 6-part (uses P2 child 'ab')
        [a, bcdefgh]      - 1-part + 7-part (uses P1 child 'a')
      
      → insert_patterns with all unique patterns from:
        1. Merge combinations using previously created partitions
        2. Root pattern child decompositions (P1, P2, P3 views)
      
      - Store this as the target partition result
  
  === 9-part partitions ===
  
    pos=0 to pos=9: "abcdefghi" - P3 WRAPPER
      
      Merge combinations:
        [abcdefgh, i]     - 8-part (target) + 1-part
        [abcdefg, hi]     - 7-part + 2-part
        [abcdef, ghi]     - 6-part + 3-part (uses P3 child 'ghi')
        [abcde, fghi]     - 5-part + 4-part
        [abcd, efghi]     - 4-part + 5-part (uses P3 child 'abcd')
      
      P3 child decomposition: [abcd, ef, ghi]
      
      → insert_patterns with all patterns
      → Replace P3[0..3] with this wrapper token
  
  === 10-part partitions ===
  
    pos=0 to pos=10: "abcdefghij" - P2 WRAPPER
      
      Merge combinations:
        [abcdefghi, j]    - 9-part + 1-part
        [abcdefgh, ij]    - 8-part (target) + 2-part
        [abcdefg, hij]    - 7-part + 3-part
        [abcdef, ghij]    - 6-part + 4-part
        [abcde, fghij]    - 5-part + 5-part (uses P2 child 'fghij')
        [ab, cdefghij]    - 2-part + 8-part (uses P2 child 'ab')
      
      P2 child decomposition: [ab, cde, fghij]
      
      → insert_patterns with all patterns
      → Replace P2[0..3] with this wrapper token
  
  === 12-part partitions (P1 wrapper = full root) ===
  
    pos=0 to pos=12: "abcdefghijkl" - P1 WRAPPER (entire root node)
      
      This IS the root node, so we ADD patterns rather than create new vertex.
      
      Merge combinations (added as new root patterns):
        [abcdefghij, kl]  - 10-part + 2-part (uses P2 child 'kl')
        [abcdefghi, jkl]  - 9-part + 3-part (uses P3 child 'jkl')
        [abcdefgh, ijkl]  - 8-part (target) + 4-part
        [a, bcdefghijkl]  - 1-part + 11-part (uses P1 child 'a')
        ... and other merge combinations
      
      P1 child decomposition already exists as root pattern: [a, bc, defg, hijkl]
      
      → Add new patterns to root node
      → Replace P1 pattern [a, bc, defg, hijkl] with [abcdefgh, ijkl]
        (or keep original and add new - depends on implementation)

  === MERGE COMPLETE ===
  
  All wrapper boundaries reached. Merge terminates.

Step 2: Return target partition token and show final state

  Target token: "abcdefgh" (created at 8-part stage)

  FINAL ROOT NODE STATE:
  
    Root "abcdefghijkl" patterns AFTER join:
      P1': [abcdefgh][ijkl]           ← wrapper replacement (was [a][bc][defg][hijkl])
      P2': [P2_wrapper][kl]           ← P2_wrapper replaces [ab][cde][fghij]
      P3': [P3_wrapper][jkl]          ← P3_wrapper replaces [abcd][ef][ghi]
    
    Where wrappers are:
    
      P2_wrapper "abcdefghij" has patterns (all from merge process):
        - [abcdefgh, ij]              ← 8-part (target) + 2-part
        - [abcdefghi, j]              ← 9-part + 1-part
        - [abcdefg, hij]              ← 7-part + 3-part
        - [ab, cde, fghij]            ← original P2 children in range
        - ... (other merge combinations)
      
      P3_wrapper "abcdefghi" has patterns (all from merge process):
        - [abcdefgh, i]               ← 8-part (target) + 1-part
        - [abcdefg, hi]               ← 7-part + 2-part
        - [abcdef, ghi]               ← 6-part + 3-part (uses P3 child 'ghi')
        - [abcd, ef, ghi]             ← original P3 children in range
        - ... (other merge combinations)

CONCLUSION for Prefix:

The prefix target creates wrappers that extend from the root start (pos=0)
to the first perfect boundary at or after the target offset in each pattern.

Each pattern may have a different wrapper boundary, leading to wrappers of
different sizes:
  - P3 wrapper ends at pos=9 → complement is "i"
  - P2 wrapper ends at pos=10 → complement is "ij"  
  - P1 wrapper ends at pos=12 → complement is "ijkl" (entire root, patterns added directly)

The merge terminates once all wrapper partitions are created. No partitions
beyond the outermost wrapper boundary need to be merged.
```

### 9.2 Postfix Example

```
Postfix target with NO perfect boundary, spanning inner boundaries in all patterns:

Root patterns BEFORE join (width=10, "abcdefghij"):
  P1: [ab][cde][fgh][ij]     → boundaries at 2, 5, 8, 10
  P2: [abcd][efg][hij]       → boundaries at 4, 7, 10
  P3: [a][bcdef][ghij]       → boundaries at 1, 6, 10

Boundary uniqueness check: 1, 2, 4, 5, 6, 7, 8, 10 ✓ (all unique)

Target: Postfix starting at pos=3 ("defghij")

Analysis - target offset at pos=3:
  P1: imperfect (inside 'cde', offset 1 from start)
      Target spans boundaries at pos=5 and pos=8
      Children covered: [de, fgh, ij] where 'de' is right-split of 'cde'
  P2: imperfect (inside 'abcd', offset 3 from start)
      Target spans boundaries at pos=4 and pos=7
      Children covered: [d, efg, hij] where 'd' is right-split of 'abcd'
  P3: imperfect (inside 'bcdef', offset 2 from start)
      Target spans boundary at pos=6
      Children covered: [def, ghij] where 'def' is right-split of 'bcdef'

No perfect boundary at pos=3 → all patterns need wrappers

Target spans inner boundaries in ALL patterns:
  P1: pos=5, 8 (2 boundaries)
  P2: pos=4, 7 (2 boundaries)
  P3: pos=6 (1 boundary)

Wrapper boundaries (first perfect boundary ≤ pos=3 in each pattern):
  P1: pos=2
  P2: pos=0 (start of root - wrapper covers entire pattern)
  P3: pos=1

Step 1: Join all partitions smallest to largest

  All partition boundaries:
    - pos=1 (P3 wrapper boundary)
    - pos=2 (P1 wrapper boundary)
    - pos=3 (TARGET offset)
    - pos=4 (P2 inner boundary)
    - pos=5 (P1 inner boundary)
    - pos=6 (P3 inner boundary)
    - pos=7 (P2 inner boundary)
    - pos=8 (P1 inner boundary)
  
  Sorted offsets: [1, 2, 3, 4, 5, 6, 7, 8]
  
  === 1-part partitions (between adjacent boundaries) ===
  
    pos=1 to pos=2: "b"       (trivial/atom)
    pos=2 to pos=3: "c"       (trivial/atom)
    pos=3 to pos=4: "d"       (trivial/atom)
    pos=4 to pos=5: "e"       (trivial/atom)
    pos=5 to pos=6: "f"       (trivial/atom)
    pos=6 to pos=7: "g"       (trivial/atom)
    pos=7 to pos=8: "h"       (trivial/atom)
    pos=8 to pos=10: "ij"     (matches P1 child [ij], use directly)
  
  === 2-part partitions ===
  
    pos=1 to pos=3: "bc"      - create from [b, c]
    pos=2 to pos=4: "cd"      - create from [c, d]
    pos=3 to pos=5: "de"      - create from [d, e]
    pos=4 to pos=6: "ef"      - create from [e, f]
    pos=5 to pos=7: "fg"      - create from [f, g]
    pos=6 to pos=8: "gh"      - create from [g, h]
    pos=7 to pos=10: "hij"    - matches P2 child [hij], use directly
  
  === 3-part partitions ===
  
    pos=1 to pos=4: "bcd"     - create from [bc, d], [b, cd]
    pos=2 to pos=5: "cde"     - matches P1 child [cde], use directly
    pos=3 to pos=6: "def"     - create from [de, f], [d, ef]
    pos=4 to pos=7: "efg"     - matches P2 child [efg], use directly
    pos=5 to pos=8: "fgh"     - matches P1 child [fgh], use directly
    pos=6 to pos=10: "ghij"   - matches P3 child [ghij], use directly
  
  === 4-part partitions ===
  
    pos=1 to pos=5: "bcde"    - create from [bcd, e], [bc, de], [b, cde]
    pos=2 to pos=6: "cdef"    - create from [cde, f], [cd, ef], [c, def]
    pos=3 to pos=7: "defg"    - create from [def, g], [de, fg], [d, efg]
    pos=4 to pos=8: "efgh"    - create from [efg, h], [ef, gh], [e, fgh]
    pos=5 to pos=10: "fghij"  - create from [fgh, ij], [fg, hij], [f, ghij]
  
  === 5-part partitions ===
  
    pos=1 to pos=6: "bcdef"   - matches P3 child [bcdef], use directly
    pos=2 to pos=7: "cdefg"   - create from [cdef, g], [cde, fg], [cd, efg]
    pos=3 to pos=8: "defgh"   - create from [defg, h], [def, gh], [de, fgh]
    pos=4 to pos=10: "efghij" - create from [efgh, ij], [efg, hij], [ef, ghij]
  
  === 6-part partitions ===
  
    pos=1 to pos=7: "bcdefg"  - create from [bcdef, g], [bcde, fg], [bcd, efg]
    pos=2 to pos=8: "cdefgh"  - create from [cdefg, h], [cdef, gh], [cde, fgh]
    pos=3 to pos=10: "defghij" ← TARGET PARTITION
      
      Merge combinations from smaller partitions:
        [d, efghij]       - 1-part + 5-part
        [de, fghij]       - 2-part + 4-part
        [def, ghij]       - 3-part + 3-part (uses P3 child 'ghij')
        [defg, hij]       - 4-part + 3-part (uses P2 child 'hij')
        [defgh, ij]       - 5-part + 2-part (uses P1 child 'ij')
      
      Plus patterns derived from each root pattern's child structure:
        - P1 view: [de, fgh, ij]
            Uses: 'de' (right-split of 'cde'), 'fgh' (P1 child), 'ij' (P1 child)
        - P2 view: [d, efg, hij]
            Uses: 'd' (right-split of 'abcd'), 'efg' (P2 child), 'hij' (P2 child)
        - P3 view: [def, ghij]
            Uses: 'def' (right-split of 'bcdef'), 'ghij' (P3 child)
      
      → insert_patterns with all unique patterns
      - Store this as the target partition result
  
  === 7-part partitions ===
  
    pos=1 to pos=8: "bcdefgh" - create from [bcdefg, h], [bcdef, gh], [bcde, fgh]
    pos=2 to pos=10: "cdefghij" - P1 WRAPPER
      
      Merge combinations:
        [c, defghij]      - 1-part + 6-part (target)
        [cd, efghij]      - 2-part + 5-part
        [cde, fghij]      - 3-part + 4-part (uses P1 child 'cde')
        [cdef, ghij]      - 4-part + 3-part
        [cdefg, hij]      - 5-part + 3-part
        [cdefgh, ij]      - 6-part + 2-part (uses P1 child 'ij')
      
      P1 child decomposition: [cde, fgh, ij]
      
      → insert_patterns with all patterns
      → Replace P1[1..4] with this wrapper token
  
  === 8-part partitions ===
  
    pos=1 to pos=10: "bcdefghij" - P3 WRAPPER
      
      Merge combinations:
        [bc, defghij]     - 2-part + 6-part (target)
        [bcd, efghij]     - 3-part + 5-part
        [bcde, fghij]     - 4-part + 4-part
        [bcdef, ghij]     - 5-part + 3-part (uses P3 child 'bcdef', 'ghij')
        [bcdefg, hij]     - 6-part + 3-part
        [bcdefgh, ij]     - 7-part + 2-part
      
      P3 child decomposition: [bcdef, ghij]
      
      → insert_patterns with all patterns
      → Replace P3[1..3] with this wrapper token
  
  === 10-part partitions (P2 wrapper = full root) ===
  
    pos=0 to pos=10: "abcdefghij" - P2 WRAPPER (entire root node)
      
      This IS the root node, so we ADD patterns rather than create new vertex.
      
      Merge combinations (added as new root patterns):
        [abc, defghij]    - 3-part + 6-part (target)
        [abcd, efghij]    - 4-part + 5-part (uses P2 child 'abcd')
        [a, bcdefghij]    - 1-part + 8-part (uses P3 child 'a')
        [ab, cdefghij]    - 2-part + 7-part (uses P1 child 'ab')
        ... and other merge combinations
      
      P2 child decomposition already exists as root pattern: [abcd, efg, hij]
      
      → Add new patterns to root node
      → Replace P2 pattern with wrapper pattern

  === MERGE COMPLETE ===
  
  All wrapper boundaries reached. Merge terminates.

Step 2: Return target partition token and show final state

  Target token: "defghij" (created at 6-part stage)

  FINAL ROOT NODE STATE:
  
    Root "abcdefghij" patterns AFTER join:
      P1': [ab][P1_wrapper]           ← P1_wrapper replaces [cde][fgh][ij]
      P2': [abc, defghij]             ← new pattern added (wrapper = root)
      P3': [a][P3_wrapper]            ← P3_wrapper replaces [bcdef][ghij]
    
    Where wrappers are:
    
      P1_wrapper "cdefghij" has patterns (all from merge process):
        - [c, defghij]                ← 1-part + 6-part (target)
        - [cd, efghij]                ← 2-part + 5-part
        - [cde, fghij]                ← 3-part + 4-part (uses P1 child 'cde')
        - [cdefgh, ij]                ← 6-part + 2-part (uses P1 child 'ij')
        - [cde, fgh, ij]              ← original P1 children in range
        - ... (other merge combinations)
      
      P3_wrapper "bcdefghij" has patterns (all from merge process):
        - [bc, defghij]               ← 2-part + 6-part (target)
        - [bcd, efghij]               ← 3-part + 5-part
        - [bcdef, ghij]               ← 5-part + 3-part (uses P3 children)
        - [bcde, fghij]               ← 4-part + 4-part
        - [bcdefgh, ij]               ← 7-part + 2-part
        - ... (other merge combinations)

CONCLUSION for Postfix:

The postfix target creates wrappers that extend from the closest perfect 
boundary at or before the target offset to the root end (pos=end) in each pattern.

Each pattern may have a different wrapper boundary, leading to wrappers of
different sizes:
  - P1 wrapper starts at pos=2 → complement is "c"
  - P3 wrapper starts at pos=1 → complement is "bc"
  - P2 wrapper starts at pos=0 → complement is "abc" (entire root, patterns added directly)

The merge terminates once all wrapper partitions are created. No partitions
outside the wrapper boundaries need to be merged.
```

### 9.3 Infix Example

```
=== INITIAL STATE ===

Root node (width=12): "abcdefghijkl"

Root patterns BEFORE join:
  P1: [ab][cde][fghi][jkl]     → boundaries at 2, 5, 9, 12
  P2: [abc][defgh][ijkl]       → boundaries at 3, 8, 12
  P3: [a][bcdef][ghij][kl]     → boundaries at 1, 6, 10, 12

All boundaries (sorted): 1, 2, 3, 5, 6, 8, 9, 10, 12
  - Each position has exactly ONE pattern with a perfect boundary there
  - Position 12 is the common root end (perfect in all patterns)

Target: Infix from pos=4 to pos=10 ("efghij")

=== TARGET BORDER ANALYSIS ===

Left border (pos=4) analysis:
  P1: imperfect (inside 'cde', offset 2 from child start at pos=2)
  P2: imperfect (inside 'defgh', offset 1 from child start at pos=3)
  P3: imperfect (inside 'bcdef', offset 3 from child start at pos=1)
  → Left border is IMPERFECT in ALL patterns

Right border (pos=10) analysis:
  P1: imperfect (inside 'jkl', offset 1 from child start at pos=9)
  P2: imperfect (inside 'ijkl', offset 2 from child start at pos=8)
  P3: PERFECT (at boundary between 'ghij' and 'kl')
  → Right border is PERFECT in P3 only

Target spans inner boundaries in each pattern:
  P1: pos=5 and pos=9 (crosses [cde][fghi][jkl] → gets [e, fghi, j])
  P2: pos=8 (crosses [defgh][ijkl] → gets [efgh, ij])
  P3: pos=6 (crosses [bcdef][ghij] → gets [ef, ghij])

This ensures inner partition joining is demonstrated.

=== WRAPPER BOUNDARY CALCULATION ===

For Infix, each pattern needs TWO wrapper boundaries:
  - Left wrapper boundary: closest perfect boundary ≤ left target offset (pos=4)
  - Right wrapper boundary: closest perfect boundary ≥ right target offset (pos=10)

Wrapper boundaries per pattern:
  P1: left=2 (boundary before target), right=12 (no boundary between 10 and 12)
  P2: left=3 (boundary before target), right=12 (no boundary between 10 and 12)
  P3: left=1 (boundary before target), right=10 (target end is already perfect)

Wrapper ranges:
  P1 wrapper: pos=2 to pos=12 → "cdefghijkl" (replaces children [cde][fghi][jkl])
  P2 wrapper: pos=3 to pos=12 → "defghijkl" (replaces children [defgh][ijkl])
  P3 wrapper: pos=1 to pos=10 → "bcdefghij" (replaces children [bcdef][ghij])

=== MERGE PROCESS (smallest to largest) ===

Step 1: Collect all relevant offsets

  Sorted offsets: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
  
  Source of each offset:
    - pos=1: P3 inner boundary (left wrapper boundary for P3)
    - pos=2: P1 inner boundary (left wrapper boundary for P1)
    - pos=3: P2 inner boundary (left wrapper boundary for P2)
    - pos=4: TARGET left offset
    - pos=5: P1 inner boundary (INSIDE target)
    - pos=6: P3 inner boundary (INSIDE target)
    - pos=8: P2 inner boundary (INSIDE target)
    - pos=9: P1 inner boundary (INSIDE target)
    - pos=10: TARGET right offset (P3 boundary, right wrapper for P3)
    - pos=12: root end (right wrapper boundary for P1, P2)
  
Step 2: Build partitions smallest to largest

  === 1-part partitions (atoms/existing children) ===
  
    pos=1 to pos=2: "b"       → trivial/atom
    pos=2 to pos=3: "c"       → trivial/atom
    pos=3 to pos=4: "d"       → trivial/atom
    pos=4 to pos=5: "e"       → trivial/atom
    pos=5 to pos=6: "f"       → trivial/atom
    pos=6 to pos=8: "gh"      → create from [g, h]
    pos=8 to pos=9: "i"       → trivial/atom
    pos=9 to pos=10: "j"      → trivial/atom
    pos=10 to pos=12: "kl"    → matches P3 child [kl]
  
  === 2-part partitions ===
  
    pos=1 to pos=3: "bc"      → create from [b, c]
    pos=2 to pos=4: "cd"      → create from [c, d]
    pos=3 to pos=5: "de"      → create from [d, e]
    pos=4 to pos=6: "ef"      → create from [e, f]
    pos=5 to pos=8: "fgh"     → create from [f, gh]
    pos=6 to pos=9: "ghi"     → create from [gh, i]
    pos=8 to pos=10: "ij"     → create from [i, j]
    pos=9 to pos=12: "jkl"    → matches P1 child [jkl]
  
  === 3-part partitions ===
  
    pos=1 to pos=4: "bcd"     → create from [bc, d], [b, cd]
    pos=2 to pos=5: "cde"     → matches P1 child [cde], use directly
    pos=3 to pos=6: "def"     → create from [de, f], [d, ef]
    pos=4 to pos=8: "efgh"    → create from [ef, gh], [e, fgh]
      (also matches right-split of P2 child 'defgh')
    pos=5 to pos=9: "fghi"    → matches P1 child [fghi], use directly
    pos=6 to pos=10: "ghij"   → matches P3 child [ghij], use directly
    pos=8 to pos=12: "ijkl"   → matches P2 child [ijkl], use directly
  
  === 4-part partitions ===
  
    pos=1 to pos=5: "bcde"    → create from [bcd, e], [bc, de], [b, cde]
    pos=2 to pos=6: "cdef"    → create from [cde, f], [cd, ef]
    pos=3 to pos=8: "defgh"   → matches P2 child [defgh], use directly
    pos=4 to pos=9: "efghi"   → create from [efgh, i], [ef, ghi], [e, fghi]
    pos=5 to pos=10: "fghij"  → create from [fghi, j], [fgh, ij], [f, ghij]
    pos=6 to pos=12: "ghijkl" → create from [ghij, kl], [ghi, jkl]
  
  === 5-part partitions ===
  
    pos=1 to pos=6: "bcdef"   → matches P3 child [bcdef], use directly
    pos=2 to pos=8: "cdefgh"  → create from [cde, fgh], [cdef, gh], [cd, efgh]
    pos=3 to pos=9: "defghi"  → create from [defgh, i], [def, ghi], [d, efghi]
    
    *** pos=4 to pos=10: "efghij" ← TARGET PARTITION ***
      Patterns (all merge combinations at target boundaries):
        - [e, fghi, j]    ← from P1 boundaries (e=right-split of cde, j=left-split of jkl)
        - [efgh, ij]      ← from P2 boundaries (efgh=right-split of defgh)
        - [ef, ghij]      ← from P3 boundaries (ef=right-split of bcdef)
      → insert_patterns([ [e, fghi, j], [efgh, ij], [ef, ghij] ])
      → Store this as TARGET result
    
    pos=5 to pos=12: "fghijkl" → create from [fghi, jkl], [fghij, kl]
  
  === 6-part partitions ===
  
    pos=1 to pos=8: "bcdefgh"  → create from [bcdef, gh], [bcde, fgh]
    pos=2 to pos=9: "cdefghi"  → create from [cdefgh, i], [cde, fghi]
    pos=3 to pos=10: "defghij" → create from [defghi, j], [defgh, ij], [d, efghij]
    pos=4 to pos=12: "efghijkl" → intermediate partition (P1 right portion)
      Patterns: [e, fghi, jkl], [efghij, kl]
      (will be used when building larger wrappers)
  
  === 7-part partitions ===
  
    pos=1 to pos=9: "bcdefghi"  → create from [bcdefgh, i], [bcdef, ghi]
    pos=2 to pos=10: "cdefghij" → create from [cdefghi, j], [cde, fghij]
  
  === 8-part partitions ===
  
    pos=1 to pos=10: "bcdefghij" → P3 WRAPPER (pos=1 to pos=10)
      Patterns (merge combinations): [bcdef, ghij], [bcd, efghij], [b, cdefghij]
      → insert_patterns and replace P3[1..3] with wrapper
    
    pos=2 to pos=12: "cdefghijkl" → P1 WRAPPER (pos=2 to pos=12)
      Patterns (merge combinations): [cde, fghi, jkl], [cd, efghij, kl], [c, defghijkl]
      → insert_patterns and replace P1[1..4] with wrapper
  
  === 9-part partitions ===
  
    pos=3 to pos=12: "defghijkl" → P2 WRAPPER (pos=3 to pos=12)
      Patterns (merge combinations): [defgh, ijkl], [d, efghij, kl], [d, efghijkl]
      → insert_patterns and replace P2[1..3] with wrapper

=== MERGE COMPLETE ===

Step 3: Return target partition token

  The target partition "efghij" was created at the 5-part partition stage.
  Return the token for "efghij" to the caller.

=== FINAL ROOT NODE STATE ===

Root node (width=12): "abcdefghijkl"

Root patterns AFTER join:
  P1: [ab][cdefghijkl_wrapper]       → boundaries at 2, 12
  P2: [abc][defghijkl_wrapper]       → boundaries at 3, 12
  P3: [a][bcdefghij_wrapper][kl]     → boundaries at 1, 10, 12

Wrapper contents (all patterns are merge combinations):

  P1_wrapper "cdefghijkl" (pos=2 to pos=12):
    - [cde, fghi, jkl]      ← original P1 children in wrapper range
    - [cd, efghij, kl]      ← left-complement + target + right-complement
    - [c, defghijkl]        ← 1-part + 9-part
    - [cdef, ghijkl]        ← 4-part + 6-part
    - ... (other merge combinations from boundaries at 3,4,5,6,8,9,10)

  P2_wrapper "defghijkl" (pos=3 to pos=12):
    - [defgh, ijkl]         ← original P2 children in wrapper range
    - [d, efghij, kl]       ← left-complement + target + right-complement
    - [d, efghijkl]         ← 1-part + 8-part
    - [def, ghijkl]         ← 3-part + 6-part
    - ... (other merge combinations from boundaries at 4,5,6,8,9,10)

  P3_wrapper "bcdefghij" (pos=1 to pos=10):
    - [bcdef, ghij]         ← original P3 children in wrapper range
    - [bcd, efghij]         ← left-complement + target (no right-complement needed)
    - [b, cdefghij]         ← 1-part + 8-part
    - [bcde, fghij]         ← 4-part + 5-part
    - ... (other merge combinations from boundaries at 2,3,4,5,6,8,9)

CONCLUSION for Infix:

The infix target requires wrappers that may extend in both directions:
  - Left: from closest perfect boundary ≤ left target offset
  - Right: to closest perfect boundary ≥ right target offset

When inner boundaries exist WITHIN the target range (pos=5, pos=6, pos=8, pos=9),
they create inner partitions that are joined during the merge process. This is 
where the smallest-to-largest algorithm shines - inner boundaries automatically
get merged into progressively larger partitions until the target is reached.

Key observation: The target partition's patterns capture splits at the target 
boundaries, while inner boundaries contribute to the STRUCTURE of how intermediate
partitions are built (e.g., [e, fghi, j] uses the inner boundary at pos=5 and pos=9).
```

---

## 10. Summary

| Concern | Approach |
|---------|----------|
| Partition joining | Use existing `JoinPartition` trait |
| Token creation | `insert_patterns` with single-token reuse |
| Pattern deduplication | `HashSet<Pattern>` before insertion |
| Delta tracking | Automatic via `JoinedPartition.delta` |
| Pattern extraction | `SplitPositionCache` + existing infrastructure |
| 3+ part patterns | Automatic via `JoinInnerRangeInfo` |

The core loop is "merge smallest to largest" using existing `JoinPartition` machinery.

---

## 11. Appendix: Code References

### Current Implementation (to be replaced)
- `join_partitions` (non-root): `context/node/context.rs:157-184`
- `join_root_partitions`: `context/node/context.rs:185-210`  
- `join_incomplete_infix`: `context/node/context.rs:212-350` (**REMOVE**)
- `join_incomplete_postfix`: `context/node/context.rs:352-457` (**REMOVE**)
- `join_incomplete_prefix`: `context/node/context.rs:459-551` (**REMOVE**)

### Infrastructure to Keep
- `JoinPartition` trait: `partition/mod.rs`
- `JoinedPartition`: `joined/partition.rs`
- `JoinedPatterns`: `joined/patterns.rs`
- `JoinPartitionInfo`: `partition/info/mod.rs`
- `JoinInnerRangeInfo`: `partition/info/inner_range.rs`
- `JoinPatternInfo`: `partition/info/pattern_info.rs`
- `JoinBorders`: `context/pattern/borders.rs`
- `NodeMergeCtx`: `context/node/merge.rs`
- `FrontierSplitIterator`: `context/frontier.rs`
- `SplitVertexCache`: `split/cache/vertex.rs`
- `augment_root`: `split/cache/vertex.rs:72-107`
