Review follow-up from ticket dbe0e955-c1b4-414d-820c-10c3fbbb5d3d.

## Finding

The production memory-kernel repository exists at the sibling checkout and is pushed to mankinskin/memory-kernel, but it is not discoverable from the context-engine workspace. The reviewer requires memory-kernel to be registered as a context-engine Git submodule.

## Acceptance criteria

- context-engine records memory-kernel as a submodule at an intentional repository path.
- Clone/bootstrap documentation initializes the submodule.
- The transport-harness reference path and validation continue to work from the submodule layout.
- Submodule pointer records memory-kernel commit 45de2fd or a reviewed successor.

## Review verdict (2026-07-25): PASS with refinements

Approved for implementation. Recorded design decisions from review:

- Submodule path: `memory-kernel/` at the context-engine repo root, sibling to the existing memory-viewers/context-stack/memory-api/viewer-api submodules.
- Reference dependency style: the reference must consume transport-harness via a Git dependency pinned to the submodule branch (NOT the current relative path dependency `../../../../memory-kernel/crates/transport-harness`). Criterion 3 is refined accordingly: validation must pass with the reference resolving the harness through a branch-pinned git dependency.
- Consumer requirement: submodule is development-only (reference + tests); ordinary consumers are not required to initialize it. Bootstrap docs must document this audience split.
- First validation after implementation: `git submodule status --recursive`, then reference `cargo metadata --no-deps` proving the harness resolves through the approved git dependency.