---
tags: `#plan` `#context-read` `#context-api` `#context-cli` `#ux` `#algorithm` `#read` `#insert` `#search` `#multi-phase`
summary: Multi-phase plan to improve context-read UX, tie up loose ends between context-insert and context-read, create dungeon-crawler skill docs, and build a comprehensive CLI integration test suite.
status: 🚧
phase: 2-design (IN PROGRESS — 6 design plan files created)
---

# Plan: Context-Read UX Improvement & Algorithm Completion

**Date:** 2026-03-14
**Updated:** 2026-03-14 (both interview rounds complete, ready for design phase)
**Scope:** Large (cross-crate, algorithm redesign, CLI UX, documentation, testing)
**Crates:** `context-read`, `context-api`, `context-cli`, `context-search`, `context-insert`

---

## Table of Contents

1. [Objective](#objective)
2. [Research Summary](#research-summary)
3. [Current State Assessment](#current-state-assessment)
4. [Algorithm Family Summary](#algorithm-family-summary)
5. [Desired User Experience](#desired-user-experience)
6. [Loose Ends Inventory](#loose-ends-inventory)
7. [Proposed Architecture: `insert_next_match`](#proposed-architecture-insert_next_match)
8. [Dungeon Crawler Skill Documentation Plan](#dungeon-crawler-skill-documentation-plan)
9. [Integration Test Suite Plan](#integration-test-suite-plan)
10. [Interview Questions](#interview-questions)
11. [Phase Plan Overview](#phase-plan-overview)

---

## Objective

Transform the `context-read` command into the **main feature** of the context navigation stack by:

1. **Improving the text-to-graph pipeline** — accept raw strings or files, read text in the current workspace
2. **Automatically creating atoms** for new characters (no duplicates)
3. **Iterative largest-match indexing** — using iterators, search for known patterns, index largest matches, track overlapping largest matches
4. **Implementing `insert_next_match`** — a single operation that calls search at the current cursor position, guarantees a new valid token for the largest match found
5. **Response architecture** for different result kinds that all resolve to a single response token
6. **Exportable skill documentation** using dungeon-crawler examples
7. **Comprehensive CLI integration test suite** with informal, manual test cases

---

## Research Summary

### Crate Health (as of 2026-03-14)

| Crate | Tests | Pass | Fail | Ignored | Status |
|-------|-------|------|------|---------|--------|
| `context-trace` | ~39 | 39 | 0 | 0 | ✅ Stable |
| `context-search` | 64 | 57 | 0 | 7 | ✅ Stable (known repeat-pattern bug) |
| `context-insert` | 30 | 29 | 0 | 1 | ✅ Stable |
| `context-read` | 60 | 31 | 29 | 0 | ❌ Broken (overlap/repeat logic) |
| `context-api` | 292 | 277 | 15 | 0 | ⚠️ Partial (insert/read integration) |

### Architecture Stack

```
┌──────────────────────────────────────────────────────────┐
│ context-cli (user-facing binary)                         │
│   → parses CLI args / REPL input                         │
│   → delegates to context-api                             │
├──────────────────────────────────────────────────────────┤
│ context-api (unified API layer)                          │
│   → Command/CommandResult protocol                       │
│   → WorkspaceApi trait                                   │
│   → Workspace management + persistence                   │
├──────────────────────────────────────────────────────────┤
│ context-read (orchestration — THIS IS THE FOCUS)         │
│   → ReadRequest → ReadCtx → SegmentIter → Expansion     │
│   → Drives the search → insert → commit pipeline        │
├──────────────────────────────────────────────────────────┤
│ context-insert (graph mutation)                          │
│   → Split-join architecture                              │
│   → insert_or_get_complete, insert_init                  │
├──────────────────────────────────────────────────────────┤
│ context-search (pattern matching)                        │
│   → BFS parent exploration                               │
│   → Response with PathCoverage + CheckpointedCursor      │
├──────────────────────────────────────────────────────────┤
│ context-trace (foundation)                               │
│   → Hypergraph, Token, Pattern, TraceCache               │
│   → DashMap + per-vertex RwLock                          │
└──────────────────────────────────────────────────────────┘
```

### Key Documents Reviewed

| Document | Location | Relevance |
|----------|----------|-----------|
| Context-Read Completion Plan | `agents/plans/20260218_PLAN_CONTEXT_READ_COMPLETION.md` | Previous attempt at fixing context-read (not executed) |
| Search Repeat-Pattern Bug | `agents/analysis/20260205_SEARCH_REPEAT_PATTERN_BUG.md` | Critical: search returns `abab` instead of `ababab` for `[ab,ab,ab]` |
| Insert Edge Cases | `agents/analysis/20260205_CONTEXT_INSERT_EDGE_CASES.md` | 5 failure modes documented, 2 fixed |
| Read State Analysis | `agents/analysis/20260206_CONTEXT_READ_STATE_ANALYSIS.md` | Root cause: `append_to_pattern` destroys vertices |
| API Phase 2 Plan | `agents/plans/20260310_PLAN_CONTEXT_API_PHASE2.md` | Read commands defined but simplified |
| Block Iteration Guide | `agents/guides/20260207_BLOCK_ITER_OVERLAP_EXPANSION.md` | SegmentIter + BlockExpansionCtx |
| BandChain Guide | `agents/guides/20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md` | Overlap link tracking |

---

## Current State Assessment

### What Works

1. **Linear reads** — Strings with no repeated substrings are read correctly as flat atom chains
2. **Simple pattern creation** — `insert_pattern([a, b])` creates vertices correctly
3. **Search** — `find_ancestor` finds largest containing patterns (with caveats below)
4. **Insert split-join** — `insert_or_get_complete` correctly splits/joins for partial matches
5. **Atom management** — Auto-creation, deduplication, lookup all working
6. **API command protocol** — `Command`/`CommandResult` serialization round-trips
7. **Workspace lifecycle** — Create, open, close, save, delete, list all working
8. **CLI REPL** — Interactive mode with workspace context tracking

### What's Broken

1. **29/60 context-read tests fail** — Any input with repeated substrings produces wrong graph structure
2. **15/292 context-api tests fail** — Insert reports `already_existed=true` incorrectly; read returns truncated text
3. **Cursor advancement** — `CursorCtx` doesn't advance after `insert_or_get_complete`
4. **`append_to_pattern` destroys vertices** — Modifies width in-place, corrupting intermediate tokens
5. **Search repeat-pattern bug** — Queue is prematurely cleared after first match, missing sibling candidates
6. **ComplementBuilder** — Returns minimal `TraceCache` (documented TODO)
7. **`ExpandCtx` inserts full cursor** — Should insert only the overlap portion
8. **`OverlapStack`** — Most methods commented out; not functional

### What's Missing for `context-cli read <string>`

1. **No text-based read command** — `read-pattern` and `read-as-text` only accept numeric vertex indices
2. **No search+read+insert pipeline** — User must manually do `insert-sequence` → `read-pattern <index>`
3. **No file input** — No mechanism to read text from files or stdin
4. **REPL rejects non-numeric read input** — `read hello` fails with "not a valid index"
5. **No combined insert-then-read** — No single command that reads text and shows its decomposition
6. **The underlying `context-read` crate is broken** for repeated substrings

---

## Algorithm Family Summary

### 1. Read Algorithm (`context-read`)

**Purpose:** Transform raw text into a deduplicated, hierarchical hypergraph representation.

**Core Idea:** Given a string, identify all recurring substrings and store each unique substring only once as a vertex, with multiple decompositions (child patterns) capturing the different contexts in which it appears. The result is a single root token representing the entire input, composed of the largest known tokens.

**Pipeline:**
```
text → chars → NewAtomIndices → SegmentIter → [unknown|known] segments
  → unknown: append atoms directly to root
  → known: BlockExpansionCtx → ExpansionCtx → BandState → collapse → commit to root
```

**Key Property:** The root token is a maximal-compression representation: each recurring substring is stored once and referenced by position in parent patterns.

**Current Status:** Linear reads work; overlap/repeat detection broken.

### 2. Insert Algorithm (`context-insert`)

**Purpose:** Insert a new token into the hypergraph while maintaining the containment hierarchy invariant (two nodes have a path between them iff one is a substring of the other).

**Core Idea:** Given a search Response with a partial match, split the matched token at the match boundary and join the new content alongside the split pieces. The split-join architecture ensures no existing references are broken.

**Pipeline:**
```
searchable → search() → Response
  → if complete: return existing token
  → if partial: Response → InitInterval { root, cache, end_bound }
    → SplitPhase: trace cache → SplitStates → SplitCache (per-vertex split positions)
    → JoinPhase: FrontierSplitIterator → NodeJoinCtx → MergeCtx → new Token
```

**Three Entry Points:**
- `insert(searchable)` — Search then auto-insert if incomplete
- `insert_init(extract, init)` — Insert directly from pre-computed `InitInterval`
- `insert_or_get_complete(searchable)` — Search; return existing if complete, otherwise insert

**Current Status:** ✅ Working. 29/29 tests pass.

### 3. Search Algorithm (`context-search`)

**Purpose:** Given a sequence of tokens, find the largest existing token in the hypergraph that matches (is an ancestor of) the query.

**Core Idea:** BFS parent exploration starting from the first query token. For each parent candidate, compare leaf tokens of the candidate against the query. Track match state via a typestate cursor (`Matched`/`Candidate`/`Mismatched`). The `Response` unifies all result types with two orthogonal properties: `query_exhausted()` and `is_entire_root()`.

**Pipeline:**
```
input → Searchable::start_search → StartCtx → first parent batch
  → SearchState (BFS Iterator):
    → pop SearchNode (ParentCandidate | ChildCandidate)
    → CompareState: compare leaf tokens (decompose if widths differ)
    → RootCursor: advance through candidate, match/mismatch/exhaust
    → Response { cache: TraceCache, end: MatchResult, events }
```

**Result Classification (`PathCoverage`):**
- `EntireRoot` — Complete token match (query = existing token)
- `Prefix` — Query matches start of a token
- `Postfix` — Query matches end of a token
- `Range` — Query matches a middle slice

**Current Status:** ✅ Working with known bug: premature queue clearing for repeated patterns (`[ab,ab,ab]` → returns `abab` instead of `ababab`).

---

## Desired User Experience

### `context-cli read <string>`

The ideal user experience when using `context-cli read` should be a **single command** that takes raw text and produces a fully indexed decomposition tree:

```bash
# Read a string into the current workspace (creates atoms automatically)
$ context-cli read myworkspace "hello world hello"

Reading "hello world hello" (17 chars)...
  → Created 8 new atoms: h, e, l, o, ' ', w, r, d
  → Indexed 3 unique tokens: "hello" (5), " world " (7), "hello" (reused)
  → Root token: #42 (width 17)

  Decomposition:
    #42 "hello world hello"
    ├── #38 "hello"         (appears 2x)
    │   ├── h
    │   ├── e
    │   ├── l
    │   ├── l
    │   └── o
    ├── #39 " world "
    │   ├── ' '
    │   ├── w
    │   ├── o
    │   ├── r
    │   ├── l
    │   └── d
    ├── ' '
    └── #38 "hello"         (reused)
```

### Key UX Properties

1. **Single command** — `context-cli read <workspace> <text>` does everything
2. **Auto-atom creation** — No need to `add-atom` first; unknown characters are auto-registered
3. **Deduplication visible** — Output shows which tokens are reused
4. **Tree decomposition** — Shows hierarchical structure, not just flat text
5. **Streaming-friendly** — Should work for large inputs (files, pipes)
6. **Incremental** — Reading more text into the same workspace enriches the graph; shared substrings are discovered across reads
7. **Idempotent** — Reading the same text twice returns the same root token

### Extended Features (file/stdin support)

```bash
# Read from a file
$ context-cli read myworkspace --file ./game_log.txt

# Read from stdin
$ cat game_log.txt | context-cli read myworkspace --stdin

# Read multiple files  
$ context-cli read myworkspace --file log1.txt --file log2.txt

# REPL usage
myworkspace> read "hello world"
myworkspace> read --file ./data.txt
```

### REPL Integration

In the REPL, `read` should become the **primary command** with smart argument parsing:

```
myworkspace> read hello world
  → reads "hello world" as text

myworkspace> read 42
  → reads vertex #42 as decomposition tree (backwards compatible)

myworkspace> read --file data.txt
  → reads file contents as text
```

---

## Loose Ends Inventory

### Between `context-insert` and `context-read`

| # | Loose End | Location | Severity | Description |
|---|-----------|----------|----------|-------------|
| 1 | Cursor advancement after `insert_or_get_complete` | `context-read/expansion/mod.rs`, `cursor.rs` | 🔴 Critical | `CursorCtx` doesn't advance after finding a match. The `PatternRangePath` cursor stays at position 0. |
| 2 | `insert_or_get_complete` returns first match, not largest | `context-search/match/iterator.rs:102-106` | 🔴 Critical | Queue is cleared after first `RootCursor` match. Sibling candidates (like `ababab` when `abab` matches first) are lost. |
| 3 | `ExpandCtx` inserts full cursor instead of overlap portion | `context-read/expansion/chain/expand.rs:57-62` | 🟡 High | The insert call operates on the entire remaining cursor pattern, not the specific overlapping subsequence. |
| 4 | `append_to_pattern` modifies vertex width in-place | `context-trace/graph/insert/parents.rs:122` | 🔴 Critical | Destructively changes vertex width via `*vertex.width_mut() += width.0`, corrupting the intermediate vertex for reuse. |
| 5 | `ComplementBuilder` returns minimal `TraceCache` | `context-read/complement.rs:60-69` | 🟡 High | TODO: should use search/checkpoint API to build a proper trace cache. |
| 6 | `build_postfix_complement` has partial token extraction TODO | `context-read/expansion/chain/mod.rs:330-334` | 🟡 High | Doesn't handle the case where a token partially overlaps. |
| 7 | `OverlapStack` methods commented out | `context-read/expansion/stack.rs` | 🟠 Medium | `find_appendable_band`, `NestedStack` struct are commented out. Stack-based overlap tracking not functional. |
| 8 | `expansion/chain/op.rs` is empty | `context-read/expansion/chain/op.rs` | 🟢 Low | Empty file — chain operation types live in `link.rs` instead. |
| 9 | Response architecture for `insert_next_match` | Not yet designed | 🔴 Critical | No unified operation exists that searches at cursor position and guarantees a valid result token. |
| 10 | `context-api` read commands only accept vertex indices | `context-api/commands/read.rs` | 🟡 High | No `ReadSequence { text }` or `ReadFile { path }` command variant. |
| 11 | `context-api` insert reports `already_existed` incorrectly | `context-api/commands/insert.rs` | 🟠 Medium | 11 test failures in insert commands. |
| 12 | `PathNode` widths hardcoded to 1 in visualization | `context-search/search/events.rs` (7 TODOs) | 🟢 Low | Affects visualization accuracy, not search correctness. |

### Between `context-api` and `context-cli`

| # | Loose End | Location | Severity | Description |
|---|-----------|----------|----------|-------------|
| 13 | No `ReadSequence` command | `context-api/commands/mod.rs` | 🔴 Critical | Missing `Command::ReadSequence { workspace, text }` variant. |
| 14 | No file input support | `context-cli/src/main.rs` | 🟡 High | No `--file` or `--stdin` argument on any command. |
| 15 | REPL `read` only accepts numeric indices | `context-cli/src/repl.rs:468-490` | 🟡 High | Needs smart parsing: numeric → vertex lookup, text → read sequence. |
| 16 | REPL `search` only uses first token | `context-cli/src/repl.rs` | 🟠 Medium | `search` doesn't join multiple words like `insert` does. |

---

### Proposed Architecture: `insert_next_match`

### Concept

`insert_next_match` is a **renaming and enhancement of the existing `insert_or_get_complete`** on the `ToInsertCtx` trait in `context-insert`. It is NOT a fresh implementation — it evolves the existing API to:

1. Support a distinct "no expansion" response variant
2. Carry rich response data (search `Response` for caching/debugging) in all variants
3. Always return `IndexWithPath` (no generics — simplified API)
4. Be usable as the primitive inside context-read's expansion pipeline (`ExpandCtx`)

The method stays on `ToInsertCtx` in `context-insert`. The expansion pipeline in `context-read` (`ExpansionCtx`/`BandState`/`BlockExpansionCtx`) remains and delegates to `insert_next_match`.

### Signature (proposed — evolves existing API)

The current signature:
```rust
// CURRENT (to be renamed + enhanced)
fn insert_or_get_complete(
    &self,
    searchable: impl Searchable<InsertTraversal>,
) -> Result<Result<R, R::Error>, ErrorReason>
```

Proposed new signature (simplified — no generics, always returns IndexWithPath):
```rust
// NEW — replaces insert_or_get_complete
fn insert_next_match(
    &self,
    searchable: impl Searchable<InsertTraversal>,
) -> Result<InsertOutcome, ErrorReason>
```

Where `InsertOutcome` replaces the confusing nested `Result`:

```rust
/// Outcome of insert_next_match — always resolves to a single token + path.
/// Each variant carries the matched/created IndexWithPath and the search
/// Response for caching, debugging, and downstream visibility.
pub enum InsertOutcome {
    /// Newly created via split+join pipeline.
    /// The query extended beyond what was known; a new token was inserted.
    Created {
        result: IndexWithPath,
        response: Response,
    },

    /// Full match already existed — the query was fully consumed by an
    /// existing token. No insertion was needed.
    Complete {
        result: IndexWithPath,
        response: Response,
    },

    /// No expansion: the search found an existing token at the start of
    /// the query, but the query extends beyond it. No new token was created.
    /// The caller should advance by the returned token's width and try again.
    NoExpansion {
        result: IndexWithPath,
        response: Response,
    },
}

impl InsertOutcome {
    /// The matched/created token + cursor path (all variants carry this)
    pub fn result(&self) -> &IndexWithPath { ... }

    /// Consume into the IndexWithPath
    pub fn into_result(self) -> IndexWithPath { ... }

    /// The token (shorthand for result().index)
    pub fn token(&self) -> Token { self.result().index }

    /// The search response (for caching, debugging, trace inspection)
    pub fn response(&self) -> &Response { ... }

    /// Whether the match expanded beyond what was already known
    pub fn is_expanded(&self) -> bool {
        matches!(self, InsertOutcome::Created { .. })
    }

    /// Whether the query was fully consumed by an existing token
    pub fn is_complete(&self) -> bool {
        matches!(self, InsertOutcome::Complete { .. })
    }

    /// Whether no expansion occurred (starting token is the best match)
    pub fn is_no_expansion(&self) -> bool {
        matches!(self, InsertOutcome::NoExpansion { .. })
    }
}
```

**Design decisions (from interview):**
- **Flat enum over nested Result** (Q14): `InsertOutcome` is a flat 3-variant enum. The outer `Result<InsertOutcome, ErrorReason>` handles hard errors. Clean and unambiguous.
- **Rich responses in all variants** (Q15): Each variant carries the search `Response` for improved caching and visibility. Callers can inspect why no expansion happened.
- **Always `IndexWithPath`, no generics** (Q16): Simplified API. The `InsertResult` trait generic parameter and `TryInitWith` encoding are removed from this method. Context-read always needs the cursor path; context-api benefits from path info too.

### Mapping from Current Logic to `InsertOutcome`

Inside `InsertCtx::insert_impl` (context.rs), the current branches map to:

| Current code branch | Current return | New return |
|---------------------|---------------|------------|
| `is_entire_root() && query_exhausted()` | `Ok(R::try_init(...))` → `Ok(Err(iwp))` for IndexWithPath | `Ok(InsertOutcome::Complete { result, response })` |
| `is_entire_root() && !query_exhausted()` | `Ok(R::try_init(...))` → `Ok(Err(iwp))` for IndexWithPath | `Ok(InsertOutcome::NoExpansion { result, response })` |
| `!is_entire_root()` (partial match) | `Ok(Ok(R))` via `insert_init` | `Ok(InsertOutcome::Created { result, response })` |
| search error | `Err(ErrorState)` | `Err(ErrorReason)` |

The key improvement: the current API conflates "complete match" and "no expansion" into the same `Err(R::Error)` variant. The new `InsertOutcome` makes them distinct. The `TryInitWith` trait can be simplified or removed since the Ok/Err encoding is no longer needed.

**Note:** The existing generic `insert()` and `insert_init()` methods on `ToInsertCtx<R>` remain unchanged — only `insert_or_get_complete` is renamed and simplified to `insert_next_match`. The `InsertResult` trait and `R` parameter continue to be used by `insert()` and `insert_init()`.

### How Context-Read Uses `insert_next_match`

The expansion loop in `ExpandCtx` (context-read) iterates over postfixes of the anchor token in descending size:

```
for each postfix of anchor_token (descending by size):
    result = graph.insert_next_match(postfix_pattern + remaining_cursor)
    match result:
        Created { result, response }     → overlap found! Build expansion band, break
        Complete { result, response }    → exact match, consume and continue
        NoExpansion { result, response } → skip this postfix, try next smaller one
if no postfix expanded:
    → append anchor_token to band as-is, advance cursor by anchor width
```

This is the "travel down postfixes in descending size to find the largest postfix which allows an expansion" behavior described in the interview answer. The `response` field in each variant provides the search trace cache, which can be reused for subsequent operations (e.g., building complements, debugging expansion decisions).

### Read Algorithm Using `insert_next_match` (Outer Loop)

The outer read loop (in `ReadCtx`) processes segments from `SegmentIter`:

```
read(text):
  1. atoms = text.chars().map(|c| get_or_create_atom(c))
  2. segments = SegmentIter::new(atoms.to_new_atom_indices(graph))
  3. root = RootManager::new(graph)
  4. for segment in segments:
       root.append_pattern(segment.unknown)  // unknown atoms → direct append
       if !segment.known.is_empty():
         block = BlockExpansionCtx::new(root, segment.known)
         block.process()  // drives ExpansionCtx → ExpandCtx → insert_next_match
         root = block.finish()
  5. return root.token()
```

### Handling Overlapping Largest Matches

The read algorithm maintains **multiple bands (patterns) side-by-side**:
- Each band covers a contiguous atom range with specific token boundaries
- **Invariant (assertion):** No two bands have token boundaries at the same atom positions (detectable by cumulative atom width of preceding tokens)
- Overlaps are detected when a postfix of the anchor token matches the prefix of the remaining cursor
- An overlap creates a new band (via `BandState::WithOverlap`) with different token boundaries
- `BandState::collapse` groups both bands into the same vertex via `insert_patterns`
- For now, the invariant is enforced via `debug_assert!` in the expansion loop (Q19: keep it simple, use assertions for validation)
- A future improved multi-band architecture is designed but not implemented yet — the algorithm is extremely sensitive and we must implement with correct understanding first

**Key insight:** overlapping matches are automatically tracked by the hypergraph's containment hierarchy. If "ab" and "abab" both exist, there's a path from "ab" to "abab" (since "ab" is a substring). The read algorithm only needs to index the **largest** match; smaller matches are reachable via the hierarchy. Alternative decompositions (e.g., `ababab = [abab, ab]` AND `[ab, abab]`) are created by the band collapse mechanism.

---

## Dungeon Crawler Skill Documentation Plan

### Concept

Create **exportable skill documents** (separate from internal guides) that explain the hypergraph model using a **dungeon crawler terminal game** as the running example. This makes the abstract concepts concrete and memorable.

### Why Dungeon Crawlers?

A dungeon-crawler terminal game produces **highly repetitive text**:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Room: Cave       │    │ Room: Cave       │    │ Room: Dungeon    │
│                  │    │                  │    │                  │
│ You see a goblin │    │ You see a chest  │    │ You see a goblin │
│ HP: 100/100      │    │ HP: 85/100       │    │ HP: 85/100       │
│ > attack goblin  │    │ > open chest     │    │ > attack goblin  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

Many substrings recur: `"Room: "`, `"You see a "`, `"HP: "`, `"/100"`, `"goblin"`, etc. These are perfect for demonstrating hierarchical deduplication.

### Skill Documents (3-4 documents)

#### Skill 1: "The Hypergraph Model — Tokens All the Way Down"

**Audience:** Engineers new to the context-engine

**Content:**
- What is a token in the hypergraph (vertex with width, children, parents)
- Atoms: the leaf level (individual characters)
- Patterns: sequences of tokens that decompose a parent token
- Multiple decompositions: one token, many ways to split it
- The containment hierarchy: "ab" is a child of "abc" because it's a substring
- The reachability invariant: path exists iff substring relationship exists

**Dungeon Crawler Example:**
```
Game log: "You see a goblin. You see a chest."

Atoms: Y, o, u, ' ', s, e, a, g, b, l, i, n, '.', c, h, t

Tokens built during read:
  "You"          = [Y, o, u]
  " see"         = [' ', s, e, e]
  " a "          = [' ', a, ' ']
  "You see a "   = ["You", " see", " a "]      ← reused!
  "goblin"       = [g, o, b, l, i, n]
  "chest"        = [c, h, e, s, t]
  "You see a goblin" = ["You see a ", "goblin"]
  "You see a chest"  = ["You see a ", "chest"]  ← shares "You see a "!

Root: ["You see a goblin", ". ", "You see a chest", "."]
```

#### Skill 2: "Reading Text — The Iterative Largest-Match Algorithm"

**Audience:** Engineers working on or with context-read

**Content:**
- The read pipeline: text → atoms → segments → expansion → root
- What "largest match" means (search finds the biggest known token)
- How `insert_next_match` works step by step
- Cursor advancement and the iteration loop
- Why we greedily match the largest token (compression efficiency)
- How new tokens emerge from partial matches (split-join insertion)

**Dungeon Crawler Example:**
```
First read: "You see a goblin"
  → All atoms are new, creates flat sequence
  
Second read: "You see a chest"
  → "You see a " is KNOWN (largest match = width 10)
  → "chest" is NEW (no match beyond individual atoms)
  → Result: ["You see a ", "chest"]
  → Graph now has shared "You see a " token!

Third read: "You see a goblin again"
  → "You see a goblin" is KNOWN (largest match = width 16!)
  → " again" is NEW
  → Result: ["You see a goblin", " again"]
```

#### Skill 3: "Context Completion — Finding Meaning in Small Tokens"

**Audience:** Engineers interested in the analysis/query capabilities

**Content:**
- Small tokens have "context" = their parent tokens in the hierarchy
- Looking up parents of "goblin" reveals all contexts it appears in
- Comparing two game logs by decomposition similarity
- How the hypergraph enables substring-based similarity search
- "Context completion": given a small token, what larger structures contain it?

**Dungeon Crawler Example:**
```
After indexing 100 dungeon crawler games:

Token "goblin" appears in parents:
  → "You see a goblin"     (30 games)
  → "attack goblin"        (25 games)  
  → "goblin drops loot"    (20 games)
  → "a goblin appears"     (15 games)

Token "dragon" appears in parents:
  → "You see a dragon"     (5 games — rare!)
  → "dragon breathes fire"  (3 games)

Similarity: "goblin" and "dragon" share context pattern "You see a ___"
  → Both are "encounter targets" in the game's implicit grammar
```

#### Skill 4: "Overlapping Decompositions — Why One Token Has Many Patterns"

**Audience:** Engineers debugging pattern issues or working on the insert algorithm

**Content:**
- Why a single token needs multiple child patterns
- How overlaps create alternative decompositions
- The `BandState::WithOverlap` mechanism
- Real example: `"abcabc"` has both `[abc, abc]` and potential overlap decompositions
- How this preserves all structural information

**Dungeon Crawler Example:**
```
Game screens often share boundaries:

Screen 1: "Room: Cave\nYou see a goblin"
Screen 2: "You see a goblin\nHP: 100/100"

"You see a goblin" appears at the END of Screen 1 and the START of Screen 2.

The token "Screen1+Screen2" has TWO decompositions:
  [Screen1, Screen2]                     ← standard
  [Room: Cave\n, "You see a goblin", \nHP: 100/100]  ← overlap-aware

Both are valid and stored in the same vertex!
```

---

## Integration Test Suite Plan

### Philosophy

- **Failing tests are good** — they point attention to unexpected behavior
- **Correct test expectations matter more** than making tests pass
- **Manual, informal style** — executable in CLI, easy to understand
- **Incremental complexity** — start simple, build to complex scenarios
- **Cross-layer testing** — test through the CLI, not just individual crates

### Test Categories

#### Category 1: Atom Management (6 tests)
```bash
# T1.1: Create workspace and add single atom
context-cli create test-atoms
context-cli add-atom test-atoms a
context-cli list-atoms test-atoms
# EXPECT: atom 'a' exists with index 0

# T1.2: Add duplicate atom (should not create new)
context-cli add-atom test-atoms a
context-cli list-atoms test-atoms
# EXPECT: still only 1 atom, same index

# T1.3: Add multiple atoms at once
context-cli add-atoms test-atoms "hello"
context-cli list-atoms test-atoms
# EXPECT: atoms h, e, l, o created (plus 'a' from before)

# T1.4: Atoms from add-atoms are deduplicated
context-cli add-atoms test-atoms "hello"
context-cli list-atoms test-atoms
# EXPECT: same count as before, no duplicates

# T1.5: Special characters as atoms
context-cli add-atoms test-atoms "!@# "
# EXPECT: !, @, #, space all created

# T1.6: Empty string creates no atoms
context-cli add-atoms test-atoms ""
# EXPECT: no change
```

#### Category 2: Basic Read (8 tests)
```bash
# T2.1: Read a simple string (the main UX test!)
context-cli create test-read
context-cli read test-read "abc"
# EXPECT: root token created, width=3, atoms a,b,c auto-created

# T2.2: Read creates atoms automatically
context-cli read test-read "xyz"
# EXPECT: atoms x,y,z created, root token width=3

# T2.3: Read same string twice returns same token
context-cli read test-read "abc"
# EXPECT: same root token as T2.1

# T2.4: Read empty string
context-cli read test-read ""
# EXPECT: no token created, graceful message

# T2.5: Read single character
context-cli read test-read "x"
# EXPECT: returns atom token, width=1

# T2.6: Read with spaces
context-cli read test-read "a b c"
# EXPECT: root token width=5 (spaces are atoms)

# T2.7: Read and then read-as-text to verify
context-cli read test-read "hello"
# Note the index from output
context-cli read-as-text test-read <index>
# EXPECT: outputs "hello"

# T2.8: Read and show decomposition tree
context-cli read test-read "hello"
context-cli read-pattern test-read <index>
# EXPECT: tree with h, e, l, l, o as leaves
```

#### Category 3: Deduplication / Shared Substrings (8 tests)
```bash
# T3.1: Repeated substring detected
context-cli create test-dedup
context-cli read test-dedup "abab"
# EXPECT: "ab" token reused, root = [ab, ab]

# T3.2: Shared prefix across reads
context-cli read test-dedup "abcde"
context-cli read test-dedup "abcxy"
# EXPECT: "abc" token shared between both roots

# T3.3: Shared suffix across reads
context-cli read test-dedup "hello world"
context-cli read test-dedup "brave world"
# EXPECT: " world" or "world" token shared

# T3.4: Multiple shared substrings
context-cli read test-dedup "the cat sat on the mat"
# EXPECT: "the " or "at" tokens reused

# T3.5: Triple repetition
context-cli create test-triple
context-cli read test-triple "abcabcabc"
# EXPECT: "abc" and "abcabc" as intermediates

# T3.6: Infix sharing
context-cli create test-infix
context-cli read test-infix "subdivision"
context-cli read test-infix "visualization"
# EXPECT: shared substring "vis" or "ion"

# T3.7: Incremental enrichment
context-cli create test-incr
context-cli read test-incr "hypergraph"
context-cli read test-incr "hyper"
context-cli read test-incr "graph"
# EXPECT: "hypergraph" decomposes to [hyper, graph]

# T3.8: Rotational overlaps
context-cli create test-rotate
context-cli read test-rotate "abcde"
context-cli read test-rotate "bcdea"
context-cli read test-rotate "cdeab"
# EXPECT: shared tokens "bcde", "cde", etc.
```

#### Category 4: File/Stdin Input (4 tests)
```bash
# T4.1: Read from file
echo "hello world" > /tmp/test_input.txt
context-cli create test-file
context-cli read test-file --file /tmp/test_input.txt
# EXPECT: root token created for "hello world"

# T4.2: Read from stdin
echo "hello world" | context-cli read test-file --stdin
# EXPECT: root token (should be same as T4.1 if same text)

# T4.3: Read large file
# (generate a file with repeated content)
context-cli read test-file --file /tmp/large_input.txt
# EXPECT: completes without error, shows compression ratio

# T4.4: Read multiple files
context-cli read test-file --file /tmp/file1.txt --file /tmp/file2.txt
# EXPECT: each file gets its own root, shared substrings detected across files
```

#### Category 5: REPL Integration (6 tests)
```bash
# T5.1: REPL read with text (new behavior)
# In REPL:
# > create test-repl
# > read "hello world"
# EXPECT: root token created

# T5.2: REPL read with index (backwards compatible)
# > read 5
# EXPECT: decomposition tree for vertex 5

# T5.3: REPL read without quotes
# > read hello world
# EXPECT: reads "hello world" as text

# T5.4: REPL insert then read
# > insert "hello world"
# > read "hello world"
# EXPECT: same root token

# T5.5: REPL search after read
# > read "the quick brown fox"
# > search "quick"
# EXPECT: finds "quick" within the graph

# T5.6: REPL show after read
# > read "abcabc"
# > show
# EXPECT: graph visualization showing shared structure
```

#### Category 6: Edge Cases & Error Handling (6 tests)
```bash
# T6.1: Read into non-existent workspace
context-cli read nonexistent "hello"
# EXPECT: error "workspace not found"

# T6.2: Read into closed workspace
context-cli create test-err && context-cli close test-err
context-cli read test-err "hello"
# EXPECT: error or auto-reopens

# T6.3: Very long string
context-cli create test-long
context-cli read test-long "$(python3 -c "print('ab' * 1000)")"
# EXPECT: completes, shows compression (width < 2000 characters if deduplication works)

# T6.4: Unicode characters
context-cli read test-long "こんにちは世界"
# EXPECT: each character is an atom

# T6.5: Newlines in input
context-cli read test-long "line1\nline2\nline1"
# EXPECT: "line1" reused, '\n' as atom

# T6.6: Read and validate graph integrity
context-cli read test-long "abcabc"
context-cli validate test-long
# EXPECT: graph validates successfully
```

### Test Harness Structure (Rust)

Integration tests live in `tools/context-cli/tests/` as a Rust test module:

```
tools/context-cli/tests/
├── integration/
│   ├── mod.rs                   # Test module root
│   ├── helpers.rs               # Shared test utilities (workspace setup/teardown, assertion helpers)
│   ├── test_atoms.rs            # Category 1: Atom management
│   ├── test_basic_read.rs       # Category 2: Basic read operations
│   ├── test_deduplication.rs    # Category 3: Shared substring detection
│   ├── test_file_input.rs       # Category 4: File/stdin input
│   ├── test_repl.rs             # Category 5: REPL integration (via CLI binary)
│   └── test_edge_cases.rs       # Category 6: Error handling
└── cli_integration.rs           # Crate-level test file that imports integration/
```

Tests in Categories 1–4, 6 call `context-api` functions directly for speed and precise assertions.
Tests in Category 5 invoke the `context-cli` binary via `std::process::Command` for true end-to-end UX validation.

---

## Interview Questions — Round 1 (Answered)

> Answers received 2026-03-14. Integrated into the plan below with analysis.

### Architecture & Scope

**Q1: `insert_next_match` placement — which crate should own this operation?**

> **Answer:** `insert_next_match` should be part of a **renaming of the existing insert call infrastructure** in `context-insert`. No fresh implementation — rename and adapt the existing `insert_or_get_complete` API.

**Analysis:** This means we rename `insert_or_get_complete` → `insert_next_match` across the codebase. The method stays on `ToInsertCtx` trait in `context-insert`. The signature evolves to support the "no expansion" response variant. Call sites affected:
- `context-insert/src/insert/mod.rs` — trait declaration + default impl
- `context-insert/src/insert/context.rs` — `InsertCtx` impl
- `context-api/src/commands/insert.rs` — 2 call sites (`insert_first_match`, `insert_sequence`)
- `context-insert/src/tests/` — ~9 test call sites
- `context-read/src/expansion/chain/expand.rs` — uses `ToInsertCtx::insert` (may switch to renamed API)
- Documentation in `agents/` — ~8 references

**Q2: Should `insert_next_match` replace the current pipeline, or work alongside it?**

> **Answer:** It is **used inside** the current expansion pipeline. `context-read` uses the `context-insert` API. The expansion pipeline (`ExpansionCtx`/`BandState`/`BlockExpansionCtx`) stays and delegates to `insert_next_match`.

**Analysis:** The current `ExpandCtx` in `context-read/expansion/chain/expand.rs` already calls `ToInsertCtx::insert`. The rename targets this call site. The expansion pipeline's state machine (`BandState::Single` → `WithOverlap` → `collapse`) remains intact. `insert_next_match` becomes the primitive that `ExpandCtx` delegates to, with better response typing so the expansion loop can distinguish "found expansion" vs "no expansion" vs "exact match".

**Q3: How should we handle the `append_to_pattern` vertex destruction bug?**

> **Answer:** **Fix the function.** We need to preserve tokens already referenced by larger parents. Appending to a vertex during reading must create new vertices to add context after a root token and reset its root adaptively. Only when a root can be modified safely (no external references) can we update in-place.

**Analysis:** The current guard (`child_patterns().len() == 1 && parents().is_empty()`) is necessary but not sufficient. Per Q17, we **split into two explicit functions**:

1. **`extend_root_pattern(parent, pattern_id, new) → Token`** — Safe function that always creates a new vertex. Copies the existing pattern, appends new tokens, creates a new vertex with the combined pattern. The original vertex is untouched. This is the default for context-read's `RootManager`.

2. **`append_to_owned_pattern(parent, pattern_id, new) → Token`** — In-place function that modifies the vertex directly. Only safe when the vertex is truly "owned" (no parents, single pattern, no external Token handles). Callers must guarantee safety. Used only in controlled scenarios where the vertex was just created and has no external references.

The split lives in `context-trace` (making the API explicit about safety) but is kept **separate from the domain logic fixes** per Q4 below. The old `append_to_pattern` is deprecated in favor of the two explicit functions.

**Q4: Should we fix the search repeat-pattern bug as part of this plan?**

> **Answer:** **Keep domain logic fixes separate for now.** Focus on creating a valid testing environment and improved usability and documentation.

**Analysis:** The search repeat-pattern bug (`[ab,ab,ab]` → `abab` instead of `ababab`) stays as a known issue. We:
- Document it in the test suite as an expected failure
- Write tests that demonstrate the correct expected behavior (which will fail)
- Do NOT attempt to fix `SearchIterator::next()` queue-clearing logic in this plan
- The `append_to_pattern` fix IS in scope (Q3) because it's a data-integrity issue, not domain logic

**Impact:** Some context-read tests will continue to fail due to the search bug. This is acceptable — failing tests pointing to known issues is the goal.

### Read Algorithm Design

**Q5: When `insert_next_match` finds no larger match, what happens?**

> **Answer:** `insert_next_match` is **allowed to not expand** and only return the starting token. However the response type must reflect this. During reading, there is a phase where we **travel down the postfixes of a token in descending size** to find the largest postfix which allows an expansion into context. While we find no expansion, we skip. For this we want to use `insert_next_match`.

**Analysis:** The response type needs a distinct "no expansion" variant (`InsertOutcome::NoExpansion`). The read algorithm's expansion loop uses it to skip postfixes that don't advance:
```
for each postfix of anchor_token (descending by size):
    result = insert_next_match(postfix, remaining_cursor)
    if result.is_expanded():
        → use this expansion, break
    else:
        → skip, try next smaller postfix
if no postfix expanded:
    → append anchor_token to band as-is, advance cursor
```

See the `InsertOutcome` enum definition in the "Proposed Architecture" section above. The `response` field on each variant provides the search Response for caching and debugging visibility (Q15 answer).

**Q6: How should overlapping largest matches be tracked?**

> **Answer:** The read algorithm maintains **multiple patterns side-by-side** where overlapping tokens can be placed. Each overlap is either appended to an existing band or creates a new band. **Invariant:** no two patterns are allowed to have token boundaries at the same positions (detectable by atom width of preceding tokens). Different compositions for the same larger string should already have been grouped by block expansion. This requirement is an **assertion maintained during the reading loop**.

**Analysis:** This confirms the existing `BandState` design direction but with a clearer invariant:
- **Band = a pattern being built** — a sequence of tokens covering a contiguous atom range
- **Multiple bands** = multiple alternative decompositions being tracked simultaneously
- **Assertion:** for any two bands `B1`, `B2` covering the same atom range, their cumulative token boundary positions must differ (i.e., they can't split at the same atom positions)
- When the expansion loop finds an overlap at a postfix boundary, it creates a new band (via `BandState::WithOverlap`) with different token boundaries
- `BandState::collapse` groups these into the same vertex with multiple child patterns via `insert_patterns`

Per Q19: **keep it simple for now** — use `BandState` as-is with `debug_assert!` for validation. The `OverlapStack` stays commented out. A future improved multi-band architecture can be designed but not implemented until we have correct understanding verified by the test suite. The algorithm is extremely sensitive.

**Q7: One-pass or multi-pass?**

> **Answer:** The reading algorithm should be designed as a **one-pass async stream consumer**. It should be able to consume an asynchronous stream of text left to right. For now we assume only synchronous input streams.

**Analysis:** The current `Iterator`-based architecture (`SegmentIter → ReadCtx → ExpansionCtx → ExpandCtx`) is already well-suited for this:
- `ReadCtx` implements `Iterator` — each `.next()` processes one segment
- The chain is pull-based (lazy) — items are computed on demand
- `context-read` already declares `tokio`, `tokio-stream`, `futures`, `async-trait`, `async-recursion`, and `pin-project-lite` as dependencies (currently unused)

Design principle: **keep the synchronous `Iterator` core, design the API so it can be wrapped in a `Stream` later**. Concretely:
- The `ReadCtx` iterator stays synchronous
- Input can be `impl Iterator<Item = char>` (sync) now, `impl Stream<Item = char>` (async) later
- The `ReadRequest` API gains a `from_reader(impl Read)` method for file/stdin input
- No `async fn` in core algorithm crates — async adapters live at the edge (context-api, context-http)

**Lazy atom resolution (Q18):** Chars are resolved against the graph state **at the time they are consumed** (lazy). The current block-based design uses the known/unknown distinction as a domain optimization: encountering an unknown token ends a "known" block. Most execution happens within "known" blocks where all possible chars have been seen. This prunes search paths over new characters, which can't be in any pattern yet. This property may also emerge naturally from the search algorithm (new characters always result in no expansion), but the explicit block separation provides a clear, efficient implementation.

### UX & CLI

**Q8: Default output for `context-cli read <string>`?**

> **Answer:** For now, **show only a summary**. In the future, we can output visualizations, logs, or more.

**Analysis:** Output format (summary only for now):
```
Read "hello world hello" → root #42 (width 17, 3 unique tokens, 8 atoms)
```
Future flags: `--tree`, `--json`, `--verbose`, `--viz`. Kept out of scope for this plan.

**Q9: Should `read` accept token references?**

> **Answer:** Either **text, or a list of tokens**. Both are valid inputs.

**Analysis:** The CLI's `read` command needs smart parsing:
- If all args parse as `usize` → treat as token index list
- If args contain non-numeric content → treat as text
- Explicit: `read --text "hello"` vs `read --tokens 1 2 3`
- REPL shorthand: `read 42` (index) vs `read "hello"` (text) vs `read hello world` (text)

**Q10: File input — separate or concatenated?**

> **Answer:** **Both should be possible.** Either an ordered list of files (separate roots, ordered) or an unordered set of files (separate roots, no particular order).

**Analysis:** CLI flags:
- `read --file a.txt --file b.txt` → ordered list, each file gets its own root, processed in order
- `read --files a.txt b.txt` → unordered set, each file gets its own root, order unspecified
- Shared substrings are discovered across files via the shared hypergraph
- No concatenation mode needed initially (user can `cat` files themselves)

### Testing & Documentation

**Q11: CLI integration tests — shell scripts or Rust?**

> **Answer:** **Rust test harness.**

**Analysis:** Create integration tests in `tools/context-cli/tests/` using Rust's `#[test]` framework. Tests invoke the CLI binary via `std::process::Command` or directly call `context-api` functions. Benefits:
- Type-safe assertions
- Integrates with `cargo test`
- Can use `assert!` macros with descriptive messages
- Can share test utilities via a helper module
- Failures are reported with full context

**Q12: Dungeon crawler skill docs — where?**

> **Answer:** New `docs/skills/` directory (separate from internal `agents/guides/`).

**Analysis:** Create:
```
docs/skills/
├── README.md                              # Index of skill documents
├── 01_hypergraph_model.md                 # Skill 1: Tokens all the way down
├── 02_reading_text.md                     # Skill 2: Iterative largest-match
├── 03_context_completion.md               # Skill 3: Finding meaning in small tokens
└── 04_overlapping_decompositions.md       # Skill 4: Why one token has many patterns
```
These are **external-facing** documentation, not agent workflow docs. They use the dungeon crawler example throughout for consistency.

**Q13: Existing failing tests vs new correct tests?**

> **Answer:** **Design new correct tests** that define the desired behavior, then review existing tests and plan for updating or creating correct tests. Test failures are allowed! The objective is to understand the mismatch between expectations and implementation.

**Analysis:** Test strategy:
1. Write new tests in the Rust harness that define the correct expected behavior
2. Accept that many will fail — each failure documents a gap
3. Review existing 29 failing context-read tests: are their expectations correct? Update or annotate
4. Review existing 15 failing context-api tests: same treatment
5. Create a test status tracker (`FAILING_TESTS.md`) that maps each failure to a root cause

---

## Interview Questions — Round 2 (Answered)

> Follow-up questions based on deeper research. Answers received 2026-03-14.

### `insert_next_match` Return Type Design

**Q14: Flat `InsertOutcome` enum or nested `Result`?**

> **Answer:** Yes, flat enum looks clean.

**Decision:** `Result<InsertOutcome, ErrorReason>` with flat 3-variant `InsertOutcome` enum. No nested `Result`. See "Proposed Architecture" section for full type definition.

**Q15: Should `InsertOutcome::NoExpansion` carry the search `Response`?**

> **Answer:** Yes — providing rich responses is good for improved caching and visibility.

**Decision:** All three `InsertOutcome` variants carry a `response: Response` field alongside `result: IndexWithPath`. This enables downstream caching, trace inspection, and debugging without re-searching.

**Q16: Always `IndexWithPath` or keep generic `R: InsertResult`?**

> **Answer:** Simplify — always return the path, remove the generics.

**Decision:** `insert_next_match` always returns `InsertOutcome` (which contains `IndexWithPath`). The `InsertResult` trait generic `R` parameter is removed from this method. The existing `insert()` and `insert_init()` methods keep their generics. The `TryInitWith` trait encoding is no longer needed for `insert_next_match`.

### `append_to_pattern` Fix Scope

**Q17: Same signature with adaptive behavior, or split into explicit functions?**

> **Answer:** Split into explicit functions.

**Decision:** Replace `append_to_pattern` with two explicit functions:
1. **`extend_root_pattern(parent, pattern_id, new) → Token`** — Always creates a new vertex. Safe for any context. Default choice.
2. **`append_to_owned_pattern(parent, pattern_id, new) → Token`** — In-place modification. Caller must guarantee safety (no parents, single pattern, no external handles).

The old `append_to_pattern` is deprecated. All 3 call sites in `RootManager` switch to `extend_root_pattern` by default (the `parents().is_empty()` guard can optionally route to `append_to_owned_pattern` for performance).

### Stream Consumer Design

**Q18: Lazy or eager atom resolution?**

> **Answer:** Lazy is correct. The current implementation uses the known/unknown distinction to separate input into blocks. Encountering an unknown token ends a "known" block. Most execution happens within "known" blocks. The block-based organization prunes search paths over new characters (which can't be in any pattern yet). This property may also emerge naturally from search (new characters → no expansion), but explicit blocks provide clarity and efficiency.

**Decision:** Atoms are resolved lazily (at consumption time). The block-based segmentation (`SegmentIter`) is a domain optimization, not a fundamental requirement. The `NewAtomIndices` eager resolution is kept as the mechanism to *detect* block boundaries, but the conceptual model is lazy: each char's known/new status reflects the graph state when that char is processed (including atoms added earlier in the same read).

### Multi-Band Overlap Tracking

**Q19: Revive `OverlapStack`, new `BandTracker`, or keep `BandState` + assertions?**

> **Answer:** Keep it simple for now. Use assertions for validation only. Design a future improved architecture conceptually but don't implement it yet. The algorithm is extremely sensitive — must implement with correct understanding first.

**Decision:** `BandState` stays as-is (`Single`/`WithOverlap`). Add `debug_assert!` for the token-boundary-position invariant. The `OverlapStack` remains commented out. A future `BandTracker` design can be documented in `agents/designs/` but not implemented in this plan.

### Test Harness Design

**Q20: CLI binary invocation, direct API calls, or both?**

> **Answer:** Yes, both.

**Decision:** Integration test suite uses both approaches:
- **API-level tests** (Categories 1–4, 6): Call `context-api` functions directly for speed and precise assertions on graph structures.
- **CLI-level tests** (Category 5): Invoke the `context-cli` binary via `std::process::Command` for true end-to-end UX validation (output format, error messages, REPL behavior).

---

## Phase Plan Overview

### Phase 1: Research ✅ COMPLETE
- [x] Research all 5 crates (context-trace, context-search, context-insert, context-read, context-api)
- [x] Research CLI tool (context-cli)
- [x] Document current state (test results, module status, loose ends)
- [x] Identify 16 loose ends between crates
- [x] Design proposed architecture (`InsertOutcome`, `insert_next_match`)
- [x] Prepare interview questions (Round 1: Q1–Q13)
- [x] Get answers from user (Round 1)
- [x] Integrate answers, deep-dive into insert API, async patterns, append_to_pattern
- [x] Prepare follow-up questions (Round 2: Q14–Q20)
- [x] Get answers from user (Round 2)
- [x] Finalize all design decisions

### Phase 2: Design ✅ COMPLETE
- ✅ Created detailed implementation plan files:
  - ✅ [`PLAN_INSERT_NEXT_MATCH.md`](20260314_PLAN_INSERT_NEXT_MATCH.md) — Rename `insert_or_get_complete` → `insert_next_match`, add `InsertOutcome` enum (flat, non-generic, with Response), update all 8 production + ~9 test call sites. Includes Response extraction design, migration guide, and 10 atomic execution steps.
  - ✅ [`PLAN_APPEND_TO_PATTERN_FIX.md`](20260314_PLAN_APPEND_TO_PATTERN_FIX.md) — Split into `extend_root_pattern` (safe) + `append_to_owned_pattern` (in-place), update 3 RootManager call sites, deprecate old function. Includes concrete corruption example and 7 execution steps.
  - ✅ [`PLAN_CLI_READ_UX.md`](20260314_PLAN_CLI_READ_UX.md) — CLI changes: `Command::ReadSequence`, `Command::ReadFile`, file input (`--file`), REPL smart parsing (text vs index vs tokens), summary output. 18 execution steps across 4 phases.
  - ✅ [`PLAN_READ_STREAM_DESIGN.md`](20260314_PLAN_READ_STREAM_DESIGN.md) — Lazy atom resolution, generic `SegmentIter<I>`, `ReadSequenceIter`, `from_reader(impl Read)`, async dependency cleanup, future async Stream pattern (design only). 7 execution steps.
  - ✅ [`PLAN_DUNGEON_CRAWLER_SKILLS.md`](20260314_PLAN_DUNGEON_CRAWLER_SKILLS.md) — 4 skill documents in `docs/skills/` with dungeon crawler examples. Includes document template, content outlines, validation scripts. 8 execution steps.
  - ✅ [`PLAN_INTEGRATION_TESTS.md`](20260314_PLAN_INTEGRATION_TESTS.md) — Rust test harness in `tools/context-cli/tests/`, 6 categories, 38+ tests, API-level + CLI-level, `FAILING_TESTS.md` tracker. Includes `TestWorkspace` helper design and failure root cause template. 11 execution steps.
- Dependency graph between plan files (see below)
- Risk assessment and mitigation strategies (included in each plan)
- Test failure inventory: map each current 29+15 failure to a root cause (in PLAN_INTEGRATION_TESTS.md)

#### Dependency Graph

```
PLAN_INSERT_NEXT_MATCH ──────┐ (no dependencies — top priority)
                             │
PLAN_APPEND_TO_PATTERN_FIX ──┤ (independent — can be parallel)
                             │
                             ▼
PLAN_CLI_READ_UX ────────────┤ (depends on INSERT_NEXT_MATCH for ReadSequence impl)
                             │
PLAN_READ_STREAM_DESIGN ─────┤ (depends on INSERT_NEXT_MATCH for expansion pipeline)
                             │
                             ▼
PLAN_DUNGEON_CRAWLER_SKILLS ─┤ (depends on algorithm plans for accurate examples)
                             │
PLAN_INTEGRATION_TESTS ──────┘ (depends on CLI_READ_UX for test targets)
```

**Recommended execution order:**
1. `PLAN_INSERT_NEXT_MATCH` + `PLAN_APPEND_TO_PATTERN_FIX` (parallel — foundation fixes)
2. `PLAN_CLI_READ_UX` + `PLAN_READ_STREAM_DESIGN` (parallel — depend on step 1)
3. `PLAN_DUNGEON_CRAWLER_SKILLS` + `PLAN_INTEGRATION_TESTS` (parallel — depend on step 2)

### Phase 3: Implement (🚧 IN PROGRESS)
- **3a: Foundation fixes** ✅ COMPLETE (2026-03-14)
  - ✅ Split `append_to_pattern` into `extend_root_pattern` (safe, creates new vertex) + `append_to_owned_pattern` (in-place with debug_assert guards) in `context-trace` (`parents.rs`). Original `append_to_pattern` deprecated.
  - ✅ Added `InsertOutcome` enum (`Created`/`Complete`/`NoExpansion`, each carrying `IndexWithPath` + `Response`) in new `context-insert/src/insert/outcome.rs`. Added `insert_next_match` method on `InsertCtx` and `ToInsertCtx` trait. Re-exported `InsertOutcome` from crate root.
  - ✅ Migrated all production call sites: `context-api` `insert_first_match` + `insert_sequence` (2 sites), `context-read` `ExpansionCtx::new` (1 site). All use fully-qualified `ToInsertCtx::<IndexWithPath>::insert_next_match(...)` syntax for type disambiguation.
  - ✅ Migrated all test call sites: `context-insert` tests (11 sites across `context_read_scenarios.rs` + `expanded_overlap.rs`), `context-read` cursor tests (12 sites in `cursor.rs`). All use fully-qualified syntax.
  - ✅ Deprecated `insert_or_get_complete` on both trait and impl with `#[deprecated(since = "0.2.0")]`. Zero deprecation warnings remain.
  - ✅ Verified: no new test failures introduced. Pre-existing failures unchanged (context-api: 15, context-read: 29).
  - ⬜ Clean up unused async dependencies in `context-read` (deferred — not blocking)
- **3b: CLI & API layer**
  - Add `Command::ReadSequence { workspace, text }` to `context-api`
  - Add `Command::ReadFile { workspace, path }` to `context-api`
  - Update `context-cli` with text input, file input, REPL smart parsing
  - Summary output format for read results
- **3c: Documentation**
  - Write 4 dungeon crawler skill documents in `docs/skills/`
  - Update `CHEAT_SHEET.md` with new `InsertOutcome` patterns
  - Update `agents/guides/INDEX.md`
- **3d: Test suite**
  - Create Rust integration test harness in `tools/context-cli/tests/`
  - Write tests for all 6 categories (38+ test cases)
  - Create `FAILING_TESTS.md` mapping each failure to root cause
  - Review and annotate existing failing tests

### Phase 4: Validate
- Run all existing tests (context-read, context-api, context-insert, context-search)
- Run new integration test suite
- Manual CLI testing with real-world text
- Validate dungeon crawler examples produce expected graph structures
- Create test status report: passing, failing (expected), failing (unexpected)
- Document remaining issues and next steps

---

## Research Findings (Round 2)

### `insert_or_get_complete` → `insert_next_match` Rename Scope

**8 production code locations** to rename:
1. `context-insert/src/insert/mod.rs` L31 — trait method declaration
2. `context-insert/src/insert/mod.rs` L35 — default impl body
3. `context-insert/src/insert/context.rs` L62 — `InsertCtx` method
4. `context-api/src/commands/insert.rs` L76 — `insert_first_match` call site
5. `context-api/src/commands/insert.rs` L164 — `insert_sequence` call site
6. `context-api/src/commands/insert.rs` L82, L170 — error messages

**~9 test code locations** to rename (context-insert tests).

**~8 documentation references** in `agents/` directory.

The rename is mechanical. The signature change (`Result<Result<R, R::Error>, ErrorReason>` → `Result<InsertOutcome<R>, ErrorReason>`) requires updating the match arms at all call sites, but the logic mapping is direct (see "Mapping from Current Logic" table above).

### `InsertOutcome` Derivation from Existing Code

The current `insert_impl` in `context-insert/src/insert/context.rs` L181-L216 has three branches that map cleanly to `InsertOutcome`:
- `is_entire_root() && query_exhausted()` → `Complete`
- `is_entire_root() && !query_exhausted()` → `NoExpansion`
- else (partial match → split+join) → `Created`

The `TryInitWith` trait (used to encode the current Ok/Err semantics) can be removed or simplified — `InsertOutcome` makes it unnecessary.

### `append_to_pattern` Fix Strategy

The function at `context-trace/src/graph/insert/parents.rs` L92-145 does three destructive things:
1. `node.get_parent_mut(...).width += width` — mutates children's parent records
2. `pattern.extend(new.iter())` — extends pattern in place
3. `*vertex.width_mut() += width.0` — changes vertex width

All 3 call sites in `RootManager` already guard with `child_patterns().len() == 1 && parents().is_empty()`. The fix: make `append_to_pattern` itself enforce safety by creating a new vertex when the parent has external references, rather than relying on callers to check.

### Async Dependencies in `context-read`

`context-read` declares `tokio`, `tokio-stream`, `async-std`, `futures`, `async-trait`, `async-recursion`, and `pin-project-lite` but **none are used in source code**. These are aspirational. The current architecture is a synchronous iterator chain (`SegmentIter → ReadCtx → ExpansionCtx → ExpandCtx → BandExpandingIterator`) which is the correct foundation for a future async stream adapter.

---

## Related Documents

- [20260218_PLAN_CONTEXT_READ_COMPLETION.md](20260218_PLAN_CONTEXT_READ_COMPLETION.md) — Previous (unexecuted) plan, 7 work items identified
- [20260310_PLAN_CONTEXT_API_OVERVIEW.md](20260310_PLAN_CONTEXT_API_OVERVIEW.md) — API master plan (5 phases)
- [20260310_PLAN_CONTEXT_API_PHASE2.md](20260310_PLAN_CONTEXT_API_PHASE2.md) — API Phase 2 (read commands)
- [20260205_SEARCH_REPEAT_PATTERN_BUG.md](../analysis/20260205_SEARCH_REPEAT_PATTERN_BUG.md) — Search bug (out of scope, documented as known issue)
- [20260205_CONTEXT_INSERT_EDGE_CASES.md](../analysis/20260205_CONTEXT_INSERT_EDGE_CASES.md) — Insert edge cases (2/5 fixed)
- [20260206_CONTEXT_READ_STATE_ANALYSIS.md](../analysis/20260206_CONTEXT_READ_STATE_ANALYSIS.md) — Read state analysis (`append_to_pattern` root cause)
- [20260207_BLOCK_ITER_OVERLAP_EXPANSION.md](../guides/20260207_BLOCK_ITER_OVERLAP_EXPANSION.md) — Block iteration guide
- [20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md](../guides/20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md) — BandChain guide

---

## Next Steps

1. ~~Answer Round 1 questions (Q1–Q13)~~ ✅
2. ~~Answer Round 2 questions (Q14–Q20)~~ ✅
3. ~~Proceed to Phase 2: Design — Create 6 detailed plan files + dependency graph~~ ✅
4. **Proceed to Phase 3: Implement** — Execute plans in dependency order:
   - **3a: Foundation fixes (parallel)**
     - Execute [`PLAN_INSERT_NEXT_MATCH.md`](20260314_PLAN_INSERT_NEXT_MATCH.md) — 10 steps
     - Execute [`PLAN_APPEND_TO_PATTERN_FIX.md`](20260314_PLAN_APPEND_TO_PATTERN_FIX.md) — 7 steps
   - **3b: CLI & API + Stream design (parallel, after 3a)**
     - Execute [`PLAN_CLI_READ_UX.md`](20260314_PLAN_CLI_READ_UX.md) — 18 steps
     - Execute [`PLAN_READ_STREAM_DESIGN.md`](20260314_PLAN_READ_STREAM_DESIGN.md) — 7 steps
   - **3c: Documentation + Testing (parallel, after 3b)**
     - Execute [`PLAN_DUNGEON_CRAWLER_SKILLS.md`](20260314_PLAN_DUNGEON_CRAWLER_SKILLS.md) — 8 steps
     - Execute [`PLAN_INTEGRATION_TESTS.md`](20260314_PLAN_INTEGRATION_TESTS.md) — 11 steps

## Key Design Decisions Summary

| # | Decision | Source |
|---|----------|--------|
| D1 | `insert_next_match` lives in `context-insert` as a rename of `insert_or_get_complete` | Q1 |
| D2 | Expansion pipeline (`ExpansionCtx`/`BandState`) stays, delegates to `insert_next_match` | Q2 |
| D3 | Split `append_to_pattern` → `extend_root_pattern` (safe) + `append_to_owned_pattern` (in-place) | Q3, Q17 |
| D4 | Search repeat-pattern bug is out of scope (documented as known issue in tests) | Q4 |
| D5 | `InsertOutcome`: flat 3-variant enum (`Created`, `Complete`, `NoExpansion`) | Q5, Q14 |
| D6 | All `InsertOutcome` variants carry `Response` for caching/debugging | Q15 |
| D7 | `insert_next_match` always returns `IndexWithPath` (no generics) | Q16 |
| D8 | Multi-band overlap: `BandState` as-is + `debug_assert!` for invariant | Q6, Q19 |
| D9 | One-pass stream consumer, sync iterator core, future async wrapper | Q7 |
| D10 | Lazy atom resolution (char status resolved at consumption time) | Q18 |
| D11 | Summary output for `context-cli read` (future: `--tree`, `--json`) | Q8 |
| D12 | Read accepts text or token list (smart parsing in CLI/REPL) | Q9 |
| D13 | File input: ordered list (`--file`) or unordered set (`--files`), separate roots | Q10 |
| D14 | Rust test harness, API-level + CLI-level tests | Q11, Q20 |
| D15 | Skill docs in `docs/skills/` (separate from `agents/guides/`) | Q12 |
| D16 | Design new correct tests first, review existing, accept failures | Q13 |