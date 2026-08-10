---
description: "Use when bootstrapping a session or diagnosing automatic worktree provisioning. Covers the capture-hook triggers, provisioning policy, environment controls, silent-skip guards, troubleshooting, and worktree tooling."
applyTo: "**"
---

## What Fires And When

The capture hook can provision a session worktree before the session's first tool call. VS Code loads [.github/hooks/hooks.json](../../../.github/hooks/hooks.json) through the `.chat.hookFilesLocations` setting in [.vscode/settings.json](../../../.vscode/settings.json). The registered binary is `session-capture-hook`, installed on `PATH` at `~/.cargo/bin/session-capture-hook`.

`UserPromptSubmit` is the only registered event that runs before any tool call. Eager provisioning is attached to that event so a session receives an isolated worktree before implementation begins. The event timeout is 300 seconds to allow a cold provision.

The binary usage is:

```text
Usage: session-capture-hook (session sync ingest) [--from-hook-stdin] [--transcript-path <PATH>] [--store-root <PATH>] [--workspace-slug <SLUG>] [--trigger <NAME>]
```

The hook writes `{}` to stdout for both success and early skip, with diagnostics only on stderr. There is no hook log file, so an externally silent success is indistinguishable from a silent skip.

## Hook Events

| Event | Registered command | Timeout |
|---|---|---:|
| `UserPromptSubmit` | `session-capture-hook --from-hook-stdin` | 300s |
| `PreToolUse` | `bash tools/agent-hooks/rtk-hook-copilot.sh`, then `bash tools/agent-hooks/preflight-write.sh` | 5s, 30s |
| `PostToolUse` | `bash tools/agent-hooks/validate-docs.sh`, `bash tools/agent-hooks/terminal-pwd.sh`, then `session-capture-hook --from-hook-stdin` | 30s, 5s, 120s |
| `Stop` | `session-capture-hook --from-hook-stdin` | 120s |
| `SessionEnd` | `session-capture-hook --from-hook-stdin` | 120s |

Hook stdin accepts `transcript_path`, `workspace_slug`, `hook_event_name`, `tool_use_id`, `session_id`, and `tool_response`. `hook_event_name` supplies the trigger. A blank or unknown trigger normalizes to `stop`.

## Provisioning Decision Flow

The hook crate lives in [memory-api/crates/session-capture-hook](../../../memory-api/crates/session-capture-hook/), with entry point [main.rs](../../../memory-api/crates/session-capture-hook/src/main.rs). The provisioning crate is [memory-api/crates/session-worktree-provision](../../../memory-api/crates/session-worktree-provision/), with policy in [policy.rs](../../../memory-api/crates/session-worktree-provision/src/policy.rs). Both crates compile into the ordinary release binary; provisioning is not feature-gated or test-only.

Eager provisioning runs only for `UserPromptSubmit` with a non-blank session id. A successful provision persists the session-to-worktree assignment, so a fresh session-store handle resolves `session.exe lookup` to the provisioned worktree path and branch. `WORKTREE_EAGER_PROVISION` is opt-out: the condition treats unset as enabled and only `0` as disabled.

The policy follows reuse, reclaim, then create:

1. **Reuse**: `AlreadyProvisioned` returns only when a registered `{short_id}-` worktree has a recorded owner matching the full session id. A prefix-matching worktree owned by another session returns `SessionOwnershipConflict`. A registered worktree with no recorded owner remains claimable by a prefix-matching session for backward compatibility.
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

For a valid `UserPromptSubmit`, the hook routes and attempts eager provisioning before transcript capture or capture-store resolution. The transcript guard runs next and skips only transcript capture when the transcript path is absent. Capture-store resolution then determines whether the hook can persist a session record; a blank `workspace_slug` or an unresolvable store root does not block provisioning.

The resolver uses `MCP_MAIN_CHECKOUT` when that environment variable has a non-empty value; otherwise the current directory anchors the checkout. An explicit external store that does not match the resolved checkout prevents provisioning, but ordinary capture-resolution failures only prevent transcript capture and persistence.

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

The `-session` suffix is a placeholder for a topic not yet declared. The `{short_id}-` prefix permits a renamed `<short_id>-<topic-slug>` worktree to be considered for reuse, but reuse requires a matching recorded full session id; legacy ownerless worktrees remain claimable. Follow [## 1b. Name the topic (rename the worktree)](../commit/branch-worktree.instructions.md#1b-name-the-topic-rename-the-worktree) before session check-in; use the session's first eight characters when manually bootstrapping a worktree that the hook should adopt.

## Environment Variables

| Name | Default | Effect |
|---|---|---|
| `WORKTREE_EAGER_PROVISION` | unset = enabled | Set to `0` to disable eager provisioning. |
| `WORKTREE_MAX` | `8` | Maximum registered worktrees before `CapReached`. |
| `WORKTREE_IDLE_SECS` | `86400` | Idle window before reclamation is allowed. |
| `WORKTREE_STALE_SECS` | `14400` | Stale-session window. |
| `MCP_MAIN_CHECKOUT` | current directory | Overrides the anchor checkout; an empty string is ignored. |

## Silent-Skip Guards

Malformed hook input cannot identify a valid event or session and therefore skips provisioning. Provisioning also skips for any event other than `UserPromptSubmit`, a blank `session_id`, an unavailable current directory, an invalid anchor checkout, `WORKTREE_EAGER_PROVISION=0`, or a mismatched explicit external store. `AlreadyProvisioned` reuses a registered `{short_id}-` worktree only when the recorded owner matches the full session id; a conflicting owner returns `SessionOwnershipConflict`.

A missing transcript path skips only transcript capture, and an unresolvable capture store or blank `workspace_slug` does not block provisioning. All provisioning failures still exit 0 and emit `{}` with diagnostics on stderr, so a silent success, reuse, failure, and skip remain indistinguishable from outside the hook.

## Troubleshooting

Because the hook is intentionally quiet, use this checklist to establish whether automatic provisioning fired:

1. **Check the persisted assignment.** Open a fresh session-store handle and run `./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon`; the returned `worktree_path` and `branch` must match the provisioned worktree and feature branch. This is the acceptance check for eager provisioning.
2. **Check the current checkout.** Run `git rev-parse --show-toplevel`. A repository-root path rather than `.worktrees/...` means automatic provisioning did not place the session in a worktree.
3. **Check registered worktrees.** Run `git worktree list` and find a name beginning with the session id's first eight characters.
4. **Check hook registration.** Reload the VS Code window after changes to [.github/hooks/hooks.json](../../../.github/hooks/hooks.json) or [.vscode/settings.json](../../../.vscode/settings.json), because VS Code reads hook registration at window start. A missing `.session/sessions/` record indicates that provisioning did not persist the required assignment.
4. **Check the opt-out.** Run `echo "[$WORKTREE_EAGER_PROVISION]"`. A value of `0` disables provisioning.
5. **Check the cap.** Compare the `git worktree list` count to `WORKTREE_MAX`, which defaults to 8.
6. **Use the manual fallback.** Run `./target/debug/worktree-ctl.exe new <short-id> <slug>`. A matching `<short-id>` makes the hook reuse the manually created worktree on the next prompt instead of creating another worktree.
7. **Repair a missing submodule worktree path.** If repository-root `git status` reports `fatal: cannot chdir to '../../../../../.worktrees/<name>/memory-api': No such file or directory`, followed by `fatal: 'git status --porcelain=2' failed in submodule memory-api`, a rolled-back or manually deleted worktree left `core.worktree` in the shared submodule config pointing at the missing directory. `git worktree prune` cannot self-heal because it reaches the same error. Run `git config --file .git/modules/<submodule>/config --unset core.worktree`, then `git -C <submodule> worktree prune` and `git worktree prune` at the repository root. Check all five submodules with `git config --file .git/modules/<name>/config --get core.worktree`. Stale `.git/modules/<name>/worktrees/<entry>/gitdir` entries pointing at missing paths are a separate, milder symptom cleared by the same prune.

When hand-constructing a `transcript_path` for a manual hook invocation under Git Bash, use a Windows-style `C:/...` path. The native binary does not resolve POSIX `/tmp/...` paths produced by `mktemp`.

`WORKTREE_IDLE_SECS` defaults to 24 hours. Existing worktrees have been touched recently, so reclaim will not occur in the near term; new sessions pay the full cold-provision cost and count toward `WORKTREE_MAX`.

## Worktree CLI Reference

`./target/debug/worktree-ctl.exe` supports `--dry-run` on every subcommand, and `-h` or `--help` prints usage.

| Subcommand | Signature | Behavior |
|---|---|---|
| `new` | `new <short-id> <slug> [--allow-additional] [--preserve-main-changes] [--dry-run]` | Creates `.worktrees/<short-id>-<slug>` and `agent/<short-id>-<slug>` from local `main`, populates submodules offline, and rolls back on failure. |
| `list` | `list [--dry-run]` | Lists `.worktrees` entries, branches, submodule initialization, and unregistered debris. |
| `rebase` | `rebase <name> [--dry-run]` | Rebases the worktree branch onto local `main`; stops on conflict. |
| `merge` | `merge <name> [--dry-run]` | Fast-forwards nested submodule branches and then the superproject into `main`. |
| `remove` | `remove <name> [--force] [--dry-run]` | Refuses dirty worktrees unless forced, then removes the worktree, prunes registrations, and deletes the merged branch. |
| `rename` | `rename <source-name> <target-name> [--dry-run]` | Re-topics a clean worktree through filesystem relocation, repair, and branch rename. |
| `finish` | `finish <name> [--dry-run]` | Evaluates completion and preserves or reclaims the worktree according to lifecycle gates. |
| `doctor` | `doctor [--dry-run]` | Repairs deinitialized main-checkout submodules and reports stale registrations. |

## Verification Evidence

Measured on 2026-08-08:

- `cargo test -p session-worktree-provision -p session-capture-hook` passed 56 tests across 5 suites. The repository state was identical before and after the run.
- A live run of the installed binary with a brand-new session id and a nonexistent `transcript_path` exited 0 in 61 seconds, created and registered a worktree, and populated all 5 of 5 submodules. The same payload with `hook_event_name: "Stop"` provisioned nothing.
- An earlier live run with a valid transcript and a brand-new session id exited 0 in 56 seconds, created both the worktree and session record, and populated all 5 of 5 submodules. Measured cold provisioning is therefore 56-61 seconds; the earlier 92-second cold run remains historical context. Wall-clock duration and worktree registration do not prove the required persisted assignment.
- A second identical invocation completed in 0.077 seconds, created nothing, and demonstrated reuse for the same session. Cleanup restored the baseline exactly.

## Known Gaps And Follow-Up

On 2026-08-08, two ordering defects in `session-capture-hook` were confirmed fixed: superproject `28b9f05d` / submodule `a02828e8` moved provisioning ahead of capture-store resolution, and superproject `f671a3b0` / submodule `87f2d336` moved provisioning ahead of the transcript-capture guard.

The Rust lifecycle rewrite is complete: `cargo test -p worktree-ctl` passes 28 tests (10 unit, 15 lifecycle-contract, and 3 maintenance tests), superseding the 16 retired shell contracts for dry runs, dirty-checkout acknowledgement and preservation, finish/remove/rename behavior, submodule initialization, explicit override, commit-ahead preservation, and session reuse.

Ticket `5e6cf4f8` owns the completed `worktree-ctl` migration and retirement of the Bash implementation. Related ticket `3d535b2c` (Add prompt-time worktree bootstrap hook) is open; ticket `0f5acbfe` (Session-id worktree routing: discovery resolver, capture-hook self-heal, and terminal stdio isolation) is reviewed.

## Manual Lifecycle Protocol

Automatic provisioning only establishes or reuses the isolated checkout. [branch-worktree.instructions.md](../commit/branch-worktree.instructions.md) owns the manual lifecycle: claim the session and board, work and commit on the feature branch, rebase onto local `main`, mark ready to merge, and leave merging to the root orchestrator.