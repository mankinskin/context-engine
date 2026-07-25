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

## Outcome (2026-07-25)

memory-fixtures leaf extraction COMPLETE; memory-matrix + in-tree consumer rewiring split to follow-up 15e632f1.

- memory-fixtures extracted to standalone repo and pushed: github.com/mankinskin/memory-fixtures, branch main @ 96bcaf3.
  - Bundled the workspace fixture data (`test-fixtures/memory-workspace-fixture`, 56 files) into the repo and added a dual-layout `fixture_source_root()` resolver (repo-local first, in-tree fallback) so identical source works standalone and in-tree.
  - Standalone validation: `cargo test --all-features` → 5 passed; `cargo fmt --check` → ok.
  - External consumability proven: a fresh scratch consumer resolved `memory-fixtures` via `{ git = "...", branch = "main" }` and linked its public API (`cargo test` → 1 passed).
- viewer-api domain-independence CONFIRMED: `cargo tree -p viewer-api -e normal` shows no reverse dependency on any workflow-domain crate (ticket/spec/rule/doc/test/log/feedback/session/audit/peek) nor memory-api/memory-matrix; it depends only on `context-api` (graph engine shared types + log parser).
- Dependency-declaration convention DOCUMENTED in the memory-fixtures README: consumers use a branch-pinned git dependency `{ git = "https://github.com/mankinskin/memory-fixtures", branch = "main" }` from `[dev-dependencies]`, mirroring the memory-kernel extraction pattern.

### Deferred to follow-up 15e632f1
- Rewiring the in-tree path deps (spec-api/ticket-api dev-deps, memory-matrix dep) to the external git dependency — touches the memory-api submodule (other-agent activity); left intact and working so the tree stays green.
- memory-matrix extraction — it is a cross-domain consumer depending on the full domain-crate set + their cli/http/mcp tools, so it cannot build as a standalone shared lib until those domain crates are extracted (per-tool phase). Re-scoped out of "shared support libs."

Acceptance criteria for the memory-fixtures substrate are met; memory-matrix was found not to be a shared leaf and was re-scoped.