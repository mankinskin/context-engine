# Bug Reports & Problem Analyses Index

Known issues, bug reports, and architectural problem analyses.

## Confidence Ratings

| Rating | Meaning | Agent Action |
|--------|---------|-------------|
| 🟢 **High** | Root cause confirmed, solution verified | Trust analysis, apply fix if not already done |
| 🟡 **Medium** | Analysis incomplete or fix untested | Verify before applying, may need more investigation |
| 🔴 **Low** | Preliminary analysis or possibly fixed | Check if still relevant before investigating |

## Quick Search by Tag

| Tag | Description |
|-----|-------------|
| `#search` | Search algorithm issues |
| `#formatting` | Display and formatting problems |
| `#architecture` | Architectural issues and design problems |
| `#matching` | Pattern matching bugs |
| `#cache` | Caching-related issues |

---

## All Bug Reports & Analyses

### BUG_REPORT_CAN_ADVANCE.md
**Confidence:** 🟢 High - Root cause confirmed, reproducible

**Summary:** `can_advance`/`advance` inconsistency causing panic in `range1` test.

**Tags:** `#search` `#matching` `#panic`

**Root cause:** Code checks if child path can advance but then advances cursor path (independent paths).

**Location:** `crates/context-search/src/match/root_cursor.rs:91-108`

**Error:** `query_advanced returned Break when can_advance was true`

**Status:** Root cause identified, fix strategy documented

---

### DEBUG_VS_COMPACT_FORMAT.md
**Confidence:** 🟢 High - Architectural principle, actively followed

**Summary:** Architectural guidance on separation between `Debug` and `CompactFormat` traits.

**Tags:** `#formatting` `#architecture` `#design-pattern`

**Problem:** Overriding `Debug` trait on domain types violates separation of concerns and breaks standard debugging.

**Solution:**
- Always `derive(Debug)` on domain types
- Implement `CompactFormat` for custom formatting
- Use wrapper types (`DebugFormat`, `CompactDebugFormat`) to control output

**Key principle:** Never override `Debug` trait - use custom traits instead.

**Key locations:**
- `CompactFormat` trait definition
- Wrapper types: `DebugFormat`, `CompactDebugFormat`, `WithFormat`
- Domain types: Token, Path, SearchState, etc.

---

### SEARCH_ALGORITHM_ANALYSIS_SUMMARY.md
**Confidence:** 🟡 Medium - Thorough analysis but implementation may have changed

**Summary:** Comprehensive analysis of current vs desired search algorithm behavior.

**Tags:** `#search` `#algorithm` `#architecture` `#deviation`

**Problem areas:**
1. Best match checkpointing - current implementation doesn't track smallest Complete match
2. Queue clearing - missing on Complete match discovery
3. Trace cache commitment - traces on every update instead of only final
4. Width comparison - no comparison between Complete matches

**Contains:**
- Side-by-side comparison of current vs desired behavior
- Test case analysis (`find_ancestor1_a_b_c_c`)
- Root cause identification for each deviation
- Deep dive into best match checkpointing issues

**Related:** `BEST_MATCH_IMPLEMENTATION_STRATEGY.md` in agents/implemented/ contains the fix plan.

**Key locations:**
- `crates/context-search/src/match/root_cursor.rs`
- `crates/context-search/src/search.rs` - SearchState and last_match tracking
- BinaryHeap processing in search loop
