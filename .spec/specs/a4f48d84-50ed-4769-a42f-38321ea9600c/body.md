# Workflow Validation Metadata and Default Tool Behavior

## Goal

Define workflow validation as default behavior of the memory-system tool stack rather than as a separate wrapper tool path.

Validation specifications, executions, outcomes, blockers, and traceability should live in first-class metadata owned by shared libraries. Existing and future tool surfaces such as `ticket`, `spec`, `doc`, `test`, and `log` should expose that behavior directly.

## Problem

The repository workflow requires structured validation evidence, but the current prototype introduced a separate repository-local artifact store and wrapper commands. That proves some mechanics, but it makes workflow capture an optional extra step instead of default memory behavior and splits ownership away from the ticket/spec/doc layers.

## Scope

Rewrite the target architecture for the first embedded workflow slice.

The first implementation slice should:

- define native workflow validation metadata owned by the shared memory-system APIs
- define how validation specifications, executions, outcomes, and blockers are represented without a separate workflow artifact store
- make workflow capture part of normal ticket/spec/doc behavior and future `test-api` and `log-api` behavior
- define the minimal default configuration knobs that turn workflow capture on for the existing tool surfaces
- describe how any wrapper-only prototype logic is absorbed into shared libraries or discarded

## Architecture direction

The target architecture is:

- `ticket-api` and `spec-api` store workflow-facing metadata and cross-store references directly in their native models
- `doc-api` owns documentation-validation metadata and a future `doc-cli` becomes the thin CLI surface for those operations
- a future `test-api` owns validation specifications, executions, and outcomes such as `passed`, `failed`, and `blocked`
- a future `log-api` owns captured validation logs and retrieval for review/debugging evidence
- CLI, MCP, and HTTP surfaces are thin interfaces over shared-library behavior; workflow capture is not a second command path users must remember
- any wrapper-only prototype code is migration context, not the product goal

## Non-goals

- preserving a dedicated wrapper validation CLI as the long-term public interface
- keeping validation state in a separate path-only JSON artifact store
- implementing every `test-api`, `log-api`, `doc-cli`, HTTP, and MCP surface in the same change
- migrating every existing prototype artifact before the shared metadata model exists

## Acceptance criteria

- This spec no longer depends on a dedicated wrapper validation CLI or wrapper-owned artifact store.
- Validation workflow state is defined as default shared-library behavior across `ticket-api`, `spec-api`, `doc-api`, and future `test-api` / `log-api`.
- First-class responsibilities for `test-api` and `log-api` are part of the design, including native identifiers and cross-store links.
- Any wrapper-only prototype implementation is explicitly described as migration context rather than target architecture.
- Existing CLI, MCP, and HTTP surfaces are treated as the default way users interact with workflow metadata once the shared-library behavior lands.

## Current state

- Existing wrapper-oriented prototype work is not the product goal.
- Data-model ideas may still be retained, but repository-local artifact files are not the intended source of truth.
- Prototype-only validation results do not satisfy this rewritten architecture by themselves.

## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

## Traceability

- [.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c](.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c)
- [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876)
- [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a)
- [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)
- [memory-viewers/memory-api/tools/cli/ticket-cli/README.md](memory-viewers/memory-api/tools/cli/ticket-cli/README.md)
- [memory-viewers/memory-api/tools/cli/spec-cli/README.md](memory-viewers/memory-api/tools/cli/spec-cli/README.md)

