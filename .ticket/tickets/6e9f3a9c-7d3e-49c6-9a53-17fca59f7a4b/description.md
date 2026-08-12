# Specification

[Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)

# Files

- `.agents/agents/orchestrator.agent.md` (edit)
- `.agents/agents/simplify.agent.md` (edit)
- `.agents/agents/command.agent.md` (delete)

# Agent Responsibilities

- Orchestrator Agent: Route work across the complete roster and preserve the red thread between tasks, sessions, and goals.
- Simplify Agent: Absorb Instruction Agent rule-steward responsibility by authoring and improving instruction files and agent templates while removing contradictions and repetition.

# Responsibility Boundaries

- Orchestrator Agent routes the full roster but does not replace specialist execution.
- Simplify Agent owns instruction-file and agent-template rule stewardship; no separate Instruction Agent is created.
- Ad-hoc terminal work not covered by a specialist routes to the Implement Agent after `.agents/agents/command.agent.md` is removed.

# Acceptance Criteria

1. A reviewer can read each edited template and find YAML frontmatter with `name`, `description`, `tools` as a list, `argument-hint`, `user-invocable: true`, and a bare vendor-free `model`.
2. A reviewer can find, in order, `## MCP Tool Grant`, `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, and `## Output Format` in each edited template.
3. The orchestrator routes among the full roster and retains the required red thread; Simplify Agent holds the rule-steward responsibility; no separate Instruction Agent exists.
4. `.agents/agents/command.agent.md` is absent and residual ad-hoc terminal work is directed to Implement Agent.
5. Edited templates reference applicable `.agents/instructions/**` rules rather than restating an existing rule inline.