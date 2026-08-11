# Session Creation Should Happen in the Target Worktree

It is a mistake to create the session in the main branch. The process should create the session only in the target worktree, leaving `main` practically untouched until the session merges back into `main`.

In other words, new sessions should not be created in `main` at all. They must be created in the target worktree.