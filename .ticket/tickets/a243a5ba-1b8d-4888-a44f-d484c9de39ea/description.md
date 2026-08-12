# Specification

[Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)

# Files

- `.agents/agents/installer.agent.md` (create)
- `.agents/agents/bug-report.agent.md` (create)
- `.agents/agents/session-learning.agent.md` (create)
- `.agents/agents/scoping.agent.md` (create)

# Agent Responsibilities

- Installer Agent: Install and update tools and external skills, record installed versions, support reinstall, and run post-install checks.
- Bug Report Agent: Document a defect with reproducible evidence, create the bug ticket, and link the defect to its owning specification and component.
- Session Learning Agent: Analyze prior sessions and artifacts, extract learning and improvement opportunities, record feedback, and decide whether each finding becomes a bug or feature ticket.
- Scoping Agent: Estimate work, split work into isolated task blocks and phases, build the ticket hierarchy and dependency graph, and support later decomposition.

# Responsibility Boundaries

- Installer Agent manages tool and skill lifecycle; Session Bootstrap Agent initializes an execution session; neither role owns feature implementation.
- Bug Report Agent documents reproducible defect evidence and creates the defect ticket; Ticket Refinement Agent improves a known ticket scope, criteria, fields, and links.
- Session Learning Agent analyzes historical artifacts and records durable feedback; Handoff Agent packages current-session state for the immediate next actor and does not perform retrospective analysis.
- Scoping Agent creates estimates, phases, a ticket hierarchy, and dependency graph; Ticket Refinement Agent improves a known ticket without becoming the graph planner.

# Acceptance Criteria

1. A reviewer can read the four produced files and find YAML frontmatter with `name`, `description`, `tools` as a list, `argument-hint`, `user-invocable: true`, and a bare vendor-free `model`.
2. A reviewer can find, in order, `## MCP Tool Grant`, `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, and `## Output Format` in each file.
3. Each file has only the stated responsibility and honors the boundaries above.
4. Each file references applicable `.agents/instructions/**` rules rather than restating an existing rule inline.