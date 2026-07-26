## Goal

Author a new **Iteration Agent** template (`.agents/agents/iteration.agent.md`) and matching prompt (`.agents/prompts/iteration.prompt.md`). It is a **thin orchestrator** for the post-implementation transition phase.

## Canonical sequence

**Review → Interview → Commit → Handoff** (only approved work is committed).

1. **Review** — delegate to the Review Agent to verify acceptance criteria against the finished, validated implementation. Findings become follow-up tickets; unmet criteria return the ticket to `in-implementation`.
2. **Interview** — delegate to the Interview Agent to resolve any remaining open questions / escalations with the user.
3. **Commit** — delegate to the Commit Agent to commit only the approved work (pre-commit hooks, rule sync, generated files, submodule pointers, conventional messages).
4. **Handoff** — delegate to the Handoff Agent to (re)define the next self-contained handoff package for the next implementation session.

## Behavior

- Orchestrator only: it sequences and gates, and authors the next-handoff decision; it does not implement code, run research, or clarify with the user directly (that is delegated to Interview).
- A failed review returns the ticket and **immediately re-packages that returned work into the next handoff** (see epic).
- Advances three axes: the repo (commit), the tickets (close/return), and user content/feedback (review + interview).
- Binds a durable session and persists progress as a workflow graph; ends with `session_handoff`.

## Acceptance criteria

- `.agents/agents/iteration.agent.md` exists with tool grants limited to orchestration + delegation (agent, session-mcp, ticket-mcp, spec-mcp, read; no direct edit/search-heavy surface beyond what gating needs).
- `.agents/prompts/iteration.prompt.md` exists and documents the Review→Interview→Commit→Handoff sequence and gates.
- The template references the existing Review, Interview, Commit, and Handoff agents by their `.agents/agents/*.agent.md` names.
- The agent is listed in the workspace agent catalog after a rule scan.