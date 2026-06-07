Build a generator that reads the spec store via spec-api and emits a hierarchical markdown folder tree representing the spec hierarchy under `.spec/` (e.g. `.spec/README.md` as the root, with separate node files and folders for child specs, using relative markdown links). It also emits `.spec/index.toon` as its co-located machine sidecar.

## Scope
- Implement a `spec-index` subcommand (or extend `spec-cli`) that traverses the spec tree depth-first to FULL depth (D3) — there is no default depth cap.
- Output is written under `.spec/` as a directory tree: one markdown file per spec node, with a canonical-named folder per node holding its child nodes, using relative links (e.g., `[My Child](./child/xyz/README.md)`).
- For each spec node: emit title, component, state, summary, acceptance criteria excerpt, and a ContextRef.
- Conforms to the ContextNode schema (0dba399a) and outputs `.spec/index.toon` as the primary machine-readable format (TOON, D8).
- Emit an `.agents/` agent-hook node pointing agents at the spec tree root (D1).
- Regenerates and commits all indices to git (D5) when spec.toml changes are staged in the pre-commit hook (D2).

## Acceptance criteria
- The entire spec tree maps to a full-depth relative markdown folder hierarchy under `.spec/` (one file per node, one canonical folder per node's children).
- Root `.spec/README.md` carries the table of contents.
- Sibling and child navigation uses relative markdown links.
- Co-located `.spec/index.toon` sidecar is generated.
- The digest remains identical on unchanged inputs.

## Non-goals
- No central `.context/` store.
- Does not duplicate full spec bodies.

## Resolved design decisions
- D3: full depth, one file per node, one canonical-named folder per node for its children, relative file links.
- D8: TOON sidecar primary. D2: pre-commit hook. D5: committed to git.