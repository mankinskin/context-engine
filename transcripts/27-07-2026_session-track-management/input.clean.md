# Track/Thread Management Across Sessions

## Problem

We now have a workflow that works on very large tasks across multiple sessions, but we need much stronger segregation and composition of those sessions.

The intended cycle looks like this:

1. A planning session starts: it creates tickets, talks with the user, and collects the initial task.
2. It produces a handoff for the next agent.
3. The next session implements the first steps based on that handoff and produces an implementation summary.
4. The summary goes to another agent / a new session that performs the implementation review and clarifies open questions with the user. That completes one iteration.
5. That session again produces a handoff, which goes to the next implementation session, and so on.

Along the way, various new tickets can appear, possibly a few side tracks.

The critical requirement: across this entire chain of sessions and handoffs, the main goal must stay in the foreground. Every session collaborating on a given ticket via handoffs must know exactly what the goal is, what the scope is, what the frame is, and when they are done.

Re-communicating all of that through every single handoff is inefficient. It also risks losing details or letting the goal silently drift.

## Proposed Architecture

Introduce a global thread or track:

- Scoped at the highest level, with a clear goal and a clear definition of done.
- Multiple sessions can collaborate inside it or alternate within it.
- We already have the principle of session workspaces; we could extend or repurpose that, if there are no conflicts, to bring multiple sessions under one umbrella.

## Sub-Agent Sessions

Today it is not clear whether a sub-agent of a session is a new persistent session or exists inside its orchestrator's session. We want every sub-agent session to be an isolated session.

That implies a more compact session format: smaller sessions should still get a complete durable (persisted) session, but it must not be as heavy — no generating all kinds of boilerplate and files that are never used.

## Traceability

Implementing this requires special attention to session traceability:

- Every inline response and request must link which session triggered the action or produced the result.
- Inline answers from sessions must be traceable back to the durable session by pure text copying alone.
- Sessions must know which workspace or track they are currently active in, i.e. which thread they are working in.
- We may eventually want to support parallel sessions within such threads.

## Target Behavior

The goal is that we can hand over an arbitrarily large task — for example "write a complete social media platform". The first agent sets up the environment so that all subsequent sessions are subordinated to that goal, move within it, and work toward it until it is implemented and verified, at which point the goal or track can be considered complete.

It makes sense to always provide a fixed ticket per track, so progress management can be handled there.

## Scope Constraint

We do not want to reimplement existing capabilities. This is really only a very thin layer on top of our existing systems: the session system, the ticket system, the test system, the spec system — all the systems we already have. Its purpose is to carry very large tasks through from start to finish in a focused and validated way.

## Immediate Next Step

1. Take an inventory of the existing state: from already implemented tools through to plans that have already been designed.
2. Integrate this layer — call it track management or thread management, name to be decided — on top of that existing state, and plan it.

The goal for now is a plan: how we want to implement this, what it should look like, and answering all open questions in detail so that we can implement this system from start to finish.
