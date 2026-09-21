## Completed

All 6 domains relocated from `memory-api`/`memory-viewers` into
`workflow-tools/{audit,test,doc,log,feedback,peek}/crates/*`, following the
established recipe (physical move, intra/cross-domain dep fixes, root
Cargo.toml members + `[patch]` blocks, artifacts.toml, living docs).

Cross-domain deps fixed:
- `log-api` → `test-api` (git, was in-tree path)
- `test-cli` → `log-api` (git, was in-tree path)
- `memory-matrix` → `audit-api`/`test-api`/`log-api` (git, were in-tree path)
- `rule`, `rule-api`, `session-api`, `spec-api`, `ticket-api`, `ticket-viewer`
  → `feedback-api`/`feedback-http`/`test-api` repointed from
  `mankinskin/memory-api` to `mankinskin/feedback`/`mankinskin/test`.

Also fixed a legacy hardcoded path table in the root `install-tools.sh`
(`direct_path_for`) that still pointed `peek-cli`/`test-cli` at
`memory-api/tools/cli/*` — this predates `artifacts.toml` and wasn't caught
by the artifacts.toml grep from the prior relocation round.

Validation: `cargo build --workspace` clean on both the worktree and the
main checkout after fast-forward. Package tests for every moved crate pass;
the only 3 failures encountered (`peek-cli`'s `repo_map_contracts` test
missing a `viewer-api` dev-dependency, and 2 `memory-matrix` matrix-test
flakes) were reconfirmed identical on the unmodified pre-change baseline —
pre-existing, not introduced here.

`install-tools.sh --all` succeeded 30/30 after one non-offline
`cargo install` cache-warm (same pattern as the prior session-crate
relocation round).

## Merge sequence

Commits landed bottom-up on each repo's `main`: audit, test, log, doc,
feedback, peek → rule, spec, session, ticket (dependency repoints) →
memory-viewers, memory-api → workflow-tools → context-engine root.
Two repos (`workflow-tools`, `memory-api`) needed a rebase mid-flight
because other concurrent session activity had advanced their `origin/main`
past this session's fork point; both rebased cleanly (one `ticket`
submodule-pointer conflict in `workflow-tools`, resolved to the final merged
`ticket` main tip).

## Not addressed (out of scope)

- `compact-terminal-*` / `fs-*` stay in `memory-api` (not requested).
- worktree-ctl submodule-of-submodule bootstrap gap — tracked separately in
  ticket 5bea5e20.
- Historical `.ticket/`/`.spec/` references to old paths left untouched as
  archival record.
