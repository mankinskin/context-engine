## Problem

HTTP read probes against a configured workspace path with no on-disk `.ticket` root must not auto-create stores. The regression target is workspace-resolved read/list probes.

## Implemented Reproducer

- Added: `memory-api/tools/http/ticket-http/tests/no_auto_init_missing_workspace.rs`
- New test builds an app with `WorkspaceRegistry::single(<missing .ticket path>)` and probes:
  - `GET /api/workspaces`
  - `GET /api/tickets?workspace=<canonical>&limit=10`
- Contract assertion: both probes must leave `.ticket` absent; ticket listing should return `404` for missing workspace instead of implicitly creating/opening it.

## Validation

- Command: `cargo test --manifest-path tools/http/ticket-http/Cargo.toml --test no_auto_init_missing_workspace -- --nocapture`
- Result: **failing (expected reproducer)**
- Failure evidence: `left: 200 right: 404` at `no_auto_init_missing_workspace.rs:66`.

## Blocker Summary

Current behavior returns `200` for `/api/tickets` in this missing-store fixture path, indicating implicit initialization/open behavior remains in the HTTP workspace-resolution path.

## Next Step

Adjust HTTP workspace store resolution to strict-open semantics for read-only probes in missing-store scenarios, then re-run this test to green.
