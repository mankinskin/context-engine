## Problem

We have deep tooling for *implementing* work but almost none for the post-implementation transition: turning a finished, validated implementation into the next self-contained handoff. Today an implementation session must also commit, review, resolve open questions with the user, move tickets, and orient the next steps — mixing discovery/clarification concerns into implementation and leaving the loop open.

## Goal

Close the loop by (1) freeing the implementation phase from any search or user-clarification tooling, and (2) adding an **Iteration Agent** that conducts the transition phase and (re)defines the next handoff.

## Canonical loop

```
implementation (self-contained, no search / no user-clarification)
  → produces a validated change + a complete handoff package
ITERATION AGENT:  Review → Interview → Commit → Handoff
  → advances the repo (commit), the tickets (close/return), and the
    user content/feedback (review + interview), then (re)defines the
    next handoff package
next handoff package → next implementation
```

Canonical order inside the Iteration Agent is **Review → Interview → Commit → Handoff**: only approved work is committed. A failed review returns the ticket to `in-implementation` and that returned work is immediately re-packaged into the next handoff.

## Decisions (from design interview 2026-07-26)

- Iteration Agent is a **thin orchestrator** delegating to existing Commit, Review, Interview, and Handoff agents; it owns sequencing, gating, and next-handoff authoring.
- Name: **Iteration Agent**.
- User Q&A for open questions is delegated to the **Interview Agent**.
- The **handoff package** both extends the existing `session_handoff` record and satisfies a newly documented schema (required fields for a self-contained handoff).
- The **Implement Agent** has its search + user-clarification (askQuestions) tools removed, enforced by a phase-separation rule.
- Two specs: iteration-loop workflow, and handoff-package schema.
- Durable cross-session rules: phase separation, handoff-package schema, loop closure, escalation gate.

## Children

- T1 Iteration Agent template + prompt
- T2 Handoff-package schema spec + `session_handoff` field enforcement
- T3 Iteration-loop workflow spec
- T4 Implement Agent phase isolation (tool-surface restriction + phase-separation rule)
- T5 Durable cross-session rules (loop-closure + escalation-gate instruction files)

## Acceptance criteria

- An Iteration Agent template exists and runs Review→Interview→Commit→Handoff, delegating to the existing agents.
- Implementation sessions no longer need search or user-clarification tools; the restriction is enforced and documented.
- A finished implementation always terminates in a durable handoff package that the next implementation can execute with zero discovery.
- No ticket reaches `done` while an unresolved user escalation exists.
- Both specs and the four durable rules are authored and linked to their tickets.