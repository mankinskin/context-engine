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

## Implementation (2026-07-25): DONE

- Initialized a spec-api `.spec` store inside the memory-kernel submodule (`spec init`), then authored the canonical spec: id e5294ae5-6bff-44dc-81a9-24a44615b775, slug `transport-harness`, component `transport-harness`, scope public. Body covers Motivation, Responsibilities, Non-goals, Features (default=[] guard), Public API boundaries, Guards, Positions, Validation-ownership. Store: memory-kernel/.spec/specs/e5294ae5-6bff-44dc-81a9-24a44615b775.
- Slimmed context-engine spec 53a23ab2 to a Reference-only body pointing at the canonical memory-kernel spec + implementation + validation home; removed the duplicated normative Positions/Validation blocks.
- Slimmed WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md "Harness And Frontends" section to reference the canonical spec instead of restating harness responsibilities/features.
- Rooted validation evidence in memory-kernel's `.test` store (workspace-slug memory-kernel): spec vt-transport-harness-spec, execution exec-vt-transport-harness-spec-20260725 (passed).

### Validation (all passed)

- `spec refs validate` (memory-kernel root) -> valid, 0 refs.
- `spec health` (memory-kernel root) -> 0 issues.
- context-engine references the canonical spec without duplicating normative requirements (53a23ab2 + contract doc are pointers).

### State move blocker

`update_ticket to_state=in-review` returns `store error: no schema for type 'task'` — the known schema defect. Ticket remains in `new`; state not falsified.