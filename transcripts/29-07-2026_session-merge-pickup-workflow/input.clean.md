# Session Merge and Pickup Workflow

We need a way to merge separately started sessions onto a shared track or ticket, and later split them apart again. This is important for handoff, orchestration, and keeping repository work coordinated.

## Current direction

- The long-term vision is a root session that coordinates most or all repository work for a day, running all day, starting subagents that handle found tasks, starting subagents that review, and starting individual tracks and letting them run fully to completion. Within those subagents the system would interact with the user through interviews or messaging services to gather new tasks or reviews, driving overall progress in the repository.
- That model is not realistic or practical yet. It risks wasting tokens inefficiently, doing the wrong work, executing tasks incorrectly, and failing to verify acceptance criteria properly.
- For now, sessions are still started manually for specific tracks, tickets, or ad hoc tasks.
- The existing structured handoff already separates planning from implementation and isolates implementation from question-and-answer cycles.
- The current orchestration model can run multiple workflow steps and delegate implementation or review to subagents, but it is usually limited to an isolated goal and often does not carry a track through to full completion. In practice, it usually completes only certain increments for a given track.
- The Iteration Agent already helps review and close a track's or ticket's implementation so the next increment can start.
- We already have a rough system for coordinating multiple sessions that work on one track, and a rough system for running multiple sessions in parallel or sequentially. What is still missing is merging sessions back together and redistributing work across them.

## Missing capability

We still need a true merge and pickup system that can:

- merge sessions that began independently but converged on the same work,
- redistribute work from several overlapping sessions,
- represent many-to-many relationships between sessions and handoffs: a session must be able to initiate multiple other sessions through multiple handoffs (for example, an orchestrating agent that starts subagents), and a session must also be able to merge multiple other sessions into one through a pickup or merge step,
- keep a provenance chain showing which session handed work off to which other session and which session picked up whose work,
- associate multiple sessions with a track, and
- represent tracks that overlap, depend on one another, or contain smaller tracks.

## Example

If three sessions in the Session API project separately improve session transcript cost gating, handoff references, and track management, but their work starts to depend on each other, they should no longer be treated as three unrelated sessions. They should be merged into a shared session or shared track that can absorb the prior work and context, complete the overlapping part, and later hand the work back out again when the tracks diverge.

## Required workflow

The system must support a full cycle where:

1. We start from several separate requests.
2. One or more sessions work on them independently.
3. We merge those sessions into a shared track when they converge.
4. The shared track unlocks the dependent work.
5. Once the shared work is complete, the session can hand the tracks back out into separate specialized sessions.
6. Each merge or pickup preserves references to the originating sessions and their handoffs.

## Track structure

- A track is a larger connected system of related tickets rooted in an epic or tracker ticket.
- It must be possible to start and link a chain, tree, or full graph of sessions for a track, distributing all of its task steps among themselves while keeping the connection through references.
- Sessions should work on the smallest possible track they belong to.
- Smaller tracks must also know which larger tracks use or depend on them.

## Worked example: orchestrator merge and split

- An orchestrator starts multiple tracks and itself works a larger track that depends on those smaller tracks.
- The smaller tracks are worked by subagents, each with its own session, running in parallel or on different branches.
- While working, those smaller agents can spawn further sessions on the same track, or start a smaller shared track that both of them depend on and use, started through the same session.
- The orchestrator merges the handoffs of the two tracks' next steps into a single handoff (or merge) for that shared track, so a further session can work on it.
- Once the shared track's session ends, the orchestrator generates two handoffs again from it — one for each original track — so those tracks can continue with their newly unlocked tasks.
- When both original tracks finish, the orchestrator merges them once more into a single shared handoff or pickup for itself, so it can conclude: both of its tracks are done, they both completed the shared track, they completed their own tasks, and it can now close its own track and hand off to the next session.

## Task

Because these workflows are complex, the system must materialize all of these links as a full graph structure across the repository, using references between entities and detailed session logging. The task is to fully work out this idea to its last detail and materialize a complete plan that:

- closes the gaps that currently prevent us from understanding how to validate that the system works correctly,
- documents the exact work steps for the agents and makes them accessible, and
- breaks the whole plan into clearly defined, goal-oriented work steps so the improvement can be integrated into the system as quickly as possible.

## Planning process

We should not invent work that already exists in the plan. The process should be:

1. First interview the user to clarify the exact shape of the system.
2. Turn that into a rough overarching plan with goals and acceptance criteria.
3. Design the necessary components structurally.
4. Search the existing planning and implementation work to find matching components.
5. Reuse and extend existing plans where possible.
6. Only define new items when they are not already covered.
7. Keep asking architectural questions when needed.
8. Write the goals, acceptance criteria, and chosen design into specs before implementation.
9. Use a ticket first if needed to drive the spec work.
10. Start implementation only after the spec and sequencing are finalized.
11. Orchestrate the work through several subagents and sessions until the full track is implemented and validated.

## Open Questions

- The transcript uses handoff, pickup, and merge somewhat interchangeably for the same operation; the exact canonical terminology still needs to be fixed.