---
tags: `#plan` `#context-insert` `#debugging` `#testing`
summary: > **Status:** Implementation in Progress
status: 📋
---

# Selective Partition Merge - Implementation Strategy

> **Status:** Implementation in Progress  
> **Created:** 2026-01-27  
> **Depends On:** [20260127_SELECTIVE_PARTITION_MERGE.md](20260127_SELECTIVE_PARTITION_MERGE.md)

## Implementation Approach

### Strategy: Top-Down Required Partition Labeling

Compute the required partition set during the augmentation phase (`root_augmentation`), starting from wrapper partitions and labeling downward.

## Current Code Analysis

### Key Flow
1. `root_augmentation()` in `split/cache/vertex.rs`:
   - Computes `target_positions` (original split offsets)
   - Adds inner offsets via `add_inner_offsets()`
   - Adds wrapper offsets via `add_wrapper_offsets_*()` 
   - Returns `(Vec<SplitTraceState>, PartitionRange)` where `PartitionRange` is `target_range`

2. `merge_sub_partitions()` in `join/context/node/merge/context.rs`:
   - Iterates ALL partition ranges in the operating range
   - Creates tokens for EVERY combination (the bug)
   - Returns `(Token, RangeMap)`

### Insertion Points
1. **After offset augmentation:** `root_augmentation()` already knows:
   - `target_range` - the target partition
   - `wrapper_splits` - the wrapper boundary offsets
   - From these we can compute all required partitions

2. **Before merge iteration:** Pass `RequiredPartitions` to `merge_sub_partitions()`

3. **In merge loop:** Skip partition ranges not in required set

## Phase 1: Data Structure

### New Type: `RequiredPartitions`

**Location:** `crates/context-insert/src/join/context/node/merge/required.rs`

```rust
use super::PartitionRange;
use std::collections::HashSet;

/// Tracks which partition ranges require token creation.
/// 
/// A partition is required if:
/// 1. It is the target partition
/// 2. It is a wrapper partition (for unperfect splits)
/// 3. It is an inner partition (prevents repetition at unperfect splits)
/// 4. It is an overlap between two required partitions
#[derive(Debug, Clone, Default)]
pub struct RequiredPartitions {
    required: HashSet<PartitionRange>,
}

impl RequiredPartitions {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Mark a partition range as required
    pub fn add(&mut self, range: PartitionRange) {
        self.required.insert(range);
    }
    
    /// Check if a partition range is required
    pub fn is_required(&self, range: &PartitionRange) -> bool {
        self.required.contains(range)
    }
    
    /// Compute all overlaps between currently required partitions
    /// and add them to the required set. Repeats until fixed point.
    pub fn close_under_overlaps(&mut self) {
        loop {
            let current: Vec<_> = self.required.iter().cloned().collect();
            let before = self.required.len();
            
            for i in 0..current.len() {
                for j in (i+1)..current.len() {
                    if let Some(overlap) = current[i].overlap(&current[j]) {
                        self.required.insert(overlap);
                    }
                }
            }
            
            if self.required.len() == before {
                break;
            }
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &PartitionRange> {
        self.required.iter()
    }
}
```

### Add `overlap()` to `PartitionRange`

**Location:** `crates/context-insert/src/join/context/node/merge/partition_range.rs`

```rust
impl PartitionRange {
    /// Compute the overlap of two partition ranges, if it exists and is 
    /// a proper subset of both ranges.
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let start = (*self.start()).max(*other.start());
        let end = (*self.end()).min(*other.end());
        
        // Must be non-empty and a proper subset of at least one range
        if start <= end {
            let overlap = Self::new(start..=end);
            // Only return if it's a proper subset (not equal to either input)
            if overlap != *self && overlap != *other {
                return Some(overlap);
            }
        }
        None
    }
}
```

## Phase 2: Compute Required Set in Augmentation

### Modify `root_augmentation()` 

After computing all offsets, compute required partitions:

```rust
pub fn root_augmentation(
    &mut self,
    ctx: NodeTraceCtx,
    root_mode: RootMode,
) -> (Vec<SplitTraceState>, PartitionRange, RequiredPartitions) {
    // ... existing code to compute positions and target_range ...
    
    // Compute required partitions
    let required = self.compute_required_partitions(
        &target_range,
        root_mode,
        &wrapper_ranges,  // Need to track these
    );
    
    (next, target_range, required)
}

fn compute_required_partitions(
    &self,
    target_range: &PartitionRange,
    root_mode: RootMode,
    wrapper_ranges: &[PartitionRange],
) -> RequiredPartitions {
    let mut required = RequiredPartitions::new();
    
    // 1. Target is always required
    required.add(target_range.clone());
    
    // 2. Wrappers are required (one per pattern with unperfect split)
    for wrapper in wrapper_ranges {
        required.add(wrapper.clone());
    }
    
    // 3. Compute overlaps until fixed point (handles inner partitions)
    // Inner partitions ARE overlaps: `ab` is overlap of `aby` and `abyz`
    required.close_under_overlaps();
    
    required
}
```

### Track Wrapper Ranges

In `add_wrapper_offsets_*()` functions, track which partition ranges are wrappers.

**Key insight:** The wrapper partition range is from target_range.start() to the wrapper boundary offset index.

## Phase 3: Filter Merge Loop

### Modify `merge_sub_partitions()`

```rust
pub fn merge_sub_partitions(
    &mut self,
    target_range: Option<PartitionRange>,
    required: &RequiredPartitions,  // NEW PARAMETER
) -> (Token, RangeMap) {
    // ... existing setup ...

    for len in 1..=op_len {
        for start in op_start..=(op_start + op_len - len) {
            let end = start + len - 1;
            let partition_range = PartitionRange::new(start..=end);
            
            // NEW: Skip non-required partitions
            if !required.is_required(&partition_range) {
                debug!(?partition_range, "Skipping non-required partition");
                continue;
            }

            // ... existing merge logic ...
        }
    }
    
    // ...
}
```

## Phase 4: Thread Through Call Chain

Need to pass `RequiredPartitions` from augmentation to merge:

1. `SplitJoinRoot::split()` calls `root_augmentation()`
2. Store `RequiredPartitions` in context
3. `SplitJoinRoot::join()` passes to `merge_sub_partitions()`

## Implementation Order

1. ✅ Add `RequiredPartitions` struct in new file `required.rs`
2. ✅ Add `overlap()` method to `PartitionRange`
3. ✅ Modify `root_augmentation()` to compute and return required partitions
4. ✅ Thread `RequiredPartitions` through join context
5. ✅ Filter in `merge_sub_partitions()`
6. ✅ Run tests to validate

## Test Validation

For `insert_infix1`:
- Required set: `{1..=2 (ab), 1..=3 (aby), 1..=4 (abyz)}`
- Skipped: `{2..=3 (by), 2..=4 (byz)}`
- Result: `aby` has 1 pattern `[ab, y]`
