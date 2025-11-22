# How-To Guides Index

Persistent guidance for common patterns and troubleshooting.

## Confidence Ratings

| Rating | Meaning | Agent Action |
|--------|---------|-------------|
| 🟢 **High** | Verified, current, complete | Trust and apply directly |
| 🟡 **Medium** | Mostly accurate, may have gaps | Apply but verify edge cases |
| 🔴 **Low** | Outdated or incomplete | Use as starting point, explore thoroughly |

## Quick Search by Tag

| Tag | Description |
|-----|-------------|
| `#testing` | Test setup, tracing, debugging |
| `#tracing` | Logging infrastructure |
| `#api` | API usage patterns |
| `#macros` | Macro usage |
| `#refactoring` | Code transformation tools |
| `#algorithm` | Search algorithms, matching logic |
| `#advance-cycle` | Root cursor advancement and parent exploration |
| `#checkpoint` | Checkpoint cursor system |
| `#parent-exploration` | Hierarchical token expansion |
| `#search-flow` | Overall search process |
| `#root-cursor` | RootCursor implementation details |

---

## All Guides

### TOKEN_TEST_LABELING_GUIDE.md
**Confidence:** 🟢 High - Recently verified, complete solution

**Summary:** Fix token test labeling so logs show readable names like `"a"` instead of `T0w1`.

**Tags:** `#testing` `#tracing` `#logging` `#debugging`

**Solves:**
- Tokens showing as `T0w1` instead of readable names
- Missing graph parameter in `init_test_tracing!()`
- Test graph registration issues

---

### COMPACT_FORMAT_GUIDE.md
**Confidence:** 🟢 High - Current, well-documented system

**Summary:** Compact formatting system for readable tracing logs with multiple output levels.

**Tags:** `#tracing` `#logging` `#formatting`

**Solves:**
- Verbose log output
- Single-line vs multi-line format control
- Custom type formatting in logs

---

### TRACING_GUIDE.md
**Confidence:** 🟢 High - Stable infrastructure, actively used

**Summary:** Tracing infrastructure, structured logging, test log files, and configuration.

**Tags:** `#tracing` `#logging` `#testing`

**Solves:**
- Tracing setup in tests
- Log level and filter configuration
- Test log file usage
- Structured logging with spans

---

### ROOTED_PATH_MACRO_GUIDE.md
**Confidence:** 🟡 Medium - Functional but may have undocumented edge cases

**Summary:** Using `rooted_path!` macro for clean path construction.

**Tags:** `#macros` `#api` `#paths`

**Solves:**
- Verbose path construction
- IndexRangePath/PatternRangePath creation
- Paths with child locations

---

### UNIFIED_API_GUIDE.md
**Confidence:** 🟢 High - Complete implementation, tested

**Summary:** Unified API for parsing, generating, and transforming import/export statements in refactor-tool.

**Tags:** `#refactoring` `#api` `#imports`

**Solves:**
- Processing Rust import/export statements
- Merging and transforming use statements
- Import analysis and replacement

---

### SEARCH_ALGORITHM_GUIDE.md
**Confidence:** 🟢 High - Comprehensive, current design

**Summary:** How the hierarchical pattern search algorithm works, including query exhaustion vs exact match distinction.

**Tags:** `#search` `#algorithm` `#hierarchical` `#pattern-matching` `#response-api`

**Solves:**
- Understanding search algorithm flow
- Query exhaustion vs exact match semantics
- Checkpoint system and result types
- Hierarchical pattern discovery

---

### DESIRED_SEARCH_ALGORITHM.md
**Confidence:** 🟢 High - Authoritative algorithm specification

**Summary:** Official specification for the desired search algorithm behavior - bottom-up exploration with ascending width priority.

**Tags:** `#search` `#algorithm` `#specification` `#design`

**Solves:**
- Algorithm specification and design goals
- Best match tracking and queue management
- Parent state tracking and comparison process
- Bottom-up exploration strategy

**Note:** This is the specification document. See SEARCH_ALGORITHM_GUIDE.md for explanation of current implementation, and agents/analysis/ALGORITHM_COMPARISON.md for detailed comparison.

---

### CONTEXT_INSERT_GUIDE.md
**Confidence:** 🟢 High - Verified patterns and tested examples

**Summary:** Practical guide for using context-insert to add patterns to hypergraphs through split-join architecture.

**Tags:** `#insert` `#patterns` `#split-join` `#initialization` `#testing`

**Solves:**
- How to insert patterns into the graph
- Converting search results to InitInterval
- Understanding insertion modes (insert, insert_init, insert_or_get_complete)
- Testing insertion operations
- Debugging insertion failures
- Performance optimization tips
- Common mistakes and solutions

**Key Patterns:**
- Basic pattern insertion
- Insert only if needed
- Handle partial matches
- Multiple representations
- Incremental building
- Batch operations

**Related:** See agents/analysis/CONTEXT_INSERT_ANALYSIS.md for algorithm details, crates/context-insert/HIGH_LEVEL_GUIDE.md for concepts.

---

### ADVANCE_CYCLE_GUIDE.md
**Confidence:** 🟢 High - Complete current implementation documentation

**Summary:** Complete guide to hierarchical search advance cycle with checkpointed cursors - how patterns match across token boundaries through parent exploration.

**Tags:** `#advance-cycle` `#checkpoint` `#parent-exploration` `#search-flow` `#root-cursor` `#algorithm` `#hierarchical`

**Solves:**
- Understanding the advance cycle flow (Initial Match → Root Advancement → Parent Exploration → Result Selection)
- Checkpoint vs current cursor semantics (confirmed vs exploring positions)
- When and why parent exploration is triggered
- How priority queue ordering works (min-heap by token width)
- Debugging checkpoint-related issues
- End index calculation problems
- Best checkpoint tracking logic

**Key Concepts:**
- Checkpointed cursor architecture (current + checkpoint fields)
- Dual cursor system (query + child cursors)
- Hybrid cursor construction for parent exploration
- State transitions and end conditions
- Queue clearing and hierarchical expansion

**Critical Functions:**
- `advance_to_end()` - Main advancement loop with 3 outcomes
- `create_checkpoint_state()` - Partial match for parent exploration
- `create_end_state()` - Query exhaustion or mismatch results

**Common Issues:**
- Wrong end_index (use current.path + checkpoint.atom_position)
- Queue not clearing after match
- Parent exploration not triggering
- Best checkpoint not optimal

**Related:** SEARCH_ALGORITHM_GUIDE.md for algorithm overview, CHEAT_SHEET.md for types and patterns.
