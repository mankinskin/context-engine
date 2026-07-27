## Problem
Large tasks span many sessions and handoffs; the top-level goal, scope, and definition-of-done drift or must be re-communicated in every handoff. Sub-agent session semantics are undefined.

## Scope
A thin layer over the existing session, ticket, spec, and test systems.

## Explicit Non-Goals
- No new store
- No track entity
- No parallel sessions (deferred)
- No reimplementation of the iteration loop

## Definition of Done
All 6 child tickets closed and validated.

## Dependencies
Hard prerequisites (must land first): 3eaceaae, 0a45bedb

## Sequencing Decision
effba966 (Dynamic Session Bootstrap) was downgraded from hard prerequisite to related on user decision: its 26 open descendants are not required for track schema work. Hard prerequisites remain 3eaceaae (Surface workspace_session_id, in-review) and 0a45bedb (Flatten session store).

## Related Work
- Tickets: 1fbf2d84 (Close Iteration Loop), 0647a212 (Persist Handoff Records), d3af78d7 (Handoff Package Schema), 5755b694 (Iteration Loop Workflow), 76e831f2 (Iteration Agent Template), effba966 (Dynamic Session Bootstrap)
- Spec: 823b22cf (Session API Persistence)
- Design docs: tmp/spec-iteration-loop.md, tmp/spec-handoff-package-schema.md, tmp/epic-default-agent-tools.md, DESIGN_AGENT_HARNESS.md
- Source proposal: transcripts/27-07-2026_session-track-management/input.clean.md