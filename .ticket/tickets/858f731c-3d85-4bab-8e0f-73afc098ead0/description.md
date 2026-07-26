## Goal

Isolate the implementation phase so it needs **no search and no user-clarification** tooling. Enforce it on the Implement Agent's tool surface and document it as a durable phase-separation rule.

## Work

- Edit `.agents/agents/implement.agent.md`: remove search + `vscode/askQuestions` from its `tools` grant (keep read/edit/execute/test + narrow MCP needed to execute a handoff package). Anything that would require discovery or user Q&A is out of scope for implementation and belongs to the Iteration Agent phase.
- Add a **phase-separation instruction rule** (`.agents/instructions/orchestration/phase-separation.instructions.md`): implementation executes a complete handoff package; discovery and user clarification happen only in research/interview/iteration phases.
- Ensure the Implement Agent contract points to the handoff-package schema as its required input.

## Acceptance criteria

- Implement Agent `tools` no longer include search or askQuestions.
- Implement Agent contract states it consumes a handoff package and must escalate (not clarify inline) if the package is incomplete.
- Phase-separation instruction file exists with a `Use when` description and passes rule scan.
- Ticket linked to the epic; rule cross-linked to the iteration-loop spec (T3).