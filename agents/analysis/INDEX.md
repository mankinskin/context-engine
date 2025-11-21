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
