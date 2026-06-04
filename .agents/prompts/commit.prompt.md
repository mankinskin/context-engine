---
description: "Commit staged or unstaged changes across the repo and all submodules, handling pre-commit hooks, rule sync, and generated files correctly."
name: "commit"
argument-hint: "[message]"
agent: "agent"
---

# Commit Changes

Commit all pending changes across the root repo and submodules following the repository's commit conventions.

Reference [commit.instructions.md](./.agents/instructions/commit.instructions.md) and [AGENTS.md](./AGENTS.md).

## Workflow

1. Run `git status --short` and `git submodule foreach --recursive 'git status --short'` to survey all changes.
2. Identify dirty submodules (lowercase `m` in status output) and plan commit order: deepest-first, then parent pointer updates.
3. Check whether any rule-managed files have drifted. If `.rule/`, `rule-targets.yaml`, or any rule-generated output was changed, run `cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml` before staging.
4. Stage and commit in logical batches (see [commit.instructions.md](.agents/instructions/commit.instructions.md) for batch order).
5. For each batch, write a conventional-commit message: `<type>(<scope>): <imperative summary>`.
6. After all root-repo commits are done, update submodule pointers deepest-first.
7. Verify clean state with `git status --short`.

## Key rules

- Never edit rule-managed files (AGENTS.md, copilot-instructions.md, instruction/prompt/agent files) directly. Always regenerate via `rule sync-targets`.
- Commit submodules in deepest-first order before updating parent pointers.
- The pre-commit hook blocks commits that stage rule-related files with drifted generated outputs. Fix by running `rule sync-targets` and re-staging.
- Use `git commit --no-verify` only for confirmed false-positive hook failures; document why in the commit message.

## Response

Return:
- files committed per batch, with commit message
- any pre-commit hook failures and how they were resolved
- submodule pointer updates made
- final `git status --short` output confirming clean state
