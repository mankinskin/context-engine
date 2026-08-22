<!-- aligned-structure:v2 -->

# Tests

## Responsibility And Interface

Expose ticket criteria to measurement and persist outcomes as queryable evidence.
Consume Tickets' two criteria; record definitions/executions under
`.test/<workspace>/specs/` and `executions/` through test-mcp or
`./target/debug/test.exe --store-root "$PWD/.test" record-spec|record`; provide
three criteria to Implementation.

## Behavior And Contract

- `tests-measurable-criteria`: maps feasible criteria to commands or manual checks.
- `tests-recorded-evidence`: stores ticket/spec/criterion ids, outcome, and detail.
- `tests-documented-manual-criteria`: records unavailable automation for review.

## Boundaries And Failure Cases

Tests do not redefine criteria or bury outcomes in ticket prose. Failed/blocked
commands remain visible with command and reason; missing test context returns to
the owner for refinement rather than becoming a fabricated pass.

## Acceptance Evidence And Position

`test.exe --store-root "$PWD/.test" list --ticket <id>` must return a real
execution; manual evidence includes criterion, method, and limitation. No run
belongs to this draft, so no `validated_by` is set. The validation-evidence
instruction defines this implemented store contract.
