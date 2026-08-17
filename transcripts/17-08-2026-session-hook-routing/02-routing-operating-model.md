# Routing Operating Model

## Authority and States

The main checkout owns committed session history and a committed working-branch declaration. `<main>/.session/local/worktree-registry.json` is the authoritative local-only UUID-to-canonical-path registry. `.session/sessions/**` is tracked; `.session/local/**` is ignored. See [.gitignore](.gitignore#L30-L47).

| State | Committed main record | Local registry | Worktree | Resolution |
| --- | --- | --- | --- | --- |
| Main only | `.session/sessions/<uuid>/session.json` exists; no active branch declaration. | No UUID entry. | None. | Capture/MCP use main session/store. |
| Registered | Record commits `agent/<uuid>/<slug>` but no absolute path. | UUID maps canonical path, same branch, active status. | Canonical Git checkout. | Capture/MCP write only there. |
| Deregistered | New commit unsets branch declaration. | No UUID entry. | Former worktree clean/equal to `main`, then removable. | Main-only result. |

`metadata.worktree` currently mixes path and branch. Target state commits only branch state and keeps path/status in ignored registry; committed absolute paths are forbidden.

## Transfer: `worktree-ctl transfer-session`

The new command belongs at [worktree-ctl](tools/worktree/worktree-ctl/src/main.rs#L20-L166), whose current binary should delegate orchestration to `session-worktree-provision` beside `WorktreeGit`/`provision_for_session`; inject a narrow session-record persistence port. `session_check_in` is unsuitable because it writes path-bearing state and can create directories. See [session runtime](memory-api/crates/session-api/src/store/config/worktree_runtime.rs#L1-L151).

| Order | Action | Failure rule |
| --- | --- | --- |
| 1 | Validate UUID, clean main, no active registration, intended branch. | Fail before writes. |
| 2 | Commit main-session branch declaration as `transfer-pending`; read back. | Fail before creation. |
| 3 | Create `.worktrees/<uuid>/<slug>` on branch; populate submodules. | Block; do not revert step 2. Recovery is later forward unset commit. |
| 4 | Validate canonical Git checkout, branch equality, store containment. | Block; later forward unset only. |
| 5 | Atomically write/read ignored registry entry. | Block; preserve pending declaration until completed or forward-unset. |
| 6 | Resume capture/MCP only after read-back. | Never fall back to another checkout. |

The committed entry precedes instantiation; the registry cannot be committed because it is local-only.

## Resolution Contract

| Consumer | Main-only | Registered | Bad registration |
| --- | --- | --- | --- |
| Capture hook | Persist `<main>/.session`. | Read registry, validate UUID/path/branch/Git checkout, persist only `<worktree>/.session`. | Missing, noncanonical, missing, branch-mismatched path blocks capture; no positional substitute. |
| `mcp-toolmon` | Explicit main anchor and main session/store. | Read same registry, rewrite only registered worktree, enforce containment. | Reject with UUID/condition diagnostic; never forward main/other worktree. |
| Resolver | Typed main-only target. | Registry authoritative; positional discovery migration-only. | Typed invalid-registration error. |

Dirty registered worktree blocks capture/mutation. Main may receive explicit diagnostic metadata only; normal transcript/entity writes never recover by targeting main.

## Deregistration: `worktree-ctl deregister-session`

The operation is hook-free: no capture hook, MCP tool, or auto-provisioning.

| Preconditions | Operation | Observable post-state |
| --- | --- | --- |
| Entry exists; canonical Git worktree; clean worktree/submodules; `git diff --exit-code main...<branch>` empty; no active lease/process. | Lock registry; commit main record unsetting branch; remove/read local entry. | No active branch, no UUID registry entry, resolver main-only, existing removal allowed. |

Failure before commit preserves registered state. Failure after commit blocks removal until registry reconciliation. Repeat is idempotent only when both states prove main-only.

## Compatibility

| Existing condition | Rule |
| --- | --- |
| Eager nested/legacy path-bearing record | Explicit migration validates path/branch, writes registry, then forward record revision retains only branch declaration. |
| Positional path without registry/branch | Unregistered debris or blocked migration candidate; never implicit routing. [2b657154 Handle unregistered worktree debris during removal](.ticket/tickets/2b657154-df78-4bb3-807a-66c9ff811ceb/ticket.toml) remains adjacent. |
| Missing registry after transfer | Block; recovery/migration required, never positional restoration. |
| [200e9ecc session_check_in rejects a single nested worktree unless a persisted assignment exists](.ticket/tickets/200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915/ticket.toml) | Supersede positional expectation; legacy reader only for migration. |
