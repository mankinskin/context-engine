---
tags: `#analysis` `#context-trace` `#context-search` `#context-insert` `#algorithm` `#debugging` `#testing` `#performance`
summary: 1. Cache split results for repeated operations
---

# Context-Insert Algorithm Analysis

**Deep analysis of insertion algorithm phases, dependencies, and performance**

---

## Algorithm Summary

**Purpose:** Safe pattern insertion via split-join (add new patterns without modifying existing)

**Phases:** Search → Initialize (InitInterval) → Split → Join → Extract  
**Complexity:** O(d·p + k + m) where d=depth, p=parents/level, k=split points, m=merges

**Key Types:**
- `InitInterval{root, cache, end_bound}` - Conversion from incomplete search
- `IntervalGraph` - Processing structure for split-join
- `SplitCache{root_mode, entries}` - Tracks split locations
- `JoinContext` - Manages merge operations

---

## Phase Details

| Phase | Input | Output | Key Operations |
|-------|-------|--------|----------------|
| Initialize | Response (incomplete search) | InitInterval | Extract root, cache, checkpoint_position |
| IntervalGraph | InitInterval + graph | IntervalGraph | Create TraceCtx, SplitTraceStatesCtx |
| Split | IntervalGraph | SplitCache | Identify boundaries, track positions, cache splits |
| Join | SplitCache | New patterns | Merge components, create tokens, link parents |
| Extract | Join result | Token | Return new token or interval state |

**RootMode variants:** Prefix (extend before), Postfix (extend after), Infix (extend middle)

**Range Roles:** Pre/Post/In - Determines how split boundaries handled at borders

---

## Dependencies

### External Crates
- `petgraph` - Graph algorithms
- `indexmap` - Order-preserving maps
- `tracing` - Logging/debugging
- `thiserror` - Error handling

### Internal Crates
| Depends On | Used For |
|------------|----------|
| context-trace | Hypergraph, paths, traversal, cache |
| context-search | Response type, search result handling |

**Flow:** search (context-search) → init → insert (context-insert) → result

---

## Testing

**Test Structure:** Integration tests in `tests/` (prefix, postfix, infix, interval_graph, split_cache)

**Key Helpers:**
- `init_test_tracing!(&graph)` - Setup tracing
- `insert_atoms!`, `insert_patterns!` - Graph setup
- `from_search_result(response)` - Create InsertCtx

**Categories:** Basic insertion (single patterns), chained insertion (sequential), position handling (boundary cases)

---

## Performance

### Time Complexity
- Search: O(n·log m) (n=query length, m=graph size)
- Split: O(d·p) (d=depth, p=parents per level)
- Join: O(k+m) (k=split points, m=merges)
- **Total: O(d·p + k + m)**

### Space
- TraceCache: O(v) (v=visited vertices)
- SplitCache: O(k·p) (k=splits, p=parents/split)
- JoinContext: O(m) (m=new patterns)

### Optimization Opportunities
1. Cache split results for repeated operations
2. Lazy split evaluation (defer until needed)
3. Parallel join (independent merges)
4. Reduce TraceCache size (prune unused)

---

## Design Rationale

**Why Split-Join?**
- Preserves existing references (no in-place modification)
- Maintains all graph invariants
- Enables concurrent reads during insertion
- Supports undo/rollback easily

**Why Interval Management?**
- Tracks insertion progress for partial operations
- Enables resumable insertion
- Separates concerns (split state vs join state)

**Why Multiple Roles (Pre/Post/In)?**
- Precise boundary handling (prefix vs postfix vs infix)
- Different split strategies per position type
- Optimizations for specific cases

---

## Known Issues

| Issue | Impact | Status |
|-------|--------|--------|
| `end_bound` semantics unclear | Position calc errors | See CONTEXT_INSERT_ARCHITECTURE |
| PathCoverage handling | Test failures | Investigation needed |
| Multiple representations | Complexity | Design trade-off |
| Dependency bloat | Compile time, binary size | Future reduction planned |

---

## Algorithm Specification

**Inputs:**
- Graph state (Hypergraph)
- InitInterval (from search or manual construction)
- Extraction mode (complete vs interval)

**Outputs:**
- New token representing extended pattern
- Updated graph (immutable reference semantics)
- Split/join metadata (via caches)

**Guarantees:**
1. No existing patterns modified
2. All graph invariants maintained
3. New pattern reachable from superstrings
4. Width consistency preserved
5. Parent-child bidirectionality maintained

**Postconditions:**
- `graph.find_ancestor(extended_query)` succeeds
- Old queries still work (non-breaking)
- New token has valid patterns (≥2 children for non-atoms)

---

## Future Enhancements

1. **Batch insertion** - Insert multiple patterns in single pass
2. **Incremental splitting** - Lazy evaluation, defer splits
3. **Split cache persistence** - Reuse across operations
4. **Parallel join** - Concurrent merge operations
5. **Dependency reduction** - Minimize external crates

---

## vs. Traditional Approaches

| Approach | Modification | Refs | Concurrency | Invariants |
|----------|--------------|------|-------------|------------|
| Trie insert | In-place | Breaks | Locks needed | Manual check |
| Immutable | Copy-on-write | Safe | Lock-free | Auto-maintained |
| **Split-join** | Add-only | **Safe** | **Concurrent reads** | **Auto-maintained** |

**Advantage:** Best of both worlds - safe references + efficient updates
