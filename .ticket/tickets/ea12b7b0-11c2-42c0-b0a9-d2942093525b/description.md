## Goal

Add durable cross-session instruction rules so the loop discipline applies to **all** sessions, not just when the Iteration Agent runs.

## Rules to author (`.agents/instructions/`)

1. **Loop closure** — every finished implementation ends with a durable handoff package **and** a ticket transition (closed or returned); a session must not end an implementation without producing the next handoff.
2. **Escalation gate** — no ticket reaches `done` while an unresolved user escalation exists; open escalations block closure and must be surfaced to the Interview/Iteration phase.

(Phase-separation and handoff-package-schema rules are authored in T4 and T2 respectively; this ticket adds the two remaining cross-cutting rules and cross-links all four.)

## Acceptance criteria

- Two instruction files exist, each with a `Use when ...` description and no `applyTo` (session-wide), passing rule scan.
- They cross-reference the iteration-loop spec (T3), the handoff-package schema (T2), and phase-separation rule (T4).
- AGENTS.md quality-gates / task-routing updated (or linked) so the loop-closure and escalation-gate rules are discoverable.
- Ticket linked to the epic.