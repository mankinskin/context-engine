---
tags: `#plan` `#algorithm` `#debugging` `#testing` `#refactoring`
summary: The current `join_root_partitions` implementation has grown complex with three separate `join_incomplete_*` functions that share structural similar...
status: 📋
---

# Specification: Root Node Join Refactoring

**Date:** 2025-01-03  
**Status:** DRAFT - READY FOR IMPLEMENTATION PLANNING  
**Related:** `20251211_PLAN_wrapper_partition_calculation.md`, `context.rs`

## 1. Problem Statement

The current `join_root_partitions` implementation has grown complex with three separate `join_incomplete_*` functions that share structural similarities but differ in specific details. The code is:
- Hard to maintain (duplication across prefix/postfix/infix cases)
- Uses imperative loops rather than functional patterns
- Lacks clear separation of the common algorithm from type-specific operations

### Goals:
1. **Generic Parameterization**: Use traits to abstract over Prefix/Postfix/Infix target partitions
2. **Unified Algorithm**: Define common root join steps, specializing only where needed
3. **Functional Paradigms**: Replace loops with iterator chains
4. **Clear Data Structures**: Design types that make navigation through offsets/partitions intuitive

---

## 2. Key Concepts & Invariants

### 2.1 Graph Invariants (Critical for Understanding)

1. **Unique Child Boundaries**: All child boundaries in all child patterns of any node are at **different atom positions**. No two child boundaries may share the same atom offset.

2. **No Duplicate Ranges**: No range of children can exist twice anywhere in the graph. When joining a partition:
   - If a single token already exists for the partition range → reuse it
   - If no single token exists → create a new vertex from the child pattern cut-out

3. **Wrapper Offset Uniqueness**: Since wrapper offsets are at perfect child boundaries, and all child boundaries are at unique positions, **each wrapper offset is perfect in exactly one pattern**.

### 2.2 Partition Types

| Partition Type | Definition | When Different from Target |
|---------------|------------|---------------------------|
| **Target Partition** | The partition defined by target offset(s) from the search result | N/A - this is the goal |
| **Inner Partition** | Partition from perfect boundaries *inside* the target | When target offset is imperfect in that pattern |
| **Wrapper Partition** | Partition from perfect boundaries *around* the target | When target offset is imperfect in that pattern |

**Key insight**: Each child pattern has its own inner and wrapper partitions. These only differ from the target partition when the target offset is not perfect (not at a child boundary) in that specific pattern.

### 2.3 Offset Types

| Offset Type | Description |
|------------|-------------|
| **Target Offset** | Defines the target partition boundary (from search result) |
| **Inner Offset** | Perfect child boundary inside the target partition range |
| **Wrapper Offset** | Perfect child boundary outside/at the target partition range |

**Important**: An offset can be multiple types simultaneously. A target offset that happens to be at a perfect child boundary is also a wrapper offset (and potentially an inner offset boundary).

---

## 3. Current Architecture Analysis

### 3.1 Non-Root Node Join (`join_partitions`)

The non-root flow:
```
1. Clone vertex cache (all split positions)
2. Create iterator over positions
3. Join ALL partitions between splits:
   - First: Prefix (before first split)
   - Middle: Infix (between adjacent splits)
   - Last: Postfix (after last split)
4. Assert widths sum correctly
5. Merge partitions via NodeMergeCtx::merge_node
   - Creates left/right pairs for each split position
   - Builds composite patterns for each offset
```

**Key insight**: Non-root nodes join ALL partitions because parent nodes may reference ANY partition from either side of any split. The merge step creates the split mapping (left/right tokens for each offset position).

### 3.2 Root Node Join (`join_root_partitions`)

The root flow:
```
1. Get RootMode (Prefix/Postfix/Infix)
2. Get vertex cache offsets (includes wrapper offsets from augmentation)
3. Extract target offset(s) from offsets
4. Join partitions in order: smallest to largest (respecting delta accumulation)
5. For each pattern:
   - If target offset is perfect: use target partition directly
   - If target offset is imperfect: create wrapper vertex
6. Return target partition token
```

**Key insight**: Root nodes only NEED the target partition indexed. Wrapper vertices integrate the target into patterns where the target offset isn't at a perfect child boundary.

### 3.3 Augmentation Phase (Pre-Join)

During `augment_root`:
1. Add inner offsets for the target partition
2. **Add wrapper offsets** at perfect child boundaries around the target
3. These wrapper offsets ensure we have split positions at perfect boundaries
4. Wrapper offsets trigger splits in child vertices during bottom-up joining

**Critical**: All wrapper offset calculation happens during augmentation. By the time we reach root joining, all child splits are already computed.

### 3.4 Split Cache Structure

```rust
SplitVertexCache {
    positions: BTreeMap<NonZeroUsize, SplitPositionCache>
}

SplitPositionCache {
    top: HashSet<PosKey>,              // Parent positions referencing this
    pattern_splits: TokenTracePositions // PatternId -> TokenTracePos
}

TokenTracePos {
    sub_index: usize,                   // Child index in pattern
    inner_offset: Option<NonZeroUsize>  // Offset within child (None = perfect boundary)
}
```

---

## 4. Delta Propagation Model

### 4.1 What is Delta?

When multiple children are joined into a single token, the pattern length decreases. **Delta** tracks this reduction per pattern:
- `delta[pattern_id] = original_entry_count - 1`
- Used to adjust sub-indices when accessing positions after a joined range

### 4.2 Join Order & Delta Accumulation

Just like non-root joining, root joining must:
1. **Join smaller partitions first** (inner partitions before target, target before wrapper)
2. **Track deltas** from each join operation
3. **Apply accumulated deltas** to sub-indices when building larger partitions

Example flow for Postfix target:
```
1. Join inner partitions (if any) → accumulate inner_delta
2. Join target partition (using adjusted indices) → get target_delta  
3. Join wrapper complement (using adjusted indices) → get wrapper patterns
4. Create wrapper vertex with target + complement patterns
```

### 4.3 Delta Application

When accessing `pattern_splits[pid].sub_index()` for a later partition:
```rust
let adjusted_offset = offset.split.clone() - accumulated_delta;
let sub_index = adjusted_offset.pattern_splits[&pid].sub_index();
```

---

## 5. Proposed Data Structures

### 5.1 Root Join Context

```rust
/// Complete context for root join operation
pub struct RootJoinContext<'a, R: TargetRole> {
    /// Reference to node join context
    ctx: &'a mut NodeJoinCtx<'_>,
    
    /// All offsets from vertex cache (sorted by position)
    /// Includes target offsets + wrapper offsets + inner offsets
    all_offsets: Vec<SplitOffset>,
    
    /// Index/indices of target offset(s) within all_offsets
    target_indices: R::TargetIndices,
    
    /// Accumulated deltas from joined partitions (per pattern)
    delta_accumulator: PatternSubDeltas,
}
```

### 5.2 Offset Navigation

```rust
/// Represents a split position with its pattern-level information
#[derive(Debug, Clone)]
pub struct SplitOffset {
    pub position: NonZeroUsize,
    pub pattern_info: TokenTracePositions,
}

impl SplitOffset {
    /// Check if this offset is perfect (at child boundary) in the given pattern
    pub fn is_perfect_in(&self, pattern_id: &PatternId) -> bool {
        self.pattern_info
            .get(pattern_id)
            .map(|pos| pos.inner_offset().is_none())
            .unwrap_or(false)
    }
    
    /// Get the pattern where this offset is perfect (exactly one by invariant)
    pub fn perfect_pattern(&self) -> Option<PatternId> {
        self.pattern_info
            .iter()
            .find(|(_, pos)| pos.inner_offset().is_none())
            .map(|(pid, _)| *pid)
    }
}

/// Per-pattern partition bounds determined by offsets
#[derive(Debug, Clone)]
pub struct PatternPartitionBounds {
    pub pattern_id: PatternId,
    /// Start child index in this pattern
    pub start_index: usize,
    /// End child index (exclusive) in this pattern  
    pub end_index: usize,
    /// Whether start is at perfect boundary
    pub perfect_start: bool,
    /// Whether end is at perfect boundary
    pub perfect_end: bool,
}
```

### 5.3 Partition Join Result

```rust
/// Result of joining any partition (reuse existing or create new)
pub enum PartitionResult {
    /// Existing token found for this range
    Existing(Token),
    /// New vertex created from patterns
    Created {
        token: Token,
        delta: PatternSubDeltas,
    },
}

impl PartitionResult {
    pub fn token(&self) -> Token {
        match self {
            Self::Existing(t) => *t,
            Self::Created { token, .. } => *token,
        }
    }
    
    pub fn delta(&self) -> PatternSubDeltas {
        match self {
            Self::Existing(_) => Default::default(),
            Self::Created { delta, .. } => delta.clone(),
        }
    }
}
```

---

## 6. Proposed Trait Hierarchy

### 6.1 Target Role Trait

```rust
/// Defines the target partition type (Prefix/Postfix/Infix)
pub trait TargetRole: Sized {
    /// Type representing target offset indices within the sorted offset list
    /// - Prefix/Postfix: single index
    /// - Infix: pair of indices (left, right)
    type TargetIndices;
    
    /// Perfect boundary type
    type Perfect: BorderPerfect;
    
    /// Number of target offsets (1 for Prefix/Postfix, 2 for Infix)
    const TARGET_OFFSET_COUNT: usize;
    
    /// Identify target offset indices from RootMode
    fn identify_target_indices(
        offsets: &[SplitOffset],
        root_mode: RootMode,
    ) -> Self::TargetIndices;
    
    /// Get iterator over patterns needing wrapper (imperfect target boundaries)
    fn patterns_needing_wrapper<'a>(
        offsets: &'a [SplitOffset],
        target_indices: &Self::TargetIndices,
    ) -> impl Iterator<Item = PatternId> + 'a;
    
    /// Get wrapper bounds for a specific pattern
    fn wrapper_bounds(
        offsets: &[SplitOffset],
        target_indices: &Self::TargetIndices,
        pattern_id: PatternId,
    ) -> PatternPartitionBounds;
}
```

### 6.2 Target Role Implementations

```rust
impl TargetRole for Pre<Join> {
    type TargetIndices = usize;  // Index of right boundary
    type Perfect = SinglePerfect;
    const TARGET_OFFSET_COUNT: usize = 1;
    
    fn identify_target_indices(offsets: &[SplitOffset], _: RootMode) -> usize {
        // For prefix: target offset is the first offset
        0
    }
    
    fn patterns_needing_wrapper<'a>(
        offsets: &'a [SplitOffset],
        target_idx: &usize,
    ) -> impl Iterator<Item = PatternId> + 'a {
        // Patterns where target offset is NOT perfect
        offsets[*target_idx].pattern_info.iter()
            .filter(|(_, pos)| pos.inner_offset().is_some())
            .map(|(pid, _)| *pid)
    }
}

impl TargetRole for Post<Join> {
    type TargetIndices = usize;  // Index of left boundary
    type Perfect = SinglePerfect;
    const TARGET_OFFSET_COUNT: usize = 1;
    
    fn identify_target_indices(offsets: &[SplitOffset], _: RootMode) -> usize {
        // For postfix: target offset is the first (only) offset
        0
    }
}

impl TargetRole for In<Join> {
    type TargetIndices = (usize, usize);  // (left_idx, right_idx)
    type Perfect = DoublePerfect;
    const TARGET_OFFSET_COUNT: usize = 2;
    
    fn identify_target_indices(offsets: &[SplitOffset], _: RootMode) -> (usize, usize) {
        // For infix: first two offsets define target
        (0, 1)
    }
    
    fn patterns_needing_wrapper<'a>(
        offsets: &'a [SplitOffset],
        (left_idx, right_idx): &(usize, usize),
    ) -> impl Iterator<Item = PatternId> + 'a {
        // Patterns where EITHER target offset is imperfect
        let left_imperfect: HashSet<_> = offsets[*left_idx].pattern_info.iter()
            .filter(|(_, pos)| pos.inner_offset().is_some())
            .map(|(pid, _)| *pid)
            .collect();
        let right_imperfect: HashSet<_> = offsets[*right_idx].pattern_info.iter()
            .filter(|(_, pos)| pos.inner_offset().is_some())
            .map(|(pid, _)| *pid)
            .collect();
        left_imperfect.union(&right_imperfect).copied()
    }
}
```

### 6.3 Partition Joiner Trait

```rust
/// Handles joining partitions between offsets
pub trait PartitionJoiner {
    /// Join a partition defined by offset range, respecting deltas
    fn join_partition_range(
        &mut self,
        start_offset: Option<&SplitOffset>,  // None = start of node
        end_offset: Option<&SplitOffset>,    // None = end of node
        delta: &PatternSubDeltas,
    ) -> PartitionResult;
}
```

---

## 7. Proposed Algorithm Steps

### 7.1 High-Level Algorithm

```
join_root_partitions<R: TargetRole>(ctx):
    1. Collect all offsets from vertex cache (sorted by position)
    2. Identify target offset indices based on RootMode
    3. For each pattern:
       a. Determine if target offsets are perfect in this pattern
       b. If all perfect: target partition directly usable
       c. If any imperfect: need to build wrapper
    4. Join partitions in size order (smallest to largest):
       a. Inner partitions (between offsets inside target range)
       b. Target partition
       c. Wrapper complements (partitions between target and wrapper bounds)
    5. For each pattern needing wrapper:
       a. Create wrapper vertex with [complement, target] or [target, complement] patterns
       b. Also include any alternative patterns from child structure
       c. Replace wrapper range in root pattern
    6. Return target partition token
```

### 7.2 Detailed Steps

#### Step 1: Initialize and Collect Offsets

```rust
fn init_root_join<R: TargetRole>(
    ctx: &mut NodeJoinCtx,
) -> RootJoinContext<R> {
    let cache = ctx.vertex_cache();
    
    // All offsets sorted by position (includes target + wrapper + inner)
    let all_offsets: Vec<SplitOffset> = cache.iter()
        .map(|(pos, cache)| SplitOffset {
            position: *pos,
            pattern_info: cache.pattern_splits.clone(),
        })
        .collect();
    
    let target_indices = R::identify_target_indices(&all_offsets, root_mode);
    
    RootJoinContext {
        ctx,
        all_offsets,
        target_indices,
        delta_accumulator: Default::default(),
    }
}
```

#### Step 2: Partition Join Order

For proper delta accumulation, partitions must be joined from smallest to largest:

```rust
/// Generate join order for all partitions
fn partition_join_order<R: TargetRole>(
    all_offsets: &[SplitOffset],
    target_indices: &R::TargetIndices,
) -> Vec<PartitionSpec> {
    // 1. Inner partitions (between adjacent offsets within target range)
    // 2. Target partition
    // 3. Wrapper partitions (for each pattern needing wrapper)
    
    // The key is: smaller partitions first, so their deltas can be applied
    // to larger partition calculations
}
```

#### Step 3: Join Target Partition

```rust
fn join_target<R: TargetRole>(
    root_ctx: &mut RootJoinContext<R>,
) -> PartitionResult {
    let partition = match R::TARGET_OFFSET_COUNT {
        1 => {
            let offset = &root_ctx.all_offsets[root_ctx.target_indices];
            // Create Prefix or Postfix partition
        }
        2 => {
            let (left, right) = root_ctx.target_indices;
            // Create Infix partition from two offsets
        }
    };
    
    // Apply accumulated delta before joining
    let adjusted_partition = partition.apply_delta(&root_ctx.delta_accumulator);
    
    // Join: either find existing token or create new vertex
    root_ctx.ctx.join_partition(adjusted_partition)
}
```

#### Step 4: Create Wrappers for Imperfect Patterns

```rust
fn create_pattern_wrapper<R: TargetRole>(
    root_ctx: &mut RootJoinContext<R>,
    pattern_id: PatternId,
    target_token: Token,
) {
    let bounds = R::wrapper_bounds(
        &root_ctx.all_offsets,
        &root_ctx.target_indices,
        pattern_id,
    );
    
    // Join complement partition(s)
    let complement = join_complement(root_ctx, pattern_id, &bounds);
    
    // Build wrapper patterns:
    // - Target pattern: [complement, target] or [target, complement] (target detected and stored)
    // - Other patterns: merge combinations derived from original child structure
    let wrapper_patterns = build_wrapper_patterns(
        target_token,
        complement,
        &bounds,
        root_ctx.ctx,
    );
    
    // Create wrapper vertex
    let wrapper = root_ctx.ctx.trav.insert_patterns(wrapper_patterns);
    
    // Replace in root pattern
    let loc = root_ctx.ctx.index.to_pattern_location(pattern_id);
    root_ctx.ctx.trav.replace_in_pattern(
        loc,
        bounds.start_index..bounds.end_index,
        vec![wrapper],
    );
}
```

#### Step 5: Main Entry Point

```rust
pub fn join_root_partitions_generic<R: TargetRole>(
    ctx: &mut NodeJoinCtx,
    root_mode: RootMode,
) -> Token {
    let mut root_ctx = init_root_join::<R>(ctx, root_mode);
    
    // Join inner partitions first (accumulates delta)
    join_inner_partitions(&mut root_ctx);
    
    // Join target partition
    let target_result = join_target::<R>(&mut root_ctx);
    let target_token = target_result.token();
    root_ctx.delta_accumulator.merge(target_result.delta());
    
    // Create wrappers for patterns with imperfect target boundaries
    R::patterns_needing_wrapper(&root_ctx.all_offsets, &root_ctx.target_indices)
        .for_each(|pid| {
            create_pattern_wrapper::<R>(&mut root_ctx, pid, target_token);
        });
    
    target_token
}
```

---

## 8. Examples

This section provides detailed worked examples for each target type (Prefix, Postfix, Infix),
demonstrating the smallest-to-largest merge algorithm with delta tracking.

**Key constraint**: During joining, the target partition spans **at least one child boundary 
in every pattern**. This is because the search phase would have stopped at a smaller 
containing parent otherwise.

### 8.1 Prefix Example

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

=== PARTITION DEFINITIONS ===

Inner partitions (from closest perfect boundary at/inside target to target end):
  For Prefix, inner partition spans from pos=0 to the closest perfect boundary ≤ target offset.
  
  P1: (0, 7) = "abcdefg" = [a, bc, defg]     ← already exists as child sequence
  P2: (0, 5) = "abcde"   = [ab, cde]         ← already exists as child sequence
  P3: (0, 6) = "abcdef"  = [abcd, ef]        ← already exists as child sequence

  All inner partitions use existing children - no joining needed.

Target partition:
  (0, 8) = "abcdefgh"
  
  Patterns from each root pattern's view:
    P1: [a, bc, defg, h]     where 'h' is left-split of 'hijkl'
    P2: [ab, cde, fgh]       where 'fgh' is left-split of 'fghij'
    P3: [abcd, ef, gh]       where 'gh' is left-split of 'ghi'

Wrapper partitions (from pos=0 to wrapper boundary):
  P1: (0, 12) = "abcdefghijkl" = entire root (patterns added to root, no separate wrapper)
  P2: (0, 10) = "abcdefghij"   = [target, ij] = [abcdefgh, ij]
  P3: (0, 9)  = "abcdefghi"    = [target, i]  = [abcdefgh, i]

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

### 8.2 Postfix Example

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

=== PARTITION DEFINITIONS ===

Inner partitions (from closest perfect boundary ≥ target offset to root end):
  For Postfix, inner partition spans from closest perfect boundary ≥ target offset to pos=end.
  
  P1: (5, 10) = "fghij"  = [fgh, ij]       ← needs joining (2 children)
  P2: (4, 10) = "efghij" = [efg, hij]      ← needs joining (2 children, but 'efg' doesn't exist yet)
  P3: (6, 10) = "ghij"   = [ghij]          ← already exists as single child

  Inner partitions P1 and P2 require joining before target can be built.

Target partition:
  (3, 10) = "defghij"
  
  Patterns from each root pattern's view:
    P1: [de, fgh, ij]     where 'de' is right-split of 'cde'
        Or using inner: [de, inner_P1] = [de, fghij]
    P2: [d, efg, hij]     where 'd' is right-split of 'abcd'
        Or using inner: [d, inner_P2] = [d, efghij]
    P3: [def, ghij]       where 'def' is right-split of 'bcdef'
        Or using inner: [def, inner_P3] = [def, ghij]

Wrapper partitions (from wrapper boundary to pos=end):
  P1: (2, 10) = "cdefghij"   = [c, target] = [c, defghij]
  P2: (0, 10) = "abcdefghij" = entire root (patterns added to root, no separate wrapper)
  P3: (1, 10) = "bcdefghij"  = [bc, target] = [bc, defghij] where 'bc' is right-split of 'bcdef'

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

### 8.3 Infix Example

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

=== PARTITION DEFINITIONS ===

Inner partitions (from closest perfect boundary ≥ left target to closest ≤ right target):
  For Infix, inner partition spans between the perfect boundaries closest to each target offset.
  
  P1: (5, 9)  = "fghi"   = [fghi]           ← already exists as single child
  P2: (3, 8)  = "defgh"  = [defgh]          ← already exists as single child  
  P3: (6, 10) = "ghij"   = [ghij]           ← already exists as single child

  All inner partitions are existing children - no joining needed.

Target partition:
  (4, 10) = "efghij"
  
  Patterns from each root pattern's view:
    P1: [e, fghi, j]      where 'e' is right-split of 'cde', 'j' is left-split of 'jkl'
        Or using inner: [e, inner_P1, j] = [e, fghi, j]
    P2: [efgh, ij]        where 'efgh' is right-split of 'defgh', 'ij' is left-split of 'ijkl'
        Or using inner: [efgh, ij] (inner_P2 'defgh' doesn't help here - target starts inside it)
    P3: [ef, ghij]        where 'ef' is right-split of 'bcdef'
        Or using inner: [ef, inner_P3] = [ef, ghij]

Wrapper partitions (from left wrapper boundary to right wrapper boundary):
  P1: (2, 12)  = "cdefghijkl" = [cd, target, kl] = [cd, efghij, kl]
      where 'cd' is right-split of 'cde', 'kl' is right-split of 'jkl'
  P2: (3, 12)  = "defghijkl"  = [d, target, kl] = [d, efghij, kl]  
      where 'd' is right-split of 'defgh', 'kl' is right-split of 'ijkl'
  P3: (1, 10)  = "bcdefghij"  = [bcd, target] = [bcd, efghij]
      where 'bcd' is right-split of 'bcdef' (right boundary is already perfect)

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

## 9. Remaining Questions

### Q1: Offset Ordering in SplitVertexCache [ANSWERED]

**Question**: How are offsets ordered relative to their roles?

**Answer**: Offsets are sorted by position value (BTreeMap). Role identification uses 
**Option A: RootMode + position relative to known target offset(s)**.

The algorithm knows the target offset(s) because they're the input to the join operation.
Given the RootMode, wrapper offsets are identified by their position relative to target:

```
Role identification by RootMode:

  Prefix (single target offset at pos=T):
    - Target offset: pos=T (the input)
    - Wrapper offsets: all positions > T (extending toward root end)
    - Example from 8.1: T=8, wrappers at 9, 10, 12
    
  Postfix (single target offset at pos=T):
    - Target offset: pos=T (the input)
    - Wrapper offsets: all positions < T (extending toward root start)
    - Example from 8.2: T=3, wrappers at 0, 1, 2
    
  Infix (two target offsets at pos=L and pos=R):
    - Target offsets: pos=L and pos=R (the inputs)
    - Left wrapper offsets: all positions < L
    - Right wrapper offsets: all positions > R
    - Example from 8.3: L=4, R=10, left wrappers at 1,2,3, right wrappers at 12

No metadata storage needed during augmentation - roles are implicit from position.
```

**Implementation approach**:
```rust
fn classify_offset(pos: usize, mode: RootMode, target_bounds: (usize, Option<usize>)) -> OffsetRole {
    let (left_target, right_target) = target_bounds;
    match mode {
        RootMode::Prefix => {
            if pos == left_target { OffsetRole::Target }
            else if pos > left_target { OffsetRole::Wrapper }
            else { OffsetRole::Inner }  // shouldn't occur for Prefix
        }
        RootMode::Postfix => {
            if pos == left_target { OffsetRole::Target }
            else if pos < left_target { OffsetRole::Wrapper }
            else { OffsetRole::Inner }  // shouldn't occur for Postfix
        }
        RootMode::Infix => {
            let right = right_target.unwrap();
            if pos == left_target || pos == right { OffsetRole::Target }
            else if pos < left_target { OffsetRole::LeftWrapper }
            else if pos > right { OffsetRole::RightWrapper }
            else { OffsetRole::Inner }  // inside target range
        }
    }
}
```

---

## 10. Implementation Checklist

1. [ ] **Define helper data structures**
   - `SplitOffset` with pattern info and perfect-check methods
   - `PatternPartitionBounds` for wrapper ranges
   - `PartitionResult` enum for join results

2. [ ] **Implement `TargetRole` trait**
   - `impl TargetRole for Pre<Join>`
   - `impl TargetRole for Post<Join>`
   - `impl TargetRole for In<Join>`

3. [ ] **Implement `RootJoinContext`**
   - Initialization from vertex cache
   - Delta accumulation
   - Offset access methods

4. [ ] **Implement join functions**
   - `join_inner_partitions` (with delta tracking)
   - `join_target` (generic over TargetRole)
   - `join_complement` (for wrapper building)

5. [ ] **Implement wrapper creation**
   - `create_pattern_wrapper` (builds wrapper vertex)
   - `build_wrapper_patterns` (target + merge combination patterns)

6. [ ] **Refactor entry point**
   - Replace `join_root_partitions` match with generic dispatch
   - Remove `join_incomplete_*` functions

7. [ ] **Testing**
   - Unit tests for each TargetRole
   - Integration tests with various perfect/imperfect scenarios
   - Regression tests for existing behavior

---

## Appendix: Code References

- `join_partitions` (non-root): `context.rs:157-184`
- `join_root_partitions`: `context.rs:185-210`  
- `join_incomplete_infix`: `context.rs:212-350`
- `join_incomplete_postfix`: `context.rs:352-457`
- `join_incomplete_prefix`: `context.rs:459-551`
- `SplitVertexCache`: `split/cache/vertex.rs`
- `augment_root`: `split/cache/vertex.rs:72-107`
- `JoinedPartition`: `join/joined/partition.rs`
