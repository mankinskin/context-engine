---
description: "Use when methodically executing a compiled ROADMAP.md from a prompt-ingestion dossier. Covers reading the roadmap and dossier together, walking waypoints in dependency order, collecting per-waypoint context from the dossier before acting, and delegating oversized waypoints as one isolated unit."
applyTo: "**/*.md"
---

## Purpose

A compiled `ROADMAP.md` (see [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md)) is a route, not a summary to skim once and improvise from. This instruction is the execution-side counterpart: it governs how an executing session — most often the Orchestrator Agent — walks a roadmap's waypoints methodically instead of reconstructing context from memory partway through.

## Required Procedure

1. **Read the roadmap and the dossier together.** Before acting on anything, read `ROADMAP.md` in full — starting with its outcome summary — plus the dossier's `README.md` index and `ARTIFACTS.md`, so every cited id/path resolves to a real artifact instead of an assumed one.
2. **Execute waypoints in order.** Walk the Roadmap Waypoints section top to bottom, respecting its dependency order. Do not start a waypoint whose declared dependency (an earlier waypoint, a ticket, a decision) is not yet satisfied. Mark each waypoint's status as it starts and completes so the roadmap stays a live progress record, not a static plan.
3. **Collect per-waypoint context from the dossier before acting.** For each waypoint, resolve its cited artifact ids/paths — tickets, specs, the numbered work-package docs (`01-...md`, ...), code/config paths — via the dossier before writing anything. A waypoint's one-line roadmap summary is a pointer, not the context itself; the cited work-package doc is normative and must be read.
4. **Delegate large waypoints as one isolated unit.** A waypoint marked cross-session, or one backed by a ticket created during roadmap compilation (per [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md)'s "Ticket Creation During Refinement"), is executed as a single delegated dispatch scoped to that waypoint and its ticket — not split into ad hoc smaller asks, and not folded into neighboring waypoints.
5. **Validate before advancing.** Run the waypoint's declared validation gate and confirm it passes before marking the waypoint done and moving to the next one. Do not defer validation to the end of the route.

## Delegating a Waypoint

Follow [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) for model-tier selection when dispatching a waypoint. A single-session waypoint routes like any other bounded implementation unit; a ticket-backed waypoint routes to Ticket Refinement, Scoping, or Implement per that ticket's own state, not per the roadmap alone.

## Handling Drift

If a waypoint's dossier context has gone stale — a cited artifact no longer resolves, a validation command no longer exists — do not silently improvise a substitute. Record the drift against `ROADMAP.md`'s "Active blockers" section (adding one if none exists) and escalate per [escalation-gate.instructions.md](escalation-gate.instructions.md) instead of guessing.
