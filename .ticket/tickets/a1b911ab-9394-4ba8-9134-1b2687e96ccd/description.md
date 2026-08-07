## Objective

Let an MCP server resolve `session_id` to the worktree that session should write
into, using only the session id plus a one-time filesystem discovery — no
hand-managed routing file.

## Background

`session-workspace-resolver` used to read a machine-local side-car index at
`.session-routing/worktree-index.json`. That index has been deleted; resolution
is now anchored on the process working directory. Because every MCP server is
launched once at the main checkout and never restarted per session, every
session resolved to the main checkout regardless of which worktree it owned.
The plumbing carrying `session_id` through the proxy was already in place end
to end; only the lookup body needed a real implementation.

Hard-failing on a missing assignment was also a deadlock: a session could not
check in until it had resolved, and could not resolve until it had checked in.

## Scope

Resolution only. Worktree lifecycle, recycling, eager creation in the
`UserPromptSubmit` hook, and the rewrite of `worktree.sh` as a Rust binary are
tracked on 5e6cf4f8, which depends on this ticket.

## Acceptance Criteria

1. Given a session whose worktree exists at `.worktrees/<short-id>-<slug>`, a
   `tools/call` carrying that `session_id` resolves to that worktree, with no
   routing index present anywhere on disk.
2. Discovery uses a glob fast path on `.worktrees/<short-id>-*`; exactly one
   match resolves, zero matches falls through to a scan of `.worktrees/*/` for
   `.session/sessions/<session_id>/session.json`.
3. Two or more glob matches fail with a distinct named error. No arbitrary
   choice is ever made between candidates.
4. Successful discovery is cached for the proxy process lifetime; a second call
   for the same session performs no additional filesystem walk. Misses are not
   cached, since the hook may create the worktree immediately afterwards.
5. When nothing is discovered, resolution fails with `MissingSessionWorktree`.
   Resolution never silently falls back to the main checkout.
6. A worktree assignment recorded in the session store always takes precedence
   over a discoverable worktree.

## Non-Goals

- Worktree creation, locking, reclamation, or reuse. See 5e6cf4f8.
- Reaping the pre-existing orphaned `.worktrees/` directories that git no
  longer registers.
- Any change to the merge protocol or the root-orchestrator merge monopoly.
