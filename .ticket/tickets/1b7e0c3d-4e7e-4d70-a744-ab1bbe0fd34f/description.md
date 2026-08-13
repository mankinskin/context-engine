# Problem

Workflow-tools repository extraction is blocked by a repository-level dependency cycle. `ticket-api` depends non-optionally on the legacy base crate at `memory-api/crates/memory-api`, while seven crates in the `memory-api` repository (`session-api`, `audit-api`, `session-capture-hook`, `session-worktree-provision`, `memory-matrix`, `spec-cli`, and `rule-cli`) depend non-optionally on `ticket-api`. The crate graph is acyclic, but extracting either side leaves the two repositories mutually dependent.

`memory-kernel` already provides the extracted shared base library as `memory_kernel` at `memory-kernel/src/`. Ticket `e12d8343` published the extraction at commit `4c0c7a3`. The legacy crate at `memory-api/crates/memory-api` is a duplicate that must be removed after all consumers cut over to the kernel repository below both sides of the cycle.

## Scope

Migrate all 23 workspace manifests currently path-depending on `memory-api/crates/memory-api` across `memory-api/crates/*`, `memory-api/tools/{cli,mcp,http}/*`, and `memory-viewers/ticket-viewer` to `memory_kernel`. Delete the legacy crate only after all consumers resolve the kernel through one documented dependency-resolution policy.

The migration is a port, not a package rename. The legacy source advanced 13 commits after extraction: `5bd4c2aa`, `ffb6fbe6`, `197c1817`, `39533fa7`, `29101949`, `c6eb0a3a`, `9f247fd0`, `ddc70e9d`, `cc2a22c5`, `4b5046bb`, `084c8914`, `058c50a6`, and `c51114c8`. The kernel received no source changes in the same period; its two later commits touched only `transport-harness`. The trees differ: kernel `src` has 48 files / 14,264 lines, and the legacy source has 47 files / 15,447 lines.

## Compatibility Gaps

The legacy API has these public items absent from the kernel:

- `model::entity::{SpecRef, TicketPart, TicketRefEntry}` and `TicketPart::new`.
- `EntityManifest::{related_specs, set_related_specs, refs, set_refs, legacy_spec_link_entries, parts, set_parts}`.
- `storage::board::ActiveWorktree` and `BoardSnapshot::active_worktrees: Vec<ActiveWorktree>`.

The kernel alone provides `interoperability::InteroperableArtifact`, which replaced the legacy crate's non-optional `test-api` path dependency. `test-api` still defines a duplicate `InteroperableArtifact` contract at `memory-api/crates/test-api/src/interoperability.rs`; reconcile the two definitions into one authoritative contract.

Consumer sampling shows `session-api` and `spec-api` use only paths already provided by the kernel. `ticket-api` is the known incompatible consumer because it imports `model::entity::{SpecRef, TicketPart, TicketRefEntry}`.

## Open Architectural Decisions

Do not assume every legacy-only item belongs in the neutral kernel. Triage each legacy-only API item: forward-port genuinely neutral infrastructure into `memory-kernel`, or move ticket/spec-specific concepts into `ticket-api` or `spec-api`. The required explicit decisions are ownership of `ActiveWorktree` / `BoardSnapshot::active_worktrees` and the `EntityManifest` sidecar methods.

Settle one documented dependency-resolution policy for cross-repository dependencies. The current policy is inconsistent: `memory-api/crates/ticket/Cargo.toml` uses a relative submodule path for `transport-harness`, while `workflow-tools-contract-reference/crates/example/Cargo.toml` uses a git URL.

## Acceptance Criteria

1. No workspace crate or manifest depends on `memory-api/crates/memory-api` by path or package alias.
2. `memory-api/crates/memory-api` is deleted after all former consumers compile against `memory_kernel`.
3. All 23 former consumers resolve `memory_kernel` under one documented dependency-resolution policy, including the selected treatment of `transport-harness`.
4. Each of the 13 post-extraction legacy commits is inventory-triaged, with every relevant change either forward-ported into `memory-kernel` or relocated into its owning domain crate with rationale.
5. The legacy-only API gaps are resolved through recorded ownership decisions; `ActiveWorktree` / `BoardSnapshot::active_worktrees` and the `EntityManifest` sidecar methods have explicit outcomes.
6. `test-api` and `memory_kernel` use one authoritative `InteroperableArtifact` definition with no duplicate contract.
7. `ticket-api`, including its `SpecRef`, `TicketPart`, and `TicketRefEntry` usage, compiles against the resolved ownership surface.
8. `cargo build --workspace` passes after the cutover.

## Dependencies and Context

This is a prerequisite for parent tracker `858c5286` and ticket-tool extraction `ba4aaa9c`, and contributes to migration epic `69eb4118`. It is linked to the neutral-kernel architecture tickets `2b1279bd` and `37e07148`, plus completed extraction ticket `e12d8343`.

## Acceptance Criteria Clarification

Criterion 3's documented dependency-resolution policy is git URL-based: all cross-repository dependencies resolve by git URL, and relative submodule paths remain development-only overrides.