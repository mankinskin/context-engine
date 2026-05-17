# Workflow Documentation Validation via doc-api and doc-cli

## Goal

Make documentation validation first-class behavior in the memory-system doc layer rather than a separate wrapper command path.

Documentation checks, generated-guidance validation, manual verification, and partial-coverage reporting should live in native metadata owned by `doc-api` and surfaced by a future `doc-cli` plus the normal ticket/spec workflow surfaces.

## Problem

Documentation updates are required by the repository workflow, but the current spec still routes the solution through wrapper-oriented command flows. That keeps doc validation outside the normal memory-system ownership model and leaves `doc-api` without the workflow responsibilities it should own.

## Scope

Rewrite the first workflow-facing documentation validation slice around `doc-api` and a future `doc-cli`.

The first implementation slice should:

- define native documentation-validation metadata in `doc-api`
- support validation records for authored docs, generated guidance surfaces, and manual verification steps
- allow unsupported or partial coverage to be reported explicitly in native workflow metadata
- define how documentation validation links to tickets, specs, and future `test-api` / `log-api` entities
- define `doc-cli` as a thin CLI over `doc-api`, not as a second storage model

## Architecture direction

The target architecture is:

- `doc-api` owns the data model for documentation validation status, manual checks, generated-guidance checks, and coverage gaps
- a future `doc-cli` is the primary CLI surface for doc inspection and workflow-driven validation operations
- generated-guidance checks such as `rule sync-targets --check` are captured as native documentation-validation records
- ticket/spec workflow metadata references documentation-validation state through shared libraries rather than through wrapper-owned artifacts
- any wrapper-only documentation commands are treated only as migration context until the doc-owned model lands

## Non-goals

- complete parsing or linting coverage for every markdown/doc surface in one pass
- replacing existing doc generation flows
- keeping a dedicated wrapper documentation command path as the long-term public interface
- solving every documentation ownership and rendering problem in the same ticket

## Acceptance criteria

- This spec no longer treats a separate wrapper documentation command path as the target surface.
- `doc-api` owns documentation-validation metadata and `doc-cli` is defined as the primary CLI surface.
- Generated-guidance checks and manual documentation verification are captured in native workflow metadata.
- Unsupported or partial documentation coverage is explicit in the doc-owned model.
- Any existing wrapper implementation is explicitly described as migration context rather than target architecture.

## Current state

- Existing wrapper-oriented documentation commands do not define the target architecture.
- Current command prototypes may inform migration helpers, but the long-term storage and identity model belongs in `doc-api`.

## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

## Traceability

- [.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5](.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5)
- [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876)
- [memory-viewers/memory-api/crates/doc-api/src/lib.rs](memory-viewers/memory-api/crates/doc-api/src/lib.rs)

