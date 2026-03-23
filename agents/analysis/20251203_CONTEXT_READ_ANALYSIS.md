---
tags: `#analysis` `#context-trace` `#context-search` `#context-read` `#algorithm` `#performance`
summary: ```rust
---

# Context-Read Analysis

**High-level graph reading and pattern expansion system**

**Status:** Partially implemented - core expansion infrastructure ready, reading ops in development

---

## Capabilities

| Function | Purpose | Algorithm |
|----------|---------|-----------|
| Read sequences | Build hierarchical patterns from atoms | Block iteration → classify (new/known) → expand known |
| Expand patterns | Find largest enclosing context | Expansion cursor → band chain → complement |
| Multiple representations | Handle tokens with multiple child patterns | Expand all → select optimal for context |
| Calculate complements | Find "missing piece" in expansions | Root range - expansion range = complement |

---

## Core Types

```rust
ReadCtx<R>                    // Reading context with cursor
BlockIter                     // Iterate through new/known blocks
ExpansionCtx                  // Pattern expansion state
BandChain                     // Linked expansion bands
Complement                    // Complement calculation result

// Key operations
read_ctx.block_iter()         // Get block iterator
expansion_ctx.expand()        // Expand pattern
band_chain.next_band()        // Traverse expansion
```

---

## Algorithm Flow

**Reading:**
1. Classify atoms (new vs known)
2. Block iteration (group consecutive)
3. Insert new atoms
4. Expand known patterns
5. Build optimal decomposition

**Expansion:**
1. Start with partial pattern
2. Create expansion cursor
3. Build band chain (prefix/postfix)
4. Calculate complements
5. Return complete context

**Complexity:** O(n·d) where n=sequence length, d=pattern depth

---

## Key Components

| Component | Role |
|-----------|------|
| RootManager | Manage root token context |
| CursorCtx | Track expansion position |
| ExpansionLink | Link between expansion bands |
| BandExpansion | Individual expansion step |
| BandCap | Expansion termination |

---

## Dependencies

**Internal:** context-trace (graph, paths), context-search (search ops)  
**External:** indexmap, tracing, thiserror

---

## Design Rationale

**Block iteration:** Groups consecutive known/unknown → fewer operations  
**Expansion chains:** Incremental discovery → memory efficient  
**Complement ops:** Pattern arithmetic → flexible composition

---

## Future Enhancements

1. **Pattern discovery** - Automatic frequent pattern detection
2. **Context-aware reading** - Adapt to usage patterns
3. **Streaming** - Process infinite sequences
4. **Compression** - Optimize storage via pattern sharing
5. **Async reading** - Non-blocking operations

---

## TODO

- Complete band operations
- Async reading support
- Advanced complement operations
- Pattern optimization heuristics
- Multiple read strategies

---

## Performance

**Time:** O(n·d) for n atoms, d depth  
**Space:** O(b·c) for b blocks, c chains  
**Bottlenecks:** Deep hierarchies, many representations  
**Optimizations:** Cache expansions, lazy evaluation, parallel bands
