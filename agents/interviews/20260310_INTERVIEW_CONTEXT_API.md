---
tags: `#context-api` `#architecture` `#multi-phase` `#api-design`
summary: Overview and refinement interview for the context-api crate — unified, feature-gated access to context-* algorithms for multiple interface servers
status: 📋 Design
---

# Interview: context-api Crate — Unified Hypergraph API Layer

**Date:** 2026-03-10
**Feature:** Create `context-api` crate as the unified interface layer over context-{trace,search,insert,read}
**Status:** Design / Refinement

---

## 1. Project Overview

### 1.1 What We're Building

A new `crates/context-api` crate that serves as the **single public interface** for all hypergraph operations. Today, consumers must depend on `context-trace`, `context-search`, `context-insert`, and `context-read` directly, wiring together the correct trait calls, error handling, and type conversions themselves. `context-api` replaces that with:

- A **Workspace** model — named hypergraphs stored on disk in a user directory
- A **command-oriented API** — atomic operations like `add_atom`, `add_pattern`, `search_pattern`, `insert_first_match`, `read_pattern`
- **Feature-gated interface adapters** — the same core logic exposed as a CLI, HTTP server, MCP server, or future ACP server
- **Validation at the boundary** — patterns are validated before they reach the graph; invalid inputs produce clear errors
- A path toward an **interpreted instruction language** for composing hypergraph operations

### 1.2 Current Architecture (Before)

```
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  log-viewer  │  │  doc-viewer  │  │ dungeon-crawl│  ... future tools
│  (bin+MCP)   │  │  (bin+MCP)   │  │   (bin)      │
└──────┬───────┘  └──────┬───────┘  └──────┬───────┘
       │                 │                 │
       └─────── viewer-api (shared HTTP/MCP infra) ──┘
                         │
         (no unified graph API — each tool rolls its own)
                         │
   ┌─────────────────────┼─────────────────────┐
   │                     │                     │
context-read ──► context-insert ──► context-search ──► context-trace
```

### 1.3 Proposed Architecture (After)

```
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ CLI app  │ │ HTTP srv │ │ MCP srv  │ │ ACP srv  │ │ WASM lib │
│(feature) │ │(feature) │ │(feature) │ │(feature) │ │(feature) │
└────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘
     │            │            │            │            │
     └────────────┴────────────┴─────┬──────┴────────────┘
                                     │
                              ┌──────┴───────┐
                              │  context-api │  ← NEW: unified command API
                              │              │     workspace management
                              │  (core, no   │     validation layer
                              │   features)  │     ts-rs type exports
                              └──────┬───────┘
                                     │
   ┌─────────────────────────────────┼─────────────────────────┐
   │                                 │                         │
context-read ──────► context-insert ──────► context-search ──────► context-trace
```

### 1.4 What Moves / What Stays

| Item | Current Location | Proposed |
|------|-----------------|----------|
| `ts-rs` type exports (SnapshotNode, etc.) | `context-trace::graph::snapshot` | Stay, but `context-api` re-exports & adds API-level types |
| `GraphSnapshot`, `to_graph_snapshot()` | `context-trace::graph::snapshot` | Stay; `context-api` wraps via workspace commands |
| `Find` trait (`find_ancestor`, `find_sequence`) | `context-search::search` | Stay; `context-api` wraps as `search_pattern` command |
| `ToInsertCtx` trait (`insert`, `insert_or_get_complete`) | `context-insert::insert` | Stay; `context-api` wraps as `insert_first_match` command |
| `HypergraphRef` (Arc<RwLock<Hypergraph>>) | `context-trace::graph` | Stay; `context-api::Workspace` owns one per graph |
| Visualization types | `context-trace::graph::visualization` | Stay; `context-api` optionally re-exports for frontends |
| Viewer-api HTTP/MCP infra | `tools/viewer-api` | Can be used *by* feature-gated adapters inside `context-api` |
| Log-viewer / Doc-viewer | `tools/` | Unchanged — they remain separate tools |

### 1.5 Key Design Decisions to Make

1. **Workspace storage format** — How are hypergraphs persisted on disk?
2. **Command model** — Enum-based? Trait-based? Both?
3. **Error model** — Unified error type across all operations?
4. **Feature gate granularity** — One feature per server type? Finer?
5. **Validation layer** — What constitutes a "valid pattern"?
6. **Instruction language** — Scope for phase 1 vs. future?
7. **ts-rs strategy** — Generate from `context-api` types? Re-export from `context-trace`?
8. **Async vs. sync** — Core API sync, adapters async? Or async all the way?

---

## 2. Existing Infrastructure Analysis

### 2.1 ts-rs Types Already Exported

Currently in `context-trace` (exported to `tools/log-viewer/frontend/src/types/generated/`):

| Type | File | Purpose |
|------|------|---------|
| `GraphSnapshot` | `graph/snapshot.rs` | Full graph topology (nodes + edges) |
| `SnapshotNode` | `graph/snapshot.rs` | Vertex: index, label, width |
| `SnapshotEdge` | `graph/snapshot.rs` | Edge: from, to, pattern_idx, sub_index |
| `EdgeRef` | `graph/search_path.rs` | Edge reference for search path viz |
| `PathNode` | `graph/search_path.rs` | Node in search path |
| `PathTransition` | `graph/search_path.rs` | Step in path construction |
| `VizPathGraph` | `graph/search_path.rs` | Complete search path graph |
| `OperationType` | `graph/visualization.rs` | Search/Insert/Read/Query enum |
| `Transition` | `graph/visualization.rs` | Algorithm step animation frame |
| `LocationInfo` | `graph/visualization.rs` | Node highlighting state |
| `QueryInfo` | `graph/visualization.rs` | Search/insert query metadata |
| `GraphOpEvent` | `graph/visualization.rs` | Mutation events (add/remove/merge) |
| `GraphDiff` | `graph/visualization.rs` | Before/after graph state |
| `VizEvent` | `graph/visualization.rs` | Top-level visualization event |

Additionally in `tools/log-viewer`:

| Type | File | Purpose |
|------|------|---------|
| `LogEntry` | `log_parser.rs` | Parsed log line |
| `AssertionDiff` | `log_parser.rs` | Test assertion diff |
| `LogFileInfo`, `LogContentResponse`, etc. | `types.rs` | HTTP API types |

### 2.2 Serialization Status of Core Graph Types

`Hypergraph<G>` already derives `Serialize` and `Deserialize` (via `serde`). It uses `DashMap` internally (which supports serde). This means **we can persist an entire graph to JSON/bincode/messagepack** today — no structural changes needed.

Key types and their serde status:
- `Hypergraph<G>`: ✅ Serialize + Deserialize
- `BaseGraphKind`: ✅ Serialize + Deserialize
- `Token`: ✅ Serialize (via serde derive)
- `VertexData`: ✅ Serialize (via serde derive)
- `PatternId` (UUID): ✅ Serialize
- `TraceCache`: needs verification — may need serde derives added
- `Response` (search result): needs verification

### 2.3 Existing MCP Server Pattern

Both `log-viewer` and `doc-viewer` follow the same MCP pattern using `rmcp`:

```
struct MyServer {
    // app state
    tool_router: ToolRouter<Self>,
}

#[tool(/* ... */)]
impl MyServer {
    async fn my_tool(&self, input: MyInput) -> Result<CallToolResult, McpError> { ... }
}

impl ServerHandler for MyServer {
    fn get_info(&self) -> ServerInfo { ... }
}

// Entry point
async fn run_mcp_server(state: AppState) -> Result<()> {
    let server = MyServer::new(state);
    let transport = stdio::StdioTransport::new();
    server.serve(transport).await
}
```

This pattern is well-established and can be directly reused for a `context-api` MCP adapter.

### 2.4 Existing HTTP Server Pattern

`viewer-api` provides `ServerConfig`, `run_server`, CORS, static file serving, dev proxy. The pattern is:

```rust
fn create_routes(state: AppState, static_dir: Option<PathBuf>) -> Router { ... }

run_server(config, state, create_routes, Some(mcp_factory)).await
```

---

## 3. Proposed Command Model (Pseudo-code)

### 3.1 Core Types

```pseudo
// A workspace is a named, persisted hypergraph
struct Workspace {
    name: String,
    path: PathBuf,
    graph: HypergraphRef,           // Arc<RwLock<Hypergraph<BaseGraphKind>>>
    metadata: WorkspaceMetadata,    // created_at, modified_at, description, etc.
}

struct WorkspaceManager {
    base_dir: PathBuf,              // e.g. ~/.context-engine/workspaces/
    open_workspaces: HashMap<String, Workspace>,
}

// Result types for API consumers
struct AtomInfo { index: usize, label: String, width: 1 }
struct PatternInfo { index: usize, label: String, width: usize, children: Vec<TokenInfo> }
struct TokenInfo { index: usize, label: String, width: usize }
struct SearchResult { found: bool, complete: bool, token: Option<TokenInfo>, partial: Option<PartialMatch> }
struct SnapshotInfo = GraphSnapshot  // re-export existing type
```

### 3.2 Workspace Management Commands

```pseudo
// Lifecycle
create_workspace(name, description?) -> WorkspaceInfo
list_workspaces() -> Vec<WorkspaceInfo>
open_workspace(name) -> WorkspaceInfo
close_workspace(name) -> ()
delete_workspace(name) -> ()
export_workspace(name, format: json|bincode) -> bytes
import_workspace(name, data, format) -> WorkspaceInfo

// Metadata
get_workspace_info(name) -> WorkspaceInfo { name, vertex_count, edge_count, created, modified }
```

### 3.3 Basic Graph Commands

```pseudo
// Atoms
add_atom(workspace, label: String) -> AtomInfo
    // Idempotent: if atom with this label exists, return existing

get_atom(workspace, label: String) -> Option<AtomInfo>

list_atoms(workspace) -> Vec<AtomInfo>

// Patterns (validated!)
add_pattern(workspace, tokens: Vec<TokenRef>) -> Result<PatternInfo, ValidationError>
    // TokenRef = either { atom: "label" } or { index: usize }
    // Validates:
    //   - All referenced tokens exist in the graph
    //   - Pattern has at least 2 elements
    //   - Elements are valid vertex indices
    // Uses context-insert under the hood

get_vertex(workspace, index: usize) -> Option<VertexInfo>
    // Returns atom or pattern info with children and parents

list_vertices(workspace, filter?: { min_width?, max_width?, has_label? }) -> Vec<TokenInfo>
```

### 3.4 Algorithm Commands

```pseudo
// Search
search_pattern(workspace, query: Vec<TokenRef>) -> SearchResult
    // Wraps Find::find_ancestor
    // Returns complete match, partial match info, or not-found

search_sequence(workspace, text: String) -> SearchResult
    // Wraps Find::find_sequence (atomizes the string first)

// Insert
insert_first_match(workspace, query: Vec<TokenRef>) -> InsertResult
    // Searches for the pattern; if not found as a single vertex, inserts it
    // Returns the resulting vertex (new or existing)

insert_sequence(workspace, text: String) -> InsertResult
    // Atomizes string, then inserts the full sequence

// Read
read_pattern(workspace, index: usize) -> PatternReadResult
    // Returns the full recursive decomposition of a vertex
    // Uses context-read expansion

read_as_text(workspace, index: usize) -> String
    // Reads vertex as concatenated atom labels (leaf traversal)
```

### 3.5 Developer / Debug Commands

```pseudo
// Snapshot
get_snapshot(workspace) -> GraphSnapshot
    // Returns full graph topology (existing to_graph_snapshot)

// Trace cache inspection
get_trace_cache(workspace, query: Vec<TokenRef>) -> TraceCacheInfo
    // Runs a search and returns the trace cache contents

// Validation
validate_graph(workspace) -> ValidationReport
    // Runs graph invariant checks

// Graph statistics
get_statistics(workspace) -> GraphStatistics
    // vertex_count, edge_count, max_depth, atom_count, pattern_count, etc.
```

### 3.6 Future: Instruction Language (Phase N)

```pseudo
// An interpreted mini-language for composing operations
execute(workspace, program: String) -> ExecutionResult

// Example programs:
//   "let x = atom 'hello'; let y = atom 'world'; pattern [x, y]"
//   "search 'hello world' | if not_found then insert"
//   "read vertex 42 | depth 3"
//   "for v in vertices where width > 5 { print read(v) }"
```

---

## 4. Feature Gate Design

```toml
[features]
default = []

# Interface adapters (each pulls in its own deps)
cli     = ["clap"]
http    = ["axum", "tokio", "tower-http"]
mcp     = ["rmcp"]
acp     = []  # future
wasm    = ["wasm-bindgen"]

# Output format support
ts-gen  = ["ts-rs"]           # TypeScript type generation

# Dev/debug features
dev     = []                  # Extra debug commands, trace cache inspection
```

### Feature → Dependency Mapping

| Feature | Extra deps | What it enables |
|---------|-----------|-----------------|
| (none) | context-trace, context-search, context-insert, context-read, serde, serde_json | Core API, Workspace, all commands |
| `cli` | clap | `fn main()` with subcommand CLI |
| `http` | axum, tokio, tower-http, (viewer-api?) | REST API server over workspace commands |
| `mcp` | rmcp | MCP server (stdio transport) exposing commands as tools |
| `ts-gen` | ts-rs | `#[derive(TS)]` on all public API types |
| `dev` | (none) | Extra commands: trace cache, split graph inspection |

---

## 5. Refinement Questions

### Batch 1: Workspace & Persistence

#### Q1. Workspace Storage Location
Where should workspaces be stored by default?

- [ ] A. `~/.context-engine/workspaces/<name>/` (XDG-style, per-user)
- [ ] B. `./.context-engine/` (project-local, like `.git`)
- [ ] C. Configurable via env var `CONTEXT_ENGINE_HOME`, defaulting to A
- [ ] D. Other: ___

**Answer:**

---

#### Q2. Persistence Format
What serialization format for on-disk hypergraphs?

- [ ] A. **JSON** — human-readable, easy to debug, larger files
- [ ] B. **Bincode** — fast, compact, not human-readable
- [ ] C. **Both** — JSON as default, bincode as opt-in for large graphs
- [ ] D. **MessagePack** — compact + somewhat readable
- [ ] E. **SQLite** — structured, queryable, supports incremental updates
- [ ] F. Other: ___

**Answer:**

---

#### Q3. Auto-save Behavior
Should workspaces auto-persist after each mutation?

- [ ] A. **Auto-save** every mutation (safe, slower for bulk operations)
- [ ] B. **Explicit save** — user calls `save_workspace(name)` when ready
- [ ] C. **Hybrid** — auto-save with configurable debounce interval (e.g. 5s)
- [ ] D. **WAL-style** — append operations to a log, compact on close
- [ ] E. Other: ___

**Answer:**

---

#### Q4. Concurrent Access
Should multiple processes be able to open the same workspace?

- [ ] A. **Single-writer** — file lock, one process at a time
- [ ] B. **Multi-reader, single-writer** — read-only access for others
- [ ] C. **Not initially** — just document the limitation, solve later
- [ ] D. Other: ___

**Answer:**

---

### Batch 2: API Design

#### Q5. Command Dispatch Model
How should the core API be structured?

- [ ] A. **Trait-based** — `trait WorkspaceApi { fn add_atom(...); fn search_pattern(...); }` implemented by `WorkspaceManager`
- [ ] B. **Enum-based** — `enum Command { AddAtom { label }, SearchPattern { query }, ... }` dispatched by a single `execute(cmd)` function
- [ ] C. **Both** — trait for Rust consumers, enum for serialized interfaces (CLI/MCP/HTTP)
- [ ] D. **Method-based** — just `impl Workspace { pub fn add_atom(...) }`, no trait abstraction
- [ ] E. Other: ___

**Answer:**

---

#### Q6. Error Strategy
How should errors be surfaced?

- [ ] A. **Single `ApiError` enum** — all commands return `Result<T, ApiError>` with variants for each failure mode
- [ ] B. **Per-command error types** — `AddAtomError`, `SearchError`, etc.
- [ ] C. **`thiserror` + `anyhow`** — structured errors internally, `anyhow` at boundaries
- [ ] D. Other: ___

**Answer:**

---

#### Q7. Atom Identity
When adding an atom, what makes it unique?

- [ ] A. **Label string** — `add_atom("hello")` twice returns the same vertex (idempotent by label)
- [ ] B. **Always new** — each `add_atom("hello")` creates a distinct vertex
- [ ] C. **Configurable** — `add_atom("hello", deduplicate=true|false)`
- [ ] D. **Follow current behavior** — whatever `graph.insert_atom()` does today (currently: deduplicates by `Atom` value via `atom_keys` reverse lookup)
- [ ] E. Other: ___

**Answer:**

---

#### Q8. TokenRef Format for Patterns
How should API callers reference tokens when building patterns?

- [ ] A. **By index only** — `[0, 1, 2]` (callers must know indices)
- [ ] B. **By label only** — `["hello", "world"]` (looked up as atoms)
- [ ] C. **Union type** — `{ "atom": "hello" }` or `{ "index": 42 }` or `{ "pattern": [0, 1] }`
- [ ] D. **String DSL** — `"hello world"` parsed into atoms, `"[hello, world]"` as explicit pattern
- [ ] E. Other: ___

**Answer:**

---

### Batch 3: Interface Adapters

#### Q9. CLI Scope (Phase 1)
What should the CLI adapter support initially?

- [ ] A. **Full command set** — all workspace + graph + algorithm commands from day 1
- [ ] B. **Minimal viable** — create/open/list workspaces, add atoms, add patterns, search, snapshot
- [ ] C. **Interactive REPL** — a shell where you type commands against an open workspace
- [ ] D. **B + C** — subcommand CLI for scripting, plus an interactive REPL mode
- [ ] E. Other: ___

**Answer:**

---

#### Q10. HTTP API Style
What style of HTTP API?

- [ ] A. **REST** — `POST /workspaces/{name}/atoms`, `GET /workspaces/{name}/vertices/{id}`, etc.
- [ ] B. **RPC-style** — `POST /api/execute` with command JSON body
- [ ] C. **GraphQL** — single endpoint, typed schema
- [ ] D. **A + B** — REST for simple ops, RPC for complex algorithm commands
- [ ] E. Other: ___

**Answer:**

---

#### Q11. MCP Tool Granularity
How should commands map to MCP tools?

- [ ] A. **One tool per command** — `add_atom`, `search_pattern`, `insert_first_match`, etc. (~15 tools)
- [ ] B. **Grouped CRUD** — like doc-viewer: `list`, `search`, `create`, `update`, `delete`, `execute` (~6 tools)
- [ ] C. **Single `execute` tool** — one tool that takes a command enum as input
- [ ] D. **A for common, C for advanced** — individual tools for frequent ops, `execute` for the rest
- [ ] E. Other: ___

**Answer:**

---

#### Q12. Should context-api Produce Binaries?
Should `context-api` contain `[[bin]]` targets, or should binaries be separate crates?

- [ ] A. **Binaries in context-api** — `context-api --cli`, `context-api --http`, `context-api --mcp` (feature-gated bins)
- [ ] B. **Separate thin bin crates** — `tools/context-cli`, `tools/context-server`, etc. that depend on `context-api`
- [ ] C. **One binary, multiple modes** — single `context-engine` binary, mode selected by flag (like viewer-api pattern)
- [ ] D. Other: ___

**Answer:**

---

### Batch 4: Validation & Semantics

#### Q13. Pattern Validation Rules
What should `add_pattern` validate beyond "tokens exist"?

- [ ] A. **Minimum: tokens exist + length ≥ 2** (matching current `insert_pattern` requirements)
- [ ] B. **A + no duplicate patterns** — reject if an identical pattern already exists (return existing instead)
- [ ] C. **A + reachability invariant check** — ensure the pattern doesn't violate substring containment rules
- [ ] D. **A + B + width consistency** — verify computed width matches sum of children
- [ ] E. Other: ___

**Answer:**

---

#### Q14. "Insert First Match" Semantics
What should happen when the search finds a partial match?

- [ ] A. **Always insert** — insert the full query pattern, splitting existing patterns as needed (current context-insert behavior)
- [ ] B. **Fail on partial** — only insert if completely not found; partial match returns an error
- [ ] C. **Configurable** — `insert_first_match(query, on_partial: Insert | Fail | ReturnPartial)`
- [ ] D. **A as default, expose option** — default is current insert behavior, but offer the choice
- [ ] E. Other: ___

**Answer:**

---

#### Q15. Read Depth Control
When reading a pattern recursively, how deep should we go?

- [ ] A. **Always full depth** — expand all the way to atoms
- [ ] B. **Configurable depth** — `read_pattern(index, depth: Option<usize>)`
- [ ] C. **Two modes** — `read_shallow(index)` (one level) and `read_deep(index)` (full)
- [ ] D. **B with default=full**
- [ ] E. Other: ___

**Answer:**

---

### Batch 5: Type Generation & Interop

#### Q16. ts-rs Export Strategy
How should TypeScript types be generated?

- [ ] A. **From context-api only** — all API-level types get `#[derive(TS)]`, re-exporting relevant context-trace types
- [ ] B. **From each crate** — keep existing context-trace exports, add context-api exports alongside
- [ ] C. **Centralized in context-api** — move all TS generation here, remove from context-trace
- [ ] D. **A for API types, B for visualization types** — API types from context-api, viz types stay in context-trace
- [ ] E. Other: ___

**Answer:**

---

#### Q17. ts-rs Export Target
Where should generated `.ts` files go?

- [ ] A. **A new `context-api/generated/ts/` directory** — consumers copy from there
- [ ] B. **Keep current pattern** — export to `tools/log-viewer/frontend/src/types/generated/` (and add more targets)
- [ ] C. **A dedicated `types/` workspace member** — a standalone package consumers can `npm install`
- [ ] D. **Configurable at build time** — `TS_EXPORT_DIR` env var
- [ ] E. Other: ___

**Answer:**

---

### Batch 6: Scope & Phasing

#### Q18. Phase 1 Scope
What should be in the first deliverable?

- [ ] A. **Core only** — `Workspace`, `WorkspaceManager`, all graph commands, no interface adapters yet
- [ ] B. **Core + CLI** — A plus a basic CLI for manual testing
- [ ] C. **Core + MCP** — A plus MCP server (most useful for agent workflows)
- [ ] D. **Core + CLI + MCP** — the minimum viable product for both human and agent use
- [ ] E. Other: ___

**Answer:**

---

#### Q19. Instruction Language Priority
When should the interpreted instruction language be tackled?

- [ ] A. **Phase 1** — start with a minimal DSL (`let`, `atom`, `pattern`, `search`, `if`)
- [ ] B. **Phase 2** — after core API and at least one adapter are stable
- [ ] C. **Phase 3+** — nice-to-have, focus on API stability first
- [ ] D. **Design now, implement later** — write the grammar spec in phase 1, implement in phase 2
- [ ] E. Other: ___

**Answer:**

---

#### Q20. Relationship to Existing Viewer Tools
How should `log-viewer` and `doc-viewer` relate to `context-api`?

- [ ] A. **Independent** — they stay as-is, `context-api` is a separate concern
- [ ] B. **Gradual migration** — viewers eventually use `context-api` for any graph operations they need
- [ ] C. **Shared infra** — `context-api` http/mcp features reuse `viewer-api` infrastructure
- [ ] D. **B + C** — viewers adopt `context-api` for graph ops, `context-api` reuses `viewer-api` for serving
- [ ] E. Other: ___

**Answer:**

---

### Batch 7: Advanced / Edge Cases

#### Q21. Graph Naming & Namespacing
Can multiple workspaces reference each other?

- [ ] A. **No** — each workspace is fully isolated
- [ ] B. **Import/link** — a workspace can import vertices from another (copy-on-read)
- [ ] C. **Future consideration** — design for isolation now, add linking later
- [ ] D. Other: ___

**Answer:**

---

#### Q22. Undo / History
Should the API support undo?

- [ ] A. **No** — hypergraph operations are append-only by nature (patterns are never deleted)
- [ ] B. **Operation log** — record every command, allow replay/rollback
- [ ] C. **Snapshots** — manual save points, restore to snapshot
- [ ] D. **C as a future feature** — not phase 1
- [ ] E. Other: ___

**Answer:**

---

#### Q23. Bulk Operations
Should there be batch/bulk commands?

- [ ] A. **Yes from the start** — `add_atoms(["a","b","c"])`, `insert_sequences(["hello","world"])`
- [ ] B. **Not initially** — single operations first, batch as optimization later
- [ ] C. **Via instruction language** — bulk ops are just programs
- [ ] D. **A for atoms/patterns, B for algorithms** — bulk creation is easy, bulk search is complex
- [ ] E. Other: ___

**Answer:**

---

#### Q24. Streaming / Subscription
Should the API support event streams?

- [ ] A. **No** — request/response only
- [ ] B. **HTTP SSE** — subscribe to graph mutation events
- [ ] C. **Callback hooks** — `on_vertex_added`, `on_pattern_inserted`
- [ ] D. **Future consideration** — design the event model now, implement later
- [ ] E. Other: ___

**Answer:**

---

#### Q25. What Should "context-api" Be Named?
Final crate name?

- [ ] A. **`context-api`** — clear and consistent with `context-{trace,search,insert,read}`
- [ ] B. **`context-engine`** — matches the repo name, signals "this is the top-level"
- [ ] C. **`context-workspace`** — emphasizes the workspace model
- [ ] D. **`context-hub`** — emphasizes the "central access point" role
- [ ] E. Other: ___

**Answer:**

---

## 6. Preliminary Phase Breakdown

*To be refined after interview answers*

### Phase 1 — Foundation
- Create `crates/context-api` with `Cargo.toml` (core deps only)
- `Workspace` struct wrapping `HypergraphRef`
- `WorkspaceManager` with create/open/close/list/delete
- Persistence (save/load hypergraph to disk)
- Basic graph commands: `add_atom`, `add_pattern` (with validation), `get_vertex`, `list_vertices`
- `get_snapshot`, `get_statistics`
- Unified error type
- Unit tests for all commands

### Phase 2 — Algorithms
- `search_pattern`, `search_sequence` (wrapping `Find`)
- `insert_first_match`, `insert_sequence` (wrapping `ToInsertCtx`)
- `read_pattern`, `read_as_text` (wrapping `context-read`)
- Developer commands: `get_trace_cache`, `validate_graph`
- Integration tests: search → insert → read round-trips

### Phase 3 — CLI Adapter
- Feature-gated `cli` module with `clap` subcommands
- Subcommands for all workspace + graph + algorithm commands
- Interactive REPL mode (if chosen in Q9)
- `[[bin]]` target or separate thin crate (depending on Q12)
- Human-friendly output formatting (tables, colored diffs)

### Phase 4 — MCP Adapter
- Feature-gated `mcp` module using `rmcp` (same pattern as log-viewer/doc-viewer)
- MCP tools mapped from command set (granularity per Q11)
- Stdio transport for agent integration
- End-to-end test: agent workflow creating a workspace, inserting data, querying

### Phase 5 — HTTP Adapter
- Feature-gated `http` module using `axum` (reusing `viewer-api` patterns)
- REST or RPC endpoints (per Q10)
- CORS, static file serving for potential web frontends
- Optional WebSocket/SSE for streaming events (per Q24)

### Phase 6 — Advanced & Future
- `ts-rs` type generation for all API types (feature-gated)
- Export/import workspace commands
- Instruction language design & implementation (per Q19)
- ACP adapter (when spec stabilizes)
- WASM target for browser-embedded use
- Cross-workspace linking (per Q21)
- Bulk operation optimizations (per Q23)

---

## 7. Next Steps

1. **Answer the 25 questions above** — prioritize Batches 1-3 (Workspace, API Design, Interface Adapters) as they unblock Phase 1 implementation
2. **Create plan files** — after answers are collected, generate:
   - `agents/plans/20260310_PLAN_CONTEXT_API_OVERVIEW.md` — master plan with all phases
   - `agents/plans/20260310_PLAN_CONTEXT_API_PHASE1.md` — detailed Phase 1 execution steps
   - `agents/plans/20260310_PLAN_CONTEXT_API_PHASE2.md` — detailed Phase 2 execution steps
   - Additional phase plans as needed
3. **Prototype `Cargo.toml`** — validate dependency graph compiles before full implementation
4. **Implement Phase 1** — in a fresh execution session with the plan loaded

---

## Appendix A: Dependency Graph After context-api

```
context-api
├── context-read
│   ├── context-insert
│   │   ├── context-search
│   │   │   └── context-trace
│   │   └── context-trace
│   ├── context-search
│   │   └── context-trace
│   └── context-trace
├── serde + serde_json          (always)
├── clap                        (feature: cli)
├── axum + tokio + tower-http   (feature: http)
├── rmcp                        (feature: mcp)
├── ts-rs                       (feature: ts-gen)
└── wasm-bindgen                (feature: wasm)
```

## Appendix B: Existing Public Traits the API Will Wrap

| Trait | Crate | Key Methods | context-api Command |
|-------|-------|-------------|-------------------|
| `Find` | context-search | `find_ancestor(query)`, `find_sequence(chars)` | `search_pattern`, `search_sequence` |
| `Searchable` | context-search | `search(trav)` → `Response` | (internal, used by Find) |
| `ToInsertCtx` | context-insert | `insert(searchable)`, `insert_or_get_complete(searchable)` | `insert_first_match` |
| `Hypergraph::insert_atom` | context-trace | `insert_atom(atom)` → `Token` | `add_atom` |
| `Hypergraph::insert_pattern` | context-trace | `insert_pattern(tokens)` → `Token` | `add_pattern` |
| `Hypergraph::to_graph_snapshot` | context-trace | `to_graph_snapshot()` → `GraphSnapshot` | `get_snapshot` |
| `Hypergraph::vertex_data` | context-trace | `vertex_data(idx)` → `&VertexData` | `get_vertex` |

## Appendix C: File Structure Sketch

```
crates/context-api/
├── Cargo.toml
├── src/
│   ├── lib.rs                  # Public re-exports, feature gates
│   ├── error.rs                # Unified ApiError type
│   ├── types.rs                # API-level types (AtomInfo, PatternInfo, SearchResult, etc.)
│   ├── workspace/
│   │   ├── mod.rs              # Workspace struct
│   │   ├── manager.rs          # WorkspaceManager (create/open/close/list/delete)
│   │   ├── persistence.rs      # Save/load to disk
│   │   └── metadata.rs         # WorkspaceMetadata, timestamps, description
│   ├── commands/
│   │   ├── mod.rs              # Command enum (if chosen), dispatch
│   │   ├── atoms.rs            # add_atom, get_atom, list_atoms
│   │   ├── patterns.rs         # add_pattern (with validation), get_vertex, list_vertices
│   │   ├── search.rs           # search_pattern, search_sequence
│   │   ├── insert.rs           # insert_first_match, insert_sequence
│   │   ├── read.rs             # read_pattern, read_as_text
│   │   └── debug.rs            # get_snapshot, get_statistics, validate_graph, get_trace_cache
│   ├── validation.rs           # Pattern validation logic
│   ├── adapters/
│   │   ├── mod.rs              # Adapter trait / shared utilities
│   │   ├── cli.rs              # #[cfg(feature = "cli")] — clap subcommands
│   │   ├── http.rs             # #[cfg(feature = "http")] — axum routes
│   │   └── mcp.rs              # #[cfg(feature = "mcp")] — rmcp server
│   └── tests/
│       ├── workspace_tests.rs
│       ├── command_tests.rs
│       └── integration_tests.rs
├── README.md
└── HIGH_LEVEL_GUIDE.md
```