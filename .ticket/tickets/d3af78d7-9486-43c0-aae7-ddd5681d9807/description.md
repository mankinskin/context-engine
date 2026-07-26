## Goal

Define the **handoff-package schema**: the required fields that make the next implementation session fully self-contained (zero discovery, zero user clarification). Document it as a spec and enforce it by extending the existing `session_handoff` record.

## Required fields (draft — refine in spec)

- **objective** — the single goal of the next implementation unit.
- **target tickets** — ticket ids + current state + acceptance criteria (inlined, not referenced-by-lookup).
- **target files** — explicit workspace-relative paths to touch.
- **decisions already made** — resolved design choices so implementation does not re-decide.
- **validation commands** — exact commands / checks that prove the unit done.
- **out-of-scope / non-goals** — explicit boundaries.
- **context anchors** — prior findings, links, ids needed so no search is required.
- **open escalations** — must be empty for a package to be implementation-ready.

## Work

- Author the handoff-package schema spec (paired with T3 iteration-loop spec; separate spec per design decision).
- Extend `session_handoff` so a produced record must carry these fields (validation / required-field enforcement) and can be read back by the next session.

## Acceptance criteria

- A handoff-package schema spec exists enumerating required + optional fields with definitions.
- `session_handoff` produces/validates a record satisfying the schema; a package missing required fields is rejected or flagged.
- A next implementation session can execute purely from the package without search or user Q&A.
- Spec is linked to this ticket and to the epic.

## Review round 2 — FAIL (spec-wording alignment outstanding)

Code enforcement (AC2) and spec linkage (AC4) now pass. One blocking finding remains:

- Spec 5e52039d body (~line 15) still lists `validation` as a required PACKAGE field. The implementation treats validation as a separate `create_handoff_record` parameter (`validation: Vec<SessionValidationGate>`), NOT a field on the `SessionHandoffPackage` struct. Update the spec body so `validation` is documented as a separate handoff-record parameter, not a package schema field. This is a spec-text-only edit; no code change needed.