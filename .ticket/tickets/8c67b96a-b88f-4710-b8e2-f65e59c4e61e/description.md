## Re-scoped 2026-07-27 — original premise was stale

This ticket originally claimed that `session_handoff` accepts only `workspace`, `workspace_session_id`, and `validation[]`. **That is no longer true and must not be implemented against.**

Verified empirically on 2026-07-27 by persisting handoff `74d8b170-aef5-4a12-a8ec-e665e34bb585` into workspace session `0101b7ef-e717-4c94-bebd-c8d55f6aaa82`: `session_handoff` accepts `objective`, `target_tickets`, `target_files`, `decisions`, `non_goals`, `context_anchors`, `open_escalations`, and `risk_notes`, and the persisted record round-trips all of them **except one**. Ownership is therefore already correct: the session record holds the package.

## Problem — the three gaps that actually remain

1. **`open_escalations` is silently dropped.** It is accepted as an input parameter but does not appear in the persisted record schema. The returned record requires `objective`, `target_tickets`, `target_files`, `decisions`, `non_goals`, `context_anchors`, `risk_notes` — and no `open_escalations`. This is exactly the failure class that motivated the ticket: a caller supplies a field, the call succeeds, and the data vanishes. It is worse than a rejection because nothing signals the loss.

   This matters because `open_escalations` must be empty for a package to be implementation-ready. A gate that reads a field the store never persisted cannot enforce anything.

2. **`SessionValidationGate` still has no `command` field.** Validation commands pasted into a handoff are lost. The live workaround was creating test-api `ValidationSpec` entries (`val-session-api-lib-suite`, `val-session-api-build`) and referencing them by id, which forces every validation command through a second store.

3. **Ticket `forward_handoff_package` inversion.** Confirm whether tickets still carry a full package body inline now that the record owns it. If they do, the duplicate storage path must be collapsed to a reference.

## Decision

The session record already owns the package. Close the remaining holes so nothing supplied by a caller is silently discarded.

## Scope

- Persist `open_escalations` in the handoff record and include it in the returned schema. Decide whether it is a required field like its siblings; an empty list is meaningful and should be stored as such, not omitted.
- Audit `session_handoff` for any other accepted-but-not-persisted parameter. `open_escalations` was found by round-trip diffing inputs against the returned record schema — apply that same diff to every parameter and make silent drops impossible.
- Add a `command` field to `SessionValidationGate` so validation commands survive in the handoff itself, and keep `validation_spec_id` as the optional link to test-api evidence rather than the only way to express a command.
- Determine whether ticket `forward_handoff_package` still stores package bodies inline. If it does, collapse it to a reference to the owning handoff record and provide a back-compat read path for existing inline packages (at minimum epic `1fbf2d84` and ticket `9d527ad1`).
- Do NOT re-implement the eight-field package schema. It already exists and works.

## Acceptance Criteria

1. `open_escalations` supplied to `session_handoff` is persisted and returned unchanged, including when it is an empty list.
2. A round-trip test asserts that every parameter accepted by `session_handoff` appears in the persisted record. Any parameter that is accepted but not persisted fails the test rather than being silently dropped.
3. `SessionValidationGate` carries a `command` field; a validation command supplied in a handoff survives the round trip without requiring a test-api `ValidationSpec` entry.
4. Ticket `forward_handoff_package` either resolves to the owning handoff record or is confirmed already free of inline bodies, with the finding recorded.
5. Existing inline packages remain readable after the change.
6. The package has exactly one authoritative home — no parallel storage path.

## Context

- **Verification artifact**: handoff `74d8b170-aef5-4a12-a8ec-e665e34bb585`, persisted 2026-07-27 into workspace session `0101b7ef-e717-4c94-bebd-c8d55f6aaa82`. This is the record that proved seven of the eight fields already round-trip and that `open_escalations` does not.
- Earlier records showing the older shape: `9cd7050b-63b3-430a-8732-8f27952aaaf4`, handoff `dcf86212`
- Possible inversion examples to check: tickets `9d527ad1` and `1fbf2d84` (`forward_handoff_package`)
- Related precedent: the `ValidationSpec.command` workaround in the test-api store, using `val-session-api-lib-suite` and `val-session-api-build`
- Governing spec: `5e52039d` Handoff Package Schema (state: reviewed)
- Blocks `fb14754e`, which extends `context_anchors` to carry store-qualified physical paths

Raised during the iteration that closed ticket `41ff230b`. Re-scoped 2026-07-27 after the original premise was empirically disproved.