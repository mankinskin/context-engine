# Alternate Decomposition Fix - Implementation Summary

**Date:** 2026-02-09  
**Status:** Implemented, test passing  
**Test:** `validate_three_repeated`

## Problem Statement

When reading input "aaa", context-read produced only one child pattern `[a, aa]` for vertex "aaa", but the correct output (per ngrams reference) should have two patterns: `[a, aa]` AND `[aa, a]`.

```
Expected for "aaa":
  - Child patterns: [[a, aa], [aa, a]]  ← Both decompositions

Actual (before fix):
  - Child patterns: [[a, aa]]  ← Missing [aa, a]
```

## Root Cause Analysis

The expansion algorithm only tracked **forward** overlaps during block processing. It didn't detect that the bundled result can have **alternate decompositions** when postfixes of the result overlap with the root/prefix region.

For "aaa" with `unknown=[a@0]`, `known=[a@1, a@2]`:
1. Unknown "a" becomes root
2. Known `[a, a]` gets bundled into "aa" 
3. Root "a" + known "aa" = "aaa"
4. **Missing step:** Check if postfixes of "aa" can form overlaps with root "a"
   - Postfix of "aa" is "a" → matches root!
   - So `[aa, a]` is also a valid decomposition

## Key Insight: Postfix Expansion Creates Overlap Bands

When postfixes of the bundled result match the root, an alternate decomposition exists:

```
Position:    0       1       2
             a       a       a
             
Root:        a@0
Known:               aa@1-2
             
Result:      aaa@0-2

Postfix of "aa" is "a" which matches root "a"
  → String check: "aa"+"a" == "a"+"aa" == "aaa" ✓
  → Add [aa, a] as overlap band
```

## Implementation

### 1. Overlap Detection in `block.rs`

After processing the known block, check for overlaps with the root:

```rust
// In BlockExpansionCtx::process(), after `while ctx.next().is_some() {}`

if let Some(root_token) = self.root.root {
    let final_token = ctx.chain.final_token();

    // Check if any postfix of final_token matches root
    for (_, postfix) in final_token.postfix_iter(self.root.graph.clone()) {
        if postfix.vertex_index() == root_token.vertex_index() {
            // Verify the overlap is valid: swapped order must produce same string
            let root_str = self.root.graph.index_string(root_token.vertex_index());
            let final_str = self.root.graph.index_string(final_token.vertex_index());

            let forward = format!("{}{}", root_str, final_str);
            let swapped = format!("{}{}", final_str, root_str);

            if forward == swapped {
                // Add [final_token, root_token] as overlap band
                ctx.chain.append_front_complement(final_token, root_token);
            }
            break;
        }
    }
}
```

### 2. Fixed `final_token()` in `chain/mod.rs`

Bands are ordered by `end_bound`. The first band is the main sequential bundle:

```rust
/// Get the final bundled token from the first band (main sequential bundle).
pub(crate) fn final_token(&self) -> Token {
    self.first().unwrap().last_token()  // Changed from last() to first()
}
```

### 3. Clean `commit_chain` in `root.rs`

Removed the ad-hoc postfix checking from `commit_chain` since overlaps are now properly detected during BandChain creation. The commit logic just processes pre-built bands.

## Files Changed

| File | Change |
|------|--------|
| `context-read/src/expansion/block.rs` | Added postfix overlap detection after processing known block |
| `context-read/src/expansion/chain/mod.rs` | Fixed `final_token()` to return from first band |
| `context-read/src/context/root.rs` | Cleaned up (removed redundant postfix check) |

## Test Results

### Before Fix
- ngrams validation: 7 passed, 3 failed
- `validate_three_repeated`: **FAILED**

### After Fix
- ngrams validation: 8 passed, 2 failed
- `validate_three_repeated`: **PASSED** ✓

### Remaining Failures (Pre-existing)
- `validate_palindrome` - Different issue (not related to this fix)
- `validate_triple_repeat` - Different issue (missing "ab" vertex)

## Architectural Understanding

### BandChain Structure

```
BandChain {
    bands: BTreeSet<Band>  // Ordered by end_bound
}

Band {
    pattern: Pattern,      // e.g., [aa] or [aa, a]
    start_bound: AtomPosition,
    end_bound: AtomPosition,  // Key for ordering
}
```

- **First band:** Main sequential bundle (smallest end_bound)
- **Subsequent bands:** Overlap decompositions (larger end_bounds, start at 0)

### Processing Flow

```
1. Segment input into unknown/known blocks
2. Append unknown to root (fresh root created)
3. BlockExpansionCtx processes known:
   a. ExpansionCtx bundles known pattern
   b. NEW: Check postfixes of bundle against root for overlaps
   c. Add any overlap bands to chain
4. commit_chain combines root + bundle, adds overlap patterns
```

## Future Work

1. **Generalize overlap detection:** Current implementation only checks immediate root. May need to handle nested overlaps.

2. **Fix remaining test failures:**
   - `validate_palindrome` ("abba")
   - `validate_triple_repeat` ("ababab")

3. **Performance:** String comparison for overlap validation is O(n). Consider using token width comparison first as optimization.

4. **Recursive overlaps:** The analysis mentions that overlaps can be nested. Current implementation finds one level of overlap. May need BFS/DFS traversal for complex patterns.

## Key Learnings

1. **Overlaps belong in the chain, not in commit:** The BandChain should contain all decompositions before committing. `commit_chain` should only construct the final graph structure.

2. **Postfix iteration is the mechanism:** The graph already has parent relationships. Using `postfix_iter()` traverses these to find overlaps naturally.

3. **String equality for validation:** When postfix matches, verify string equality to ensure the alternate decomposition is semantically valid (e.g., "aa"+"a" == "a"+"aa" for "aaa", but "ba"+"a" ≠ "a"+"ba" for "aba").
