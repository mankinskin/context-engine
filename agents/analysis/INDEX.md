# Algorithm Analysis Index

Deep analysis and comparison documents for algorithms and architectural decisions.

## Confidence Ratings

| Rating | Meaning | Agent Action |
|--------|---------|-------------|
| 🟢 **High** | Current, accurate analysis | Trust conclusions |
| 🟡 **Medium** | May be outdated or incomplete | Verify current state |
| 🔴 **Low** | Historical or superseded | Use for context only |

## Quick Search by Tag

| Tag | Description |
|-----|-------------|
| `#search` | Search algorithm analysis |
| `#algorithm` | Algorithm comparisons |
| `#architecture` | Architectural analysis |
| `#design` | Design decisions |

---

## All Analysis Documents

### ALGORITHM_COMPARISON.md
**Confidence:** 🟡 Medium - Detailed comparison but may reflect older implementation

**Summary:** Detailed comparison between desired search algorithm and current `find_ancestor` implementation.

**Tags:** `#search` `#algorithm` `#comparison` `#architecture`

**Key Findings:**
- High-level alignment between desired and current implementation
- Bottom-up exploration with ascending width priority matches design
- Differences in queue clearing behavior (removed after bugs)
- Best match tracking vs last_match in SearchState
- Initialization and incremental tracing differences

**Related Files:**
- `crates/context-search/src/search.rs` - Main search implementation
- `agents/guides/SEARCH_ALGORITHM_GUIDE.md` - How the algorithm works
- `agents/guides/DESIRED_SEARCH_ALGORITHM.md` - Desired algorithm specification

---

### CONTEXT_INSERT_ANALYSIS.md
**Confidence:** 🟢 High - Comprehensive analysis of current implementation

**Summary:** Deep analysis of context-insert algorithm including split-join architecture, dependencies, performance, and design insights.

**Tags:** `#insert` `#algorithm` `#split-join` `#dependencies` `#performance`

**Key Findings:**
- Split-join architecture enables safe pattern insertion without modifying existing structures
- Algorithm phases: Search → Initialize → Split → Join → Result
- Time complexity: O(d*p + k*p + m*c) dominated by search and split
- Space complexity: O(cache + k*p) for split cache
- External dependencies could be reduced (linked-hash-map, maplit, pretty_assertions)
- InitInterval bridges search results to insertion operations

**Covered Topics:**
- Complete algorithm flow with examples
- Dependency analysis (internal and external)
- Testing patterns and helpers
- Performance characteristics and optimizations
- Design rationale (why split-join, why intervals, why roles)
- Algorithm specification with guarantees

**Related Files:**
- `crates/context-insert/src/` - Implementation
- `crates/context-insert/HIGH_LEVEL_GUIDE.md` - Concepts
- `agents/guides/CONTEXT_INSERT_GUIDE.md` - Usage patterns
- `crates/context-insert/src/tests/` - Test examples

---

### GRAPH_INVARIANTS.md
**Confidence:** 🟢 High - Reviewed and finalized by author

**Summary:** Specification of the eight core required invariants that all hypergraph operations must preserve. Complete formal specification with examples, validation approaches, and maintenance guidelines.

**Tags:** `#invariants` `#graph` `#correctness` `#specification` `#validation` `#required`

**Core Required Invariants (8):**
- 🟢 **Width Consistency** - Sum of children widths equals parent width (validated)
- 🟢 **Pattern Completeness** - Non-atoms have patterns with ≥2 children (validated)
- 🟢 **Parent-Child Bidirectional** - Relationships maintained in both directions (enforced)
- 🟢 **Atom Uniqueness** - Each atom value appears once (structural guarantee)
- 🟢 **Multiple Representation Consistency** - All patterns of token represent same string (required)
- 🟢 **Substring Reachability** - All substrings reachable from superstrings (required)
- 🟢 **String-Token Uniqueness** - Each string has at most one token (required)
- 🟢 **Position Validity** - All positions within valid bounds (required)

**Key Principles:**
- Each token represents exactly one unique string
- String-token mapping is bijective (one-to-one)
- All eight invariants are mandatory
- All operations must preserve all invariants

**Covered Topics:**
- Each invariant with formal definition and examples
- Derived properties (6 properties from core invariants)
- Invariant maintenance during insert/search/read
- Testing patterns for all eight invariants
- Common violations (7 violation types)
- Formal verification approach

**Related Files:**
- `crates/context-trace/src/graph/vertex/data.rs` - Validation code (width, patterns)
- `crates/context-trace/src/graph/insert.rs` - Parent-child updates
- `crates/context-trace/src/graph/mod.rs` - Atom uniqueness via atom_keys
- `crates/context-insert/src/` - Invariant preservation during insertion

---

### CONTEXT_READ_ANALYSIS.md
**Confidence:** 🟡 Medium - Based on partial implementation, future features inferred

**Summary:** Analysis of context-read layer for high-level graph reading and expansion through block iteration, expansion chains, and complement operations.

**Tags:** `#read` `#expansion` `#blocks` `#complement` `#high-level` `#orchestration`

**Key Concepts:**
- **Block Iteration** - Split sequences into known/unknown blocks for efficient processing
- **Expansion Chains** - Track series of pattern expansions to find largest context
- **Complement Operations** - Calculate missing pieces when patterns don't align
- **ReadCtx** - Main orchestrator combining search, insert, and expansion
- **Band Chains** - Ordered series of overlapping patterns during expansion

**Current State:**
- Core infrastructure implemented (blocks, expansion, complements)
- Basic reading operations functional
- Advanced features (async, optimization) planned

**Future Capabilities:**
- Intelligent pattern discovery
- Context-aware reading
- Streaming pattern processing
- Pattern compression
- Parallel block processing

**Covered Topics:**
- Algorithm flow (block → expand → complement → build)
- Component deep dive (ReadCtx, BlockIter, ExpansionCtx, BandChain)
- Complement calculation and usage
- Performance characteristics
- Integration with other layers
- Use cases (NLP, code analysis, compression, learning)

**Related Files:**
- `crates/context-read/src/` - Implementation
- `crates/context-read/README.md` - Overview
- Depends on: context-trace, context-search, context-insert

---

### TRAIT_CONSOLIDATION_V2_ISSUES.md
**Confidence:** 🟢 High - Current analysis of trait consolidation state

**Summary:** Comprehensive analysis of issues remaining from Phase 1 trait consolidation, identifying 18 qualified trait calls and ~30 deprecation warnings that need resolution.

**Tags:** `#refactoring` `#traits` `#api` `#technical-debt` `#architecture`

**Key Issues Identified:**
1. **Incomplete Trait Hierarchy** - Missing Tier 2 concrete role accessors (StartPath/EndPath)
2. **18 Qualified Trait Calls** - Verbose `HasRolePath::<R>::role_path()` syntax throughout
3. **Confusing Deprecation** - HasRolePath marked deprecated but architecturally necessary
4. **Role-Generic Pattern Not Supported** - PathAccessor can't handle dual-role RootedRangePath
5. **Inconsistent Migration State** - Mix of old and new APIs

**Root Cause:**
- PathAccessor designed for path vector access only (`&Vec<Node>`)
- Many algorithms need RolePath struct access (for `root_entry` field)
- RootedRangePath has dual roles (Start + End), can't implement PathAccessor twice (E0119)

**Proposed Solution:**
- Add Tier 2 traits: StartPathAccessor, EndPathAccessor, RangePathAccessor
- Keep HasRolePath but un-deprecate it (serves different purpose than PathAccessor)
- Migrate 18 call sites to use concrete role accessors
- Remove truly deprecated traits (HasPath, HasRootedPath, etc.)

**Impact:**
- 18 files with qualified trait calls
- ~30 deprecation warnings in build output
- Unclear which API to use for new code

**Related Files:**
- `crates/context-trace/src/path/accessors/path_accessor.rs` - Tier 1 traits (existing)
- `crates/context-trace/src/path/accessors/has_path.rs` - Deprecated traits
- `agents/plans/PLAN_TRAIT_CONSOLIDATION_V2.md` - Full migration plan
- Call sites in: index_range.rs, pattern_range.rs, role_path/mod.rs, path/mod.rs, trace/child/state.rs, cursor/path.rs

---

### ADVANCED_QUERY_CURSOR_RESPONSE.md
**Confidence:** 🟢 High - Fresh analysis with detailed solutions

**Summary:** Analysis of `find_consecutive1` test failure caused by missing advanced query cursor state in Response/MatchResult. Comprehensive comparison of 5 architectural solutions with recommendation for unified cursor position approach.

**Tags:** `#search` `#response` `#cursor` `#architecture` `#bug-analysis` `#design-decision`

**Problem:**
- Test expects `end_index=3` (first unmatched token "a") but gets `end_index=2` (last matched token "i")
- After matching "ghi", query advances to look for "a" but never finds it
- `MatchResult` only stores checkpoint cursor (Matched state), loses advanced query position
- `create_parent_exploration_state()` creates cursor with checkpoint position but advanced path
- Second consecutive search starts from wrong position

**Root Cause:**
- `MatchResult.cursor` represents checkpoint only (last confirmed match)
- When query advances but child cannot (need parent exploration), we lose advanced query state
- Current code uses `current().path` but `checkpoint.atom_position` - mismatch!

**Solution Options Compared (5):**
1. **Optional Candidate Cursor** - Add `advanced_cursor: Option<PatternCursor>` (+5% code, medium usability)
2. **Generic MatchResult** - Make cursor state generic with enum (+20% code, low usability, breaks API)
3. **QueryState in Response** - Add enum to Response (+8% code, medium-high usability)
4. **Unified CursorPosition** - Mirror `Checkpointed<C>` pattern (+10% code, high usability) ✅ **RECOMMENDED**
5. **Extend PathCoverage** - Add AdvancedQuery variant (+12% code, low simplicity)

**Recommended: Option 4 - Unified Cursor Position**
```rust
pub struct CursorPosition {
    pub checkpoint: PatternCursor,  // Last confirmed match
    pub current: PatternCursor,     // Current exploration position
}
```

**Implementation Plan (7 phases, ~3.5 hours):**
1. Create CursorPosition type with accessors
2. Update MatchResult structure
3. Fix create_parent_exploration_state()
4. Fix create_result_from_state()
5. Update all callers
6. Update Response API
7. Fix tests

**Rationale:**
- Mirrors internal `Checkpointed<C>` architecture
- Clear `.checkpoint()` vs `.current()` semantics
- Single field manages both states
- Type-safe without generics
- Natural migration path

**Impact on best_match:**
- 6 locations update best_match in search flow
- Inconclusive end (need parent) is the bug location
- Need to preserve advanced query cursor for consecutive searches

**Design Questions:**
1. Naming preference? (CursorPosition ✅)
2. Default accessor behavior? (.cursor() → checkpoint ✅)
3. Breaking change acceptable? (Yes - migration needed)
4. Consecutive search start point? (Current/advanced ✅)
5. Documentation priority? (High - core concept)

**Related Files:**
- `crates/context-search/src/state/matched/mod.rs` - MatchResult definition
- `crates/context-search/src/match/root_cursor/advance.rs` - create_parent_exploration_state()
- `crates/context-search/src/search/mod.rs` - create_result_from_state(), best_match logic
- `crates/context-search/src/tests/search/consecutive.rs` - Failing test
- `target/test-logs/find_consecutive1.log` - Test execution trace

