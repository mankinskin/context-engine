<!-- aligned-structure:v2 -->

# Worktree Synchronization And Integration

## Target Code Location

[workflow-tools/session/crates/worktree-ctl/src/sync.rs](../../../workflow-tools/session/crates/worktree-ctl/src/sync.rs) defines `handle_sync`; [workflow-tools/session/crates/worktree-ctl/tests/maintenance.rs](../../../workflow-tools/session/crates/worktree-ctl/tests/maintenance.rs) exercises integration and merge behavior.

## Naming Conventions

The future persisted `component_id` is `worktree-synchronization`. Use
`handle_sync` and `worktree-sync-` criterion ids, including
`worktree-sync-preserves-gitlinks`.

## Reading Order

1. [fa6e85c8 Worktree Control Component Pilot](../fa6e85c8-866c-4d53-bb50-b78bd651e8ce/body.md) - composing parent.
2. [a623ea02 Worktree Gitlink Integrity](../a623ea02-e1a9-4c8c-81ea-f1f5fb3b4a9f/body.md) - gitlink provider.

## Responsibility

If implemented, synchronization integrates worktree changes using the shared Git boundary while preserving required gitlink integrity checks.

## Interfaces And Dependencies

`handle_sync` opens and consumes `WorktreeGit`; its integration path consumes
gitlink verification/status partitioning before committing rebased gitlinks.
The pilot has not yet established the component that owns the Git-operation
provider contract.

## Behavior

- `worktree-sync-uses-git`: route repository operations through `WorktreeGit`.
- `worktree-sync-preserves-gitlinks`: invoke gitlink integrity behavior before integration completes.
- `worktree-sync-integration-coverage`: maintenance tests cover successful and rejected integration paths.

## Boundaries And Failure Cases

Synchronization does not define reclaim policy or CLI parsing. Git/open failures and invalid gitlink states prevent mutation; a failed integrity decision cannot be bypassed by sync.

## Provider/Consumer Contract

Consumes `worktree-gitlink-containment` from [a623ea02 Worktree Gitlink Integrity](../a623ea02-e1a9-4c8c-81ea-f1f5fb3b4a9f/body.md). It uses `WorktreeGit`, but has no asserted provider edge for it until boundary research is complete; it does not consume `ProvisionPolicy` or `provision_for_session`.

## Examples

`handle_sync` performs its integration through `WorktreeGit`; an unresolved gitlink state reported by the integrity component stops the integration path before a commit.

## Evidence

Position: `implemented`. Run `cargo test --manifest-path workflow-tools/session/Cargo.toml -p worktree-ctl --test maintenance`.

## Scope

Owns synchronization/integration behavior, not lifecycle dispatch or low-level policy decisions.
