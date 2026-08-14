# Sub-Agent Run Capture

`SubagentStart` and `SubagentStop` are captured through
`tools/agent-hooks/capture-hook-stdin.sh`. Each raw payload is retained at
`.session/local/hook-captures/<event>.json`, and the capture hook persists the
lifecycle event in the parent session's `events.json`. The persisted event is
keyed by the parent session id and `agent_id`; it retains `agent_type`, the
hook timestamp, and the stop state.

## Diagnose A Dispatch

Use repository tooling only; no transcript JSONL inspection is required.

```bash
session subagent-rollups --session-id <session-id> --toon
session delegation-cost --session-id <session-id> --toon
```

`subagent-rollups` reports one lifecycle row for each captured `agent_id` with
its `agent_type`, dispatch and stop timestamps, outcome, token counts, and
turn counts. `delegation-cost` reports each `runSubagent` transcript span and
its `subagents[].failures` entries, which identify failed tools and the
captured reason. A `stopped` lifecycle outcome means the sub-agent stopped; it
does not claim that the delegated work passed. Use the failure entries to
determine whether a stopped dispatch failed and why.

The hook payloads are sufficient for lifecycle state, but not for child turn
or tool-call reconstruction. The existing parent transcript capture supplies
span-level turn, token, and failure data. Ingesting arbitrary VS Code child
transcript JSONL files would require an explicit `agent_id` to transcript
provenance link, so the repository does not treat those external files as an
authoritative source.

## Missing b9020ba2 Record

Session `b9020ba2-df5d-426a-b1b9-228ef159cad1` should have a record at
`.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl/.session/sessions/b9020ba2-df5d-426a-b1b9-228ef159cad1/session.json`.
That record is absent. The actual Git worktree entry is the nested
`.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl/.git`.
The current session-bootstrap validator instead requires a `.git` entry at the
direct parent `.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/.git`, rejects
the nested assignment, and therefore never writes the expected session record.