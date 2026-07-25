Review follow-up from ticket dbe0e955-c1b4-414d-820c-10c3fbbb5d3d.

## Finding

The transport-harness contract is currently documented only in context-engine. The reviewer considers that unusable because the production crate is owned by memory-kernel. The canonical contract must be a spec in the memory-kernel repository, with context-engine referencing rather than owning it.

## Acceptance criteria

- memory-kernel has a canonical spec covering transport-harness responsibilities, non-goals, features, public API boundaries, and validation guards.
- The spec links implementation and validation evidence in memory-kernel.
- context-engine's workflow-tools contract references the memory-kernel spec and does not duplicate canonical requirements.
- The spec remains discoverable through the context-engine submodule layout.

## Review verdict (2026-07-25): PASS with refinements

Approved for implementation. Recorded design decisions from review:

- Canonical surface: create a new spec-api-managed `.spec` store inside memory-kernel to own the canonical transport-harness spec (not a plain markdown doc).
- Dependency: this ticket formally depends on 9451f439 (submodule registration). Criterion 4 (discoverability through the submodule layout) is only satisfiable once the submodule exists, so 9451f439 must land first.
- Validation-evidence home: memory-kernel owns the transport-harness validation evidence; context-engine links to it rather than embedding inline results (removing the duplicated Positions/Validation blocks currently in spec 53a23ab2).
- Requirement split: normative harness responsibilities/non-goals/features/API/guards live in the memory-kernel canonical spec; context-engine's WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md and spec 53a23ab2 are slimmed to references only.
- First validation after implementation: validate the new canonical spec's references from the memory-kernel root, then confirm context-engine references it without duplicating normative requirements.