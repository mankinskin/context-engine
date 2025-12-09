# Bug Report: Missing `abcd` Vertex in insert_postfix1 Test

**Date:** 2025-12-07  
**Test:** `crates/context-insert/src/tests/cases/insert/postfix.rs::insert_postfix1`  
**Status:** FAILING

## Problem Summary

The `insert_postfix1` test fails because the `abcd` vertex is not created during pattern insertion. The test expects:
- Initial graph: `ababcd` (pattern `[ab, ab, c, d]`) with existing patterns `ab` and `ababcd`
- Query: Search for `[a, b, c, d]` returns Postfix match at `ababcd[1..]` 
- After insert: should create vertices `cd`, `bcd`, and **`abcd`**
- **Issue**: `abcd` vertex is missing

### Test Failure

```
assertion: assert_indices!(graph, cd, abcd)
error: abcd: Complete response has non-EntireRoot path: Postfix(...)
  Found abcd as Postfix within "ababcd" at root_entry: 1
  Expected abcd as EntireRoot (complete vertex)
```

## Expected Behavior

According to the problem statement:
> "The insert call should use the trace cache through ababcd to construct the split cache, with split positions for each traced vertex (split positions are relative to the vertex itself, i.e. the vertex itself is positioned at 0)."

> "The abcd vertex should be created, to efficiently store the overlapping tokens ab and bcd in minimal overlapping patterns."

The insertion should create a **wrapper vertex** `abcd` containing the pattern `[ab, bcd]` or `[a, bcd]` to represent the overlapping sequence.

## Current Behavior

### Trace Cache Structure

From test expectations (lines 34-51):
```rust
InitInterval {
    root: ababcd,
    cache: {
        ababcd => (BU { 1 => ab -> (ababcd_id, 1) }, TD {}),
        ab => (BU { 1 => b -> (ab_id, 1) }, TD {}),
        b => (BU {}, TD {}),
    },
    end_bound: 3.into(),
}
```

Split positions:
- `ababcd` vertex: split at position 1 (after first `ab`)
- `ab` vertex: split at position 1 (after `a`)
- `b` vertex: leaf (no split)

### Join Operations Observed

From test log (lines 7-85), three `JoinedPatterns` events:

1. **Pattern `[c, d]` → creates `cd`**
   - Range: `2..` in ababcd
   - Perfect: `Some(ababcd_id)` 
   - Result: calls `replace_in_pattern` 
   - Status: ✓ Works correctly

2. **Pattern `[b, cd]` → should create `bcd`**
   - Perfect: `None`
   - Delta in ababcd: 1
   - Status: ✓ Appears to work

3. **Pattern `[ab, ?]` → should create wrapper**
   - Perfect: `None`
   - Delta in ababcd: 0
   - Status: ❌ Does NOT create `abcd` wrapper

### Root Processing Logic

In `join_root_partitions` (context.rs:191-243), for `RootMode::Postfix`:

```rust
RootMode::Postfix => Postfix::new(offset)
    .join_partition(self)
    .inspect(|part| {
        if part.perfect.is_none() {
            let pre = match Prefix::new(offset).join_partition(self) {
                Ok(pre) => pre.index,
                Err(c) => c,
            };
            self.ctx.trav.add_pattern_with_update(
                index,
                Pattern::from(vec![pre, part.index]),
            );
        }
    })
    .map(|part| part.index)
```

This logic should create `[prefix, postfix]` wrapper when `perfect.is_none()`, but apparently it's not being triggered for the case that should create `abcd`.

## Root Cause Analysis - CONFIRMED

### Split Position Calculation Issue

The bug is in how split positions are calculated - they use **atom-level offsets** instead of **pattern-entry-level offsets**.

From enhanced logging and code analysis:

**Split Calculation** (in `bottom_up_splits`):
```rust
let inner_offset = Offset::new(*token.width() - **inner_width);
let outer_offset = *node.expect_child_offset(location);
let node_offset = inner_offset + outer_offset;
```

For `ababcd` pattern `[ab, ab, c, d]` with bottom-up trace at position 1 in the second `ab`:
- `location` = (ababcd_id, 1) - second `ab` at pattern entry index 1
- `token` = `ab` with width 2
- `inner_width` = 1 (trace position within `ab`)
- `inner_offset` = 2 - 1 = 1 (offset within the `ab` token)
- `outer_offset` = 2 (second `ab` starts at **atom position 2** in `ababcd`)
- `node_offset` = 1 + 2 = **3** (atom position 3)

**Result**: Split at **atom position 3** in `ababcd` = `[a,b,a,b,c,d]`:
- Prefix: `aba` (atoms 0-2) = `[ab, a]`
- Postfix: `bcd` (atoms 3-5) = `[b, c, d]`
- Wrapper: `[aba, bcd]` (width 6) ❌

**Expected**: Split at **pattern entry 1**:
- Prefix: `ab` (entry 0)
- Postfix: `abcd` (entries 1-3) = `[ab, cd]` 
- Should create `abcd` vertex

### Wrapper Creation Confirmed But Wrong

From test logs:
```
join_root_partitions - Postfix mode: creating wrapper pattern
root=T5w6 (ababcd)
wrapper_pattern=[T8w3, T7w3]
```

Where:
- T7w3 = `bcd` (postfix partition, width 3)
- T8w3 = `aba` (prefix partition, width 3) 
- Wrapper created: `[aba, bcd]` = 6-atom sequence ❌

The wrapper IS being created, but with wrong partitions!

### Why abcd is Missing

The insertion range is `[b, c, d]` (atoms 1-4 in original `ababcd`):
- Creates `cd` ✓
- Creates `bcd` ✓
- Should create `abcd` = `[ab, cd]` or `[a, bcd]` ❌

`abcd` should span atoms `[a, b, c, d]` (positions 0-3), but:
- Prefix ends at atom 2 (`aba`)
- Postfix starts at atom 3 (`bcd`)
- No partition spans atoms 0-3

The wrapper `[aba, bcd]` spans atoms 0-5, which is the full `ababcd` range, not the sub-range `abcd`.

### Core Issue

Split calculation in `bottom_up_splits` uses atom-level arithmetic:
- Traces provide atom positions within child tokens
- Outer offsets are atom positions of children in parent
- Sum gives atom position in parent

This is **semantically correct for atom-level indexing** but creates **wrong semantic groupings** for wrapper patterns. The split should recognize pattern-entry boundaries, not just atom positions.

## Investigation Needed

1. ✅ Verify which vertices are processed in the frontier iteration
2. ✅ Confirm split positions in cache
3. ✅ Trace root partition processing for `ababcd`
4. ❓ Determine why wrapper creation logic is not triggered
5. ❓ Identify correct layer for creating `abcd` wrapper
6. ❓ Understand the relationship between "inside partitions" and wrapper creation

## Potential Fix Strategies

### Option 1: Multi-Level Wrapper Creation ⭐ RECOMMENDED
The `join_root_partitions` method should create wrappers at multiple levels, not just for the split point:

1. **Current**: Creates wrapper `[prefix@split, postfix@split]`
2. **Needed**: Also create wrappers for overlapping child ranges

For `ababcd` with split at position 3 (atom level):
- Split gives: prefix=`aba`, postfix=`bcd`
- Additionally create: `abcd` = range [0..4] = `[ab, cd]` (pattern-entry view)

**Implementation**: After creating split partitions in `join_root_partitions`:
- Analyze child patterns that overlap insertion range
- Create additional wrapper vertices for meaningful subranges
- Use pattern-entry boundaries, not just atom split points

### Option 2: Pattern-Entry-Level Split Calculation
Modify `bottom_up_splits` to calculate splits at pattern-entry boundaries:
- Track pattern entry indices in trace cache (not just atom positions)
- Calculate splits relative to pattern structure
- Create prefix/postfix at entry boundaries

**Challenge**: Would require significant changes to trace cache structure.

### Option 3: Post-Process After Join
Add a post-processing step after all joins complete:
- Analyze created vertices
- Identify missing wrappers for overlapping ranges
- Create missing vertices

**Challenge**: May be inefficient and harder to maintain invariants.

## Recommended Fix

**Option 1** is most promising because:
1. Localized change in `join_root_partitions`
2. Preserves existing atom-level split semantics
3. Adds wrapper creation logic only where needed
4. Handles the specific case mentioned in problem statement

The fix should:
1. After creating `prefix` and `postfix` partitions
2. Check if there are child patterns that span across the split
3. For each overlapping range, create appropriate wrapper vertices
4. Use pattern-entry information from the parent's child patterns
