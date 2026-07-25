Phase E. Update the agent entry points and guidance across all three install sites so an agent starting at any level is pointed at the workflow-skill guidance: (1) context-engine (uses the tools), (2) workflow-tools root (works on the tools/guidance), (3) each individual tool repo (works on that tool).

## Scope
- Update AGENTS.md, .agents/instructions, and the agents folder at each site to route into the workflow-skill.
- Ensure guidance is offered primarily via the skill, with AGENTS.md as the thin entry point.
- Keep path-scoped instruction precedence coherent across nested repos.

## Acceptance criteria
- Each of the three site types has a working entry point that resolves to the workflow-skill guidance.
- No duplicated/conflicting guidance across nested sites (aligns with the skill scope plan).
- An agent bootstrapped at any site discovers tools and next tasks.

## Dependencies
- Blocked by skill scope/precedence and context-engine reframing.