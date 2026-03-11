---
tags: `#context-api` `#phase1` `#foundation` `#cli` `#workspace` `#persistence`
summary: Phase 1 — Create context-api crate with workspace management, basic graph commands, persistence, error types, and CLI adapter
status: 📋
---

# Plan: context-api Phase 1 — Foundation + CLI

## Objective

Create the `crates/context-api` library crate and `tools/context-cli` binary crate from scratch. This phase delivers the core workspace model (create, open, close, save, delete), basic graph commands (add atoms, add simple patterns, get/list vertices), bincode persistence with explicit commit semantics, a unified error model, and a fully functional CLI with subcommands and interactive REPL.

## Context

### Interview Reference

All design decisions: `agents/interviews/20260310_INTERVIEW_CONTEXT_API.md`
Master plan: `agents/plans/20260310_PLAN_CONTEXT_API_OVERVIEW.md`

### Key Decisions Affecting This Phase

- **Storage:** `./.context-engine/<workspace-name>/` (project-local)
- **Persistence:** Bincode, full graph in memory, explicit `save` command
- **Concurrency:** Multi-reader, single-writer (file locking via `fs2`)
- **API model:** `WorkspaceApi` trait + `Command` enum
- **Errors:** Per-command types (`AtomError`, `PatternError`, `WorkspaceError`) composed into `ApiError`
- **Atoms:** Single `char`, deduplicated by char value (idempotent)
- **TokenRef:** By numeric index or string label
- **Validation:** `add_simple_pattern` accepts only atoms, rejects atoms that already have a parent
- **Bulk:** `add_atoms(Set<char>)` — unordered set
- **CLI:** Full command set + interactive REPL, separate binary in `tools/context-cli`
- **Snapshots:** `get_snapshot` re-uses existing `Hypergraph::to_graph_snapshot()`

### Dependencies (External Crates)

| Crate | Version | Purpose |
|-------|---------|---------|
| `context-trace` | path | Graph types, `Hypergraph`, `HypergraphRef`, `GraphSnapshot` |
| `context-search` | path | (Phase 2, but listed as dep for forward compatibility) |
| `context-insert` | path | (Phase 2) |
| `context-read` | path | (Phase 2) |
| `serde` | 1 | Serialization derives |
| `serde_json` | 1 | Metadata persistence |
| `bincode` | 1 | Graph persistence |
| `thiserror` | 2 | Error derive macros |
| `fs2` | 0.4 | Cross-platform file locking |
| `chrono` | 0.4 | Timestamps in metadata |
| `tracing` | 0.1 | Structured logging |
| `clap` | 4 | CLI argument parsing (context-cli only) |
| `rustyline` | 14 | REPL line editing (context-cli only) |
| `tempfile` | 3 | Test fixtures (dev-dependency) |
| `pretty_assertions` | 1 | Test output (dev-dependency) |

### Files Affected

All files are **new** (greenfield):

**`crates/context-api/`:**
- `Cargo.toml`
- `README.md`
- `src/lib.rs`
- `src/error.rs`
- `src/types.rs`
- `src/workspace/mod.rs`
- `src/workspace/manager.rs`
- `src/workspace/persistence.rs`
- `src/workspace/metadata.rs`
- `src/commands/mod.rs`
- `src/commands/atoms.rs`
- `src/commands/patterns.rs`
- `src/commands/debug.rs`
- `src/validation.rs`
- `src/tests/mod.rs`
- `src/tests/workspace_tests.rs`
- `src/tests/atom_tests.rs`
- `src/tests/pattern_tests.rs`
- `src/tests/command_tests.rs`
- `src/tests/persistence_tests.rs`

**`tools/context-cli/`:**
- `Cargo.toml`
- `src/main.rs`
- `src/commands.rs`
- `src/repl.rs`
- `src/output.rs`

**Workspace root:**
- `Cargo.toml` — add `crates/context-api` and `tools/context-cli` to `[workspace.members]`

---

## Analysis

### Current State

There is no unified API layer. Consumers must depend on individual `context-*` crates and wire together trait calls (`Find::find_ancestor`, `ToInsertCtx::insert`, etc.) manually. There is no workspace concept, no persistence, and no CLI.

The building blocks exist:
- `Hypergraph<BaseGraphKind>` supports `Serialize`/`Deserialize` via serde
- `Hypergraph::insert_atom(Atom::Element(ch))` inserts a single-char atom (deduplicates via `atom_keys`)
- `Hypergraph::insert_pattern(Vec<Token>)` inserts a pattern from tokens
- `Hypergraph::to_graph_snapshot()` produces a `GraphSnapshot`
- `HypergraphRef` provides `Arc<Hypergraph>` with `Deref`
- `VertexData` provides `has_parents()`, `child_patterns()`, `is_atom()`, etc.

### Desired State

A working `context-api` library crate that:
1. Manages named workspaces stored in `./.context-engine/<name>/`
2. Loads/saves hypergraphs as bincode with explicit commit
3. Provides a `WorkspaceApi` trait with all Phase 1 commands
4. Provides a `Command` enum that deserializes from JSON and dispatches to trait methods
5. Has comprehensive tests for every operation

A working `tools/context-cli` binary that:
1. Exposes all Phase 1 commands as clap subcommands
2. Provides an interactive REPL mode
3. Formats output in human-readable tables/text

---

## Execution Steps

### Step 1: Workspace Setup — Cargo.toml and Crate Skeleton

**1.1** Add `crates/context-api` and `tools/context-cli` to workspace `Cargo.toml`:

```toml
# In root Cargo.toml [workspace] members list, add:
"crates/context-api",
"tools/context-cli",
```

**1.2** Create `crates/context-api/Cargo.toml`:

```toml
[package]
name = "context-api"
version = "0.1.0"
edition = "2024"
description = "Unified API for hypergraph workspace management and operations"

[features]
default = []
ts-gen = ["ts-rs"]
dev = []

[dependencies]
context-trace = { path = "../context-trace", features = ["test-api"] }
context-search = { path = "../context-search", features = ["test-api"] }
context-insert = { path = "../context-insert", features = ["test-api"] }
# context-read = { path = "../context-read" }  # Phase 2

serde = { version = "1", features = ["derive"] }
serde_json = "1"
bincode = "1"
thiserror = "2"
fs2 = "0.4"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"

ts-rs = { version = "10", features = ["serde-json-impl"], optional = true }

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
```

**1.3** Create `tools/context-cli/Cargo.toml`:

```toml
[package]
name = "context-cli"
version = "0.1.0"
edition = "2024"
description = "CLI for context-engine hypergraph workspaces"

[[bin]]
name = "context-cli"
path = "src/main.rs"

[dependencies]
context-api = { path = "../../crates/context-api" }
clap = { version = "4", features = ["derive"] }
rustyline = "14"
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**1.4** Create minimal `crates/context-api/src/lib.rs`:

```rust
pub mod error;
pub mod types;
pub mod workspace;
pub mod commands;
pub mod validation;

#[cfg(test)]
mod tests;
```

**Verification:** `cargo check -p context-api` and `cargo check -p context-cli` compile (with stub modules).

---

### Step 2: Error Types

**File:** `src/error.rs`

Define the per-command error types and the composed `ApiError`.

```pseudo
// All errors derive Debug, Display, Error via thiserror

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error(transparent)] Workspace(WorkspaceError),
    #[error(transparent)] Atom(AtomError),
    #[error(transparent)] Pattern(PatternError),
    #[error(transparent)] Search(SearchError),      // placeholder for Phase 2
    #[error(transparent)] Insert(InsertError),      // placeholder for Phase 2
    #[error(transparent)] Read(ReadError),          // placeholder for Phase 2
}

#[derive(Debug, thiserror::Error)]
enum WorkspaceError {
    NotFound { name: String },
    AlreadyExists { name: String },
    NotOpen { name: String },
    AlreadyOpen { name: String },
    IoError(#[from] std::io::Error),
    LockConflict { name: String },
    SerializationError(String),
}

#[derive(Debug, thiserror::Error)]
enum AtomError {
    WorkspaceNotOpen { workspace: String },
    // Note: add_atom is idempotent, so no "already exists" error
}

#[derive(Debug, thiserror::Error)]
enum PatternError {
    WorkspaceNotOpen { workspace: String },
    AtomNotFound { ch: char },
    TooShort { len: usize },                         // need >= 2
    AtomAlreadyInPattern { ch: char, existing_parent: usize },
    DuplicateAtomInInput { ch: char },
}

// Phase 2 placeholders (empty for now, filled in Phase 2 plan):
#[derive(Debug, thiserror::Error)]
enum SearchError { /* ... */ }

#[derive(Debug, thiserror::Error)]
enum InsertError { /* ... */ }

#[derive(Debug, thiserror::Error)]
enum ReadError { /* ... */ }
```

Each enum variant should have a human-readable `#[error("...")]` message. Implement `From<XError>` for `ApiError` via `#[error(transparent)]`.

**Verification:** Module compiles, error formatting produces readable strings.

---

### Step 3: API Types

**File:** `src/types.rs`

Public types returned by all API commands. These are the stable interface — internal graph types are never leaked.

```pseudo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct AtomInfo {
    index: usize,
    ch: char,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TokenInfo {
    index: usize,
    label: String,      // concatenated atom chars for the full token
    width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PatternInfo {
    index: usize,
    label: String,
    width: usize,
    children: Vec<TokenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct VertexInfo {
    index: usize,
    label: String,
    width: usize,
    is_atom: bool,
    children: Vec<Vec<TokenInfo>>,   // one Vec<TokenInfo> per child pattern
    parent_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct WorkspaceInfo {
    name: String,
    vertex_count: usize,
    atom_count: usize,
    pattern_count: usize,
    created_at: String,     // ISO 8601
    modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GraphStatistics {
    vertex_count: usize,
    atom_count: usize,
    pattern_count: usize,
    max_width: usize,
    edge_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum TokenRef {
    Index(usize),
    Label(String),
}

// Re-export GraphSnapshot from context-trace
pub use context_trace::graph::snapshot::GraphSnapshot;
```

Conversion functions:

```pseudo
impl AtomInfo {
    fn from_graph(graph: &Hypergraph, token: Token, ch: char) -> Self { ... }
}

impl TokenInfo {
    fn from_graph(graph: &Hypergraph, token: Token) -> Self { ... }
}

impl VertexInfo {
    fn from_graph(graph: &Hypergraph, index: VertexIndex) -> Option<Self> { ... }
}

impl GraphStatistics {
    fn from_graph(graph: &Hypergraph) -> Self { ... }
}
```

These converters access `graph.vertex_data()`, `graph.index_string()`, `graph.is_atom()`, `vertex.child_patterns()`, `vertex.parents()`, etc.

**Verification:** Types compile, `serde_json::to_string` round-trips for each type.

---

### Step 4: Workspace Metadata

**File:** `src/workspace/metadata.rs`

```pseudo
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkspaceMetadata {
    name: String,
    description: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    modified_at: chrono::DateTime<chrono::Utc>,
}

impl WorkspaceMetadata {
    fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        Self { name: name.to_string(), description: None, created_at: now, modified_at: now }
    }

    fn touch(&mut self) {
        self.modified_at = chrono::Utc::now();
    }
}
```

**Verification:** Serializes to/from JSON.

---

### Step 5: Persistence Layer

**File:** `src/workspace/persistence.rs`

Handles reading/writing the workspace directory structure:

```
.context-engine/<name>/
├── graph.bin           # bincode-serialized Hypergraph<BaseGraphKind>
├── metadata.json       # serde_json WorkspaceMetadata
└── .lock               # file lock (fs2)
```

```pseudo
const CONTEXT_DIR: &str = ".context-engine";
const GRAPH_FILE: &str = "graph.bin";
const METADATA_FILE: &str = "metadata.json";
const LOCK_FILE: &str = ".lock";

fn workspace_dir(base: &Path, name: &str) -> PathBuf {
    base.join(CONTEXT_DIR).join(name)
}

fn workspace_exists(base: &Path, name: &str) -> bool {
    workspace_dir(base, name).join(METADATA_FILE).exists()
}

fn list_workspace_names(base: &Path) -> Result<Vec<String>, WorkspaceError> {
    // Read .context-engine/ directory entries
    // Filter to those containing metadata.json
    // Return sorted names
}

fn save_graph(dir: &Path, graph: &Hypergraph<BaseGraphKind>) -> Result<(), WorkspaceError> {
    // Atomic write: serialize to bincode, write to graph.bin.tmp, rename to graph.bin
    let bytes = bincode::serialize(graph)
        .map_err(|e| WorkspaceError::SerializationError(e.to_string()))?;
    let tmp_path = dir.join(format!("{}.tmp", GRAPH_FILE));
    let final_path = dir.join(GRAPH_FILE);
    std::fs::write(&tmp_path, &bytes)?;
    std::fs::rename(&tmp_path, &final_path)?;
    Ok(())
}

fn load_graph(dir: &Path) -> Result<Hypergraph<BaseGraphKind>, WorkspaceError> {
    let bytes = std::fs::read(dir.join(GRAPH_FILE))?;
    bincode::deserialize(&bytes)
        .map_err(|e| WorkspaceError::SerializationError(e.to_string()))
}

fn save_metadata(dir: &Path, metadata: &WorkspaceMetadata) -> Result<(), WorkspaceError> {
    let json = serde_json::to_string_pretty(metadata)
        .map_err(|e| WorkspaceError::SerializationError(e.to_string()))?;
    let tmp_path = dir.join(format!("{}.tmp", METADATA_FILE));
    let final_path = dir.join(METADATA_FILE);
    std::fs::write(&tmp_path, &json)?;
    std::fs::rename(&tmp_path, &final_path)?;
    Ok(())
}

fn load_metadata(dir: &Path) -> Result<WorkspaceMetadata, WorkspaceError> {
    let json = std::fs::read_to_string(dir.join(METADATA_FILE))?;
    serde_json::from_str(&json)
        .map_err(|e| WorkspaceError::SerializationError(e.to_string()))
}

// File locking (fs2)
struct WorkspaceLock {
    _file: std::fs::File,   // holds the lock while alive
}

fn acquire_write_lock(dir: &Path) -> Result<WorkspaceLock, WorkspaceError> {
    use fs2::FileExt;
    let lock_path = dir.join(LOCK_FILE);
    let file = std::fs::OpenOptions::new()
        .create(true).write(true).open(&lock_path)?;
    file.try_lock_exclusive()
        .map_err(|_| WorkspaceError::LockConflict {
            name: dir.file_name().unwrap().to_string_lossy().to_string()
        })?;
    Ok(WorkspaceLock { _file: file })
}

fn acquire_read_lock(dir: &Path) -> Result<WorkspaceLock, WorkspaceError> {
    use fs2::FileExt;
    let lock_path = dir.join(LOCK_FILE);
    let file = std::fs::OpenOptions::new()
        .create(true).read(true).write(true).open(&lock_path)?;
    file.try_lock_shared()
        .map_err(|_| WorkspaceError::LockConflict {
            name: dir.file_name().unwrap().to_string_lossy().to_string()
        })?;
    Ok(WorkspaceLock { _file: file })
}
```

**Verification:** Round-trip test — create a `Hypergraph`, insert atoms, serialize, deserialize, verify identical.

---

### Step 6: Workspace Struct

**File:** `src/workspace/mod.rs`

```pseudo
pub mod manager;
pub mod metadata;
pub mod persistence;

pub struct Workspace {
    pub(crate) name: String,
    pub(crate) dir: PathBuf,
    pub(crate) graph: Hypergraph<BaseGraphKind>,
    pub(crate) metadata: WorkspaceMetadata,
    pub(crate) lock: Option<WorkspaceLock>,   // Some = write lock, None = read-only
    pub(crate) dirty: bool,                    // has unsaved changes
}

impl Workspace {
    // Accessor: get a reference to the graph
    pub fn graph(&self) -> &Hypergraph<BaseGraphKind> { &self.graph }

    // Accessor: get a mutable reference to the graph (marks dirty)
    pub(crate) fn graph_mut(&mut self) -> &mut Hypergraph<BaseGraphKind> {
        self.dirty = true;
        &mut self.graph
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn is_dirty(&self) -> bool { self.dirty }

    pub fn to_info(&self) -> WorkspaceInfo {
        // Build WorkspaceInfo from graph statistics + metadata timestamps
    }
}
```

**Note:** The `Workspace` holds the graph directly (not `HypergraphRef`) because the workspace owns the graph exclusively. `HypergraphRef` (`Arc<Hypergraph>`) is used when passing to search/insert algorithms that expect it (Phase 2 will create a temporary `HypergraphRef` from a reference).

---

### Step 7: WorkspaceManager

**File:** `src/workspace/manager.rs`

```pseudo
pub struct WorkspaceManager {
    base_dir: PathBuf,
    workspaces: HashMap<String, Workspace>,
}

impl WorkspaceManager {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir, workspaces: HashMap::new() }
    }

    // Use current directory as base
    pub fn current_dir() -> Result<Self, WorkspaceError> {
        Ok(Self::new(std::env::current_dir()?))
    }

    pub fn create_workspace(&mut self, name: &str) -> Result<WorkspaceInfo, WorkspaceError> {
        let dir = persistence::workspace_dir(&self.base_dir, name);
        if dir.exists() {
            return Err(WorkspaceError::AlreadyExists { name: name.to_string() });
        }
        std::fs::create_dir_all(&dir)?;

        let graph = Hypergraph::default();
        let metadata = WorkspaceMetadata::new(name);

        // Save initial state
        persistence::save_graph(&dir, &graph)?;
        persistence::save_metadata(&dir, &metadata)?;

        let lock = persistence::acquire_write_lock(&dir)?;
        let ws = Workspace {
            name: name.to_string(), dir, graph, metadata,
            lock: Some(lock), dirty: false,
        };
        let info = ws.to_info();
        self.workspaces.insert(name.to_string(), ws);
        Ok(info)
    }

    pub fn open_workspace(&mut self, name: &str) -> Result<WorkspaceInfo, WorkspaceError> {
        if self.workspaces.contains_key(name) {
            return Err(WorkspaceError::AlreadyOpen { name: name.to_string() });
        }
        let dir = persistence::workspace_dir(&self.base_dir, name);
        if !persistence::workspace_exists(&self.base_dir, name) {
            return Err(WorkspaceError::NotFound { name: name.to_string() });
        }

        let lock = persistence::acquire_write_lock(&dir)?;
        let graph = persistence::load_graph(&dir)?;
        let metadata = persistence::load_metadata(&dir)?;

        let ws = Workspace {
            name: name.to_string(), dir, graph, metadata,
            lock: Some(lock), dirty: false,
        };
        let info = ws.to_info();
        self.workspaces.insert(name.to_string(), ws);
        Ok(info)
    }

    pub fn save_workspace(&mut self, name: &str) -> Result<(), WorkspaceError> {
        let ws = self.get_workspace_mut(name)?;
        ws.metadata.touch();
        persistence::save_graph(&ws.dir, &ws.graph)?;
        persistence::save_metadata(&ws.dir, &ws.metadata)?;
        ws.dirty = false;
        Ok(())
    }

    pub fn close_workspace(&mut self, name: &str) -> Result<(), WorkspaceError> {
        // Note: does NOT auto-save — caller must save first if desired
        let ws = self.workspaces.remove(name)
            .ok_or(WorkspaceError::NotOpen { name: name.to_string() })?;
        if ws.dirty {
            tracing::warn!(workspace = name, "Closing workspace with unsaved changes");
        }
        // Lock is released when Workspace (and its WorkspaceLock) is dropped
        Ok(())
    }

    pub fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, WorkspaceError> {
        let mut result = Vec::new();

        // Include open workspaces (live data)
        for ws in self.workspaces.values() {
            result.push(ws.to_info());
        }

        // Include closed workspaces from disk
        for name in persistence::list_workspace_names(&self.base_dir)? {
            if !self.workspaces.contains_key(&name) {
                let dir = persistence::workspace_dir(&self.base_dir, &name);
                if let Ok(metadata) = persistence::load_metadata(&dir) {
                    // Load just metadata, not the full graph
                    result.push(WorkspaceInfo {
                        name: name.clone(),
                        vertex_count: 0, // unknown when not open
                        atom_count: 0,
                        pattern_count: 0,
                        created_at: metadata.created_at.to_rfc3339(),
                        modified_at: metadata.modified_at.to_rfc3339(),
                    });
                }
            }
        }

        result.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(result)
    }

    pub fn delete_workspace(&mut self, name: &str) -> Result<(), WorkspaceError> {
        // Close if open
        self.workspaces.remove(name);
        let dir = persistence::workspace_dir(&self.base_dir, name);
        if !dir.exists() {
            return Err(WorkspaceError::NotFound { name: name.to_string() });
        }
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    // Internal: get open workspace by name
    pub(crate) fn get_workspace(&self, name: &str) -> Result<&Workspace, WorkspaceError> {
        self.workspaces.get(name)
            .ok_or(WorkspaceError::NotOpen { name: name.to_string() })
    }

    pub(crate) fn get_workspace_mut(&mut self, name: &str) -> Result<&mut Workspace, WorkspaceError> {
        self.workspaces.get_mut(name)
            .ok_or(WorkspaceError::NotOpen { name: name.to_string() })
    }
}
```

**Verification:** Create → list → open → close → delete lifecycle works. Verify lock prevents double-open.

---

### Step 8: Validation Layer

**File:** `src/validation.rs`

```pseudo
use crate::error::PatternError;
use context_trace::{Hypergraph, graph::kind::BaseGraphKind};

pub fn validate_simple_pattern(
    graph: &Hypergraph<BaseGraphKind>,
    atoms: &[char],
) -> Result<(), PatternError> {
    // 1. Length >= 2
    if atoms.len() < 2 {
        return Err(PatternError::TooShort { len: atoms.len() });
    }

    // 2. No duplicate chars in input
    let mut seen = std::collections::HashSet::new();
    for &ch in atoms {
        if !seen.insert(ch) {
            return Err(PatternError::DuplicateAtomInInput { ch });
        }
    }

    // 3. Each char must exist as an atom, and must not already have a parent
    for &ch in atoms {
        // Look up atom by char value
        let atom_token = graph.get_atom_token_by_value(ch)
            .ok_or(PatternError::AtomNotFound { ch })?;

        // Check: atom has no parent patterns
        let vertex_data = graph.expect_vertex_data_by_index(atom_token.index);
        if !vertex_data.parent_entries().is_empty() {
            // Find the first parent's index for the error message
            let first_parent_index = vertex_data
                .parent_entries().iter().next()
                .map(|p| p.parent_index().0)
                .unwrap_or(0);
            return Err(PatternError::AtomAlreadyInPattern {
                ch,
                existing_parent: first_parent_index,
            });
        }
    }

    Ok(())
}
```

**Note:** The exact method names on `Hypergraph` for looking up atoms by value and checking parents need to be verified against the actual context-trace API. The `atom_keys` DashMap provides reverse lookup. We may need to add a small public helper method to `Hypergraph` if one doesn't exist (e.g. `get_atom_token_by_value(ch) -> Option<Token>`). If so, that's a minor addition to context-trace, documented as a prerequisite.

**Verification:** Test all validation paths — too short, duplicate chars, atom not found, atom already in pattern.

---

### Step 9: Atom Commands

**File:** `src/commands/atoms.rs`

```pseudo
use crate::{types::AtomInfo, error::AtomError, workspace::manager::WorkspaceManager};
use context_trace::graph::vertex::atom::Atom;

impl WorkspaceManager {
    pub fn add_atom(&mut self, ws_name: &str, ch: char) -> Result<AtomInfo, AtomError> {
        let ws = self.get_workspace_mut(ws_name)
            .map_err(|_| AtomError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_mut();

        // insert_atom is idempotent (deduplicates by atom value)
        let token = graph.insert_atom(Atom::Element(ch));
        Ok(AtomInfo { index: token.index.0, ch })
    }

    pub fn add_atoms(
        &mut self,
        ws_name: &str,
        chars: std::collections::HashSet<char>,
    ) -> Result<Vec<AtomInfo>, AtomError> {
        let ws = self.get_workspace_mut(ws_name)
            .map_err(|_| AtomError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_mut();

        let mut results = Vec::with_capacity(chars.len());
        for ch in chars {
            let token = graph.insert_atom(Atom::Element(ch));
            results.push(AtomInfo { index: token.index.0, ch });
        }
        Ok(results)
    }

    pub fn get_atom(&self, ws_name: &str, ch: char) -> Result<Option<AtomInfo>, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        let graph = ws.graph();
        match graph.get_atom_token_by_value(ch) {
            Some(token) => Ok(Some(AtomInfo { index: token.index.0, ch })),
            None => Ok(None),
        }
    }

    pub fn list_atoms(&self, ws_name: &str) -> Result<Vec<AtomInfo>, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        let graph = ws.graph();
        // Iterate all vertices, filter to atoms, build AtomInfo for each
        let mut atoms: Vec<AtomInfo> = graph.atom_iter()
            .map(|(ch, token)| AtomInfo { index: token.index.0, ch })
            .collect();
        atoms.sort_by_key(|a| a.index);
        Ok(atoms)
    }
}
```

**Note:** The exact atom iteration API on `Hypergraph` needs to be verified. The `atoms` DashMap and `atom_keys` DashMap provide the data. We may need a helper method `atom_iter() -> impl Iterator<Item = (char, Token)>`.

**Verification:** Add atom → get atom returns same. Add same atom twice → same index. List atoms returns all.

---

### Step 10: Pattern Commands

**File:** `src/commands/patterns.rs`

```pseudo
impl WorkspaceManager {
    pub fn add_simple_pattern(
        &mut self,
        ws_name: &str,
        atoms: Vec<char>,
    ) -> Result<PatternInfo, PatternError> {
        // 1. Validate
        {
            let ws = self.get_workspace(ws_name)
                .map_err(|_| PatternError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
            crate::validation::validate_simple_pattern(ws.graph(), &atoms)?;
        }

        // 2. Resolve atom chars to tokens
        let ws = self.get_workspace_mut(ws_name)
            .map_err(|_| PatternError::WorkspaceNotOpen { workspace: ws_name.to_string() })?;
        let graph = ws.graph_mut();

        let tokens: Vec<Token> = atoms.iter()
            .map(|&ch| graph.get_atom_token_by_value(ch).unwrap())  // safe: validated above
            .collect();

        // 3. Insert pattern
        let pattern_token = graph.insert_pattern(tokens);

        // 4. Build result
        let children: Vec<TokenInfo> = atoms.iter()
            .map(|&ch| {
                let t = graph.get_atom_token_by_value(ch).unwrap();
                TokenInfo::from_graph(graph, t)
            })
            .collect();

        Ok(PatternInfo {
            index: pattern_token.index.0,
            label: atoms.iter().collect::<String>(),
            width: atoms.len(),
            children,
        })
    }

    pub fn get_vertex(
        &self,
        ws_name: &str,
        index: usize,
    ) -> Result<Option<VertexInfo>, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        Ok(VertexInfo::from_graph(ws.graph(), VertexIndex(index)))
    }

    pub fn list_vertices(&self, ws_name: &str) -> Result<Vec<TokenInfo>, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        let graph = ws.graph();
        let mut vertices: Vec<TokenInfo> = graph.vertex_iter()
            .map(|(_, data)| TokenInfo::from_graph(graph, data.to_token()))
            .collect();
        vertices.sort_by_key(|v| v.index);
        Ok(vertices)
    }
}
```

**Verification:** Add atoms → add_simple_pattern succeeds. Try add_simple_pattern with atom that has a parent → error. Try with < 2 atoms → error.

---

### Step 11: Debug Commands

**File:** `src/commands/debug.rs`

```pseudo
impl WorkspaceManager {
    pub fn get_snapshot(&self, ws_name: &str) -> Result<GraphSnapshot, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        Ok(ws.graph().to_graph_snapshot())
    }

    pub fn get_statistics(&self, ws_name: &str) -> Result<GraphStatistics, ApiError> {
        let ws = self.get_workspace(ws_name)?;
        Ok(GraphStatistics::from_graph(ws.graph()))
    }
}
```

**Verification:** Create workspace, add data, get_snapshot returns non-empty nodes/edges, statistics counts match.

---

### Step 12: Command Enum and Dispatch

**File:** `src/commands/mod.rs`

```pseudo
pub mod atoms;
pub mod patterns;
pub mod debug;

// The WorkspaceApi trait (for Rust consumers who want a typed interface)
pub trait WorkspaceApi {
    fn create_workspace(&mut self, name: &str) -> Result<WorkspaceInfo, WorkspaceError>;
    fn open_workspace(&mut self, name: &str) -> Result<WorkspaceInfo, WorkspaceError>;
    fn close_workspace(&mut self, name: &str) -> Result<(), WorkspaceError>;
    fn save_workspace(&mut self, name: &str) -> Result<(), WorkspaceError>;
    fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, WorkspaceError>;
    fn delete_workspace(&mut self, name: &str) -> Result<(), WorkspaceError>;

    fn add_atom(&mut self, ws: &str, ch: char) -> Result<AtomInfo, AtomError>;
    fn add_atoms(&mut self, ws: &str, chars: HashSet<char>) -> Result<Vec<AtomInfo>, AtomError>;
    fn get_atom(&self, ws: &str, ch: char) -> Result<Option<AtomInfo>, ApiError>;
    fn list_atoms(&self, ws: &str) -> Result<Vec<AtomInfo>, ApiError>;

    fn add_simple_pattern(&mut self, ws: &str, atoms: Vec<char>) -> Result<PatternInfo, PatternError>;
    fn get_vertex(&self, ws: &str, index: usize) -> Result<Option<VertexInfo>, ApiError>;
    fn list_vertices(&self, ws: &str) -> Result<Vec<TokenInfo>, ApiError>;

    fn get_snapshot(&self, ws: &str) -> Result<GraphSnapshot, ApiError>;
    fn get_statistics(&self, ws: &str) -> Result<GraphStatistics, ApiError>;
}

// impl WorkspaceApi for WorkspaceManager { ... }
// (delegates to the methods defined in steps 7, 9, 10, 11)

// The Command enum (for serialized interfaces: CLI, MCP, HTTP)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
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

    // Debug
    GetSnapshot { workspace: String },
    GetStatistics { workspace: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum CommandResult {
    WorkspaceInfo(WorkspaceInfo),
    WorkspaceInfoList(Vec<WorkspaceInfo>),
    AtomInfo(AtomInfo),
    AtomInfoList(Vec<AtomInfo>),
    OptionalAtomInfo(Option<AtomInfo>),
    PatternInfo(PatternInfo),
    OptionalVertexInfo(Option<VertexInfo>),
    TokenInfoList(Vec<TokenInfo>),
    Snapshot(GraphSnapshot),
    Statistics(GraphStatistics),
    Ok,
}

// Dispatch function
fn execute(manager: &mut WorkspaceManager, cmd: Command) -> Result<CommandResult, ApiError> {
    match cmd {
        Command::CreateWorkspace { name } => {
            let info = manager.create_workspace(&name)?;
            Ok(CommandResult::WorkspaceInfo(info))
        }
        Command::AddAtom { workspace, ch } => {
            let info = manager.add_atom(&workspace, ch)?;
            Ok(CommandResult::AtomInfo(info))
        }
        // ... etc for all variants
    }
}
```

**Verification:** Round-trip: serialize `Command` to JSON → deserialize → execute → serialize `CommandResult` → deserialize. All variants covered.

---

### Step 13: CLI — Main and Subcommands

**File:** `tools/context-cli/src/main.rs`

```pseudo
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "context-cli", about = "Context Engine hypergraph workspace CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Create a new workspace
    Create { name: String },
    /// Open an existing workspace
    Open { name: String },
    /// Close an open workspace
    Close { name: String },
    /// Save workspace to disk
    Save { name: String },
    /// List all workspaces
    List,
    /// Delete a workspace
    Delete { name: String },
    /// Add a single-character atom
    AddAtom { workspace: String, ch: char },
    /// Add multiple atoms
    AddAtoms { workspace: String, chars: String },  // "abcde" → set of chars
    /// Add a simple pattern from atom chars
    AddSimplePattern { workspace: String, atoms: String },  // "abc" → ['a','b','c']
    /// Get vertex info by index
    GetVertex { workspace: String, index: usize },
    /// List all vertices
    ListVertices { workspace: String },
    /// List all atoms
    ListAtoms { workspace: String },
    /// Get graph snapshot (JSON)
    Snapshot { workspace: String },
    /// Get graph statistics
    Stats { workspace: String },
    /// Start interactive REPL
    Repl,
}

fn main() {
    // Init tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let mut manager = WorkspaceManager::current_dir().unwrap();

    match cli.command {
        Some(cmd) => execute_command(&mut manager, cmd),
        None => {
            // No subcommand → start REPL
            repl::run(&mut manager);
        }
    }
}

fn execute_command(manager: &mut WorkspaceManager, cmd: CliCommand) {
    let result = match cmd {
        CliCommand::Create { name } => {
            manager.create_workspace(&name)
                .map(|info| output::print_workspace_info(&info))
        }
        CliCommand::AddAtom { workspace, ch } => {
            manager.add_atom(&workspace, ch)
                .map(|info| output::print_atom_info(&info))
        }
        CliCommand::AddAtoms { workspace, chars } => {
            let char_set: HashSet<char> = chars.chars().collect();
            manager.add_atoms(&workspace, char_set)
                .map(|infos| output::print_atom_info_list(&infos))
        }
        CliCommand::AddSimplePattern { workspace, atoms } => {
            let atom_chars: Vec<char> = atoms.chars().collect();
            manager.add_simple_pattern(&workspace, atom_chars)
                .map(|info| output::print_pattern_info(&info))
        }
        // ... etc
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
```

**Verification:** `cargo build -p context-cli` produces a binary. `context-cli create test` creates a workspace directory.

---

### Step 14: CLI — REPL

**File:** `tools/context-cli/src/repl.rs`

```pseudo
use rustyline::{DefaultEditor, error::ReadlineError};

pub fn run(manager: &mut WorkspaceManager) {
    println!("Context Engine REPL (type 'help' for commands, 'quit' to exit)");

    let mut rl = DefaultEditor::new().unwrap();
    let mut current_workspace: Option<String> = None;

    loop {
        let prompt = match &current_workspace {
            Some(name) => format!("({name})> "),
            None => "> ".to_string(),
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() { continue; }
                rl.add_history_entry(line).ok();

                match line {
                    "quit" | "exit" => break,
                    "help" => print_help(),
                    _ => {
                        execute_repl_line(manager, &mut current_workspace, line);
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }
}

fn execute_repl_line(
    manager: &mut WorkspaceManager,
    current_ws: &mut Option<String>,
    line: &str,
) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() { return; }

    match parts[0] {
        "create" => { /* parse name, call manager.create_workspace */ }
        "open" => {
            /* parse name, call manager.open_workspace, set current_ws */
        }
        "close" => { /* close current_ws or named */ }
        "save" => { /* save current_ws or named */ }
        "list" => { /* list_workspaces */ }
        "delete" => { /* delete named */ }
        "atom" => {
            // "atom a" or "atom abc" (add multiple)
            // Uses current_ws
        }
        "pattern" => {
            // "pattern abc" → add_simple_pattern(['a','b','c'])
        }
        "vertex" => {
            // "vertex 42" → get_vertex
        }
        "vertices" => { /* list_vertices */ }
        "atoms" => { /* list_atoms */ }
        "snapshot" => { /* get_snapshot, print JSON */ }
        "stats" => { /* get_statistics */ }
        _ => {
            eprintln!("Unknown command: '{}'. Type 'help' for commands.", parts[0]);
        }
    }
}

fn print_help() {
    println!("Workspace commands:");
    println!("  create <name>        Create a new workspace");
    println!("  open <name>          Open a workspace (sets as current)");
    println!("  close                Close the current workspace");
    println!("  save                 Save the current workspace to disk");
    println!("  list                 List all workspaces");
    println!("  delete <name>        Delete a workspace");
    println!();
    println!("Graph commands (require an open workspace):");
    println!("  atom <chars>         Add atom(s): 'atom a' or 'atom abcde'");
    println!("  pattern <chars>      Add simple pattern from atoms: 'pattern abc'");
    println!("  vertex <index>       Show vertex details");
    println!("  vertices             List all vertices");
    println!("  atoms                List all atoms");
    println!("  snapshot             Print graph snapshot as JSON");
    println!("  stats                Print graph statistics");
    println!();
    println!("  help                 Show this help");
    println!("  quit / exit          Exit the REPL");
}
```

**Verification:** Start REPL, create workspace, add atoms, add pattern, list vertices, save, quit, reopen, verify data persisted.

---

### Step 15: CLI — Output Formatting

**File:** `tools/context-cli/src/output.rs`

```pseudo
pub fn print_workspace_info(info: &WorkspaceInfo) {
    println!("Workspace: {}", info.name);
    println!("  Vertices: {}, Atoms: {}, Patterns: {}", info.vertex_count, info.atom_count, info.pattern_count);
    println!("  Created:  {}", info.created_at);
    println!("  Modified: {}", info.modified_at);
}

pub fn print_atom_info(info: &AtomInfo) {
    println!("Atom '{}' (index: {})", info.ch, info.index);
}

pub fn print_atom_info_list(infos: &[AtomInfo]) {
    println!("Atoms ({}):", infos.len());
    for info in infos {
        println!("  '{}' → {}", info.ch, info.index);
    }
}

pub fn print_pattern_info(info: &PatternInfo) {
    println!("Pattern \"{}\" (index: {}, width: {})", info.label, info.index, info.width);
    println!("  Children: {}", info.children.iter()
        .map(|c| format!("{}({})", c.label, c.index))
        .collect::<Vec<_>>().join(" → "));
}

pub fn print_vertex_info(info: &VertexInfo) {
    println!("Vertex {} \"{}\" (width: {}, {})", info.index, info.label, info.width,
        if info.is_atom { "atom" } else { "pattern" });
    for (i, children) in info.children.iter().enumerate() {
        println!("  Pattern {}: {}", i, children.iter()
            .map(|c| format!("{}({})", c.label, c.index))
            .collect::<Vec<_>>().join(", "));
    }
    println!("  Parents: {}", info.parent_count);
}

pub fn print_token_info_list(infos: &[TokenInfo]) {
    println!("Vertices ({}):", infos.len());
    for info in infos {
        println!("  [{}] \"{}\" (width: {})", info.index, info.label, info.width);
    }
}

pub fn print_statistics(stats: &GraphStatistics) {
    println!("Graph Statistics:");
    println!("  Vertices: {}", stats.vertex_count);
    println!("  Atoms:    {}", stats.atom_count);
    println!("  Patterns: {}", stats.pattern_count);
    println!("  Edges:    {}", stats.edge_count);
    println!("  Max width: {}", stats.max_width);
}
```

---

### Step 16: Tests

**File:** `src/tests/mod.rs` and sub-files

Write tests for each concern:

**`workspace_tests.rs`:**
- Create workspace → directory exists, metadata.json exists, graph.bin exists
- Open workspace → returns correct info
- Open nonexistent → `WorkspaceError::NotFound`
- Create duplicate → `WorkspaceError::AlreadyExists`
- Close workspace → removed from manager
- Close non-open → `WorkspaceError::NotOpen`
- Delete workspace → directory removed
- Save → modifies `modified_at` timestamp
- List → shows both open and disk-only workspaces

**`atom_tests.rs`:**
- Add atom → returns correct `AtomInfo`
- Add same atom twice → same index (idempotent)
- Add atoms (bulk) → correct count, all unique indices
- Get atom → returns `Some` for existing, `None` for missing
- List atoms → returns all, sorted by index

**`pattern_tests.rs`:**
- Add simple pattern `['a','b']` → succeeds, correct label, width, children
- Add simple pattern with 1 atom → `PatternError::TooShort`
- Add simple pattern with nonexistent atom → `PatternError::AtomNotFound`
- Add simple pattern with duplicate char → `PatternError::DuplicateAtomInInput`
- Add simple pattern with atom that already has parent → `PatternError::AtomAlreadyInPattern`
- Get vertex (atom) → `is_atom: true`
- Get vertex (pattern) → `is_atom: false`, children present
- Get vertex (missing) → `None`
- List vertices → includes both atoms and patterns

**`persistence_tests.rs`:**
- Create → save → close → open → verify atom and pattern data persisted
- Bincode round-trip: serialize graph → deserialize → vertex_count matches
- Atomic write: verify temp file doesn't persist on success

**`command_tests.rs`:**
- JSON round-trip for every `Command` variant
- `execute()` dispatches correctly for each command
- Error results are serializable to JSON

All tests use `tempfile::TempDir` as the base directory to avoid filesystem pollution.

**Verification:** `cargo test -p context-api` — all tests pass.

---

### Step 17: README

**File:** `crates/context-api/README.md`

Brief description of the crate's purpose, feature flags, quick usage example, and pointer to the CLI.

---

### Step 18: Final Verification

- [ ] `cargo check --workspace` — no errors
- [ ] `cargo test -p context-api` — all tests pass
- [ ] `cargo build -p context-cli` — binary builds
- [ ] Manual test: `context-cli create demo && context-cli add-atom demo a && context-cli add-atom demo b && context-cli add-simple-pattern demo ab && context-cli snapshot demo && context-cli save demo`
- [ ] Manual test: `context-cli repl` → `create demo` → `open demo` → `atom abc` → `pattern abc` → `vertices` → `save` → `quit`
- [ ] Workspace directory `.context-engine/demo/` contains `graph.bin` and `metadata.json`
- [ ] Reopen: `context-cli open demo && context-cli list-atoms demo` → shows a, b, c

---

## Prerequisites / Minor Context-Trace Additions

The following small helpers may need to be added to `context-trace` if they don't already exist as public methods:

1. **`Hypergraph::get_atom_token_by_value(&self, ch: G::Atom) -> Option<Token>`**
   — Lookup in the `atom_keys` DashMap. Currently `atom_keys: DashMap<G::Atom, VertexKey>` exists but may not have a public accessor that returns a `Token`.

2. **`Hypergraph::atom_iter(&self) -> impl Iterator<Item = (G::Atom, Token)>`**
   — Iterate all atoms with their char values and tokens. The `atoms` DashMap exists but may need a public iterator.

3. **`VertexData::parent_entries(&self) -> &[Parent]`** or similar
   — Check if there's a public way to test "does this vertex have any parents". `has_vertex_data::HasVertexData` may already provide this.

These are minor (< 10 lines each) and can be added in a preliminary commit.

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Bincode can't round-trip `Hypergraph` (DashMap, AtomicUsize) | Low | High | Test serialization first in Step 5 before building anything on top. If it fails, implement custom `Serialize`/`Deserialize` or use `serde_json` as fallback. |
| `context-trace` lacks public helpers for atom lookup | Medium | Low | Add small public methods (see Prerequisites above). Minimal change to existing crate. |
| `fs2` file locking behaves differently on Windows | Low | Low | `fs2` abstracts platform differences. Test on CI. |
| REPL complexity grows — argument parsing edge cases | Medium | Low | Keep REPL commands simple (space-separated). Complex inputs go through CLI subcommands. |
| `insert_pattern` internal invariant assumptions | Low | Medium | `add_simple_pattern` validates heavily before calling `insert_pattern`, so invalid states should not be reachable. |

## Notes

### Questions for User
- Should the REPL support command history persistence across sessions (e.g. `.context-engine/.repl_history`)?
- Should `list_workspaces` show vertex counts for closed workspaces (requires loading graph.bin, which could be slow for large graphs)?

### Deviations from Plan
*(To be filled during execution)*

### Lessons Learned
*(To be filled after execution)*