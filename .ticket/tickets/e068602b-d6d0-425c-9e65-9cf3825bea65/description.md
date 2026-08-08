# Verified observation (2026-08-08)

Running `git -C <submodule> worktree prune` across the five submodules removed `.git/modules/context-stack/worktrees/context-stack2` while the owning worktree was alive and healthy on disk.

The affected worktree's `.worktrees/45bd0e3f-worktree-config-hijack/context-stack/.git` still contained `gitdir: C:/Users/linus/git/context-engine/.git/modules/context-stack/worktrees/context-stack2`, pointing at the deleted registration. Every Git command in the worktree that recursed into submodules then failed with `fatal: not a git repository: .../.git/modules/context-stack/worktrees/context-stack2`. The worktree could not stage, commit, diff, or report status. Plain filesystem reads still worked, which is how the uncommitted work was rescued.

The trigger appears to be pruning while the worktree directory was mid-rename: the directory had been renamed on disk but the registration had not yet been re-resolved, so prune judged the live registration stale.

Impact: silent, total loss of Git usability for a worktree that looks healthy on disk. Uncommitted work is stranded and recoverable only by copying files out.

## Acceptance criteria

1. Pruning never removes a submodule linked-worktree registration whose owning worktree directory still exists on disk under any name.
2. A regression test renames a worktree directory, runs root-level and per-submodule `worktree prune`, and asserts all five submodules in the renamed worktree remain fully usable: `git -C <worktree> status` succeeds with no `fatal:` output.
3. `doctor` detects a worktree whose `<submodule>/.git` gitdir points at a non-existent registration, reports the broken state with the exact repair command, and exits non-zero.
4. A documented, non-destructive repair path restores the registration without recreating the worktree.

## Related tickets

Cross-reference `723c2bea`, `72314c5e`, `5e6cf4f8`, and `2b65715`.