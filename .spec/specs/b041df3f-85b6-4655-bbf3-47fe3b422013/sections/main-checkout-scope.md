# Main Checkout Scope

A small, self-contained change to one existing file or one new file may use the main checkout without worktree provisioning, session check-in, or board check-in. The change must first confirm that no active board entry owns the path, stage only the changed path, and run focused validation before commit.

Changes spanning multiple files or components, submodules, active concurrent work, or risky behavior require a worktree branch and session/board claims.