## Objective

Integrate the **session-coupled feedback signal** into the delegation loop: the ability to flag specific problem spots and draw attention to particular scenarios, which is mostly coupled to a session.

## Scope (from transcript)

- Feedback lets an agent/user mark specific problem spots and highlight scenarios worth attention.
- This capability is mostly coupled to a session, so it must attach to session/entity context.
- The transcript flags that this capability may be incomplete.

## First task: implementation-status audit

- Audit how far the session-coupled feedback capability is implemented today (feedback-api, feedback-mcp, session-api coupling).
- Report the gap between current state and the "flag problem spots per session" requirement before building further.

## Acceptance criteria

- Written audit of current feedback + session coupling state (what exists, what is missing).
- Defined mechanism for attaching feedback to a session/entity and surfacing it into the delegation evaluation.
- Feedback entries target canonical entity URNs (per AGENTS.md feedback workflow).

## Anchor

Feeds the delegation quality/cost metric ticket (sibling under this epic).