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
2. [66fbd896 Worktree Git Operations](../66fbd896-19d4-4bb7-898c-7cdc76375a5e/body.md) - inspection-and-metadata provider.
3. [a623ea02 Worktree Gitlink Integrity](../a623ea02-e1a9-4c8c-81ea-f1f5fb3b4a9f/body.md) - containment-and-status provider.
4. [585aa074 Session Worktree Lifecycle Rewrite](../585aa074-356a-4169-b08b-4e3aba659a72/body.md) - related hybrid-access evidence.

## Responsibility

If implemented, synchronization integrates worktree changes using Git Operations for inspection and metadata while preserving required gitlink containment and status checks.

## Interfaces And Dependencies

`handle_sync` opens and consumes `WorktreeGit` for inspection and worktree
metadata; its integration path consumes gitlink verification/status partitioning
before committing rebased gitlinks. Local helpers perform rebase, merge, and
stash writes.

## Behavior

- `worktree-sync-uses-git`: use `WorktreeGit` for inspection and worktree metadata.
- `worktree-sync-preserves-gitlinks`: invoke gitlink integrity behavior before integration completes.
- `worktree-sync-integration-coverage`: maintenance tests cover successful and rejected integration paths.

## Boundaries And Failure Cases

Synchronization does not define reclaim policy or CLI parsing, and it does not route every repository mutation through `WorktreeGit`. Git/open failures and invalid gitlink states prevent mutation; a failed integrity decision cannot be bypassed by sync.

## Provider/Consumer Contract

Consumes `worktree-git-inspection-metadata` from [66fbd896 Worktree Git Operations](../66fbd896-19d4-4bb7-898c-7cdc76375a5e/body.md) and `worktree-gitlink-containment` from [a623ea02 Worktree Gitlink Integrity](../a623ea02-e1a9-4c8c-81ea-f1f5fb3b4a9f/body.md) for containment/status. It does not consume `ProvisionPolicy` or `provision_for_session`.

## Examples

`handle_sync` performs its integration through `WorktreeGit`; an unresolved gitlink state reported by the integrity component stops the integration path before a commit.

## Evidence

Position: `implemented`. Run `cargo test --manifest-path workflow-tools/session/Cargo.toml -p worktree-ctl --test maintenance`.

## Scope

Owns synchronization/integration behavior, not lifecycle dispatch or low-level policy decisions.
