# Specification

[Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)

# Files

- `.agents/agents/session-bootstrap.agent.md` (create)
- `.agents/agents/merge.agent.md` (create)
- `.agents/agents/cleanup.agent.md` (create)

# Agent Responsibilities

- Session Bootstrap Agent: Initialize a session end to end: resolve the session UUID, provision and rename the worktree, check in the session and board, and pin task-relevant instructions.
- Merge Agent: Integrate a completed feature branch bottom-up, enforce the gitlink invariant, fast-forward only, then tear down the merged worktree and branch.
- Cleanup Agent: Maintain workspace hygiene through safe temporary-file, duplication, stale-worktree, and stale-branch cleanup with audit and health checks.

# Responsibility Boundaries

- Session Bootstrap Agent initializes an execution session; Installer Agent manages the tool and skill lifecycle; neither role owns feature implementation.
- Merge Agent alone integrates a completed feature branch and enforces bottom-up fast-forward and gitlink rules; Cleanup Agent only removes stale or temporary workspace material safely.
- Cleanup Agent performs hygiene and audit work; Merge Agent alone integrates a completed feature branch.

# Acceptance Criteria

1. A reviewer can read the three produced files and find YAML frontmatter with `name`, `description`, `tools` as a list, `argument-hint`, `user-invocable: true`, and a bare vendor-free `model`.
2. A reviewer can find, in order, `## MCP Tool Grant`, `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, and `## Output Format` in each file.
3. Each file has only the stated responsibility and honors the boundaries above.
4. Each file references applicable `.agents/instructions/**` rules rather than restating an existing rule inline.