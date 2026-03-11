---
tags: `#context-api` `#phase2` `#algorithms` `#search` `#insert` `#read`
summary: Phase 2 — Add algorithm commands (search, insert, read) and developer debug commands to context-api, update CLI
status: 📋
---

# Plan: context-api Phase 2 — Algorithm Commands

## Objective

Extend `crates/context-api` with the algorithm commands that wrap `context-search` (`Find`), `context-insert` (`ToInsertCtx`), and `context-read`. This phase adds `search_pattern`, `search_sequence`, `insert_first_match`, `insert_sequence`, `insert_sequences`, `read_pattern`, `read_as_text`, plus developer commands `get_trace_cache` and `validate_graph`. The CLI is updated with corresponding subcommands and REPL commands.

## Context

### Prerequisites

- **Phase 1 complete** — `crates/context-api` and `tools/context-cli` exist with workspace management, atom/pattern commands, persistence, and error types.
- **context-read dependency enabled** — Phase 1 commented out `context-read` in `Cargo.toml`; this phase enables it.

### Interview Reference

All design decisions: `agents/interviews/20260310_INTERVIEW_CONTEXT_API.md`
Master plan: `agents/plans/20260310_PLAN_CONTEXT_API_OVERVIEW.md`
Phase 1 plan: `agents/plans/20260310_PLAN_CONTEXT_API_PHASE1.md`

### Key Decisions Affecting This Phase

- **Search:** Wraps `Find::find_ancestor` and `Find::find_sequence` — returns complete/partial/not-found
- **Insert:** Always insert, splitting as needed (current context-insert behavior)
- **Read:** Always full depth — the algorithm determines conclusive state
- **TokenRef:** By numeric vertex index or string label; resolution searches the graph
- **Bulk:** `insert_sequences(Set<String>)` — unordered set
- **Errors:** Per-command types (`SearchError`, `InsertError`, `ReadError`) composed into `ApiError`

### Crate Dependencies Involved

| Crate | Traits / Functions Used |
|-------|----------------------|
| `context-search` | `Find::find_ancestor`, `Find::find_sequence`, `Searchable`, `Response` |
| `context-insert` | `ToInsertCtx::insert`, `ToInsertCtx::insert_or_get_complete` |
| `context-read` | Expansion / read context (public API TBD — may need wrappers) |
| `context-trace` | `HypergraphRef`, `Token`, `GraphSnapshot`, graph validation |

### Files Affected

**Modified:**
- `crates/context-api/Cargo.toml` — enable `context-read` dependency
- `crates/context-api/src/lib.rs` — (no changes expected, modules already declared)
- `crates/context-api/src/error.rs` — fill in `SearchError`, `InsertError`, `ReadError` variants
- `crates/context-api/src/types.rs` — add `SearchResult`, `InsertResult`, `PatternReadResult`, `PartialMatchInfo`, `TraceCacheInfo`, `ValidationReport`
- `crates/context-api/src/commands/mod.rs` — add `Command` variants for search/insert/read/debug, update `WorkspaceApi` trait, update `execute()` dispatch
- `tools/context-cli/src/commands.rs` — add CLI subcommands
- `tools/context-cli/src/repl.rs` — add REPL commands
- `tools/context-cli/src/output.rs` — add formatters for new result types

**New:**
- `crates/context-api/src/commands/search.rs` — search command implementations
- `crates/context-api/src/commands/insert.rs` — insert command implementations
- `crates/context-api/src/commands/read.rs` — read command implementations
- `crates/context-api/src/resolve.rs` — `TokenRef` resolution logic (shared by search/insert)
- `crates/context-api/src/tests/search_tests.rs`
- `crates/context-api/src/tests/insert_tests.rs`
- `crates/context-api/src/tests/read_tests.rs`
- `crates/context-api/src/tests/integration_tests.rs` — end-to-end round-trips

---

## Analysis

### Current State (After Phase 1)

The `context-api` crate has:
- `WorkspaceManager` with workspace lifecycle (create/open/close/save/delete)
- `Workspace` holding a `Hypergraph<BaseGraphKind>` in memory
- Atom and simple pattern commands
- `Command` enum for serialized dispatch
- `WorkspaceApi` trait for Rust consumers
- CLI with subcommands and REPL

Missing: any way to search for patterns, insert arbitrary sequences, or read/expand vertices beyond listing them.

### Desired State

After Phase 2, users can:

```pseudo
// Via CLI:
context-cli search-sequence myworkspace "hello world"
context-cli insert-sequence myworkspace "hello world"
context-cli read-pattern myworkspace 42
context-cli read-as-text myworkspace 42
context-cli validate myworkspace

// Via REPL:
(myworkspace)> search hello world
(myworkspace)> insert hello world
(myworkspace)> read 42
(myworkspace)> text 42
(myworkspace)> validate

// Via Command JSON:
{"command": "search_sequence", "workspace": "myworkspace", "text": "hello world"}
{"command": "insert_first_match", "workspace": "myworkspace", "query": [{"index": 0}, {"label": "bc"}]}
{"command": "read_pattern", "workspace": "myworkspace", "index": 42}
```

### Key Technical Challenges

1. **HypergraphRef creation** — The search and insert APIs (`Find`, `ToInsertCtx`) expect `HypergraphRef` (which is `Arc<Hypergraph>`), but `Workspace` owns the graph directly. We need to create a temporary `HypergraphRef` for each operation. Since `HypergraphRef` is `Arc<Hypergraph>`, and we have `&Hypergraph`, we can either:
   - Store `HypergraphRef` inside `Workspace` instead of `Hypergraph` directly
   - Create a temporary `HypergraphRef` by cloning the Arc for the operation

   **Decision:** Change `Workspace` to store `HypergraphRef` internally. This makes the graph shareable without extra cloning. Mutation goes through `Arc::get_mut` (which succeeds when there's only one Arc reference — our case during workspace operations) or we use the existing interior mutability of `Hypergraph` (which uses `DashMap` internally and allows concurrent reads/writes at the vertex level).

2. **Response → SearchResult conversion** — The `context-search::Response` type contains a `TraceCache` and `EndState` with private fields. We need to use the public accessor methods (`is_complete()`, `expect_complete()`, `root_token()`, `query_exhausted()`, etc.) to build our `SearchResult`.

3. **context-read public API** — `context-read` has mostly `pub(crate)` modules. We may need to add a thin public wrapper or use the existing `context` module. This needs investigation during implementation.

4. **TokenRef resolution** — Resolving `TokenRef::Label("abc")` requires searching the graph for a token whose atom sequence matches "abc". For single-char labels, it's a direct atom lookup. For multi-char labels, we use `Find::find_sequence`. This creates a dependency: token resolution itself uses the search API.

---

## Execution Steps

### Step 1: Enable context-read Dependency

**File:** `crates/context-api/Cargo.toml`

Uncomment or add:

```toml
context-read = { path = "../context-read", features = ["test-api"] }
```

**Verification:** `cargo check -p context-api` compiles.

---

### Step 2: Change Workspace to Store HypergraphRef

**File:** `crates/context-api/src/workspace/mod.rs`

```pseudo
pub struct Workspace {
    pub(crate) name: String,
    pub(crate) dir: PathBuf,
    pub(crate) graph: HypergraphRef<BaseGraphKind>,  // changed from Hypergraph
    pub(crate) metadata: WorkspaceMetadata,
    pub(crate) lock: Option<WorkspaceLock>,
    pub(crate) dirty: bool,
}

impl Workspace {
    pub fn graph(&self) -> &Hypergraph<BaseGraphKind> {
        &self.graph  // Deref through HypergraphRef → &Hypergraph
    }

    pub fn graph_ref(&self) -> &HypergraphRef<BaseGraphKind> {
        &self.graph
    }

    // For mutations: since Hypergraph uses DashMap internally,
    // most operations work through &self (interior mutability).
    // The graph_mut() method marks dirty and returns &Hypergraph
    // (which supports mutation via DashMap's interior mutability).
    pub(crate) fn graph_mut(&mut self) -> &Hypergraph<BaseGraphKind> {
        self.dirty = true;
        &self.graph
    }
}
```

**Update:** `WorkspaceManager::create_workspace` and `open_workspace` to construct `HypergraphRef::new(graph)` instead of storing the raw `Hypergraph`.

**Update:** `persistence::save_graph` needs to accept `&Hypergraph` (which it gets via deref from `HypergraphRef`).

**Verification:** All Phase 1 tests still pass after this refactor.

---

### Step 3: TokenRef Resolution Module

**File:** `crates/context-api/src/resolve.rs`

This module resolves `TokenRef` values into concrete `Token` values from the graph.

```pseudo
use context_trace::{Hypergraph, Token, VertexIndex, graph::kind::BaseGraphKind};
use context_search::Find;
use crate::types::TokenRef;
use crate::error::SearchError;

/// Resolve a single TokenRef to a Token in the graph.
pub fn resolve_token_ref(
    graph: &HypergraphRef<BaseGraphKind>,
    token_ref: &TokenRef,
) -> Result<Token, SearchError> {
    match token_ref {
        TokenRef::Index(idx) => {
            // Direct vertex lookup by index
            let vi = VertexIndex(*idx);
            let data = graph.try_get_vertex_data(vi)
                .ok_or(SearchError::TokenNotFound { token_ref: token_ref.clone() })?;
            Ok(data.to_token())
        }
        TokenRef::Label(label) => {
            if label.len() == 1 {
                // Single char → atom lookup
                let ch = label.chars().next().unwrap();
                graph.get_atom_token_by_value(ch)
                    .ok_or(SearchError::TokenNotFound { token_ref: token_ref.clone() })
            } else {
                // Multi-char → search for the token by its atom sequence
                let response = graph.find_sequence(label.chars())
                    .map_err(|e| SearchError::InternalError(format!("{e:?}")))?;

                if response.is_complete() {
                    Ok(response.expect_complete("token ref resolution").root_parent())
                } else {
                    Err(SearchError::TokenNotFound { token_ref: token_ref.clone() })
                }
            }
        }
    }
}

/// Resolve a list of TokenRefs to Tokens.
pub fn resolve_token_refs(
    graph: &HypergraphRef<BaseGraphKind>,
    refs: &[TokenRef],
) -> Result<Vec<Token>, SearchError> {
    refs.iter().map(|r| resolve_token_ref(graph, r)).collect()
}
```

**Verification:** Unit tests — resolve by index (existing/missing), resolve by single-char label, resolve by multi-char label (existing/missing).

---

### Step 4: Error Types for Search, Insert, Read

**File:** `crates/context-api/src/error.rs` (modify existing)

```pseudo
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("workspace '{workspace}' is not open")]
    WorkspaceNotOpen { workspace: String },

    #[error("token not found in graph: {token_ref:?}")]
    TokenNotFound { token_ref: TokenRef },

    #[error("query must contain at least 2 tokens")]
    QueryTooShort,

    #[error("search failed: {0}")]
    InternalError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertError {
    #[error("workspace '{workspace}' is not open")]
    WorkspaceNotOpen { workspace: String },

    #[error("token not found in graph: {token_ref:?}")]
    TokenNotFound { token_ref: TokenRef },

    #[error("query must contain at least 2 tokens")]
    QueryTooShort,

    #[error("insert failed: {0}")]
    InternalError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("workspace '{workspace}' is not open")]
    WorkspaceNotOpen { workspace: String },

    #[error("vertex {index} not found in graph")]
    VertexNotFound { index: usize },

    #[error("read failed: {0}")]
    InternalError(String),
}
```

Also add `From` impls to convert `SearchError` into `InsertError` (since insert uses search internally via token resolution):

```pseudo
impl From<SearchError> for InsertError {
    fn from(e: SearchError) -> Self {
        match e {
            SearchError::WorkspaceNotOpen { workspace } => InsertError::WorkspaceNotOpen { workspace },
            SearchError::TokenNotFound { token_ref } => InsertError::TokenNotFound { token_ref },
            SearchError::QueryTooShort => InsertError::QueryTooShort,
            SearchError::InternalError(msg) => InsertError::InternalError(msg),
        }
    }
}
```

**Verification:** Compiles, error messages are human-readable.

---

### Step 5: Result Types for Search, Insert, Read

**File:** `crates/context-api/src/types.rs` (modify existing)

```pseudo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    /// Whether the full query was found as a single vertex
    pub complete: bool,
    /// The matched token (if complete)
    pub token: Option<TokenInfo>,
    /// Whether the entire query was consumed during search
    pub query_exhausted: bool,
    /// Partial match information (if incomplete)
    pub partial: Option<PartialMatchInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartialMatchInfo {
    /// How the query was partially matched
    pub kind: PartialMatchKind,
    /// The root token of the partial match path
    pub root_token: Option<TokenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartialMatchKind {
    /// Matched from the start (postfix remaining)
    Postfix,
    /// Matched from the end (prefix remaining)
    Prefix,
    /// Matched a range in the middle
    Range,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InsertResult {
    /// The token representing the inserted (or existing) pattern
    pub token: TokenInfo,
    /// Whether this was an existing vertex (true) or newly created (false)
    pub already_existed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PatternReadResult {
    /// The root vertex being read
    pub root: TokenInfo,
    /// The full text (concatenated leaf atoms)
    pub text: String,
    /// Recursive decomposition tree
    pub tree: ReadNode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReadNode {
    pub token: TokenInfo,
    /// Children (empty if this is an atom)
    pub children: Vec<ReadNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceCacheInfo {
    /// Number of vertices visited during the search
    pub vertex_count: usize,
    /// Summary of each cached vertex
    pub entries: Vec<TraceCacheEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceCacheEntry {
    pub token: TokenInfo,
    pub bottom_up_count: usize,
    pub top_down_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationReport {
    pub valid: bool,
    pub vertex_count: usize,
    pub issues: Vec<String>,
}
```

**Verification:** All types serialize/deserialize via serde_json.

---

### Step 6: Search Commands

**File:** `crates/context-api/src/commands/search.rs`

```pseudo
use context_search::Find;
use crate::resolve::resolve_token_refs;

impl WorkspaceManager {
    pub fn search_pattern(
        &self,
        ws_name: &str,
        query: Vec<TokenRef>,
    ) -> Result<SearchResult, SearchError> {
        if query.len() < 2 {
            return Err(SearchError::QueryTooShort);
        }

        let ws = self.get_workspace(ws_name)
            .map_err(|_| SearchError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_ref();

        // Resolve TokenRefs to Tokens
        let tokens = resolve_token_refs(graph, &query)?;

        // Execute search via Find::find_ancestor
        match graph.find_ancestor(&tokens) {
            Ok(response) => {
                Ok(build_search_result(graph, &response))
            }
            Err(reason) => {
                // ErrorReason::SingleIndex means the query resolved to a single
                // existing vertex — that IS a complete match
                Err(SearchError::InternalError(format!("{reason:?}")))
            }
        }
    }

    pub fn search_sequence(
        &self,
        ws_name: &str,
        text: &str,
    ) -> Result<SearchResult, SearchError> {
        if text.len() < 2 {
            return Err(SearchError::QueryTooShort);
        }

        let ws = self.get_workspace(ws_name)
            .map_err(|_| SearchError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_ref();

        match graph.find_sequence(text.chars()) {
            Ok(response) => {
                Ok(build_search_result(graph, &response))
            }
            Err(reason) => {
                Err(SearchError::InternalError(format!("{reason:?}")))
            }
        }
    }
}

/// Convert a context-search Response into our API SearchResult
fn build_search_result(
    graph: &HypergraphRef<BaseGraphKind>,
    response: &Response,
) -> SearchResult {
    if response.is_complete() {
        let path = response.as_complete().unwrap();
        let root = path.root_parent();
        SearchResult {
            complete: true,
            token: Some(TokenInfo::from_graph(graph, root)),
            query_exhausted: response.query_exhausted(),
            partial: None,
        }
    } else {
        // Build partial match info from the response's end state
        // The exact extraction depends on the PathEnum variant
        // (Postfix, Prefix, Range)
        SearchResult {
            complete: false,
            token: None,
            query_exhausted: response.query_exhausted(),
            partial: Some(build_partial_match_info(graph, response)),
        }
    }
}

fn build_partial_match_info(
    graph: &HypergraphRef<BaseGraphKind>,
    response: &Response,
) -> PartialMatchInfo {
    // Use response accessor methods to determine the partial match kind.
    // The exact implementation depends on the public API of Response/EndState.
    // Key methods: response.end.path (PathEnum), root_token(), etc.
    //
    // This is where we map context-search internals to our stable API types.
    // If the response accessors are insufficient, we may need to add
    // public helpers to context-search (minor change).
    PartialMatchInfo {
        kind: PartialMatchKind::Postfix, // determine from response
        root_token: None, // extract if available
    }
}
```

**Note:** The exact `Response` accessor methods need to be verified against the current `context-search` public API. The CHEAT_SHEET says to use `is_complete()`, `expect_complete()`, `root_token()`, `query_exhausted()`. If additional accessors are needed for partial match details, we may need to add them to `context-search` as a prerequisite.

**Verification:** Search for existing pattern → complete. Search for non-existent → incomplete/not-found. Search for partial → partial match info populated.

---

### Step 7: Insert Commands

**File:** `crates/context-api/src/commands/insert.rs`

```pseudo
use context_insert::ToInsertCtx;
use context_search::Searchable;
use crate::resolve::resolve_token_refs;

impl WorkspaceManager {
    pub fn insert_first_match(
        &mut self,
        ws_name: &str,
        query: Vec<TokenRef>,
    ) -> Result<InsertResult, InsertError> {
        if query.len() < 2 {
            return Err(InsertError::QueryTooShort);
        }

        let ws = self.get_workspace_mut(ws_name)
            .map_err(|_| InsertError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_ref();

        // Resolve TokenRefs to Tokens
        let tokens = resolve_token_refs(graph, &query)?;

        // First, search to see if it already exists
        let search_result = match graph.find_ancestor(&tokens) {
            Ok(response) if response.is_complete() => {
                let path = response.expect_complete("insert check");
                let root = path.root_parent();
                return Ok(InsertResult {
                    token: TokenInfo::from_graph(graph, root),
                    already_existed: true,
                });
            }
            Ok(_response) => None,  // partial match — proceed to insert
            Err(_) => None,         // not found — proceed to insert
        };

        // Insert via ToInsertCtx
        // HypergraphRef implements HasGraph, which provides insert_context()
        let result = graph.insert_or_get_complete(&tokens)
            .map_err(|e| InsertError::InternalError(format!("{e:?}")))?;

        Ok(InsertResult {
            token: TokenInfo::from_graph(graph, result),
            already_existed: false,
        })
    }

    pub fn insert_sequence(
        &mut self,
        ws_name: &str,
        text: &str,
    ) -> Result<InsertResult, InsertError> {
        if text.len() < 2 {
            return Err(InsertError::QueryTooShort);
        }

        let ws = self.get_workspace_mut(ws_name)
            .map_err(|_| InsertError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_ref();

        // Ensure all chars exist as atoms first
        for ch in text.chars() {
            graph.insert_atom(context_trace::Atom::Element(ch));
        }

        // Search first to check if it already exists
        match graph.find_sequence(text.chars()) {
            Ok(response) if response.is_complete() => {
                let path = response.expect_complete("insert_sequence check");
                let root = path.root_parent();
                return Ok(InsertResult {
                    token: TokenInfo::from_graph(graph, root),
                    already_existed: true,
                });
            }
            _ => {}
        }

        // Build token sequence from chars and insert
        let tokens: Vec<Token> = text.chars()
            .map(|ch| graph.get_atom_token_by_value(ch).unwrap())
            .collect();

        let result = graph.insert_or_get_complete(&tokens)
            .map_err(|e| InsertError::InternalError(format!("{e:?}")))?;

        Ok(InsertResult {
            token: TokenInfo::from_graph(graph, result),
            already_existed: false,
        })
    }

    pub fn insert_sequences(
        &mut self,
        ws_name: &str,
        texts: std::collections::HashSet<String>,
    ) -> Result<Vec<InsertResult>, InsertError> {
        // Insert each sequence independently (order doesn't matter — unordered set)
        let mut results = Vec::with_capacity(texts.len());
        for text in texts {
            results.push(self.insert_sequence(ws_name, &text)?);
        }
        Ok(results)
    }
}
```

**Important:** The insert operations mutate the graph via interior mutability (`DashMap`). The `Workspace.dirty` flag must be set. Since `insert_sequence` calls `graph_mut()` which sets dirty, this is handled. But since `HypergraphRef` uses interior mutability, even `&self` access can mutate the graph. We need to ensure `dirty` is set correctly — the `insert_*` methods should be on `&mut self` to ensure the dirty flag is set.

**Verification:**
- Insert new sequence → `already_existed: false`, token has correct label
- Insert existing sequence → `already_existed: true`, same token index
- Insert sequence with new chars → atoms auto-created
- Bulk insert_sequences → all results returned

---

### Step 8: Read Commands

**File:** `crates/context-api/src/commands/read.rs`

```pseudo
impl WorkspaceManager {
    pub fn read_pattern(
        &self,
        ws_name: &str,
        index: usize,
    ) -> Result<PatternReadResult, ReadError> {
        let ws = self.get_workspace(ws_name)
            .map_err(|_| ReadError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph();

        let vi = VertexIndex(index);
        let data = graph.try_get_vertex_data(vi)
            .ok_or(ReadError::VertexNotFound { index })?;

        let root_token = data.to_token();
        let root_info = TokenInfo::from_graph(graph, root_token);

        // Build recursive tree
        let tree = build_read_tree(graph, root_token);

        // Collect leaf text
        let text = collect_leaf_text(graph, root_token);

        Ok(PatternReadResult { root: root_info, text, tree })
    }

    pub fn read_as_text(
        &self,
        ws_name: &str,
        index: usize,
    ) -> Result<String, ReadError> {
        let ws = self.get_workspace(ws_name)
            .map_err(|_| ReadError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph();

        let vi = VertexIndex(index);
        let data = graph.try_get_vertex_data(vi)
            .ok_or(ReadError::VertexNotFound { index })?;

        Ok(collect_leaf_text(graph, data.to_token()))
    }
}

/// Recursively build a ReadNode tree by expanding child patterns.
///
/// For each vertex:
/// - If it's an atom → leaf node (no children)
/// - If it has child patterns → pick the first child pattern and recurse into each child token
///
/// Note: A vertex may have multiple child patterns (different decompositions).
/// For the read tree, we use the first pattern. A future enhancement could
/// expose all patterns or let the caller choose.
fn build_read_tree(graph: &Hypergraph<BaseGraphKind>, token: Token) -> ReadNode {
    let data = graph.expect_vertex_data_by_index(token.index);
    let token_info = TokenInfo::from_graph(graph, token);

    if data.is_atom() {
        return ReadNode { token: token_info, children: vec![] };
    }

    // Get the first child pattern (if any)
    let children = match data.first_child_pattern() {
        Some(pattern) => {
            pattern.iter()
                .map(|&child_token| build_read_tree(graph, child_token))
                .collect()
        }
        None => vec![],
    };

    ReadNode { token: token_info, children }
}

/// Collect leaf text by recursively traversing to atoms and concatenating their chars.
fn collect_leaf_text(graph: &Hypergraph<BaseGraphKind>, token: Token) -> String {
    let data = graph.expect_vertex_data_by_index(token.index);

    if data.is_atom() {
        // Get the atom's char value
        return graph.index_string(token.index);
    }

    // Recurse through first child pattern
    match data.first_child_pattern() {
        Some(pattern) => {
            pattern.iter()
                .map(|&child_token| collect_leaf_text(graph, child_token))
                .collect()
        }
        None => String::new(),
    }
}
```

**Note on context-read integration:** The above implementation does a straightforward recursive traversal using `context-trace` primitives, which is sufficient for Phase 2. Full `context-read` integration (with its expansion chains, cursors, and optimization) can be added later if needed. The recursive approach gives correct results and is simpler to implement.

**Verification:**
- Read an atom → tree with no children, text is single char
- Read a pattern → tree has children, text is concatenated chars
- Read a deep nested pattern → full recursive expansion
- Read nonexistent index → `ReadError::VertexNotFound`

---

### Step 9: Debug Commands (Trace Cache, Validate)

**File:** `crates/context-api/src/commands/debug.rs` (modify existing)

Add to the existing debug commands:

```pseudo
impl WorkspaceManager {
    // (existing: get_snapshot, get_statistics)

    pub fn get_trace_cache(
        &self,
        ws_name: &str,
        query: Vec<TokenRef>,
    ) -> Result<TraceCacheInfo, SearchError> {
        let ws = self.get_workspace(ws_name)
            .map_err(|_| SearchError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_ref();

        let tokens = resolve_token_refs(graph, &query)?;

        // Run a search to populate the trace cache
        let response = graph.find_ancestor(&tokens)
            .map_err(|e| SearchError::InternalError(format!("{e:?}")))?;

        // Extract cache info from the response
        let cache = &response.cache;
        let entries: Vec<TraceCacheEntry> = cache.entries.iter()
            .map(|(token, vertex_cache)| {
                TraceCacheEntry {
                    token: TokenInfo::from_graph(graph, *token),
                    bottom_up_count: vertex_cache.bottom_up.len(),
                    top_down_count: vertex_cache.top_down.len(),
                }
            })
            .collect();

        Ok(TraceCacheInfo {
            vertex_count: entries.len(),
            entries,
        })
    }

    pub fn validate_graph(
        &self,
        ws_name: &str,
    ) -> Result<ValidationReport, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        let graph = ws.graph();

        // Use existing graph validation if available
        // Otherwise, perform basic checks:
        let mut issues = Vec::new();
        let mut vertex_count = 0;

        for (_key, data) in graph.vertex_iter() {
            vertex_count += 1;

            // Check: all child pattern tokens reference existing vertices
            for (_, pattern) in data.child_patterns().iter() {
                for token in pattern.iter() {
                    if graph.try_get_vertex_data(token.index).is_none() {
                        issues.push(format!(
                            "Vertex {} has child token {} which does not exist",
                            data.vertex_index().0, token.index.0
                        ));
                    }
                }
            }

            // Check: token width matches sum of children widths
            if !data.is_atom() {
                for (_, pattern) in data.child_patterns().iter() {
                    let child_width_sum: usize = pattern.iter()
                        .map(|t| t.width.0)
                        .sum();
                    if child_width_sum != data.to_token().width.0 {
                        issues.push(format!(
                            "Vertex {} has width {} but children sum to {}",
                            data.vertex_index().0, data.to_token().width.0, child_width_sum
                        ));
                    }
                }
            }
        }

        Ok(ValidationReport {
            valid: issues.is_empty(),
            vertex_count,
            issues,
        })
    }
}
```

**Note:** The `response.cache` field access depends on `TraceCache` being publicly accessible on `Response`. The CHEAT_SHEET says "Response fields are private — use accessors". We may need to check if there's a public `cache()` accessor or if we need to add one. If the cache is truly private, we can skip the trace cache command for now and mark it as blocked.

**Verification:** Validate on a well-formed graph → `valid: true`. Validate after manual corruption → issues reported. Trace cache on a search → entries populated.

---

### Step 10: Update Command Enum and WorkspaceApi Trait

**File:** `crates/context-api/src/commands/mod.rs`

Add the new command variants:

```pseudo
// Add to WorkspaceApi trait:
fn search_pattern(&self, ws: &str, query: Vec<TokenRef>) -> Result<SearchResult, SearchError>;
fn search_sequence(&self, ws: &str, text: &str) -> Result<SearchResult, SearchError>;
fn insert_first_match(&mut self, ws: &str, query: Vec<TokenRef>) -> Result<InsertResult, InsertError>;
fn insert_sequence(&mut self, ws: &str, text: &str) -> Result<InsertResult, InsertError>;
fn insert_sequences(&mut self, ws: &str, texts: HashSet<String>) -> Result<Vec<InsertResult>, InsertError>;
fn read_pattern(&self, ws: &str, index: usize) -> Result<PatternReadResult, ReadError>;
fn read_as_text(&self, ws: &str, index: usize) -> Result<String, ReadError>;
fn get_trace_cache(&self, ws: &str, query: Vec<TokenRef>) -> Result<TraceCacheInfo, SearchError>;
fn validate_graph(&self, ws: &str) -> Result<ValidationReport, ApiError>;

// Add to Command enum:
SearchPattern { workspace: String, query: Vec<TokenRef> },
SearchSequence { workspace: String, text: String },
InsertFirstMatch { workspace: String, query: Vec<TokenRef> },
InsertSequence { workspace: String, text: String },
InsertSequences { workspace: String, texts: HashSet<String> },
ReadPattern { workspace: String, index: usize },
ReadAsText { workspace: String, index: usize },
GetTraceCache { workspace: String, query: Vec<TokenRef> },
ValidateGraph { workspace: String },

// Add to CommandResult enum:
SearchResult(SearchResult),
InsertResult(InsertResult),
InsertResultList(Vec<InsertResult>),
ReadResult(PatternReadResult),
Text(String),
TraceCacheInfo(TraceCacheInfo),
ValidationReport(ValidationReport),

// Add to execute() dispatch:
Command::SearchPattern { workspace, query } =>
    Ok(CommandResult::SearchResult(manager.search_pattern(&workspace, query)?)),
Command::SearchSequence { workspace, text } =>
    Ok(CommandResult::SearchResult(manager.search_sequence(&workspace, &text)?)),
Command::InsertFirstMatch { workspace, query } =>
    Ok(CommandResult::InsertResult(manager.insert_first_match(&workspace, query)?)),
Command::InsertSequence { workspace, text } =>
    Ok(CommandResult::InsertResult(manager.insert_sequence(&workspace, &text)?)),
Command::InsertSequences { workspace, texts } =>
    Ok(CommandResult::InsertResultList(manager.insert_sequences(&workspace, texts)?)),
Command::ReadPattern { workspace, index } =>
    Ok(CommandResult::ReadResult(manager.read_pattern(&workspace, index)?)),
Command::ReadAsText { workspace, index } =>
    Ok(CommandResult::Text(manager.read_as_text(&workspace, index)?)),
Command::GetTraceCache { workspace, query } =>
    Ok(CommandResult::TraceCacheInfo(manager.get_trace_cache(&workspace, query)?)),
Command::ValidateGraph { workspace } =>
    Ok(CommandResult::ValidationReport(manager.validate_graph(&workspace)?)),
```

**Verification:** JSON round-trip for all new `Command` variants. `execute()` dispatches correctly.

---

### Step 11: Update CLI Subcommands

**File:** `tools/context-cli/src/commands.rs`

Add new subcommands:

```pseudo
// Add to CliCommand enum:
/// Search for a token sequence in the graph
SearchPattern {
    workspace: String,
    /// Token references: numbers are indices, strings are labels
    query: Vec<String>,
},
/// Search for a text sequence (splits into chars)
SearchSequence { workspace: String, text: String },
/// Insert a token sequence (search + insert if not found)
InsertFirstMatch {
    workspace: String,
    query: Vec<String>,
},
/// Insert a text sequence (auto-creates atoms)
InsertSequence { workspace: String, text: String },
/// Read a vertex as a decomposition tree
ReadPattern { workspace: String, index: usize },
/// Read a vertex as concatenated text
ReadAsText { workspace: String, index: usize },
/// Validate graph integrity
Validate { workspace: String },
/// Inspect trace cache for a query
TraceCache {
    workspace: String,
    query: Vec<String>,
},
```

Add parser for TokenRef from CLI strings:

```pseudo
fn parse_token_ref(s: &str) -> TokenRef {
    // If it parses as a number → TokenRef::Index
    // Otherwise → TokenRef::Label
    match s.parse::<usize>() {
        Ok(n) => TokenRef::Index(n),
        Err(_) => TokenRef::Label(s.to_string()),
    }
}

fn parse_token_refs(strings: &[String]) -> Vec<TokenRef> {
    strings.iter().map(|s| parse_token_ref(s)).collect()
}
```

Wire into `execute_command()`.

---

### Step 12: Update CLI REPL

**File:** `tools/context-cli/src/repl.rs`

Add new REPL commands:

```pseudo
// In execute_repl_line match:
"search" => {
    // "search abc" → search_sequence
    // "search 0 1 2" → search_pattern (if all are numbers or mixed)
    // Simple heuristic: if single arg with no spaces, treat as sequence
    let arg = parts[1..].join(" ");
    if parts.len() == 2 && !parts[1].parse::<usize>().is_ok() {
        // Single string → search_sequence
        manager.search_sequence(ws, &arg)...
    } else {
        // Multiple args → search_pattern with TokenRefs
        let refs = parse_token_refs(&parts[1..]);
        manager.search_pattern(ws, refs)...
    }
}
"insert" => {
    // "insert hello world" → insert_sequence("hello world")
    let text = parts[1..].join(" ");
    manager.insert_sequence(ws, &text)...
}
"read" => {
    // "read 42" → read_pattern
    let index: usize = parts[1].parse()?;
    manager.read_pattern(ws, index)...
}
"text" => {
    // "text 42" → read_as_text
    let index: usize = parts[1].parse()?;
    manager.read_as_text(ws, index)...
}
"validate" => {
    manager.validate_graph(ws)...
}
"trace" => {
    // "trace abc" → get_trace_cache with sequence search
    let refs = parse_token_refs(&parts[1..]);
    manager.get_trace_cache(ws, refs)...
}
```

Update help text to include new commands.

---

### Step 13: Update CLI Output Formatting

**File:** `tools/context-cli/src/output.rs`

```pseudo
pub fn print_search_result(result: &SearchResult) {
    if result.complete {
        let token = result.token.as_ref().unwrap();
        println!("✓ Found: \"{}\" (index: {}, width: {})", token.label, token.index, token.width);
    } else if result.query_exhausted {
        println!("~ Partial match (query exhausted)");
        if let Some(partial) = &result.partial {
            println!("  Kind: {:?}", partial.kind);
            if let Some(root) = &partial.root_token {
                println!("  Root: \"{}\" (index: {})", root.label, root.index);
            }
        }
    } else {
        println!("✗ Not found");
    }
}

pub fn print_insert_result(result: &InsertResult) {
    if result.already_existed {
        println!("= Existing: \"{}\" (index: {}, width: {})",
            result.token.label, result.token.index, result.token.width);
    } else {
        println!("+ Inserted: \"{}\" (index: {}, width: {})",
            result.token.label, result.token.index, result.token.width);
    }
}

pub fn print_read_result(result: &PatternReadResult) {
    println!("Root: \"{}\" (index: {}, width: {})", result.root.label, result.root.index, result.root.width);
    println!("Text: \"{}\"", result.text);
    println!("Tree:");
    print_read_tree(&result.tree, 0);
}

fn print_read_tree(node: &ReadNode, depth: usize) {
    let indent = "  ".repeat(depth);
    if node.children.is_empty() {
        println!("{indent}'{label}' [{index}]", label = node.token.label, index = node.token.index);
    } else {
        println!("{indent}\"{label}\" [{index}] (width: {width})",
            label = node.token.label, index = node.token.index, width = node.token.width);
        for child in &node.children {
            print_read_tree(child, depth + 1);
        }
    }
}

pub fn print_validation_report(report: &ValidationReport) {
    if report.valid {
        println!("✓ Graph is valid ({} vertices)", report.vertex_count);
    } else {
        println!("✗ Graph has {} issue(s) ({} vertices):", report.issues.len(), report.vertex_count);
        for issue in &report.issues {
            println!("  - {issue}");
        }
    }
}

pub fn print_trace_cache(info: &TraceCacheInfo) {
    println!("Trace cache ({} vertices):", info.vertex_count);
    for entry in &info.entries {
        println!("  [{index}] \"{label}\" (width: {width}) — BU: {bu}, TD: {td}",
            index = entry.token.index,
            label = entry.token.label,
            width = entry.token.width,
            bu = entry.bottom_up_count,
            td = entry.top_down_count,
        );
    }
}
```

---

### Step 14: Tests

**File:** `crates/context-api/src/tests/search_tests.rs`

```pseudo
#[test]
fn search_existing_sequence() {
    // Create workspace, add atoms 'a','b','c', add_simple_pattern ['a','b']
    // search_sequence("ab") → complete, token matches the pattern
}

#[test]
fn search_nonexistent_sequence() {
    // Create workspace with atoms 'a','b'
    // search_sequence("cd") → QueryTooShort or not-found
}

#[test]
fn search_partial_match() {
    // Create workspace, insert "ab" as pattern
    // search_sequence("abc") → incomplete (postfix: 'c' unmatched)
}

#[test]
fn search_pattern_by_token_ref() {
    // Create workspace, add atoms, patterns
    // search_pattern([TokenRef::Index(0), TokenRef::Index(1)]) → result
}

#[test]
fn search_pattern_by_label() {
    // Create workspace with existing pattern "ab"
    // search_pattern([TokenRef::Label("ab".into()), TokenRef::Label("c".into())]) → result
}

#[test]
fn search_too_short() {
    // search_sequence("a") → QueryTooShort
    // search_pattern([single_ref]) → QueryTooShort
}
```

**File:** `crates/context-api/src/tests/insert_tests.rs`

```pseudo
#[test]
fn insert_new_sequence() {
    // Create workspace, add atoms 'a','b','c'
    // insert_sequence("abc") → already_existed: false
    // Verify: search_sequence("abc") → complete
}

#[test]
fn insert_existing_sequence() {
    // Create workspace, insert "ab"
    // insert_sequence("ab") → already_existed: true, same index
}

#[test]
fn insert_auto_creates_atoms() {
    // Create workspace (empty)
    // insert_sequence("hello") → succeeds, atoms h,e,l,o created
    // list_atoms → contains h, e, l, o
}

#[test]
fn insert_first_match_by_token_ref() {
    // Create workspace, add atoms
    // insert_first_match with TokenRef::Index values
}

#[test]
fn insert_sequences_bulk() {
    // Create workspace
    // insert_sequences({"abc", "def"}) → 2 results
    // Both searchable afterwards
}

#[test]
fn insert_preserves_graph_integrity() {
    // Insert multiple overlapping sequences
    // validate_graph → valid: true
}
```

**File:** `crates/context-api/src/tests/read_tests.rs`

```pseudo
#[test]
fn read_atom() {
    // Create workspace, add atom 'a'
    // read_pattern(a_index) → tree with no children, text "a"
}

#[test]
fn read_simple_pattern() {
    // Create workspace, add_simple_pattern ['a','b']
    // read_pattern(pattern_index) → tree with 2 children, text "ab"
}

#[test]
fn read_nested_pattern() {
    // Create workspace, insert_sequence("abcd")
    // read_pattern(abcd_index) → recursive tree, text "abcd"
}

#[test]
fn read_as_text() {
    // Create workspace, insert_sequence("hello")
    // read_as_text(hello_index) → "hello"
}

#[test]
fn read_nonexistent() {
    // read_pattern(99999) → VertexNotFound
}
```

**File:** `crates/context-api/src/tests/integration_tests.rs`

```pseudo
#[test]
fn full_round_trip_search_insert_read() {
    // 1. Create workspace
    // 2. insert_sequence("hello")
    // 3. search_sequence("hello") → complete
    // 4. read_as_text(hello_index) → "hello"
    // 5. read_pattern(hello_index) → tree with 5 atom children
    // 6. insert_sequence("hello world") → extends graph
    // 7. search_sequence("hello world") → complete
    // 8. read_as_text(hello_world_index) → "hello world"
    // 9. save → close → open → search_sequence("hello") → still complete
}

#[test]
fn command_json_round_trip_all_new_commands() {
    // Serialize each new Command variant to JSON, deserialize, execute
}

#[test]
fn validate_after_operations() {
    // Insert several sequences
    // validate_graph → valid: true
}
```

All tests use `tempfile::TempDir`.

**Verification:** `cargo test -p context-api` — all tests pass.

---

### Step 15: Final Verification

- [ ] `cargo check --workspace` — no errors
- [ ] `cargo test -p context-api` — all tests pass (Phase 1 + Phase 2)
- [ ] `cargo build -p context-cli` — binary builds
- [ ] Manual CLI test:
  ```
  context-cli create demo
  context-cli insert-sequence demo "hello world"
  context-cli search-sequence demo "hello"
  context-cli search-sequence demo "hello world"
  context-cli read-as-text demo <index>
  context-cli read-pattern demo <index>
  context-cli validate demo
  context-cli stats demo
  context-cli save demo
  ```
- [ ] Manual REPL test:
  ```
  > create demo
  > open demo
  > insert hello
  > insert world
  > search hello
  > read <index>
  > text <index>
  > validate
  > save
  > quit
  ```

---

## Prerequisites / Possible Context-* Additions

These are small additions that may be needed in upstream crates:

1. **`context-search::Response` cache access** — Verify that `response.cache` or a `response.cache()` accessor is public. If not, add `pub fn cache(&self) -> &TraceCache` to `Response`.

2. **`VertexData::first_child_pattern(&self) -> Option<&Pattern>`** — May already exist. Needed for the read tree traversal. If not, add a helper that returns the first pattern from `child_patterns()`.

3. **`Hypergraph::try_get_vertex_data(&self, idx: VertexIndex) -> Option<VertexData>`** — A non-panicking version of `expect_vertex_data`. May already exist as `get_vertex`.

4. **`Hypergraph::is_atom(&self, idx: VertexIndex) -> bool`** — Convenience method. May already exist on `VertexData`.

These are all < 5 lines each.

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `Response` fields are private, can't extract partial match details | Medium | Medium | Use only the public accessors documented in CHEAT_SHEET. If insufficient, add accessors to context-search (small change). |
| `context-read` public API is too limited for rich read results | Medium | Low | Phase 2 uses direct recursive traversal via context-trace primitives. Full context-read integration deferred. |
| `insert_or_get_complete` API doesn't match our expected usage | Low | Medium | Verify exact trait method signatures. May need `insert` instead of `insert_or_get_complete`. |
| `HypergraphRef` interior mutability conflicts with `Workspace.dirty` tracking | Medium | Medium | Ensure all mutating API paths go through `&mut self` methods that set `dirty = true`. |
| Token resolution via search creates recursive dependency (search needs graph, resolution uses search) | Low | Low | Resolution is a simple helper, not a full recursive call. Single-char labels bypass search entirely. |
| Large insert operations are slow | Low | Low | Not a Phase 2 concern. Profile if users report issues. |

## Notes

### Open Questions
- Should `insert_sequence` with a single char return the atom directly, or error with `QueryTooShort`? (Current plan: error, since single atoms should use `add_atom`)
- Should `read_pattern` show all child patterns or just the first? (Current plan: first only, with a note for future enhancement)
- Should `validate_graph` use context-trace's existing `validate_expansion` method if available?

### Deviations from Plan
*(To be filled during execution)*

### Lessons Learned
*(To be filled after execution)*