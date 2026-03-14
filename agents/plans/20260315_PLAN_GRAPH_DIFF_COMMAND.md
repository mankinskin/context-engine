---
tags: `#plan` `#cli` `#api` `#context-api` `#context-cli` `#graph-diff` `#comparison`
summary: Add a graph-diff command to context-api and context-cli that compares two workspace graphs label-by-label and emits a structured, human-readable diff. Enables REPL and CLI users to inspect structural differences between any two workspaces — including the ngrams oracle and the context-read output.
status: 📋 planning
phase: 3-implement
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
sibling: 20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md
design_decisions: D17, D21, D22, D23, D24
depends_on:
  - RC-1 fix recommended (but diff command is independently useful even with empty read graphs)
---

# Plan: Graph Diff Command (Phase 3.2)

**Date:** 2026-03-15
**Scope:** Medium (new API type + command, new REPL verb, new CLI subcommand, output formatter)
**Crates:** `context-api`, `context-cli`

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [User Experience](#user-experience)
4. [Design Decisions](#design-decisions)
5. [API Layer](#api-layer)
6. [CLI Layer](#cli-layer)
7. [REPL Integration](#repl-integration)
8. [Comparison Algorithm](#comparison-algorithm)
9. [Output Formatting](#output-formatting)
10. [Files Affected](#files-affected)
11. [Execution Steps](#execution-steps)
12. [Validation](#validation)
13. [Risks & Mitigations](#risks--mitigations)
14. [Related Documents](#related-documents)
15. [Notes](#notes)

---

## Objective

Add a `compare` command to the context-engine CLI and API that takes two
workspace names, extracts their hypergraphs as label-indexed representations,
computes a structural diff, and returns a human-readable report.

The command serves three concrete use cases:

1. **Oracle validation (interactive):** After running `create-ngrams` and
   `read-sequence` on the same string, a user can immediately compare the two
   workspaces to see whether context-read produced the same tokens and
   decompositions that the ngrams algorithm found.

2. **Debugging algorithm divergences:** When a test fails, a developer can
   point `compare` at the "expected" and "actual" workspaces to see exactly
   which vertices and patterns differ.

3. **Regression detection:** Two snapshots of the same workspace taken at
   different times can be compared to detect unintended graph mutations.

---

## Context

### Parent Plan

Child of
[`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md).
Sits in Phase 3 alongside:

- **3.1** ([`20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md`](20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md)) — integration
  test harness that *programmatically* compares graphs; shares the `LabelMap`
  concept introduced there.
- **3d** — general integration test suite.
- **3c** — dungeon-crawler skill documentation.

### Relationship to Phase 3.1

Phase 3.1 builds a **test-internal** graph comparison library
(`common/graph_compare.rs`).  Phase 3.2 exposes the same logic as a
**first-class API command** and CLI verb so that humans — not just tests —
can use it.

The `LabelMap` types and comparison algorithm defined in Phase 3.1 are
the conceptual foundation; Phase 3.2 re-implements them inside `context-api`
as production types (not just test helpers).

### Why Now?

The graph diff command is independently useful even before RC-1 is fixed:

- A user can compare two ngrams workspaces built from different strings.
- A user can compare the result of two successive `insert` calls to verify
  that the graph grew as expected.
- The command provides immediate value as a diagnostic tool during the
  debugging sessions that will fix RC-1 and RC-3.

---

## User Experience

### REPL Session — Oracle Workflow

```
> create-ngrams ngrams-abcabc --timeout 30 abcabc
Created workspace 'ngrams-abcabc' from ngrams (6 vertices, 3 atoms).
(workspace 'ngrams-abcabc' is now active)

> create read-abcabc
Created workspace 'read-abcabc'.

> use read-abcabc
Switched to workspace 'read-abcabc'.

> read abcabc
Root: "abcabc" (index: 6, width: 6)
Text: "abcabc"
Tree: ...

> compare ngrams-abcabc read-abcabc
Graph diff: 'ngrams-abcabc' (A) vs 'read-abcabc' (B)
─────────────────────────────────────────────────────
Shared: 7 vertices
  ✓ "a"       (w=1)
  ✓ "b"       (w=1)
  ✓ "c"       (w=1)
  ✓ "ab"      (w=2)  patterns match
  ✓ "bc"      (w=2)  patterns match
  ✓ "abc"     (w=3)  patterns match
  ✓ "abcabc"  (w=6)  patterns match

Only in A (ngrams-abcabc): 0
Only in B (read-abcabc): 0
Pattern differences: 0

Result: EQUIVALENT ✓
```

### REPL Session — Divergence Found

```
> compare ngrams-hello read-hello
Graph diff: 'ngrams-hello' (A) vs 'read-hello' (B)
─────────────────────────────────────────────────────
Shared: 5 vertices
  ✓ "h"  (w=1)
  ✓ "e"  (w=1)
  ✓ "l"  (w=1)
  ✓ "o"  (w=1)
  ~ "hello" (w=5)  PATTERN MISMATCH
    A patterns:
      ["h", "e", "l", "l", "o"]
    B patterns:
      ["hel", "lo"]
    Common patterns: none

Only in A (ngrams-hello): 2
  - "ll"   (w=2)
  - "llo"  (w=3)

Only in B (read-hello): 1
  + "hel"  (w=3)  [unverified by oracle — single-position merge]
  + "lo"   (w=2)  [unverified by oracle — single-position merge]

Pattern differences: 1 ("hello")

Result: DIVERGENT ✗
  The root token has no common pattern between A and B.
  This may indicate an algorithm correctness issue.
```

### REPL Session — Subset Check

```
> compare ngrams-abcabc read-abcabc --mode subset
Graph diff (subset mode): is 'read-abcabc' ⊆ 'ngrams-abcabc'?
─────────────────────────────────────────────────────────────
Every vertex in B is present in A: YES ✓
Every vertex in B has matching width in A: YES ✓
Every pattern in B has a match in A: YES ✓

Result: B IS A VALID SUBSET OF A ✓
```

### CLI (Non-Interactive)

```bash
# Full diff
context-cli compare ngrams-abcabc read-abcabc

# Subset check only (exit code 0 if subset, 1 if not)
context-cli compare ngrams-abcabc read-abcabc --mode subset

# JSON output for scripting
context-cli compare ngrams-abcabc read-abcabc --format json

# Diff of a single vertex across two workspaces
context-cli compare-vertex ngrams-abcabc 5 read-abcabc 6
```

---

## Design Decisions

### D17 (Shared with Phase 3.1): Label-Indexed Comparison

Vertex indices are internal implementation details.  All comparison is
performed using the **string label** as the canonical key.  Labels are unique
within a graph (no two vertices have the same string representation).

### D21: Hierarchical Result — Summary → Vertex → Pattern

The `GraphDiffResult` type is structured at three levels:

```
GraphDiffResult
├── summary: DiffSummary          ← counts, overall verdict
├── shared: Vec<SharedVertex>     ← vertices present in both graphs
│   ├── match_kind: VertexMatchKind  (Identical | PatternMismatch | WidthMismatch)
│   └── pattern_diff: Option<PatternDiff>
├── only_in_a: Vec<VertexInfo>    ← vertices present in A but not B
└── only_in_b: Vec<VertexInfo>    ← vertices present in B but not A
```

Callers who only need a pass/fail can check `summary.result`.
Callers who need details drill into `shared`, `only_in_a`, `only_in_b`.

### D22: "A" Is the Reference, "B" Is the Candidate

The command signature is `compare <workspace_a> <workspace_b>`.  In the
context of oracle validation:

- **A** = ngrams workspace (the reference / ground truth)
- **B** = context-read workspace (the candidate under test)

`only_in_a` = "missing from candidate" (expected but not produced).
`only_in_b` = "extra in candidate" (produced but not in reference).

This naming is fixed in the API; the CLI and REPL docs will explain the
convention clearly.

### D23: Comparison Mode — Full Diff vs. Subset Check

The `compare` command supports two modes via an optional `--mode` flag:

| Mode | Description | Exit / Result |
|------|-------------|---------------|
| `full` (default) | Compute complete diff: shared, only_in_a, only_in_b, pattern diffs | `EQUIVALENT` or `DIVERGENT` |
| `subset` | Check only whether every vertex in B is present in A with matching structure | `SUBSET` or `NOT_SUBSET` |

Subset mode is faster (stops early if any B-vertex is missing from A) and
is used in the oracle validation workflow and CI checks.

### D24: Snapshot-Based — No Cross-Workspace Locks

The diff is computed from two `GraphSnapshot` values.  Snapshots are taken
at the start of the command and released immediately.  No workspace locks
are held during the comparison computation, avoiding lock-ordering issues
when both workspaces are held by the same `WorkspaceManager`.

---

## API Layer

### New Types in `context-api/src/types.rs`

```rust
/// Overall verdict of a graph comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffVerdict {
    /// Every vertex in both graphs matches by label, width, and patterns.
    Equivalent,
    /// B is a structural subset of A (subset mode only).
    Subset,
    /// Graphs differ — see details.
    Divergent,
}

/// High-level counts and verdict.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSummary {
    pub verdict: DiffVerdict,
    pub shared_count: usize,
    pub only_in_a_count: usize,
    pub only_in_b_count: usize,
    pub pattern_mismatch_count: usize,
    pub width_mismatch_count: usize,
}

/// A vertex present in both graphs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedVertex {
    pub label: String,
    pub width: usize,
    pub match_kind: VertexMatchKind,
    /// Present only when match_kind == PatternMismatch.
    pub pattern_diff: Option<PatternDiff>,
}

/// How the shared vertex's structure compares between A and B.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VertexMatchKind {
    /// Label, width, and all patterns are identical.
    Identical,
    /// Label matches but width differs.
    WidthMismatch { width_a: usize, width_b: usize },
    /// Width matches but no common pattern exists.
    PatternMismatch,
    /// Both patterns and width match, but B has additional patterns not in A.
    ExtraPatterns,
}

/// Detailed pattern-level diff for a vertex.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDiff {
    /// Patterns present in A but not B (missing from candidate).
    pub only_in_a: Vec<Vec<String>>,
    /// Patterns present in B but not A (extra in candidate).
    pub only_in_b: Vec<Vec<String>>,
    /// Patterns present in both.
    pub common: Vec<Vec<String>>,
}

/// A lightweight vertex entry used in only_in_a / only_in_b lists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffVertexEntry {
    pub label: String,
    pub width: usize,
    /// All child patterns of this vertex.
    pub patterns: Vec<Vec<String>>,
    /// Whether this vertex is an atom (no children).
    pub is_atom: bool,
}

/// Full result of a graph diff operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDiffResult {
    pub workspace_a: String,
    pub workspace_b: String,
    pub mode: CompareMode,
    pub summary: DiffSummary,
    pub shared: Vec<SharedVertex>,
    pub only_in_a: Vec<DiffVertexEntry>,
    pub only_in_b: Vec<DiffVertexEntry>,
}

/// The comparison mode used.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareMode {
    Full,
    Subset,
}
```

### New Commands in `context-api/src/commands/mod.rs`

Add to the `Command` enum:

```rust
/// Compare two workspace graphs and return a structured diff.
CompareWorkspaces {
    workspace_a: String,
    workspace_b: String,
    /// Comparison mode: "full" (default) or "subset".
    #[serde(default)]
    mode: CompareMode,
},

/// Compare a single vertex from workspace A against a single vertex from B.
CompareVertices {
    workspace_a: String,
    index_a: usize,
    workspace_b: String,
    index_b: usize,
},
```

Add to the `CommandResult` enum:

```rust
GraphDiff(GraphDiffResult),
```

### New Module `context-api/src/commands/compare.rs`

```rust
//! Graph comparison commands.
//!
//! - `compare_workspaces` — full or subset structural diff of two graphs.
//! - `compare_vertices` — focused comparison of two individual vertices.

impl WorkspaceManager {
    /// Compare two workspace graphs.
    ///
    /// Both workspaces must be open. Snapshots are taken and released
    /// immediately (no cross-workspace locks held during computation).
    pub fn compare_workspaces(
        &mut self,
        ws_a: &str,
        ws_b: &str,
        mode: CompareMode,
    ) -> Result<GraphDiffResult, CompareError> { ... }

    /// Compare two individual vertices from (potentially different) workspaces.
    pub fn compare_vertices(
        &mut self,
        ws_a: &str,
        index_a: usize,
        ws_b: &str,
        index_b: usize,
    ) -> Result<GraphDiffResult, CompareError> { ... }
}
```

### Error Type

```rust
#[derive(Debug, thiserror::Error)]
pub enum CompareError {
    #[error("workspace '{name}' is not open")]
    WorkspaceNotOpen { name: String },
    #[error("vertex index {index} not found in workspace '{workspace}'")]
    VertexNotFound { workspace: String, index: usize },
}
```

### `WorkspaceApi` Trait Extension

Add the two methods to the `WorkspaceApi` trait in `commands/mod.rs`:

```rust
fn compare_workspaces(
    &mut self,
    workspace_a: &str,
    workspace_b: &str,
    mode: CompareMode,
) -> Result<GraphDiffResult, CompareError>;

fn compare_vertices(
    &mut self,
    workspace_a: &str,
    index_a: usize,
    workspace_b: &str,
    index_b: usize,
) -> Result<GraphDiffResult, CompareError>;
```

### `execute` Dispatch

```rust
Command::CompareWorkspaces { workspace_a, workspace_b, mode } => {
    let result = manager.compare_workspaces(&workspace_a, &workspace_b, mode)?;
    Ok(CommandResult::GraphDiff(result))
},
Command::CompareVertices { workspace_a, index_a, workspace_b, index_b } => {
    let result = manager.compare_vertices(&workspace_a, index_a, &workspace_b, index_b)?;
    Ok(CommandResult::GraphDiff(result))
},
```

---

## Comparison Algorithm

The comparison algorithm is a pure function:

```rust
fn compare(
    snap_a: &GraphSnapshot,
    workspace_a: &str,
    snap_b: &GraphSnapshot,
    workspace_b: &str,
    mode: CompareMode,
) -> GraphDiffResult
```

### Step 1: Build Label Maps

Convert each `GraphSnapshot` into a `LabelMap`:

```
LabelMap = HashMap<label: String, Entry {
    width: usize,
    patterns: BTreeSet<Vec<String>>,  // sorted set of child-label sequences
    is_atom: bool,
}>
```

Building a `LabelMap` from a `GraphSnapshot`:

1. Build `idx_to_label: HashMap<usize, String>` from `snapshot.nodes`.
2. Group `snapshot.edges` by `(from, pattern_idx)` — each group is one pattern.
3. Within each group, sort by `sub_index` and resolve child indices to labels.
4. For each node, collect all its patterns as `BTreeSet<Vec<String>>`.
5. Mark atoms: nodes with `width == 1` and an empty edge list are atoms.

### Step 2: Classify Vertices

Partition labels into three sets:

```
keys_a = set of labels in map_a
keys_b = set of labels in map_b

shared   = keys_a ∩ keys_b
only_in_a = keys_a \ keys_b
only_in_b = keys_b \ keys_a
```

### Step 3: Classify Shared Vertices

For each label `l` in `shared`:

```
entry_a = map_a[l]
entry_b = map_b[l]

if entry_a.width != entry_b.width:
    → WidthMismatch { width_a, width_b }

elif entry_a.patterns == entry_b.patterns:
    → Identical

elif entry_b.patterns ⊆ entry_a.patterns:
    # B has a subset of A's patterns — this is normal for context-read vs ngrams
    → Identical  (patterns in B are a valid subset of A's patterns)

elif entry_a.patterns ∩ entry_b.patterns is non-empty:
    # Some patterns in B are new (not in A), but there is at least one common
    # pattern — the vertex is structurally compatible
    → ExtraPatterns { pattern_diff }

else:
    # No common pattern — genuine structural divergence
    → PatternMismatch { pattern_diff }
```

> **Key subtlety on pattern subsumption:** A vertex in the context-read graph
> may have a pattern `["hel", "lo"]` while the ngrams graph has `["h", "ello"]`
> and `["hell", "o"]` for the same vertex `"hello"`.  Neither is a subset of
> the other.  Both are valid decompositions.  The algorithm reports this as
> `ExtraPatterns` rather than `PatternMismatch` because both are structurally
> correct — they differ only in which sub-tokens were selected.

### Step 4: Compute Pattern Diffs

For each `PatternMismatch` or `ExtraPatterns` vertex:

```
only_in_a = patterns_a \ patterns_b
only_in_b = patterns_b \ patterns_a
common    = patterns_a ∩ patterns_b
```

### Step 5: Compute Verdict

```
if mode == Subset:
    verdict = if only_in_b.is_empty()
              && all shared vertices are not WidthMismatch
              && all shared vertices have patterns_b ⊆ patterns_a:
                  Subset
              else:
                  NotSubset → DiffVerdict::Divergent

if mode == Full:
    verdict = if only_in_a.is_empty()
              && only_in_b.is_empty()
              && no PatternMismatch or WidthMismatch among shared:
                  Equivalent
              else:
                  Divergent
```

### Complexity

| Step | Complexity |
|------|-----------|
| Build label maps | O(V + E) |
| Classify vertices | O(V log V) |
| Classify shared vertices | O(V × P × K) where P = max patterns per vertex, K = max pattern length |
| Compute pattern diffs | O(V × P log P) |
| **Total** | O(V × P × K) — dominated by pattern comparison |

For typical small graphs (V ≤ 100, P ≤ 3, K ≤ 10): sub-millisecond.

---

## CLI Layer

### New Subcommand in `CliCommand`

```rust
/// Compare two workspace graphs (full diff or subset check).
Compare {
    /// The reference workspace (A).
    workspace_a: String,
    /// The candidate workspace (B).
    workspace_b: String,
    /// Comparison mode: "full" (default) or "subset".
    #[clap(long, default_value = "full")]
    mode: CompareMode,
    /// Output format: "human" (default) or "json".
    #[clap(long, default_value = "human")]
    format: OutputFormat,
},

/// Compare two individual vertices from (potentially different) workspaces.
CompareVertex {
    workspace_a: String,
    #[clap(long = "vertex-a")]
    index_a: usize,
    workspace_b: String,
    #[clap(long = "vertex-b")]
    index_b: usize,
},
```

### Dispatch in `execute_subcommand`

```rust
CliCommand::Compare { workspace_a, workspace_b, mode, format } => {
    // Both workspaces must be open before comparison.
    // Open them if not already open.
    ensure_workspace_open(&mut manager, &workspace_a);
    ensure_workspace_open(&mut manager, &workspace_b);
    let cmd = Command::CompareWorkspaces {
        workspace_a,
        workspace_b,
        mode,
    };
    execute_and_print_format(&mut manager, cmd, format);
},
```

### Workspace Auto-Open

If either workspace is not currently open, the compare command will attempt
to open it from disk before running the diff.  This allows:

```bash
context-cli compare ngrams-abc read-abc
# Even if neither workspace is currently open in memory.
```

---

## REPL Integration

### New `compare` Verb

```rust
"compare" =>
    if parts.len() < 3 {
        eprintln!(
            "Usage: compare <workspace-a> <workspace-b> [--mode full|subset]"
        );
    } else {
        let ws_a = parts[1].to_string();
        let ws_b = parts[2].to_string();
        let mode = parts.iter().skip(3)
            .position(|p| *p == "--mode")
            .and_then(|i| parts.get(i + 4))
            .map(|s| CompareMode::from_str(s).unwrap_or_default())
            .unwrap_or_default();

        // Auto-open workspaces if not open
        try_open_if_closed(manager, &ws_a);
        try_open_if_closed(manager, &ws_b);

        execute_and_print(
            manager,
            Command::CompareWorkspaces {
                workspace_a: ws_a,
                workspace_b: ws_b,
                mode,
            },
            *tracing_enabled,
            current_ws.as_deref(),
        ).ok();
    },

"compare-vertex" =>
    if parts.len() < 5 {
        eprintln!(
            "Usage: compare-vertex <ws-a> <index-a> <ws-b> <index-b>"
        );
    } else {
        // parse indices, dispatch CompareVertices
        ...
    },
```

### Help Text Addition

```
Compare commands:
  compare <ws-a> <ws-b> [--mode full|subset]
                         Compare two workspace graphs (diff or subset check)
  compare-vertex <ws-a> <idx-a> <ws-b> <idx-b>
                         Compare two individual vertices across workspaces
```

---

## Output Formatting

### Human-Readable Format

New function in `tools/context-cli/src/output.rs`:

```rust
pub fn print_graph_diff(result: &GraphDiffResult) {
    println!(
        "Graph diff: '{}' (A) vs '{}' (B)",
        result.workspace_a, result.workspace_b
    );
    println!("{}", "─".repeat(54));

    // -- Shared vertices --------------------------------------------------
    if !result.shared.is_empty() {
        println!("Shared: {} vertex/vertices", result.shared.len());
        for sv in &result.shared {
            match &sv.match_kind {
                VertexMatchKind::Identical => {
                    println!("  ✓ {:12}  (w={})", format!("{:?}", sv.label), sv.width);
                },
                VertexMatchKind::ExtraPatterns => {
                    println!(
                        "  ~ {:12}  (w={})  extra patterns in B",
                        format!("{:?}", sv.label), sv.width
                    );
                    if let Some(diff) = &sv.pattern_diff {
                        print_pattern_diff(diff, 4);
                    }
                },
                VertexMatchKind::PatternMismatch => {
                    println!(
                        "  ~ {:12}  (w={})  PATTERN MISMATCH",
                        format!("{:?}", sv.label), sv.width
                    );
                    if let Some(diff) = &sv.pattern_diff {
                        print_pattern_diff(diff, 4);
                    }
                },
                VertexMatchKind::WidthMismatch { width_a, width_b } => {
                    println!(
                        "  ! {:12}  WIDTH MISMATCH  A={}  B={}",
                        format!("{:?}", sv.label), width_a, width_b
                    );
                },
            }
        }
        println!();
    }

    // -- Only in A --------------------------------------------------------
    if !result.only_in_a.is_empty() {
        println!("Only in '{}' (A): {}", result.workspace_a, result.only_in_a.len());
        for v in &result.only_in_a {
            println!("  - {:12}  (w={})", format!("{:?}", v.label), v.width);
        }
        println!();
    }

    // -- Only in B --------------------------------------------------------
    if !result.only_in_b.is_empty() {
        println!("Only in '{}' (B): {}", result.workspace_b, result.only_in_b.len());
        for v in &result.only_in_b {
            let note = if v.is_atom { "" } else { "  [unverified by oracle]" };
            println!("  + {:12}  (w={}){}", format!("{:?}", v.label), v.width, note);
        }
        println!();
    }

    // -- Summary ----------------------------------------------------------
    println!(
        "Pattern differences: {}",
        result.summary.pattern_mismatch_count
    );
    println!();
    match result.summary.verdict {
        DiffVerdict::Equivalent =>
            println!("Result: EQUIVALENT ✓"),
        DiffVerdict::Subset =>
            println!("Result: B IS A VALID SUBSET OF A ✓"),
        DiffVerdict::Divergent => {
            println!("Result: DIVERGENT ✗");
            if result.summary.width_mismatch_count > 0 {
                println!(
                    "  {} width mismatch(es) — likely a correctness bug.",
                    result.summary.width_mismatch_count
                );
            }
            if result.summary.pattern_mismatch_count > 0 {
                println!(
                    "  {} vertex/vertices with no common pattern — check algorithm.",
                    result.summary.pattern_mismatch_count
                );
            }
        },
    }
}

fn print_pattern_diff(diff: &PatternDiff, indent: usize) {
    let pad = " ".repeat(indent);
    if !diff.common.is_empty() {
        for p in &diff.common {
            println!("{}  = [{}]", pad, p.iter().map(|s| format!("{:?}", s)).join(", "));
        }
    }
    for p in &diff.only_in_a {
        println!("{}  - [{}]  (only in A)", pad, p.iter().map(|s| format!("{:?}", s)).join(", "));
    }
    for p in &diff.only_in_b {
        println!("{}  + [{}]  (only in B)", pad, p.iter().map(|s| format!("{:?}", s)).join(", "));
    }
}
```

### JSON Format

When `--format json` is passed, `GraphDiffResult` is serialised directly via
`serde_json::to_string_pretty`.  The structure is self-describing.

### Exit Codes (CLI Binary)

| Verdict | Exit Code |
|---------|-----------|
| `Equivalent` or `Subset` | `0` |
| `Divergent` | `1` |
| Error (workspace not found, etc.) | `2` |

This enables `compare` to be used in shell scripts and CI pipelines:

```bash
context-cli compare ngrams-abc read-abc --mode subset || echo "VALIDATION FAILED"
```

---

## Files Affected

### New Files

| Path | Description |
|------|-------------|
| `crates/context-api/src/commands/compare.rs` | `compare_workspaces`, `compare_vertices` implementations |

### Modified Files

| File | Change |
|------|--------|
| `crates/context-api/src/types.rs` | Add `GraphDiffResult`, `DiffSummary`, `SharedVertex`, `VertexMatchKind`, `PatternDiff`, `DiffVertexEntry`, `DiffVerdict`, `CompareMode` |
| `crates/context-api/src/commands/mod.rs` | Add `Command::CompareWorkspaces`, `Command::CompareVertices`, `CommandResult::GraphDiff`, `WorkspaceApi` trait methods, `execute` dispatch, `compare.rs` module declaration |
| `crates/context-api/src/error.rs` | Add `CompareError` |
| `tools/context-cli/src/main.rs` | Add `CliCommand::Compare`, `CliCommand::CompareVertex` variants |
| `tools/context-cli/src/repl.rs` | Add `compare` and `compare-vertex` REPL verbs, help text |
| `tools/context-cli/src/output.rs` | Add `print_graph_diff`, `print_pattern_diff` formatters |

---

## Execution Steps

### Step 1: Define New Types in `context-api/src/types.rs`

Add `DiffVerdict`, `DiffSummary`, `SharedVertex`, `VertexMatchKind`, `PatternDiff`,
`DiffVertexEntry`, `GraphDiffResult`, `CompareMode` to `types.rs`.

**Entry criteria:** `context-api` compiles cleanly.
**Exit criteria:** All new types derive `Debug`, `Clone`, `Serialize`, `Deserialize`;
`cargo check -p context-api` passes with zero errors.

### Step 2: Add `CompareError` to `context-api/src/error.rs`

```rust
#[derive(Debug, thiserror::Error)]
pub enum CompareError {
    #[error("workspace '{name}' is not open")]
    WorkspaceNotOpen { name: String },
    #[error("vertex index {index} not found in workspace '{workspace}'")]
    VertexNotFound { workspace: String, index: usize },
}
```

Implement `From<CompareError>` for `ApiError`.

**Entry criteria:** Step 1 done.
**Exit criteria:** `context-api` compiles; `CompareError` is importable.

### Step 3: Implement `crates/context-api/src/commands/compare.rs`

Implement the comparison algorithm (Steps 1–5 from §Comparison Algorithm):

1. `build_label_map(snap: &GraphSnapshot) -> LabelMap`
2. `compare_label_maps(map_a, map_b, ws_a, ws_b, mode) -> GraphDiffResult`
3. `WorkspaceManager::compare_workspaces(...)` — takes snapshots, delegates to (2)
4. `WorkspaceManager::compare_vertices(...)` — extracts single-vertex sub-graphs, delegates to (2)

**Entry criteria:** Steps 1–2 done.
**Exit criteria:** Unit tests for `build_label_map` and `compare_label_maps` pass
with hand-crafted `GraphSnapshot` fixtures covering: identical graphs, extra vertex
in B, width mismatch, pattern mismatch, pattern subset.

Unit test fixtures to write:

| Test | Scenario |
|------|----------|
| `compare_identical_graphs` | A == B → `Equivalent` |
| `compare_extra_vertex_in_b` | B has one extra atom → `Divergent`, `only_in_b.len()==1` |
| `compare_missing_vertex_in_b` | B missing one atom → `Divergent`, `only_in_a.len()==1` |
| `compare_width_mismatch` | Same label, different width → `WidthMismatch` |
| `compare_pattern_mismatch` | Same label+width, no common pattern → `PatternMismatch` |
| `compare_pattern_subset` | B patterns ⊆ A patterns → `Identical` (B uses A's patterns) |
| `compare_extra_patterns_in_b` | B has patterns A doesn't, plus common ones → `ExtraPatterns` |
| `compare_subset_mode_pass` | Subset check: B ⊆ A → `Subset` |
| `compare_subset_mode_fail` | Subset check: B ⊄ A → `Divergent` |

### Step 4: Register `compare.rs` Module and Add Commands

In `context-api/src/commands/mod.rs`:

1. Add `pub mod compare;` module declaration.
2. Add `CompareWorkspaces` and `CompareVertices` to `Command` enum.
3. Add `GraphDiff(GraphDiffResult)` to `CommandResult` enum.
4. Add `compare_workspaces` and `compare_vertices` to `WorkspaceApi` trait.
5. Add dispatch arms in `execute`.

**Entry criteria:** Step 3 done.
**Exit criteria:** `cargo test -p context-api` passes; command name round-trip serde tests pass.

### Step 5: Add Output Formatter to `context-cli/src/output.rs`

Implement `print_graph_diff` and `print_pattern_diff` as described in
§Output Formatting.  Add `CommandResult::GraphDiff(result) => print_graph_diff(result)`
to the `print_command_result` dispatch.

**Entry criteria:** Step 4 done.
**Exit criteria:** `print_graph_diff` renders a hand-crafted `GraphDiffResult` correctly
(verified by snapshot test or visual inspection).

### Step 6: Add CLI Subcommands to `context-cli/src/main.rs`

Add `CliCommand::Compare` and `CliCommand::CompareVertex` variants.
Add dispatch cases in `execute_subcommand`.

**Entry criteria:** Step 5 done.
**Exit criteria:** `context-cli compare --help` prints usage; `context-cli compare nonexistent-a nonexistent-b`
exits with a clear error.

### Step 7: Add REPL Verbs to `context-cli/src/repl.rs`

Add `"compare"` and `"compare-vertex"` match arms in `execute_repl_line`.
Add entries in `print_help`.

**Entry criteria:** Step 6 done.
**Exit criteria:** In a live REPL session, `help` shows both new commands; `compare ws ws`
with an open workspace executes without panic.

### Step 8: Integration Tests

Add `tools/context-cli/tests/integration/compare_tests.rs` with tests covering:

| Test | Scenario | Expected |
|------|----------|----------|
| `compare_identical_workspaces` | Same workspace compared against itself | `Equivalent` |
| `compare_superset_vs_subset` | Insert "abc" + "abd" in A; only "abc" in B | `Divergent`, `only_in_a` has "abd" structure |
| `compare_empty_workspaces` | Both empty | `Equivalent`, zero vertices |
| `compare_atom_mismatch` | A has atom 'x', B does not | `Divergent` |
| `compare_subset_mode` | B ⊆ A — succeeds with `Subset` | exit code 0 simulation |
| `compare_vertex_level` | Single vertex comparison across workspaces | `GraphDiff` returned |

**Entry criteria:** Step 7 done.
**Exit criteria:** All new tests pass; no regressions in existing test suite.

### Step 9: Update Documentation and FAILING_TESTS.md

1. Add `compare` and `compare-vertex` to the REPL help reference in any skill docs.
2. Update `FAILING_TESTS.md` summary table (new passing tests count).
3. Add a brief description of the command to `README.md` or a new `docs/commands/compare.md`.

---

## Validation

### Correctness Criteria

| Criterion | How Verified |
|-----------|--------------|
| Identical graphs → `Equivalent` | Unit test `compare_identical_graphs` |
| Width mismatch detected | Unit test `compare_width_mismatch` |
| Pattern mismatch detected | Unit test `compare_pattern_mismatch` |
| Pattern subset treated as `Identical` | Unit test `compare_pattern_subset` |
| `only_in_b` vertices flagged as "unverified" | Output formatter test |
| Exit code 0 on `Equivalent`/`Subset` | Integration test via process spawn |
| Exit code 1 on `Divergent` | Integration test via process spawn |
| JSON output is valid JSON | `serde_json::from_str` round-trip test |

### REPL Smoke Test (Manual)

```bash
cargo build -p context-cli

# Launch REPL
context-cli repl

# Create two identical workspaces and compare
> create ws-a
> insert abc
> create ws-b
> insert abc
> compare ws-a ws-b
# Expected: Result: EQUIVALENT ✓

# Create a diverging workspace
> create ws-c
> insert abx
> compare ws-a ws-c
# Expected: Result: DIVERGENT ✗ — vertices "b" vs "x" differ, etc.

# Subset check
> compare ws-a ws-b --mode subset
# Expected: Result: B IS A VALID SUBSET OF A ✓
```

### Oracle Workflow Smoke Test (Manual, Post RC-1 Fix)

```bash
context-cli repl
> create-ngrams ngrams-abcabc --timeout 30 abcabc
> create read-abcabc
> use read-abcabc
> read abcabc
> compare ngrams-abcabc read-abcabc --mode subset
# Expected: Result: B IS A VALID SUBSET OF A ✓
```

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Labels contain truncated text (RC-1 active) | High | Comparison returns meaningless results (all tokens labelled "a") | Document clearly: compare is only meaningful after RC-1 is fixed for the read workspace; works correctly for ngrams-vs-ngrams comparisons today |
| Cross-workspace lock ordering issue | Low | Deadlock when both workspaces are in the same manager | Use snapshot-based comparison (D24): take both snapshots, drop all locks, then compare |
| Pattern ordering non-determinism in `GraphSnapshot` | Medium | Flaky test results | Canonicalise: sort patterns by first child label before inserting into `BTreeSet` |
| `vertex_key_string` returns empty string for some vertices | Low | Empty labels in `LabelMap` cause spurious "shared" matches | Filter out empty labels during `LabelMap` construction; log a warning |
| CLI exit code not propagated through `execute_and_print` | Medium | `compare` always exits 0 regardless of verdict | Add `execute_and_print_with_exit_code` variant that returns the verdict for the CLI to act on |

---

## Related Documents

| Document | Relationship |
|----------|-------------|
| [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) | Parent plan; this is Phase 3.2 |
| [`20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md`](20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md) | Sibling Phase 3.1 — shares LabelMap concept; oracle tests will use `compare` internally |
| [`20260314_PLAN_INTEGRATION_TESTS.md`](20260314_PLAN_INTEGRATION_TESTS.md) | Phase 3d — general integration tests; `compare_tests.rs` is added alongside these |
| `crates/context-api/src/types.rs` | Extended with new diff types |
| `crates/context-trace/src/graph/snapshot.rs` | `GraphSnapshot` is the input to the comparison algorithm |
| `tools/context-cli/tests/FAILING_TESTS.md` | Updated with new passing tests after Step 8 |
| `docs/skills/03_context_completion.md` | Mentions `compare` as a diagnostic tool in the "Try It Yourself" REPL section |

---

## Notes

### Questions for User

- Should `compare` auto-open workspaces that are saved on disk but not currently
  open in memory?  Default proposed: yes (auto-open, auto-close after snapshot).
  This keeps the UX frictionless but adds disk I/O.
- Should `ExtraPatterns` in B (B has patterns A doesn't) be treated as a
  warning or a hard failure in `--mode subset`?  Proposed: treat as **pass** in
  subset mode — extra patterns in B are valid decompositions not in the oracle,
  which is acceptable.
- Should the `compare` REPL verb require both workspaces to be open, or should
  it auto-open from disk?  Proposed: auto-open from disk if closed, same as the
  CLI subcommand.
- Is a `compare-vertex` command useful enough to implement in the same phase, or
  should it be deferred to a later cleanup pass?

### Deviations from Plan

*(To be filled during implementation.)*

### Lessons Learned

*(To be filled after implementation.)*