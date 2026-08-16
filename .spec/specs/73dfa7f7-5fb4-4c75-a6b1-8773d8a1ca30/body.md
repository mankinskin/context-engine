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

A human works directly in the normal VS Code integrated terminal. The human
controls all terminal input and explicitly records selected output through the
`session` CLI. A guidance agent reads only bounded, session-scoped output and
status. No agent-facing operation can send input or execute a command in that
terminal.

## Contract

### Ownership

No terminal application or ticket-vscode terminal command is required. A human
opens and uses the normal VS Code integrated terminal. The human explicitly
runs `session terminal-create`, `terminal-append-output`, `terminal-peek`, and
`terminal-close` as needed. The human must record only selected output, never
commands, prompts, keystrokes, or secrets.

### Session record

`session-api` persists each observer under
`.session/sessions/<session-id>/terminals/<terminal-id>/` with a manifest and
append-only output events. The manifest records session id, terminal id, label,
working directory, creation time, and terminal status. Output events record a
monotonic sequence, timestamp, source `terminal-output`, and output data.

### Agent surface

`session-cli` provides the human-operated observer lifecycle. `session-mcp`
exposes observer status and bounded output reads without a terminal-input,
command, shell, cwd-mutation, or execute argument. `peek` returns bounded
events with a cursor and has-more signal. Closing a terminal prevents
additional output append events.

### Lifecycle

An observer session begins open and becomes closed when the human runs
`session terminal-close`. Output after close is rejected. The normal VS Code
terminal remains the only terminal user interface.

## Guards

Before this specification is verified, evidence must show:

1. `session-api` round-trips manifest and output events, preserves event order,
   rejects post-close append, and bounds peek results.
2. `session-mcp` exposes no schema for terminal input or command execution.
3. `session-cli` exposes the human-operated lifecycle without an input or
   execution verb, and a manual normal VS Code terminal trace can be read back.
4. No ticket-vscode terminal application or command is required for observer
   sessions.
5. Relevant Rust tests pass.

## Positions

| Code reference | Status | Required position |
| --- | --- | --- |
| `memory-api/crates/session-api` | implemented | Observer manifest/event persistence and bounded reads. |
| `memory-api/tools/cli/session-cli` | implemented | Human-operated observer lifecycle CLI operations with no input verb. |
| `memory-api/tools/mcp/session-mcp` | implemented | Read-only agent observer tools. |
| `memory-api/tools/ticket-vscode` | excluded | No terminal UI or output capture is required. |
| `compact-terminal-*` | implemented but excluded | Remains one-shot execution and never backs observer input. |

## Non-goals

- A custom terminal application or a ticket-vscode terminal command.
- Agent-side terminal input, `sendText`, or arbitrary shell execution.
- Recording terminal input, secrets, or keystrokes.
- Replacing compact-terminal's one-shot command tooling.

## Governing-rule requirement

The rule-introduces-spec mechanism presents this specification as coming soon
until every guard has passing evidence. Per-template grant rules in
`ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd` must keep observer tools separate from
command-execution grants.

## Traceability

This is a child of the compact-terminal contract `63c60c9d` and related to the
interactive-learning specification `03d93adb-59a8-44be-af95-3b4b208e7e9a`.