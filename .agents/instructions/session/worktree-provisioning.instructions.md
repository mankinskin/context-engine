---
description: "Use when bootstrapping a session or diagnosing automatic worktree provisioning. Covers the capture-hook triggers, provisioning policy, environment controls, silent-skip guards, troubleshooting, and worktree tooling."
applyTo: "**"
---

## What Fires And When

The capture hook can provision a session worktree before the session's first tool call. VS Code loads [.github/hooks/hooks.json](../../../.github/hooks/hooks.json) through the `.chat.hookFilesLocations` setting in [.vscode/settings.json](../../../.vscode/settings.json). The registered binary is `copilot-capture-hook`, installed on `PATH` at `~/.cargo/bin/copilot-capture-hook`.

`UserPromptSubmit` is the only registered event that runs before any tool call. Eager provisioning is attached to that event so a session receives an isolated worktree before implementation begins. The event timeout is 300 seconds to allow a cold provision.

The binary usage is:

```text
Usage: copilot-capture-hook (session sync ingest) [--from-hook-stdin] [--transcript-path <PATH>] [--store-root <PATH>] [--workspace-slug <SLUG>] [--trigger <NAME>]
```

The hook writes `{}` to stdout for both success and early skip, with diagnostics only on stderr. There is no hook log file, so an externally silent success is indistinguishable from a silent skip.

## Hook Events

| Event | Registered command | Timeout |
|---|---|---:|
| `UserPromptSubmit` | `copilot-capture-hook --from-hook-stdin` | 300s |
| `PreToolUse` | `bash tools/agent-hooks/rtk-hook-copilot.sh`, then `bash tools/agent-hooks/preflight-write.sh` | 5s, 30s |
| `PostToolUse` | `bash tools/agent-hooks/validate-docs.sh`, `bash tools/agent-hooks/terminal-pwd.sh`, then `copilot-capture-hook --from-hook-stdin` | 30s, 5s, 120s |
| `Stop` | `copilot-capture-hook --from-hook-stdin` | 120s |
| `SessionEnd` | `copilot-capture-hook --from-hook-stdin` | 120s |

Hook stdin accepts `transcript_path`, `workspace_slug`, `hook_event_name`, `tool_use_id`, `session_id`, and `tool_response`. `hook_event_name` supplies the trigger. A blank or unknown trigger normalizes to `stop`.

## Provisioning Decision Flow

The hook crate lives in [memory-api/crates/session-capture-hook](../../../memory-api/crates/session-capture-hook/), with entry point [main.rs](../../../memory-api/crates/session-capture-hook/src/main.rs). The provisioning crate is [memory-api/crates/session-worktree-provision](../../../memory-api/crates/session-worktree-provision/), with policy in [policy.rs](../../../memory-api/crates/session-worktree-provision/src/policy.rs). Both crates compile into the ordinary release binary; provisioning is not feature-gated or test-only.

Eager provisioning runs only for `UserPromptSubmit` with a non-blank session id. `WORKTREE_EAGER_PROVISION` is opt-out: the condition treats unset as enabled and only `0` as disabled.

The policy follows reuse, reclaim, then create:

1. **Reuse**: `AlreadyProvisioned` returns when a registered worktree name starts with `{short_id}-`. A manually created `{short_id}-<slug>` worktree is therefore adopted instead of duplicated.
2. **Reclaim**: a candidate must have no session-store activity, a branch, a clean worktree, no current directory inside the worktree, idle age beyond `WORKTREE_IDLE_SECS`, no dirty submodule path, and zero commits ahead of `main`. Candidates sort by mtime then name. Failed reclaim falls through to create.
3. **Create**: below `WORKTREE_MAX`, provisioning creates a new branch from `main`. At the cap, with no reclaim candidate, provisioning returns `CapReached`.

Reclaim relocates the directory, runs `git worktree repair`, and renames the branch. `git worktree move` cannot perform the relocation because every repository worktree contains five submodules.

## Reclaim Gates

Reclaim is deliberately conservative. Every gate below must pass before a registered worktree becomes a reclaim candidate.

| Gate | Required condition |
|---|---|
| Session activity | No session-store activity belongs to the candidate. |
| Branch | The candidate has a branch. |
| Working tree | The candidate is clean. |
| Current directory | The candidate does not contain the process current directory. |
| Idle age | The candidate has been idle longer than `WORKTREE_IDLE_SECS`. |
| Submodules | No submodule path is dirty. |
| Branch history | The candidate is zero commits ahead of `main`. |

The policy orders passing candidates by modification time and then name. A failed relocation, repair, or rename is not terminal: the policy continues to the create path when capacity remains.

## Capture Resolution

The capture hook first resolves a session-specific store root before any provisioning decision. A valid capture path needs a non-blank session id, a usable current directory, an anchored checkout containing `.session`, successful resolver construction, and a session mapping to a worktree or store root.

The resolver uses `MCP_MAIN_CHECKOUT` when that environment variable has a non-empty value; otherwise the current directory anchors the checkout. Missing prerequisites result in an early return rather than an error visible through stdout.

For a valid `UserPromptSubmit`, [main.rs](../../../memory-api/crates/session-capture-hook/src/main.rs) evaluates `WORKTREE_EAGER_PROVISION` before calling the policy. The equivalent source condition is:

```rust
std::env::var_os("WORKTREE_EAGER_PROVISION").is_none_or(|value| value != "0")
```

The condition means unset enables eager provisioning. A literal `0` is the only documented opt-out value.

## Git Responsibilities

The provisioning crate uses `git2` (libgit2) for every read: opening repositories, listing worktrees, branch existence, dirty checks, ahead/behind, gitlink lookup, and `.gitmodules` parsing.

The `git` subprocess performs only writes that libgit2 cannot express: `git worktree add -b`, `git worktree add --detach`, filesystem relocation followed by `git worktree repair`, `git branch -m`, `git worktree remove --force`, `git worktree prune`, `git branch -d` or `-D`, and nested `git worktree repair`.

## Naming And Paths

`short_id` is the first eight characters of the session id. Eager provisioning uses these names:

| Resource | Template |
|---|---|
| Worktree | `<main_checkout>/.worktrees/{short_id}-session` |
| Branch | `agent/{short_id}-session` |

The reuse prefix intentionally permits a manual `<short_id>-<slug>` worktree. Use the session's first eight characters when manually bootstrapping a worktree that the hook should adopt.

## Environment Variables

| Name | Default | Effect |
|---|---|---|
| `WORKTREE_EAGER_PROVISION` | unset = enabled | Set to `0` to disable eager provisioning. |
| `WORKTREE_MAX` | `8` | Maximum registered worktrees before `CapReached`. |
| `WORKTREE_IDLE_SECS` | `86400` | Idle window before reclamation is allowed. |
| `WORKTREE_STALE_SECS` | `14400` | Stale-session window. |
| `MCP_MAIN_CHECKOUT` | current directory | Overrides the anchor checkout; an empty string is ignored. |

## Silent-Skip Guards

`run()` returns early if capture-store resolution returns `None`. Resolution fails closed when the payload lacks a non-blank session id, `current_dir()` fails, the anchored checkout lacks `.session`, resolver construction fails, or the session cannot map to a worktree or store root.

Provisioning is not reached without `UserPromptSubmit` and a non-blank `session_id`. `WORKTREE_EAGER_PROVISION=0` skips eager provisioning, and `AlreadyProvisioned` short-circuits an existing matching worktree. These early paths emit the same `{}` stdout as successful creation.

## Troubleshooting

Because the hook is intentionally quiet, use this checklist to establish whether automatic provisioning fired:

1. **Check the current checkout.** Run `git rev-parse --show-toplevel`. A repository-root path rather than `.worktrees/...` means automatic provisioning did not place the session in a worktree.
2. **Check registered worktrees.** Run `git worktree list` and find a name beginning with the session id's first eight characters.
3. **Check hook execution.** Look for a recent directory in `.session/sessions/` matching the session id. No record means the hook did not execute for that session. Reload the VS Code window after changes to [.github/hooks/hooks.json](../../../.github/hooks/hooks.json) or [.vscode/settings.json](../../../.vscode/settings.json), because VS Code reads hook registration at window start.
4. **Check the opt-out.** Run `echo "[$WORKTREE_EAGER_PROVISION]"`. A value of `0` disables provisioning.
5. **Check the cap.** Compare the `git worktree list` count to `WORKTREE_MAX`, which defaults to 8.
6. **Use the manual fallback.** Run `bash tools/worktree/worktree.sh new <short-id> <slug>`. A matching `<short-id>` makes the hook reuse the manually created worktree on the next prompt instead of creating another worktree.

`WORKTREE_IDLE_SECS` defaults to 24 hours. Existing worktrees have been touched recently, so reclaim will not occur in the near term; new sessions pay the full cold-provision cost and count toward `WORKTREE_MAX`.

## Shell Helper Reference

[tools/worktree/worktree.sh](../../../tools/worktree/worktree.sh) supports `--dry-run` on every subcommand, and `-h` or `--help` prints usage.

| Subcommand | Signature | Behavior |
|---|---|---|
| `new` | `new <short-id> <slug>` | Creates `.worktrees/<short-id>-<slug>` and `agent/<short-id>-<slug>` from local `main`, populates submodules offline, and rolls back on failure. |
| `list` | `list` | Lists `.worktrees` entries, branches, submodule initialization, and unregistered debris. |
| `rebase` | `rebase <name>` | Rebases the worktree branch onto local `main`; stops on conflict. |
| `merge` | `merge <name>` | Fast-forwards nested submodule branches and then the superproject into `main`. |
| `remove` | `remove <name>` | Removes the worktree, prunes registrations, and deletes the branch. |
| `doctor` | `doctor` | Repairs deinitialized main-checkout submodules and reports stale registrations. |

## Verification Evidence

Measured on 2026-08-07:

- `cargo test -p session-worktree-provision -p session-capture-hook` passed 38 tests: 10 in `session-capture-hook`, 4 in `copilot_stop_hook_e2e`, and 24 in `session-worktree-provision`. The repository state was byte-identical before and after the run; the tests no longer create worktrees in the developer checkout.
- A live end-to-end invocation of the installed binary with a synthetic `UserPromptSubmit` payload exited 0, wrote `{}`, cold-provisioned in 92 seconds, created the worktree and branch, and populated all 5 of 5 submodules.
- A second identical invocation completed in 0.077 seconds, created nothing, and demonstrated the reuse path. Cleanup restored the baseline exactly.

## Known Gaps And Follow-Up

`bash tools/worktree/tests/run.sh` currently reports 6 passed and 10 failed. The first concrete failure is `error: unknown subcommand: rename`; affected tests cover dry runs, dirty-checkout acknowledgement and preservation, finish/remove/rename behavior, submodule initialization, explicit override, commit-ahead preservation, and session reuse. Those tests encode the not-yet-merged Rust rewrite contract, so the failures are a known rewrite gap rather than a provisioning regression.

Ticket `5e6cf4f8` (Rewrite worktree.sh as a Rust binary and add worktree lifecycle recycling) is open. The planned `worktree-ctl` binary will provide `list`, `rebase`, `merge`, `remove`, and `doctor`, retiring the shell scripts. Related ticket `3d535b2c` (Add prompt-time worktree bootstrap hook) is open; ticket `0f5acbfe` (Session-id worktree routing: discovery resolver, capture-hook self-heal, and terminal stdio isolation) is reviewed.

## Manual Lifecycle Protocol

Automatic provisioning only establishes or reuses the isolated checkout. [branch-worktree.instructions.md](../commit/branch-worktree.instructions.md) owns the manual lifecycle: claim the session and board, work and commit on the feature branch, rebase onto local `main`, mark ready to merge, and leave merging to the root orchestrator.