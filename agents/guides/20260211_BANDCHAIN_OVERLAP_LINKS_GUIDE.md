# BandChain and Overlap Links Guide

**Date:** 2026-02-11  
**Confidence:** 🟢 High  
**Tags:** #context-read #bandchain #expansion #decomposition #paths

## Overview

This guide explains how BandChain generates decompositions through expansion, and how the OverlapLink abstraction tracks overlaps between tokens to enable retrieval of all valid decompositions.

## Key Concepts

### BandChain

**Location:** `crates/context-read/src/expansion/chain/mod.rs`

A BandChain is an ordered collection (BTreeSet) of Band structures that tracks sequential expansions and overlap decompositions during context reading.

**Structure:**
```rust
pub(crate) struct BandChain {
    pub(crate) bands: BTreeSet<Band>,      // Ordered by end_bound
    pub(crate) links: Vec<OverlapLink>,    // Overlap links for decompositions
}
```

**Key Properties:**
- **First band**: Contains the sequential expansion result (main pattern)
- **Overlap bands**: Bands after the first contain alternate decompositions `[complement, expansion]`
- **Links**: Each OverlapLink corresponds to an expansion that created an overlap band

### Band

**Location:** `crates/context-read/src/expansion/chain/band.rs`

```rust
pub(crate) struct Band {
    pub(crate) pattern: Pattern,           // Token sequence
    pub(crate) start_bound: AtomPosition,  // Starting position
    pub(crate) end_bound: AtomPosition,    // Ending position (used for ordering)
}
```

Bands are ordered in the BTreeSet by their `end_bound`, allowing efficient lookup of bands that end at specific positions.

### OverlapLink

**Location:** `crates/context-read/src/expansion/chain/link.rs`

Represents the overlap between two tokens in a decomposition, capturing the dual perspective of the overlap region.

```rust
pub(crate) struct OverlapLink {
    /// Top-down child path from starting root to the expandable postfix.
    /// This represents the overlap region token from the first token's perspective.
    pub(crate) child_path: IndexEndPath,
    
    /// Bottom-up then top-down search path from the expansion.
    /// This represents the overlap region token from the second token's perspective.
    pub(crate) search_path: IndexStartPath,
    
    /// Position where the overlap starts in the input sequence.
    pub(crate) start_bound: usize,
}
```

**Purpose:**
- Captures both views of the overlap region (from first token and from second token)
- Enables reconstruction of complement tokens
- Provides information needed to generate all valid decompositions

## Path Types (from context-trace)

### RolePath

**Location:** `crates/context-trace/src/path/structs/rooted/role_path.rs`

Paths have directional semantics through the `PathRole` trait:
- `RolePath<Start>` - Path with Start role (bottom-up, then top-down from expansion)
- `RolePath<End>` - Path with End role (top-down from root postfix)

### Common Path Type Aliases

```rust
// From context-trace
type IndexStartPath = IndexRolePath<Start>;
type IndexEndPath = IndexRolePath<End>;
type PatternRangePath = RootedRangePath<Pattern, ChildLocation, ChildLocation>;
```

**PatternRangePath:**
- Composite of Start and End RolePaths
- Used as cursor to track position within pattern during expansion
- Has methods: `start_path()`, `end_path()`, `path_root()`

## How Decompositions Are Generated

### Step-by-Step Flow

1. **Initialization** - `ExpansionCtx::new()`
   - Creates BandChain with initial token from cursor's root pattern
   - First band contains starting token

2. **Expansion Loop** - `ExpansionCtx::next()`
   - Creates `ExpandCtx` to iterate over postfixes of the last band's token
   - For each postfix, attempts insertion via `ToInsertCtx::insert()`
   - Returns either `ChainOp::Expansion` or `ChainOp::Cap`

3. **Apply Expansion** - `ExpansionCtx::apply_op(ChainOp::Expansion)`
   - Updates cursor to expansion path
   - Creates `ExpansionLink` with overlap information
   - Creates `OverlapLink` from ExpansionLink
   - Builds complement token using `ComplementBuilder`
   - Appends new band `[complement, expansion]` via `append_front_complement()`
   - Stores the OverlapLink via `append_overlap_link()`

4. **Apply Cap** - `ExpansionCtx::apply_op(ChainOp::Cap)`
   - Pops first band
   - Appends cap token to the band (extending sequential result)
   - Re-inserts band into chain

5. **Commit to Root** - `RootManager::commit_chain()`
   - Final token from first band becomes main pattern
   - Overlap bands added as child patterns (decompositions)

### Example: "aaa" Input

```
Position:    0       1       2
Input:       a       a       a

Segmentation: {unknown: [a@0], known: [a@1, a@2]}

Step 1: Create initial band
  Band 1: "a"@0 (start_bound=0, end_bound=1)

Step 2: Process known [a@1, a@2]
  - Consume a@1, combine with "a"@0 → root = "aa"@0-1
  - Band 1: "aa"@0-1 (start_bound=0, end_bound=2)

Step 3: Expand postfixes into remaining [a@2]
  - Postfix "a"@1 (of "aa"@0-1) expands into [a@2]
  - Creates expansion "aa"@1-2
  - OVERLAP at position 1!
  
  Creates OverlapLink:
    - child_path: IndexEndPath to "a"@1 (postfix of "aa"@0-1)
    - search_path: IndexStartPath to "a"@1 (prefix of "aa"@1-2)
    - start_bound: 1
  
  Builds complement: "a"@0 (everything before position 1)
  
  Adds Band 2: ["a"@0, "aa"@1-2] (start_bound=0, end_bound=3)

Final BandChain:
  - Band 1: "aa"@0-1, "a"@2  → [aa, a] decomposition
  - Band 2: "a"@0, "aa"@1-2  → [a, aa] decomposition (from overlap)
  - Link 1: Overlap link for Band 2
```

## Using Overlap Links

### Retrieving Decompositions

The overlap links provide the information needed to:

1. **Identify overlap regions**: Both `child_path` and `search_path` point to the same overlap token from different perspectives

2. **Build complement tokens**: Use the paths to reconstruct the prefix (complement) that wasn't part of the overlap

3. **Generate all decompositions**: Combine overlap information with band data

### Path Interpretation

**child_path (IndexEndPath):**
- Top-down navigation from the starting root
- Ends at the postfix that was expandable
- Represents the overlap token as seen from the first token's structure

**search_path (IndexStartPath):**
- Bottom-up from the expansion, then top-down
- Starts at the prefix of the expansion
- Represents the overlap token as seen from the second token's structure

### Complement Building

The `ComplementBuilder` uses the paths from `ExpansionLink`:
- Gets root token from `root_postfix` path
- Calculates intersection start from `root_postfix.root_child_index()`
- Creates `InitInterval` for the complement range [0, intersection_start)
- Uses `graph.insert_init()` to create the complement token

## Related Types and Functions

### ExpansionLink

**Location:** `crates/context-read/src/expansion/link.rs`

```rust
pub(crate) struct ExpansionLink {
    pub(crate) expansion_prefix: IndexStartPath,
    pub(crate) root_postfix: IndexEndPath,
    pub(crate) start_bound: usize,
}
```

This is the intermediate representation used by `ComplementBuilder`. It's converted to `OverlapLink` for storage in the BandChain.

### insert_or_complete

**Location:** `context-insert/src/insert/mod.rs`

```rust
fn insert_or_get_complete(
    &self,
    searchable: impl Searchable<InsertTraversal>,
) -> Result<Result<IndexWithPath, Error>, ErrorReason>
```

Used to search for a pattern and either:
- Return existing match if found
- Insert new pattern if not found
- Handle edge cases like single-index patterns

## Common Patterns

### Creating an Expansion

```rust
// In ExpansionCtx::apply_op()
let expansion_link = self.create_expansion_link(&exp);
let overlap_link = self.create_overlap_link(&expansion_link);
let complement = ComplementBuilder::new(expansion_link).build(&self.cursor.graph);

self.chain.append_front_complement(complement, exp.expansion.index);
self.chain.append_overlap_link(overlap_link);
```

### Iterating Overlap Bands

```rust
// Get all decompositions (excluding the main sequential result)
for band in chain.overlap_bands() {
    // Each band.pattern is a [complement, expansion] decomposition
    let decomposition = &band.pattern;
}
```

### Accessing Stored Links

```rust
// Links correspond to overlap bands in order
for (link, band) in chain.links.iter().zip(chain.overlap_bands()) {
    // link contains the path information for this overlap
    // band contains the resulting [complement, expansion] pattern
}
```

## Troubleshooting

### Missing Decompositions

**Symptom:** Expected decomposition not appearing in child patterns

**Check:**
1. Was an expansion created? Look for `ChainOp::Expansion` in logs
2. Was an overlap link stored? Check `append_overlap_link` debug output
3. Did the postfix iteration find the relevant postfix?

**Common Causes:**
- Postfix not iterable (token has no parent patterns)
- Insertion failed (became Cap instead of Expansion)
- Overlap band was merged incorrectly

### Path Confusion

**Symptom:** Unclear which path represents what

**Remember:**
- **IndexEndPath** (child_path): "Where we came from" - root postfix perspective
- **IndexStartPath** (search_path): "Where we're going" - expansion prefix perspective
- Both point to the SAME overlap token, just from different structural perspectives

### Complement Building Errors

**Symptom:** Complement token incorrect or missing

**Check:**
1. `root_postfix.root_child_index()` - should be > 0
2. Root token from `root_postfix.graph_root_child(graph)` - should be valid
3. InitInterval end_bound - should equal intersection_start

## Future Enhancements

Potential improvements to the overlap link system:

1. **Map-based storage**: Use `BTreeMap<AtomPosition, OverlapLink>` keyed by band end bounds for O(log n) lookup
2. **On-demand complement building**: Cache complement tokens or build them lazily
3. **Link validation**: Add invariant checks to ensure links match their corresponding bands
4. **Bidirectional navigation**: Support traversing from links back to their source bands

## Related Files

- `crates/context-read/src/expansion/chain/mod.rs` - BandChain implementation
- `crates/context-read/src/expansion/chain/link.rs` - OverlapLink, ChainOp types
- `crates/context-read/src/expansion/chain/band.rs` - Band structure
- `crates/context-read/src/expansion/mod.rs` - ExpansionCtx and expansion logic
- `crates/context-read/src/expansion/chain/expand.rs` - ExpandCtx postfix iteration
- `crates/context-read/src/complement.rs` - ComplementBuilder
- `crates/context-read/src/context/root.rs` - commit_chain function
- `crates/context-trace/src/path/structs/rooted/` - Path type definitions
- `agents/analysis/ALTERNATE_DECOMPOSITION_ANALYSIS.md` - Algorithm analysis and examples
