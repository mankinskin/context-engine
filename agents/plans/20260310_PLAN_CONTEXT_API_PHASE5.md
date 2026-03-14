---
tags: `#context-api` `#phase5` `#typescript` `#ts-rs` `#npm` `#types` `#advanced`
summary: Phase 5 — Centralize ts-rs generation in context-api, publish npm-installable TypeScript types package, migrate existing consumers, and design future instruction language
status: 📋
---

# Plan: context-api Phase 5 — TypeScript Types + Advanced

## Objective

Centralize all TypeScript type generation in the `context-api` crate (behind a `ts-gen` feature flag), publish the generated types as a dedicated npm-installable package at `packages/context-types/`, migrate existing consumers (`log-viewer`, `doc-viewer`) to import from this package instead of their local generated files, and produce a design sketch for a future instruction language grammar. This phase also adds workspace export/import commands (JSON and bincode formats) for data portability.

## Context

### Prerequisites

- **Phase 2 complete** — All API types (`Command`, `CommandResult`, `AtomInfo`, `TokenInfo`, `PatternInfo`, `SearchResult`, `InsertResult`, `PatternReadResult`, `GraphSnapshot`, `GraphStatistics`, etc.) must be stable and finalized.
- **Phase 1 complete** — Core types exist and are exercised by tests.
- **Phase 3/4 recommended** — MCP and HTTP adapters validate the JSON shapes that TypeScript consumers will interact with.

### Interview Reference

- `agents/interviews/20260310_INTERVIEW_CONTEXT_API.md` — Q15 (ts-rs: **centralize in context-api behind feature flag**), Q16 (npm package: **yes, `packages/context-types/`**), Q17 (instruction language: **design sketch only, no implementation**)
- Master plan: `agents/plans/20260310_PLAN_CONTEXT_API_OVERVIEW.md`

### Key Decisions Affecting This Phase

- **Centralize ts-rs** — Move all `#[derive(TS)]` annotations from `context-trace` into `context-api` wrapper types (or re-export with ts-rs on the re-exports). The goal is a single `cargo test -p context-api --features ts-gen` command that generates ALL TypeScript types into one output directory.
- **Feature-gated** — The `ts-gen` feature in `context-api` enables `ts-rs` derives. Default builds don't pull in ts-rs.
- **npm package** — `packages/context-types/` is a publishable npm package containing only the generated `.ts` files plus a hand-written `index.ts` barrel export.
- **Migration** — `log-viewer/frontend` and `doc-viewer/frontend` (if applicable) switch from local `src/types/generated/` to importing `@context-engine/types` (or the package name chosen).
- **Export/import** — New commands `ExportWorkspace` and `ImportWorkspace` for data portability (JSON and bincode).
- **Instruction language** — Design document only; no parser implementation in this phase.

### Dependencies (External Crates)

| Crate | Version | Purpose |
|-------|---------|---------|
| `ts-rs` | 10 | TypeScript type generation from Rust structs/enums |
| `schemars` | 0.8 | JSON Schema generation (already added in Phase 3) |

### Files Affected

**Modified (existing):**
- `crates/context-api/Cargo.toml` — ensure `ts-gen` feature includes `ts-rs` dep
- `crates/context-api/src/types.rs` — add `#[derive(TS)]` behind `#[cfg_attr(feature = "ts-gen", derive(TS))]`
- `crates/context-api/src/error.rs` — same ts-rs derive pattern
- `crates/context-api/src/commands/mod.rs` — `Command` + `CommandResult` ts-rs derives
- `crates/context-trace/src/graph/snapshot.rs` — remove `#[derive(TS)]` and `#[ts(...)]` attrs (migrated to context-api)
- `crates/context-trace/src/graph/visualization.rs` — remove `#[derive(TS)]` and `#[ts(...)]` attrs
- `crates/context-trace/src/graph/search_path.rs` — remove `#[derive(TS)]` and `#[ts(...)]` attrs
- `crates/context-trace/Cargo.toml` — make `ts-rs` dependency optional or remove if no longer needed
- `tools/log-viewer/src/types.rs` — remove `#[derive(TS)]` and `#[ts(...)]` attrs (migrated)
- `tools/log-viewer/src/log_parser.rs` — remove `#[derive(TS)]` and `#[ts(...)]` attrs
- `tools/log-viewer/frontend/package.json` — add `@context-engine/types` dependency
- `tools/log-viewer/frontend/src/types/` — remove `generated/` directory, update imports

**New:**
- `packages/context-types/package.json`
- `packages/context-types/tsconfig.json`
- `packages/context-types/src/index.ts` — barrel export
- `packages/context-types/src/generated/` — ts-rs output target directory
- `packages/context-types/README.md`
- `crates/context-api/src/ts_export.rs` — module that re-exports all types with ts-rs derives (or inline in types.rs)
- `crates/context-api/src/commands/export_import.rs` — export/import workspace commands
- `agents/designs/20260310_DESIGN_INSTRUCTION_LANGUAGE.md` — instruction language grammar sketch

**Workspace root:**
- `Cargo.toml` — no workspace member changes (packages/ is npm, not Rust)

---

## Analysis

### Current State (Before This Phase)

TypeScript type generation is **scattered** across multiple crates:

1. **`context-trace`** (14 types) — `GraphSnapshot`, `SnapshotVertex`, `SnapshotEdge`, `GraphOpEvent`, `NodeHighlightState`, `GraphMutation`, `MutationDiff`, `SearchPath`, `SearchPathNode`, `SearchPathEdge`, `SearchPathTransition`, `QueryInfo`, `OperationType` — all derive `TS` and export to `../tools/log-viewer/frontend/src/types/generated/`.

2. **`log-viewer`** (5 types) — `LogFileInfo`, `LogContentResponse`, `SearchResponse`, `JqQueryResponse`, `AssertionDiff`, `LogEntry` — derive `TS` and export to `../frontend/src/types/generated/`.

Each crate generates types to its own frontend directory. There is no shared TypeScript types package.

### Desired State

1. **Single generation source** — `context-api` with `ts-gen` feature is the only crate that generates TypeScript types.
2. **Single output target** — All types export to `packages/context-types/src/generated/`.
3. **npm package** — `packages/context-types/` is a proper npm package that:
   - Contains all generated types
   - Has a barrel `index.ts` exporting everything
   - Can be installed via npm/pnpm workspace link or published to registry
4. **Consumers updated** — `log-viewer/frontend` imports from `@context-engine/types` instead of local generated files.
5. **New API types** — All `context-api` types (Command, CommandResult, AtomInfo, etc.) also have TypeScript definitions.
6. **Export/import** — Users can export a workspace to JSON (human-readable) or bincode (compact) and import it elsewhere.
7. **Instruction language** — A design document with grammar sketch for a future DSL.

### Migration Strategy

The migration from scattered `#[derive(TS)]` to centralized generation must be done carefully to avoid breaking the `log-viewer` frontend during the transition.

**Approach: Wrapper re-exports in context-api**

Rather than modifying `context-trace` types directly (which would require `context-trace` to depend on `context-api` — a circular dependency), we use **wrapper types or re-export modules** in `context-api`:

```pseudo
// crates/context-api/src/ts_export.rs
// This module exists solely to re-derive TS on types from other crates.
// It creates newtype wrappers or uses ts-rs's #[ts(as = "...")] on re-exports.

#[cfg(feature = "ts-gen")]
mod ts_types {
    use ts_rs::TS;
    use serde::Serialize;

    // Option A: Newtype wrappers that serialize identically
    #[derive(Serialize, TS)]
    #[ts(export, export_to = "../../../../packages/context-types/src/generated/")]
    #[serde(transparent)]
    pub struct GraphSnapshot(pub context_trace::graph::snapshot::GraphSnapshot);

    // Option B: Mirror types (if serde(transparent) causes issues)
    // Manually keep in sync — validated by tests.
}
```

**Alternative approach: Keep ts-rs in context-trace but change export target**

If wrapper types prove too cumbersome, we can instead:
1. Keep `#[derive(TS)]` in `context-trace` but change the `export_to` path to `packages/context-types/src/generated/`
2. Add `#[derive(TS)]` to `context-api` types with the same export target
3. The barrel `index.ts` re-exports everything from one location

This is simpler but means ts-rs generation is still split across crates. The `cargo test` commands to generate types would need to run for multiple crates.

**Recommended: Hybrid approach**
- Keep `#[derive(TS)]` in `context-trace` (those types are tightly coupled to trace internals)
- Add `#[derive(TS)]` to all `context-api` types
- Change ALL `export_to` paths to point to `packages/context-types/src/generated/`
- Add `log-viewer`-specific types to `context-api` or keep them in `log-viewer` but export to the same target
- A single script generates everything: `cargo test --features ts-gen -p context-api -p context-trace -p log-viewer`

---

## Execution Steps

### Step 1: Create the npm Package Skeleton

Create `packages/context-types/`:

```pseudo
packages/context-types/
├── package.json
├── tsconfig.json
├── README.md
└── src/
    ├── index.ts          # Barrel re-export
    └── generated/        # ts-rs output target (gitignored contents, keep .gitkeep)
        └── .gitkeep
```

**`package.json`:**
```json
{
  "name": "@context-engine/types",
  "version": "0.1.0",
  "description": "TypeScript type definitions for the context-engine hypergraph API",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist/"
  ],
  "scripts": {
    "generate": "cd ../../ && cargo test --features ts-gen -p context-api -p context-trace export_bindings",
    "build": "npm run generate && tsc",
    "clean": "rm -rf dist/ src/generated/*.ts",
    "prepublishOnly": "npm run build"
  },
  "keywords": ["context-engine", "hypergraph", "types", "typescript"],
  "license": "MIT",
  "devDependencies": {
    "typescript": "^5.4"
  }
}
```

**`tsconfig.json`:**
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*.ts"],
  "exclude": ["node_modules", "dist"]
}
```

- [ ] Create `packages/context-types/package.json`
- [ ] Create `packages/context-types/tsconfig.json`
- [ ] Create `packages/context-types/src/generated/.gitkeep`
- [ ] Create `packages/context-types/README.md`
- [ ] Verification: `cd packages/context-types && npm install` succeeds

---

### Step 2: Add ts-rs Derives to context-api Types

Update `crates/context-api/Cargo.toml` to ensure ts-gen feature is properly configured:

```toml
[features]
default = []
ts-gen = ["ts-rs"]
dev = []

[dependencies]
ts-rs = { version = "10", features = ["serde-json-impl"], optional = true }
```

Add `#[cfg_attr(feature = "ts-gen", derive(TS))]` and `#[cfg_attr(feature = "ts-gen", ts(export, export_to = "..."))]` to all public API types:

**Types to annotate:**

| Module | Types |
|--------|-------|
| `types.rs` | `AtomInfo`, `TokenInfo`, `PatternInfo`, `VertexInfo`, `WorkspaceInfo`, `GraphStatistics`, `TokenRef`, `SearchResult`, `InsertResult`, `PatternReadResult`, `ValidationReport` |
| `error.rs` | `ApiError`, `WorkspaceError`, `AtomError`, `PatternError`, `SearchError`, `InsertError`, `ReadError` |
| `commands/mod.rs` | `Command`, `CommandResult` |
| `workspace/metadata.rs` | `WorkspaceMetadata` |

All export paths should point to:
```
../../../../packages/context-types/src/generated/
```
(Relative from `crates/context-api/src/` to `packages/context-types/src/generated/`)

Example:
```pseudo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-gen", derive(TS))]
#[cfg_attr(feature = "ts-gen", ts(
    export,
    export_to = "../../../../packages/context-types/src/generated/"
))]
pub struct AtomInfo {
    pub char: char,
    pub index: usize,
}
```

- [ ] Add ts-rs derives to all types in `types.rs`
- [ ] Add ts-rs derives to all types in `error.rs`
- [ ] Add ts-rs derives to `Command` and `CommandResult` in `commands/mod.rs`
- [ ] Add ts-rs derives to `WorkspaceMetadata`
- [ ] Verification: `cargo test -p context-api --features ts-gen` generates `.ts` files in `packages/context-types/src/generated/`

---

### Step 3: Migrate context-trace ts-rs Export Paths

Update the `export_to` paths in `context-trace` to point to the shared package:

**Files to modify:**
- `crates/context-trace/src/graph/snapshot.rs` — 3 types: `GraphSnapshot`, `SnapshotVertex`, `SnapshotEdge`
- `crates/context-trace/src/graph/visualization.rs` — 8 types: `OperationType`, `GraphOpEvent`, `NodeHighlightState`, `QueryInfo`, `GraphMutation`, `MutationDiff`, `SearchVisualizationEvent`
- `crates/context-trace/src/graph/search_path.rs` — 4 types: `SearchPathEdge`, `SearchPathNode`, `SearchPathTransition`, `SearchPath`

**Change all `export_to` from:**
```
export_to = "../../../tools/log-viewer/frontend/src/types/generated/"
```
**To:**
```
export_to = "../../../packages/context-types/src/generated/"
```

This is a simple find-and-replace operation.

- [ ] Update `snapshot.rs` export paths (3 types)
- [ ] Update `visualization.rs` export paths (8 types)
- [ ] Update `search_path.rs` export paths (4 types)
- [ ] Verification: `cargo test -p context-trace --features ts-rs` generates files in `packages/context-types/src/generated/`

---

### Step 4: Migrate log-viewer ts-rs Export Paths

Update the `export_to` paths in `log-viewer` types:

**Files to modify:**
- `tools/log-viewer/src/types.rs` — `LogFileInfo`, `LogContentResponse`, `SearchResponse`, `JqQueryResponse`
- `tools/log-viewer/src/log_parser.rs` — `AssertionDiff`, `LogEntry`

**Change all `export_to` from:**
```
export_to = "../frontend/src/types/generated/"
```
**To:**
```
export_to = "../../../packages/context-types/src/generated/"
```

- [ ] Update `types.rs` export paths (4 types)
- [ ] Update `log_parser.rs` export paths (2 types)
- [ ] Verification: `cargo test -p log-viewer` generates files in `packages/context-types/src/generated/`

---

### Step 5: Create the Barrel Export

Create `packages/context-types/src/index.ts` that re-exports all generated types:

```pseudo
// Auto-generated barrel export for @context-engine/types
// Run `npm run generate` to regenerate the generated/ directory.

// === context-api types ===
export type { AtomInfo } from "./generated/AtomInfo";
export type { TokenInfo } from "./generated/TokenInfo";
export type { PatternInfo } from "./generated/PatternInfo";
export type { VertexInfo } from "./generated/VertexInfo";
export type { WorkspaceInfo } from "./generated/WorkspaceInfo";
export type { GraphStatistics } from "./generated/GraphStatistics";
export type { TokenRef } from "./generated/TokenRef";
export type { SearchResult } from "./generated/SearchResult";
export type { InsertResult } from "./generated/InsertResult";
export type { PatternReadResult } from "./generated/PatternReadResult";
export type { ValidationReport } from "./generated/ValidationReport";
export type { Command } from "./generated/Command";
export type { CommandResult } from "./generated/CommandResult";
export type { ApiError } from "./generated/ApiError";
export type { WorkspaceError } from "./generated/WorkspaceError";
export type { AtomError } from "./generated/AtomError";
export type { PatternError } from "./generated/PatternError";
export type { SearchError } from "./generated/SearchError";
export type { InsertError } from "./generated/InsertError";
export type { ReadError } from "./generated/ReadError";
export type { WorkspaceMetadata } from "./generated/WorkspaceMetadata";

// === context-trace types (visualization/snapshot) ===
export type { GraphSnapshot } from "./generated/GraphSnapshot";
export type { SnapshotVertex } from "./generated/SnapshotVertex";
export type { SnapshotEdge } from "./generated/SnapshotEdge";
export type { OperationType } from "./generated/OperationType";
export type { GraphOpEvent } from "./generated/GraphOpEvent";
export type { NodeHighlightState } from "./generated/NodeHighlightState";
export type { QueryInfo } from "./generated/QueryInfo";
export type { GraphMutation } from "./generated/GraphMutation";
export type { MutationDiff } from "./generated/MutationDiff";
export type { SearchPath } from "./generated/SearchPath";
export type { SearchPathNode } from "./generated/SearchPathNode";
export type { SearchPathEdge } from "./generated/SearchPathEdge";
export type { SearchPathTransition } from "./generated/SearchPathTransition";

// === log-viewer types ===
export type { LogFileInfo } from "./generated/LogFileInfo";
export type { LogContentResponse } from "./generated/LogContentResponse";
export type { SearchResponse } from "./generated/SearchResponse";
export type { JqQueryResponse } from "./generated/JqQueryResponse";
export type { AssertionDiff } from "./generated/AssertionDiff";
export type { LogEntry } from "./generated/LogEntry";
```

**Note:** This file will need to be regenerated whenever types are added or removed. Consider adding a script that auto-generates it by scanning the `generated/` directory.

- [ ] Create `packages/context-types/src/index.ts`
- [ ] Verification: `cd packages/context-types && npm run build` compiles successfully

---

### Step 6: Update log-viewer Frontend Imports

Update `tools/log-viewer/frontend/` to consume types from the shared package:

1. **Add workspace dependency** — If using pnpm/npm workspaces, add to `tools/log-viewer/frontend/package.json`:
   ```json
   {
     "dependencies": {
       "@context-engine/types": "workspace:*"
     }
   }
   ```

   If NOT using npm workspaces, use a relative path:
   ```json
   {
     "dependencies": {
       "@context-engine/types": "file:../../../packages/context-types"
     }
   }
   ```

2. **Find and replace imports** — Change all:
   ```typescript
   import type { GraphSnapshot } from "../types/generated/GraphSnapshot";
   ```
   To:
   ```typescript
   import type { GraphSnapshot } from "@context-engine/types";
   ```

3. **Delete old generated directory** — Remove `tools/log-viewer/frontend/src/types/generated/` since types now come from the package.

- [ ] Add `@context-engine/types` dependency to log-viewer frontend
- [ ] Update all import statements in log-viewer frontend
- [ ] Delete `tools/log-viewer/frontend/src/types/generated/`
- [ ] Verification: `cd tools/log-viewer/frontend && npm run build` (or equivalent) — no TypeScript errors
- [ ] Verification: `cd tools/log-viewer/frontend && npm run typecheck` (if available) — passes

---

### Step 7: Generation Script

Create a top-level script that generates all types in one step:

**`scripts/generate-types.sh`:**
```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Generating TypeScript types ==="

# Clean previous output
rm -f packages/context-types/src/generated/*.ts
# Keep .gitkeep
touch packages/context-types/src/generated/.gitkeep

# Generate types from all crates that have ts-rs derives
echo "Generating context-api types..."
cargo test -p context-api --features ts-gen export_bindings -- --ignored 2>/dev/null || true

echo "Generating context-trace types..."
cargo test -p context-trace export_bindings -- --ignored 2>/dev/null || true

echo "Generating log-viewer types..."
cargo test -p log-viewer export_bindings -- --ignored 2>/dev/null || true

# Count generated files
COUNT=$(ls -1 packages/context-types/src/generated/*.ts 2>/dev/null | wc -l)
echo "=== Generated $COUNT TypeScript type files ==="

# Build the npm package
echo "Building @context-engine/types..."
cd packages/context-types
npm run build
echo "=== Done ==="
```

**`scripts/generate-types.ps1`** (Windows equivalent):
```powershell
Write-Host "=== Generating TypeScript types ==="

Remove-Item -Force packages/context-types/src/generated/*.ts -ErrorAction SilentlyContinue

cargo test -p context-api --features ts-gen export_bindings -- --ignored 2>$null
cargo test -p context-trace export_bindings -- --ignored 2>$null
cargo test -p log-viewer export_bindings -- --ignored 2>$null

$count = (Get-ChildItem packages/context-types/src/generated/*.ts).Count
Write-Host "=== Generated $count TypeScript type files ==="

Set-Location packages/context-types
npm run build
Write-Host "=== Done ==="
```

- [ ] Create `scripts/generate-types.sh` (make executable)
- [ ] Create `scripts/generate-types.ps1`
- [ ] Verification: `./scripts/generate-types.sh` runs end-to-end and produces a buildable package

---

### Step 8: Export/Import Workspace Commands

Add data portability commands to `context-api`:

**New file: `crates/context-api/src/commands/export_import.rs`**

```pseudo
/// Export a workspace to a portable format.
///
/// JSON: human-readable, useful for debugging and small graphs.
/// Bincode: compact binary, useful for backup and transfer of large graphs.
pub enum ExportFormat {
    Json,
    Bincode,
}

/// Exported workspace data — everything needed to reconstruct the workspace.
pub struct WorkspaceExport {
    pub metadata: WorkspaceMetadata,
    pub graph_data: Vec<u8>,         // serialized Hypergraph (in the requested format)
    pub format: ExportFormat,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub context_api_version: String, // for forward compatibility checks
}
```

**New Command variants:**
```pseudo
// Add to Command enum:
ExportWorkspace {
    workspace: String,
    format: ExportFormat,  // "json" or "bincode"
    path: Option<String>,  // output file path; None = return as CommandResult
},
ImportWorkspace {
    name: String,          // name for the imported workspace
    path: String,          // input file path
    overwrite: bool,       // overwrite if workspace already exists
},
```

**New CommandResult variant:**
```pseudo
ExportData(WorkspaceExport),  // when no path specified — inline result
```

**Implementation:**
- `export_workspace`: serialize the in-memory `Hypergraph` to the requested format, bundle with metadata, write to file or return inline.
- `import_workspace`: read file, detect format (by extension or magic bytes), deserialize graph, create workspace directory, save.

- [x] Create `crates/context-api/src/commands/export_import.rs`
- [x] Add `ExportWorkspace` and `ImportWorkspace` to `Command` enum
- [x] Add `ExportFormat` to types with ts-rs derive
- [x] Implement export logic in `WorkspaceManager`
- [x] Implement import logic in `WorkspaceManager`
- [x] Add CLI subcommands: `context-cli export <workspace> --format json --output graph.json`
- [x] Add CLI subcommands: `context-cli import <name> --from graph.json [--overwrite]`
- [x] Add tests: export → import round-trip preserves all graph data
- [x] Verification: `cargo test -p context-api` — export/import tests pass (22 tests)

**Implementation notes (deviations from plan):**
- `WorkspaceExport` struct was **not** created as a public type. Instead, the JSON envelope (`JsonExport`) and bincode envelope use internal formats because `Hypergraph`'s serde uses non-string map keys incompatible with `serde_json`. The JSON format embeds the graph as **base64-encoded bincode bytes** (`graph_b64` field) while keeping metadata human-readable. The bincode format uses a custom length-prefixed framing with a magic header (`CXEI`) to avoid bincode misreading serde-tagged enums.
- `CommandResult::ExportData { data: Vec<u8>, format: ExportFormat }` is returned for inline exports (when no output path is given).
- Format auto-detection on import uses JSON parse attempt first, then checks for the `CXEI` magic header for bincode — no reliance on file extension.
- `WorkspaceApi` trait extended with `export_workspace` and `import_workspace` methods.
- Exhaustive match arms added to `context-cli`, `context-mcp`, and `context-http` for the new `Command` and `CommandResult` variants.

---

### Step 9: Instruction Language Design Document

Create a design sketch for a future instruction language (DSL) that could express graph operations in a human-readable text format.

**File: `agents/designs/20260310_DESIGN_INSTRUCTION_LANGUAGE.md`**

The document should cover:

1. **Motivation** — Why a DSL? (Batch operations, reproducibility, scripting, version-controlled graph definitions)
2. **Grammar sketch** — EBNF or PEG-style grammar for basic operations:
   ```
   program      = statement*
   statement    = atom_decl | pattern_decl | insert_stmt | search_stmt | workspace_stmt
   atom_decl    = "atom" CHAR ("," CHAR)* ";"
   pattern_decl = "pattern" CHAR+ ";"
   insert_stmt  = "insert" STRING ";"
   search_stmt  = "search" STRING ";"
   workspace_stmt = ("create" | "open" | "save" | "close") IDENT ";"
   ```
3. **Example programs:**
   ```
   create my_graph;
   atom a, b, c, d;
   pattern abc;
   insert "abcd";
   search "abc";
   save my_graph;
   ```
4. **Relationship to Command enum** — Each instruction maps 1:1 to a `Command` variant. The instruction language is sugar over `Command` JSON.
5. **Parser approach** — Recommend `winnow` or `pest` for Rust parser implementation (future phase).
6. **Open questions** — Variables, conditionals, loops, comments, error handling semantics.

This is a **design document only** — no code implementation in this phase.

- [ ] Create `agents/designs/` directory if it doesn't exist
- [ ] Create `agents/designs/20260310_DESIGN_INSTRUCTION_LANGUAGE.md`
- [ ] Verification: Document reviewed for completeness and clarity

---

### Step 10: README and Documentation

**`packages/context-types/README.md`:**
```pseudo
# @context-engine/types

TypeScript type definitions for the context-engine hypergraph API.

## Installation

```bash
npm install @context-engine/types
```

## Usage

```typescript
import type { Command, CommandResult, GraphSnapshot, AtomInfo } from "@context-engine/types";

// Send a command to the HTTP API
const command: Command = {
  command: "create_workspace",
  name: "my-graph"
};

const response = await fetch("http://localhost:3100/api/execute", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(command),
});

const result: CommandResult = await response.json();
```

## Regenerating Types

Types are generated from Rust source code using ts-rs.

```bash
# From the repository root:
./scripts/generate-types.sh

# Or manually:
cargo test -p context-api --features ts-gen export_bindings -- --ignored
cargo test -p context-trace export_bindings -- --ignored
cd packages/context-types && npm run build
```

## Available Types

### API Types (from context-api)
- `Command` — All available commands
- `CommandResult` — Result types for each command
- `AtomInfo`, `TokenInfo`, `PatternInfo`, `VertexInfo` — Graph element types
- `WorkspaceInfo`, `WorkspaceMetadata` — Workspace metadata
- `SearchResult`, `InsertResult`, `PatternReadResult` — Algorithm results
- `GraphStatistics`, `ValidationReport` — Diagnostic types
- `ApiError`, `WorkspaceError`, etc. — Error types

### Visualization Types (from context-trace)
- `GraphSnapshot`, `SnapshotVertex`, `SnapshotEdge` — Graph snapshots
- `GraphOpEvent`, `NodeHighlightState` — Visualization events
- `SearchPath`, `SearchPathNode`, `SearchPathEdge` — Search path data
- `GraphMutation`, `MutationDiff` — Mutation tracking

### Log Viewer Types
- `LogFileInfo`, `LogContentResponse`, `LogEntry` — Log parsing
```

- [ ] Create/update `packages/context-types/README.md`
- [ ] Update `crates/context-api/README.md` to mention ts-gen feature and the types package
- [ ] Update master plan overview to mark Phase 5 scope as documented

---

### Step 11: Update INDEX.md and Plan Files

- [ ] Update `agents/plans/INDEX.md` to include this plan
- [ ] Update master overview `20260310_PLAN_CONTEXT_API_OVERVIEW.md` Phase 5 entry if any scope changed during this planning

---

### Step 12: Final Verification

- [ ] `cargo check --workspace` — no errors
- [ ] `cargo test -p context-api --features ts-gen` — ts-rs generation succeeds
- [ ] `cargo test -p context-trace` — still passes (ts-rs paths changed but tests still work)
- [ ] `cargo test -p log-viewer` — still passes
- [ ] `packages/context-types/src/generated/` contains `.ts` files for ALL types (~30+ files)
- [ ] `cd packages/context-types && npm run build` — compiles successfully
- [ ] `packages/context-types/dist/` contains `.js`, `.d.ts`, and `.d.ts.map` files
- [ ] `cd tools/log-viewer/frontend && npm run build` — no TypeScript errors after import migration
- [ ] Export/import round-trip test: create workspace → add data → export JSON → delete workspace → import JSON → verify identical data
- [ ] `agents/designs/20260310_DESIGN_INSTRUCTION_LANGUAGE.md` exists and contains grammar sketch
- [ ] No stale `generated/` directories remain in `tools/log-viewer/frontend/src/types/`

---

## Type Inventory

Complete list of types that will have TypeScript definitions after this phase:

### From context-api (~22 types)

| Type | Module | Category |
|------|--------|----------|
| `Command` | `commands/mod.rs` | API |
| `CommandResult` | `commands/mod.rs` | API |
| `AtomInfo` | `types.rs` | Data |
| `TokenInfo` | `types.rs` | Data |
| `PatternInfo` | `types.rs` | Data |
| `VertexInfo` | `types.rs` | Data |
| `WorkspaceInfo` | `types.rs` | Data |
| `GraphStatistics` | `types.rs` | Data |
| `TokenRef` | `types.rs` | Data |
| `SearchResult` | `types.rs` | Data |
| `InsertResult` | `types.rs` | Data |
| `PatternReadResult` | `types.rs` | Data |
| `ValidationReport` | `types.rs` | Data |
| `ExportFormat` | `commands/export_import.rs` | Data |
| `WorkspaceMetadata` | `workspace/metadata.rs` | Data |
| `ApiError` | `error.rs` | Error |
| `WorkspaceError` | `error.rs` | Error |
| `AtomError` | `error.rs` | Error |
| `PatternError` | `error.rs` | Error |
| `SearchError` | `error.rs` | Error |
| `InsertError` | `error.rs` | Error |
| `ReadError` | `error.rs` | Error |

### From context-trace (~15 types)

| Type | Module | Category |
|------|--------|----------|
| `GraphSnapshot` | `graph/snapshot.rs` | Visualization |
| `SnapshotVertex` | `graph/snapshot.rs` | Visualization |
| `SnapshotEdge` | `graph/snapshot.rs` | Visualization |
| `OperationType` | `graph/visualization.rs` | Visualization |
| `GraphOpEvent` | `graph/visualization.rs` | Visualization |
| `NodeHighlightState` | `graph/visualization.rs` | Visualization |
| `QueryInfo` | `graph/visualization.rs` | Visualization |
| `GraphMutation` | `graph/visualization.rs` | Visualization |
| `MutationDiff` | `graph/visualization.rs` | Visualization |
| `SearchVisualizationEvent` | `graph/visualization.rs` | Visualization |
| `SearchPath` | `graph/search_path.rs` | Visualization |
| `SearchPathNode` | `graph/search_path.rs` | Visualization |
| `SearchPathEdge` | `graph/search_path.rs` | Visualization |
| `SearchPathTransition` | `graph/search_path.rs` | Visualization |

### From log-viewer (~6 types)

| Type | Module | Category |
|------|--------|----------|
| `LogFileInfo` | `types.rs` | Log |
| `LogContentResponse` | `types.rs` | Log |
| `SearchResponse` | `types.rs` | Log |
| `JqQueryResponse` | `types.rs` | Log |
| `AssertionDiff` | `log_parser.rs` | Log |
| `LogEntry` | `log_parser.rs` | Log |

**Total: ~43 TypeScript type definitions**

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| ts-rs path resolution differs on Windows vs Unix | Medium | Medium | Use forward slashes in `export_to` (ts-rs handles this). Test on Windows CI. |
| `#[serde(transparent)]` wrapper types don't generate correct TS | Medium | Medium | If transparent wrappers fail, use the hybrid approach (keep ts-rs in source crates, change paths only) |
| ts-rs version incompatibility across crates | Low | High | Pin exact ts-rs version in workspace `[dependencies]` |
| log-viewer frontend breaks during import migration | Medium | High | Do the migration in a single commit: change paths + update imports + delete old generated/ atomically. Run frontend build as verification. |
| `Command` enum is too complex for ts-rs to generate cleanly | Medium | Medium | ts-rs handles tagged enums well. Test early. If problematic, split into sub-types or use `#[ts(type = "...")]` overrides. |
| npm workspace linking breaks on Windows | Low | Medium | Use `file:` protocol links instead of `workspace:*` for portability |
| Generated files get committed and go stale | Medium | Low | Add `packages/context-types/src/generated/*.ts` to `.gitignore` (except `.gitkeep`). Regenerate in CI. |
| `async-graphql` types need separate ts-rs treatment (Phase 4 interaction) | Low | Low | Phase 4 GraphQL has its own schema introspection — no overlap with ts-rs |
| Export/import bincode format changes break compatibility | Medium | Medium | Include `context_api_version` in export header. On import, validate version and reject incompatible formats with a clear error. |

---

## Validation Criteria

**How to verify success:**
- [ ] Single `./scripts/generate-types.sh` command generates ALL TypeScript types
- [ ] `packages/context-types/` builds as a valid npm package
- [ ] `log-viewer` frontend compiles with imports from `@context-engine/types`
- [ ] No `#[ts(export_to = "...log-viewer/frontend...")]` paths remain in any Rust file
- [ ] All `context-api` public types have TypeScript definitions
- [ ] Export → import round-trip preserves graph data (unit test)
- [ ] Instruction language design document is complete and reviewed
- [ ] `cargo test --workspace` passes (no regressions)
- [ ] `npm run build` in `packages/context-types` produces `dist/` with type declarations

---

## Notes

### npm Workspaces Consideration

If the project later adopts npm/pnpm workspaces at the root level, the `packages/` directory is already in the conventional location. The `package.json` uses `@context-engine/types` as the scoped package name, which is compatible with both workspace linking and registry publishing.

### CI Integration

The type generation should be part of CI to catch drift:
```yaml
- name: Generate TypeScript types
  run: ./scripts/generate-types.sh
- name: Check for uncommitted changes
  run: git diff --exit-code packages/context-types/dist/
```

Alternatively, if generated files are gitignored, CI should:
1. Generate types
2. Build the npm package
3. Run frontend builds that depend on the package
4. Verify no TypeScript errors

### Versioning Strategy

The `@context-engine/types` package version should track `context-api` version:
- `context-api` 0.1.0 → `@context-engine/types` 0.1.0
- Breaking API type changes bump the major version of both

### Future: doc-viewer Types

If `doc-viewer` gains TypeScript types in the future, they should also be centralized into `packages/context-types/`. The barrel export and generation script are designed to accommodate additional source crates.

### Deviations from Plan
*(To be filled during execution)*

### Lessons Learned
*(To be filled after execution)*