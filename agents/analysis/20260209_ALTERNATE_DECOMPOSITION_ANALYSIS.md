---
tags: `#analysis` `#context-insert` `#context-read` `#algorithm` `#debugging` `#testing`
summary: When reading input "aaa", context-read produces only one child pattern `[a, aa]` for vertex "aaa", but the correct output (per ngrams reference) sh...
---

# Alternate Decomposition Bug Analysis

## Issue Summary

When reading input "aaa", context-read produces only one child pattern `[a, aa]` for vertex "aaa", but the correct output (per ngrams reference) should have two patterns: `[a, aa]` AND `[aa, a]`.

## Current Behavior

For input "aaa":
```
Input positions: 0   1   2
                 a   a   a

Segmentation: {unknown: [a@0], known: [a@1, a@2]}

Current output for "aaa":
  - Child patterns: [[a, aa]]  ← Only one decomposition
```

## Expected Behavior

```
Expected output for "aaa":
  - Child patterns: [[a, aa], [aa, a]]  ← Both decompositions
```

## Root Cause

The expansion algorithm currently only tracks **forward** overlaps during block processing. It doesn't detect that the bundled result can have **alternate decompositions** when postfixes of the result overlap with the prefix region.

## The Fix: Postfix Expansion into Remaining Pattern

### Algorithm Flow for "aaa"

```
Step 1: Segment input
  {unknown: [a@0], known: [a@1, a@2]}

Step 2: Process unknown [a@0]
  → root = "a"@0

Step 3: Process known block [a@1, a@2]
  
  3a. Start with first atom a@1
      - No expansions for "a" alone (no parents to traverse)
      - Finish current sub-block, advance past a@1
  
  3b. Root becomes "aa"@0-1 (combining a@0 + a@1)
  
  3c. EXPANSION STEP: Expand postfixes of root "aa"@0-1 into remaining known [a@2]
      
      Postfix of "aa"@0-1 is "a"@1
      
      Can "a"@1 expand into remaining [a@2]?
      → YES! They combine to form "aa"@1-2
      → This is an OVERLAP: "aa"@0-1 and "aa"@1-2 share position 1
  
  3d. The overlap extends range to [0,3) = "aaa"
      
      From this overlap, we get BOTH decompositions:
      
      Split at position 2 (overlap end): 
        - [aa@0-1, a@2] = [aa, a]  ← First decomposition found
      
      Split at position 1 (overlap start):
        - [a@0, aa@1-2] = [a, aa]  ← Second decomposition (complement)

Step 4: Final result for "aaa"
  Child patterns: [[aa, a], [a, aa]]
```

### Key Insight: Overlaps Generate Both Decompositions

When a postfix of the current root expands and creates an overlap, that single overlap produces TWO decompositions:

```
Position:    0       1       2
             a       a       a
             
Root:        aa@0-1
                     ├───────┤
Expansion:           aa@1-2
             
Overlap at position 1 creates:
  [aa@0-1, a@2]  ← Split at expansion end (position 2)
  [a@0, aa@1-2]  ← Complement: what's before the postfix start
```

The **complement** comes from the postfix path: when we expand postfix "a"@1 into "aa"@1-2, the complement is everything BEFORE position 1 in the original root, which is "a"@0. Combined with the expansion result, we get [a@0, aa@1-2].

## Implementation Strategy

### Where to Implement

The fix should be integrated into the expansion logic, specifically during step 3c above - when we expand postfixes of the current root into the remaining known pattern.

The key location is in `context-read/src/context/root.rs` in the `commit_chain` method, or in the expansion chain logic.

### Pseudocode

```rust
fn expand_postfixes_into_remaining(
    &mut self, 
    current_root: &Token,
    remaining_known: &[TokenPosition],  // What's left of the known pattern
) -> Vec<ChildPattern> {
    let mut decompositions = Vec::new();
    
    // For each postfix of the current root token
    for (postfix_start, postfix) in current_root.postfixes() {
        // Can this postfix expand into the remaining known pattern?
        if let Some(expansion) = self.try_expand(postfix, remaining_known) {
            // We found an overlap!
            // This creates the [complement, expansion] decomposition
            
            // The complement is everything BEFORE postfix_start in the root
            let complement = current_root.prefix_before(postfix_start);
            
            decompositions.push(
                ChildPattern::new(vec![complement, expansion.token])
            );
            
            // The [root, remaining_after] decomposition comes from the 
            // normal forward bundling process
        }
    }
    
    decompositions
}
```

### Integration Point

In `commit_chain`, after bundling atoms into the current root, check for expansions:

```rust
// Existing: bundle atoms into root
let root_token = /* ... existing bundling logic ... */;

// NEW: Before finishing the block, try expanding postfixes into remaining known
let postfix_decompositions = self.expand_postfixes_into_remaining(
    &root_token,
    &remaining_known_pattern,
);

overlap_patterns.extend(postfix_decompositions);
```

## Key Insight

The key mechanism is **postfix expansion into remaining known pattern**. When we have built a root token and there's still known pattern remaining, we check if any postfix of the root can expand (via overlap/parent traversal) into that remaining pattern. If so, the **complement** of that postfix in the root becomes part of an alternate decomposition.

For "aaa" (width 3):
- Root after processing a@0 + a@1 = "aa"@0-1
- Remaining known: [a@2]
- Postfix "a"@1 expands into remaining → finds "aa"@1-2
- Complement of postfix@1 is "a"@0
- Decompositions: [aa@0-1, a@2] and [a@0, aa@1-2]

## Related Files

- `context-read/src/context/root.rs` - `commit_chain` function
- `context-read/src/expansion/mod.rs` - `ExpansionCtx` and postfix iteration
- `context-read/src/expansion/chain/expand.rs` - `ExpandCtx` postfix handling
- `context-read/src/complement.rs` - `ComplementBuilder` for prefix extraction
- `context-read/src/expansion/chain/link.rs` - `OverlapLink` structure
- `context-read/src/expansion/chain/mod.rs` - `BandChain` with overlap links storage

## Test Case

The failing test is in `context-read/src/tests/ngrams_validation.rs`:
- `validate_three_repeated` - input "aaa"

After fix, run:
```bash
cargo test -p context-read -- validate_three_repeated
```

## Priority

**High** - This is a correctness bug. The graph structure should contain all valid decompositions for proper pattern matching and search operations.

---

## Implementation Update (2026-02-11)

### Overlap Link Abstraction - IMPLEMENTED

The overlap "link" abstraction has been added to the BandChain to track overlaps between tokens in decompositions.

#### Changes Made:

1. **Enhanced OverlapLink Structure** (`expansion/chain/link.rs`):
   - `child_path: IndexEndPath` - Top-down child path from starting root to expandable postfix (overlap region from first token's view)
   - `search_path: IndexStartPath` - Bottom-up then top-down search path from expansion (overlap region from second token's view)
   - `start_bound: usize` - Position where the overlap starts in the input sequence
   - Added comprehensive documentation explaining the dual-perspective nature of overlaps

2. **BandChain Storage** (`expansion/chain/mod.rs`):
   - Added `links: Vec<OverlapLink>` field to store overlap links
   - Added `append_overlap_link()` method to add links when expansions occur
   - Updated initialization to create empty links vector
   - Cleaned up commented-out code

3. **Expansion Logic** (`expansion/mod.rs`):
   - Modified `apply_op()` to create and store overlap links during expansions
   - Added `create_overlap_link()` method that converts `ExpansionLink` to `OverlapLink`
   - Overlap links are now created alongside complement tokens during band chain extension

#### Key Concepts Researched:

**BandChain**:
- An ordered collection (BTreeSet) of Band structures tracking sequential expansions and overlap decompositions
- First band contains the sequential expansion result
- Overlap bands (after first) contain alternate decompositions `[complement, expansion]`
- Now includes a vector of OverlapLink objects corresponding to each expansion

**RolePath / RangePath**:
- `RolePath<Start>` - Path with Start role (bottom-up then top-down from expansion)
- `RolePath<End>` - Path with End role (top-down from root postfix)
- `PatternRangePath` - Composite of Start/End RolePaths used as cursor
- `IndexStartPath` - Alias for `IndexRolePath<Start>`
- `IndexEndPath` - Alias for `IndexRolePath<End>`

**PathCursor**:
- `CursorCtx` wraps a mutable `PatternRangePath` reference
- `PatternRangePath` is the actual cursor type tracking position within pattern during expansion
- Used to navigate through the pattern during expansion and insertion operations

**insert_or_complete**:
- Defined in `context-insert` crate, implemented on `HypergraphRef`
- Searches for query pattern and either returns existing match or inserts new pattern
- Used in `ExpansionCtx::new()` to get/create initial bundle from cursor
- Returns `Result<Result<IndexWithPath, Error>, ErrorReason>` with multiple success/failure cases

#### How Overlap Links Enable Decomposition Retrieval:

The `OverlapLink` structure provides the necessary information to:
1. **Identify overlap regions**: The `child_path` and `search_path` both point to the same overlap token from different perspectives
2. **Build complements**: The paths can be used to reconstruct the prefix (complement) token that wasn't part of the overlap
3. **Generate alternate decompositions**: By combining the overlap information with band data, we can derive both `[prefix, expansion]` and `[root, suffix]` decompositions

Example for "aaa":
- Root: "aa"@0-1, Remaining: [a@2]
- Postfix "a"@1 expands → "aa"@1-2 (overlap at position 1)
- OverlapLink created:
  - `child_path`: IndexEndPath to "a"@1 (postfix of "aa"@0-1)
  - `search_path`: IndexStartPath to "a"@1 (prefix of "aa"@1-2)
  - `start_bound`: 1
- This link helps derive:
  - `[aa@0-1, a@2]` - from forward expansion
  - `[a@0, aa@1-2]` - from complement using the link

#### Future Work:

The overlap links are now being stored and can be used to:
- Implement methods to retrieve all decompositions from the band chain
- Build complement tokens on-demand using the stored path information
- Potentially optimize storage by using a map keyed by band end bounds (as suggested by the original TODO comment)
