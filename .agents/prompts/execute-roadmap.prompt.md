---
description: "Execute a compiled ROADMAP.md from a prompt-ingestion dossier methodically: read the roadmap and dossier together, walk waypoints in dependency order, collect per-waypoint context before acting, and delegate oversized waypoints as one isolated unit."
name: "execute-roadmap"
argument-hint: "<path to a dossier's ROADMAP.md, or the dossier folder>"
agent: "agent"
---

# Execute Roadmap

Walk a compiled `ROADMAP.md` to completion, one waypoint at a time, without losing the dossier context each waypoint depends on.

Follow [roadmap-execution.instructions.md](../instructions/orchestration/roadmap-execution.instructions.md) as the authoritative procedure. This prompt only resolves the input and sequences the walk.

## Workflow

1. **Resolve the input.** If the argument is a folder, locate `ROADMAP.md` inside it; if it is a direct path to `ROADMAP.md`, use it. Reject (and ask instead of guessing) if more than one `ROADMAP.md`-shaped file exists and the choice is material — a versioned snapshot (`ROADMAP.v1.md`, ...) is never the target.
2. **Read the roadmap and the dossier together.** Read `ROADMAP.md` in full, then the dossier's `README.md` and `ARTIFACTS.md`, before touching any waypoint.
3. **Walk the waypoints in order.** For each waypoint, in dependency order: collect its cited context from the dossier, execute it directly if single-session, or delegate it as one isolated unit if it is ticket-backed or cross-session, then run its validation gate before advancing.
4. **Update the roadmap as you go.** Mark each waypoint's status as it starts and completes so `ROADMAP.md` stays a live progress record.
5. **Escalate drift instead of improvising.** If a waypoint's cited context is stale, record it against "Active blockers" and stop for that waypoint rather than guessing a substitute.

## Constraints

- Do not skip ahead to a waypoint whose declared dependency is not yet satisfied.
- Do not execute a ticket-backed or cross-session waypoint as a series of small ad hoc edits — dispatch it as one delegated unit scoped to that waypoint and its ticket.
- Do not mark a waypoint done without running its declared validation gate.
- Do not silently substitute a stale artifact reference — escalate per [escalation-gate.instructions.md](../instructions/orchestration/escalation-gate.instructions.md).

## Response

- resolved `ROADMAP.md` path and its outcome summary
- waypoints completed this run, each with its validation result
- waypoints remaining, in order, with any that are ticket-backed called out
- any drift or blocker recorded, and what it blocks
