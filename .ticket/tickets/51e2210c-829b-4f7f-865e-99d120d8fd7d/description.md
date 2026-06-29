## Problem

The memory-matrix currently exercises domain operations against initialized fixture stores. It does not separately prove missing-store behavior, and some domain helpers intentionally use `open_or_init()`, which is the wrong primitive for the negative contract.

## Scope

Extend memory-matrix with a missing-store policy slice across applicable memory-api domains.

## Acceptance Criteria

- The matrix can materialize a fixture variant with selected hidden store roots absent.
- For each applicable domain, strict open/read/discovery/search behavior against a missing store returns a missing/uninitialized/blocked result without creating the hidden store root.
- Explicit init or explicit write/create positive controls are represented separately and are the only cells allowed to create store roots.
- Ticket, spec, and rule domains distinguish strict `open()` from `open_or_init()` in the matrix helpers.
- Test-api, log-api, audit-api, session-api, and doc-api rows are either covered with domain-appropriate assertions or marked blocked with concrete API-gap reasons.
- The matrix records validation evidence for pass/fail/blocked cells using the existing test-api evidence flow.
