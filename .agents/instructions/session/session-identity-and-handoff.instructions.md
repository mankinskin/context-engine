---
description: "Use at the start of and at the end of every session, and whenever picking up handed-off work: session identity declaration, worktree binding, transcript-visible traceability, and verified techniques for inspecting prior sessions."
applyTo: "**"
---

## Single Session Identifier

- The Copilot session UUID is the only session identifier. The UUID comes from the Copilot hook payload and keys the on-disk record directory `.session/sessions/<session-uuid>/`.
- Session IDs must be UUIDs. A slug-shaped value is rejected; session identity must always be supplied explicitly, with no marker-file or other fallback.
- Runtime state (`active_run_id`, `runs`, `pinned_entities`, and `workflow`) lives in `.session/sessions/<session-uuid>/session.json`.
- New worktrees are named `.worktrees/<session-uuid>/<slug>` and branches are named `agent/<session-uuid>/<slug>`. Existing flat `.worktrees/<session-short-id>-<slug>` worktrees remain supported during transition and are not migrated. Positional discovery selects nested first; more than one valid slug directory for one UUID is `AmbiguousSessionWorktree`.

## Resolve Your Own Identity First

Obtain the UUID from the Copilot hook payload and supply it explicitly. The primary command initializes or resumes the UUID-owned runtime state:

```bash
./target/debug/session.exe init --session-id <uuid> --workspace . --toon
```

The result contains `context.session_id`, `active_run_id`, and `runs[]`. There is no fallback session-identity resolution.

Resolve an explicitly registered worktree when the task requires one:

```bash
./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon
```

The lookup returns `session_id`, `owner_id`, `ticket_id`, `worktree_path`, `branch`, `allocation_mode`, and `status`. No assignment is normal for a small task: tools run in the main checkout. For worktree-backed work, create the worktree, register it with `session_check_in`, and then use the returned assignment for all tools and commands. A missing or ambiguous assignment blocks only worktree-backed mutations. `git rev-parse --show-toplevel` confirms the current execution checkout.

## Opening Declaration

The first substantive response must state the session and execution checkout. Use one of these templates:

```text
session: <uuid> | worktree: .worktrees/<uuid>/<slug> | branch: agent/<uuid>/<slug>
```

```text
session: <uuid> | checkout: main
```

Resolve every placeholder from the current session; never copy an identifier, worktree, or branch from a previous transcript.

## Runtime Attestation

Before the first task command and after every sub-agent dispatch, compare the
session lookup with the actual execution context:

```text
session_id | code_worktree | git_toplevel | branch | entity_store_root | command_cwd
```

When a session has an assignment, `code_worktree`, `git_toplevel`, and `branch`
must match it. When it has none, they must identify the main checkout. `entity_store_root`
must equal the explicit value in the handoff package; never derive it from the
current directory. A mismatch stops the unit before any further read, write,
build, or validation command.

## Claim Order

For a worktree-backed task, create the worktree with its final topic slug, run `session_check_in` and `board_check_in`, then make the first edit. [branch-worktree.instructions.md](../commit/branch-worktree.instructions.md) owns the commands. A small main-checkout change skips those worktree-specific claims after checking that no active board entry owns the path.

### Check-in registers an explicit worktree

Use `session_check_in` only after creating a worktree for a worktree-backed task. Pass the created `worktree_path` and branch exactly as created. Normal tools may execute in the main checkout while no session worktree is registered; once registered, they must execute in that assignment.

## Closing Traceability Footer

Every agent final response ends with this footer so lineage is greppable from the chat transcript alone, rather than only from the board and session store:

```
session: <uuid> | worktree: .worktrees/<uuid>/<slug> | branch: agent/<uuid>/<slug> | ticket: <short-id> <title>
```

Resolve every placeholder from the current session and its claimed ticket; never copy values from a previous transcript or instruction example.

The footer applies to sub-agents too: a sub-agent's single returned message carries the footer, so a write-and-die Worker's one step remains attributable after the session is gone. See [write-and-die.instructions.md](../orchestration/write-and-die.instructions.md).

## Inspecting a Prior Session

[session-artifacts.instructions.md](../orchestration/session-artifacts.instructions.md) owns the precedence rule: read durable artifacts first (ticket, then spec, then handoff package); use a bounded transcript slice only when durable artifacts are insufficient; never dump a raw transcript.

Which sessions touched a ticket:

```bash
./target/debug/session.exe sessions-for-ticket <ticket-id> --workspace . --toon
```

The command returns `count` and `sessions[]`.

Resolve a session's worktree, branch, and ticket:

```bash
./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon
```

The command returns the positionally discovered session-worktree fields. `MissingSessionWorktree` or `AmbiguousSessionWorktree` means no unambiguous worktree is available; inspect the nested and legacy layouts and repair or create the worktree before continuing.

Inspect transcript shape before reading content:

```bash
./target/debug/session.exe peek-skeleton --session-id <uuid> --preview-chars 40 --toon
```

The command returns `total_turns` and `entries[]{sequence,role,preview,content_len}`. Use the skeleton to choose a range instead of guessing.

Read a bounded transcript window:

```bash
./target/debug/session.exe peek-range --session-id <uuid> --start <n> --end <m> --toon
```

The command returns `turns[]{sequence,role,content}`. Keep the window small and widen only on evidence from the skeleton.

Inspect what sub-agents cost and did:

```bash
./target/debug/session.exe subagent-rollups --session-id <uuid> --toon
```

The command returns per-run `turn_count`, `tool_call_count`, `input_tokens`, `output_tokens`, `cache_read_tokens`, and `cache_write_tokens`.

Read a prior handoff package from disk because no read subcommand exists:

```bash
cat .session/sessions/<uuid>/handoffs/<handoff-id>/handoff.md
```

The structured form is `handoff.json` in the same directory. `session.exe handoff` is write-only and requires `--objective`, `--higher-level-objective`, and at least one `--upward-context` JSON entry. Not every session has a `handoffs/` directory.

### Normalize what you find

Convert every prior-session finding to `scope | finding | outcome | blocker | pointer` before carrying the finding forward, as required by [session-artifacts.instructions.md](../orchestration/session-artifacts.instructions.md).

## Known Defect

`./target/debug/session.exe query --workspace . --limit 5 --toon` aborts the entire listing when one record is unreadable: `session error: session data was not found at .session/sessions/<uuid>/session.json`. Until fixed, prefer `sessions-for-ticket` or a direct `ls .session/sessions/`. Ticket 7be23bd8 tracks the defect as out of scope.