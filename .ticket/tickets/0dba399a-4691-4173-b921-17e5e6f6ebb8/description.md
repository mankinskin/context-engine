Define the canonical `IndexEntry` schema used by every generated memory-api store index artifact. `IndexEntry` represents a single entity captured in a domain index — a ticket, spec, rule, audit finding, test case, etc. `IndexRef` is a typed cross-reference link between index entries. These types live entirely in the memory-api crate stack; they have no relationship to context-stack types.

## Scope
- Define an `IndexEntry` struct (Rust with serde derives) with fields: id, kind, source_path, title, summary, keywords, scope, non_goals, relations (parent, children, depends_on, related), digest, tags, generated_at, source_modified_at.
- Define an `IndexRef` struct for child/parent references: canonical path, entry_id, relation_kind, digest, content_kind, optional anchor.
- Define the `RelationKind` enum: parent, child, depends_on, related, blocks, supersedes.
- Define the `ContentKind` enum: ticket, spec, rule, test, audit_finding, workspace_summary, rule_catalog, index, agent_hook. The `agent_hook` variant covers entries emitted under `.agents/` for agent-client consumption (D1 third surface).
- The schema must support all three D1 placement surfaces without prescribing layout: folder-level README index entries, workspace-folder index entries (e.g. `.ticket/`), and `.agents/` agent-hook entries.
- Write the digest normalization rules: hash input fields, algorithm (stable, compact), stability contract (digest must not change when source has not changed).

## Acceptance criteria
- A single schema file (Rust struct with serde derives) that can be validated.
- Every field has a doc comment explaining its purpose and stability expectations.
- The digest normalization contract is written down and testable: given the same source text and references, the digest is identical across runs.
- The schema compiles cleanly and serializes to both TOON (primary, D8) and JSON (opt-in).
- `ContentKind` includes `agent_hook` so `.agents`-targeted index entries round-trip through the same schema.

## Non-goals
- Does not implement any generator binary.
- Does not define where generated files land in the repo (placement is owned by each generator and the sidecar ticket).
- No dependency on or modification of context-stack crates.

## Resolved design decisions
- D1: index entries live across the entire file tree — folder-level READMEs, workspace-folder indexes, and `.agents/` agent-hook entries.
- D8: TOON is the primary serialization; JSON is opt-in only. References stay slim but dense.
This ticket blocks every generator and the sidecar manifest ticket.