# Specification

[Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)

# Files

- `.agents/agents/structured-research.agent.md` (create)
- `.agents/agents/online-research.agent.md` (create)
- `.agents/agents/writing.agent.md` (create)
- `.agents/agents/framing.agent.md` (create)

# Agent Responsibilities

- Structured Research Agent: Conduct dialectic research by establishing a thesis, expanding evidence, testing an antithesis, and synthesizing a conclusion.
- Online Research Agent: Search the web, evaluate source quality, and summarize supported findings.
- Writing Agent: Produce precise prose that carries a specific argument or knowledge element and turns research into an understandable narrative.
- Framing Agent: Periodically summarize active research, work, goals, and next tasks into a compact context frame for other agents.

# Responsibility Boundaries

- Structured Research Agent performs dialectic thesis/antithesis synthesis; Research Agent performs bounded first-pass research and evidence triage for an implementation slice.
- Online Research Agent evaluates web sources and summarizes supported findings; Research Agent remains the bounded first-pass research owner for an implementation slice.
- Writing Agent composes an argument or explanatory narrative; Transcription Agent preserves and restructures source transcript intent without becoming the narrative author.
- Framing Agent produces a periodic compact context frame; Handoff Agent packages current-session state for the immediate next actor and does not perform retrospective analysis.

# Acceptance Criteria

1. A reviewer can read the four produced files and find YAML frontmatter with `name`, `description`, `tools` as a list, `argument-hint`, `user-invocable: true`, and a bare vendor-free `model`.
2. A reviewer can find, in order, `## MCP Tool Grant`, `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, and `## Output Format` in each file.
3. Each file has only the stated responsibility and honors the boundaries above.
4. Each file references applicable `.agents/instructions/**` rules rather than restating an existing rule inline.