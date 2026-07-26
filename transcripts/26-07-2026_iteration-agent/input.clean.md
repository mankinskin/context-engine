# Closing the Loop: An Iteration Agent for Post-Implementation Handoff

## Problem

When we work on our now very complex ticket workflows — dependency-connected work steps or goals — we still lack a real transition solution for moving out of an implementation and into a session that:

- clarifies the last open questions with the user, and
- in that same session, where possible, already collects the next steps for a new session — i.e. produces a **handoff** from a finished session.

The overall aim is to keep tightening this loop. We already have a lot of tooling for *implementing*, but we do not yet have much tooling for bringing together all the steps needed *after* an implementation so we can get back to a new implementation — that is, for closing the circle.

## Steps That Must Happen After an Implementation

1. **Commit the work** sensibly and correctly.
2. **Review** for any obvious errors, and confirm there are no open questions — i.e. no escalations still requiring the user's attention.
3. **Close or return tickets first**: before orientation, tickets should already have been closed where possible, or sent back into implementation.
4. **Orient**: capture what has now been done and what should be done next.
5. **Produce the next steps / a complete handoff** for a new session, based on the tickets and the user's answers from the interview, so that the next steps can then be executed.

## Design Goal: Free the Implementation Phase

Ideally the implementation phase itself no longer needs to make commits or change, move, or otherwise manage tickets. We want to free the implementation entirely from any tools that have to *search* for something or *clarify* something with the user.

Instead, we want a self-contained package — the handoff — that an agent can implement and validate end to end without problems, and then hand off to review. The review step is exactly what we are doing right now.

## The Agent We Need

We need an agent (an agent template) that can fully execute this review-and-transition step.

- A good name might be **Iteration Agent**, because it denotes one iteration in the workflow.
- Alternatively it could be called an **Increment Agent**, because it performs all the necessary transitions:
  - it advances the repository,
  - it advances the tickets, and
  - it advances the content and the feedback with the user.

It brings everything together and (re)defines the current handoff — the current goal or goals being worked on — for the current sessions.

## Requests

Please help me plan:

- **Which agents** we want to add.
- **Which rules** we want to add that could be useful across all kinds of sessions.
- **In which order** the steps should be performed.
- **How the handing over of handoffs** should be designed.
