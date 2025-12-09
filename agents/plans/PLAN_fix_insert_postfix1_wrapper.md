# Fix Plan: Create Missing `abcd` Wrapper Vertex

**Date:** 2025-12-07  
**Bug Report:** `20251207_INSERT_POSTFIX1_MISSING_ABCD.md`  
**Status:** READY FOR IMPLEMENTATION

## Problem Summary

Split at atom position 3 in `ababcd` creates:
- Prefix: `aba` (atoms 0-2)
- Postfix: `bcd` (atoms 3-5)
- Wrapper: `[aba, bcd]` ❌

Missing: `abcd` = `[ab, cd]` (atoms 0-3)

## Solution Approach

### Key Concept: Minimal Wrapping Vertex

**The core idea is to store multiple overlapping tokens in a minimal wrapping vertex, instead of duplicating the surrounding context.**

When inserting a pattern, we:
1. Identify the entry index positions from the role paths (start and/or end)
2. Create a wrapper around only those entries
3. Replace that specific range with the wrapper vertex in the original pattern_id pattern

### Example: Inserting "mnxyop" into a Pattern

Original pattern: `[h, i, j, k, lmn, x, y, opq, r, s, t]`

When inserting "mnxyop":
- Overlaps entries at indices 4-7: `[lmn, x, y, opq]`
- After joining: `[x, y]` → `xy` (creating a "delta" since pattern size changed from 2 to 1)

Create wrapper for only the overlapping range:
```rust
wrapper = [
    [lmn, xy, opq],      // Pattern 1: full entry tokens with joined middle
    [l, mnxyop, q]        // Pattern 2: complement tokens with inserted pattern
]
```

Result: `[h, i, j, k, wrapper, r, s, t]`
- Surrounding tokens `[h, i, j, k]` and `[r, s, t]` remain **unchanged**
- Only the overlapping range is wrapped

### Multi-Level Wrapper Creation

Enhance `join_root_partitions` to:

## Implementation Plan

### Key Insight: No Need to Search for Overlaps

**Important:** We don't need to search for overlapping entries because:
- Every pattern in the original root vertex is already overlapping
- Common index borders among vertex child patterns are not allowed
- All index borders in all patterns are at different atom positions
- The split run has already joined all smaller vertices
- Each child has a clean split into two partitions at positions where they're intersected in the root

### 1. Use the "Wrapper Range" from Split Partitions

Instead of searching, we extract the wrapper range directly from the split partitions:

```rust
fn extract_wrapper_range(
    &self,
    split_partitions: &SplitPartitions,
) -> (SubIndex, SubIndex) {
    // The wrapper range is the subrange in each pattern containing the splits
    // Each entry/exit child already has splits extracted by the split run
    // We just need to identify which pattern entries contain these splits
}
```

### 2. Handle Perfect Splits Correctly

**Perfect Split:** A split where there already exists an index border in a pattern.

#### Case 1: Single Perfect Split Border
- Simply replace the resulting partitions in the existing pattern with the perfect split border
- No wrapper needed - the pattern already has the correct structure

#### Case 2: Two Perfect Split Borders (in different patterns)
- Create **two wrappers**, one for each pattern with a perfect split
- **Pattern with perfect split on start side:** 
  - Wrapper starts at perfect split
  - Ends after the last vertex intersected by the end split
- **Pattern with perfect split on end side:**
  - Wrapper starts at the index intersected by the start split
  - Ends at the perfect split in the pattern
- The `join_root_partitions` result token is the overlap vertex of these two wrappers
- This overlap is not directly contained in the root

#### Case 3: Infix with No Perfect Splits
- Similar to Case 2, need to create two wrappers
- The "closest" index borders outside the split range may be in different patterns
- Create one wrapper for each pattern containing the split range boundaries

### 3. Implement Wrapper Creation Based on Split Type

```rust
fn create_wrappers_for_splits(
    &mut self,
    prefix_part: &JoinedPartition<Pre>,
    postfix_part: &JoinedPartition<Post>,
    parent_index: Token,
) -> Token {
    // Check for perfect splits
    let has_perfect_start = prefix_part.perfect.is_some();
    let has_perfect_end = postfix_part.perfect.is_some();
    
    match (has_perfect_start, has_perfect_end) {
        // Case 1: Single perfect split (or both in same pattern)
        (true, false) | (false, true) => {
            // Replace partitions directly in existing pattern
            self.replace_with_perfect_split(prefix_part, postfix_part, parent_index)
        },
        
        // Case 2: Two perfect splits in different patterns
        (true, true) if different_patterns(prefix_part, postfix_part) => {
            let wrapper1 = self.create_wrapper_from_perfect_start(prefix_part, postfix_part);
            let wrapper2 = self.create_wrapper_from_perfect_end(prefix_part, postfix_part);
            // Return the overlap vertex of these two wrappers
            self.find_or_create_overlap(wrapper1, wrapper2)
        },
        
        // Case 3: No perfect splits (infix case)
        (false, false) => {
            self.create_wrappers_for_infix(prefix_part, postfix_part, parent_index)
        },
        
        // Both perfect in same pattern
        _ => {
            self.replace_range_in_pattern(prefix_part, postfix_part, parent_index)
        }
    }
}
```

### 4. Extract Split Partitions from Entry/Exit Children

```rust
fn extract_split_partitions_from_children(
    &self,
    prefix_part: &JoinedPartition<Pre>,
    postfix_part: &JoinedPartition<Post>,
) -> Vec<Pattern> {
    // Each child has a clean split into two partitions
    // Use the wrapper range (subrange in each pattern containing the splits)
    // to extract the splits from all entry/exit children
    
    let mut patterns = Vec::new();
    
    // Get the range of pattern entries affected by the splits
    let wrapper_range = self.get_wrapper_range(prefix_part, postfix_part);
    
    for entry_idx in wrapper_range {
        let child_token = self.get_child_at_entry(entry_idx);
        let split_partitions = self.get_split_partitions_for_child(child_token);
        
        // Build patterns using these partitions
        patterns.push(split_partitions.to_pattern());
    }
    
    patterns
}
```

### 5. Get Wrapper Range from Delta Information

```rust
fn get_wrapper_range(
    &self,
    prefix_part: &JoinedPartition<Pre>,
    postfix_part: &JoinedPartition<Post>,
) -> Range<SubIndex> {
    // Use delta field to determine which pattern entries are involved
    // delta indicates the pattern entry index where partition starts
    
    let start_entry = prefix_part.delta.iter()
        .next()
        .map(|(_, &idx)| idx)
        .unwrap_or(0);
    
    let end_entry = postfix_part.delta.iter()
        .next()
        .map(|(_, &idx)| idx + 1) // +1 because range is exclusive on end
        .unwrap_or(self.patterns().len());
    
    start_entry..end_entry
}
```

## Specific Fix for `ababcd` Case

For `ababcd` = `[ab@0, ab@1, c@2, d@3]` with split after atom position 3:

**Split Analysis**:
- Split occurs within entry 1 (`ab` at position 1)
- Prefix partition: includes entries 0 and partial entry 1
- Postfix partition: includes partial entry 1, entries 2, 3

**Wrapper Range from Delta**:
- Postfix `delta` indicates start at entry 1
- Wrapper range: entries `[1, 2, 3]` = `[ab, c, d]`
- After joining inner partitions: `[c, d]` → `cd`
- Final wrapper range: `[ab, cd]`

**Split Partitions**:
- Entry 1 (`ab`) splits into: `[a]` (prefix) and `[b]` (postfix)
- Entries 2, 3 (`c`, `d`) are in postfix, joined to `cd`

**Wrapper Patterns**:
1. `[ab, cd]` - using full entry token and joined postfix tokens
2. `[a, bcd]` - using prefix partition of entry 1 and complete postfix partition

This creates the needed `abcd` vertex with both representations.

## Algorithm Summary

### For Each Root Mode:

**Postfix Mode:**
1. Join prefix and postfix partitions
2. Check `delta` to find wrapper range start (entry index where postfix begins)
3. Wrapper range: from `delta` entry to end of pattern
4. Extract split partitions from children in wrapper range
5. Build wrapper patterns from these partitions
6. Replace wrapper range in root pattern with new wrapper vertex

**Prefix Mode:**
1. Join prefix and postfix partitions
2. Check `delta` to find wrapper range end (entry index where prefix ends)
3. Wrapper range: from start of pattern to `delta` entry
4. Extract split partitions from children in wrapper range
5. Build wrapper patterns from these partitions
6. Replace wrapper range in root pattern with new wrapper vertex

**Infix Mode:**
1. Join prefix, infix, and postfix partitions
2. Check `delta` for both start and end of infix
3. Wrapper range: from start `delta` entry to end `delta` entry
4. Handle perfect splits:
   - If both boundaries are perfect in different patterns: create two wrappers
   - If no perfect splits: create wrappers for each pattern containing boundaries
5. Extract split partitions and build wrapper patterns
6. Replace wrapper range with new wrapper vertex (or overlap of two wrappers)

## Testing Strategy

1. Run `insert_postfix1` test
2. Verify `abcd` vertex is created
3. Check that wrapper contains expected pattern `[ab, cd]`
4. Verify no extra unnecessary wrappers created
5. Run full context-insert test suite

## Rollback Plan

If fix causes regressions:
1. Revert `join_root_partitions` changes
2. Keep enhanced logging for debugging
3. Re-evaluate approach

## Success Criteria

- ✅ `insert_postfix1` test passes
- ✅ `assert_indices!(graph, cd, abcd)` succeeds
- ✅ `abcd` found as EntireRoot, not Postfix
- ✅ No other tests broken
- ✅ Minimal code changes (localized to one method)
