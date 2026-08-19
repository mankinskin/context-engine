## Problem

`worktree-ctl bootstrap` populates only the direct, first-level submodules of
`context-engine` (via `git.submodule_paths()` in
`workflow-tools/session/crates/worktree-ctl/src/main.rs`, formerly
`tools/worktree/worktree-ctl/src/main.rs`). It does not recurse into
`workflow-tools`'s own nested submodules (`session`, `rule`, `ticket`, `spec`,
`test`, `feedback`, `doc`, `audit`, `log`, `peek`, `interview`).

Every new worktree created against a ticket that touches one of these nested
domain repos currently requires manual population, e.g.:

```bash
sha=$(git -C "$WT/workflow-tools" ls-tree HEAD "$m" | awk '{print $3}')
git -C "$MAIN/workflow-tools/$m" worktree add --detach "$WT/workflow-tools/$m" "$sha"
```

for each nested submodule `$m` needed, before `cargo build --workspace`
succeeds in the new worktree.

## Evidence

Hit repeatedly across the `workflow-tools/session` crate-relocation series
(session-record-merge, session-workspace-resolver, mcp-toolmon,
toolmon-costgate, toolmon-policy-api, worktree-ctl, model-prices) — every new
worktree needed manual `git worktree add --detach` for `session`, `rule`,
`ticket`, `spec` submodules-of-submodules before the build would succeed.

## Acceptance Criteria

- `worktree-ctl bootstrap` recurses at least one level deeper: after
  populating `context-engine`'s direct submodules, it also detects and
  populates each direct submodule's own submodules (starting with
  `workflow-tools`'s nested domain repos).
- A fresh worktree created via `worktree-ctl bootstrap` for a ticket touching
  any `workflow-tools/<domain>` crate builds with `cargo build --workspace`
  without any manual `git submodule update --init` / `git worktree add`
  step.
- Existing worktree-ctl tests continue to pass; add a regression test
  covering the nested-submodule-of-submodule bootstrap case if the harness
  supports it.

## Notes

Flagged as a follow-up during the `workflow-tools/session` crate relocation
work; not addressed inline to keep that change mechanical and scoped.
