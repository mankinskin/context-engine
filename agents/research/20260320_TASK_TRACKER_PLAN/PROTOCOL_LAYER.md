# Protocol Layer — Human CLI vs Agent Protocol

## Core Principle

`TaskCommand` is the canonical machine protocol.
The human CLI is a UX adapter on top of the same command contract.

All agent-oriented surfaces send structured `TaskCommand` JSON directly:

- `ticket exec` reads one command or a transactional batch from stdin
- `ticket serve --stdio` speaks persistent JSONL over stdio
- `context-http` accepts the same command payload over HTTP
- `context-mcp` wraps the same command payload for tool calls

The CLI subcommand layer exists for human ergonomics only.

## Split of Responsibilities

### Human Surface

Human operators use CLI subcommands optimized for discoverability and shell use:

- short flags
- prefix matching where safe
- cwd-based index-root inference
- terminal-oriented formatting

Example:

```bash
ticket update a3f2 -s done
```

### Agent Surface

Agents use explicit, self-contained JSON payloads with no shell quoting burden:

- full UUIDs only
- structured patch objects, not `--field k=v` parsing
- explicit `index_root`, no cwd inference
- response field selection to reduce token output
- optional persistent session transport for long-running work

Example:

```json
{
  "command": "task_update",
  "index_root": "/absolute/path/to/ticket-index",
  "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "patch": {
    "state": "done"
  },
  "fields": ["id", "state", "updated_at"]
}
```

## Supported Protocol Modes

### Mode 1: Human CLI

`ticket create`, `ticket get`, `ticket update`, `ticket list`, etc.

Use case:
- humans doing ad-hoc operations
- local debugging
- shell scripting

Properties:
- excellent ergonomics for people
- poor token efficiency for agents
- one process spawn per operation

### Mode 2: `ticket exec`

Stateless stdin JSON command execution.

Examples:

```bash
echo '{"command":"task_create","index_root":"/abs/path","title":"wire CRUD","state":"open"}' | ticket exec
```

```bash
cat <<'JSON' | ticket exec --batch
{"command":"task_search","index_root":"/abs/path","query":"state:open","limit":5}
{"command":"task_claim","index_root":"/abs/path","id":"a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c","intent":"implementing feature"}
JSON
```

Use case:
- agents doing discrete operations
- test harnesses
- non-persistent automation

Properties:
- no shell quoting issues
- lower token cost than CLI flags
- transactional batch support
- still pays process startup cost per invocation

### Mode 3: `ticket serve --stdio`

Persistent JSONL command transport over stdio.

Example session:

```json
{"id":1,"command":"task_search","query":"state:open","limit":1}
{"id":2,"command":"task_claim","id":"a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c","intent":"refining"}
{"id":3,"command":"task_update","id":"a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c","patch":{"state":"in-progress"}}
```

Use case:
- lease-heavy long-running agents
- swarm coordinators
- high-throughput workflows

Properties:
- one process spawn per session
- redb and Tantivy stay open for session lifetime
- request/response correlation by request ID
- server-side lease renewal while connection is alive

## Self-Containment Contract

Agent protocol payloads must be fully explicit.

Required rules:

- include `index_root` on every `ticket exec`, HTTP, and MCP command
- use full UUIDs, never short prefixes
- encode updates as structured objects
- never rely on cwd, environment, or shell escaping for correctness

`ticket serve --stdio` binds `index_root` at startup, then all later requests are scoped to that session.

## Batch Semantics

`ticket exec --batch` is transactional.

- the entire batch succeeds or fails as a unit
- no partial commits to filesystem, redb, Tantivy, or history
- deterministic replay is a design goal
- batch failure returns the failing command index and structured error envelope

This is intentionally stricter than "best effort" scripting because agent-driven automation
must not leave ambiguous partial state behind.

## Lease Renewal Policy

For `ticket serve --stdio`, the server automatically renews leases held by the session
while the connection remains healthy.

Implications:

- agents do not need to emit explicit heartbeat commands in the common persistent case
- `ticket heartbeat` remains available for stateless transports and external adapters
- if the process hangs but the connection stays alive, progress may stall while the lease remains valid
- lease events must record renewal source (`manual_heartbeat` vs `session_liveness`)

## Response Shape Controls

Agent protocol commands support response projection:

```json
{
  "command": "task_get",
  "index_root": "/absolute/path/to/ticket-index",
  "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "fields": ["id", "state", "title"]
}
```

Rules:

- projection is optional
- omitted `fields` means full response envelope
- invalid field selectors return structured validation error
- selectors apply to result payloads, not to error envelopes

## Swarm Instruction Model

Swarm operation uses a dispatch model with independent workers.

- coordinator searches, schedules, and dispatches work
- workers independently call the ticket system to claim, update, and release work
- coordinator does not become the sole writer for all worker progress
- ticket system remains the shared source of truth for scalable swarm state

This avoids coordinator context overload while preserving distributed execution.

## Rollout

### Phase 1

- human CLI subcommands
- `ticket exec` for agent JSON stdin
- shared `TaskCommand` contract for both

### Phase 1.5

- `ticket serve --stdio`
- session-bound lease renewal
- persistent redb/Tantivy handles for lease-heavy workflows

### Phase 5

- HTTP adapter
- MCP adapter
- same `TaskCommand` payload model across all transports

## Non-Goals

- no shell-string command construction requirement for agents
- no CLI flag parsing as the canonical machine interface
- no hidden context inference in machine protocol modes