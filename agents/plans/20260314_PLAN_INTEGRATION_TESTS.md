---
tags: `#plan` `#testing` `#integration` `#context-api` `#context-cli` `#context-read`
summary: Comprehensive integration test suite for the Context-Read UX Improvement project — 38+ tests across 6 categories, API-level and CLI-level, with known-failure tracking.
status: 🚧 implementing
phase: 3-implement
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
design_decisions: D14, D16
progress: "Phases 1-4 complete (harness + 5 categories + FAILING_TESTS.md). Phase 5 (REPL tests) deferred. 32 tests: 23 pass, 9 known failures."
---

# Plan: Integration Test Suite for Context-Read UX Improvement

**Date:** 2026-03-14
**Scope:** Large (new test harness, 38+ test cases, 6 categories, cross-crate)
**Crates:** `context-api`, `context-cli`, `context-read`, `context-insert`, `context-search`

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [Test Philosophy](#test-philosophy)
4. [Files Affected](#files-affected)
5. [Test Helper Design](#test-helper-design)
6. [Test Categories](#test-categories)
7. [Execution Steps](#execution-steps)
8. [FAILING_TESTS.md Format](#failing_testsmd-format)
9. [Test Execution Strategy](#test-execution-strategy)
10. [Validation](#validation)
11. [Risks & Mitigations](#risks--mitigations)
12. [Related Documents](#related-documents)

---

## Objective

Build a comprehensive Rust integration test suite for the context-engine CLI and API that validates atom management, text reading, deduplication, file input, REPL integration, and edge cases. Tests are written to express **correct expected behavior**; known failures are documented rather than hidden.

---

## Context

### Parent Plan

This plan is a child of [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md), specifically Phase 3d (Test Suite) and Phase 4 (Validate). It implements the test harness described in the parent plan's **Integration Test Suite Plan** section (L619–L856).

### Design Decisions

| Decision | Summary |
|----------|---------|
| **D14** | Rust test harness, API-level + CLI-level tests |
| **D16** | Design new correct tests first, review existing, accept failures |

### Current Crate Test Status

| Crate | Status | Notes |
|-------|--------|-------|
| `context-trace` | ✅ Stable | All tests pass |
| `context-search` | ⚠️ Largely stable | 1 known bug — search queue cleared prematurely after first match |
| `context-insert` | ✅ Stable | All tests pass |
| `context-read` | ❌ ~29/60 failing | Overlap/repeat handling broken (cursor advancement, `append_to_pattern`) |
| `context-api` | ⚠️ Partial failures | ~15/292 failing — insert reports `already_existed=true` incorrectly; read returns truncated text |

### Architecture Stack (Bottom → Top)

```
context-trace          (hypergraph storage, vertices, patterns)
  ↓
context-search         (find ancestors/descendants in graph)
  ↓
context-insert         (insert_or_get_complete → insert_next_match)
  ↓
context-read           (iterative largest-match text indexing)
  ↓
context-api            (Command/CommandResult protocol, WorkspaceManager)
  ↓
context-cli            (CLI binary, REPL, file I/O)
```

---

## Test Philosophy

1. **Write correct tests first** — Tests express what *should* happen, not what currently happens
2. **Known failures are acceptable and informative** — They document bugs and guide fixes
3. **Two test levels:**
   - **API-level** (Categories 1–4, 6): Call `context_api::commands::execute()` directly — fast, precise, no process overhead
   - **CLI-level** (Category 5): Invoke the `context-cli` binary via `std::process::Command` — true end-to-end UX validation
4. **Isolated workspaces** — Each test creates a temporary workspace in a temp directory; teardown is automatic via `Drop`
5. **Incremental complexity** — Start with atoms (simplest), build to deduplication (complex), finish with edge cases
6. **`FAILING_TESTS.md` tracker** — Every failing test is mapped to a root cause, not silenced

---

## Files Affected

### New Files

```
tools/context-cli/tests/
├── integration/
│   ├── mod.rs                    # Test module root — declares submodules
│   ├── atom_tests.rs             # Category 1: Atom management (6 tests)
│   ├── basic_read_tests.rs       # Category 2: Basic read operations (8 tests)
│   ├── dedup_tests.rs            # Category 3: Deduplication / shared substrings (8 tests)
│   ├── file_input_tests.rs       # Category 4: File/stdin input (4 tests)
│   ├── repl_tests.rs             # Category 5: REPL integration via CLI binary (6 tests)
│   └── edge_case_tests.rs        # Category 6: Edge cases & error handling (6 tests)
├── common/
│   ├── mod.rs                    # Common module root
│   └── helpers.rs                # Test fixtures, workspace setup/teardown, assertion helpers
├── cli_integration.rs            # Crate-level test entry point (imports integration/)
└── FAILING_TESTS.md              # Maps known test failures to root causes
```

### Modified Files

| File | Change |
|------|--------|
| `tools/context-cli/Cargo.toml` | Add `[dev-dependencies]` for `tempfile` |

---

## Test Helper Design

### `common/helpers.rs` — Full Implementation

```rust
//! Shared test utilities for context-cli integration tests.
//!
//! Provides workspace lifecycle helpers, command shorthands, and assertion
//! utilities. Every test gets an isolated temporary directory that is
//! cleaned up automatically when the `TestWorkspace` guard is dropped.

use context_api::{
    commands::{execute, Command, CommandResult},
    workspace::manager::WorkspaceManager,
};
use std::path::PathBuf;
use tempfile::TempDir;

/// A self-contained test workspace with automatic cleanup.
///
/// On creation, initialises a `WorkspaceManager` rooted in a temporary
/// directory and creates + opens a workspace with the given name.
/// On drop, the temporary directory (and all workspace data) is removed.
pub struct TestWorkspace {
    pub manager: WorkspaceManager,
    pub name: String,
    _temp_dir: TempDir, // kept alive for the duration of the test
}

impl TestWorkspace {
    /// Create a new isolated test workspace.
    ///
    /// # Panics
    ///
    /// Panics if workspace creation or opening fails — this is a test
    /// setup helper, so failures should be immediate and loud.
    pub fn new(name: &str) -> Self {
        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let mut manager = WorkspaceManager::new(temp_dir.path().to_path_buf());

        let cmd = Command::CreateWorkspace {
            name: name.to_string(),
        };
        execute(&mut manager, cmd).expect("failed to create test workspace");

        Self {
            manager,
            name: name.to_string(),
            _temp_dir: temp_dir,
        }
    }

    /// Execute a command against this workspace's manager.
    pub fn exec(&mut self, cmd: Command) -> Result<CommandResult, context_api::error::ApiError> {
        execute(&mut self.manager, cmd)
    }

    /// Shorthand: insert a text sequence into this workspace.
    pub fn insert_text(&mut self, text: &str) -> CommandResult {
        let cmd = Command::InsertSequence {
            workspace: self.name.clone(),
            text: text.to_string(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("insert_text({text:?}) failed: {e}"))
    }

    /// Shorthand: read a pattern by vertex index.
    pub fn read_pattern(&mut self, index: usize) -> CommandResult {
        let cmd = Command::ReadPattern {
            workspace: self.name.clone(),
            index,
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("read_pattern({index}) failed: {e}"))
    }

    /// Shorthand: read as text by vertex index.
    pub fn read_as_text(&mut self, index: usize) -> CommandResult {
        let cmd = Command::ReadAsText {
            workspace: self.name.clone(),
            index,
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("read_as_text({index}) failed: {e}"))
    }

    /// Shorthand: search for a text sequence.
    pub fn search_text(&mut self, text: &str) -> CommandResult {
        let cmd = Command::SearchSequence {
            workspace: self.name.clone(),
            text: text.to_string(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("search_text({text:?}) failed: {e}"))
    }

    /// Shorthand: add a single atom.
    pub fn add_atom(&mut self, ch: char) -> CommandResult {
        let cmd = Command::AddAtom {
            workspace: self.name.clone(),
            ch,
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("add_atom({ch:?}) failed: {e}"))
    }

    /// Shorthand: add multiple atoms from a string.
    pub fn add_atoms(&mut self, chars: &str) -> CommandResult {
        let cmd = Command::AddAtoms {
            workspace: self.name.clone(),
            chars: chars.chars().collect(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("add_atoms({chars:?}) failed: {e}"))
    }

    /// Shorthand: list all atoms.
    pub fn list_atoms(&mut self) -> CommandResult {
        let cmd = Command::ListAtoms {
            workspace: self.name.clone(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("list_atoms failed: {e}"))
    }

    /// Shorthand: get a vertex by index.
    pub fn get_vertex(&mut self, index: usize) -> CommandResult {
        let cmd = Command::GetVertex {
            workspace: self.name.clone(),
            index,
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("get_vertex({index}) failed: {e}"))
    }

    /// Shorthand: list all vertices.
    pub fn list_vertices(&mut self) -> CommandResult {
        let cmd = Command::ListVertices {
            workspace: self.name.clone(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("list_vertices failed: {e}"))
    }

    /// Shorthand: validate graph integrity.
    pub fn validate_graph(&mut self) -> CommandResult {
        let cmd = Command::ValidateGraph {
            workspace: self.name.clone(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("validate_graph failed: {e}"))
    }

    /// Shorthand: get workspace statistics.
    pub fn get_statistics(&mut self) -> CommandResult {
        let cmd = Command::GetStatistics {
            workspace: self.name.clone(),
        };
        self.exec(cmd)
            .unwrap_or_else(|e| panic!("get_statistics failed: {e}"))
    }

    /// Return the base directory for this workspace.
    pub fn base_dir(&self) -> PathBuf {
        self._temp_dir.path().to_path_buf()
    }
}

// -----------------------------------------------------------------------
// Assertion helpers
// -----------------------------------------------------------------------

/// Extract the text string from a `CommandResult::Text` variant.
///
/// # Panics
///
/// Panics if the result is not `Text`.
pub fn unwrap_text(result: &CommandResult) -> &str {
    match result {
        CommandResult::Text(s) => s.as_str(),
        other => panic!("expected CommandResult::Text, got {other:?}"),
    }
}

/// Extract atom info from a `CommandResult::AtomInfo` variant.
pub fn unwrap_atom_info(result: &CommandResult) -> &context_api::commands::AtomInfo {
    match result {
        CommandResult::AtomInfo(info) => info,
        other => panic!("expected CommandResult::AtomInfo, got {other:?}"),
    }
}

/// Extract the atom list from a `CommandResult::AtomInfoList` variant.
pub fn unwrap_atom_list(result: &CommandResult) -> &[context_api::commands::AtomInfo] {
    match result {
        CommandResult::AtomInfoList { atoms } => atoms.as_slice(),
        other => panic!("expected CommandResult::AtomInfoList, got {other:?}"),
    }
}

/// Extract vertex info from a `CommandResult::VertexInfo` variant.
pub fn unwrap_vertex_info(result: &CommandResult) -> &context_api::commands::VertexInfo {
    match result {
        CommandResult::VertexInfo(info) => info,
        other => panic!("expected CommandResult::VertexInfo, got {other:?}"),
    }
}

/// Extract insert result from a `CommandResult::InsertResult` variant.
pub fn unwrap_insert_result(result: &CommandResult) -> &context_api::commands::InsertResult {
    match result {
        CommandResult::InsertResult(info) => info,
        other => panic!("expected CommandResult::InsertResult, got {other:?}"),
    }
}

/// Extract the read tree from a `CommandResult::ReadTree` variant.
pub fn unwrap_read_tree(result: &CommandResult) -> &context_api::commands::ReadTree {
    match result {
        CommandResult::ReadTree(tree) => tree,
        other => panic!("expected CommandResult::ReadTree, got {other:?}"),
    }
}
```

### `common/mod.rs`

```rust
pub mod helpers;
```

### `integration/mod.rs`

```rust
mod atom_tests;
mod basic_read_tests;
mod dedup_tests;
mod file_input_tests;
mod repl_tests;
mod edge_case_tests;
```

### `cli_integration.rs`

```rust
//! Crate-level integration test entry point.
//!
//! Imports all test modules from integration/ and common/.
#[path = "common/mod.rs"]
mod common;

#[path = "integration/mod.rs"]
mod integration;
```

---

## Test Categories

### Category 1: Atom Management (6 tests)

**Level:** API-level (calls `execute()` directly)
**Dependencies:** None (atoms are the simplest unit)

| Test Name | Description | Expected Behavior |
|-----------|-------------|-------------------|
| `atom_create_basic` | Create atoms 'a', 'b', 'c' individually | Each returns `AtomInfo` with correct character and unique index |
| `atom_create_unicode` | Create unicode atoms (emoji 🎯, CJK 漢字) | Each character is a valid atom; indices are unique |
| `atom_create_duplicate` | Create atom 'a' twice | Second call returns the same index as the first (no duplicate vertex) |
| `atom_list_all` | List all atoms after creating 'a', 'b', 'c' | Returns 3 atoms; all present with correct characters |
| `atom_get_by_char` | Get atom by character after creation | Returns `AtomInfo` with matching character and the same index as when created |
| `atom_auto_create_on_insert` | Insert "abc" via `InsertSequence` | Atoms 'a', 'b', 'c' exist in workspace afterwards; verified via `ListAtoms` |

#### Example Test — `atom_create_basic`

```rust
#[test]
fn atom_create_basic() {
    let mut ws = TestWorkspace::new("atom-basic");

    let r_a = ws.add_atom('a');
    let r_b = ws.add_atom('b');
    let r_c = ws.add_atom('c');

    let a = unwrap_atom_info(&r_a);
    let b = unwrap_atom_info(&r_b);
    let c = unwrap_atom_info(&r_c);

    assert_eq!(a.character, 'a');
    assert_eq!(b.character, 'b');
    assert_eq!(c.character, 'c');

    // Each atom must have a unique index
    assert_ne!(a.index, b.index);
    assert_ne!(b.index, c.index);
    assert_ne!(a.index, c.index);
}
```

#### Example Test — `atom_create_duplicate`

```rust
#[test]
fn atom_create_duplicate() {
    let mut ws = TestWorkspace::new("atom-dup");

    let first = ws.add_atom('x');
    let second = ws.add_atom('x');

    let first_info = unwrap_atom_info(&first);
    let second_info = unwrap_atom_info(&second);

    // Must return the same vertex — no duplicate created
    assert_eq!(first_info.index, second_info.index);
    assert_eq!(first_info.character, second_info.character);
}
```

#### Example Test — `atom_auto_create_on_insert`

```rust
#[test]
fn atom_auto_create_on_insert() {
    let mut ws = TestWorkspace::new("atom-auto");

    // Insert "abc" — should auto-create atoms a, b, c
    ws.insert_text("abc");

    let atoms_result = ws.list_atoms();
    let atoms = unwrap_atom_list(&atoms_result);

    let chars: Vec<char> = atoms.iter().map(|a| a.character).collect();
    assert!(chars.contains(&'a'), "atom 'a' missing after insert");
    assert!(chars.contains(&'b'), "atom 'b' missing after insert");
    assert!(chars.contains(&'c'), "atom 'c' missing after insert");
}
```

---

### Category 2: Basic Read (8 tests)

**Level:** API-level
**Dependencies:** Atom management must work (Category 1)

| Test Name | Description | Expected Behavior |
|-----------|-------------|-------------------|
| `read_single_atom` | Insert "a", read vertex for 'a' | `ReadAsText` returns "a"; `ReadPattern` shows a leaf node (width=1) |
| `read_known_pattern` | Insert "abc", then `ReadPattern` on the root | Tree has 3 leaves: a, b, c |
| `read_unknown_text` | Read text "xyz" in an empty workspace (via `InsertSequence`) | Auto-creates atoms x, y, z; creates root token; `ReadAsText` returns "xyz" |
| `read_mixed_known_unknown` | Insert "ab", then insert "abcd" | "ab" prefix is reused; "cd" is new; root token reads back as "abcd" |
| `read_repeated_pattern` | Insert "abab" when "ab" is already known | "ab" token is reused; root decomposes to [ab, ab] or equivalent |
| `read_produces_decomposition_tree` | Insert "hello", `ReadPattern` on root | Returns a `ReadTree` with correct depth and leaf structure |
| `read_text_output` | Insert "hello", get root index, `ReadAsText` | Returns the string "hello" exactly |
| `read_empty_string` | Attempt `InsertSequence` with "" | Returns error or empty result — no crash, no vertex created |

---

### Category 3: Deduplication / Shared Substrings (8 tests)

**Level:** API-level
**Dependencies:** Basic read must work (Category 2)
**Note:** Many of these tests will initially fail due to context-read overlap handling bugs (~29/60 tests failing). Each failure is expected and will be documented in `FAILING_TESTS.md`.

| Test Name | Description | Expected Behavior |
|-----------|-------------|-------------------|
| `dedup_exact_match` | Insert "abc" twice | Both return the same root vertex index |
| `dedup_shared_prefix` | Insert "abc" then "abd" | "ab" is a shared token reachable from both roots |
| `dedup_shared_suffix` | Insert "xbc" then "abc" | "bc" is a shared token reachable from both roots |
| `dedup_substring` | Insert "abcde" then "bcd" | "bcd" is a reachable subgraph within the first insertion's graph |
| `dedup_overlapping_patterns` | Insert "abc", then insert "abcabc" | "abc" token is reused within the "abcabc" decomposition |
| `dedup_no_duplicate_vertices` | Insert "abc", "abd", "abe" | Vertex count is strictly less than 3×3 atoms + 3 roots (shared "ab" reduces count) |
| `dedup_insert_then_read` | Insert "hello", read back via `ReadAsText` | Returns "hello" — Complete match, no re-indexing needed |
| `dedup_multiple_decompositions` | Insert "ab", "bc", then "abc" | Root for "abc" may decompose as [a, bc] or [ab, c] — at least one decomposition exists |

---

### Category 4: File/Stdin Input (4 tests)

**Level:** API-level for content verification; CLI-level for `--file` flag
**Dependencies:** Basic read (Category 2); CLI changes from `PLAN_CLI_READ_UX.md`
**Note:** These tests depend on CLI modifications that add `--file` support. They should be written now with correct expectations but may be blocked until the CLI changes land.

| Test Name | Description | Expected Behavior |
|-----------|-------------|-------------------|
| `file_read_basic` | Write "hello world" to a temp file, read via API | Root token created; `ReadAsText` returns "hello world" |
| `file_read_unicode` | Write "こんにちは世界" to a temp file, read | Each character is an atom; text reads back correctly |
| `file_read_empty` | Read an empty file | Graceful handling — no crash, no vertex created, informative result |
| `file_read_nonexistent` | Attempt to read from a nonexistent path | Returns `ApiError` (file not found); workspace is not corrupted |

---

### Category 5: REPL Integration (6 tests)

**Level:** CLI-level (invokes `context-cli` binary via `std::process::Command`)
**Dependencies:** All API-level categories; CLI changes from `PLAN_CLI_READ_UX.md`
**Note:** These tests shell out to the compiled binary. They require the binary to be built (`cargo build -p context-cli`) before running.

| Test Name | Description | Expected Behavior |
|-----------|-------------|-------------------|
| `repl_read_numeric` | Send `read 5` to REPL via stdin | Dispatches `ReadPattern` for vertex 5; outputs decomposition tree or error if index invalid |
| `repl_read_text` | Send `read hello` to REPL | Dispatches `ReadSequence` (or `InsertSequence` + `ReadPattern`); outputs decomposition |
| `repl_text_numeric` | Send `text 5` to REPL | Dispatches `ReadAsText` for vertex 5; outputs the concatenated text |
| `repl_search_text` | Send `search hello` to REPL | Dispatches `SearchSequence`; outputs search results |
| `repl_search_numeric` | Send `search 5` to REPL | Dispatches `SearchPattern` for vertex 5; outputs search results |
| `repl_insert_then_read` | Send `insert hello` then `read hello` | Round-trip: insert creates token, read finds it |

#### Example CLI-Level Test — `repl_insert_then_read`

```rust
use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn repl_insert_then_read() {
    // Ensure binary is built
    let bin = env!("CARGO_BIN_EXE_context-cli");

    let mut child = Command::new(bin)
        .arg("--workspace-dir")
        .arg(tempfile::tempdir().unwrap().path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start context-cli");

    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "create repl-test").unwrap();
    writeln!(stdin, "insert hello").unwrap();
    writeln!(stdin, "read hello").unwrap();
    writeln!(stdin, "exit").unwrap();

    let output = child.wait_with_output().expect("failed to read output");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        output.status.success(),
        "context-cli exited with error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // After insert + read, the output should contain the decomposition
    // of "hello" — at minimum the word itself should appear in output
    assert!(
        stdout.contains("hello"),
        "expected 'hello' in REPL output, got:\n{stdout}"
    );
}
```

---

### Category 6: Edge Cases & Error Handling (6 tests)

**Level:** API-level
**Dependencies:** Workspace lifecycle must work

| Test Name | Description | Expected Behavior |
|-----------|-------------|-------------------|
| `error_read_no_workspace` | `ReadPattern` with a workspace name that was never created | Returns `ApiError::WorkspaceNotFound` (or similar) |
| `error_read_invalid_index` | `ReadPattern` with index `999999` in a workspace with only a few vertices | Returns `ApiError` indicating vertex not found |
| `error_read_closed_workspace` | Create workspace, close it, then attempt `ReadPattern` | Returns `ApiError::WorkspaceNotOpen` (or similar) |
| `edge_single_char` | Insert "x", read back | Root is the atom itself; `ReadAsText` returns "x" |
| `edge_long_text` | Insert a 1000-character string ("ab" repeated 500 times) | Completes without panic or timeout; `ReadAsText` returns original string; vertex count << 1000 (deduplication) |
| `edge_repeated_single_char` | Insert "aaaa" with only atom 'a' known | Root token created; `ReadAsText` returns "aaaa"; decomposition uses 'a' repeatedly |

---

## Execution Steps

### Phase 1: Preparation

- [ ] **Step 1: Create directory structure**
  - Create `tools/context-cli/tests/integration/`
  - Create `tools/context-cli/tests/common/`
  - Add `tempfile = "3"` to `[dev-dependencies]` in `tools/context-cli/Cargo.toml`
  - Verify: `cargo check -p context-cli --tests`

- [ ] **Step 2: Create `common/helpers.rs` with test utility functions**
  - Implement `TestWorkspace` struct with `new()`, `exec()`, and all shorthands
  - Implement assertion helpers (`unwrap_text`, `unwrap_atom_info`, etc.)
  - Create `common/mod.rs` exporting `helpers`
  - Create `cli_integration.rs` entry point
  - Verify: `cargo test -p context-cli --test cli_integration --no-run`

### Phase 2: Core Tests (API-Level)

- [ ] **Step 3: Write Category 1 tests — Atom Management (6 tests)**
  - File: `integration/atom_tests.rs`
  - Tests: `atom_create_basic`, `atom_create_unicode`, `atom_create_duplicate`, `atom_list_all`, `atom_get_by_char`, `atom_auto_create_on_insert`
  - These should all pass (atom management is stable)
  - Verify: `cargo test -p context-cli --test cli_integration atom_`

- [ ] **Step 4: Write Category 2 tests — Basic Read (8 tests)**
  - File: `integration/basic_read_tests.rs`
  - Tests: `read_single_atom`, `read_known_pattern`, `read_unknown_text`, `read_mixed_known_unknown`, `read_repeated_pattern`, `read_produces_decomposition_tree`, `read_text_output`, `read_empty_string`
  - Expected: Simple linear reads pass; `read_repeated_pattern` may fail (context-read bug)
  - Verify: `cargo test -p context-cli --test cli_integration read_`

- [ ] **Step 5: Write Category 3 tests — Deduplication (8 tests)**
  - File: `integration/dedup_tests.rs`
  - Tests: `dedup_exact_match`, `dedup_shared_prefix`, `dedup_shared_suffix`, `dedup_substring`, `dedup_overlapping_patterns`, `dedup_no_duplicate_vertices`, `dedup_insert_then_read`, `dedup_multiple_decompositions`
  - Expected: Several will fail due to context-read overlap handling (cursor advancement, `append_to_pattern`)
  - Verify: `cargo test -p context-cli --test cli_integration dedup_`

### Phase 3: Extended Tests

- [ ] **Step 6: Write Category 4 tests — File Input (4 tests)**
  - File: `integration/file_input_tests.rs`
  - Tests: `file_read_basic`, `file_read_unicode`, `file_read_empty`, `file_read_nonexistent`
  - Note: May be blocked until CLI `--file` flag is implemented
  - For now, test via API-level `InsertSequence` with file contents read in the test itself
  - Verify: `cargo test -p context-cli --test cli_integration file_`

- [ ] **Step 7: Write Category 5 tests — REPL Integration (6 tests)**
  - File: `integration/repl_tests.rs`
  - Tests: `repl_read_numeric`, `repl_read_text`, `repl_text_numeric`, `repl_search_text`, `repl_search_numeric`, `repl_insert_then_read`
  - Note: May be blocked until REPL smart parsing is implemented
  - Uses `std::process::Command` to invoke the `context-cli` binary
  - Verify: `cargo test -p context-cli --test cli_integration repl_`

- [ ] **Step 8: Write Category 6 tests — Edge Cases (6 tests)**
  - File: `integration/edge_case_tests.rs`
  - Tests: `error_read_no_workspace`, `error_read_invalid_index`, `error_read_closed_workspace`, `edge_single_char`, `edge_long_text`, `edge_repeated_single_char`
  - Error handling tests should pass; `edge_long_text` and `edge_repeated_single_char` may fail (dedup bugs)
  - Verify: `cargo test -p context-cli --test cli_integration edge_` and `cargo test -p context-cli --test cli_integration error_`

### Phase 4: Failure Documentation

- [ ] **Step 9: Run all tests, create `FAILING_TESTS.md`**
  - Run: `cargo test -p context-cli --test cli_integration 2>&1 | tee test_results.txt`
  - For each failing test, identify the root cause (context-read cursor bug, insert `already_existed` bug, etc.)
  - Write `FAILING_TESTS.md` with the failure → root cause mapping (see format below)

- [ ] **Step 10: Annotate failing tests with tracking comments**
  - Add `// KNOWN_FAILURE: <root_cause_id>` comments to each failing test
  - Optionally create a custom `expect_failure!` macro or use `#[ignore]` with a reason string for tests that panic/hang
  - Do NOT use `#[should_panic]` to mask real bugs — only for tests that intentionally test error paths
  - For tests that are expected to fail due to known bugs but don't panic (they return wrong results), leave them as regular `#[test]` so CI shows the failure count

### Phase 5: Documentation

- [ ] **Step 11: Update plan index**
  - Add this plan to `agents/plans/INDEX.md`
  - Verify: `FAILING_TESTS.md` exists and is populated

---

## FAILING_TESTS.md Format

Template for documenting known failures:

```markdown
# Failing Tests Tracker

> Auto-generated after running: `cargo test -p context-cli --test cli_integration`
> Last updated: YYYY-MM-DD

## Summary

| Status | Count |
|--------|-------|
| ✅ Passing | NN |
| ❌ Failing | NN |
| ⏭️ Ignored | NN |
| **Total** | **38** |

## Root Causes

| ID | Root Cause | Crate | Affected Tests |
|----|-----------|-------|----------------|
| RC-1 | Cursor advancement not working after `insert_or_get_complete` | `context-read` | dedup_shared_prefix, dedup_overlapping_patterns, ... |
| RC-2 | `append_to_pattern` destroys vertices (modifies width in-place) | `context-trace` | dedup_shared_suffix, dedup_substring, ... |
| RC-3 | Search queue cleared prematurely after first match | `context-search` | dedup_multiple_decompositions, ... |
| RC-4 | Insert reports `already_existed=true` incorrectly | `context-api` | dedup_exact_match (if wrong variant returned), ... |
| RC-5 | ReadAsText returns truncated text | `context-api` | read_text_output (for long strings), ... |
| RC-6 | REPL rejects non-numeric read input | `context-cli` | repl_read_text, repl_search_text, ... |
| RC-7 | No `--file` CLI flag yet | `context-cli` | file_read_basic, file_read_unicode, ... |

## Failure Details

### ❌ `dedup_shared_prefix`

- **Root Cause:** RC-1 (cursor advancement)
- **Expected:** "abc" and "abd" share an "ab" token
- **Actual:** Each string creates independent atoms; no shared intermediate token
- **Fix Plan:** `PLAN_INSERT_NEXT_MATCH.md` (WI-1: Redesign CursorCtx)
- **Blocking:** Phase 3a foundation fixes

### ❌ `dedup_overlapping_patterns`

- **Root Cause:** RC-1 + RC-2
- **Expected:** "abcabc" reuses "abc" token
- **Actual:** Panics in `append_to_pattern` or produces wrong vertex count
- **Fix Plan:** `PLAN_APPEND_TO_PATTERN_FIX.md` + `PLAN_INSERT_NEXT_MATCH.md`
- **Blocking:** Phase 3a foundation fixes

<!-- ... one section per failing test ... -->
```

---

## Test Execution Strategy

### Parallelism

Rust's default test harness runs tests in parallel (one thread per test). Because each test creates its own `TestWorkspace` backed by an independent `TempDir`, **all API-level tests can run in parallel safely**.

| Category | Parallel-Safe? | Notes |
|----------|---------------|-------|
| 1 — Atom Management | ✅ Yes | Each test has isolated workspace |
| 2 — Basic Read | ✅ Yes | Each test has isolated workspace |
| 3 — Deduplication | ✅ Yes | Each test has isolated workspace |
| 4 — File Input | ✅ Yes | Each test creates its own temp files |
| 5 — REPL Integration | ⚠️ Caution | Each test spawns a separate process; may hit port/resource limits if many run simultaneously |
| 6 — Edge Cases | ✅ Yes | Each test has isolated workspace |

### Timeout Strategy

For tests that may hang (e.g., due to context-read infinite loops on broken cursor advancement):

```rust
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

/// Run a closure with a timeout. Returns None if the closure doesn't
/// complete within the given duration.
pub fn with_timeout<T: Send + 'static>(
    duration: Duration,
    f: impl FnOnce() -> T + Send + 'static,
) -> Option<T> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });
    rx.recv_timeout(duration).ok()
}
```

Usage in tests that may hang:

```rust
#[test]
fn edge_long_text() {
    let result = with_timeout(Duration::from_secs(30), || {
        let mut ws = TestWorkspace::new("edge-long");
        let text: String = "ab".repeat(500);
        ws.insert_text(&text);
        let stats = ws.get_statistics();
        (ws, stats)
    });

    assert!(result.is_some(), "Test timed out after 30s — possible infinite loop in context-read");
    // ... further assertions on result ...
}
```

### Recommended Test Invocations

```bash
# Run all integration tests
cargo test -p context-cli --test cli_integration

# Run a single category
cargo test -p context-cli --test cli_integration atom_
cargo test -p context-cli --test cli_integration read_
cargo test -p context-cli --test cli_integration dedup_
cargo test -p context-cli --test cli_integration file_
cargo test -p context-cli --test cli_integration repl_
cargo test -p context-cli --test cli_integration edge_
cargo test -p context-cli --test cli_integration error_

# Run a single test
cargo test -p context-cli --test cli_integration dedup_exact_match -- --exact

# Run with output (for debugging failures)
cargo test -p context-cli --test cli_integration -- --nocapture

# Run only ignored (known-blocked) tests
cargo test -p context-cli --test cli_integration -- --ignored
```

---

## Validation

### Success Criteria

- [ ] **Harness compiles:** `cargo test -p context-cli --test cli_integration --no-run` succeeds
- [ ] **All 38 tests exist:** `cargo test -p context-cli --test cli_integration -- --list` shows 38+ tests
- [ ] **Category 1 passes:** All 6 atom tests pass (atom management is stable)
- [ ] **Category 6 error tests pass:** `error_read_no_workspace`, `error_read_invalid_index`, `error_read_closed_workspace` all pass
- [ ] **No unexpected panics:** Tests that fail do so with assertion errors, not panics or hangs
- [ ] **`FAILING_TESTS.md` exists:** Every failing test has a documented root cause
- [ ] **Root causes link to fix plans:** Each RC-* entry references the plan that will fix it
- [ ] **Graph integrity holds:** `validate_graph` passes in every test that successfully creates a workspace

### Verification Command

```bash
cargo test -p context-cli --test cli_integration 2>&1 | tail -5
```

Expected output (approximate):

```
test result: FAILED. 26 passed; 12 failed; 0 ignored; 0 measured; 0 filtered out
```

The exact numbers will depend on which context-read fixes have landed. The key invariant is: **every failure is documented in `FAILING_TESTS.md`**.

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Context-read panics crash the test process | High | High | Use `std::panic::catch_unwind` in `with_timeout` wrapper; mark crash-prone tests with `#[ignore]` and a reason |
| Context-read infinite loops block test runner | Medium | High | Apply 30-second timeouts via `with_timeout` helper for dedup and long-text tests |
| `CommandResult` enum variants change during development | Medium | Medium | Assertion helpers centralize variant matching; only `helpers.rs` needs updating |
| REPL output format changes break CLI-level tests | Medium | Low | CLI tests assert on key substrings, not exact output; use `contains()` over `eq()` |
| Temp directory cleanup fails on Windows (file locking) | Low | Low | `TempDir` handles cleanup on drop; tests that spawn child processes must `wait()` before dropping |
| API types don't implement `Debug` for assertion messages | Low | Medium | Helper functions provide descriptive panic messages with context |
| Parallel test execution causes resource exhaustion | Low | Low | REPL tests (Category 5) can be marked `#[serial]` using the `serial_test` crate if needed |

---

## Related Documents

| Document | Relationship |
|----------|-------------|
| [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) | Parent plan — this test suite is Phase 3d/4 |
| [`20260218_PLAN_CONTEXT_READ_COMPLETION.md`](20260218_PLAN_CONTEXT_READ_COMPLETION.md) | Root cause analysis for context-read failures (RC-1, RC-2) |
| [`20251206_PLAN_END_TO_END_TEST_REGISTRY.md`](20251206_PLAN_END_TO_END_TEST_REGISTRY.md) | Prior art — end-to-end test design patterns |
| `PLAN_INSERT_NEXT_MATCH.md` (planned) | Fix for RC-1 (cursor advancement) |
| `PLAN_APPEND_TO_PATTERN_FIX.md` (planned) | Fix for RC-2 (`append_to_pattern` corruption) |
| `PLAN_CLI_READ_UX.md` (planned) | CLI changes needed for Categories 4 & 5 |

---

## Notes

### Questions for User
- Should Category 5 (REPL) tests use `#[ignore]` until CLI smart parsing lands, or should they be written as stubs?
- Is `serial_test` acceptable as a dev-dependency for REPL test serialization, or should we use a simpler approach?

### Deviations from Plan
<!-- Track changes made during execution -->
-

### Lessons Learned
<!-- Post-execution: what would you do differently? -->
-
```

The plan is now created at `context-engine/agents/plans/20260314_PLAN_INTEGRATION_TESTS.md`. Here's a summary of what's in it:

**Structure:** YAML frontmatter with tags, summary, status, parent reference, and design decision IDs — consistent with the project's existing plan format.

**Key sections:**

1. **Objective** — Build the 38+ test integration suite with correct-first philosophy
2. **Context** — References the parent UX improvement plan, design decisions D14/D16, current crate health
3. **Test Philosophy** — Two-level testing (API + CLI), isolated workspaces, known failures documented not hidden
4. **Files Affected** — Complete directory tree for the new test harness under `tools/context-cli/tests/`
5. **Test Helper Design** — Full `TestWorkspace` struct with `new()`, `exec()`, 10+ shorthand methods, and assertion helpers (`unwrap_text`, `unwrap_atom_info`, etc.)
6. **6 Test Categories (38 tests)** — Each with a table of test names, descriptions, and expected behavior, plus 3 fully-coded example tests (`atom_create_basic`, `atom_create_duplicate`, `atom_auto_create_on_insert`)
7. **Execution Steps** — 11 ordered steps across 5 phases (Preparation → Core Tests → Extended Tests → Failure Documentation → Documentation)
8. **FAILING_TESTS.md Format** — Complete template mapping 7 root causes (RC-1 through RC-7) to affected tests with fix plan references
9. **Test Execution Strategy** — Parallelism analysis per category, `with_timeout` helper for hang protection, recommended `cargo test` invocations
10. **Validation** — 8 success criteria plus the expected verification command output
11. **Risks & Mitigations** — 7 risks from panics/infinite loops to Windows file locking, each with a concrete mitigation