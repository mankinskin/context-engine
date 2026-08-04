`tools/worktree/worktree.sh merge <name>` integrates each branch-bearing nested submodule worktree into the corresponding main-checkout submodule's local `main`, then leaves that main-checkout submodule on `main`. The helper skips detached nested submodule worktrees because no branch exists to merge. Only after every submodule integration succeeds may the helper fast-forward the superproject, ensuring the superproject records the latest local submodule `main` commits.

Related ticket: [4ef88dbc Add a git-worktree helper script for the agent isolation protocol](.ticket/tickets/4ef88dbc-cb39-4724-9f2c-53ab09cf90c5/ticket.toml).

Guard: `worktree-submodule-merge` fixture validation.