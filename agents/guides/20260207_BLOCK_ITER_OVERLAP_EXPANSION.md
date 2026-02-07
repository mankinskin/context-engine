---
confidence: 🟢
tags: `#context-read` `#expansion` `#algorithm` `#pattern-matching`
summary: How SegmentIter partitions sequences and BlockExpansionCtx expands blocks using context-search
---

# Segment Iteration and Block Expansion

## Overview

The context-read crate processes input sequences through two key mechanisms:
1. **Segment Iteration** - Partitioning input into unknown/known pairs
2. **Block Expansion** - Using context-search to find largest prefixes and detect overlaps

## Segment Iteration

### SegmentIter

`SegmentIter` partitions a sequence of `NewAtomIndex` values into alternating segments of unknown (new) and known (existing) atoms.

```rust
pub struct SegmentIter {
    iter: Peekable<IntoIter<NewAtomIndex>>,
}

pub struct NextSegment {
    pub unknown: Pattern,  // New atoms not yet in the graph
    pub known: Pattern,    // Atoms already in the graph
}
```

### How It Works

When reading "abcabcabc" where "abc" was never seen before:
1. First read: all atoms are **new** → `unknown=[a,b,c], known=[]`
2. After inserting unknown, atoms now exist
3. Remaining atoms are known → `unknown=[], known=[a,b,c,a,b,c]`

The iterator groups consecutive new atoms together, then consecutive known atoms:

```
Input atoms: [New(a), New(b), New(c), Known(a), Known(b), Known(c), Known(a), Known(b), Known(c)]
                      ↓
NextSegment { unknown: [a,b,c], known: [a,b,c,a,b,c] }
```

## Block Expansion

### BlockExpansionCtx

`BlockExpansionCtx` manages block construction and committing. It contains the `RootManager` and processes known patterns by:
1. Finding the largest prefix using context-search
2. Committing blocks when complete
3. Detecting overlaps between blocks

```rust
pub struct BlockExpansionCtx {
    root: RootManager,      // Manages root token construction
    known: Pattern,         // Remaining known pattern to process
    current_block: Token,   // Current block being built
}
```

### The Block Expansion Algorithm

For input "abcabcabc" with `unknown=[a,b,c]` and `known=[a,b,c,a,b,c]`:

**Phase 1: Initial Block Creation**
1. Create BlockExpansionCtx with unknown pattern `[a,b,c]`
2. This creates the initial block token "abc" (the "unknown block")

**Phase 2: Process Known Pattern**
When `process(known)` is called:

1. **Find largest prefix in known pattern** (using context-search):
   - Search known pattern `[a,b,c,a,b,c]` for largest matching prefix
   - context-search finds "abc" at positions 0-3 (no larger prefix exists)
   - **Commit block**: Combine unknown block + found prefix = `[abc, abc]` → "abcabc"
   - Remaining known pattern: `[a,b,c]` (positions 3-6)

2. **Expand postfix of committed block for overlaps** (using ExpandCtx):
   - Look at postfix of "abcabc" (which is "abc")
   - Check for overlaps with remaining pattern `[a,b,c]`
   - Find overlap: postfix "abc" expands into remaining `[a,b,c]` → `[abc, abc]` = "abcabc"
   - This matches our existing "abcabc" token!
   - **Commit final block**: Create "abcabcabc" with two decompositions:
     - `[abc, abcabc]`
     - `[abcabc, abc]`

### BandChain (Ordered Overlap Map)

`BandChain` is a `BTreeSet<Band>` ordered by `end_bound` (atom position). It tracks committed blocks and their position bounds.

```rust
pub struct Band {
    pub pattern: Pattern,      // The tokens forming this band
    pub start_bound: AtomPosition,
    pub end_bound: AtomPosition,  // Key for ordering
}
```

### ExpandCtx (Postfix Iteration)

`ExpandCtx` iterates through postfixes of a token to find overlapping patterns:

```rust
pub struct ExpandCtx<'a> {
    pub ctx: &'a ExpansionCtx<'a>,
    pub postfix_path: IndexEndPath,
    pub postfix_iter: PostfixIterator<'a, ReadCtx>,
}
```

For each postfix position, it either finds:
- **Expansion**: A matching continuation → new block committed
- **Cap**: No match → extend existing block pattern

## Integration with ReadCtx

```rust
fn read_segment(&mut self, segment: NextSegment) {
    let NextSegment { unknown, known } = segment;
    
    // Create BlockExpansionCtx with unknown pattern
    // This initializes the first block from unknown
    let mut block_ctx = BlockExpansionCtx::new(
        self.root.take(),  // Move RootManager into block ctx
        unknown,
    );
    
    // Process known pattern, advancing and committing blocks
    if !known.is_empty() {
        block_ctx.process(known);
    }
    
    // Return RootManager with final result
    self.root = block_ctx.finish();
}
```

## Complete Data Flow Example

```
Input: "abcabcabc"

Step 1: SegmentIter produces
  NextSegment { unknown: [a,b,c], known: [a,b,c,a,b,c] }

Step 2: BlockExpansionCtx::new(root, unknown=[a,b,c])
  - Creates initial "unknown block" = "abc"
  - Block chain: [Band{abc, 0..3}]

Step 3: block_ctx.process(known=[a,b,c,a,b,c])
  
  Phase A - Find largest prefix in known:
    - context-search on [a,b,c,a,b,c]
    - Finds "abc" at 0..3 as largest prefix
    - Commit: [unknown_abc, prefix_abc] = "abcabc"
    - Block chain: [Band{abcabc, 0..6}]
    - Remaining known: [a,b,c] (positions 3..6 of original)
  
  Phase B - Expand postfix for overlaps:
    - Postfix of "abcabc" is "abc" 
    - Remaining pattern: [a,b,c]
    - Postfix "abc" + remaining [a,b,c] = [abc, abc] = "abcabc" (overlap found!)
    - Commit: "abcabcabc" with decompositions:
      - [abc, abcabc]  (first abc + abcabc)
      - [abcabc, abc]  (abcabc + last abc)
    - Block chain: [Band{abcabcabc, 0..9}]

Step 4: block_ctx.finish()
  - Returns RootManager with root = "abcabcabc"
```

## Key Files

| File | Purpose |
|------|---------|
| `sequence/segment_iter.rs` | SegmentIter partitions unknown/known |
| `expansion/block.rs` | BlockExpansionCtx - block construction and committing |
| `expansion/mod.rs` | ExpansionCtx for overlap detection |
| `expansion/chain/mod.rs` | BandChain ordered overlap map |
| `expansion/chain/band.rs` | Band structure |
| `expansion/chain/expand.rs` | ExpandCtx postfix iteration |
| `context/mod.rs` | ReadCtx orchestrates the flow |
| `context/root.rs` | RootManager - root token construction |

## Common Mistakes

- The **unknown pattern** creates the first block ("unknown block")
- When committing with prefix, we combine **unknown block + largest prefix found in known**
- BlockExpansionCtx **owns** the RootManager during processing
- context-search finds largest **prefix** in known pattern, not continuation of unknown
- Overlaps are detected by expanding **postfix** of committed blocks
- BandChain is ordered by **end_bound**, not start_bound

## Related

- `context-search/HIGH_LEVEL_GUIDE.md` - How patterns are searched
- `context-insert/HIGH_LEVEL_GUIDE.md` - How tokens are inserted  
- `context-trace/HIGH_LEVEL_GUIDE.md` - Hypergraph structure and paths
- `CHEAT_SHEET.md` - Quick API reference
