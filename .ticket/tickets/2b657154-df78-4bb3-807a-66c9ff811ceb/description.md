`tools/worktree/worktree.sh list` correctly labels unregistered directories beneath `.worktrees/` as `UNREGISTERED-DEBRIS`, but `remove <name>` identifies such a directory with `git -C <path> rev-parse`, which resolves the enclosing main repository. The removal path then invokes `git worktree remove` on an unregistered path and can resolve the branch as `main`.

Required behavior:
- `remove <name>` must distinguish Git-registered worktrees from unregistered debris using the same membership source as `list`.
- For unregistered debris, clean any nested submodule worktree registrations, delete only the validated debris directory, and prune registrations without invoking superproject worktree removal or deleting a branch.
- Preserve existing teardown behavior for registered worktrees.

Validation: create a temporary unregistered `.worktrees/<name>` directory, run `remove <name>`, verify the directory disappears and `git worktree list --porcelain` remains unchanged; also exercise a registered-worktree dry run.


Status: Implemented on `agent/2b657154-unregistered-debris-removal` (uncommitted). The stale `install-ctl-foundation` board lease was released. `f3c2b8a9-install-ctl` is now empty but remains until all shells/processes leave that path.


Merged to local `main` as commit `00e2ddd`; the merged feature worktree and branch were removed. Ticket state could not move from `ready` because the configured workflow exposes no successor state.