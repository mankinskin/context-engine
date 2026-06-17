<!-- spec-index:file generated=true -->

## agent-rules

<!-- spec-index:entry id=347b6f97-5ebf-46c6-a0e1-cc8afc600319 slug=agent-rules/browser-frontend-playwright-mcp digest=79ab84143124 -->
### Prefer MCP Playwright/browser tools for browser frontend validation _(root)_

Browser-hosted frontend testing guidance must prefer MCP Playwright/browser tools before repo-local Playwright wrappers or manual browser steps.

- slug: `agent-rules/browser-frontend-playwright-mcp`
- scope: internal
- tags: agent-rules, draft, root, scope:internal
- ref: `.spec/specs/347b6f97-5ebf-46c6-a0e1-cc8afc600319/spec.toml`

## agent-tooling

<!-- spec-index:entry id=3ccdde3a-368c-4655-a6c8-20a58822c83d slug=agent-tooling/peek-api digest=3e9fe04aa3df -->
### peek-api transport layering _(root)_

Define a reusable `peek-api` layer that owns token-bounded file inspection and structural skeleton rendering so CLI and MCP transports share one contract and one error model.

- slug: `agent-tooling/peek-api`
- scope: internal
- tags: agent-tooling, draft, root, scope:internal
- ref: `.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d/spec.toml`

## agent-workflow

<!-- spec-index:entry id=e101aa4a-f5c1-413b-abb4-f4360c0a1123 slug=agent-workflow/spec-system-guidance digest=51d66975636a -->
### Spec system guidance and Spec Agent workflow _(root)_

Add generated agent guidance for spec-system work so spec creation and updates consistently follow a clear workflow.

- slug: `agent-workflow/spec-system-guidance`
- scope: internal
- tags: agent-workflow, draft, root, scope:internal
- ref: `.spec/specs/e101aa4a-f5c1-413b-abb4-f4360c0a1123/spec.toml`

## audit

<!-- spec-index:entry id=0c3f11d3-2475-470c-a191-beedd2c8e53c slug=audit digest=3e43998120c5 -->
### audit _(root)_

`audit` is the repository quality audit tool for this workspace. Its code is split across three layers:

- slug: `audit`
- scope: system
- tags: audit, draft, root, scope:system
- ref: `memory-viewers/memory-api/.spec/specs/0c3f11d3-2475-470c-a191-beedd2c8e53c/spec.toml`

## audit-api

<!-- spec-index:entry id=5b404022-6a67-4395-90e0-1e4282fd83b4 slug=audit-api/workspace-graph-health-and-board-check-in-validation digest=b18a6a7b70ee -->
### audit-api: workspace graph health and board check-in validation _(root)_

Current topology checks can detect orphan tickets and planned convergence risks, but they do not enforce whether dependency requirements are defined, whether required dependency evidence is passing, …

- slug: `audit-api/workspace-graph-health-and-board-check-in-validation`
- tags: audit-api, draft, root
- ref: `.spec/specs/5b404022-6a67-4395-90e0-1e4282fd83b4/spec.toml`

<!-- spec-index:entry id=5c8b350c-3bd1-4103-ab49-3fb7aea33126 slug=audit-api/store-health-metrics-and-cleanup-feedback-loops digest=e4fc44648a97 -->
### audit-api: store health metrics and cleanup feedback loops _(root)_

Spec, rule, and ticket stores accumulate stale, conflicting, and low-value entries over time. Current checks are useful but insufficient for sustained curation because they do not combine change acti…

- slug: `audit-api/store-health-metrics-and-cleanup-feedback-loops`
- tags: audit-api, draft, root
- ref: `.spec/specs/5c8b350c-3bd1-4103-ab49-3fb7aea33126/spec.toml`

<!-- spec-index:entry id=a6318461-3a06-4d6d-aabb-7e06c33f4e1b slug=audit-api/ticket-dependency-topology-validation digest=bc04ece565a3 -->
### audit-api ticket dependency topology validation _(root)_

`audit-api` should flag orphan tickets so every active ticket participates

- slug: `audit-api/ticket-dependency-topology-validation`
- tags: audit-api, draft, root
- ref: `memory-viewers/memory-api/.spec/specs/a6318461-3a06-4d6d-aabb-7e06c33f4e1b/spec.toml`

## context-engine

<!-- spec-index:entry id=18b6a9c5-7d9c-483c-a228-113903ed495f slug=generated-context/index-hierarchy-semantic-refs digest=b5ac845ba43f -->
### Generated context index hierarchy and semantic reference format _(root)_

Define a generated, human-readable markdown index hierarchy that integrates with the repository structure, and define a compact machine-friendly semantic summary format (TOON) for similarity search, …

- slug: `generated-context/index-hierarchy-semantic-refs`
- scope: internal
- children (6): `generated-context/digest-input-contract`, `generated-context/git-hook-automation`, `generated-context/rendering-pipeline-integration`, `generated-context/thin-generator-architecture`, `generated-context/peek-lod-validation`, `generated-context/benchmarking-profiling-plan`
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/18b6a9c5-7d9c-483c-a228-113903ed495f/spec.toml`

<!-- spec-index:entry id=2860a8db-0c4e-4e94-984a-c10a72a67ffc slug=context-engine/session-worktree-default-workflow digest=db862d94c94d -->
### default worktree-backed session workflow _(root)_

Make dedicated git worktrees the default workflow for new agent sessions in this repository so parallel implementation tracks do not share one staging area.

- slug: `context-engine/session-worktree-default-workflow`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/2860a8db-0c4e-4e94-984a-c10a72a67ffc/spec.toml`

<!-- spec-index:entry id=298f2f92-df7e-4c8a-b4fe-b63c27622142 slug=context-engine/specification-contract/expectation-oriented-spec-contract-and-store-owned-evidence digest=dc59cbbf9c36 -->
### Expectation-oriented specification contract and store-owned evidence workflow _(root)_

Define an expectation-oriented specification contract for this repository so specs capture intended system properties plus explicit acceptance and evidence requirements, while tickets carry problem s…

- slug: `context-engine/specification-contract/expectation-oriented-spec-contract-and-store-owned-evidence`
- tags: context-engine, draft, root
- ref: `.spec/specs/298f2f92-df7e-4c8a-b4fe-b63c27622142/spec.toml`

<!-- spec-index:entry id=38e337c2-cdda-4488-9aa7-b47a300563b0 slug=context-engine/cross-store-workflow-traceability-links digest=a405e78fb707 -->
### cross-store workflow traceability metadata _(root)_

<!-- spec-api:file generated=true -->

- slug: `context-engine/cross-store-workflow-traceability-links`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/38e337c2-cdda-4488-9aa7-b47a300563b0/spec.toml`

<!-- spec-index:entry id=47465a64-0c5f-4ddc-8d38-018048090af2 slug=context-engine/repository-workflow-guidance digest=cb5e00b8ff6d -->
### repository workflow guidance _(root)_

Repository guidance is partly rule-generated today, but nested workspaces still carry hand-written agent files and the parent workspace duplicates child target definitions directly in its own `rule-t…

- slug: `context-engine/repository-workflow-guidance`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/47465a64-0c5f-4ddc-8d38-018048090af2/spec.toml`

<!-- spec-index:entry id=4aa3cbc9-4d95-41ea-8fc3-f4b46eb3483e slug=repo/workspace/root-level-context-stack-layout digest=8e3ae8a38917 -->
### Root-level context-stack workspace layout _(root)_

Relocate the context-stack submodule from crates/context-stack to a top-level context-stack directory and remove deprecated repository folders that should no longer participate in the active workspac…

- slug: `repo/workspace/root-level-context-stack-layout`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/4aa3cbc9-4d95-41ea-8fc3-f4b46eb3483e/spec.toml`

<!-- spec-index:entry id=5b69006d-e2ce-425d-91a7-97f4517a9bc6 slug=context-engine/install-tools-refreshes-viewer-binaries digest=68943f2a5e34 -->
### Install tools refreshes viewer binaries _(root)_

Keep the PATH-first viewer workflow intact by making the shared repo installer refresh every shipped viewer binary.

- slug: `context-engine/install-tools-refreshes-viewer-binaries`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/5b69006d-e2ce-425d-91a7-97f4517a9bc6/spec.toml`

<!-- spec-index:entry id=69f8d8ce-d895-4270-b3f9-0d6c951f2a9a slug=context-engine/agents/prompts/reviews digest=ef82680781e9 -->
### reviews prompt target _(root)_

Define a reusable generated prompt under `.agents/prompts/reviews.prompt.md` for reviewing the highest-ranked `in-review` tickets using the repository ticket workflow.

- slug: `context-engine/agents/prompts/reviews`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/69f8d8ce-d895-4270-b3f9-0d6c951f2a9a/spec.toml`

<!-- spec-index:entry id=6d73975e-5af4-4ac6-b732-33c381bf768d slug=repo-guidance/readmes/context-engine-root digest=87ce9c3849f9 -->
### context-engine root README tree generation

Generate the top-level `context-engine` README tree from the root `.rule` store, including the repo root plus the root-owned first-level child README surfaces that currently break the navigation chai…

- slug: `repo-guidance/readmes/context-engine-root`
- scope: internal
- parent: `repo-guidance/readmes/manual-repos`
- tags: context-engine, draft, scope:internal
- ref: `.spec/specs/6d73975e-5af4-4ac6-b732-33c381bf768d/spec.toml`

<!-- spec-index:entry id=954d9807-f357-41e5-9fd4-b1da39e0933d slug=context-engine/recurring-principles digest=9b971158b2c9 -->
### Recurring cross-cutting principles _(root)_

<!-- spec-api:file generated=true -->

- slug: `context-engine/recurring-principles`
- scope: public
- tags: context-engine, draft, root, scope:public
- ref: `.spec/specs/954d9807-f357-41e5-9fd4-b1da39e0933d/spec.toml`

<!-- spec-index:entry id=96dc0068-d05d-4e61-b785-144272119fa9 slug=context-engine/workflow-guidance-generation-and-session-capture digest=d6853009e5a2 -->
### workflow guidance generation and session capture scaffolding

Generate the requested workflow prompt and agent files from canonical rule-api entries, then seed a bounded first session-api scaffold for storing Copilot chat sessions in the memory-api store.

- slug: `context-engine/workflow-guidance-generation-and-session-capture`
- scope: internal
- parent: `context-engine/workflow-validation-tool`
- children (1): `context-engine/handoff-workflow-prompts`
- tags: context-engine, draft, scope:internal
- ref: `.spec/specs/96dc0068-d05d-4e61-b785-144272119fa9/spec.toml`

<!-- spec-index:entry id=9983b865-5082-437a-945a-05c26a56c113 slug=context-engine/submodule-branch-tracking digest=9314e36b132f -->
### submodule branch tracking workflow _(root)_

Top-level submodules in the repository currently land in detached HEAD state, which makes local commits easy to create without advancing the intended `main` branch in each submodule.

- slug: `context-engine/submodule-branch-tracking`
- scope: internal
- tags: active, context-engine, root, scope:internal
- ref: `.spec/specs/9983b865-5082-437a-945a-05c26a56c113/spec.toml`

<!-- spec-index:entry id=9c3ec5a5-0e1e-4061-b66a-16006611f671 slug=program/store-expansion-and-operational-health digest=d99550e6c790 -->
### Store expansion and operational health program _(root)_

Coordinate the next major store vectors (interview, feedback, health auditing, and scaffold automation) as a dependency-aware program aligned to the cross-store architecture constraints.

- slug: `program/store-expansion-and-operational-health`
- tags: context-engine, draft, root
- ref: `.spec/specs/9c3ec5a5-0e1e-4061-b66a-16006611f671/spec.toml`

<!-- spec-index:entry id=9e04ff58-9160-4766-b307-74c0fb32a92c slug=context-engine/handoff-workflow-prompts digest=5dedf5e07fa3 -->
### handoff workflow prompts

Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session handoffs that help a new session resume a specific implementation track quickly.

- slug: `context-engine/handoff-workflow-prompts`
- scope: internal
- parent: `context-engine/workflow-guidance-generation-and-session-capture`
- tags: context-engine, draft, scope:internal
- ref: `.spec/specs/9e04ff58-9160-4766-b307-74c0fb32a92c/spec.toml`

<!-- spec-index:entry id=a4f48d84-50ed-4769-a42f-38321ea9600c slug=context-engine/workflow-validation-tool digest=68b0fc7e6c0d -->
### workflow validation metadata and default tool behavior _(root)_

<!-- spec-api:file generated=true -->

- slug: `context-engine/workflow-validation-tool`
- scope: internal
- children (1): `context-engine/workflow-guidance-generation-and-session-capture`
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c/spec.toml`

<!-- spec-index:entry id=bcea435e-c397-4e30-846f-c5e58cfa0755 slug=context-engine/smooth-repository-readme-surfaces digest=9cfb5b8830a0 -->
### Smooth repository README surfaces _(root)_

Make the repository-root README surfaces navigable and consistent enough to iterate on without re-auditing the same structural gaps each time.

- slug: `context-engine/smooth-repository-readme-surfaces`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/bcea435e-c397-4e30-846f-c5e58cfa0755/spec.toml`

<!-- spec-index:entry id=cf5e2942-1a47-43cc-a0ee-14e5774680a6 slug=context-engine/workflow-documentation-validation-tooling digest=961467fa93c6 -->
### workflow documentation validation via doc-api and doc-cli _(root)_

<!-- spec-api:file generated=true -->

- slug: `context-engine/workflow-documentation-validation-tooling`
- scope: internal
- tags: context-engine, draft, root, scope:internal
- ref: `.spec/specs/cf5e2942-1a47-43cc-a0ee-14e5774680a6/spec.toml`

<!-- spec-index:entry id=fb968dea-b658-4c0d-b138-266bdd2615f9 slug=sandbox-orchestration-v1-functional digest=7e18d0af4427 -->
### functional v1 sandbox orchestration layer _(root)_

Define the first functional sandbox orchestration slice for this repository's agentic workflow system while keeping memory-stack traceability intact. The first delivery target must produce isolated a…

- slug: `sandbox-orchestration-v1-functional`
- tags: context-engine, draft, root
- ref: `.spec/specs/fb968dea-b658-4c0d-b138-266bdd2615f9/spec.toml`

## context-read

<!-- spec-index:entry id=904871fa-0b97-4484-9540-f2926e32476f slug=context-stack/graph-induction/read-sequence/induced-graph-structure digest=98e603be4f89 -->
### induced graph structure

This internal child spec records the graph facts that must hold after the

- slug: `context-stack/graph-induction/read-sequence/induced-graph-structure`
- scope: internal
- parent: `context-stack/graph-induction/read-sequence`
- tags: context-read, draft, scope:internal
- ref: `.spec/specs/904871fa-0b97-4484-9540-f2926e32476f/spec.toml`

<!-- spec-index:entry id=e0913182-7a5e-4c8f-a750-799afd58baae slug=context-stack/graph-induction/read-sequence/context-read-pipeline digest=0319c9ddf2ff -->
### context-read pipeline

This internal child spec defines the intended `context-read` algorithm for

- slug: `context-stack/graph-induction/read-sequence/context-read-pipeline`
- scope: internal
- parent: `context-stack/graph-induction/read-sequence`
- tags: context-read, draft, scope:internal
- ref: `.spec/specs/e0913182-7a5e-4c8f-a750-799afd58baae/spec.toml`

## context-stack

<!-- spec-index:entry id=0c141c86-957c-4b93-b86e-f50a05e84763 slug=context-stack/tool-history-transplant digest=c20e9527d36c -->
### tool history transplant

This spec records the decision to replace the old bash extraction flow with `crane-cli` for context-stack tool-history migrations.

- slug: `context-stack/tool-history-transplant`
- scope: system
- parent: `context-stack`
- tags: context-stack, draft, scope:system
- ref: `.spec/specs/0c141c86-957c-4b93-b86e-f50a05e84763/spec.toml`

<!-- spec-index:entry id=16c3ad95-451d-4c09-a118-ca90bcefed9a slug=context-stack/graph-induction digest=98cacc4197d1 -->
### graph induction

Graph induction is the part of the context stack that accepts token sequences or

- slug: `context-stack/graph-induction`
- scope: public
- parent: `context-stack`
- children (4): `context-stack/graph-induction/read-sequence`, `context-stack/graph-induction/insert-first-match`, `context-stack/graph-induction/insert-sequences`, `context-stack/graph-induction/insert-sequence`
- tags: context-stack, draft, scope:public
- ref: `.spec/specs/16c3ad95-451d-4c09-a118-ca90bcefed9a/spec.toml`

<!-- spec-index:entry id=7fd5639f-a62b-4eb4-abe2-215c4bb2d0de slug=context-stack/graph-induction/read-sequence digest=936438648b71 -->
### read_sequence

`WorkspaceManager::read_sequence` reads text through the graph and returns one

- slug: `context-stack/graph-induction/read-sequence`
- scope: public
- parent: `context-stack/graph-induction`
- children (2): `context-stack/graph-induction/read-sequence/induced-graph-structure`, `context-stack/graph-induction/read-sequence/context-read-pipeline`
- tags: context-stack, draft, scope:public
- ref: `.spec/specs/7fd5639f-a62b-4eb4-abe2-215c4bb2d0de/spec.toml`

<!-- spec-index:entry id=92112188-8acd-4573-bb22-f784fcf371ca slug=context-stack/graph-induction/insert-first-match digest=4837f80b4897 -->
### insert_first_match

`WorkspaceManager::insert_first_match` induces graph structure from an existing

- slug: `context-stack/graph-induction/insert-first-match`
- scope: public
- parent: `context-stack/graph-induction`
- tags: context-stack, draft, scope:public
- ref: `.spec/specs/92112188-8acd-4573-bb22-f784fcf371ca/spec.toml`

<!-- spec-index:entry id=b8ddb54c-8939-4bbd-a3b3-2e469caa09da slug=repo-guidance/readmes/context-stack digest=a58cc1e86eba -->
### context-stack rule-backed README tree

Give `context-stack` a repo-local rule workspace and generate both its root README and its first-level child README tree from local rules.

- slug: `repo-guidance/readmes/context-stack`
- scope: internal
- parent: `repo-guidance/readmes/manual-repos`
- tags: context-stack, draft, scope:internal
- ref: `.spec/specs/b8ddb54c-8939-4bbd-a3b3-2e469caa09da/spec.toml`

<!-- spec-index:entry id=bbd92962-33f4-4b9e-b301-f4ce9909c135 slug=context-stack/graph-induction/insert-sequences digest=71224cd94b3d -->
### insert_sequences

`WorkspaceManager::insert_sequences` performs bulk graph induction over an

- slug: `context-stack/graph-induction/insert-sequences`
- scope: public
- parent: `context-stack/graph-induction`
- tags: context-stack, draft, scope:public
- ref: `.spec/specs/bbd92962-33f4-4b9e-b301-f4ce9909c135/spec.toml`

<!-- spec-index:entry id=c6feacef-a06e-4769-a4cf-5c557be50f7d slug=context-stack digest=de2b8b3f9fb1 -->
### context-stack _(root)_

This branch captures the most confident specification material currently

- slug: `context-stack`
- scope: system
- children (2): `context-stack/tool-history-transplant`, `context-stack/graph-induction`
- tags: context-stack, draft, root, scope:system
- ref: `.spec/specs/c6feacef-a06e-4769-a4cf-5c557be50f7d/spec.toml`

<!-- spec-index:entry id=e631e914-1840-4fec-9df5-b50a85c6cf00 slug=context-stack/graph-induction/insert-sequence digest=72c4c3e49213 -->
### insert_sequence

`WorkspaceManager::insert_sequence` induces graph structure from a text string.

- slug: `context-stack/graph-induction/insert-sequence`
- scope: public
- parent: `context-stack/graph-induction`
- tags: context-stack, draft, scope:public
- ref: `.spec/specs/e631e914-1840-4fec-9df5-b50a85c6cf00/spec.toml`

## doc-api

<!-- spec-index:entry id=24baf686-38fd-417d-9528-bebc02a556d0 slug=doc-api digest=cb6cffe00add -->
### doc-api _(root)_

`memory-api` should gain a `docs` domain so humans and agents can navigate repository structure through the same family pattern already used for rules, specs, tickets, and audits. The first primitive…

- slug: `doc-api`
- scope: public
- tags: doc-api, draft, root, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/24baf686-38fd-417d-9528-bebc02a556d0/spec.toml`

## feedback-api

<!-- spec-index:entry id=e4ac6ae2-3ef6-4104-911d-eb27fae70d1a slug=feedback-api/structured-feedback-inbox-and-deep-reconciliation-search digest=bd9074e3c084 -->
### feedback-api: structured feedback inbox and deep reconciliation search _(root)_

User and agent feedback is fragmented, hard to query at scale, and not consistently tied to remediation workflows. Teams need an inbox-like store with structured metadata, deep search, and reconcilia…

- slug: `feedback-api/structured-feedback-inbox-and-deep-reconciliation-search`
- tags: draft, feedback-api, root
- ref: `.spec/specs/e4ac6ae2-3ef6-4104-911d-eb27fae70d1a/spec.toml`

## interview-api

<!-- spec-index:entry id=7e9131d1-6c7a-4e59-9208-5990079040d5 slug=interview-api/persistent-interview-sessions-and-survey-synthesis digest=5ebf75465d8e -->
### interview-api: persistent interview sessions and survey synthesis _(root)_

Interview and survey workflows are currently ad hoc and difficult to iterate collaboratively. Responses are not reliably preserved as structured records that can be revised, merged, and turned into a…

- slug: `interview-api/persistent-interview-sessions-and-survey-synthesis`
- tags: draft, interview-api, root
- ref: `.spec/specs/7e9131d1-6c7a-4e59-9208-5990079040d5/spec.toml`

## memory-api

<!-- spec-index:entry id=0c34e5d3-8b30-4b8b-9876-fc8f7e02ed9e slug=memory-api/storage/board digest=c08c75538c0a -->
### board

Source: `crates/memory-api/src/storage/board.rs`

- slug: `memory-api/storage/board`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/0c34e5d3-8b30-4b8b-9876-fc8f7e02ed9e/spec.toml`

<!-- spec-index:entry id=12045379-ce22-416b-8a5e-31d560c35992 slug=memory-api/model/filesystem digest=cbe29a3bcfbb -->
### filesystem

Source: `crates/memory-api/src/model/filesystem.rs`

- slug: `memory-api/model/filesystem`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/12045379-ce22-416b-8a5e-31d560c35992/spec.toml`

<!-- spec-index:entry id=121a0e79-a212-40af-8041-83c1ea11f029 slug=memory-api/storage/indexed digest=12172a5ac480 -->
### indexed

Source: `crates/memory-api/src/storage/indexed.rs`

- slug: `memory-api/storage/indexed`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/121a0e79-a212-40af-8041-83c1ea11f029/spec.toml`

<!-- spec-index:entry id=2594c83e-12e3-49d9-a5b7-abe7122e5f52 slug=memory-api/storage/entity-store digest=c5ea44804026 -->
### entity_store

Source: `crates/memory-api/src/storage/entity_store.rs`

- slug: `memory-api/storage/entity-store`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/2594c83e-12e3-49d9-a5b7-abe7122e5f52/spec.toml`

<!-- spec-index:entry id=267acfdc-7df4-4090-9f17-7d6d71232814 slug=memory-api/model/entity digest=0ef7329ad980 -->
### entity

Source: `crates/memory-api/src/model/entity.rs`

- slug: `memory-api/model/entity`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/267acfdc-7df4-4090-9f17-7d6d71232814/spec.toml`

<!-- spec-index:entry id=3235e977-b12a-4981-aa09-6edf218bb97d slug=memory-api/model/schema digest=1d29c3e323f0 -->
### schema

Source: `crates/memory-api/src/model/schema.rs`

- slug: `memory-api/model/schema`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/3235e977-b12a-4981-aa09-6edf218bb97d/spec.toml`

<!-- spec-index:entry id=449fe68a-541c-4804-bbfd-476af783f80c slug=generated-context/digest-input-contract digest=9438d05156ed -->
### Domain digest input contract for generated index entries

Define the domain-level digest input contract for generated memory-index entries so every generator derives a stable `IndexEntry` payload before calling `seal()`. Given identical source inputs, every…

- slug: `generated-context/digest-input-contract`
- scope: internal
- parent: `generated-context/index-hierarchy-semantic-refs`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/449fe68a-541c-4804-bbfd-476af783f80c/spec.toml`

<!-- spec-index:entry id=4a16fea7-5af5-477f-835d-6c0c94216bfe slug=memory-api/error digest=032cb1be270b -->
### error

Source: `crates/memory-api/src/error.rs`

- slug: `memory-api/error`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/4a16fea7-5af5-477f-835d-6c0c94216bfe/spec.toml`

<!-- spec-index:entry id=4f7d84d0-9876-43d5-9dd6-90a7f3ebc56c slug=repo-guidance/readmes/memory-api-adoption digest=be13abf8e5e0 -->
### memory-api README schema adoption

Migrate `memory-api` from bespoke README target structure to the shared schema and extend its generated tool README surfaces with parent links back to the repo root.

- slug: `repo-guidance/readmes/memory-api-adoption`
- scope: internal
- parent: `repo-guidance/readmes/generated-repos`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/4f7d84d0-9876-43d5-9dd6-90a7f3ebc56c/spec.toml`

<!-- spec-index:entry id=53c70cae-731b-41b5-bd1a-1de9a98eb36f slug=generated-context/git-hook-automation digest=25999655f5e0 -->
### Git hook automation for store-index regeneration

Define the repository-local git-hook automation contract for store-index regeneration so generator tickets have one concrete execution surface instead of each hand-waving at "pre-commit/post-commit h…

- slug: `generated-context/git-hook-automation`
- scope: internal
- parent: `generated-context/index-hierarchy-semantic-refs`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/53c70cae-731b-41b5-bd1a-1de9a98eb36f/spec.toml`

<!-- spec-index:entry id=6571abcf-b1b9-4259-b81c-78783e227467 slug=architecture/cross-store-workspace-interaction digest=a1322f2e7e69 -->
### Cross-store workspace interaction architecture _(root)_

Define a workspace architecture where each store remains domain-isolated while cross-store interaction is enabled through contract interfaces and API-layer composition.

- slug: `architecture/cross-store-workspace-interaction`
- scope: public
- tags: draft, memory-api, root, scope:public
- ref: `.spec/specs/6571abcf-b1b9-4259-b81c-78783e227467/spec.toml`

<!-- spec-index:entry id=6e63979a-f29b-4c6f-a4b7-5264fd9c29d4 slug=memory-api/cli/toon-format-support digest=a1ba6b40accd -->
### Add TOON format support across the memory-api CLI suite _(root)_

Add a compact TOON machine-readable format alongside existing JSON output across the memory-api CLI suite.

- slug: `memory-api/cli/toon-format-support`
- scope: internal
- tags: draft, memory-api, root, scope:internal
- ref: `.spec/specs/6e63979a-f29b-4c6f-a4b7-5264fd9c29d4/spec.toml`

<!-- spec-index:entry id=8074d6f7-b888-4e57-95e6-06dde96384b8 slug=memory-api digest=5aca386722a6 -->
### memory-api _(root)_

Bootstrapped from source analysis.

- slug: `memory-api`
- scope: public
- children (19): `ticket-query/expressive-query-and-ordering`, `memory-api/storage/board`, `memory-api/model/filesystem`, `memory-api/storage/indexed`, `memory-api/storage/entity-store`, `memory-api/model/entity`, `memory-api/model/schema`, `memory-api/error`, `memory-api/storage/index`, `memory-api/model/query`, `memory-api/workspace`, `memory-api/storage/store-bootstrap-open`, `memory-api/model/schema-registry`, `memory-api/model/edge`, `ticket-cli/board-option-naming`, `memory-api/model/manifest-format`, `memory-api/storage/entity-fs`, `memory-api/storage/search`, `memory-api/storage/schema`
- tags: draft, memory-api, root, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/8074d6f7-b888-4e57-95e6-06dde96384b8/spec.toml`

<!-- spec-index:entry id=86b37a8b-4798-4cfe-971e-d53bb842ae80 slug=memory-api/storage/index digest=1b88108c229d -->
### index

Source: `crates/memory-api/src/storage/index.rs`

- slug: `memory-api/storage/index`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/86b37a8b-4798-4cfe-971e-d53bb842ae80/spec.toml`

<!-- spec-index:entry id=9109f12a-cc02-47ae-948f-98008b6c167d slug=generated-context/rendering-pipeline-integration digest=8855a57171ff -->
### Shared rendering pipeline integration for generated store indexes

Define the shared rendering-pipeline integration for generated store indexes so README/index files route through one rendering paradigm instead of per-domain ad hoc renderers. Store-index generation …

- slug: `generated-context/rendering-pipeline-integration`
- scope: internal
- parent: `generated-context/index-hierarchy-semantic-refs`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/9109f12a-cc02-47ae-948f-98008b6c167d/spec.toml`

<!-- spec-index:entry id=9c33e0fa-2a18-45ba-a837-a04d71638944 slug=memory-api/model/query digest=5c59f3d82bd1 -->
### query

Source: `crates/memory-api/src/model/query.rs`

- slug: `memory-api/model/query`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/9c33e0fa-2a18-45ba-a837-a04d71638944/spec.toml`

<!-- spec-index:entry id=ae5ef697-0ee5-4f74-9dca-2cb268290dae slug=memory-api/workspace digest=4d9d32ae8d78 -->
### workspace

Source: `crates/memory-api/src/workspace.rs`

- slug: `memory-api/workspace`
- scope: public
- parent: `memory-api`
- children (3): `ticket-api/workflow/unblocked-by-discovery`, `ticket-api/workspaces/ancestor-dependency-visibility`, `ticket-api/workflow/best-next-ordering`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/ae5ef697-0ee5-4f74-9dca-2cb268290dae/spec.toml`

<!-- spec-index:entry id=b9129cf5-ddad-49f5-9dd0-60fdb72ee306 slug=memory-api/storage/store-bootstrap-open digest=9dd96ee8aef1 -->
### store bootstrap open

The local memory-api store wrappers currently expose two low-level entry points:

- slug: `memory-api/storage/store-bootstrap-open`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/b9129cf5-ddad-49f5-9dd0-60fdb72ee306/spec.toml`

<!-- spec-index:entry id=bb77b62c-bb18-4274-8ba2-7543f6d81ff5 slug=memory-api/model/schema-registry digest=bae14915971c -->
### schema_registry

Source: `crates/memory-api/src/model/schema_registry.rs`

- slug: `memory-api/model/schema-registry`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/bb77b62c-bb18-4274-8ba2-7543f6d81ff5/spec.toml`

<!-- spec-index:entry id=be7d7ae7-bb1f-4438-813d-f69f17d65cf6 slug=memory-api/install-contracts/cli-and-viewer-installation digest=d623c938a8b0 -->
### CLI and Viewer Installation Contract _(root)_

This spec defines the canonical installation contract for the `memory-api` operator surfaces that are exposed to users through generated README documentation. The goal is to keep one synchronized ins…

- slug: `memory-api/install-contracts/cli-and-viewer-installation`
- scope: public
- tags: draft, memory-api, root, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/be7d7ae7-bb1f-4438-813d-f69f17d65cf6/spec.toml`

<!-- spec-index:entry id=bf217ce5-8890-4749-9a2d-deffb6d0f4dd slug=generated-context/thin-generator-architecture digest=d789ec0c4865 -->
### Domain-owned thin generator architecture for store indexes

Define the architecture boundary for store-index generation so each domain owns a **thin** generator while `memory-api` exposes only reusable, domain-agnostic infrastructure. An implementer must be a…

- slug: `generated-context/thin-generator-architecture`
- scope: internal
- parent: `generated-context/index-hierarchy-semantic-refs`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/bf217ce5-8890-4749-9a2d-deffb6d0f4dd/spec.toml`

<!-- spec-index:entry id=c4f7b0ae-9690-4cc2-b25f-c8ec49a504d0 slug=generated-context/peek-lod-validation digest=1d5cba4363f0 -->
### peek-cli consumption and level-of-detail validation for generated indexes

Define the `peek-cli` consumption and level-of-detail (LOD) validation plan for generated store indexes, so generators are reviewed against token-efficient agent consumption, not just correctness. Ge…

- slug: `generated-context/peek-lod-validation`
- scope: internal
- parent: `generated-context/index-hierarchy-semantic-refs`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/c4f7b0ae-9690-4cc2-b25f-c8ec49a504d0/spec.toml`

<!-- spec-index:entry id=c598ddb2-4d3a-4b81-90ea-8b25a54b8469 slug=generated-context/benchmarking-profiling-plan digest=4f30bbb21316 -->
### Benchmarking and profiling plan for store-index generation

Define the benchmarking and profiling plan for store-index generation so hook automation and generator implementations share explicit latency budgets, a repeatable measurement method, and evidence re…

- slug: `generated-context/benchmarking-profiling-plan`
- scope: internal
- parent: `generated-context/index-hierarchy-semantic-refs`
- tags: draft, memory-api, scope:internal
- ref: `.spec/specs/c598ddb2-4d3a-4b81-90ea-8b25a54b8469/spec.toml`

<!-- spec-index:entry id=d3dea825-4850-4a95-a03d-4884ce85a232 slug=memory-api/model/edge digest=66e865cf3037 -->
### edge

Source: `crates/memory-api/src/model/edge.rs`

- slug: `memory-api/model/edge`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/d3dea825-4850-4a95-a03d-4884ce85a232/spec.toml`

<!-- spec-index:entry id=de14117a-df7b-4f2a-87b8-55074ade0487 slug=memory-api/model/manifest-format digest=bad558034568 -->
### manifest_format

Source: `crates/memory-api/src/model/manifest_format.rs`

- slug: `memory-api/model/manifest-format`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/de14117a-df7b-4f2a-87b8-55074ade0487/spec.toml`

<!-- spec-index:entry id=f081e8f4-7d28-4505-9faf-85c65b033b44 slug=memory-api/storage/entity-fs digest=8be3e3ac2de9 -->
### entity_fs

Source: `crates/memory-api/src/storage/entity_fs.rs`

- slug: `memory-api/storage/entity-fs`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/f081e8f4-7d28-4505-9faf-85c65b033b44/spec.toml`

<!-- spec-index:entry id=f156fa16-7910-4c98-b69c-f848073dba00 slug=memory-api/storage/search digest=506819504d14 -->
### search

Source: `crates/memory-api/src/storage/search.rs`

- slug: `memory-api/storage/search`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/f156fa16-7910-4c98-b69c-f848073dba00/spec.toml`

<!-- spec-index:entry id=f9c32554-9884-41c4-8b5b-d1d32b37e341 slug=memory-api/recurring-principles digest=c0211a8c9e3c -->
### memory-api recurring principles _(root)_

<!-- spec-api:file generated=true -->

- slug: `memory-api/recurring-principles`
- scope: public
- tags: draft, memory-api, root, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/f9c32554-9884-41c4-8b5b-d1d32b37e341/spec.toml`

<!-- spec-index:entry id=fded16bb-1a4b-4dd9-a610-c26459d19403 slug=memory-api/storage/schema digest=c15b2b3b9c77 -->
### schema

Source: `crates/memory-api/src/storage/schema.rs`

- slug: `memory-api/storage/schema`
- scope: public
- parent: `memory-api`
- tags: draft, memory-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/fded16bb-1a4b-4dd9-a610-c26459d19403/spec.toml`

## memory-viewers

<!-- spec-index:entry id=cfbb4500-4632-4a95-96a1-838dc4dccfd5 slug=repo-guidance/readmes/memory-viewers-adoption digest=fd33892af44b -->
### memory-viewers aggregate README schema adoption

Adopt the shared README schema in the aggregate `memory-viewers` repo root and normalize its child blocks after the `memory-api` and `viewer-api` child roots settle.

- slug: `repo-guidance/readmes/memory-viewers-adoption`
- scope: internal
- parent: `repo-guidance/readmes/generated-repos`
- tags: draft, memory-viewers, scope:internal
- ref: `.spec/specs/cfbb4500-4632-4a95-96a1-838dc4dccfd5/spec.toml`

## probe

<!-- spec-index:entry id=a67fdd95-a5c7-44f4-a861-8b427f1923cd slug=probe/spec digest=ec03ced99f5a -->
### Probe Spec _(root)_

- slug: `probe/spec`
- tags: draft, probe, root
- ref: `memory-viewers/memory-api/.spec/specs/a67fdd95-a5c7-44f4-a861-8b427f1923cd/spec.toml`

## repo-guidance

<!-- spec-index:entry id=3c4c0c4f-7d5c-4fa5-9c45-037420e56cee slug=repo-guidance/readmes/generated-repos digest=21515ea016a9 -->
### Generated repository README schema adoption

Adopt the shared README schema across the existing generated repos in the `memory-viewers` family so those workspaces stop carrying bespoke README target layouts and gain consistent parent/child navi…

- slug: `repo-guidance/readmes/generated-repos`
- scope: internal
- parent: `repo-guidance/readmes/shared-schema-rollout`
- children (3): `repo-guidance/readmes/memory-api-adoption`, `repo-guidance/readmes/viewer-api-adoption`, `repo-guidance/readmes/memory-viewers-adoption`
- tags: draft, repo-guidance, scope:internal
- ref: `.spec/specs/3c4c0c4f-7d5c-4fa5-9c45-037420e56cee/spec.toml`

<!-- spec-index:entry id=59216a24-598e-48eb-a280-ee8766857c0b slug=repo-guidance/readmes/manual-repos digest=f47559291807 -->
### Manual repository README generation rollout

Migrate the manual README trees in `context-engine` and `context-stack` onto the same rule-backed generation flow and parent/child navigation contract already used in the generated nested workspaces.

- slug: `repo-guidance/readmes/manual-repos`
- scope: internal
- parent: `repo-guidance/readmes/shared-schema-rollout`
- children (2): `repo-guidance/readmes/context-engine-root`, `repo-guidance/readmes/context-stack`
- tags: draft, repo-guidance, scope:internal
- ref: `.spec/specs/59216a24-598e-48eb-a280-ee8766857c0b/spec.toml`

<!-- spec-index:entry id=7b0ad285-c7df-4b19-a9d7-a11e71bab2ba slug=context-engine/repo-guidance/cline-agent-integration digest=b212f6bf270a -->
### Cline agent client integration with client-agnostic .agents/ standard _(root)_

Integrate the Cline Agent Client (uses `.clinerules/` by default) while maintaining client agnosticity. The canonical standard remains `.agents/` + `AGENTS.md` as the source of truth for all agent gu…

- slug: `context-engine/repo-guidance/cline-agent-integration`
- scope: internal
- tags: draft, repo-guidance, root, scope:internal
- ref: `.spec/specs/7b0ad285-c7df-4b19-a9d7-a11e71bab2ba/spec.toml`

<!-- spec-index:entry id=b9bf7713-5644-413d-a99d-00866828d534 slug=repo-guidance/readmes/shared-schema-rollout digest=7f759867829b -->
### README shared schema rollout _(root)_

Standardize repository README generation across `context-engine`, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api` so the repo-root and first-level child README trees share one rule-…

- slug: `repo-guidance/readmes/shared-schema-rollout`
- scope: internal
- children (4): `repo-guidance/readmes/generated-repos`, `repo-guidance/readmes/manual-repos`, `repo-guidance/readmes/completeness-audit`, `repo-guidance/readmes/shared-schema`
- tags: draft, repo-guidance, root, scope:internal
- ref: `.spec/specs/b9bf7713-5644-413d-a99d-00866828d534/spec.toml`

<!-- spec-index:entry id=c163eba4-51c5-496e-88cd-ef8bcd8fe433 slug=repo-guidance/readmes/completeness-audit digest=c27607852d19 -->
### README completeness audit

Add a mechanical completeness check for the README contract so missing generation, parent or child navigation blocks, installable-content sections, or command-doc references fail before review.

- slug: `repo-guidance/readmes/completeness-audit`
- scope: internal
- parent: `repo-guidance/readmes/shared-schema-rollout`
- tags: draft, repo-guidance, scope:internal
- ref: `.spec/specs/c163eba4-51c5-496e-88cd-ef8bcd8fe433/spec.toml`

## rule-api

<!-- spec-index:entry id=0e9beee3-9974-48df-ad22-a79504c438af slug=rule-api/store digest=095c43e3a4ca -->
### store _(root)_

Source:

- slug: `rule-api/store`
- scope: public
- tags: draft, root, rule-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/0e9beee3-9974-48df-ad22-a79504c438af/spec.toml`

<!-- spec-index:entry id=3b96ec1c-4e99-48f4-86e5-a36ba24b827a slug=rule-api/workspaces/memory-api-readme-generation digest=efd9a3af655d -->
### memory-api Rule Workspace and README Generation

`memory-api` needs its own repo-local rule workspace so the repo README and local usage guides are authored next to the crates and tools they describe. The local target config should stay manageable …

- slug: `rule-api/workspaces/memory-api-readme-generation`
- scope: public
- parent: `rule-api/workspaces`
- tags: draft, rule-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/3b96ec1c-4e99-48f4-86e5-a36ba24b827a/spec.toml`

<!-- spec-index:entry id=5de125ad-eb0c-4bcb-8e6d-175df1ba33a6 slug=rule-api/workspaces/nested-resolution digest=46319357f3b1 -->
### Nested Workspace Discovery and Target Resolution

Nested rule workspaces should extend the existing `rule-api` store and target model from "one store + one config" into an explicitly scanned workspace graph. The owning repo workspace remains the uni…

- slug: `rule-api/workspaces/nested-resolution`
- scope: public
- parent: `rule-api/workspaces`
- tags: draft, rule-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/5de125ad-eb0c-4bcb-8e6d-175df1ba33a6/spec.toml`

<!-- spec-index:entry id=cde871d1-d390-454e-ae0b-94b152baca15 slug=repo-guidance/readmes/shared-schema digest=c411feb0f86c -->
### Shared README schema and validation

Add a shared README schema layer to `rule-api` so repository README targets can inherit a standard structure, enforce required blocks, and expose enough validation detail to fail fast when navigation…

- slug: `repo-guidance/readmes/shared-schema`
- scope: internal
- parent: `repo-guidance/readmes/shared-schema-rollout`
- tags: draft, rule-api, scope:internal
- ref: `.spec/specs/cde871d1-d390-454e-ae0b-94b152baca15/spec.toml`

<!-- spec-index:entry id=e815f261-ca7d-4957-8f68-666e1e1dfbfe slug=rule-api/workspaces digest=e76b6ba303d7 -->
### Rule Workspace Topology and Composition _(root)_

`rule-api` already supports canonical rule storage in `.rule/rules/**`, repo-scoped filtering, hierarchical target outlines, and deterministic generation from a single `rule-targets.yaml`. The next s…

- slug: `rule-api/workspaces`
- scope: public
- children (2): `rule-api/workspaces/memory-api-readme-generation`, `rule-api/workspaces/nested-resolution`
- tags: draft, root, rule-api, scope:public
- ref: `memory-viewers/memory-api/.spec/specs/e815f261-ca7d-4957-8f68-666e1e1dfbfe/spec.toml`

## scaffold

<!-- spec-index:entry id=9ee9387f-5384-42a9-95c4-ecbad1713030 slug=scaffold/rule-generated-domain-store-bootstrap-instructions-and-slash-skill digest=f78479657e96 -->
### scaffold: rule-generated domain-store bootstrap instructions and slash skill _(root)_

Creating new domain stores repeatedly is slow and inconsistent. Teams need one prompt-driven bootstrap flow that generates a minimally functional store while preserving architecture constraints and r…

- slug: `scaffold/rule-generated-domain-store-bootstrap-instructions-and-slash-skill`
- tags: draft, root, scaffold
- ref: `.spec/specs/9ee9387f-5384-42a9-95c4-ecbad1713030/spec.toml`

## session-api

<!-- spec-index:entry id=09f96d83-4795-4f19-9259-64ad0d452387 slug=context-engine/session-api/vscode-copilot-stop-hook-capture digest=8c5db5a1d295 -->
### VS Code Copilot stop-hook session capture _(root)_

Wire the repository's VS Code GitHub Copilot hook configuration to persist chat sessions through `session-api` after each agent response stops.

- slug: `context-engine/session-api/vscode-copilot-stop-hook-capture`
- scope: internal
- tags: draft, root, scope:internal, session-api
- ref: `.spec/specs/09f96d83-4795-4f19-9259-64ad0d452387/spec.toml`

<!-- spec-index:entry id=36fd7849-65eb-405e-8cc5-70440f0cb7c2 slug=memory-api/session-api/hook-ingestion-read-query digest=1a40a694efab -->
### session-api hook ingestion and read query _(root)_

Extend `session-api` so repeated Copilot hook captures preserve transcript history as an append-only log and expose a first read/query API over the persisted store.

- slug: `memory-api/session-api/hook-ingestion-read-query`
- scope: internal
- tags: draft, root, scope:internal, session-api
- ref: `memory-viewers/memory-api/.spec/specs/36fd7849-65eb-405e-8cc5-70440f0cb7c2/spec.toml`

<!-- spec-index:entry id=823b22cf-c0dc-46c6-a03d-00cdd3c4c83a slug=memory-api/session-api/persistence-writer digest=783349e035b9 -->
### session-api persistence writer _(root)_

Persist `session-api` capture requests into a deterministic filesystem layout that can become the first memory-api-backed session store.

- slug: `memory-api/session-api/persistence-writer`
- scope: internal
- tags: draft, root, scope:internal, session-api
- ref: `memory-viewers/memory-api/.spec/specs/823b22cf-c0dc-46c6-a03d-00cdd3c4c83a/spec.toml`

## spec-api

<!-- spec-index:entry id=1cf68c36-7f64-4d81-b553-1947b978fbe3 slug=spec-api/generated-documents digest=17ec5d8ab4a2 -->
### generated documents

<!-- spec-api:file generated=true -->

- slug: `spec-api/generated-documents`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/1cf68c36-7f64-4d81-b553-1947b978fbe3/spec.toml`

<!-- spec-index:entry id=226ff55f-eebf-43b8-aa1e-5abf81b99101 slug=spec-api/manifest digest=6e9a97d53bff -->
### manifest

Source: `crates/spec-api/src/manifest.rs`

- slug: `spec-api/manifest`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/226ff55f-eebf-43b8-aa1e-5abf81b99101/spec.toml`

<!-- spec-index:entry id=32eaa05c-cef6-4a3b-b506-b5a5410a4674 slug=spec-api/code-ref digest=4926172b435c -->
### code_ref

Source: `crates/spec-api/src/code_ref.rs`

- slug: `spec-api/code-ref`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/32eaa05c-cef6-4a3b-b506-b5a5410a4674/spec.toml`

<!-- spec-index:entry id=351389c0-0873-4c3c-bc46-3551459ba1cd slug=spec-api/store digest=ee4292e9c739 -->
### store

Source: `crates/spec-api/src/store.rs`

- slug: `spec-api/store`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/351389c0-0873-4c3c-bc46-3551459ba1cd/spec.toml`

<!-- spec-index:entry id=597de059-859d-4aae-89ad-bfe2d84462c0 slug=spec-api/default-schema digest=02b6427ae7d6 -->
### default_schema

Source: `crates/spec-api/src/default_schema.rs`

- slug: `spec-api/default-schema`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/597de059-859d-4aae-89ad-bfe2d84462c0/spec.toml`

<!-- spec-index:entry id=83094beb-d315-4b16-b132-3ae22a528422 slug=spec-api/error digest=a50577891c31 -->
### error

Source: `crates/spec-api/src/error.rs`

- slug: `spec-api/error`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/83094beb-d315-4b16-b132-3ae22a528422/spec.toml`

<!-- spec-index:entry id=86877b97-2df6-46b5-923d-a9e608655fed slug=spec-api digest=720199458597 -->
### spec-api _(root)_

Bootstrapped from source analysis.

- slug: `spec-api`
- scope: public
- children (7): `spec-api/generated-documents`, `spec-api/manifest`, `spec-api/code-ref`, `spec-api/store`, `spec-api/default-schema`, `spec-api/error`, `spec-api/slug`
- tags: draft, root, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/86877b97-2df6-46b5-923d-a9e608655fed/spec.toml`

<!-- spec-index:entry id=b0b3c0c6-168f-4b00-93b2-e06ce506855a slug=spec-api/slug digest=82c12b0596cd -->
### slug

Source: `crates/spec-api/src/slug.rs`

- slug: `spec-api/slug`
- scope: public
- parent: `spec-api`
- tags: draft, scope:public, spec-api
- ref: `memory-viewers/memory-api/.spec/specs/b0b3c0c6-168f-4b00-93b2-e06ce506855a/spec.toml`

## spec-cli

<!-- spec-index:entry id=1d6481ff-308b-45ca-ac91-4c2f8d5546f3 slug=spec-cli digest=44294971c91a -->
### spec-cli _(root)_

Bootstrapped from source analysis.

- slug: `spec-cli`
- scope: public
- children (2): `spec-cli/cli/commands/bootstrap`, `spec-cli/cli`
- tags: draft, root, scope:public, spec-cli
- ref: `memory-viewers/memory-api/.spec/specs/1d6481ff-308b-45ca-ac91-4c2f8d5546f3/spec.toml`

<!-- spec-index:entry id=91b5b5fe-0046-424c-b5e5-3fb961f4d940 slug=spec-cli/cli/args digest=2f7a3e5ba610 -->
### args

Source: `tools/cli/spec-cli/src/cli/args.rs`

- slug: `spec-cli/cli/args`
- scope: public
- parent: `spec-cli/cli`
- tags: draft, scope:public, spec-cli
- ref: `memory-viewers/memory-api/.spec/specs/91b5b5fe-0046-424c-b5e5-3fb961f4d940/spec.toml`

<!-- spec-index:entry id=ec1013ff-2d5f-4860-8d25-64acb6c60587 slug=spec-cli/cli/commands/bootstrap digest=3767899b1677 -->
### bootstrap

Source: `tools/cli/spec-cli/src/cli/commands/bootstrap.rs`

- slug: `spec-cli/cli/commands/bootstrap`
- scope: public
- parent: `spec-cli`
- tags: draft, scope:public, spec-cli
- ref: `memory-viewers/memory-api/.spec/specs/ec1013ff-2d5f-4860-8d25-64acb6c60587/spec.toml`

<!-- spec-index:entry id=ee36710b-1725-436a-a5fc-1e5a7d27f6bb slug=spec-cli/cli digest=1af70957d27c -->
### cli

Source: `tools/cli/spec-cli/src/cli.rs`

- slug: `spec-cli/cli`
- scope: public
- parent: `spec-cli`
- children (1): `spec-cli/cli/args`
- tags: draft, scope:public, spec-cli
- ref: `memory-viewers/memory-api/.spec/specs/ee36710b-1725-436a-a5fc-1e5a7d27f6bb/spec.toml`

## spec-editor

<!-- spec-index:entry id=18d7cbc2-d454-498b-9f10-9f1a0d320f2e slug=spec-editor/body-editor digest=4c2b9ec1e284 -->
### spec-editor: body editor

The `BodyEditor` component is the core authoring surface for a spec's `body.md`.

- slug: `spec-editor/body-editor`
- scope: component
- parent: `spec-editor`
- tags: draft, scope:component, spec-editor
- ref: `.spec/specs/18d7cbc2-d454-498b-9f10-9f1a0d320f2e/spec.toml`

<!-- spec-index:entry id=62f6b6d6-eb0c-43cc-ad3e-a75347f2a705 slug=spec-editor/state-machine digest=0498203f02e6 -->
### spec-editor: state machine control

The `StateTransition` component surfaces the spec state machine as an interactive

- slug: `spec-editor/state-machine`
- scope: component
- parent: `spec-editor`
- tags: draft, scope:component, spec-editor
- ref: `.spec/specs/62f6b6d6-eb0c-43cc-ad3e-a75347f2a705/spec.toml`

<!-- spec-index:entry id=788e91e4-32d7-4ff5-bf68-485235f8211f slug=spec-editor digest=cb58c51b4fe8 -->
### spec-editor _(root)_

`spec-editor` is a **fully interactive, GPU-accelerated specification editor**. It

- slug: `spec-editor`
- scope: system
- children (4): `spec-editor/body-editor`, `spec-editor/state-machine`, `spec-editor/section-editor`, `spec-editor/coderef-editor`
- tags: draft, root, scope:system, spec-editor
- ref: `.spec/specs/788e91e4-32d7-4ff5-bf68-485235f8211f/spec.toml`

<!-- spec-index:entry id=cbac783c-e3a1-4a31-ac11-a539b39164c5 slug=spec-editor/section-editor digest=7534e2532089 -->
### spec-editor: section editor

The `SectionPanel` and `SectionEditor` components manage a spec's named subdocuments

- slug: `spec-editor/section-editor`
- scope: component
- parent: `spec-editor`
- tags: draft, scope:component, spec-editor
- ref: `.spec/specs/cbac783c-e3a1-4a31-ac11-a539b39164c5/spec.toml`

<!-- spec-index:entry id=ffb89723-178b-435b-8591-c639d3c7d5dd slug=spec-editor/coderef-editor digest=1ad295b64ff7 -->
### spec-editor: coderef editor

The `CodeRefEditor` component provides a three-step picker for adding or editing

- slug: `spec-editor/coderef-editor`
- scope: component
- parent: `spec-editor`
- tags: draft, scope:component, spec-editor
- ref: `.spec/specs/ffb89723-178b-435b-8591-c639d3c7d5dd/spec.toml`

## spec-http

<!-- spec-index:entry id=067c60c2-6be0-4f51-bbd3-80dcd29b9d35 slug=spec-http/routes digest=2d5c1c48f938 -->
### routes

Source: `tools/http/spec-http/src/routes.rs`

- slug: `spec-http/routes`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/067c60c2-6be0-4f51-bbd3-80dcd29b9d35/spec.toml`

<!-- spec-index:entry id=0df82e71-79ef-48ec-8459-43b4693d4543 slug=spec-http/handlers/tree digest=a2d48121a300 -->
### tree

Source: `tools/http/spec-http/src/handlers/tree.rs`

- slug: `spec-http/handlers/tree`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/0df82e71-79ef-48ec-8459-43b4693d4543/spec.toml`

<!-- spec-index:entry id=0f684a90-6abe-4028-a9f1-5d2d6322f1b9 slug=spec-http/state digest=86f5c3eb9dee -->
### state

Source: `tools/http/spec-http/src/state.rs`

- slug: `spec-http/state`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/0f684a90-6abe-4028-a9f1-5d2d6322f1b9/spec.toml`

<!-- spec-index:entry id=26f51d64-3bf7-4907-ae9d-2f006bb7d1fa slug=spec-http/error digest=e453e3e26d90 -->
### error

Source: `tools/http/spec-http/src/error.rs`

- slug: `spec-http/error`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/26f51d64-3bf7-4907-ae9d-2f006bb7d1fa/spec.toml`

<!-- spec-index:entry id=4482f2d0-131c-4ea0-91c0-f0bde198eedd slug=spec-http/handlers/sections digest=928c9eefbc46 -->
### sections

Source: `tools/http/spec-http/src/handlers/sections.rs`

- slug: `spec-http/handlers/sections`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/4482f2d0-131c-4ea0-91c0-f0bde198eedd/spec.toml`

<!-- spec-index:entry id=717be077-49f6-49bc-8221-ea5105ba5ff3 slug=spec-http/handlers/health digest=34da3915acce -->
### health

Source: `tools/http/spec-http/src/handlers/health.rs`

- slug: `spec-http/handlers/health`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/717be077-49f6-49bc-8221-ea5105ba5ff3/spec.toml`

<!-- spec-index:entry id=77ca6056-180d-4e88-a76a-dcf8ff280c7a slug=spec-http/handlers/specs digest=c1938a76e701 -->
### specs

Source: `tools/http/spec-http/src/handlers/specs.rs`

- slug: `spec-http/handlers/specs`
- scope: public
- parent: `spec-http`
- tags: draft, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/77ca6056-180d-4e88-a76a-dcf8ff280c7a/spec.toml`

<!-- spec-index:entry id=db5c02e5-31f5-4a40-bf9c-2227eab412b2 slug=spec-http digest=d48eef023697 -->
### spec-http _(root)_

Bootstrapped from source analysis.

- slug: `spec-http`
- scope: public
- children (7): `spec-http/routes`, `spec-http/handlers/tree`, `spec-http/state`, `spec-http/error`, `spec-http/handlers/sections`, `spec-http/handlers/health`, `spec-http/handlers/specs`
- tags: draft, root, scope:public, spec-http
- ref: `memory-viewers/memory-api/.spec/specs/db5c02e5-31f5-4a40-bf9c-2227eab412b2/spec.toml`

## spec-mcp

<!-- spec-index:entry id=4066c5a3-3e04-4eeb-81a2-91fb8e53cc94 slug=spec-mcp digest=af1a831e005f -->
### spec-mcp _(root)_

Bootstrapped from source analysis.

- slug: `spec-mcp`
- scope: public
- children (1): `spec-mcp/server`
- tags: draft, root, scope:public, spec-mcp
- ref: `memory-viewers/memory-api/.spec/specs/4066c5a3-3e04-4eeb-81a2-91fb8e53cc94/spec.toml`

<!-- spec-index:entry id=d4831fdc-9790-4a26-ac32-d675bb6a7792 slug=spec-mcp/server digest=a1023d62fdb0 -->
### server

Source: `tools/mcp/spec-mcp/src/server.rs`

- slug: `spec-mcp/server`
- scope: public
- parent: `spec-mcp`
- tags: draft, scope:public, spec-mcp
- ref: `memory-viewers/memory-api/.spec/specs/d4831fdc-9790-4a26-ac32-d675bb6a7792/spec.toml`

## spec-viewer

<!-- spec-index:entry id=8c2bbb42-4ca0-43a3-8196-334218bf543e slug=spec-viewer/spec-tree digest=161247b51d35 -->
### spec-viewer: spec-tree component

The `SpecTree` Dioxus component renders the spec hierarchy as a collapsible tree.

- slug: `spec-viewer/spec-tree`
- scope: component
- parent: `spec-viewer`
- tags: draft, scope:component, spec-viewer
- ref: `memory-viewers/.spec/specs/8c2bbb42-4ca0-43a3-8196-334218bf543e/spec.toml`

<!-- spec-index:entry id=95562129-87d8-4d86-9cd3-c8a09473a6d0 slug=spec-viewer/routes digest=24f04e68bd83 -->
### spec-viewer: routes and pages

Defines the Dioxus `Route` enum and the four page-level components for the spec-viewer SPA.

- slug: `spec-viewer/routes`
- scope: component
- parent: `spec-viewer`
- tags: draft, scope:component, spec-viewer
- ref: `memory-viewers/.spec/specs/95562129-87d8-4d86-9cd3-c8a09473a6d0/spec.toml`

<!-- spec-index:entry id=c5b11920-6ae8-4686-ae07-b8e9f8100bf8 slug=spec-viewer/theme-settings digest=92338d7d792b -->
### spec-viewer: theme settings

<!-- spec-api:file generated=true -->

- slug: `spec-viewer/theme-settings`
- scope: component
- parent: `spec-viewer`
- tags: draft, scope:component, spec-viewer
- ref: `memory-viewers/.spec/specs/c5b11920-6ae8-4686-ae07-b8e9f8100bf8/spec.toml`

<!-- spec-index:entry id=d8b515b8-86fd-4366-9fff-1946f43c3030 slug=spec-viewer digest=3640f2218fae -->
### spec-viewer _(root)_

`spec-viewer` is a single-process, GPU-accelerated web application for **reading and

- slug: `spec-viewer`
- scope: system
- children (3): `spec-viewer/spec-tree`, `spec-viewer/routes`, `spec-viewer/theme-settings`
- tags: draft, root, scope:system, spec-viewer
- ref: `memory-viewers/.spec/specs/d8b515b8-86fd-4366-9fff-1946f43c3030/spec.toml`

## test

<!-- spec-index:entry id=227df77c-c047-4a96-bab3-57b6cda0773d slug=test/foo digest=1bb55fb19107 -->
### test _(root)_

Canonical specification for the **HTTP server bootstrap** primitives exported

- slug: `test/foo`
- tags: draft, root, test
- ref: `.spec/specs/227df77c-c047-4a96-bab3-57b6cda0773d/spec.toml`

## ticket-api

<!-- spec-index:entry id=0b1888f2-7e59-45fb-95d8-1bf14ff7747f slug=ticket-api/workspaces/ancestor-dependency-visibility digest=83e0ccd69f19 -->
### Ancestor Workspace Ticket References for Child-Workspace Dependencies

Child ticket workspaces need a way to surface ancestor-owned ticket entries when those parent entries participate directly in dependency relationships with child-owned tickets.

- slug: `ticket-api/workspaces/ancestor-dependency-visibility`
- scope: public
- parent: `memory-api/workspace`
- tags: draft, scope:public, ticket-api
- ref: `memory-viewers/memory-api/.spec/specs/0b1888f2-7e59-45fb-95d8-1bf14ff7747f/spec.toml`

<!-- spec-index:entry id=1d62442b-61dc-4eeb-9b7c-e933f84470f2 slug=ticket-api/state-transition-path-unification digest=5b35c0813650 -->
### ticket-api state transition path unification _(root)_

Unify ticket state transition handling so `update_ticket` and `close_ticket` share one schema-validated transition-path implementation, remove reliance on caller-supplied `from_state`, and support ex…

- slug: `ticket-api/state-transition-path-unification`
- scope: internal
- tags: draft, root, scope:internal, ticket-api
- ref: `.spec/specs/1d62442b-61dc-4eeb-9b7c-e933f84470f2/spec.toml`

<!-- spec-index:entry id=4bd3cd3f-5851-4d9e-b499-978cb7b53275 slug=ticket-api/workflow/graph-aware-best-next digest=d8517bf1ee16 -->
### Graph-aware best-next ranking and dependency convergence _(root)_

The current best-next contract is deterministic but shallow: default next discovery ranks only dependency-satisfied candidates by candidate workflow state, priority, immediate dependees, and recency.…

- slug: `ticket-api/workflow/graph-aware-best-next`
- scope: public
- children (1): `ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering`
- tags: draft, root, scope:public, ticket-api
- ref: `memory-viewers/memory-api/.spec/specs/4bd3cd3f-5851-4d9e-b499-978cb7b53275/spec.toml`

<!-- spec-index:entry id=9074b2ef-c8fe-4bb0-a987-87063078c1ff slug=ticket-api/workflow/cross-surface-contract-parity-and-validation digest=6d91ee5d9723 -->
### Workflow and health parity for ticket interfaces _(root)_

Ticket CLI, ticket HTTP, and ticket MCP currently return different workflow and health answers for the same store because parity-critical domain behavior still lives in their interface crates instead…

- slug: `ticket-api/workflow/cross-surface-contract-parity-and-validation`
- scope: internal
- tags: draft, root, scope:internal, ticket-api
- ref: `.spec/specs/9074b2ef-c8fe-4bb0-a987-87063078c1ff/spec.toml`

<!-- spec-index:entry id=a595eb0c-f9f1-4e29-a425-120df5334f7d slug=ticket-api/workflow/scoped-selector-contract digest=f837381ed4c9 -->
### Scoped selector contract for board and next workflow discovery _(root)_

`ticket board show`, `ticket next`, MCP `board_show`/`next_tickets`, and

- slug: `ticket-api/workflow/scoped-selector-contract`
- scope: internal
- tags: draft, root, scope:internal, ticket-api
- ref: `.spec/specs/a595eb0c-f9f1-4e29-a425-120df5334f7d/spec.toml`

<!-- spec-index:entry id=b4d038e0-ade9-459b-8ba3-92fd81d80e6a slug=ticket-api/validation-aware-dependency-requirements-and-health digest=58851127be4c -->
### ticket-api: validation-aware dependency requirements and health model _(root)_

`depends_on` currently expresses structural ordering only. Validation requirements still live implicitly in ticket prose or ad hoc review habits, so the graph cannot answer whether a dependency has b…

- slug: `ticket-api/validation-aware-dependency-requirements-and-health`
- tags: draft, root, ticket-api
- ref: `.spec/specs/b4d038e0-ade9-459b-8ba3-92fd81d80e6a/spec.toml`

<!-- spec-index:entry id=d702ed9e-f75c-4727-8f05-1b2b244ec74f slug=ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering digest=e801fe811f1e -->
### Blocker trees and recently-unblocked workflow ordering

The current workflow surface has two strong but separate pieces:

- slug: `ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering`
- scope: public
- parent: `ticket-api/workflow/graph-aware-best-next`
- tags: draft, scope:public, ticket-api
- ref: `memory-viewers/memory-api/.spec/specs/d702ed9e-f75c-4727-8f05-1b2b244ec74f/spec.toml`

<!-- spec-index:entry id=e09cb882-4146-47c5-b14e-ad35a8f53173 slug=ticket-api/storage/file-backed-edge-persistence digest=b5166b730b6f -->
### Persist dependency edges in tracked ticket files _(root)_

Dependency edges created through `ticket link` and removed through `ticket unlink` must survive a rebuild from the tracked ticket store, not only the ignored SQLite index.

- slug: `ticket-api/storage/file-backed-edge-persistence`
- scope: internal
- tags: draft, root, scope:internal, ticket-api
- ref: `memory-viewers/memory-api/.spec/specs/e09cb882-4146-47c5-b14e-ad35a8f53173/spec.toml`

<!-- spec-index:entry id=ec22fe34-2d24-4dc5-a067-85121bed3655 slug=ticket-api/workflow/best-next-ordering digest=c72d90485612 -->
### Cross-interface best-next ordering

Best-next-ticket discovery must remain consistent anywhere the repository surfaces candidate work.

- slug: `ticket-api/workflow/best-next-ordering`
- scope: public
- parent: `memory-api/workspace`
- tags: draft, scope:public, ticket-api
- ref: `memory-viewers/memory-api/.spec/specs/ec22fe34-2d24-4dc5-a067-85121bed3655/spec.toml`

<!-- spec-index:entry id=fa5265cc-e82f-4bbe-b1b5-dad7e6672d06 slug=ticket-api/model/tracker-improvement-effort-field digest=531ab06c13e4 -->
### Tracker-improvement effort field _(root)_

The built-in `tracker-improvement` ticket schema includes an optional `effort` field used to capture the estimated token budget required to complete the ticket.

- slug: `ticket-api/model/tracker-improvement-effort-field`
- scope: internal
- tags: draft, root, scope:internal, ticket-api
- ref: `.spec/specs/fa5265cc-e82f-4bbe-b1b5-dad7e6672d06/spec.toml`

## ticket-cli

<!-- spec-index:entry id=0386c4d0-15c4-4561-a33f-63b881c852c5 slug=ticket-api/workflow/unblocked-by-discovery digest=9c85b3d8fb8e -->
### CLI reverse-dependency unlock and blocker follow-up discovery

The ticket CLI needs first-class reverse-dependency workflow support: `ticket unblocked-by <id>` should show which dependents a prerequisite unlocks or still affects, and `ticket next <id>` should sh…

- slug: `ticket-api/workflow/unblocked-by-discovery`
- scope: public
- parent: `memory-api/workspace`
- tags: draft, scope:public, ticket-cli
- ref: `memory-viewers/memory-api/.spec/specs/0386c4d0-15c4-4561-a33f-63b881c852c5/spec.toml`

<!-- spec-index:entry id=42e8d710-2199-4178-9ab3-dea8d61bfc4a slug=ticket-cli/graph-rendering-and-closure-aware-dependency-display digest=3eee9ade04b1 -->
### ticket-cli: graph rendering and closure-aware dependency display _(root)_

The ticket graph is queryable, but operators still have to mentally reconstruct dependency shape, bridge nodes, and parallel tracks from list output. Existing graph-aware next planning improves ranki…

- slug: `ticket-cli/graph-rendering-and-closure-aware-dependency-display`
- tags: draft, root, ticket-cli
- ref: `.spec/specs/42e8d710-2199-4178-9ab3-dea8d61bfc4a/spec.toml`

<!-- spec-index:entry id=ddac3853-65b3-4b6b-b216-81e78e250bb1 slug=ticket-cli/board-option-naming digest=26d87dfc8642 -->
### ticket-cli board option naming

`ticket board` should present the same flag naming style as the rest of `ticket-cli`.

- slug: `ticket-cli/board-option-naming`
- scope: public
- parent: `memory-api`
- tags: draft, scope:public, ticket-cli
- ref: `memory-viewers/memory-api/.spec/specs/ddac3853-65b3-4b6b-b216-81e78e250bb1/spec.toml`

## ticket-http

<!-- spec-index:entry id=5bd4fd1a-cd0e-4893-9d83-452cd5b533bb slug=ticket-http/api/tickets digest=8ea3d9d200f2 -->
### ticket-http: ticket list endpoint _(root)_

Canonical contract for the ticket list API consumed by the Dioxus ticket-viewer explorer and related ticket-picking surfaces.

- slug: `ticket-http/api/tickets`
- scope: public
- tags: draft, root, scope:public, ticket-http
- ref: `memory-viewers/memory-api/.spec/specs/5bd4fd1a-cd0e-4893-9d83-452cd5b533bb/spec.toml`

## ticket-query

<!-- spec-index:entry id=08aa283e-34ee-47d4-83bc-4c4311a9c85f slug=ticket-query/expressive-query-and-ordering digest=e7139b541064 -->
### Expressive ticket query and ordering contract

Ticket discovery is split across several partial query mechanisms with

- slug: `ticket-query/expressive-query-and-ordering`
- scope: public
- parent: `memory-api`
- tags: draft, scope:public, ticket-query
- ref: `memory-viewers/memory-api/.spec/specs/08aa283e-34ee-47d4-83bc-4c4311a9c85f/spec.toml`

## ticket-viewer

<!-- spec-index:entry id=33e731c2-a0cf-41f7-bd7c-2df6c4545bf3 slug=ticket-viewer/explorer digest=64374b4c70db -->
### ticket-viewer: explorer interactions _(root)_

Canonical specification for the ticket-viewer navigation surfaces that let a user find and open tickets: the sidebar explorer and the quick-search overlay.

- slug: `ticket-viewer/explorer`
- scope: component
- tags: draft, root, scope:component, ticket-viewer
- ref: `memory-viewers/.spec/specs/33e731c2-a0cf-41f7-bd7c-2df6c4545bf3/spec.toml`

<!-- spec-index:entry id=5650b307-8408-4ca0-922f-c515dd0bfa27 slug=ticket-viewer/shell digest=34978418090c -->
### ticket-viewer: shell and header actions _(root)_

Canonical specification for the ticket-viewer shell-level interaction contract that sits above the existing explorer and theme-settings specs.

- slug: `ticket-viewer/shell`
- scope: component
- tags: draft, root, scope:component, ticket-viewer
- ref: `memory-viewers/.spec/specs/5650b307-8408-4ca0-922f-c515dd0bfa27/spec.toml`

<!-- spec-index:entry id=6cf1685d-dc05-4022-abbb-efdd8e94af22 slug=ticket-viewer/theme-settings digest=8806f1315f5b -->
### ticket-viewer: theme settings _(root)_

<!-- spec-api:file generated=true -->

- slug: `ticket-viewer/theme-settings`
- scope: component
- tags: draft, root, scope:component, ticket-viewer
- ref: `memory-viewers/.spec/specs/6cf1685d-dc05-4022-abbb-efdd8e94af22/spec.toml`

<!-- spec-index:entry id=8c4d51ef-c0b6-437d-bc14-672b0802cef2 slug=ticket-viewer/detail-document-and-focused-graph digest=c016b5351bd8 -->
### ticket-viewer: integrated ticket document and focused full-graph view _(root)_

The ticket-viewer main layout still splits ticket reading across separate content and detail panels, while graph mode fetches a fixed-depth root-local subgraph and treats selection changes like graph…

- slug: `ticket-viewer/detail-document-and-focused-graph`
- scope: public
- tags: draft, root, scope:public, ticket-viewer
- ref: `memory-viewers/.spec/specs/8c4d51ef-c0b6-437d-bc14-672b0802cef2/spec.toml`

<!-- spec-index:entry id=98b4f75d-3628-470d-a5cc-c91b6cc9811a slug=ticket-viewer/graph-focus-property-rendering-and-2d-presentation digest=52c382a81b12 -->
### ticket-viewer: graph focus, property-based rendering, and 2D presentation mode _(root)_

The current ticket-viewer graph still centers its presentation around rich DOM ticket cards, limited focus falloff, and a mostly 3D/isometric camera model. That makes dense graphs expensive to render…

- slug: `ticket-viewer/graph-focus-property-rendering-and-2d-presentation`
- scope: public
- tags: active, root, scope:public, ticket-viewer
- ref: `.spec/specs/98b4f75d-3628-470d-a5cc-c91b6cc9811a/spec.toml`

## ticket-vscode

<!-- spec-index:entry id=5d17db06-c8a0-46bd-a22e-8a783643d7a8 slug=ticket-vscode digest=c361c7f945d0 -->
### ticket-vscode Extension _(root)_

The `ticket-vscode` extension (VS Code package id: `ticket-viewer`, v0.1.0) surfaces the ticket graph from a running `ticket-viewer` Axum/Dioxus server directly inside VS Code's activity bar. It allo…

- slug: `ticket-vscode`
- scope: component
- tags: reviewed, root, scope:component, ticket-vscode
- ref: `memory-viewers/memory-api/.spec/specs/5d17db06-c8a0-46bd-a22e-8a783643d7a8/spec.toml`

<!-- spec-index:entry id=a592900c-f513-4ec2-8dd2-53dbd04aac7b slug=ticket-vscode/rust-wasm-port digest=d8001c4b1863 -->
### ticket-vscode Rust/WASM port _(root)_

Port `memory-viewers/memory-api/tools/ticket-vscode` to a dual-host Rust/WASM-backed architecture without breaking the existing ticket browsing workflow. Target: thin JS/TS host shell + Rust/WASM cor…

- slug: `ticket-vscode/rust-wasm-port`
- scope: internal
- tags: reviewed, root, scope:internal, ticket-vscode
- ref: `memory-viewers/memory-api/.spec/specs/a592900c-f513-4ec2-8dd2-53dbd04aac7b/spec.toml`

## viewer-api

<!-- spec-index:entry id=04a264ca-5dd6-44d4-ab5a-165822d85079 slug=viewer-api/source digest=55a6980dd73e -->
### viewer-api: source

Canonical specification for `viewer-api::source` — the safe source-file

- slug: `viewer-api/source`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/04a264ca-5dd6-44d4-ab5a-165822d85079/spec.toml`

<!-- spec-index:entry id=2e56d63e-c53a-45a0-a984-088b3f9da19a slug=viewer-api/keyboard-interaction-model digest=db04cfd27d92 -->
### viewer-api: keyboard interaction model

Draft cross-viewer interaction contract for keyboard ownership, shortcut precedence, and phased rollout across shared viewer surfaces.

- slug: `viewer-api/keyboard-interaction-model`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/2e56d63e-c53a-45a0-a984-088b3f9da19a/spec.toml`

<!-- spec-index:entry id=348e17f7-23a8-4e11-bbb3-224cf0bbe9d6 slug=viewer-api/components/tab-bar digest=aea777ff9bcf -->
### viewer-api: TabBar

Canonical specification for the shared `TabBar` Dioxus component

- slug: `viewer-api/components/tab-bar`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/348e17f7-23a8-4e11-bbb3-224cf0bbe9d6/spec.toml`

<!-- spec-index:entry id=36ebdecb-0b9e-47be-9f44-fe575aa6ad6f slug=viewer-api/theme-settings digest=01e43ac71335 -->
### viewer-api: theme settings (canonical) _(root)_

Canonical UX and behavior specification for the **shared theme-settings panel** used by every Dioxus viewer (`doc-viewer`, `log-viewer`, `ticket-viewer`, `spec-viewer`). The reference implementation …

- slug: `viewer-api/theme-settings`
- scope: public
- tags: draft, root, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/36ebdecb-0b9e-47be-9f44-fe575aa6ad6f/spec.toml`

<!-- spec-index:entry id=479e226a-b4ef-4e30-ade0-ebdabbf956ed slug=viewer-api/tracing/file-sink digest=af7dd1a4b14b -->
### viewer-api: WASM tracing file sink

Specification for shipping `tracing` records emitted by the Dioxus WASM

- slug: `viewer-api/tracing/file-sink`
- scope: public
- parent: `viewer-api/tracing`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/479e226a-b4ef-4e30-ade0-ebdabbf956ed/spec.toml`

<!-- spec-index:entry id=4c3b62b4-1198-4ce2-9ef6-df530f38297e slug=viewer-api/demo-viewer digest=8b44e26f369d -->
### viewer-api: demo-viewer _(root)_

The **demo-viewer** is a reference application that lives inside the `viewer-api`

- slug: `viewer-api/demo-viewer`
- scope: public
- children (17): `viewer-api/source`, `viewer-api/keyboard-interaction-model`, `viewer-api/components/tab-bar`, `viewer-api/components/graph3d`, `viewer-api/session`, `viewer-api/auth-middleware`, `viewer-api/sse`, `viewer-api/server-infra`, `viewer-api/components/icons-spinner`, `viewer-api/components/tree-view`, `viewer-api/components/layout`, `viewer-api/dev-proxy`, `viewer-api/store-primitives`, `viewer-api/client-log`, `viewer-api/pagination-query`, `viewer-api/components/code-viewer`, `viewer-api/effects/wgpu-overlay`
- tags: draft, root, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/4c3b62b4-1198-4ce2-9ef6-df530f38297e/spec.toml`

<!-- spec-index:entry id=4f14356f-c4bd-4554-be1e-35361de241da slug=viewer-api/components/graph3d digest=31f84a37c715 -->
### viewer-api: Graph3D

Canonical specification for the shared 3D dependency-graph Dioxus component

- slug: `viewer-api/components/graph3d`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/4f14356f-c4bd-4554-be1e-35361de241da/spec.toml`

<!-- spec-index:entry id=51c69e48-c8a1-4d45-b050-e06671fe7d71 slug=viewer-api/session digest=5c2f07a3d17b -->
### viewer-api: session

Canonical specification for `viewer-api::session` — the lightweight server

- slug: `viewer-api/session`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/51c69e48-c8a1-4d45-b050-e06671fe7d71/spec.toml`

<!-- spec-index:entry id=52521803-fd21-4b40-a4e5-6801b823d59d slug=viewer-api/auth-middleware digest=488fad23c218 -->
### viewer-api: auth + middleware + error

Canonical specification for the `viewer-api::auth`, `viewer-api::middleware`,

- slug: `viewer-api/auth-middleware`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/52521803-fd21-4b40-a4e5-6801b823d59d/spec.toml`

<!-- spec-index:entry id=54800731-e07f-4fb2-8802-fd7d2acc8c05 slug=viewer-api/sse digest=e172e979f225 -->
### viewer-api: SSE streaming

Canonical specification for `viewer-api::sse` — the server-sent-events

- slug: `viewer-api/sse`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/54800731-e07f-4fb2-8802-fd7d2acc8c05/spec.toml`

<!-- spec-index:entry id=59979a95-a4cb-4aa3-9a79-486b029532a3 slug=viewer-api/server-infra digest=5985515b0c41 -->
### viewer-api: server-infra

Canonical specification for the **HTTP server bootstrap** primitives exported

- slug: `viewer-api/server-infra`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/59979a95-a4cb-4aa3-9a79-486b029532a3/spec.toml`

<!-- spec-index:entry id=5f9a1652-943f-4d98-8812-a4f7ca1d5e61 slug=repo-guidance/readmes/viewer-api-adoption digest=adfa4475fac6 -->
### viewer-api README schema adoption

Migrate `viewer-api` to the shared README schema and extend its generated child README surfaces with parent links back to the repo root.

- slug: `repo-guidance/readmes/viewer-api-adoption`
- scope: internal
- parent: `repo-guidance/readmes/generated-repos`
- tags: draft, scope:internal, viewer-api
- ref: `.spec/specs/5f9a1652-943f-4d98-8812-a4f7ca1d5e61/spec.toml`

<!-- spec-index:entry id=798c9a3c-404a-4842-874d-484edb4209ef slug=viewer-api/recurring-principles digest=d057b8d05b71 -->
### viewer-api recurring principles _(root)_

<!-- spec-api:file generated=true -->

- slug: `viewer-api/recurring-principles`
- scope: public
- tags: draft, root, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/798c9a3c-404a-4842-874d-484edb4209ef/spec.toml`

<!-- spec-index:entry id=7b43dfd1-39aa-4585-b5fe-dc57c6d57eba slug=viewer-api/components/icons-spinner digest=6b80373c006e -->
### viewer-api: icons + Spinner

Canonical specification for the shared icon set and `Spinner` Dioxus

- slug: `viewer-api/components/icons-spinner`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/7b43dfd1-39aa-4585-b5fe-dc57c6d57eba/spec.toml`

<!-- spec-index:entry id=88c88341-5f9c-4e59-87c7-9176e4afc26a slug=temp-review-probe digest=f9d34b193f38 -->
### temp _(root)_

- slug: `temp-review-probe`
- tags: draft, root, viewer-api
- ref: `.spec/specs/88c88341-5f9c-4e59-87c7-9176e4afc26a/spec.toml`

<!-- spec-index:entry id=a20a0395-4f3b-4b55-ba7a-a0c38ba9f7a6 slug=viewer-api/components/tree-view digest=72b1e2674127 -->
### viewer-api: TreeView

Canonical specification for the shared `TreeView` Dioxus component

- slug: `viewer-api/components/tree-view`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/a20a0395-4f3b-4b55-ba7a-a0c38ba9f7a6/spec.toml`

<!-- spec-index:entry id=b06c9df8-2866-433a-af73-ae9b1f4a0f0a slug=viewer-api/tracing digest=888e8ea6d6c4 -->
### viewer-api: structured tracing for WASM frontend _(root)_

Specification for replacing ad-hoc `web_sys::console::log_1!()` calls in the

- slug: `viewer-api/tracing`
- scope: public
- children (1): `viewer-api/tracing/file-sink`
- tags: draft, root, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/b06c9df8-2866-433a-af73-ae9b1f4a0f0a/spec.toml`

<!-- spec-index:entry id=b3362691-09a0-4028-8daa-13b4c4102c15 slug=viewer-api/components/layout digest=d2018177e283 -->
### viewer-api: layout components

Canonical specification for the shared **page-shell** Dioxus components

- slug: `viewer-api/components/layout`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/b3362691-09a0-4028-8daa-13b4c4102c15/spec.toml`

<!-- spec-index:entry id=b748e117-a847-474d-92ee-b58723cee612 slug=viewer-api/dev-proxy digest=70295e90bb1f -->
### viewer-api: dev proxy

Canonical specification for `viewer-api::dev_proxy` — the optional

- slug: `viewer-api/dev-proxy`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/b748e117-a847-474d-92ee-b58723cee612/spec.toml`

<!-- spec-index:entry id=baaa35ff-4eb6-4288-b4d3-257311b98aa4 slug=viewer-api/store-primitives digest=4f0bf160fbaa -->
### viewer-api: store primitives

Canonical specification for the shared client-side store helpers under

- slug: `viewer-api/store-primitives`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/baaa35ff-4eb6-4288-b4d3-257311b98aa4/spec.toml`

<!-- spec-index:entry id=bca2c4a5-b39e-4896-91f2-8453a1f4ff60 slug=viewer-api/graph-improvements-generalization digest=ac19c9b20cfc -->
### Generalize graph improvements across all memory-viewers _(root)_

The four graph improvements implemented in ticket-viewer need to be generalized to spec-viewer and log-viewer:

- slug: `viewer-api/graph-improvements-generalization`
- scope: public
- tags: draft, root, scope:public, viewer-api
- ref: `.spec/specs/bca2c4a5-b39e-4896-91f2-8453a1f4ff60/spec.toml`

<!-- spec-index:entry id=c6e3cc79-a2de-49f1-9c99-effe1b64a873 slug=viewer-api/client-log digest=c48c17a479b6 -->
### viewer-api: client log endpoint

Canonical specification for `viewer-api::client_log` — the server endpoint

- slug: `viewer-api/client-log`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/c6e3cc79-a2de-49f1-9c99-effe1b64a873/spec.toml`

<!-- spec-index:entry id=c9b40e5d-1239-4ad6-99b1-0b759a9c4c49 slug=viewer-api/pagination-query digest=d67b72609eff -->
### viewer-api: pagination + query helpers

Canonical specification for `viewer-api::pagination` and `viewer-api::query`

- slug: `viewer-api/pagination-query`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/c9b40e5d-1239-4ad6-99b1-0b759a9c4c49/spec.toml`

<!-- spec-index:entry id=d8c6114b-1188-4bc4-a8fb-dbfd3b1816ee slug=probe digest=81667b44a148 -->
### probe _(root)_

- slug: `probe`
- tags: draft, root, viewer-api
- ref: `.spec/specs/d8c6114b-1188-4bc4-a8fb-dbfd3b1816ee/spec.toml`

<!-- spec-index:entry id=df67eee9-08a0-4a6e-b1ff-b483599d232d slug=viewer-api/components/code-viewer digest=be93132b2f4f -->
### viewer-api: CodeViewer + FileContentViewer

Canonical specification for the shared code/file display Dioxus components

- slug: `viewer-api/components/code-viewer`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/df67eee9-08a0-4a6e-b1ff-b483599d232d/spec.toml`

<!-- spec-index:entry id=f153483c-f984-4564-94ac-36234b5cbe3f slug=viewer-api/effects/wgpu-overlay digest=fd4c82fa87a8 -->
### viewer-api: WebGPU overlay

Canonical specification for the WebGPU overlay subsystem under

- slug: `viewer-api/effects/wgpu-overlay`
- scope: public
- parent: `viewer-api/demo-viewer`
- tags: draft, scope:public, viewer-api
- ref: `memory-viewers/viewer-api/.spec/specs/f153483c-f984-4564-94ac-36234b5cbe3f/spec.toml`

## viewer-ctl

<!-- spec-index:entry id=01f7eae8-555d-46e2-bb54-0e0bf2b2da90 slug=viewer-ctl/lifecycle/task digest=5336a74c9d0d -->
### tasks

A task is an ordered list of shell command invocations. Tasks are the

- slug: `viewer-ctl/lifecycle/task`
- scope: internal
- parent: `viewer-ctl`
- tags: draft, scope:internal, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/01f7eae8-555d-46e2-bb54-0e0bf2b2da90/spec.toml`

<!-- spec-index:entry id=351e65fe-0629-4a0f-9c19-27dabb36b72f slug=viewer-ctl/lifecycle/server digest=c8851508c8ab -->
### server lifecycle

Servers are long-running Rust binaries that bind a TCP port. viewer-ctl owns

- slug: `viewer-ctl/lifecycle/server`
- scope: internal
- parent: `viewer-ctl`
- tags: draft, scope:internal, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/351e65fe-0629-4a0f-9c19-27dabb36b72f/spec.toml`

<!-- spec-index:entry id=3fa36e9e-b097-4566-90e4-7d5f8053cd55 slug=viewer-ctl/config digest=225fd185aec6 -->
### configuration

The component registry that drives viewer-ctl is a single TOML file at the

- slug: `viewer-ctl/config`
- scope: internal
- parent: `viewer-ctl`
- tags: draft, scope:internal, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/3fa36e9e-b097-4566-90e4-7d5f8053cd55/spec.toml`

<!-- spec-index:entry id=4dafde12-e894-43d7-aa65-cda9a072be27 slug=viewer-ctl/cli digest=c6442bc3f712 -->
### command-line interface

The viewer-ctl command-line interface is defined by `clap` in `src/cli.rs`.

- slug: `viewer-ctl/cli`
- scope: public
- parent: `viewer-ctl`
- tags: draft, scope:public, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/4dafde12-e894-43d7-aa65-cda9a072be27/spec.toml`

<!-- spec-index:entry id=86bb3a01-ef29-4c7b-905b-9582a0d75f40 slug=viewer-ctl/process-management digest=ebea197637c5 -->
### process management

viewer-ctl needs to find and terminate processes that occupy a TCP port.

- slug: `viewer-ctl/process-management`
- scope: internal
- parent: `viewer-ctl`
- tags: draft, scope:internal, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/86bb3a01-ef29-4c7b-905b-9582a0d75f40/spec.toml`

<!-- spec-index:entry id=afe17aef-793e-419f-93c1-568bf10a2955 slug=viewer-ctl/install-layout digest=c28cc725b08e -->
### install layout

viewer-ctl installs three classes of artifacts. The layout is fixed and

- slug: `viewer-ctl/install-layout`
- scope: public
- parent: `viewer-ctl`
- tags: draft, scope:public, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/afe17aef-793e-419f-93c1-568bf10a2955/spec.toml`

<!-- spec-index:entry id=b568bb7a-6726-46ac-bb78-fbc1858da4b8 slug=viewer-ctl/lifecycle/extension digest=0fcc6d360373 -->
### extension lifecycle

VS Code extensions are TypeScript projects compiled to `out/`. viewer-ctl

- slug: `viewer-ctl/lifecycle/extension`
- scope: internal
- parent: `viewer-ctl`
- tags: draft, scope:internal, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/b568bb7a-6726-46ac-bb78-fbc1858da4b8/spec.toml`

<!-- spec-index:entry id=b7ac0b69-ed06-473a-8fbe-0058a769bf40 slug=viewer-ctl digest=b4adbba58493 -->
### viewer-ctl _(root)_

`viewer-ctl` is the **lifecycle manager** for context-engine viewer servers,

- slug: `viewer-ctl`
- scope: system
- children (8): `viewer-ctl/lifecycle/task`, `viewer-ctl/lifecycle/server`, `viewer-ctl/config`, `viewer-ctl/cli`, `viewer-ctl/process-management`, `viewer-ctl/install-layout`, `viewer-ctl/lifecycle/extension`, `viewer-ctl/lifecycle/frontend`
- tags: draft, root, scope:system, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/b7ac0b69-ed06-473a-8fbe-0058a769bf40/spec.toml`

<!-- spec-index:entry id=c23166c7-2315-4b89-9160-cde7df3086e6 slug=viewer-ctl/lifecycle/frontend digest=a376da77bec7 -->
### frontend lifecycle

Frontends are static-asset bundles produced by `trunk` (Dioxus/WASM) or

- slug: `viewer-ctl/lifecycle/frontend`
- scope: internal
- parent: `viewer-ctl`
- tags: draft, scope:internal, viewer-ctl
- ref: `memory-viewers/viewer-api/.spec/specs/c23166c7-2315-4b89-9160-cde7df3086e6/spec.toml`
