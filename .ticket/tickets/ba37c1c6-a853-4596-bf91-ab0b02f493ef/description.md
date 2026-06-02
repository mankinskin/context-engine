# Problem

The README schema work needs a stable failing test surface before any parser or renderer changes land. Without that, schema inheritance and required-block behavior will be guessed rather than proven.

## Scope

Add focused failing tests and fixtures in `rule-api` for shared README schema inheritance, schema extension, and missing required block failures.

## Assumptions To Prove

- A test fixture can model one shared `repository-readme-v1` schema plus repo-specific overrides.
- Missing parent, child, installable-content, and command-doc blocks can be expressed as deterministic failing cases.
- Imported child target configs keep config-relative output roots even when schema data is added.

## Test-First Plan

1. Add fixture configs for one shared schema and at least two consuming repos.
2. Add failing tests for inheritance, node append or replace behavior, and missing-block rejection.
3. Keep the test names stable under a dedicated prefix so later implementation tickets can target them directly.

## Acceptance Criteria

- New failing tests exist for schema inheritance and missing-block validation.
- The tests demonstrate one shared schema consumed by more than one workspace target.
- The tests can be rerun with a focused prefix.

## Validation

- `cargo test -p rule-api readme_schema_ -- --nocapture`
