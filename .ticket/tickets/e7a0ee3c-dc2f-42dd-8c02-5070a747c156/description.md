Design and implement the compact machine-readable sidecar manifest emitted alongside every local markdown index. This sidecar is the primary surface for similarity search, RAG retrieval, and automated agent tooling, co-located directly inside the store's domain-specific workspace folder.

## Scope
- Define the sidecar format: a compact TOON file with a dense sequence of serialized ContextNode objects (schema owned by 0dba399a).
- Each sidecar node must keep references slim but context-rich to minimize agent token cost risk (D8).
- Co-locate sidecar files (e.g., `.ticket/index.toon`, `.spec/index.toon`) right next to their corresponding domain index files (e.g. `.ticket/README.md`) inside the local domain directories, keeping tools isolated.
- Cover all three D1 surfaces: workspace-folder sidecars, folder-level README index nodes, and `.agents/` agent-hook nodes serialize through the same TOON contract.
- Commit the generated `.toon` files to git (D5) so they serve as stable, tracked state.
- Provide a Rust struct and serialization implementation in each domain's library crate.
- Provide a validation command per tool to verify all local source path references exist and check digest stability.

## Acceptance criteria
- Sidecar is serialized to TOON format and co-located within the isolated domain store directory.
- The sidecar can be parsed and queried for node IDs, keywords, and digests without reading the full markdown file.
- The sidecar is committed to git and updated on staged changes during the profiled pre-commit hook (D2).
- A command-line validator flags broken references or non-stale digests.

## Non-goals
- No global merged sidecar across different tools.
- No central `.context/` store folder.
- No JSON-L primary storage (opt-in JSON export is fine, but TOON is primary, D8).

## Resolved design decisions
- D8: TOON primary, slim-but-dense references. D5: committed to git. D2: profiled pre-commit regeneration. D1: shared contract across workspace, folder-README, and `.agents/` surfaces.
Depends on the ContextNode schema ticket (0dba399a).