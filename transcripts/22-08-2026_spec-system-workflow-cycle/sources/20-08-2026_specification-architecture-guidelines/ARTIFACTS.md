# Artifact Inventory

| Artifact | Current state | Relevance |
|---|---|---|
| `.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml` | Reviewed Presentation System manifest | Records only basic metadata; it is the concrete example under review. |
| `.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/body.md` | Active prose body | Contains decisions, requirements, acceptance criteria, deferred ideas, and multiple delivery tracks in one document. |
| `transcripts/20-08-2026_specification-architecture-guidelines/input.clean.md` | Cleaned original design guidance | Defines the requested component, criterion, evidence, and directed-contract semantics. |
| `transcripts/20-08-2026_specification-architecture-guidelines/input-2.clean.md` | Cleaned follow-up | Points at the Presentation System spec as a concrete case study. |
| `transcripts/20-08-2026_specification-architecture-guidelines/merged.clean.md` | Merged design guidance + case study | Folds the follow-up into the original guidance as the single current source for this dossier. |
| `workflow-tools/spec/crates/spec-api/src/manifest.rs` | Existing implementation | Models structured contracts, acceptance criteria, evidence requirements, fulfillment summaries, and related-ticket accessors. |
| `workflow-tools/spec/crates/spec-api/schemas/specification.toml` | Existing schema | Defines currently allowed typed spec edges and their direction/cycle rules. |
| `workflow-tools/spec/crates/spec-api/src/store/sections.rs` | Existing implementation | Provides named spec-section operations. |
| `workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs` | Existing implementation | Provides hierarchy traversal and parent/child support. |
| `workflow-tools/spec/crates/spec-api/src/ticket_ref.rs` | Existing implementation | Defines structured cross-store ticket references. |
| `workflow-tools/spec/crates/spec-api/tests/schema_test.rs` | Existing validation | Covers the specification schema. |

No ticket, spec, code, or store entity was created or changed by this dossier.