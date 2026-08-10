---
description: "Use at the start of and at the end of every session, and whenever picking up handed-off work: session identity declaration, worktree binding, transcript-visible traceability, and verified techniques for inspecting prior sessions."
applyTo: "**"
---

## Two Identifiers, Never Conflated

- `workspace_session_id` is a slug-plus-hex form such as `epic-kickoff-8fdfe135`. It lives in `.session/local/active_workspace_session.json`, whose only keys are `workspace_session_id` and `updated_at`.
- The session UUID is a value such as `16263c13-7f29-4780-ba09-bf94190cb87f`. The session UUID keys the on-disk record directory `.session/sessions/<uuid>/`.
- Several subcommands accept only the UUID form despite the flag name `--workspace-session-id`. `./target/debug/session.exe subagent-rollups --workspace-session-id epic-kickoff-8fdfe135 --toon` fails with `session data was not found`; the same command with the UUID succeeds.
- The worktree short-id is the first eight hex characters of the session UUID. Worktrees are therefore named `.worktrees/16263c13-session`, never after the slug form.

## Resolve Your Own Identity First

Primary command, which returns both identifiers at once:

```bash
./target/debug/session.exe init --workspace . --toon
```

The result contains `context.session_id`, `context.workspace_session_id`, `active_run_id`, and `runs[]`.

Fallback marker-file command, which returns the slug form only:

```bash
jq -r '.workspace_session_id' .session/local/active_workspace_session.json
```

Resolve the authoritative worktree binding for any session:

```bash
./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon
```

The lookup returns `session_id`, `owner_id`, `ticket_id`, `worktree_path`, `branch`, `allocation_mode`, and `status`. `git rev-parse --show-toplevel` is a hint, not an answer: a session commonly runs from the repository root while the provisioned worktree is elsewhere.

## Opening Declaration

The first substantive response must begin with the session declaration before any other content. Use this exact template:

```text
session: <uuid> (<workspace_session_id>) | worktree: .worktrees/<short-id>-<slug> | branch: agent/<short-id>-<slug>
```

For example:

```text
session: 16263c13-7f29-4780-ba09-bf94190cb87f (epic-kickoff-8fdfe135) | worktree: .worktrees/16263c13-session | branch: agent/16263c13-session
```

## Claim Order

Checklist: bootstrap worktree, rename to the topic slug, `session_check_in`, `board_check_in`, then make the first edit. The rename must precede `session_check_in`, or the stored path is stranded. [branch-worktree.instructions.md](../commit/branch-worktree.instructions.md) is the canonical owner of the commands. [worktree-provisioning.instructions.md](worktree-provisioning.instructions.md) explains how the hook provisions the `<short-id>-session` placeholder.

### Check-in targets the worktree, not the main checkout

Mutations resolved against the main checkout are refused by `require_mutation_target` in `memory-api/crates/session-workspace-resolver/src/lib.rs` with `main checkout mutations are blocked`. The guard inspects the resolved workspace, not the `worktree_path` argument. Passing a correct `worktree_path` while leaving `workspace` defaulted to the repository root still fails. Pass the worktree path as the `workspace` selector on both `session_check_in` and `board_check_in`. There is no environment-variable bypass.

## Closing Traceability Footer

Every agent final response ends with this footer so lineage is greppable from the chat transcript alone, rather than only from the board and session store:

```
session: <uuid> (<workspace_session_id>) | worktree: .worktrees/<short-id>-<slug> | branch: agent/<short-id>-<slug> | ticket: <short-id> <title>
```

For example:

```
session: 16263c13-7f29-4780-ba09-bf94190cb87f (epic-kickoff-8fdfe135) | worktree: .worktrees/8fdfe135-session-traceability-guidance | branch: agent/8fdfe135-session-traceability-guidance | ticket: 7be23bd8 Agent session identity, worktree traceability, and prior-session inspection protocol
```

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

The command returns the session assignment fields.

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
./target/debug/session.exe subagent-rollups --workspace-session-id <uuid> --toon
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