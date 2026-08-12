# Specification

[Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)

# Files

- `AGENTS.md` (edit)
- `.agents/instructions/orchestration/shared-context-bundle.instructions.md` (edit)
- `.agents/instructions/orchestration/orchestrator-delegation.instructions.md` (edit)
- `.agents/instructions/orchestration/model-prices.instructions.md` (edit)
- `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` (edit)

# Agent Responsibilities

No agent template is created by this ticket. The ticket updates only roster references so the remaining templates retain their specified responsibilities.

# Responsibility Boundaries

- Cross-reference maintenance updates names and roster references only; Batch 5 owns the orchestrator rewrite, Simplify Agent extension, and Command Agent removal.
- The ticket does not alter any agent responsibility, frontmatter, or template body contract.

# Acceptance Criteria

1. A reviewer can read the listed files and confirm no reference points to `.agents/agents/command.agent.md` or Command Agent.
2. A reviewer can confirm every roster reference points to an existing template and preserves the roles introduced by the specification.
3. A reviewer can confirm no agent template frontmatter or body section is changed: existing templates retain `name`, `description`, `tools`, `argument-hint`, `user-invocable: true`, and `model`, and retain ordered `## MCP Tool Grant`, `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, and `## Output Format` sections.
4. The files reference applicable `.agents/instructions/**` rules rather than adding an inline restatement of an existing rule.