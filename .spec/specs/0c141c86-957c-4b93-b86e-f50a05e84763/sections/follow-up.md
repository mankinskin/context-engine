# Follow-up implementation tracks

The remaining work is intentionally split into separate tickets so tool behavior, verification policy, repository integration, and cleanup sequencing do not collapse into one mixed record.

## Ticket map

- `17c99c98-9127-4bd0-90b5-c47f990b56de` — verify `crane-cli` against controlled fixtures and real dry-run review
- `7937930a-e184-41eb-9732-7ac39897d263` — add branch-root rewrite mode to `crane-cli`
- `400f92ff-0f93-46de-a79d-14bf4e2b2ce7` — retarget imported context-stack tool manifests for the standalone layout
- `aaa810f0-cc14-4226-b7d0-d81a38f856e7` — decide and execute source-of-truth cleanup for moved tools

## Why this split exists

- Verification work should be able to tighten confidence in `crane-cli` without also changing migration semantics.
- Branch-root rewrite is a separate capability change, not a side effect of manifest cleanup.
- Standalone manifest retargeting is repository-integration work and should carry its own validation gates.
- Ownership cleanup in `context-engine` depends on integration decisions and should be sequenced explicitly rather than folded into the transplant itself.
