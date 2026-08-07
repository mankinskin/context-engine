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

## Live verification, 2026-08-07

End-to-end verification in worktree `.worktrees/70abae1b-session-worktree-discovery` on branch `agent/70abae1b-session-worktree-discovery`:

- Discovery resolves a session to its worktree with no routing index, no explicit workspace argument, and no session record present. Confirmed via a ticket read and a board check-in that both landed in the worktree store.
- Resolver suite: 24 passed, 0 failed.

### Defect found and fixed: a hook-written main-pointing record defeated discovery

The capture hook infers the worktree from its own working directory, which is always the main checkout, and writes `worktree.path = <main checkout>, branch = main` on every `UserPromptSubmit`. The rule "a recorded assignment always wins over a discoverable worktree" then treated that inference artifact as authoritative, resolved the session to the main checkout, and the guard refused every call with `main checkout mutations are blocked`. The session was locked out of its own tool surface, self-healing only in the window between deleting the record and the next user prompt.

Fix applied: a receipt whose path refers to the same directory as the main checkout is distrusted and discovery is attempted instead; if nothing is discoverable the receipt is still honored, preserving prior behavior. A receipt pointing at any non-main path keeps winning over discovery, unchanged.

### AC 7 (new, DONE)

A recorded assignment that points at the main checkout does not defeat discovery. Covered by tests `a_main_pointing_record_does_not_defeat_discovery` and `a_main_pointing_record_is_honored_when_nothing_is_discoverable`.

### AC 8 (new, OPEN) — guard scope

When a session resolves to the main checkout, the guard currently refuses every call, including plain reads, which is what turned a mis-routed session into a total lockout. Decision taken: refuse mutations only and let reads through. There is currently NO read/write classification anywhere in `session-workspace-resolver` or `mcp-toolmon` — the proxy blocks unconditionally before any notion of mutation exists — so this requires introducing one. It must be deny-by-default: anything not positively classified as a read is treated as a mutation.

### Open design conflict — explicit workspace override

A decision was taken that an explicit `workspace` argument should override the guard. This conflicts with an existing deliberate invariant: `ResolveRequest::relative_workspace` is documented "It is never a selector", is confined to a path relative to the already-resolved worktree, and an absolute value is rejected via `AbsoluteRelativeWorkspace`. That invariant exists to stop a caller escaping its assigned worktree. Implementing the override as stated would invert it. NOT implemented pending a decision on whether the override should be constrained to worktrees already belonging to the same session rather than an arbitrary path.
## Decisions, 2026-08-07 (supersede earlier answers in this ticket)

**Explicit workspace override: DROPPED.** `ResolveRequest::relative_workspace` stays "never a selector". The session id remains the sole selector for which worktree a call resolves to, and an absolute `workspace` value stays rejected. Rationale: the containment invariant is what stops a session reaching into a sibling agent's worktree, and the deadlock that motivated an escape hatch was fixed by distrusting main-pointing records instead.

**AC 8 guard scope: REVERSED to block everything.** When a session resolves to the main checkout, ALL tool calls are refused, reads included. Rationale: allowing reads would let an agent silently consume state from the wrong checkout, which is a worse failure than a loud refusal. No read/write classification is introduced, and none is needed. AC 8 therefore requires NO code change — the existing unconditional block is the intended behavior and should be documented as deliberate rather than treated as a defect.

**Anchor durability caveat.** The capture hook rewrites the session record on every prompt with `worktree.path = <main checkout>, branch = main`, observed at 12:39, 12:55 and 13:23 on 2026-08-07. Any explicit anchor written by `session check-in` is therefore transient and will be clobbered by the next prompt. Correct routing currently depends on the resolver distrusting main-pointing records, not on the anchor persisting. Making the hook record the real worktree is tracked on ticket 5e6cf4f8-120c-4674-95de-d7b79c99f5b3.
### Finding: a hook-written session record cannot be corrected by `session check-in`

Attempting to record a correct anchor for session `70abae1b-14c4-4033-9265-d37fe08b02b2` failed twice with:

```
session error: session 70abae1b-14c4-4033-9265-d37fe08b02b2 ownership mismatch for worktree check-in
```

This occurred with `--owner-id copilot-agent-70abae1b` and again with `--owner-id copilot-agent`, the latter matching the existing record's `metadata.agent_id` exactly. A `--predecessor-session-id` rotation variation failed identically. Both the main store and the worktree store were left unchanged.

The consequence is that a session record written by the capture hook — which always points at the main checkout — cannot be repaired through the supported CLI path. The ownership guard rejects even the identity that wrote the record. Correct routing therefore rests entirely on the resolver distrusting main-pointing records; there is currently no way to author a correct explicit anchor. The ownership check needs to either accept the recording identity or expose a supported takeover path.