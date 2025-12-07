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

## Root Cause Analysis

The problem appears to be in the root partition processing logic. When processing `ababcd` as the root:

1. **Postfix partition** is created for entries `[1..]` (i.e., `[ab, c, d]` → `[ab, cd]`)
2. The `perfect.is_none()` check on line 217 should trigger wrapper creation
3. **Prefix partition** should be created for entries `[0..1]` (i.e., `[ab]`)
4. **Wrapper pattern** `[prefix, postfix]` should be added

**Hypothesis**: The wrapper is either:
- Not being created at all (logic not triggered)
- Being created but not at the right level (creates `[ab, bcd]` instead of `[ab, cd]` which would be `abcd`)
- Being created with wrong pattern composition

The key issue mentioned in problem statement:
> "The insert call should only create new vertices for the 'inside' partitions of vertices, when splitting them at one or more offset positions."

> "insert should not modify partitions outside of the split and not create vertices for patterns, which are not to be inserted."

This suggests the algorithm correctly avoids creating patterns for ranges outside the insertion scope (e.g., not creating patterns starting from position 0 in `ababcd`). However, it should still create the wrapper `abcd` for the **intersection** of the insertion range with the existing patterns.

## Missing Logic

The algorithm needs to create wrapper vertices at appropriate hierarchical levels. Specifically:
- When `ababcd[1..]` is split into `[ab, cd]`
- The range `[0, 1, 2, 3]` (atoms a, b, c, d) should have a wrapper vertex `abcd`
- Even though position 0 is "outside" the primary insertion range in `ababcd`

The wrapper should be: `abcd = [ab, cd]` OR `abcd = [a, bcd]` depending on the joining strategy.

## Investigation Needed

1. ✅ Verify which vertices are processed in the frontier iteration
2. ✅ Confirm split positions in cache
3. ✅ Trace root partition processing for `ababcd`
4. ❓ Determine why wrapper creation logic is not triggered
5. ❓ Identify correct layer for creating `abcd` wrapper
6. ❓ Understand the relationship between "inside partitions" and wrapper creation

## Potential Fix Strategies

### Option 1: Enhance root partition logic
Modify `join_root_partitions` to also create wrappers for child pattern ranges, not just for the root itself.

### Option 2: Add post-processing step
After all partitions are joined, analyze the resulting patterns and create missing wrappers.

### Option 3: Fix split cache construction
Ensure the split cache includes information about all patterns that need to be created, including intermediate wrappers.

## Next Steps

1. Add detailed logging to `join_root_partitions` to trace execution
2. Verify what `pre` and `part.index` values are when processing `ababcd`
3. Determine if wrapper creation is skipped or creates wrong pattern
4. Identify the correct fix location and minimal change needed
