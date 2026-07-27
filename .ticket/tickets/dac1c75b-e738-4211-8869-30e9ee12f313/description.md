## Goal

Enforce the D2 policy decision: **the Iteration Agent owns ALL ticket state transitions.** The Review Agent is now strictly verdict-only and must never call `close_ticket`, never pass `to_state` to `update_ticket`, and never move a spec to `reviewed`. This applies ALWAYS, not just when invoked by the Iteration Agent.

## Context

`.agents/agents/review.agent.md` has already been updated to match this policy. However, any other prompt, instruction, or documentation that still tells the Review Agent to close tickets or transition specs must be found and reconciled.

## Scope

Search and update:
- Agent instruction files (`.agents/instructions/`)
- Other agent templates that might reference Review Agent behavior
- Workflow prompts
- Any documentation describing the Review Agent's responsibilities
- AGENTS.md or other top-level guidance

Ensure consistent messaging: Review Agent reports verdicts and findings only; the delegating agent (typically Iteration Agent) applies the resulting transitions.

## Acceptance criteria

- All references to Review Agent closing tickets or transitioning specs have been removed or corrected
- Documentation consistently describes Review Agent as verdict-only
- No conflicting guidance remains that would cause the Review Agent to attempt state transitions