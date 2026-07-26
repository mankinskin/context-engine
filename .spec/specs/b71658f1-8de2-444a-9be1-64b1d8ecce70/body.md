## Purpose

Define how a finished, validated implementation becomes the next self-contained handoff, and which agent owns each phase. This closes the implement → transition → implement loop.

## Phase model

1. **Implementation phase** — self-contained. The Implement Agent executes a complete handoff package with no search and no user clarification. If the package is incomplete, it escalates rather than clarifying inline.
2. **Transition phase** — conducted by the **Iteration Agent** (a thin orchestrator).
3. **Next implementation phase** — driven by the handoff package the transition phase produced.

## Canonical order (transition phase)

**Review → Interview → Commit → Handoff.** Only approved work is committed.

1. **Review** (delegated to Review Agent) — verify acceptance criteria against the validated implementation. Findings become follow-up tickets; unmet criteria return the ticket to `in-implementation`.
2. **Interview** (delegated to Interview Agent) — resolve remaining open questions / escalations with the user.
3. **Commit** (delegated to Commit Agent) — commit only approved work (hooks, rule sync, generated files, submodule pointers, conventional messages).
4. **Handoff** (delegated to Handoff Agent) — (re)define the next self-contained handoff package.

## Gates

- **Review gate** — acceptance criteria verified before commit; failures return the ticket to `in-implementation`.
- **Escalation gate** — no ticket reaches `done` while an unresolved user escalation exists.
- **Loop-closure gate** — every finished implementation terminates in a durable handoff package plus a ticket transition (closed or returned).

## Re-packaging rule

A returned (failed-review) ticket is immediately re-packaged into the next handoff so the loop stays closed. The **Iteration Agent authors this re-package inline** (it does not delegate re-packaging to the Handoff Agent). The Handoff Agent remains responsible only for authoring the forward next-handoff in step 4 of a passing run.

## Roles

- Review, Interview, Commit, Handoff agents own their phases.
- The Iteration Agent adds only sequencing, gating, and next-handoff authoring — it does not implement, research, or clarify directly.

## Related

- Handoff-package schema spec (required fields for a self-contained handoff).
- Durable rules: phase separation, handoff-package schema, loop closure, escalation gate.
