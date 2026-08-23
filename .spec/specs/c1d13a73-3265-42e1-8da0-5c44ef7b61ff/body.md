<!-- aligned-structure:v2 -->

# Worktree Provisioning Policy

## Target Code Location

[workflow-tools/session/crates/session-worktree-provision/src/lib.rs](../../../workflow-tools/session/crates/session-worktree-provision/src/lib.rs) defines `WorktreeGit` and `WorktreeRef`; [workflow-tools/session/crates/session-worktree-provision/src/policy.rs](../../../workflow-tools/session/crates/session-worktree-provision/src/policy.rs) defines `ProvisionPolicy`, `SessionActivity`, `evaluate_reclaim_candidate`, and `provision_for_session`.

## Naming Conventions

The future persisted `component_id` is `worktree-provisioning-policy`. Use
`ProvisionPolicy` and `SessionActivity`; criterion ids use the
`worktree-provision-` prefix. `WorktreeGit` is an implemented library type, but
the accurate Git-operation provider component boundary remains unresolved.

## Reading Order

1. [fa6e85c8 Worktree Control Component Pilot](../fa6e85c8-866c-4d53-bb50-b78bd651e8ce/body.md) - composing parent.
2. [191ceae7 Worktree Control CLI Lifecycle](../191ceae7-663e-448b-bb04-46f46f38825d/body.md) - consuming CLI.
3. [workflow-tools/session/crates/session-worktree-provision/src/policy.rs](../../../workflow-tools/session/crates/session-worktree-provision/src/policy.rs) - policy provider.

## Responsibility

If implemented, consumers can make consistent worktree reclaim and provision decisions through one reusable library boundary.

## Interfaces And Dependencies

`WorktreeGit` supplies repository operations, `SessionActivity` supplies activity state, and `ProvisionPolicy` supplies policy input. `evaluate_reclaim_candidate` evaluates eligibility; `provision_for_session` produces the provision outcome.

## Behavior

- `worktree-provision-reclaim-decision`: evaluate a candidate from repository, activity, and policy inputs.
- `worktree-provision-session-provisioning`: provision or reuse a worktree through the shared library.

## Boundaries And Failure Cases

The library does not parse CLI arguments, dispatch commands, or integrate
gitlinks. This component must not claim that sync or gitlink consume
`ProvisionPolicy` or `provision_for_session`; the accurate Git-operation
provider component is deliberately not introduced by this pilot. Invalid
repository state or activity/policy errors return typed library failures.

## Provider/Consumer Contract

Provides `worktree-provision-reclaim-decision` to [191ceae7 Worktree Control CLI Lifecycle](../191ceae7-663e-448b-bb04-46f46f38825d/body.md). `worktree-provision-session-provisioning` has no confirmed consumer in this pilot; sync and gitlink have no provisioning-policy or provisioning-function edge.

## Examples

`provision_for_session(&git, activity, &policy, session)` decides reuse/reclaim/create from the same `ProvisionPolicy` regardless of which CLI lifecycle path consumes it.

## Evidence

Position: `implemented`. Run `cargo test --manifest-path workflow-tools/session/Cargo.toml -p worktree-ctl --test worktree_contracts` and the provisioning crate's focused unit tests when present.

## Scope

Owns reusable provisioning policy/library behavior, not command presentation or integration orchestration.
