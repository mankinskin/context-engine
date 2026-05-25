<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=f24d7d4e-070e-4a3f-964e-d3e0bf74274f slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/goal/l3 -->
## Goal

Make documentation validation first-class behavior in the memory-system doc layer rather than a separate wrapper command path.

<!-- spec-api:entry id=7545aa31-1435-4642-a2c0-e79ac0a7cc65 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/goal/l7 -->
Documentation checks, generated-guidance validation, manual verification, and partial-coverage reporting should live in native metadata owned by `doc-api` and surfaced by a future `doc-cli` plus the normal ticket/spec workflow surfaces.

<!-- spec-api:entry id=d1910d40-88e5-4af0-ba8c-d2f634873f6d slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/problem/l9 -->
## Problem

Documentation updates are required by the repository workflow, but the current spec still routes the solution through wrapper-oriented command flows. That keeps doc validation outside the normal memory-system ownership model and leaves `doc-api` without the workflow responsibilities it should own.

<!-- spec-api:entry id=b47b4a8b-f999-4022-9f34-b59f3b20907e slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/scope/l13 -->
## Scope

Rewrite the first workflow-facing documentation validation slice around `doc-api` and a future `doc-cli`.

<!-- spec-api:entry id=8d479e84-5979-4007-b110-3809818b547c slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/scope/l17 -->
The first implementation slice should:

<!-- spec-api:entry id=5d1af0cd-20dd-438c-aefc-827fc67a4580 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/scope/l19 -->
- define native documentation-validation metadata in `doc-api`
- support validation records for authored docs, generated guidance surfaces, and manual verification steps
- allow unsupported or partial coverage to be reported explicitly in native workflow metadata
- define how documentation validation links to tickets, specs, and future `test-api` / `log-api` entities
- define `doc-cli` as a thin CLI over `doc-api`, not as a second storage model

<!-- spec-api:entry id=c930d7fe-21fd-4929-8b32-0e1e7c1269f0 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/architecture-direction/l25 -->
## Architecture direction

The target architecture is:

<!-- spec-api:entry id=a7b8c0e0-c0ea-4329-a64a-f9b1c44b266f slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/architecture-direction/l29 -->
- `doc-api` owns the data model for documentation validation status, manual checks, generated-guidance checks, and coverage gaps
- a future `doc-cli` is the primary CLI surface for doc inspection and workflow-driven validation operations
- generated-guidance checks such as `rule sync-targets --check` are captured as native documentation-validation records
- ticket/spec workflow metadata references documentation-validation state through shared libraries rather than through wrapper-owned artifacts
- any wrapper-only documentation commands are treated only as migration context until the doc-owned model lands

<!-- spec-api:entry id=51fecef8-b662-439e-a7f8-ed9139542485 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/non-goals/l35 -->
## Non-goals

- complete parsing or linting coverage for every markdown/doc surface in one pass
- replacing existing doc generation flows
- keeping a dedicated wrapper documentation command path as the long-term public interface
- solving every documentation ownership and rendering problem in the same ticket

<!-- spec-api:entry id=269609db-5773-4328-bd62-8a32c80cc756 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/acceptance-criteria/l42 -->
## Acceptance criteria

- This spec no longer treats a separate wrapper documentation command path as the target surface.
- `doc-api` owns documentation-validation metadata and `doc-cli` is defined as the primary CLI surface.
- Generated-guidance checks and manual documentation verification are captured in native workflow metadata.
- Unsupported or partial documentation coverage is explicit in the doc-owned model.
- Any existing wrapper implementation is explicitly described as migration context rather than target architecture.

<!-- spec-api:entry id=753b959c-de9e-429d-b331-0540d5841758 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/current-state/l50 -->
## Current state

- Existing wrapper-oriented documentation commands do not define the target architecture.
- Current command prototypes may inform migration helpers, but the long-term storage and identity model belongs in `doc-api`.

<!-- spec-api:entry id=d3ff6d67-f2d7-4670-94c1-170b717f3ba3 slug=context-engine/workflow-shared/validation-results -->
## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

<!-- spec-api:entry id=90fcbe00-a780-46b6-9fca-bd1556d76d99 slug=context-engine/workflow-documentation-validation-tooling/workflow-documentation-validation-via-doc-api-and-doc-cli/traceability/l59 -->
## Traceability

- [.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5](.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5)
- [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876)
- [memory-viewers/memory-api/crates/doc-api/src/lib.rs](memory-viewers/memory-api/crates/doc-api/src/lib.rs)
