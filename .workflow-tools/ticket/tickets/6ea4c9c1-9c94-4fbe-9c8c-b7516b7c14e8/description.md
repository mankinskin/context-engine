## Objective

Continue the `workflow-tools` per-tool crate extraction: move the remaining
monorepo-coupled tool families out of `memory-api`/`memory-viewers` into
their own standalone domain repos under `workflow-tools/`, following the
same recipe used for `session`, `rule`, `ticket`, `spec`:

1. `audit` — `memory-api/crates/audit-api`, `memory-api/tools/cli/audit-cli`,
   `memory-api/tools/mcp/audit-mcp`.
2. `test` — `memory-api/crates/test-api`, `memory-api/tools/cli/test-cli`,
   `memory-api/tools/mcp/test-mcp`.
3. `doc` — `memory-api/crates/doc-api`, `memory-api/tools/http/doc-http`,
   `memory-viewers/doc-viewer`.
4. `log` — `memory-api/crates/log-api`, `memory-viewers/log-viewer`
   (+ its `frontend/dioxus` member).
5. `feedback` — `memory-api/crates/feedback-api`,
   `memory-api/tools/http/feedback-http`,
   `memory-api/tools/cli/feedback-cli`, `memory-api/tools/mcp/feedback-mcp`.
6. `peek` — `memory-api/crates/peek-api`, `memory-api/tools/cli/peek-cli`,
   `memory-api/tools/mcp/peek-mcp`.

Each destination repo (`workflow-tools/{audit,test,doc,log,feedback,peek}`)
currently exists as an empty scaffold (README only), matching the starting
state `session`/`rule` had before their extractions.

## Recipe (per domain, established precedent)

1. Physically move crate directories into `workflow-tools/<domain>/crates/`
   (or top level for viewer/frontend crates, matching existing
   `ticket-viewer`/`spec-viewer` layout).
2. Fix internal `Cargo.toml` deps: intra-domain deps go `git` → local
   sibling `path`; cross-repo deps stay/become `git`.
3. Update root `Cargo.toml` workspace `members` and add/update
   `[patch."https://github.com/mankinskin/<domain>"]` blocks.
4. Update `tools/install/artifacts.toml` `source_path` entries for any
   moved binaries.
5. Update `.vscode/mcp.json` / `.github/mcp.json` if any moved binary is
   referenced there.
6. Update living orchestration docs (`.agents/instructions/**`,
   `.agents/agents/**`, `.agents/prompts/**`) that reference the old paths;
   leave historical `.ticket/`/`.spec/` references untouched as archival
   record.
7. Validate: `cargo build --workspace`, then `cargo test` for each touched
   crate; compare any failures against the unmodified main-checkout
   baseline to confirm pre-existing vs. newly introduced.
8. Commit on the session's feature branch in each affected repo
   (domain repo, `workflow-tools` superproject, `memory-api` if applicable,
   `context-engine` root), push, then fast-forward `main` in each,
   bottom-up.
9. Re-run `install-tools.sh --all` at the end of the whole batch to confirm
   the full tool catalog still installs cleanly (30+ tools).

## Notes

- `compact-terminal-api`/`compact-terminal-mcp`/`compact-terminal-cli` and
  `fs-api`/`fs-mcp`/`fs-cli` are intentionally out of scope for this ticket
  (not named by the requester); they remain in `memory-api` for now.
- Known unfixed limitation: fresh worktrees do not auto-populate
  `workflow-tools`'s nested submodules-of-submodules — see
  ticket 5bea5e20 (worktree-ctl bootstrap gap). Manual
  `git worktree add --detach` per nested submodule is required until that
  is fixed.
- `cargo install --path <dir>` ignores the root `[patch]` table (ephemeral
  workspace); after pushing each domain repo, one non-`--offline`
  `cargo install`/`cargo update` is needed to warm the git cache before
  `install-tools.sh --offline` will succeed for tools depending on newly
  pushed commits.
