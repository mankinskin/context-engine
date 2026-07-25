Phase 0 (blocking, user action). Create the GitHub repositories required for the workflow-tools extraction, then confirm the repo URLs/owner back to the agent so extraction can begin.

Owner: `mankinskin` (confirmed).

## Repositories to create (owner: mankinskin)
Shared libraries:
- `memory-kernel` — shared storage/index/search kernel (extracted from memory-api/crates/memory-api)
- `memory-fixtures` — shared test support (memory-fixtures + memory-matrix)

Per-tool repos (bare domain names):
- `ticket`
- `spec`
- `rule`
- `doc`
- `test`
- `log`
- `feedback`
- `session`
- `audit`
- `peek`
- `interview` (placeholder; crate not yet built)

Umbrella + packaging:
- `workflow-tools`
- `workflow-skill`

Already exist (no action): `viewer-api`, `context-stack`. The `memory-api` repo name is freed after extraction; `memory-viewers` is dissolved into the per-tool repos.

## Acceptance criteria
- All listed repositories exist under github.com/mankinskin (empty/initialized is fine).
- Repo URLs (or confirmation of the owner + exact names) are provided back to the agent.
- Default branch and visibility confirmed for each repo.

## Notes
This ticket gates every extraction ticket. Do not begin Phase A/B extraction until the repos exist and are confirmed.