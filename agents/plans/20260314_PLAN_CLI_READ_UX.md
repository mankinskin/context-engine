---
tags: `#plan` `#context-api` `#context-cli` `#ux` `#read` `#cli` `#repl`
summary: CLI and API layer changes for Context-Read UX Improvement — new Command variants (ReadSequence, ReadFile), REPL smart parsing, file input support, and summary output format.
status: 📋
phase: 2-design
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
depends_on:
  - PLAN_INSERT_NEXT_MATCH (ReadSequence implementation will eventually use insert_next_match)
decisions: D11, D12, D13
---

# Plan: CLI Read UX — Commands, REPL Smart Parsing & File Input

**Date:** 2026-03-14
**Scope:** Medium (3 crates, 5 files, additive-only changes)
**Crates:** `context-api`, `context-cli`
**Parent Plan:** `20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [Files Affected](#files-affected)
4. [Execution Steps](#execution-steps)
5. [ReadSequence Implementation Design](#readsequence-implementation-design)
6. [Summary Output Format](#summary-output-format)
7. [REPL Smart Parsing Logic](#repl-smart-parsing-logic)
8. [Migration Guide](#migration-guide)
9. [Validation](#validation)
10. [Risks & Mitigations](#risks--mitigations)

---

## 1. Objective

Add `Command::ReadSequence` and `Command::ReadFile` variants to the API, update the CLI with corresponding subcommands and file input flags, and implement REPL smart parsing so `read` accepts numeric indices, text strings, or file paths.

---

## 2. Context

This plan covers **Phase 3b** (CLI & API layer) from the parent plan `20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`. It is one of six design plans produced during Phase 2.

### Parent Plan

The parent plan defines the full Context-Read UX Improvement project across four phases:
- Phase 1: Research ✅ COMPLETE
- Phase 2: Design (this plan is part of it)
- Phase 3: Implement (this plan targets 3b)
- Phase 4: Validate

### Dependency: PLAN_INSERT_NEXT_MATCH

The `ReadSequence` implementation will initially use `ReadCtx::read_sequence` from `context-read`, which internally uses `insert_or_get_complete`. Once `PLAN_INSERT_NEXT_MATCH` lands (renaming `insert_or_get_complete` → `insert_next_match` and adding `InsertOutcome`), `ReadSequence` will benefit from improved return type semantics. However, the CLI/API plumbing in this plan is **independent** — we wire up the command dispatch now and the algorithm improves later.

### Key Design Decisions

| # | Decision | Implication for this plan |
|---|----------|--------------------------|
| D11 | Summary output for `context-cli read` (future: `--tree`, `--json`) | Default output is a compact summary; tree/json are future flags |
| D12 | Read accepts text or token list (smart parsing in CLI/REPL) | REPL `read` must detect numeric vs text input |
| D13 | File input: ordered list (`--file`) or unordered set (`--files`), separate roots | `--file` reads as single ordered sequence; `--files` is future work |

---

## 3. Files Affected

### Primary Changes

| File | Lines | Change |
|------|-------|--------|
| `crates/context-api/src/commands/mod.rs` | L558–565 (Command enum) | Add `ReadSequence` and `ReadFile` variants after existing `ReadAsText` |
| `crates/context-api/src/commands/mod.rs` | L81–254 (WorkspaceApi trait) | Add `read_sequence` and `read_file` trait methods |
| `crates/context-api/src/commands/mod.rs` | L260–463 (WorkspaceApi impl) | Add delegation for new trait methods |
| `crates/context-api/src/commands/mod.rs` | L884–891 (execute dispatch) | Add match arms for `ReadSequence` and `ReadFile` |
| `crates/context-api/src/commands/mod.rs` | L1075–1116 (command_name) | Add name entries for new variants |
| `crates/context-api/src/commands/read.rs` | after L115 | Add `read_sequence` and `read_file` impl methods |
| `crates/context-api/src/error.rs` | L179–191 (ReadError) | Add `FileReadError` and `SequenceTooShort` variants |
| `tools/context-cli/src/main.rs` | L169–182 (CliCommand enum) | Add `ReadSequence` and `ReadFile` subcommands |
| `tools/context-cli/src/main.rs` | L391–591 (execute_subcommand) | Add mapping for new subcommands |
| `tools/context-cli/src/repl.rs` | L455–479 (`read` handler) | Replace numeric-only parsing with smart parsing |
| `tools/context-cli/src/repl.rs` | L838–921 (print_help) | Update help text for `read` command |
| `tools/context-cli/src/output.rs` | L297–330 (print_read_result) | Add summary output path for sequence-based reads |

### Secondary Changes (serde tests, validation)

| File | Lines | Change |
|------|-------|--------|
| `crates/context-api/src/commands/mod.rs` | L1307–1342 (serde tests) | Add serde round-trip tests for new variants |
| `crates/context-api/src/commands/mod.rs` | L1362–1467 (all-variants test) | Add new variants to exhaustive tag test |
| `crates/context-api/src/types.rs` | (no changes needed) | `PatternReadResult` is already suitable for sequence reads |

---

## 4. Execution Steps

### Phase 1: API Foundation (context-api)

#### Step 1: Add `ReadError` variants for new operations

**File:** `crates/context-api/src/error.rs` at L179–191

Add two new variants to `ReadError`:

```rust
// crates/context-api/src/error.rs — inside pub enum ReadError

/// The input text sequence is too short (must be at least 1 character).
#[error("sequence too short: need at least 1 character, got {len}")]
SequenceTooShort { len: usize },

/// Could not read the input file.
#[error("failed to read file '{path}': {reason}")]
FileReadError { path: String, reason: String },
```

**Verification:** `cargo check -p context-api`

---

#### Step 2: Add `Command::ReadSequence` to enum

**File:** `crates/context-api/src/commands/mod.rs` at L562–565 (after `ReadAsText`)

```rust
// crates/context-api/src/commands/mod.rs — inside pub enum Command

/// Read a text sequence through the graph (auto-creates atoms, builds decomposition).
ReadSequence {
    workspace: String,
    text: String,
},
```

**Verification:** `cargo check -p context-api`

---

#### Step 3: Add `Command::ReadFile` to enum

**File:** `crates/context-api/src/commands/mod.rs` — immediately after `ReadSequence`

```rust
// crates/context-api/src/commands/mod.rs — inside pub enum Command

/// Read a file's contents through the graph.
ReadFile {
    workspace: String,
    path: String,
},
```

**Verification:** `cargo check -p context-api`

---

#### Step 4: Add `WorkspaceApi` trait methods

**File:** `crates/context-api/src/commands/mod.rs` at L191–201 (after existing `read_as_text`)

```rust
// crates/context-api/src/commands/mod.rs — inside pub trait WorkspaceApi

fn read_sequence(
    &mut self,
    ws: &str,
    text: &str,
) -> Result<PatternReadResult, ReadError>;

fn read_file(
    &mut self,
    ws: &str,
    path: &str,
) -> Result<PatternReadResult, ReadError>;
```

> **Note:** These methods take `&mut self` because `read_sequence` may auto-create
> atoms (mutating the graph), unlike the existing `read_pattern`/`read_as_text`
> which only traverse.

**Verification:** `cargo check -p context-api` (will fail until impl is added — that's Step 6)

---

#### Step 5: Implement `read_sequence` on `WorkspaceManager`

**File:** `crates/context-api/src/commands/read.rs` — add after existing `read_as_text` impl (after L115)

```rust
// crates/context-api/src/commands/read.rs — inside impl WorkspaceManager

/// Read a text sequence through the graph.
///
/// Each character in the text is ensured to exist as an atom (auto-created
/// if missing). The atom sequence is then passed to `context-read`'s
/// `ReadCtx::read_sequence` to find the largest-match decomposition.
///
/// If `context-read` returns a root token, we build a `PatternReadResult`
/// from it. If it returns `None` (e.g., single-char input), we fall back
/// to returning the atom's read result directly.
///
/// # Errors
///
/// - `ReadError::WorkspaceNotOpen` if the workspace is not currently open.
/// - `ReadError::SequenceTooShort` if the text is empty.
/// - `ReadError::InternalError` on unexpected failures from the read algorithm.
pub fn read_sequence(
    &mut self,
    ws_name: &str,
    text: &str,
) -> Result<PatternReadResult, ReadError> {
    let char_count = text.chars().count();
    if char_count == 0 {
        return Err(ReadError::SequenceTooShort { len: 0 });
    }

    // For single characters, just ensure the atom exists and read it
    if char_count == 1 {
        let ch = text.chars().next().unwrap();
        let ws = self.get_workspace(ws_name).map_err(|_| {
            ReadError::WorkspaceNotOpen {
                workspace: ws_name.to_string(),
            }
        })?;
        let graph_ref = ws.graph_ref();

        // Ensure atom exists
        let atom = context_trace::graph::vertex::atom::Atom::Element(ch);
        let token = match graph_ref.get_atom_index(atom) {
            Ok(idx) => context_trace::graph::vertex::token::Token::new(idx, 1),
            Err(_) => graph_ref.insert_atom(atom),
        };

        // Read the single atom as a pattern
        return self.read_pattern(ws_name, token.index.0);
    }

    // Step (a): Get workspace and graph reference
    let ws = self.get_workspace(ws_name).map_err(|_| {
        ReadError::WorkspaceNotOpen {
            workspace: ws_name.to_string(),
        }
    })?;
    let graph_ref = ws.graph_ref();

    // Step (b): Create ReadCtx with the text (auto-creates missing atoms)
    let mut read_ctx = context_read::context::ReadCtx::new(
        graph_ref.clone(),
        text.chars(),
    );

    // Step (c): Run the read algorithm
    let root_token = read_ctx.read_sequence();

    match root_token {
        Some(token) => {
            // Mark workspace dirty (atoms may have been created)
            let ws = self.get_workspace_mut(ws_name).map_err(|_| {
                ReadError::WorkspaceNotOpen {
                    workspace: ws_name.to_string(),
                }
            })?;
            ws.mark_dirty();

            // Step (d): Build PatternReadResult from the root token
            self.read_pattern(ws_name, token.index.0)
        },
        None => {
            Err(ReadError::InternalError(
                format!("read_sequence returned None for text of length {char_count}")
            ))
        },
    }
}
```

**Key design points:**
- Uses `context_read::context::ReadCtx::new(graph_ref, text.chars())` — the `Chars` impl of `ToNewAtomIndices` calls `graph.new_atom_indices()` which handles both known and unknown atoms
- Delegates to existing `read_pattern` for the final result construction — reuses the tree-building logic
- Single-char inputs are handled as a special case (no need for the full read pipeline)

**Verification:** `cargo check -p context-api` (requires `context-read` in dependencies)

---

#### Step 6: Implement `read_file` on `WorkspaceManager`

**File:** `crates/context-api/src/commands/read.rs` — add after `read_sequence`

```rust
// crates/context-api/src/commands/read.rs — inside impl WorkspaceManager

/// Read a file's contents through the graph.
///
/// Reads the file at `path` to a string, then delegates to `read_sequence`.
///
/// # Errors
///
/// - `ReadError::FileReadError` if the file cannot be read.
/// - All errors from `read_sequence` (workspace not open, internal errors).
pub fn read_file(
    &mut self,
    ws_name: &str,
    path: &str,
) -> Result<PatternReadResult, ReadError> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        ReadError::FileReadError {
            path: path.to_string(),
            reason: e.to_string(),
        }
    })?;

    self.read_sequence(ws_name, &content)
}
```

**Verification:** `cargo check -p context-api`

---

#### Step 7: Add `WorkspaceApi` impl delegation

**File:** `crates/context-api/src/commands/mod.rs` at ~L402–408 (after existing `read_as_text` delegation)

```rust
// crates/context-api/src/commands/mod.rs — inside impl WorkspaceApi for WorkspaceManager

fn read_sequence(
    &mut self,
    ws: &str,
    text: &str,
) -> Result<PatternReadResult, ReadError> {
    WorkspaceManager::read_sequence(self, ws, text)
}

fn read_file(
    &mut self,
    ws: &str,
    path: &str,
) -> Result<PatternReadResult, ReadError> {
    WorkspaceManager::read_file(self, ws, path)
}
```

**Verification:** `cargo check -p context-api`

---

#### Step 8: Add execute dispatch for new commands

**File:** `crates/context-api/src/commands/mod.rs` at ~L891 (after `ReadAsText` dispatch)

```rust
// crates/context-api/src/commands/mod.rs — inside pub fn execute

Command::ReadSequence { workspace, text } => {
    let result = manager.read_sequence(&workspace, &text)?;
    Ok(CommandResult::ReadResult(result))
},
Command::ReadFile { workspace, path } => {
    let result = manager.read_file(&workspace, &path)?;
    Ok(CommandResult::ReadResult(result))
},
```

**Note:** Both new commands return `CommandResult::ReadResult(PatternReadResult)` — the same variant used by `ReadPattern`. No new `CommandResult` variant is needed at this stage. The existing `print_read_result` output function handles this automatically.

**Verification:** `cargo check -p context-api`

---

#### Step 9: Add `command_name` entries

**File:** `crates/context-api/src/commands/mod.rs` at ~L1099 (after `ReadAsText` entry)

```rust
// crates/context-api/src/commands/mod.rs — inside impl Command::command_name

Command::ReadSequence { .. } => "read_sequence",
Command::ReadFile { .. } => "read_file",
```

**Verification:** `cargo check -p context-api`

---

### Phase 2: CLI Layer (context-cli)

#### Step 10: Add CLI subcommands for `ReadSequence` and `ReadFile`

**File:** `tools/context-cli/src/main.rs` at ~L182 (after `ReadAsText`)

```rust
// tools/context-cli/src/main.rs — inside enum CliCommand

/// Read a text sequence through the graph (auto-creates atoms, builds decomposition).
#[command(name = "read-sequence")]
ReadSequence {
    /// Name of the open workspace.
    workspace: String,
    /// The text to read through the graph.
    text: String,
},

/// Read a file's contents through the graph.
#[command(name = "read-file")]
ReadFile {
    /// Name of the open workspace.
    workspace: String,
    /// Path to the file to read.
    path: String,
},
```

**Verification:** `cargo check -p context-cli`

---

#### Step 11: Add `execute_subcommand` mapping for new CLI commands

**File:** `tools/context-cli/src/main.rs` — inside `execute_subcommand` (after `ReadAsText` mapping at ~L462)

```rust
// tools/context-cli/src/main.rs — inside execute_subcommand match

CliCommand::ReadSequence { workspace, text } =>
    Command::ReadSequence { workspace, text },
CliCommand::ReadFile { workspace, path } =>
    Command::ReadFile { workspace, path },
```

**Verification:** `cargo check -p context-cli`

---

#### Step 12: Add `--file` flag to existing `ReadPattern` CLI subcommand (future enhancement)

> **Deferred.** The `read-file` subcommand covers this use case for now. Adding
> `--file <path>` as an alternative flag on the `read-pattern` subcommand is a
> future ergonomic improvement that can be added without breaking changes.

---

### Phase 3: REPL Smart Parsing

#### Step 13: Update REPL `read` handler with smart parsing

**File:** `tools/context-cli/src/repl.rs` at L455–479

Replace the current numeric-only `read` handler:

```rust
// tools/context-cli/src/repl.rs — inside execute_repl_line, "read" arm

"read" =>
    if let Some(ws) = require_workspace(current_ws) {
        if parts.len() < 2 {
            eprintln!(
                "Usage: read <index>    Read vertex by index\n\
                 \x20      read <text>     Read text sequence through graph\n\
                 \x20      read --file <path>  Read file contents through graph"
            );
        } else if parts[1] == "--file" {
            // File input mode
            if let Some(path) = parts.get(2) {
                execute_and_print(
                    manager,
                    Command::ReadFile {
                        workspace: ws,
                        path: path.to_string(),
                    },
                    *tracing_enabled,
                    current_ws.as_deref(),
                )
                .ok();
            } else {
                eprintln!("Usage: read --file <path>");
            }
        } else if parts.len() == 2 && parts[1].parse::<usize>().is_ok() {
            // Single numeric argument → ReadPattern (backwards compatible)
            let index = parts[1].parse::<usize>().unwrap();
            execute_and_print(
                manager,
                Command::ReadPattern {
                    workspace: ws,
                    index,
                },
                *tracing_enabled,
                current_ws.as_deref(),
            )
            .ok();
        } else {
            // Non-numeric or multi-word → ReadSequence
            let text = parts[1..].join(" ");
            execute_and_print(
                manager,
                Command::ReadSequence {
                    workspace: ws,
                    text,
                },
                *tracing_enabled,
                current_ws.as_deref(),
            )
            .ok();
        }
    },
```

**Smart parsing logic:**
1. `read 42` → numeric → `Command::ReadPattern { index: 42 }` (backwards compatible)
2. `read hello world` → text → `Command::ReadSequence { text: "hello world" }`
3. `read hello` → single non-numeric → `Command::ReadSequence { text: "hello" }`
4. `read --file data.txt` → file flag → `Command::ReadFile { path: "data.txt" }`

**Verification:** `cargo check -p context-cli`, then manual REPL testing

---

#### Step 14: Update REPL help text

**File:** `tools/context-cli/src/repl.rs` at L886–888 (read commands section)

Replace:
```
println!("  read <index>         Read a vertex as a decomposition tree");
println!("  text <index>         Read a vertex as concatenated leaf text");
```

With:
```rust
println!("  read <index>         Read a vertex as a decomposition tree");
println!("  read <text>          Read a text sequence through the graph");
println!("  read --file <path>   Read a file's contents through the graph");
println!("  text <index>         Read a vertex as concatenated leaf text");
```

**Verification:** Run REPL, type `help`, verify output

---

### Phase 4: Output & Tests

#### Step 15: Design summary output format for read results

The existing `print_read_result` in `output.rs` already prints:
- Root token info (index, label, width)
- Full text
- Decomposition tree

For sequence-based reads, we want to add a **summary line** showing what happened.
This is a future enhancement tracked by D11. For now, the existing tree output is
adequate because `CommandResult::ReadResult` is reused.

> **Deferred enhancement:** Add `--summary`, `--tree`, `--json`, `--verbose` flags.
> The `CommandResult::ReadResult` variant already contains all data needed; it's
> purely an output formatting concern.

---

#### Step 16: Add serde round-trip tests for new command variants

**File:** `crates/context-api/src/commands/mod.rs` — in the `#[cfg(test)] mod tests` block

```rust
#[test]
fn command_serde_read_sequence() {
    let cmd = Command::ReadSequence {
        workspace: "ws".into(),
        text: "hello".into(),
    };
    let json = serde_json::to_string(&cmd).unwrap();
    assert!(json.contains("read_sequence"));

    let deser: Command = serde_json::from_str(&json).unwrap();
    match deser {
        Command::ReadSequence { workspace, text } => {
            assert_eq!(workspace, "ws");
            assert_eq!(text, "hello");
        },
        other => panic!("wrong variant: {other:?}"),
    }
}

#[test]
fn command_serde_read_file() {
    let cmd = Command::ReadFile {
        workspace: "ws".into(),
        path: "/tmp/test.txt".into(),
    };
    let json = serde_json::to_string(&cmd).unwrap();
    assert!(json.contains("read_file"));

    let deser: Command = serde_json::from_str(&json).unwrap();
    match deser {
        Command::ReadFile { workspace, path } => {
            assert_eq!(workspace, "ws");
            assert_eq!(path, "/tmp/test.txt");
        },
        other => panic!("wrong variant: {other:?}"),
    }
}
```

---

#### Step 17: Add integration test for `read_sequence` via `execute`

**File:** `crates/context-api/src/commands/mod.rs` — in the `#[cfg(test)] mod tests` block

```rust
#[test]
fn execute_read_sequence_workflow() {
    let (_tmp, mut mgr) = tmp_manager();

    // Create and open workspace
    execute(&mut mgr, Command::CreateWorkspace { name: "ws".into() }).unwrap();

    // Insert some text first to populate the graph
    execute(&mut mgr, Command::InsertSequence {
        workspace: "ws".into(),
        text: "hello".into(),
    }).unwrap();

    // Now read the same text — should find the existing structure
    let result = execute(&mut mgr, Command::ReadSequence {
        workspace: "ws".into(),
        text: "hello".into(),
    }).unwrap();

    match result {
        CommandResult::ReadResult(read) => {
            assert_eq!(read.text, "hello");
            assert_eq!(read.root.width, 5);
        },
        other => panic!("expected ReadResult, got {other:?}"),
    }
}
```

**Verification:** `cargo test -p context-api execute_read_sequence_workflow`

---

#### Step 18: Add unit test for `read_sequence` in `read.rs`

**File:** `crates/context-api/src/commands/read.rs` — in the existing test module

```rust
#[test]
fn read_sequence_basic() {
    let (_tmp, mut mgr) = setup("ws");

    // read_sequence should auto-create atoms and produce a result
    let result = mgr.read_sequence("ws", "ab").unwrap();
    assert_eq!(result.text, "ab");
    assert_eq!(result.root.width, 2);
}

#[test]
fn read_sequence_single_char() {
    let (_tmp, mut mgr) = setup("ws");

    let result = mgr.read_sequence("ws", "x").unwrap();
    assert_eq!(result.text, "x");
    assert_eq!(result.root.width, 1);
    assert!(result.tree.children.is_empty(), "atom should have no children");
}

#[test]
fn read_sequence_empty_returns_error() {
    let (_tmp, mut mgr) = setup("ws");

    let err = mgr.read_sequence("ws", "").unwrap_err();
    match err {
        crate::error::ReadError::SequenceTooShort { len } => {
            assert_eq!(len, 0);
        },
        other => panic!("expected SequenceTooShort, got: {other}"),
    }
}

#[test]
fn read_sequence_after_insert_reuses_structure() {
    let (_tmp, mut mgr) = setup("ws");

    // Insert first
    let insert_result = mgr.insert_sequence("ws", "hello").unwrap();

    // Read same text — should find the same root
    let read_result = mgr.read_sequence("ws", "hello").unwrap();
    assert_eq!(read_result.text, "hello");
    assert_eq!(read_result.root.index, insert_result.token.index);
}

#[test]
fn read_sequence_workspace_not_open() {
    let (_tmp, mut mgr) = setup("ws");

    let err = mgr.read_sequence("nonexistent", "hello").unwrap_err();
    match err {
        crate::error::ReadError::WorkspaceNotOpen { workspace } => {
            assert_eq!(workspace, "nonexistent");
        },
        other => panic!("expected WorkspaceNotOpen, got: {other}"),
    }
}

#[test]
fn read_file_not_found() {
    let (_tmp, mut mgr) = setup("ws");

    let err = mgr.read_file("ws", "/nonexistent/path.txt").unwrap_err();
    match err {
        crate::error::ReadError::FileReadError { path, .. } => {
            assert_eq!(path, "/nonexistent/path.txt");
        },
        other => panic!("expected FileReadError, got: {other}"),
    }
}
```

**Verification:** `cargo test -p context-api -- read_sequence`

---

## 5. ReadSequence Implementation Design

### How `context-read` Wires into `context-api`

The `context-read` crate exposes `ReadCtx` which is the core read algorithm context:

```
ReadCtx::new(graph: HypergraphRef, seq: impl ToNewAtomIndices) -> Self
ReadCtx::read_sequence(&mut self) -> Option<Token>
```

The `ToNewAtomIndices` trait has an implementation for `Chars<'_>` that calls
`graph.new_atom_indices(chars)`, which:
1. Looks up each character as an atom in the graph
2. Returns `NewAtomIndex::Known(VertexIndex)` for existing atoms
3. Returns `NewAtomIndex::New(VertexIndex)` for newly created atoms

This means `ReadCtx::new(graph_ref, text.chars())` **automatically handles atom creation** — we don't need a separate `ensure_atoms` step like `insert_sequence` does.

### Data Flow

```
User input: "hello world"
     │
     ▼
Command::ReadSequence { workspace: "ws", text: "hello world" }
     │
     ▼
WorkspaceManager::read_sequence(ws_name, text)
     │
     ├─ (1) Get workspace, clone graph_ref
     │
     ├─ (2) ReadCtx::new(graph_ref, text.chars())
     │       └─ Internally: graph.new_atom_indices('h','e','l','l','o',' ','w','o','r','l','d')
     │          └─ Creates atoms for any unknown characters
     │          └─ Returns NewAtomIndices (Known/New for each char)
     │
     ├─ (3) read_ctx.read_sequence()
     │       └─ SegmentIter splits into known/unknown segments
     │       └─ BlockExpansionCtx processes known segments (finds largest matches)
     │       └─ RootManager builds the root token via pattern appending
     │       └─ Returns Option<Token> — the root token of the full read
     │
     ├─ (4) Mark workspace dirty (atoms may have been created)
     │
     └─ (5) self.read_pattern(ws_name, root_token.index)
            └─ Builds PatternReadResult with tree, text, root info
            └─ Returns to caller
```

### Why Reuse `read_pattern` for Result Construction?

The existing `read_pattern` method already handles:
- Building the `ReadNode` tree recursively
- Collecting leaf text
- Creating `TokenInfo` from the graph

By delegating to `read_pattern` after obtaining the root token from `ReadCtx`, we
avoid duplicating this logic and ensure consistent output formatting.

### Atom Creation: `ReadCtx` vs `insert_sequence`

| Aspect | `insert_sequence` (current) | `read_sequence` (new) |
|--------|---------------------------|----------------------|
| Atom creation | Manual: iterates chars, calls `get_atom_index`/`insert_atom` | Automatic: `Chars::to_new_atom_indices` handles it |
| Token resolution | Builds `Vec<Token>` explicitly | `NewAtomIndices` (mixed Known/New) |
| Algorithm | `insert_or_get_complete` (pure insert) | `ReadCtx::read_sequence` (read with expansion) |
| Graph mutation | Only if token is new | Always (atoms + read algorithm may insert) |

---

## 6. Summary Output Format

### Current Output (from `print_read_result`)

```
Root: "hello" (index: 42, width: 5)
Text: "hello"
Tree:
  "hello" [42] (width: 5)
    'h' [0]
    'e' [1]
    'l' [2]
    'l' [2]
    'o' [3]
```

### Proposed Summary Output (future, D11)

For `ReadSequence` results, a compact summary would show:

```
Read "hello world" (11 chars)
  Root: #42 "hello world" (width: 11)
  Atoms: 8 (3 new)
  Tokens: 3 unique
```

With flags:
- `--tree` — shows the full decomposition tree (current default)
- `--json` — outputs `PatternReadResult` as JSON
- `--verbose` — shows both summary and tree
- Default (no flag) — shows summary only

### Implementation Approach

**For this plan:** No output format changes. Both `ReadSequence` and `ReadFile` return
`CommandResult::ReadResult(PatternReadResult)`, which routes to the existing
`print_read_result` function. The tree output is informative and sufficient for
initial launch.

**Future plan:** Add a `--format` flag to CLI subcommands and a `format` parameter
to the REPL `read` command. The `print_read_result` function would dispatch based
on format. This is purely a presentation-layer change.

---

## 7. REPL Smart Parsing Logic

### Pseudocode

```
fn handle_read(parts: &[&str], ws: String):
    if parts.len() < 2:
        print usage
        return

    // Check for --file flag
    if parts[1] == "--file":
        if parts.len() < 3:
            print "Usage: read --file <path>"
            return
        execute Command::ReadFile { workspace: ws, path: parts[2] }
        return

    // Single argument that parses as usize → ReadPattern (backwards compat)
    if parts.len() == 2 AND parts[1].parse::<usize>().is_ok():
        let index = parts[1].parse::<usize>().unwrap()
        execute Command::ReadPattern { workspace: ws, index }
        return

    // Everything else → join as text and ReadSequence
    let text = parts[1..].join(" ")
    execute Command::ReadSequence { workspace: ws, text }
```

### Edge Cases

| Input | Parsing | Command |
|-------|---------|---------|
| `read` | No args | Print usage |
| `read 42` | Single numeric | `ReadPattern { index: 42 }` |
| `read 0` | Single numeric (zero) | `ReadPattern { index: 0 }` |
| `read hello` | Single non-numeric | `ReadSequence { text: "hello" }` |
| `read hello world` | Multiple words | `ReadSequence { text: "hello world" }` |
| `read 42abc` | Single non-numeric (starts with digit) | `ReadSequence { text: "42abc" }` |
| `read 42 43` | Multiple args (both numeric) | `ReadSequence { text: "42 43" }` |
| `read --file data.txt` | File flag | `ReadFile { path: "data.txt" }` |
| `read --file` | File flag, no path | Print usage |

**Note on `read 42 43`:** Two numeric arguments are treated as text, not as token
refs. This matches the `search` command's behavior where single-numeric triggers
pattern search and multi-arg uses token refs. However, for `read`, multi-numeric
token ref resolution is deferred to a future plan (requires `TokenRef` resolution
in the read pipeline).

### Comparison with `search` Smart Parsing

The existing `search` handler (L349–384) uses this logic:
- Single non-numeric → `SearchSequence`
- Multiple args or single numeric → `SearchPattern` (with `TokenRef` resolution)

The `read` handler inverts the multi-arg case:
- Single numeric → `ReadPattern` (by index)
- Everything else → `ReadSequence` (as text)

This is intentional: `read` is primarily text-oriented (D12), while `search`
supports both text and pattern-ref queries equally.

---

## 8. Migration Guide

### No Breaking Changes

All changes in this plan are **additive only**:

1. **New `Command` variants** — `ReadSequence` and `ReadFile` are added alongside existing `ReadPattern` and `ReadAsText` (which remain unchanged)
2. **New `WorkspaceApi` trait methods** — `read_sequence` and `read_file` are added; existing methods are unchanged
3. **New `ReadError` variants** — `SequenceTooShort` and `FileReadError` are added; existing variants are unchanged
4. **REPL behavior** — `read 42` still works exactly as before (numeric → `ReadPattern`); new text input is additive
5. **CLI subcommands** — `read-pattern` and `read-as-text` are unchanged; `read-sequence` and `read-file` are new subcommands
6. **Output format** — Both new commands use the existing `CommandResult::ReadResult` variant, so all existing output formatting works unchanged

### Serde Compatibility

New `Command` variants use `#[serde(tag = "type", rename_all = "snake_case")]` like all existing variants. JSON payloads for the new commands:

```json
{
  "type": "read_sequence",
  "workspace": "ws",
  "text": "hello world"
}
```

```json
{
  "type": "read_file",
  "workspace": "ws",
  "path": "/path/to/file.txt"
}
```

Old clients that don't know about these new variants will get a serde error if
they encounter them, but they will never produce them. This is the expected
behavior for additive enum variants in a locally-used API.

---

## 9. Validation

### Manual Test Scenarios

#### Scenario 1: Basic `read_sequence` via CLI

```bash
# Create and populate workspace
context-cli create test-ws
context-cli insert-sequence test-ws "hello world"

# Read the same text (should reuse existing structure)
context-cli read-sequence test-ws "hello world"
# Expected: Root with width 11, tree showing decomposition

# Read new text (auto-creates atoms, builds new structure)
context-cli read-sequence test-ws "goodbye"
# Expected: Root with width 7, atoms auto-created
```

#### Scenario 2: `read_file` via CLI

```bash
echo "test content" > /tmp/test-read.txt
context-cli read-file test-ws /tmp/test-read.txt
# Expected: Root for "test content\n", decomposition tree shown
```

#### Scenario 3: REPL smart parsing

```bash
context-cli repl
> create test-ws
> insert hello world
> read 0          # → ReadPattern for atom at index 0
> read hello      # → ReadSequence for "hello"
> read hello world  # → ReadSequence for "hello world"
> read --file /tmp/test-read.txt  # → ReadFile
> exit
```

#### Scenario 4: Error handling

```bash
context-cli read-sequence nonexistent-ws "hello"
# Expected: Error: workspace not open: 'nonexistent-ws'

context-cli read-file test-ws /nonexistent/file.txt
# Expected: Error: failed to read file '/nonexistent/file.txt': No such file or directory

context-cli read-sequence test-ws ""
# Expected: Error: sequence too short: need at least 1 character, got 0
```

### Automated Test Commands

```bash
# Unit tests for read_sequence
cargo test -p context-api -- read_sequence

# Unit tests for read_file
cargo test -p context-api -- read_file

# Serde tests for new command variants
cargo test -p context-api -- command_serde_read_sequence
cargo test -p context-api -- command_serde_read_file

# Integration test
cargo test -p context-api -- execute_read_sequence_workflow

# Full API test suite (regression check)
cargo test -p context-api

# CLI compilation check
cargo check -p context-cli
```

---

## 10. Risks & Mitigations

| # | Risk | Likelihood | Impact | Mitigation |
|---|------|-----------|--------|------------|
| R1 | `context-read` has failing tests (29 failures in context-read, 15 in context-search as of parent plan research) | High | Medium | `read_sequence` wraps `ReadCtx::read_sequence` with graceful error handling. If it returns `None`, we return `ReadError::InternalError` with a descriptive message. The caller sees a clean error, not a panic. |
| R2 | `ReadCtx::new` with `Chars` may panic on edge cases (empty iterators, unusual Unicode) | Medium | Medium | Pre-validate input length before constructing `ReadCtx`. Single-char input is handled as a special case without `ReadCtx`. |
| R3 | Atom auto-creation in `ReadCtx` may not properly mark workspace as dirty | Medium | Low | Explicitly call `ws.mark_dirty()` after `read_sequence` completes, regardless of whether new atoms were created. This is safe (worst case: an unnecessary save). |
| R4 | `context-read` dependency not in `context-api`'s `Cargo.toml` | Low | High | Verify `context-read` is listed as a dependency. If not, add `context-read = { path = "../context-read" }` to `crates/context-api/Cargo.toml`. |
| R5 | Large file reads via `read_file` could cause memory issues | Medium | Medium | For this initial implementation, `std::fs::read_to_string` loads the entire file. A future enhancement (from `PLAN_READ_STREAM_DESIGN.md`) will add streaming support via `ReadCtx`'s iterator-based design. |
| R6 | REPL smart parsing ambiguity: user wants to read text that is a number (e.g., `read 404`) | Low | Low | Numeric strings are always interpreted as indices. Users can quote or use `read-sequence` for text that happens to be numeric. Document this in help text. |
| R7 | `WorkspaceApi` trait method signature change (`&self` → `&mut self`) for read operations | Medium | High | Only the new methods (`read_sequence`, `read_file`) take `&mut self`. Existing `read_pattern` and `read_as_text` remain `&self`. No breakage. |

### Context-Read Test Failures: Containment Strategy

The parent plan documents 29 test failures in `context-read` and 15 in `context-search`. The `ReadCtx::read_sequence` method may surface these failures as runtime errors rather than test failures.

**Containment approach:**
1. Wrap `ReadCtx::read_sequence()` call in the `read_sequence` method — if it returns `None`, return `ReadError::InternalError` (not a panic)
2. Add logging (`tracing::warn!`) when `read_sequence` fails, including the input text and character count
3. Do NOT add `unwrap()` or `expect()` on any `ReadCtx` method calls
4. Document known limitations in the REPL help text: "Note: read may fail for certain text patterns while the read algorithm is being completed"

---

## Notes

### Future Enhancements (Out of Scope for This Plan)

- **`--files <path1> <path2> ...`** — Unordered set input (D13). Each file gets its own root; no ordering between files.
- **`--stdin`** — Read from stdin pipe. Requires streaming support from `PLAN_READ_STREAM_DESIGN.md`.
- **`--summary` / `--tree` / `--json` / `--verbose`** — Output format flags (D11).
- **Token ref resolution in `read`** — `read 42 43` interpreted as "read pattern composed of tokens #42 and #43". Requires `TokenRef` support in the read pipeline.
- **`ReadSequenceResult`** — A richer result type that includes atom creation stats, deduplication info, and timing. Would be a new `CommandResult` variant.

### Dependencies on Other Plans

| Plan | What this plan needs from it | Blocking? |
|------|------------------------------|-----------|
| `PLAN_INSERT_NEXT_MATCH` | Improved insert semantics used by `ReadCtx` internally | No — current `insert_or_get_complete` works |
| `PLAN_APPEND_TO_PATTERN_FIX` | Fixes to `append_to_pattern` used by `RootManager` | No — current code works for basic cases |
| `PLAN_READ_STREAM_DESIGN` | Streaming file input, async support | No — `read_file` loads entire file for now |
| `PLAN_INTEGRATION_TESTS` | Comprehensive CLI-level test suite | No — this plan includes its own tests |