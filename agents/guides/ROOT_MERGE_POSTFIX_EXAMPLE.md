# Root Merge Algorithm - Postfix Mode Example

## Overview

This guide documents the expected behavior of the root merge algorithm for the `insert_postfix1` test case.

## Test Case: insert_postfix1

**Initial state:** `ababcd = [ab, ab, c, d]`  
**Insert:** `bcd` at position 3 (postfix mode)  
**Target token:** `bcd` (width 3)

## Step 1: Split Phase (Creates 3 Offsets)

```
┌─────────┬─────────┬───┬───┐
│   ab    │   ab    │ c │ d │  ← Root node child pattern
└─────────┴─────────┴───┴───┘
     ↑         ↑      ↑   ↑
  start    offset0  o1  o2  end
```

**Position mapping (postfix mode):**
- `start` (pos 0): beginning of root pattern
- `offset 0` (pos 2): after first "ab", before second "ab" [**PROTECTED as prefix**]
- `offset 1` (pos 3): after "a", before "b" [**OPERATIONAL**]
- `offset 2` (pos 4): after "b", before "c" [**OPERATIONAL**]
- `end`: after "d"

**Key insight:** Postfix mode creates 3 offsets to split the pattern appropriately for inserting `bcd` at position 3.

## Step 2: Create Initial Partitions

**Partition range:** `1..4` (skips prefix at index 0)

### Partition Structure (3 offsets → 4 partitions)

```
┌────────────┬─────────┬─────────┬─────────┐
│  [0:prefix]│[1:infix]│[2:infix]│[3:postfix]│
│    (ab)    │   (a)   │   (b)   │   (cd)  │
│  SKIPPED   │ CREATED │ CREATED │ CREATED │
└────────────┴─────────┴─────────┴─────────┘
     │           │         │          │
  before      between   between    extends
 offset0    offset0-1  offset1-2   beyond o2
```

**Partition → Offset mapping (with prefix):**
- Partition 0 (prefix): before offset 0 [**SKIPPED in postfix mode**]
- Partition 1 (infix): between offset 0 and offset 1 → "a"
- Partition 2 (infix): between offset 1 and offset 2 → "b"
- Partition 3 (postfix): from offset 2 to end → "cd"

**Initial partitions array after creation:**
- `partitions[0]` = partition 1: token for "a"
- `partitions[1]` = partition 2: token for "b"  
- `partitions[2]` = partition 3: token for "cd"

**Expected initial partitions:** `[a, b, cd]` ✅

## Step 3: Hierarchical Merge

### Merge Algorithm

The merge uses a hierarchical approach with a `RangeMap` tracking merged tokens:

1. **Initialize RangeMap** with single-partition ranges:
   - `range_map[0..1]` = partitions[0] = "a"
   - `range_map[1..2]` = partitions[1] = "b"
   - `range_map[2..3]` = partitions[2] = "cd"

2. **Merge length 2** (merge consecutive pairs):
   - **Merge partitions [0..2]:** tokens ["a", "b"] → "ab"
     - `info_partition` returns `Err(existing "ab")` (already exists in graph)
     - Simplified Err logic: return existing token
     - Track in `range_map[0..2]` = "ab"
   
   - **Merge partitions [1..3]:** tokens ["b", "cd"] → "bcd" (**TARGET**)
     - `info_partition` tries to join ["b", "cd"]
     - Should create NEW token "bcd" with patterns from:
       - Node child patterns after merging (from `info.patterns`)
       - Previous smaller joins from `range_sub_merges`
     - Expected patterns for "bcd": `[[b, cd]]`
     - Track in `range_map[1..3]` = "bcd"

3. **Merge length 3** (merge all three partitions):
   - **Merge partitions [0..3]:** tokens ["a", "b", "cd"] OR ["ab", "bcd"] → "abcd"
     - `info_partition` tries to join the range
     - Should create NEW token "abcd" with patterns from:
       - Node child patterns: `[ab, cd]` (after "cd" replacement)
       - Range sub-merges: `[a, bcd]` (from previous merges)
     - Expected patterns for "abcd": `[[a, bcd], [ab, cd]]` (two patterns!)
     - Track in `range_map[0..3]` = "abcd"

## Step 4: Perfect Border Detection & Pattern Replacement

After hierarchical merging, the algorithm should:

1. **Replace "cd" in root pattern:**
   - Original: `[ab, ab, c, d]`
   - After: `[ab, ab, cd]`
   - Condition: Perfect border detected for "cd" partition

2. **Replace "abcd" in root pattern:**
   - Original: `[ab, ab, cd]`
   - After: `[ab, abcd]`
   - Condition: Perfect border detected for "abcd" partition

## Expected Final State

### Token Patterns (VertexData)

- **cd** (width 2): `[[c, d]]`
- **bcd** (width 3): `[[b, cd]]` ← **TARGET token**
- **abcd** (width 4): `[[a, bcd], [ab, cd]]` ← **Two patterns!**
- **ababcd** (width 6): `[[ab, abcd]]`

### Search Results

When searching for "abcd" with pattern `[a, b, c, d]`:
- Should find as **EntireRoot** (standalone token)
- Token should have VertexData with correct child patterns
- Search can traverse via either pattern: `[a, bcd]` or `[ab, cd]`

## Key Implementation Points

1. **Partition indices vs Offset indices:**
   - With prefix: partition i (i > 0) → offset (i-1) and offset i
   - Without prefix: partition i → offset i and offset (i+1)

2. **Err(existing_token) handling:**
   - Return existing token WITHOUT modification
   - Track in range_map for use in larger hierarchical merges
   - Don't try to add patterns to already-complete tokens

3. **Hierarchical token creation:**
   - Use BOTH `info_partition` (node patterns) AND `range_map` (previous merges)
   - Build patterns from both sources for new tokens
   - Create tokens even when components already exist

4. **Perfect border replacement:**
   - Detect when merged token perfectly aligns with node pattern boundaries
   - Replace matched subsequence in root node with merged token
   - Update root VertexData child patterns progressively

## Common Issues

1. **Missing offset:** Only 2 offsets instead of 3 → wrong partition structure
2. **Wrong partition mapping:** Incorrect offset index calculation
3. **Skipped hierarchical merging:** Only creating target, not intermediate tokens
4. **Missing patterns:** Not combining node patterns with range_sub_merges
5. **No pattern replacement:** Perfect borders detected but root not updated

## References

- Test file: `crates/context-insert/src/join/context/node/tests.rs::insert_postfix1`
- Merge implementation: `crates/context-insert/src/join/context/node/merge/shared.rs`
- Root merge entry: `crates/context-insert/src/join/context/node/merge/root.rs`
