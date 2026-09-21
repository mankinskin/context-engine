Align `context-engine/tools/verify-bootstrap.sh` with the supported store domains so migration baseline verification does not require an absent `.doc` store.

Source: `transcripts/30-08-2026_spec-system-improvement-planning/00-bootstrap-verifier-alignment.md` (Roadmap Waypoint 2).

## Dependencies
- `ARTIFACTS.md`
- `context-engine/tools/verify-bootstrap.sh`
- Store-domain classification from `migration-matrix.md` (Waypoint 1, completed).

## Work
1. Confirm no repository-backed `.doc` store is a migration source or destination.
2. Change the verifier's required-store policy to match the supported migration domains (`.ticket`, `.spec`, `.rule`, `.test`, `.session`, `.feedback`, `.audit`), or replace it with a migration-specific verifier that does.
3. Add a focused regression test for the resolved policy.

## Non-Goals
- Do not create a `.doc` store.
- Do not migrate or delete store records.

## Acceptance Criteria
- `bash context-engine/tools/verify-bootstrap.sh --skip-check` passes in the verified baseline repository layout.
- The focused regression test passes.
