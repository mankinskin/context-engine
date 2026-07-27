## Problem

Sub-agents are spawned with no shared context, so each rediscovers the same artifacts independently. The orchestrator already holds the digest and does not pass it down.

Cross-agent duplicate file reads measured across both sessions:

| artifact | distinct sub-agents that read it | total reads |
|---|---|---|
| `handoffs/dcf86212-*.json` | 6 (plus 4 more via a second path spelling) | 14 |
| `compact-terminal-mcp/src/server.rs` | 6 | 21 |
| `.vscode/mcp.json` + `.github/mcp.json` | 3 | 10 |
| `.spec/specs/63c60c9d*/body.md` | 3 | 3 |
| `memory-api/crates/session-api/src/model/handoff.rs` | 2 | 3 |
| `memory-api/crates/session-api/src/model/workflow.rs` | 2 | 6 |

Cross-agent duplicate commands:

- `cargo test -p compact-terminal-cli` x5
- `cargo test -p compact-terminal-mcp` x4
- `git status --short` x4
- MCP JSON-RPC `initialize` + `tools/list` handshake probe x4

Within a single agent it is worse: subagent `[9]` read `subagent_rollup.rs` **6 times**, `body.md` 4 times, `lib.rs` 4 times.

Sub-agents also read `.agents/agents/orchestrator.agent.md`, `explore.agent.md`, `default.agent.md`, and `iteration.agent.md` — spending tokens introspecting the delegation system rather than doing the delegated work.

## Why it costs

Reading `server.rs` (454 lines) 21 times across 6 agents is not 21 file reads — it is 21 turns, each carrying an estimated ~37k tokens of fixed prefix plus the file body, plus the reasoning tokens spent deciding to read it. The information was identical every time. Read counts are measured; the per-turn token figure is an estimate pending `9d527ad1`.

Parallel fan-out makes it worse, not better: in `41966513` the two parallel sub-agents `[0] Verify wildcard tool grant` and `[1] Load handoff context` issued byte-identical command sequences and search queries — the same `ticket.exe get bd5e9aee`, `spec.exe get 63c60c9d`, `spec.exe get 3ccdde3a`, `ticket.exe subgraph`, and the same eight `file_search` globs.

## Scope

- Define a context bundle passed to every sub-agent at spawn: resolved ticket/spec bodies, handoff package, relevant file digests, and validation command list — as prompt content, not as paths the child must fetch.
- For parallel fan-out specifically: compute the shared prefix of what siblings need once, in the parent, and inline it into each child prompt.
- Add per-agent read deduplication guidance: within one sub-agent, re-reading a file already in its own transcript is always waste.
- Remove the need for sub-agents to read agent templates by stating the relevant contract in the delegation prompt itself.
- Consider a session-scoped artifact cache keyed by path + content hash, so a repeat read returns a cheap "unchanged, see turn N" marker.

## Acceptance Criteria

1. Sub-agents receive resolved ticket/spec/handoff content inline; they do not fetch it themselves for context they were spawned to act on.
2. Parallel siblings do not independently issue identical discovery command sequences.
3. No single sub-agent reads the same unchanged file more than once.
4. No sub-agent reads `.agents/agents/*.agent.md` in the course of normal delegated work.
5. Measured against the benchmark in `10d21210` — whose scenario includes a fan-out of sibling sub-agents needing the same artifact — the count of artifacts read by more than two distinct sub-agents drops to zero versus the checked-in baseline.

## Evidence

- Duplicate-read and duplicate-command tables produced by `tmp/subagent_cost_probe.py`
- `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`
- `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json` — parallel spans at events 6/7 and 240/242