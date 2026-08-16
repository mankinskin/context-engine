<!-- aligned-structure:v2 -->

# Human-owned observer terminal

## Motivation

Guidance agents need evidence from terminal work without receiving the power to
send shell input or execute commands. Existing `compact-terminal` tools are
single-shot execution surfaces and therefore cannot enforce human-only input.

Related tickets:

- [0dd23fe6 Audit terminal reuse and input continuation](.ticket/tickets/0dd23fe6-6892-4d21-9927-4a81584dc77a/ticket.toml)
- [ea52bd6f Add human-owned observer terminal sessions](.ticket/tickets/ea52bd6f-aa48-43f5-9228-0bff7190abf8/ticket.toml)

## Dependent expectation

If this specification is implemented, a human can work in a VS Code terminal
while a guidance agent reads only bounded, session-scoped output and status.
No agent-facing operation can send input or execute a command in that terminal.

## Contract

### Ownership

The ticket-vscode extension creates the terminal only from an explicit human
command. The human owns all terminal input. The extension captures terminal
process output through VS Code's terminal-data API, never captures or persists
terminal input, and appends output to a session-scoped observer record.

### Session record

`session-api` persists each observer under
`.session/sessions/<session-id>/terminals/<terminal-id>/` with a manifest and
append-only output events. The manifest records session id, terminal id, label,
working directory, creation time, and terminal status. Output events record a
monotonic sequence, timestamp, source `terminal-output`, and output data.

### Agent surface

`session-cli` and `session-mcp` expose create, status, bounded peek, and close
operations. The agent surface has no terminal-input, command, shell, cwd
mutation, or execute argument. `peek` returns bounded events with a cursor and
has-more signal. Closing a terminal prevents additional output append events.

### Lifecycle

An observer session begins open, becomes closed through explicit human UI action
or terminal close, and can be marked error when extension capture fails. Output
after close is rejected. A UI host without the terminal-data API must report
observer capture unavailable instead of silently claiming observation.

## Guards

Before this specification is verified, evidence must show:

1. `session-api` round-trips manifest and output events, preserves event order,
   rejects post-close append, and bounds peek results.
2. `session-mcp` exposes only create, status, peek, and close terminal tools;
   no schema contains terminal input or command execution.
3. `session-cli` exposes equivalent observer operations without an input or
   execution verb.
4. ticket-vscode creates a terminal only on a human command, captures output
   through the terminal-data listener, and never calls `sendText`.
5. Extension unit tests and relevant Rust tests pass, followed by a manual VS
   Code terminal trace readback in the assigned worktree.

## Positions

| Code reference | Status | Required position |
| --- | --- | --- |
| `memory-api/crates/session-api` | not-implemented | Observer manifest/event persistence and bounded reads. |
| `memory-api/tools/cli/session-cli` | not-implemented | Human-observer lifecycle CLI operations with no input verb. |
| `memory-api/tools/mcp/session-mcp` | not-implemented | Read-only agent observer tools. |
| `memory-api/tools/ticket-vscode` | not-implemented | Human command, integrated terminal, and terminal-output capture. |
| `compact-terminal-*` | implemented but excluded | Remains one-shot execution and never backs observer input. |

## Non-goals

- Agent-side terminal input, `sendText`, or arbitrary shell execution.
- Recording terminal input, secrets, or keystrokes.
- Replacing compact-terminal's one-shot command tooling.
- Web-host fallback that simulates terminal capture without the VS Code API.

## Governing-rule requirement

The rule-introduces-spec mechanism presents this specification as coming soon
until every guard has passing evidence. Per-template grant rules in
`ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd` must keep observer tools separate from
command-execution grants.

## Traceability

This is a child of the compact-terminal contract `63c60c9d` and related to the
interactive-learning specification `03d93adb-59a8-44be-af95-3b4b208e7e9a`.
