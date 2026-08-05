## Objective
Move browser-tool preference from product specification into contributor instructions and retire spec `347b6f97`.

## Context
Owner decision: “contributor policy, not product behavior. Move to `.agents/instructions/`, retire the spec.”
Audit: `tmp/test-coverage-audit/03-requirements.md`.

## Acceptance criteria
- Transfer applicable policy to contributor instructions.
- Retire the product spec without losing the policy.
- Record migration traceability.

## Out of scope
- Changing browser-facing product behavior.