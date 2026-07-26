## Purpose

Define the **handoff package**: the required fields that make the next implementation session fully self-contained — zero discovery, zero user clarification. A handoff package is produced by the transition phase (Handoff Agent) and consumed by the next implementation phase (Implement Agent). It extends the existing `session_handoff` record schema and is supplied alongside required handoff-record parameters.

## Required fields

- **objective** — the single goal of the next implementation unit.
- **target_tickets** — ticket ids with current state and acceptance criteria inlined (not referenced-by-lookup).
- **target_files** — explicit workspace-relative paths expected to be touched.
- **decisions** — resolved design choices, so implementation does not re-decide.
- **non_goals** — explicit out-of-scope boundaries.
- **context_anchors** — prior findings, links, and ids needed so no search is required.
- **open_escalations** — must be empty for a package to be implementation-ready.

## Required handoff-record parameters

- **validation** — exact commands / checks that prove the unit done. This is a required parameter supplied alongside `SessionHandoffPackage` when creating the handoff record, not a field on the package struct.

## Optional fields

- **risk_notes** — known risks or fragile areas.
- **predecessor_handoff** — id of the handoff this one supersedes, for lineage.

## Enforcement

- `session_handoff` produces / validates a record that carries every required package field plus required handoff-record parameters. A package missing a required package field (or with a non-empty `open_escalations`) is rejected or flagged as not implementation-ready.
- The Implement Agent treats the package as its sole input. If a required field is missing at consume time, it escalates rather than searching or asking the user.

## Readiness rule

A handoff package is **implementation-ready** only when all required package fields are present and `open_escalations` is empty.

## Storage / durable home

The `session_handoff` record is the **source of truth** for a produced package and its required companion parameters. It is additionally **mirrored onto the target ticket** as a field/artifact so the next implementation session can load the package cold from the ticket store without a live session. The ticket mirror and the `session_handoff` record must stay consistent; the mirror carries at least `objective`, `target_tickets`, `target_files`, `validation`, and `open_escalations`.

## Related

- Iteration loop workflow spec (phases, ordering, gates).
- Phase-separation rule (implementation must not search or clarify).
- Loop-closure and escalation-gate rules.
