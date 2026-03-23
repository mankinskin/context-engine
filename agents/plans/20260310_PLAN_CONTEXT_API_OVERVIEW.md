---
tags: `#context-api` `#architecture` `#multi-phase` `#api-design` `#plan`
summary: Master plan for context-api crate — unified, feature-gated hypergraph workspace API with CLI, MCP, HTTP adapters
status: 📋
---

# Plan: context-api — Master Overview

## Objective

Create `crates/context-api`, a new library crate that provides the single unified public interface for all hypergraph operations across the context-engine workspace. It wraps `context-trace`, `context-search`, `context-insert`, and `context-read` behind a workspace-oriented, command-based API with feature-gated adapters for CLI, MCP, HTTP, and future protocols.

## Context

### Interview Reference

All design decisions are captured in:
`agents/interviews/20260310_INTERVIEW_CONTEXT_API.md` (25 questions, all answered)

### Key Design Decisions (Summary)

| Decision | Choice |
|----------|--------|
| Storage | `./.context-engine/` (project-local) |
| Persistence | Bincode, full graph in memory, explicit commit |
| Concurrency | Multi-reader, single-writer |
| API model | Trait for Rust + Command enum for serialized interfaces |
| Errors | Per-command types composed into larger enums |
| Atoms | Single char, deduplicated by char value |
| TokenRef | By numeric index or string label |
| Validation | `add_simple_pattern` — atoms only, no atom reuse in existing patterns |
| Insert semantics | Always insert (split as needed) |
| Read depth | Always full (algorithm-determined) |
| CLI | Full command set + REPL (Phase 1) |
| MCP | Single `execute` tool with command enum |
| HTTP | RPC + GraphQL |
| Binaries | Separate thin crates in `tools/` |
| ts-rs | Centralized in context-api, exported to dedicated npm package |
| Crate name | `context-api` |
| Bulk ops | Yes, unordered sets |
| Undo | Snapshot via explicit commit (reload from disk) |
| Instruction language | Phase 3+ (nice-to-have) |
| Viewer tools | Gradual migration to context-api |

### Architecture After

```
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ tools/       │  │ tools/       │  │ tools/       │  │ (future)     │
│ context-cli  │  │ context-mcp  │  │ context-http │  │ context-acp  │
│ (bin)        │  │ (bin)        │  │ (bin)        │  │ (bin)        │
└──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘
       │                 │                 │                 │
       └─────────────────┴────────┬────────┴─────────────────┘
                                  │
                           ┌──────┴───────┐
                           │  context-api │  (library crate)
                           │              │  - WorkspaceApi trait
                           │  crates/     │  - Command enum
                           │  context-api │  - Workspace + Manager
                           └──────┬───────┘  - Validation layer
                                  │
          ┌───────────────────────┼───────────────────────┐
          │                       │                       │
   context-read ──► context-insert ──► context-search ──► context-trace
```

## Phase Plan

### Phase 1 — Foundation + CLI
**Plan file:** `20260310_PLAN_CONTEXT_API_PHASE1.md`
**Scope:**
- Create `crates/context-api` crate skeleton with `Cargo.toml`
- Unified error types (per-command, composed upward)
- API-level result types (`AtomInfo`, `PatternInfo`, `TokenInfo`, `SearchResult`, etc.)
- `Workspace` struct wrapping `HypergraphRef` + metadata
- `WorkspaceManager` — create, open, close, list, delete workspaces
- Persistence — bincode save/load, explicit commit, file locking (multi-reader/single-writer)
- `WorkspaceApi` trait — all commands as trait methods
- `Command` enum — serializable dispatch for all commands
- Basic graph commands: `add_atom(char)`, `add_atoms(Set<char>)`, `add_simple_pattern`, `get_vertex`, `list_vertices`, `list_atoms`
- `get_snapshot`, `get_statistics`
- `tools/context-cli` thin binary crate with clap subcommands + REPL
- Unit tests for every command
- Integration tests for workspace lifecycle

**Depends on:** Nothing (greenfield)
**Estimated files:** ~25 new files
**Risk:** Low — all underlying APIs exist and are tested

---

### Phase 2 — Algorithm Commands
**Plan file:** `20260310_PLAN_CONTEXT_API_PHASE2.md`
**Scope:**
- `search_pattern(Vec<TokenRef>)` wrapping `Find::find_ancestor`
- `search_sequence(String)` wrapping `Find::find_sequence` (string → chars → atoms)
- `insert_first_match(Vec<TokenRef>)` wrapping `ToInsertCtx::insert`
- `insert_sequence(String)` — atomize string then insert
- `insert_sequences(Set<String>)` — bulk insert
- `read_pattern(index)` wrapping context-read expansion
- `read_as_text(index)` — leaf traversal to concatenated chars
- Developer commands: `get_trace_cache`, `validate_graph`
- Update CLI with algorithm subcommands
- Integration tests: search → insert → read round-trips

**Depends on:** Phase 1
**Estimated files:** ~10 new/modified files
**Risk:** Medium — need to handle `Response` → API result type conversion cleanly; `context-read` public API is less mature than search/insert

---

### Phase 3 — MCP Adapter
**Plan file:** `20260310_PLAN_CONTEXT_API_PHASE3.md`
**Scope:**
- `tools/context-mcp` thin binary crate
- Feature-gated `mcp` module in context-api (or standalone in the bin crate)
- Single `execute` MCP tool accepting the `Command` enum as JSON input
- `ServerHandler` implementation using `rmcp` (same pattern as log-viewer/doc-viewer)
- Stdio transport
- End-to-end test: agent workflow (create workspace → add atoms → insert sequence → search → read)

**Depends on:** Phase 2 (needs algorithm commands)
**Estimated files:** ~5 new files
**Risk:** Low — well-established MCP pattern in codebase

---

### Phase 4 — HTTP + GraphQL Adapter
**Plan file:** `20260310_PLAN_CONTEXT_API_PHASE4.md`
**Scope:**
- `tools/context-http` thin binary crate
- RPC endpoint: `POST /api/execute` accepting `Command` JSON
- GraphQL schema generated from `Command`/result types
- CORS, health check, workspace listing endpoints
- Optional: static file serving for future web frontend
- Integration tests via `axum-test`

**Depends on:** Phase 2
**Estimated files:** ~8 new files
**Risk:** Medium — GraphQL schema design requires careful type mapping

---

### Phase 5 — TypeScript Types + Advanced
**Plan file:** `20260310_PLAN_CONTEXT_API_PHASE5.md`
**Scope:**
- Dedicated `packages/context-types/` (or `tools/context-types/`) npm package
- Move all `#[derive(TS)]` from `context-trace` into `context-api` (behind `ts-gen` feature)
- Add `#[derive(TS)]` to all API-level types
- Export target: the new types package
- Update `log-viewer` and `doc-viewer` to consume from the types package
- Export/import workspace commands (JSON/bincode)
- Design document for future instruction language (grammar sketch only)

**Depends on:** Phase 2 (API types must be stable)
**Estimated files:** ~12 new/modified files
**Risk:** Medium — migrating ts-rs out of context-trace is a cross-crate refactor

---

## File Structure

```
crates/context-api/
├── Cargo.toml
├── README.md
├── HIGH_LEVEL_GUIDE.md
└── src/
    ├── lib.rs                      # Public re-exports, feature gates
    ├── error.rs                    # ApiError + per-command error types
    ├── types.rs                    # AtomInfo, PatternInfo, TokenInfo, TokenRef, etc.
    ├── workspace/
    │   ├── mod.rs                  # Workspace struct
    │   ├── manager.rs              # WorkspaceManager (create/open/close/list/delete)
    │   ├── persistence.rs          # Bincode save/load, file locking
    │   └── metadata.rs             # WorkspaceMetadata (timestamps, description)
    ├── commands/
    │   ├── mod.rs                  # Command enum, dispatch, WorkspaceApi trait
    │   ├── atoms.rs                # add_atom, add_atoms, get_atom, list_atoms
    │   ├── patterns.rs             # add_simple_pattern, get_vertex, list_vertices
    │   ├── search.rs               # search_pattern, search_sequence          (Phase 2)
    │   ├── insert.rs               # insert_first_match, insert_sequence      (Phase 2)
    │   ├── read.rs                 # read_pattern, read_as_text               (Phase 2)
    │   └── debug.rs                # get_snapshot, get_statistics, validate_graph, get_trace_cache
    ├── validation.rs               # Pattern validation (add_simple_pattern rules)
    └── tests/
        ├── workspace_tests.rs      # Workspace lifecycle
        ├── atom_tests.rs           # Atom CRUD
        ├── pattern_tests.rs        # Pattern validation
        ├── command_tests.rs        # Command enum dispatch
        └── integration_tests.rs    # End-to-end flows (Phase 2)

tools/context-cli/                  # Phase 1
├── Cargo.toml
└── src/
    ├── main.rs                     # Entry point, clap setup
    ├── commands.rs                 # Subcommand definitions
    ├── repl.rs                     # Interactive REPL
    └── output.rs                   # Human-friendly formatting

tools/context-mcp/                  # Phase 3
├── Cargo.toml
└── src/
    ├── main.rs
    └── server.rs                   # MCP ServerHandler, single execute tool

tools/context-http/                 # Phase 4
├── Cargo.toml
└── src/
    ├── main.rs
    ├── rpc.rs                      # POST /api/execute
    └── graphql.rs                  # GraphQL schema + endpoint

packages/context-types/             # Phase 5
├── package.json
├── tsconfig.json
└── src/
    └── generated/                  # ts-rs output target
```

## Cargo.toml Sketch (context-api)

```toml
[package]
name = "context-api"
version = "0.1.0"
edition = "2024"

[features]
default = []
ts-gen = ["ts-rs"]
dev = []                    # Extra debug commands

[dependencies]
context-trace = { path = "../context-trace" }
context-search = { path = "../context-search" }
context-insert = { path = "../context-insert" }
context-read = { path = "../context-read" }

serde = { version = "1", features = ["derive"] }
serde_json = "1"
bincode = "1"
thiserror = "2"
fs2 = "0.4"                # File locking (multi-reader/single-writer)
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
tracing = "0.1"

ts-rs = { version = "10", features = ["serde-json-impl"], optional = true }

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
```

## Command Model

### WorkspaceApi Trait (Rust consumers)

```pseudo
trait WorkspaceApi {
    // Workspace lifecycle
    fn create_workspace(&self, name: &str) -> Result<WorkspaceInfo, WorkspaceError>;
    fn open_workspace(&self, name: &str) -> Result<WorkspaceInfo, WorkspaceError>;
    fn close_workspace(&self, name: &str) -> Result<(), WorkspaceError>;
    fn save_workspace(&self, name: &str) -> Result<(), WorkspaceError>;
    fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, WorkspaceError>;
    fn delete_workspace(&self, name: &str) -> Result<(), WorkspaceError>;

    // Atoms
    fn add_atom(&self, ws: &str, ch: char) -> Result<AtomInfo, AtomError>;
    fn add_atoms(&self, ws: &str, chars: HashSet<char>) -> Result<Vec<AtomInfo>, AtomError>;
    fn get_atom(&self, ws: &str, ch: char) -> Result<Option<AtomInfo>, WorkspaceError>;
    fn list_atoms(&self, ws: &str) -> Result<Vec<AtomInfo>, WorkspaceError>;

    // Patterns
    fn add_simple_pattern(&self, ws: &str, atoms: Vec<char>) -> Result<PatternInfo, PatternError>;
    fn get_vertex(&self, ws: &str, index: usize) -> Result<Option<VertexInfo>, WorkspaceError>;
    fn list_vertices(&self, ws: &str) -> Result<Vec<TokenInfo>, WorkspaceError>;

    // Search (Phase 2)
    fn search_pattern(&self, ws: &str, query: Vec<TokenRef>) -> Result<SearchResult, SearchError>;
    fn search_sequence(&self, ws: &str, text: &str) -> Result<SearchResult, SearchError>;

    // Insert (Phase 2)
    fn insert_first_match(&self, ws: &str, query: Vec<TokenRef>) -> Result<InsertResult, InsertError>;
    fn insert_sequence(&self, ws: &str, text: &str) -> Result<InsertResult, InsertError>;
    fn insert_sequences(&self, ws: &str, texts: HashSet<String>) -> Result<Vec<InsertResult>, InsertError>;

    // Read (Phase 2)
    fn read_pattern(&self, ws: &str, index: usize) -> Result<PatternReadResult, ReadError>;
    fn read_as_text(&self, ws: &str, index: usize) -> Result<String, ReadError>;

    // Debug
    fn get_snapshot(&self, ws: &str) -> Result<GraphSnapshot, WorkspaceError>;
    fn get_statistics(&self, ws: &str) -> Result<GraphStatistics, WorkspaceError>;
    fn validate_graph(&self, ws: &str) -> Result<ValidationReport, WorkspaceError>;
}
```

### Command Enum (serialized interfaces)

```pseudo
enum Command {
    // Workspace
    CreateWorkspace { name: String },
    OpenWorkspace { name: String },
    CloseWorkspace { name: String },
    SaveWorkspace { name: String },
    ListWorkspaces,
    DeleteWorkspace { name: String },

    // Atoms
    AddAtom { workspace: String, ch: char },
    AddAtoms { workspace: String, chars: HashSet<char> },
    GetAtom { workspace: String, ch: char },
    ListAtoms { workspace: String },

    // Patterns
    AddSimplePattern { workspace: String, atoms: Vec<char> },
    GetVertex { workspace: String, index: usize },
    ListVertices { workspace: String },

    // Search
    SearchPattern { workspace: String, query: Vec<TokenRef> },
    SearchSequence { workspace: String, text: String },

    // Insert
    InsertFirstMatch { workspace: String, query: Vec<TokenRef> },
    InsertSequence { workspace: String, text: String },
    InsertSequences { workspace: String, texts: HashSet<String> },

    // Read
    ReadPattern { workspace: String, index: usize },
    ReadAsText { workspace: String, index: usize },

    // Debug
    GetSnapshot { workspace: String },
    GetStatistics { workspace: String },
    ValidateGraph { workspace: String },
}

enum CommandResult {
    WorkspaceInfo(WorkspaceInfo),
    AtomInfo(AtomInfo),
    AtomInfoList(Vec<AtomInfo>),
    PatternInfo(PatternInfo),
    VertexInfo(Option<VertexInfo>),
    TokenInfoList(Vec<TokenInfo>),
    SearchResult(SearchResult),
    InsertResult(InsertResult),
    ReadResult(PatternReadResult),
    Text(String),
    Snapshot(GraphSnapshot),
    Statistics(GraphStatistics),
    ValidationReport(ValidationReport),
    Unit,
}
```

## Error Model

```pseudo
// Top-level: any command can return this
enum ApiError {
    Workspace(WorkspaceError),
    Atom(AtomError),
    Pattern(PatternError),
    Search(SearchError),
    Insert(InsertError),
    Read(ReadError),
}

// Per-domain errors (specific)
enum WorkspaceError {
    NotFound { name: String },
    AlreadyExists { name: String },
    NotOpen { name: String },
    IoError(std::io::Error),
    LockConflict { name: String },
    SerializationError(String),
}

enum AtomError {
    WorkspaceNotOpen { workspace: String },
    InvalidChar { description: String },
}

enum PatternError {
    WorkspaceNotOpen { workspace: String },
    AtomNotFound { ch: char },
    TooShort { len: usize },            // need >= 2
    AtomAlreadyInPattern { ch: char, existing_parent: usize },
    DuplicateAtomInInput { ch: char },
}

enum SearchError {
    WorkspaceNotOpen { workspace: String },
    TokenNotFound { token_ref: TokenRef },
    QueryTooShort,
    InternalError(String),              // wraps ErrorReason/ErrorState
}

enum InsertError {
    WorkspaceNotOpen { workspace: String },
    TokenNotFound { token_ref: TokenRef },
    InternalError(String),
}

enum ReadError {
    WorkspaceNotOpen { workspace: String },
    VertexNotFound { index: usize },
    InternalError(String),
}
```

## TokenRef Resolution

```pseudo
enum TokenRef {
    Index(usize),       // Direct vertex index
    Label(String),      // Atom sequence string, e.g. "abc" → search for token labeled "abc"
}

// Resolution logic:
fn resolve_token_ref(graph: &Hypergraph, token_ref: &TokenRef) -> Result<Token, SearchError> {
    match token_ref {
        TokenRef::Index(idx) => {
            // Look up vertex directly by index
            graph.get_vertex(VertexIndex(*idx))
                .map(|data| data.to_token())
                .ok_or(SearchError::TokenNotFound { token_ref })
        }
        TokenRef::Label(label) => {
            if label.len() == 1 {
                // Single char → look up atom
                let ch = label.chars().next().unwrap();
                graph.get_atom_by_value(ch)
                    .ok_or(SearchError::TokenNotFound { token_ref })
            } else {
                // Multi-char → search for the sequence
                graph.find_sequence(label.chars())
                    .and_then(|r| r.expect_complete("token ref resolution"))
                    .map(|path| path.root_parent())
                    .map_err(|_| SearchError::TokenNotFound { token_ref })
            }
        }
    }
}
```

## Validation Rules (add_simple_pattern)

```pseudo
fn validate_simple_pattern(graph: &Hypergraph, atoms: &[char]) -> Result<(), PatternError> {
    // 1. Length check
    if atoms.len() < 2 {
        return Err(PatternError::TooShort { len: atoms.len() });
    }

    // 2. Duplicate check within input
    let mut seen = HashSet::new();
    for &ch in atoms {
        if !seen.insert(ch) {
            return Err(PatternError::DuplicateAtomInInput { ch });
        }
    }

    // 3. Each char must be an existing atom
    for &ch in atoms {
        let atom_token = graph.get_atom_by_value(ch)
            .ok_or(PatternError::AtomNotFound { ch })?;

        // 4. Atom must not already have a parent pattern
        let vertex_data = graph.vertex_data(atom_token.index);
        if vertex_data.has_parents() {
            let first_parent = vertex_data.first_parent_index();
            return Err(PatternError::AtomAlreadyInPattern {
                ch,
                existing_parent: first_parent,
            });
        }
    }

    Ok(())
}
```

## Persistence Model

```pseudo
// On disk: .context-engine/<workspace-name>/
//   graph.bin       — bincode-serialized Hypergraph
//   metadata.json   — human-readable workspace metadata
//   graph.bin.lock  — lock file for single-writer

// Open workspace:
fn open(name) {
    let path = base_dir / name;
    acquire_read_lock(path / "graph.bin.lock");  // or write lock if mutating
    let bytes = read(path / "graph.bin");
    let graph: Hypergraph = bincode::deserialize(bytes);
    let metadata: WorkspaceMetadata = serde_json::from_str(read(path / "metadata.json"));
    return Workspace { name, path, graph: HypergraphRef::new(graph), metadata };
}

// Save workspace (explicit commit):
fn save(workspace) {
    acquire_write_lock(workspace.path / "graph.bin.lock");
    let bytes = bincode::serialize(&*workspace.graph);
    atomic_write(workspace.path / "graph.bin", bytes);  // write to tmp, then rename
    let meta_json = serde_json::to_string_pretty(&workspace.metadata);
    atomic_write(workspace.path / "metadata.json", meta_json);
    release_write_lock();
}

// Create workspace:
fn create(name) {
    let path = base_dir / name;
    mkdir_p(path);
    let graph = Hypergraph::default();
    let metadata = WorkspaceMetadata::new(name, now());
    save(Workspace { name, path, graph, metadata });
}
```

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `Hypergraph` bincode serialization fails for some field types | Low | High | Test serialization round-trip in Phase 1 before building anything on top |
| `context-read` public API is immature / insufficient | Medium | Medium | Phase 2 may need to add public wrappers in context-read first |
| DashMap serialization overhead for large graphs | Low | Medium | Bincode handles DashMap via serde; benchmark if >100k vertices |
| File locking semantics differ across OSes | Medium | Low | Use `fs2` crate which abstracts platform differences |
| Command enum grows unwieldy | Low | Low | Group into sub-enums if needed (WorkspaceCommand, GraphCommand, etc.) |
| GraphQL schema complexity (Phase 4) | Medium | Medium | Start with RPC-only, add GraphQL incrementally |
| ts-rs migration breaks log-viewer/doc-viewer | Medium | Medium | Phase 5: update consumers in same PR, run their tests |

## Validation Criteria (Overall)

- [ ] `cargo test -p context-api` passes
- [ ] `cargo build -p context-cli` produces a working binary
- [ ] CLI can: create workspace → add atoms → add_simple_pattern → get_snapshot → save → close → reopen → verify data persisted
- [ ] Search → insert → read round-trip works end-to-end via CLI
- [ ] MCP `execute` tool accepts all commands and returns correct results
- [ ] HTTP `/api/execute` endpoint handles all commands
- [ ] All API types generate valid TypeScript definitions

## Plan Files

| Phase | Plan File | Status |
|-------|-----------|--------|
| 1 | [Phase 1: Foundation + CLI](20260310_PLAN_CONTEXT_API_PHASE1.md) | 📋 |
| 2 | [Phase 2: Algorithm Commands](20260310_PLAN_CONTEXT_API_PHASE2.md) | 📋 |
| 3 | [Phase 3: MCP Adapter](20260310_PLAN_CONTEXT_API_PHASE3.md) | 📋 |
| 4 | [Phase 4: HTTP + GraphQL](20260310_PLAN_CONTEXT_API_PHASE4.md) | 📋 |
| 5 | [Phase 5: TypeScript Types + Advanced](20260310_PLAN_CONTEXT_API_PHASE5.md) | 📋 |