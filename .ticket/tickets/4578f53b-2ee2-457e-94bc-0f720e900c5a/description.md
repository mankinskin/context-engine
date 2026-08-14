## Gap

`.agents/instructions/commit/branch-worktree.instructions.md` already contains `### Entity stores are worktree-local` at line 127, so the state-store rule is partial rather than absent. No agent template enforces the instruction. Add a constraint to `.agents/agents/implement.agent.md` Constraints at line 40 requiring writes to `.ticket/`, `.spec/`, and `.session/` to target the assigned worktree through the workspace selector. Add a detection-and-remediation step to `.agents/agents/commit.agent.md` Required Workflow after checkout confirmation at lines 59-61 for state-store records created in the root checkout.

## Session Evidence

Session and ticket records accidentally landed in the root checkout on `main`. A commit agent had to copy records into the worktree and use `git restore` on the root paths.

## Required Corrected State

Implement and Commit Agent contracts prevent root-checkout entity-store writes, detect accidental root records, copy or recreate the correct worktree records according to store semantics, and restore only the accidental root paths.