Phase A. Stabilize the shared, non-domain support libraries as standalone dependencies consumable by every per-tool repo: the `viewer-api` framework (+ viewer-ctl, already its own repo) and shared test support (`memory-fixtures` + `memory-matrix`) extracted into the `memory-fixtures` repo.

## Scope
- Extract `memory-fixtures` and `memory-matrix` into the `memory-fixtures` repo with independent build/test.
- Confirm `viewer-api` is self-contained (no reverse dependency on domain crates) and publishable as a shared dependency.
- Define versioning/dependency-declaration convention that per-tool repos will use to consume these shared libs.

## Acceptance criteria
- `memory-fixtures` builds/tests independently and is consumable as an external dependency.
- `viewer-api` has no dependency on any domain tool crate; documented as a shared framework dependency.
- A documented dependency-declaration pattern exists for per-tool repos to consume shared libs.

## Dependencies
- Blocked by provisioning.
- Provides shared substrate consumed by the per-tool extraction tickets.