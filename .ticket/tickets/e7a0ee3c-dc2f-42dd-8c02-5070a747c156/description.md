Design and implement the compact machine-readable TOON sidecar emitted alongside every memory-api store index README. This sidecar is the primary surface for similarity search, RAG retrieval, and automated agent tooling, co-located directly inside the store's domain-specific workspace folder.

## Scope
- Define the sidecar format: a compact TOON file with a dense sequence of serialized `IndexEntry` objects (schema owned by `0dba399a`).
- Each `IndexEntry` must keep references slim but context-rich to minimize agent token cost risk (D8).
- Co-locate sidecar files (e.g., `.ticket/index.toon`, `.spec/index.toon`) right next to their corresponding domain README files inside the local domain directories, keeping tools isolated.
- All three D1 surfaces share the same TOON contract: workspace-folder sidecars, folder-level README index entries, and `.agents/` agent-hook entries all serialize as `IndexEntry`.
- Commit the generated `.toon` files to git (D5) so they serve as stable, tracked state.
- Provide a Rust struct and serialization implementation in each domain's library crate inside memory-api.
- Provide a validation command per tool to verify all local source path references exist and check digest stability.

## Acceptance criteria
- Sidecar is serialized to TOON format and co-located within the isolated domain store directory.
- The sidecar can be parsed and queried for entry IDs, keywords, and digests without reading the full markdown file.
- The sidecar is committed to git and updated on staged changes during the profiled pre-commit hook (D2).
- A command-line validator flags broken references or unstale digests.

## Non-goals
- No global merged sidecar across different tools.
- No central `.context/` store folder.
- No JSON-L primary storage (opt-in JSON export is fine; TOON is primary per D8).
- No modification of context-stack crates.

## Resolved design decisions
- D8: TOON primary, slim-but-dense references. D5: committed to git. D2: profiled pre-commit regeneration. D1: shared contract across workspace, folder-README, and `.agents/` surfaces.
Depends on the IndexEntry schema ticket (0dba399a).