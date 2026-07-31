## Problem
The epic's core claim is that request->role routing becomes deterministic (first-match-wins). This must be validated, not assumed, once the routing table (C1), templates (C2), and deletions (C3) are all in place.

## Scope
- Build a validation check (script or documented manual replay) that takes a corpus of representative past requests (draw from the 226-session dataset referenced in the epic) and confirms each resolves to exactly one role/template via the C1 routing table.
- Confirm no request in the sample matches zero rows or more than one row ambiguously.
- Record results as validation evidence (test-api) linked to this ticket and to epic c608f5ac.

## Affected paths
- No fixed file scope; likely adds a script under tools/ or a validation doc — confirm target location during implementation since it depends on final template layout from C2/C3.

## Acceptance criteria
- [ ] Representative request sample replayed against the C1 routing table
- [ ] 100% of sampled requests resolve to exactly one role (or documented exceptions with follow-up ticket)
- [ ] Validation results recorded via test-mcp and linked to this ticket
