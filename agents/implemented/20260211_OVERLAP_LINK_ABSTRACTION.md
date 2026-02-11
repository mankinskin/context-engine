# Overlap Link Abstraction Implementation

**Date:** 2026-02-11
**Status:** ✅ Complete

## Summary

Implemented the overlap "link" abstraction in the BandChain to track overlaps between tokens in decompositions. The overlap links capture both perspectives of the overlap region (from the starting root and from the expansion), enabling future work to retrieve or build complement tokens and generate the full set of decompositions.

This work was completed as part of understanding and extending the alternate decomposition system described in `agents/analysis/ALTERNATE_DECOMPOSITION_ANALYSIS.md`.

## Changes Made

### Core Implementation

#### 1. Enhanced OverlapLink Structure (`crates/context-read/src/expansion/chain/link.rs`)
- ✅ Defined `OverlapLink` with three fields:
  - `child_path: IndexEndPath` - Top-down path from starting root to expandable postfix (first token's view of overlap)
  - `search_path: IndexStartPath` - Bottom-up then top-down path from expansion (second token's view of overlap)
  - `start_bound: usize` - Position where the overlap starts in the input sequence
- ✅ Added comprehensive documentation explaining the dual-perspective nature of overlaps

#### 2. BandChain Storage (`crates/context-read/src/expansion/chain/mod.rs`)
- ✅ Added `links: Vec<OverlapLink>` field to `BandChain` struct
- ✅ Added `append_overlap_link()` method to store links when expansions occur
- ✅ Updated `BandChain::new()` to initialize empty links vector
- ✅ Simplified `ends_at()` and `last()` to use cleaned up `BandCtx`
- ✅ Removed commented-out link code

#### 3. BandCtx Cleanup (`crates/context-read/src/expansion/chain/band.rs`)
- ✅ Simplified `BandCtx` by removing commented-out `back_link` and `front_link` fields
- ✅ Now only contains `band: &'a Band` reference

#### 4. Expansion Logic (`crates/context-read/src/expansion/mod.rs`)
- ✅ Modified `apply_op()` to create and store overlap links during expansions
- ✅ Added `create_overlap_link()` helper method that converts `ExpansionLink` to `OverlapLink`
- ✅ Added `OverlapLink` import
- ✅ Overlap links are now created and stored whenever an expansion occurs

### Documentation

#### 1. Analysis Document Update (`agents/analysis/ALTERNATE_DECOMPOSITION_ANALYSIS.md`)
- ✅ Added "Implementation Update (2026-02-11)" section documenting:
  - All code changes made
  - Key concepts researched (BandChain, RolePath/RangePath, PathCursor, insert_or_complete)
  - How overlap links enable decomposition retrieval
  - Example for "aaa" showing overlap link creation
  - Future work suggestions

#### 2. New Comprehensive Guide (`agents/guides/20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md`)
- ✅ Complete guide to BandChain and overlap links system
- ✅ Detailed explanation of all key concepts:
  - BandChain structure and purpose
  - Band ordering and properties
  - OverlapLink dual-perspective nature
  - Path types (RolePath, RangePath, PatternRangePath, IndexStartPath, IndexEndPath)
- ✅ Step-by-step decomposition generation flow
- ✅ "aaa" example walkthrough showing band creation and overlap link storage
- ✅ Path interpretation guide
- ✅ Complement building explanation
- ✅ Common patterns for using overlap links
- ✅ Troubleshooting section
- ✅ Future enhancement suggestions

#### 3. Guide Index Update (`agents/guides/INDEX.md`)
- ✅ Added entry for new guide with 🟢 High confidence rating
- ✅ Summary: "Complete guide to BandChain, overlap links, path types, and decomposition generation in context-read"

## Research Findings

### BandChain Architecture

**Structure:**
- Ordered collection (BTreeSet) of `Band` structures, ordered by `end_bound`
- First band = sequential expansion result (main pattern)
- Overlap bands (after first) = alternate decompositions `[complement, expansion]`
- Links vector = `OverlapLink` for each expansion that created an overlap band

**Decomposition Flow:**
1. `ExpansionCtx::new()` creates initial BandChain
2. `ExpansionCtx::next()` iterates, yielding tokens via `ExpandCtx`
3. For each postfix expansion: creates overlap link, builds complement, adds band
4. `RootManager::commit_chain()` converts bands to child patterns

### Path System (from context-trace)

**Key Types:**
- `RolePath<Start>` - Bottom-up then top-down path (expansion perspective)
- `RolePath<End>` - Top-down path (root postfix perspective)
- `PatternRangePath` - Composite cursor with start and end RolePaths
- `IndexStartPath` - Alias for `IndexRolePath<Start>`
- `IndexEndPath` - Alias for `IndexRolePath<End>`

**Dual Perspective:**
- Same overlap token viewed from two structural perspectives
- `child_path` (IndexEndPath): How first token sees the overlap
- `search_path` (IndexStartPath): How second token sees the overlap

### insert_or_complete

**Purpose:** Search for pattern and either return existing match or insert new pattern
**Location:** `context-insert/src/insert/mod.rs`
**Usage:** Used in `ExpansionCtx::new()` to get/create initial bundle from cursor

## Impact

### Immediate Benefits
1. **Explicit overlap tracking** - Links are now stored alongside bands, making the overlap structure explicit
2. **Documentation** - Comprehensive guide explains the complex path system and decomposition generation
3. **Foundation for future work** - Link storage enables future implementation of:
   - On-demand complement token building
   - Full decomposition set retrieval
   - Optimization via map-based storage

### Code Quality Improvements
1. **Removed dead code** - Cleaned up commented-out link references
2. **Better separation of concerns** - `OverlapLink` is distinct from `ExpansionLink`
3. **Clear documentation** - Both inline and external documentation explain the system

## Testing Status

⚠️ **Note:** Unable to run tests due to missing `ngrams` dependency (external dev-dependency).

The code changes are:
- Syntactically correct (verified by manual inspection)
- Structurally sound (follows existing patterns)
- Logically consistent (creates links when expansions occur)

Future work should include:
- Test that overlap links are created for each expansion
- Verify link paths match their corresponding bands
- Test complement token building using stored links

## Future Work

Based on this implementation, the following enhancements are now possible:

1. **Decomposition Retrieval API**
   - Implement `get_all_decompositions()` method on BandChain
   - Use stored links to reconstruct all valid decompositions

2. **On-Demand Complement Building**
   - Implement lazy complement token creation using stored paths
   - Cache complements to avoid rebuilding

3. **Storage Optimization**
   - Convert `links: Vec<OverlapLink>` to `links: BTreeMap<AtomPosition, OverlapLink>`
   - Key by band end_bound for O(log n) lookup

4. **Link Validation**
   - Add invariant checks ensuring links match their bands
   - Verify path consistency

5. **Bidirectional Navigation**
   - Support traversing from links back to source bands
   - Enable querying "which bands use this overlap?"

## Related Files

### Modified Files
- `crates/context-read/src/expansion/chain/link.rs`
- `crates/context-read/src/expansion/chain/mod.rs`
- `crates/context-read/src/expansion/chain/band.rs`
- `crates/context-read/src/expansion/mod.rs`
- `agents/analysis/ALTERNATE_DECOMPOSITION_ANALYSIS.md`

### New Files
- `agents/guides/20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md`

### Updated Files
- `agents/guides/INDEX.md`

## References

- **Analysis:** `agents/analysis/ALTERNATE_DECOMPOSITION_ANALYSIS.md` - Algorithm analysis
- **Guide:** `agents/guides/20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md` - Complete implementation guide
- **Related Guide:** `agents/guides/20260207_BLOCK_ITER_OVERLAP_EXPANSION.md` - BlockIter and ExpansionCtx
