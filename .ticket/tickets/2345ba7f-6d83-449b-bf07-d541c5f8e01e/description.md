Phase F. End-to-end validation and cutover across the split repositories: prove that the extracted tools build, test, and operate correctly both standalone and aggregated, and that context-engine works with workflow-tools installed as a dependency.

## Scope
- Build + test each per-tool repo, shared libs, and workflow-tools aggregate.
- Smoke every transport (cli, mcp, http) per tool.
- Browser-verify every viewer in an external fullscreen browser; capture screenshots; add/port Playwright E2E.
- Clean-checkout reproduction of context-engine using the installed workflow-tools dependency.
- Reference-integrity validation across migrated artifact stores.

## Acceptance criteria
- All repos green in CI independently and aggregated.
- Transport + viewer verification recorded (screenshots + Playwright evidence, browser resolution noted).
- Clean-checkout context-engine environment reproduces successfully.
- No dangling references post-migration.

## Dependencies
- Blocked by all extraction, artifact, skill, and reframing tickets (final gate).