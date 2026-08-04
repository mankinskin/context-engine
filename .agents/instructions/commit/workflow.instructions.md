---
description: "Use when performing the repository commit workflow. Covers checking status, regenerating generated outputs, staging logical batches, committing, and verifying clean state."
---

**Branch precondition.** Before step 1, confirm you are committing inside the task's own worktree on its own feature branch: `git branch --show-current` must print `agent/<ticket-short-id>-<slug>`, not `main`. Stage only files the task's board entry claims; `git add -A` from an implementation session is forbidden. `bash tools/worktree/worktree.sh list`, run from the main checkout, shows the registered worktrees and their branches. See [branch-worktree.instructions.md](./branch-worktree.instructions.md).

1. Check status before staging:

git status --short
git submodule foreach --recursive 'git status --short && echo "=== $name ==="'

2. Regenerate generated outputs when applicable (see [generated-files.instructions.md](./generated-files.instructions.md)).

3. Stage in logical batches and commit each batch with an appropriate conventional commit message.

4. Update submodule pointers last (deepest-first cadence).

5. Verify clean state:

git status --short

6. Rebase the feature branch onto the updated `main`, resolve any conflicts here rather than on `main`, re-run validation, then mark the branch ready to merge with a `board check-out` whose reason starts `ready-to-merge:`. The merge into `main` belongs to the root orchestrator session. See [branch-worktree.instructions.md](./branch-worktree.instructions.md).
