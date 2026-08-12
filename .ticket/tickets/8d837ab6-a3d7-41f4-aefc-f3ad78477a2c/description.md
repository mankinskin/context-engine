# Specification

[Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)

# Files

- `.agents/agents/refactoring.agent.md` (create)
- `.agents/agents/code-architect.agent.md` (create)
- `.agents/agents/surface-design.agent.md` (create)
- `.agents/agents/live-validation.agent.md` (create)

# Agent Responsibilities

- Refactoring Agent: Find and apply behavior-preserving improvements to code, documentation, and existing features, reducing duplication and improving comprehension or performance; repair encountered defects when warranted.
- Code Architect Agent: Review and improve project architecture, including language-specific design considerations.
- Surface Design Agent: Evaluate and improve UI/UX for novice and power users.
- Live Validation Agent: Exercise shipped tools, CLIs, and servers directly and report observed behavior.

# Responsibility Boundaries

- Refactoring Agent improves existing behavior while preserving contracts; Code Architect Agent evaluates architectural direction before or beyond a local refactor; Surface Design Agent owns user-facing usability rather than general architecture.
- Code Architect Agent owns architectural direction, while Refactoring Agent preserves behavior within existing contracts and Surface Design Agent owns user-facing usability.
- Surface Design Agent owns UI/UX for novice and power users, not general architecture.
- Live Validation Agent observes shipped CLIs, servers, and tools; Testing Agent owns automated test design, test execution, and validation evidence.

# Acceptance Criteria

1. A reviewer can read the four produced files and find YAML frontmatter with `name`, `description`, `tools` as a list, `argument-hint`, `user-invocable: true`, and a bare vendor-free `model`.
2. A reviewer can find, in order, `## MCP Tool Grant`, `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, and `## Output Format` in each file.
3. Each file has only the stated responsibility and honors the boundaries above.
4. Each file references applicable `.agents/instructions/**` rules rather than restating an existing rule inline.