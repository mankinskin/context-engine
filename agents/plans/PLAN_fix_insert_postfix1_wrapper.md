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

### Example: Inserting "mnoxyp" into a Pattern

Original pattern: `[h, i, j, k, lmn, x, y, opq, r, s, t]`

When inserting "mnoxyp":
- Overlaps entries at indices 4-7: `[lmn, x, y, opq]`
- After joining: `[x, y]` → `xy` (creating a "delta" since pattern size changed from 2 to 1)

Create wrapper for only the overlapping range:
```rust
wrapper = [
    [lmn, xy, opq],      // Pattern 1: full entry tokens with joined middle
    [l, mnoxyp, q]        // Pattern 2: complement tokens with inserted pattern
]
```

Result: `[h, i, j, k, wrapper, r, s, t]`
- Surrounding tokens `[h, i, j, k]` and `[r, s, t]` remain **unchanged**
- Only the overlapping range is wrapped

### Multi-Level Wrapper Creation

Enhance `join_root_partitions` to:

## Implementation Plan

### 1. Add Helper Method to Find Overlapping Ranges

Create `find_overlapping_child_ranges` method in `NodeJoinCtx`:

```rust
fn find_overlapping_child_ranges(
    &self,
    split_pos: AtomPosition,
) -> Vec<(SubIndex, SubIndex)> {
    // Find ranges of child pattern entries that overlap the split
    // Return (start_entry, end_entry) pairs for potential wrappers
}
```

Logic:
- Iterate through parent's child pattern entries
- Track cumulative atom positions
- Identify ranges that:
  - Start before split_pos
  - End after split_pos
  - Form meaningful groupings (more than one child)

### 2. Modify `join_root_partitions` for Postfix Mode

After creating the main wrapper `[prefix, postfix]`, add:

```rust
RootMode::Postfix => Postfix::new(offset)
    .join_partition(self)
    .inspect(|part| {
        if part.perfect.is_none() {
            let pre = match Prefix::new(offset).join_partition(self) {
                Ok(pre) => pre.index,
                Err(c) => c,
            };
            
            // Create main wrapper
            self.ctx.trav.add_pattern_with_update(
                index,
                Pattern::from(vec![pre, part.index]),
            );
            
            // NEW: Create additional wrappers for overlapping ranges
            self.create_overlapping_wrappers(offset, index);
        }
    })
    .map(|part| part.index),
```

### 3. Implement `create_overlapping_wrappers`

```rust
fn create_overlapping_wrappers(
    &mut self,
    offset: PosSplitCtx,
    parent_index: Token,
) {
    let split_atom_pos = self.calculate_split_atom_position(offset);
    let ranges = self.find_overlapping_child_ranges(split_atom_pos);
    
    for (start_entry, end_entry) in ranges {
        // Get or create tokens for this range
        let wrapper = self.create_wrapper_for_range(
            parent_index,
            start_entry,
            end_entry,
        );
        
        tracing::debug!(
            wrapper = ?wrapper,
            range = ?start_entry..=end_entry,
            "Created overlapping wrapper"
        );
    }
}
```

### 4. Calculate Split Atom Position from Offset

```rust
fn calculate_split_atom_position(
    &self,
    offset: PosSplitCtx,
) -> AtomPosition {
    // Extract atom position from split offset
    // This is the position where the main split occurs
    offset.pos.get() // or similar, depending on PosKey structure
}
```

### 5. Create Wrapper for Range

```rust
fn create_wrapper_for_range(
    &mut self,
    parent_index: Token,
    start_entry: SubIndex,
    end_entry: SubIndex,
) -> Token {
    // Get child tokens in range
    let children: Vec<Token> = (start_entry..=end_entry)
        .map(|i| self.get_child_at_entry(parent_index, i))
        .collect();
    
    // Check if this pattern already exists
    match self.ctx.trav.find_pattern(&children) {
        Some(existing) => existing,
        None => {
            // Create new pattern
            self.ctx.trav.insert_pattern(children)
        }
    }
}
```

## Specific Fix for `ababcd` Case

For `ababcd` = `[ab@0, ab@1, c@2, d@3]` with split after atom position 3:

**Main Split**:
- Atom split at position 3
- Prefix: `aba` (entries mix)
- Postfix: `bcd` (entries 1-3 partial)

**Overlapping Ranges** to check:
- Entries `[0, 1, 2, 3]`: Entire pattern (already exists as `ababcd`)
- Entries `[0, 1, 2]`: `[ab, ab, c]` - might be useful
- Entries `[1, 2, 3]`: `[ab, c, d]` → creates `abcd` ✅
- Entries `[2, 3]`: `[c, d]` → already created as `cd`

The key is entries `[1, 2, 3]` which span `[ab, c, d]` atoms, creating the needed `abcd` wrapper.

## Logic for Finding Overlapping Ranges

```rust
fn find_overlapping_child_ranges(
    &self,
    split_atom_pos: AtomPosition,
) -> Vec<(SubIndex, SubIndex)> {
    let patterns = self.patterns();
    let mut ranges = Vec::new();
    
    // For simplicity, start with range that crosses split
    let mut cumulative_pos = 0;
    let mut range_start = None;
    
    for (entry_idx, (_pid, pattern)) in patterns.iter().enumerate() {
        let entry_start = cumulative_pos;
        cumulative_pos += pattern.width();
        let entry_end = cumulative_pos;
        
        // Check if entry crosses or touches split
        if entry_start < split_atom_pos && entry_end >= split_atom_pos {
            if range_start.is_none() {
                range_start = Some(entry_idx);
            }
        } else if range_start.is_some() {
            // Range ended
            ranges.push((range_start.unwrap(), entry_idx - 1));
            range_start = None;
        }
    }
    
    // Handle range that extends to end
    if let Some(start) = range_start {
        ranges.push((start, patterns.len() - 1));
    }
    
    ranges
}
```

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
