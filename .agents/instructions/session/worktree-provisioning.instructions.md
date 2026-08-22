---
description: "Use when bootstrapping a session or diagnosing automatic worktree provisioning. Covers the capture-hook triggers, provisioning policy, environment controls, silent-skip guards, troubleshooting, and worktree tooling."
applyTo: "**"
---

## What Fires And When

The capture hook can provision a session worktree before the session's first tool call. VS Code loads [.github/hooks/hooks.json](../../../.github/hooks/hooks.json) through the `.chat.hookFilesLocations` setting in [.vscode/settings.json](../../../.vscode/settings.json). The registered binary is `session-capture-hook`, installed on `PATH` at `~/.cargo/bin/session-capture-hook`.

`SessionStart` is the event eager provisioning is primarily attached to, so a session receives an isolated worktree before its first prompt. If `SessionStart` was missed for a session (e.g. hooks were reconfigured mid-session, or the event never fired), the hook lazily provisions instead on the first later event that carries a session id and isn't `Stop` (`UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `SubagentStart`, `SubagentStop`) — `Stop` intentionally never provisions, so a session that never began capturing does not spring a fresh worktree into existence only at its end. The `UserPromptSubmit` timeout is 300 seconds to allow a cold provision.

The binary usage is:

```text
Usage: session-capture-hook (session sync ingest) [--from-hook-stdin] [--transcript-path <PATH>] [--store-root <PATH>] [--workspace-slug <SLUG>] [--trigger <NAME>]
```

The hook writes `{}` to stdout for both success and early skip, with human diagnostics on stderr and a structured `tracing` trail in the OS temp directory at `session-capture-hook/session-capture-hook.log` (e.g. `$TMPDIR/session-capture-hook/session-capture-hook.log` on Unix, `%TEMP%\session-capture-hook\session-capture-hook.log` on Windows; appended across invocations, never rotated, deliberately outside any `.session` checkout tree so logging is never a session-store mutation). Set `SESSION_HOOK_LOG_DIR` to relocate the log directory and `SESSION_HOOK_LOG` (or `RUST_LOG`) to change the filter level (default `info`); use `debug` to see every parsed field and decision branch.

## Hook Events

| Event | Registered command | Timeout |
|---|---|---:|
| `SessionStart` | `session-capture-hook --from-hook-stdin` | 300s |
| `UserPromptSubmit` | `session-capture-hook --from-hook-stdin` | 300s |
| `PreToolUse` | `bash tools/agent-hooks/rtk-hook-copilot.sh`, then `bash tools/agent-hooks/preflight-write.sh` | 5s, 30s |
| `PostToolUse` | `bash tools/agent-hooks/validate-docs.sh`, `bash tools/agent-hooks/terminal-pwd.sh`, then `session-capture-hook --from-hook-stdin` | 30s, 5s, 120s |
| `Stop` | `session-capture-hook --from-hook-stdin` | 120s |

Each registration in [hooks.json](../../../.github/hooks/hooks.json) invokes the binary directly by bare name (`session-capture-hook --from-hook-stdin`), resolved through `PATH` (installed at `~/.cargo/bin/session-capture-hook`). An earlier revision wrapped this in `bash -c '...'` to add an explicit `CARGO_HOME` PATH fallback, but VS Code's hook command execution on Windows does not go through a POSIX shell, so single-quoted, multi-token `bash -c` scripts get split on whitespace and silently fail to invoke anything — the bare command form is required for the hook to fire at all on Windows. The previous `tools/agent-hooks/capture-hook-stdin.sh` wrapper — which persisted raw stdin to `.session/local/hook-captures/<event>.json` before forwarding it — was removed; its diagnostic role is now served by the binary's own `tracing` file log, and its `SESSION_HOOK_CAPTURE`/`SESSION_HOOK_CAPTURE_DIR` env vars no longer apply.

`SessionEnd` is **not** a Copilot hook event and is no longer registered. The eight events are `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PreCompact`, `SubagentStart`, `SubagentStop`, and `Stop`; `Stop` is the end-of-session event. See the [hooks reference](https://code.visualstudio.com/docs/agents/reference/hooks-reference).

Every event's stdin carries `timestamp`, `hook_event_name`, `session_id`, `cwd`, and `transcript_path`, plus per-event fields: `source` (`SessionStart`), `prompt` (`UserPromptSubmit`), `tool_name`/`tool_input`/`tool_use_id` (`PreToolUse`, and `tool_response` additionally on `PostToolUse`), `trigger` (`PreCompact`), `agent_id`/`agent_type` (`SubagentStart`, plus `stop_hook_active` on `SubagentStop`), and `stop_hook_active` (`Stop`). `hook_event_name` supplies the trigger, and a blank or unknown trigger normalizes to `stop`.

`session-capture-hook` also accepts a `workspace_slug` field. That field is a repository-specific extension used by the crate's own fixtures; Copilot never sends it, so in a live session the slug always falls back to its `default` value.

## Verifying Hook Payloads

Two layers verify that hooks fire and that their payloads still match the documented schema.

Passive: every live session appends structured events to the hook's
`tracing` log file in the OS temp directory (see above). Inspect the current
shape without leaking prompt or tool content:

```bash
tail -n 50 "${SESSION_HOOK_LOG_DIR:-$TMPDIR/session-capture-hook}/session-capture-hook.log"
```

Active: [hook-capture-e2e.sh](../../../tools/agent-hooks/hook-capture-e2e.sh) drives a real headless session with the GitHub Copilot CLI, which reads the same hook schema as VS Code (`copilot help config`, key `hooks`). The script builds a throwaway repository whose hooks record each event, runs `copilot -p` with a prompt that forces a tool call, then asserts the captured payloads against the documented field sets:

```bash
bash tools/agent-hooks/hook-capture-e2e.sh
```

Exit codes are `0` for pass, `1` for schema drift or a missing required event, and `77` for skip. The script skips when `copilot` or `jq` is absent, and when the Copilot CLI is unauthenticated — authentication requires `COPILOT_GITHUB_TOKEN`, `GH_TOKEN`, or `GITHUB_TOKEN` in the environment, or a completed `/login` inside `copilot`.

## Provisioning Decision Flow

The hook crate lives in [memory-api/crates/session-capture-hook](../../../memory-api/crates/session-capture-hook/), with entry point [main.rs](../../../memory-api/crates/session-capture-hook/src/main.rs). The provisioning crate is [memory-api/crates/session-worktree-provision](../../../memory-api/crates/session-worktree-provision/), with policy in [policy.rs](../../../memory-api/crates/session-worktree-provision/src/policy.rs). Both crates compile into the ordinary release binary; provisioning is not feature-gated or test-only.

Eager provisioning runs for `SessionStart` with a non-blank session id, plus a lazy fallback on any other non-`Stop` event when no assignment is resolvable (see "What Fires And When"). A successful provision creates the session worktree and writes a minimal registration record — `session_id`, `metadata.worktree.{path,branch,allocation_mode,status}` — to the **main checkout's own** `.session/sessions/<session-uuid>/session.json` via `SessionStoreConfig::register_provisioned_worktree` (ticket 842d74cb D1: the main checkout is the authoritative session-to-worktree registry). Every `UserPromptSubmit` is also mirrored to that main store's `events.json`, including when the transcript has not flushed yet, so the initial request remains visible with the registration. This is a no-op once a record with a worktree assignment already exists, so it never clobbers a real capture. Worktree discovery is otherwise positional from the supported directory layouts. `WORKTREE_EAGER_PROVISION` is opt-out: the condition treats unset as enabled and only `0` as disabled.

The policy follows reuse, reclaim, then create:

1. **Reuse**: `AlreadyProvisioned` returns when `.worktrees/<full-session-uuid>/` has exactly one immediate slug directory. More than one slug directory for one session UUID is a deterministic ambiguity error with lexicographically ordered candidate paths. Legacy flat `.worktrees/<short-id>-<slug>` worktrees remain discoverable only when their local `.session/sessions/<full-session-uuid>/session.json` record matches the session UUID; nested wins when both layouts are valid.
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

For a valid `UserPromptSubmit`, the hook routes and attempts eager provisioning before transcript capture or capture-store resolution. The transcript guard skips capture when the path is absent, the file is zero-byte, or it contains only lifecycle records and no messages; when a hook event is available, the skip still persists that event. Capture-store resolution then determines whether the hook can persist a session record; a blank `workspace_slug` or an unresolvable store root does not block provisioning.

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

New sessions use the nested layout below. `<full-session-uuid>` is the complete session UUID, and `<slug>` is the selected topic slug:

| Resource | Template |
|---|---|
| Worktree | `<main_checkout>/.worktrees/<full-session-uuid>/session` |
| Branch | `agent/<full-session-uuid>/session` |

The `session` slug is a placeholder for a topic not yet declared. Rename the nested worktree before session check-in. Existing flat `.worktrees/<short-id>-<slug>` worktrees remain supported during transition and are not migrated. Nested layout wins when both layouts match one session UUID; multiple valid nested or legacy candidates fail with a deterministic ambiguity error instead of selecting a worktree. Follow [## 1b. Name the topic (rename the worktree)](../commit/branch-worktree.instructions.md#1b-name-the-topic-rename-the-worktree) before session check-in. [branch-worktree.instructions.md](../commit/branch-worktree.instructions.md) is canonical for manual lifecycle commands and integration order.

## Environment Variables

| Name | Default | Effect |
|---|---|---|
| `WORKTREE_EAGER_PROVISION` | unset = enabled | Set to `0` to disable eager provisioning. |
| `WORKTREE_MAX` | `8` | Maximum registered worktrees before `CapReached`. |
| `WORKTREE_IDLE_SECS` | `86400` | Idle window before reclamation is allowed. |
| `WORKTREE_STALE_SECS` | `14400` | Stale-session window. |
| `MCP_MAIN_CHECKOUT` | current directory | Overrides the anchor checkout; an empty string is ignored. |

## Silent-Skip Guards

Malformed hook input cannot identify a valid event or session and therefore skips provisioning. Provisioning also skips for any event other than `UserPromptSubmit`, a blank `session_id`, an unavailable current directory, an invalid anchor checkout, `WORKTREE_EAGER_PROVISION=0`, or a mismatched explicit external store. `AlreadyProvisioned` requires exactly one nested slug directory for the full session UUID; competing slug directories return the deterministic ambiguity error.

A missing, zero-byte, or lifecycle-only transcript skips only transcript capture, and an unresolvable capture store or blank `workspace_slug` does not block provisioning. Provisioning failures exit non-zero with the detailed error on stderr; success and intentional capture skips emit `{}`.

## Troubleshooting

Because the hook is intentionally quiet, use this checklist to establish whether automatic provisioning fired:

1. **Check positional discovery.** Run `./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon`. The resolved path must be the sole `.worktrees/<uuid>/<slug>` directory. A missing directory returns `MissingSessionWorktree`; multiple slug directories return `AmbiguousSessionWorktree` with sorted candidates. No main-checkout `.session/sessions/<uuid>/session.json` assignment record is read or written.
2. **Check the current checkout.** Run `git rev-parse --show-toplevel`. A repository-root path rather than `.worktrees/<uuid>/<slug>` means automatic provisioning did not place the session in a worktree.
3. **Check registered worktrees.** Run `git worktree list` and find `.worktrees/<full-session-uuid>/<slug>`. Existing flat `<short-id>-<slug>` entries remain valid legacy worktrees.
4. **Check hook registration.** Reload the VS Code window after changes to [.github/hooks/hooks.json](../../../.github/hooks/hooks.json) or [.vscode/settings.json](../../../.vscode/settings.json), because VS Code reads hook registration at window start. A missing worktree-local `.session/sessions/<uuid>/session.json` record may prevent legacy-layout validation, but is not a main-checkout assignment check.
5. **Check the opt-out.** Run `echo "[$WORKTREE_EAGER_PROVISION]"`. A value of `0` disables provisioning.
6. **Check the cap.** Compare the `git worktree list` count to `WORKTREE_MAX`, which defaults to 8.
7. **Use the manual fallback.** Run `./target/debug/worktree-ctl.exe new <full-session-uuid> <slug>`. The full UUID is required; the resulting nested worktree lets positional discovery resolve the session on the next prompt.
8. **Repair a missing submodule worktree path.** If repository-root `git status` reports `fatal: cannot chdir to '../../../../../.worktrees/<name>/memory-api': No such file or directory`, followed by `fatal: 'git status --porcelain=2' failed in submodule memory-api`, a rolled-back or manually deleted worktree left `core.worktree` in the shared submodule config pointing at the missing directory. `git worktree prune` cannot self-heal because it reaches the same error. Run `git config --file .git/modules/<submodule>/config --unset core.worktree`, then `git -C <submodule> worktree prune` and `git worktree prune` at the repository root. Check all five submodules with `git config --file .git/modules/<name>/config --get core.worktree`. Stale `.git/modules/<name>/worktrees/<entry>/gitdir` entries pointing at missing paths are a separate, milder symptom cleared by the same prune.

When hand-constructing a `transcript_path` for a manual hook invocation under Git Bash, use a Windows-style `C:/...` path. The native binary does not resolve POSIX `/tmp/...` paths produced by `mktemp`.

`WORKTREE_IDLE_SECS` defaults to 24 hours. Existing worktrees have been touched recently, so reclaim will not occur in the near term; new sessions pay the full cold-provision cost and count toward `WORKTREE_MAX`.

## Worktree CLI Reference

`./target/debug/worktree-ctl.exe` supports `--dry-run` on every subcommand, and `-h` or `--help` prints usage.

| Subcommand | Signature | Behavior |
|---|---|---|
| `new` | `new <full-session-uuid> <slug> [--dry-run] [--preserve-main-changes]` | Requires a full UUID and creates `.worktrees/<full-session-uuid>/<slug>` with `agent/<full-session-uuid>/<slug>` from local `main`, populates submodules offline, and rolls back on failure. |
| `list` | `list [--dry-run] [--verbose]` | Defaults to a compact color-coded summary with relative paths, lifecycle labels, offending dirty/ahead/behind repositories, and condensed branch state. `--verbose` renders the full tree with superproject and submodule branch, clean/dirty state, and divergence from local `main`; unavailable submodules and unregistered debris remain visible. |
| `rebase` | `rebase <name> [--dry-run]` | Rebases only the superproject worktree branch onto local `main`; stops on conflict. |
| `merge` | `merge <name> [--dry-run]` | Partially automates nested submodule fast-forwards and then the superproject fast-forward; it does not enforce gitlink containment. |
| `remove` | `remove <name> [--force] [--dry-run]` | Refuses dirty worktrees unless forced, then removes the worktree, prunes registrations, and deletes the merged branch. Use `<full-session-uuid>/<slug>` for nested worktrees; the UUID parent is removed only when empty. |
| `clean` | `clean [--dry-run]` | Removes registered worktrees under `.worktrees` only when the superproject and every initialized submodule are clean and not ahead of local `main`; preserves all other worktrees with a reason. |
| `rename` | `rename <source-name> <target-name> [--dry-run]` | Re-topics a clean worktree through filesystem relocation, repair, and branch rename. Use `<full-session-uuid>/<slug>` addresses for nested worktrees; bare names address legacy flat worktrees. |
| `finish` | `finish <name> [--dry-run]` | Evaluates completion and preserves or reclaims the worktree according to lifecycle gates. Use `<full-session-uuid>/<slug>` for nested worktrees. |
| `doctor` | `doctor [--dry-run]` | Repairs deinitialized main-checkout submodules and reports stale registrations. |

## Verification Evidence

Measured on 2026-08-08:

- `cargo test -p session-worktree-provision -p session-capture-hook` passed 56 tests across 5 suites. The repository state was identical before and after the run.
- A live run of the installed binary with a brand-new session id and a nonexistent `transcript_path` exited 0 in 61 seconds, created a worktree, and populated all 5 of 5 submodules. The same payload with `hook_event_name: "Stop"` provisioned nothing.
- An earlier live run with a valid transcript and a brand-new session id exited 0 in 56 seconds, created both the worktree and session record, and populated all 5 of 5 submodules. Measured cold provisioning is therefore 56-61 seconds; the earlier 92-second cold run remains historical context. Positional discovery, rather than a persisted main-checkout assignment, establishes the selected worktree.
- A second identical invocation completed in 0.077 seconds, created nothing, and demonstrated reuse for the same session. Cleanup restored the baseline exactly.

## Known Gaps And Follow-Up

On 2026-08-08, two ordering defects in `session-capture-hook` were confirmed fixed: superproject `28b9f05d` / submodule `a02828e8` moved provisioning ahead of capture-store resolution, and superproject `f671a3b0` / submodule `87f2d336` moved provisioning ahead of the transcript-capture guard.

The Rust lifecycle rewrite is complete: `cargo test -p worktree-ctl` passes 28 tests (10 unit, 15 lifecycle-contract, and 3 maintenance tests), superseding the 16 retired shell contracts for dry runs, dirty-checkout acknowledgement and preservation, finish/remove/rename behavior, submodule initialization, explicit override, commit-ahead preservation, and session reuse.

Ticket `5e6cf4f8` owns the completed `worktree-ctl` migration and retirement of the Bash implementation. Related ticket `3d535b2c` (Add prompt-time worktree bootstrap hook) is open; ticket `0f5acbfe` (Session-id worktree routing: discovery resolver, capture-hook self-heal, and terminal stdio isolation) is reviewed.

## Manual Lifecycle Protocol

Automatic provisioning only establishes or reuses the isolated checkout. [branch-worktree.instructions.md](../commit/branch-worktree.instructions.md#bottom-up-integration-sequence-canonical) owns the manual lifecycle: claim the session and board, work and commit on the feature branch, rebase affected submodules then the superproject onto local `main`, mark ready to merge, and leave bottom-up integration to the root orchestrator.