<!-- aligned-structure:v1 -->

# Summary

Define cross-store workflow traceability as native memory-store metadata instead of wrapper-owned artifact payloads.

## Behavior Story

Workflow traceability should be owned by the participating stores themselves so tickets, specs, docs, and future validation or log records expose authoritative identities directly through shared-library APIs and normal tool surfaces.

## Provided Surface Contracts

- The source of truth for workflow links lives in participating stores rather than in wrapper-owned artifacts.
- `ticket-api`, `spec-api`, and `doc-api` expose or query workflow-linked records through native metadata.
- Future `test-api` and `log-api` records participate through first-class identifiers and queryable metadata.
- CLI, MCP, and HTTP surfaces return authoritative identities and may additionally render markdown/path links as compatibility presentation.
- Any retained wrapper-owned link payloads are migration input only, not canonical state.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274)
- [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a)
- [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)

## Background Knowledge References

- Preserve markdown/path links as compatibility presentation, not source of truth.
- Prefer store-owned identities and shared-library query surfaces over wrapper-owned metadata.

## Legacy Content (Preserved)

## Goal

Define first-class, reversible workflow traceability metadata across tickets, specs, documentation, validation specifications and results, and validation logs.

The source of truth for workflow links should live in the memory stores and shared libraries themselves, with existing tool surfaces returning those identities directly.

## Problem

Workflow traceability currently depends on manual path references, and the current prototype moved structured links into wrapper-owned artifact payloads. That is not authoritative enough for the intended architecture and does not make the owning store responsible for identity, lookup, or migration.

## Scope

Rewrite the first cross-store traceability slice around first-class metadata rather than wrapper-owned artifacts.

The first implementation slice should:

- define a minimal cross-store identity and reference model spanning tickets, specs, docs, future `test-api` records, and future `log-api` records
- support linking workflow state from native store metadata rather than from a dedicated wrapper artifact store
- be queryable through existing or planned memory-system tool surfaces
- preserve current markdown path links as compatibility presentation, not source of truth
- describe migration from any existing wrapper-owned link payloads if they need to be retained temporarily

## Architecture direction

The target architecture is:

- each participating store owns the identities of its records and exposes them through shared-library APIs
- `ticket-api`, `spec-api`, and `doc-api` can reference or query workflow-linked records directly through native metadata
- future `test-api` records and `log-api` records have their own native identifiers and queryable metadata
- CLI, MCP, and HTTP surfaces return authoritative identities and can additionally render compatible markdown/path links for humans
- wrapper-owned workflow artifact payloads are not authoritative and, if retained temporarily, exist only for migration compatibility

## Non-goals

- migrating every historic markdown path reference in one pass
- preserving a wrapper-owned link registry as the canonical source of truth
- solving every frontend visualization or query UX in the same change
- introducing a cross-product object model broader than the workflow-linked ticket/spec/doc/test/log surfaces needed here

## Acceptance criteria

- Wrapper-owned workflow artifacts are no longer treated as the authoritative source of workflow links.
- First-class reversible identities span tickets, specs, docs, future `test-api` records, and future `log-api` records.
- Retrieval is defined through the existing memory-system tools and shared libraries rather than a dedicated workflow wrapper.
- Markdown/path references are retained only as compatibility presentation or migration output.
- Migration expectations from the current prototype link payloads are documented.

## Prototype status

- Existing wrapper-owned link payloads are prototype context only.
- Any retained prototype link data must be treated as migration input into store-owned metadata, not as long-term source of truth.

## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

## Traceability

- [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274)
- [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a)
- [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)
