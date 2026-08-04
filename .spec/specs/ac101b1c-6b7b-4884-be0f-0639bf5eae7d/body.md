# Unregistered Worktree Debris Removal

Ticket: 2b657154-df78-4bb3-807a-66c9ff811ceb

## Requirements

`tools/worktree/worktree.sh remove <name>` must use Git's registered-worktree list as the membership authority. A directory under `.worktrees/` that is not registered is debris, even when `git -C <directory> rev-parse` succeeds by walking to the enclosing repository.

For debris, `remove` must remove any submodule worktree registrations at the matching nested paths, delete only the named validated debris directory, and prune registrations. Debris cleanup must not execute superproject `git worktree remove` and must not derive or delete a branch.

Registered worktrees retain the current submodule teardown, superproject removal, prune, and merged-branch deletion behavior.

## Validation

Create a temporary unregistered directory beneath `.worktrees/`, run the removal command, and verify the directory no longer exists while `git worktree list --porcelain` remains unchanged. Run the registered-worktree removal dry run to confirm the existing teardown plan remains available.