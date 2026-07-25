Phase A. Extract the shared storage/index/search kernel (currently memory-api/crates/memory-api) into the standalone `memory-kernel` repository, freeing the `memory-api` name and providing the common substrate every domain tool depends on.

## Scope
- Move the shared kernel crate into `memory-kernel` with its own Cargo manifest, README, and CI.
- Preserve git history where practical (subtree/filter-repo).
- Reconcile with the neutral-naming work so the extracted kernel uses neutral API names (see linked `13912e44`, `2b1279bd`).
- Publish as a path/git dependency consumable by all per-tool repos and by context-engine transitively via workflow-tools.

## Acceptance criteria
- `memory-kernel` builds and tests independently.
- Domain crates compile against `memory-kernel` as an external dependency (via workspace/git dep) with no reference to the old umbrella path.
- Neutral naming map applied to the extracted surface (or explicit deprecation aliases retained).
- Kernel README documents the public contract and versioning.

## Dependencies
- Blocked by provisioning (repos must exist).
- Coordinates with default-store architecture tickets `2b1279bd` and `13912e44` (linked).