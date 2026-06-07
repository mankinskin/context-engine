Define the canonical ContextNode schema used by every generated context index artifact. This schema is the shared contract that all generators must emit and all search/retrieval tools must consume.

## Scope
- Define a ContextNode struct (or equivalent TOML/JSON schema) with fields: id, kind, source_path, title, summary, keywords, scope, non_goals, relations (parent, children, depends_on, related), digest, tags, generated_at, source_modified_at.
- Define a ContextRef struct for child/parent references: canonical path, node_id, relation_kind, digest, content_kind, optional anchor.
- Define the RelationKind enum: parent, child, depends_on, related, blocks, supersedes.
- Define the ContentKind enum: ticket, spec, rule, test, audit_finding, workspace_summary, rule_catalog, index, agent_hook.
- The schema must support all three D1 placement surfaces without prescribing layout: folder-level README index nodes (anywhere in the file tree), workspace-folder index nodes (e.g. `.ticket/`, `.spec/`), and agent-client consumable instruction-hook nodes emitted under `.agents/` (ContentKind `agent_hook`).
- Write the digest normalization rules: what fields are included in the hash input, hash algorithm (stable, compact), and stability contract (digest must not change when source has not changed).

## Acceptance criteria
- A single schema file (Rust struct with serde derives, or TOML schema) that can be validated.
- Every field has a doc comment explaining its purpose and stability expectations.
- The digest normalization contract is written down and testable: given the same source text and references, the digest is identical across runs.
- The schema compiles cleanly and can be serialized to both TOON (primary, D8) and JSON.
- ContentKind includes `agent_hook` so `.agents`-targeted index nodes round-trip through the same schema.

## Non-goals
- Does not implement any generator binary.
- Does not define where generated files land in the repo (placement is owned by each generator and the sidecar ticket).

## Resolved design decisions
- D1: index nodes live across the entire file tree — folder-level READMEs, workspace-folder indexes, and `.agents/` agent-hook nodes.
- D8: TOON is the primary serialization; JSON is opt-in only. References stay slim but dense.
This ticket blocks every generator and the sidecar manifest ticket.