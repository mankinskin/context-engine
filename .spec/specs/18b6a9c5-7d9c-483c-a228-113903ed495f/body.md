# Goal
Define a generated, human-readable markdown index hierarchy that integrates with the repository structure, and define a compact machine-friendly semantic summary format (TOON) for similarity search, RAG-style lookup, and automated agent tools.

# Problem
The repository has many overlapping context surfaces: tickets, specs, rules, tests, audit output, and memory workspaces. Without a structured index system, these surfaces drift into duplicated prose, inconsistent navigation, and high token overhead for agents. We need an integrated, unduplicated, and stable indexing model.

# Contract with Locked-In Decisions

## D1: Physical File Tree Locations
Integrated across the repository's physical layout:
- **Folder-level indexes**: Rendered directly as local folder `README.md` files (source of truth indexes for human scanning of folder contents).
- **Workspace-level indexes**: Saved directly inside the respective store workspace folders (e.g. `.ticket/`, `.spec/`, `.rule/`).
- **Agent-client instructions**: Saved inside the `.agents/` folder as consumable agent instruction hooks (`.instructions.md`, `.agent.md`).

## D2: Commit-Time Sync and Latency
All generators are wired into git pre-commit/post-commit hooks. To prevent delaying developer workflow, generation commands must be heavily profile-optimized for low commit latency (< 100ms for incremental runs).

## D3: Detailed Spec Tree Representation
The generated specification is rendered as a clean physical markdown folder hierarchy:
- Each spec node is a single markdown file.
- If a spec has child specs, a child subfolder with the canonical name of the parent spec is created.
- Navigating the spec tree is done via human-readable relative markdown file links.
- No content duplication: parent files link to child files, child files link back to their parent.

## D4: Rule Catalog Categorization
Rule categories are mapped automatically from the segments of their slug prefix (e.g., rule `shared/agent-rules/operating-principles` is mapped to the `shared/agent-rules` category).

## D5: Version Control
All generated index files are **committed to git**. This ensures complete transparency, auditability in PRs, and absolute consistency for agents reading checked-out checkouts.

## D6: Test-API and Log-API Gates
The Test Catalog Generator depends directly on the implementation of `86bf3da2` (test-api) and `0805fb76` (log-api) reaching completion. New test features must be integrated directly into the planning of those crates.

## D7: Complete Test Registry
The generated Test Catalog is a complete registry of the workspace's verification state:
- All test cases defined in the codebase are registered.
- Cases without execution records are explicitly marked as `not-run`.
- The catalog includes specialized sections/views for test failures, Criterion benchmarks, and test audits.

## D8: TOON Sidecar Encoding
The machine-readable sidecar uses the **TOON (Token-Optimized Object Notation)** binary/text hybrid format. This minimizes token consumption risk for agents and keeps references slim, rich, and high-density.

## D9: Workspace DAG Structure
Workspaces are modeled as a Directed Acyclic Graph (DAG) with support for multiple children and multiple parents:
- Each workspace contains a local configuration folder for its corresponding tool.
- A workspace node indexes its parent and child workspace names and physical locations.
- Workspaces serve as the root anchor for tool execution, and can explicitly reference or import other workspace stores to compose wider context graphs.

---

# Reference and Schema Definitions

## ContextRef Format
```rust
struct ContextRef {
    node_id: String,
    relation_kind: RelationKind,
    content_kind: ContentKind,
    source_path: String,          // Relative path in workspace
    anchor: Option<String>,       // Optional markdown heading or element locator
    digest: String,               // Semantic summary payload hash
}
```

## ContextNode Schema
```rust
struct ContextNode {
    id: String,
    kind: ContentKind,
    source_path: String,
    title: String,
    summary: String,              // Normalized dense paragraph or 3-5 bullets
    keywords: Vec<String>,        // Context tags for similarity indexing
    scope: String,
    non_goals: Vec<String>,
    relations: Vec<ContextRef>,   // Multiple parent/child edges (DAG)
    digest: String,               // Hashed representation of normalized fields
    tags: Vec<String>,
    generated_at: u64,
    source_modified_at: u64,
}
```

# Acceptance Criteria
- Every generated catalog/index has a corresponding `.toon` sidecar payload containing serialized `ContextNode` objects.
- Folder READMEs, store folders, and `.agents/` hooks are automatically updated by git hook commands.
- The spec hierarchy is written as physical folders and relative files.
- Command latency in the git pre-commit hook is verified and profile-bounded.
- The workspace DAG allows cross-workspace referencing and importing.

# Traceability
- Planning ticket: [fe098673](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/fe098673-f7fa-43ba-af66-047578861596/ticket.toml)
- Schema ticket: [0dba399a](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/0dba399a-4691-4173-b921-17e5e6f6ebb8/ticket.toml)
- Sidecar/Validator ticket: [e7a0ee3c](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/e7a0ee3c-dc2f-42dd-8c02-5070a747c156/ticket.toml)
