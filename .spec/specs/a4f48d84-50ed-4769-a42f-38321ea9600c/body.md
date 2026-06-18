<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=98bbe03a-236c-443b-b9e6-f01a7be717b9 slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/goal/l3 -->
## Goal

Define workflow validation as default behavior of the memory-system tool stack rather than as a separate wrapper tool path.

<!-- spec-api:entry id=0bcd3223-ae11-4569-95a5-4ce61cc0464e slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/goal/l7 -->
Validation specifications, executions, outcomes, blockers, and traceability should live in first-class metadata owned by shared libraries. Existing and future tool surfaces such as `ticket`, `spec`, `doc`, `test`, and `log` should expose that behavior directly.

<!-- spec-api:entry id=17e7433c-b673-4c89-9371-4e4798656ec9 slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/problem/l9 -->
## Problem

The repository workflow requires structured validation evidence, but the current prototype introduced a separate repository-local artifact store and wrapper commands. That proves some mechanics, but it makes workflow capture an optional extra step instead of default memory behavior and splits ownership away from the ticket/spec/doc layers.

<!-- spec-api:entry id=02369fcf-aeb1-4270-b58b-36c76cac5c5c slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/scope/l13 -->
## Scope

Rewrite the target architecture for the first embedded workflow slice.

<!-- spec-api:entry id=d95e9ebb-cc5d-4730-8c15-0c59695ee441 slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/scope/l17 -->
The first implementation slice should:

<!-- spec-api:entry id=649ed4c1-d201-4e2d-b512-315d6002038f slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/scope/l19 -->
- define native workflow validation metadata owned by the shared memory-system APIs
- define how validation specifications, executions, outcomes, and blockers are represented without a separate workflow artifact store
- make workflow capture part of normal ticket/spec/doc behavior and future `test-api` and `log-api` behavior
- define the minimal default configuration knobs that turn workflow capture on for the existing tool surfaces
- describe how any wrapper-only prototype logic is absorbed into shared libraries or discarded

<!-- spec-api:entry id=17488fe4-6ba7-480b-9dd9-014c75ea9d81 slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/architecture-direction/l25 -->
## Architecture direction

The target architecture is:

<!-- spec-api:entry id=b29e9e66-0810-4b8b-97b5-8bce9404916a slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/architecture-direction/l29 -->
- `ticket-api` and `spec-api` store workflow-facing metadata and cross-store references directly in their native models
- `doc-api` owns documentation-validation metadata and a future `doc-cli` becomes the thin CLI surface for those operations
- a future `test-api` owns validation specifications, executions, and outcomes such as `passed`, `failed`, and `blocked`
- a future `log-api` owns captured validation logs and retrieval for review/debugging evidence
- CLI, MCP, and HTTP surfaces are thin interfaces over shared-library behavior; workflow capture is not a second command path users must remember
- any wrapper-only prototype code is migration context, not the product goal

<!-- spec-api:entry id=fb596a33-446e-47c6-8efc-5f6b639a57ac slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/non-goals/l36 -->
## Non-goals

- preserving a dedicated wrapper validation CLI as the long-term public interface
- keeping validation state in a separate path-only JSON artifact store
- implementing every `test-api`, `log-api`, `doc-cli`, HTTP, and MCP surface in the same change
- migrating every existing prototype artifact before the shared metadata model exists

<!-- spec-api:entry id=e4e4d43d-d4fb-4fb9-b436-767c45d7f0d1 slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/acceptance-criteria/l43 -->
## Acceptance criteria

- This spec no longer depends on a dedicated wrapper validation CLI or wrapper-owned artifact store.
- Validation workflow state is defined as default shared-library behavior across `ticket-api`, `spec-api`, `doc-api`, and future `test-api` / `log-api`.
- First-class responsibilities for `test-api` and `log-api` are part of the design, including native identifiers and cross-store links.
- Any wrapper-only prototype implementation is explicitly described as migration context rather than target architecture.
- Existing CLI, MCP, and HTTP surfaces are treated as the default way users interact with workflow metadata once the shared-library behavior lands.

<!-- spec-api:entry id=4327bf9a-1206-4852-8194-0846845736c1 slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/current-state/l51 -->
## Current state

- Existing wrapper-oriented prototype work is not the product goal.
- Data-model ideas may still be retained, but repository-local artifact files are not the intended source of truth.
- Prototype-only validation results do not satisfy this rewritten architecture by themselves.

<!-- spec-api:entry id=d3ff6d67-f2d7-4670-94c1-170b717f3ba3 slug=context-engine/workflow-shared/validation-results -->
## Validation results

- `./target/debug/spec.exe scan --force --index-root .spec --json`

<!-- spec-api:entry id=83897081-f97d-4bde-bd51-e72e19848f2b slug=context-engine/workflow-validation-tool/workflow-validation-metadata-and-default-tool-behavior/traceability/l61 -->
## Traceability

- [.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c](.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c)
- [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876)
- [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a)
- [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)
- [memory-api/tools/cli/ticket-cli/README.md](memory-api/tools/cli/ticket-cli/README.md)
- [memory-api/tools/cli/spec-cli/README.md](memory-api/tools/cli/spec-cli/README.md)
