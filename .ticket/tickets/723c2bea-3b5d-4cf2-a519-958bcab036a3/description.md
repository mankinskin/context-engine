## Objective

Ensure failed session-worktree provisioning cannot corrupt shared submodule configuration or prevent later worktree creation.

## Scope

Make rollback restore pre-attempt submodule config and linked-worktree registrations. Extend `doctor` to identify the corruption class and add a failed-bootstrap regression test proving the main checkout remains usable.

## Done

The acceptance criteria in the `requirements` part pass, including successful main-checkout `git status` and a subsequent worktree bootstrap after simulated failure.