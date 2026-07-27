## Problem

The persisted session handoff record is thin. `session_handoff` accepts only `workspace`, `workspace_session_id`, and a `validation[]` array of gates, producing a record with `handoff_id`, `outgoing_run_id`, `resume_command`, `workflow`, and `validation`. It has no field for the substantive handoff package.

As a result the eight-field package (objective, target_tickets, target_files, decisions, validation, non_goals, context_anchors, open_escalations) is written into the ticket's `forward_handoff_package` field instead. Ownership is inverted: the ticket holds the package and the session record holds only a gate ledger.

This is the same class of schema gap that already caused a live defect: `SessionValidationGate` has no `command` field, so validation commands pasted into a handoff were silently lost. That was worked around by creating test-api `ValidationSpec` entries (`val-session-api-lib-suite`, `val-session-api-build`) and referencing them by id.

## Decision

The session should own the full handoff. The ticket should only reference it.

## Scope

- Extend the session handoff record schema to carry the full package: objective, target_tickets, target_files, decisions, non_goals, context_anchors, open_escalations (validation already exists as gates).
- Extend `session_handoff` inputs accordingly.
- Reconcile the existing inversion: ticket `forward_handoff_package` should become a reference to the owning handoff record rather than the storage location for the package body.
- Provide a migration or back-compat read path for existing tickets that already store a full package inline (at minimum epic 1fbf2d84 and ticket 9d527ad1).

## Acceptance Criteria

1. A handoff package can be persisted in full via `session_handoff` with no field dropped.
2. Round-tripping a persisted handoff returns every field unchanged.
3. Ticket `forward_handoff_package` resolves to the owning handoff record rather than duplicating its body.
4. Existing inline packages remain readable after the change.
5. No parallel storage path: the package has exactly one authoritative home.

## Context

- Existing record: .session/runtime/workspaces/0101b7ef-e717-4c94-bebd-c8d55f6aaa82/handoffs/9cd7050b-63b3-430a-8732-8f27952aaaf4.json
- Prior record showing the same shape: handoff dcf86212
- Inverted-ownership examples: tickets 9d527ad1 and 1fbf2d84 (`forward_handoff_package`)
- Related precedent: the `ValidationSpec.command` workaround in the test-api store

Raised during the iteration that closed ticket 41ff230b.