<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=4195dfd9-f5d8-46b8-aa1f-ccbb436602e2 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/goal/l3 -->
## Goal

Define first-class, reversible workflow traceability metadata across tickets, specs, documentation, validation specifications and results, and validation logs.

<!-- spec-api:entry id=bc3fb8b0-cb2a-44b8-a1f8-cf629fd88173 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/goal/l7 -->
The source of truth for workflow links should live in the memory stores and shared libraries themselves, with existing tool surfaces returning those identities directly.

<!-- spec-api:entry id=50272e3c-303a-44ff-85e5-aa27f83ce3d8 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/problem/l9 -->
## Problem

Workflow traceability currently depends on manual path references, and the current prototype moved structured links into wrapper-owned artifact payloads. That is not authoritative enough for the intended architecture and does not make the owning store responsible for identity, lookup, or migration.

<!-- spec-api:entry id=c6c6e713-f6f2-4afe-ac09-bf44810e2201 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/scope/l13 -->
## Scope

Rewrite the first cross-store traceability slice around first-class metadata rather than wrapper-owned artifacts.

<!-- spec-api:entry id=3331cbd5-53e7-405a-a57b-ee8f1586e336 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/scope/l17 -->
The first implementation slice should:

<!-- spec-api:entry id=750f5b93-6419-4b15-9f29-1b38b4d5dba9 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/scope/l19 -->
- define a minimal cross-store identity and reference model spanning tickets, specs, docs, future `test-api` records, and future `log-api` records
- support linking workflow state from native store metadata rather than from a dedicated wrapper artifact store
- be queryable through existing or planned memory-system tool surfaces
- preserve current markdown path links as compatibility presentation, not source of truth
- describe migration from any existing wrapper-owned link payloads if they need to be retained temporarily

<!-- spec-api:entry id=01546d9a-dc74-4a9b-ac2e-a326cc6214e8 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/architecture-direction/l25 -->
## Architecture direction

The target architecture is:

<!-- spec-api:entry id=70b23f62-9967-4715-b0fd-a0a6244615da slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/architecture-direction/l29 -->
- each participating store owns the identities of its records and exposes them through shared-library APIs
- `ticket-api`, `spec-api`, and `doc-api` can reference or query workflow-linked records directly through native metadata
- future `test-api` records and `log-api` records have their own native identifiers and queryable metadata
- CLI, MCP, and HTTP surfaces return authoritative identities and can additionally render compatible markdown/path links for humans
- wrapper-owned workflow artifact payloads are not authoritative and, if retained temporarily, exist only for migration compatibility

<!-- spec-api:entry id=4d121bb2-8946-4e8a-9bc5-ee5b1042b6b1 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/non-goals/l35 -->
## Non-goals

- migrating every historic markdown path reference in one pass
- preserving a wrapper-owned link registry as the canonical source of truth
- solving every frontend visualization or query UX in the same change
- introducing a cross-product object model broader than the workflow-linked ticket/spec/doc/test/log surfaces needed here

<!-- spec-api:entry id=700050bd-fc14-4362-aabc-a65e81e422c0 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/acceptance-criteria/l42 -->
## Acceptance criteria

- Wrapper-owned workflow artifacts are no longer treated as the authoritative source of workflow links.
- First-class reversible identities span tickets, specs, docs, future `test-api` records, and future `log-api` records.
- Retrieval is defined through the existing memory-system tools and shared libraries rather than a dedicated workflow wrapper.
- Markdown/path references are retained only as compatibility presentation or migration output.
- Migration expectations from the current prototype link payloads are documented.

<!-- spec-api:entry id=a38bb2f3-07eb-41e2-bfca-c6d91c3fa2cb slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/prototype-status/l50 -->
## Prototype status

- Existing wrapper-owned link payloads are prototype context only.
- Any retained prototype link data must be treated as migration input into store-owned metadata, not as long-term source of truth.

<!-- spec-api:entry id=2892c145-154f-43f2-9562-096e7250f103 slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/validation-results/l55 -->
## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

<!-- spec-api:entry id=d3ff6d67-f2d7-4670-94c1-170b717f3ba3 slug=context-engine/workflow-shared/validation-results -->
## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

<!-- spec-api:entry id=8ed78a60-e315-4594-8256-2a2a30b1998a slug=context-engine/cross-store-workflow-traceability-links/cross-store-workflow-traceability-metadata/traceability/l59 -->
## Traceability

- [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274)
- [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a)
- [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)
