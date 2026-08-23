<!-- aligned-structure:v2 -->

# Worktree Control CLI Lifecycle

## Target Code Location

[workflow-tools/session/crates/worktree-ctl/src/main.rs](../../../workflow-tools/session/crates/worktree-ctl/src/main.rs) defines `Cli`, `Command`, `dispatch`, and lifecycle handlers; [workflow-tools/session/crates/worktree-ctl/tests/worktree_contracts.rs](../../../workflow-tools/session/crates/worktree-ctl/tests/worktree_contracts.rs) covers lifecycle contracts.

## Naming Conventions

Use `Cli`, `Command`, and `dispatch`; criterion ids use the `worktree-cli-` prefix, including `worktree-cli-delegates-provisioning-policy`.

## Reading Order

1. [fa6e85c8 Worktree Control Component Pilot](../../fa6e85c8-866c-4d53-bb50-b78bd651e8ce/body.md) - composing parent and parent-owned criteria.
2. [c1d13a73 Worktree Provisioning Policy](../../c1d13a73-3265-42e1-8da0-5c44ef7b61ff/body.md) - shared provisioning-library provider.
3. [workflow-tools/session/crates/worktree-ctl/src/main.rs](../../../workflow-tools/session/crates/worktree-ctl/src/main.rs) - implemented dispatch boundary.

## Responsibility

If implemented, operators can invoke the supported lifecycle commands and rely on the CLI dispatch layer to route provisioning decisions through the shared policy library.

## Interfaces And Dependencies

`Cli` parses a `Command`; `dispatch(Command)` selects lifecycle handlers. Lifecycle handlers consume `WorktreeGit`, `WorktreeRef`, `ProvisionPolicy`, `evaluate_reclaim_candidate`, and `provision_for_session` from `session-worktree-provision`.

## Behavior

- `worktree-cli-dispatches-lifecycle`: each supported command reaches its intended handler.
- `worktree-cli-delegates-provisioning-policy`: handlers use the provisioning library for reclaim/provision decisions rather than reproducing policy logic.
- `worktree-cli-contract-coverage`: integration tests exercise user-visible lifecycle behavior.

## Boundaries And Failure Cases

The CLI does not own provisioning policy or invent an MCP/API parity surface. Parse failure, invalid checkout, rejected policy decision, or library error is surfaced as command failure without duplicating library policy.

## Provider/Consumer Contract

Consumes `worktree-provision-reclaim-decision` and `worktree-provision-session-provisioning` from [c1d13a73 Worktree Provisioning Policy](../../c1d13a73-3265-42e1-8da0-5c44ef7b61ff/body.md); provides lifecycle behavior to operators.

## Examples

A lifecycle command reaches `dispatch`, opens `WorktreeGit`, and calls `provision_for_session` with `ProvisionPolicy::default()` rather than selecting a reclaim candidate in CLI-local code.

## Evidence

Position: `implemented` for the named CLI/library calls. Run `cargo test --manifest-path workflow-tools/session/Cargo.toml -p worktree-ctl --test worktree_contracts`.

## Scope

Owns CLI dispatch/lifecycle behavior, not synchronization, gitlink checks, or policy semantics.
