# agent-tooling/compact-terminal

## Goal

Specify the token-bounded command-execution surface of the default agent tool
suite: run a shell command and return a bounded result, spilling oversized
output to a transient file that can be inspected with windowed and pattern
reads instead of replaying the command.

## Problem

Raw terminal capture is the single largest uncontrolled token sink in an agent
loop: a failing `cargo test` can emit tens of thousands of lines of which a
handful matter. `compact-terminal-mcp` already solves this, but the behavior was
never specified, and it exists only as an MCP transport -- there is no
`compact-terminal-api` crate and no CLI transport. That diverges from the
`*-api` behavior + thin `*-cli`/`*-mcp` transport layering established by
`agent-tooling/peek-api`, and it means the spill/preview contract cannot be
reused or tested independently of MCP.

## Delivered Implementation

The implementation follows the peek-api layering pattern with three crates:

1. **`memory-api/crates/compact-terminal-api`** -- core execution, inline/spill
   decision, spill file lifecycle, and bounded spill reading, with
   transport-independent types and error model.
2. **`memory-api/tools/cli/compact-terminal-cli`** -- CLI transport exposing
   `run` and `read-spill` commands for MCP-unavailable fallback scenarios.
3. **`memory-api/tools/mcp/compact-terminal-mcp`** -- MCP transport adapter
   delegating to `compact-terminal-api`.

The MCP transport exposes two tools:

`run` -- executes a command via `sh -c` with `command`, optional `cwd`,
`inline_limit` (default 4096 bytes), and `timeout_secs`.

- Output at or under `inline_limit` returns `kind: "inline"` with `exit_code`,
  full `stdout`, `stderr`, and `elapsed_ms`.
- Output over the limit returns `kind: "spilled"` with `exit_code`,
  `stdout_preview`, `stderr_preview`, `total_bytes`, `total_lines`,
  `spill_file`, `elapsed_ms`, and a `next_steps` hint list.

`read_spill` -- reads a bounded window from a spill file produced by `run`,
addressed either by `start`/`end` line range or by `grep` pattern.

### MCP Transport Contract Note

MCP transports in this repository must call `ServerCapabilities::enable_tools()`
in their `get_info()` implementation to advertise tool availability to clients.
Omitting this call makes tools invisible despite wildcard tool grants in agent
configurations. A regression test (`server_advertises_tools_capability` in
`compact-terminal-mcp/tests/integration_tests.rs`) guards this requirement.

## Scope

- The command-execution request/response contract, including the inline vs
  spilled decision and the shape of each variant.
- The spill file lifecycle: creation, addressing by handle, and the guarantee
  that a handle stays readable for the duration of the agent session.
- Bounded re-inspection of a spill by line range and by pattern.
- Exit-code and timeout propagation, and the error model for a missing or
  expired spill handle.
- Extraction of this behavior into a `compact-terminal-api` crate with thin
  `compact-terminal-cli` and `compact-terminal-mcp` transports.

## Non-goals

- Changing the observable inline/spilled response shapes for existing callers.
- Replacing the `rtk` proxy, which performs a complementary filtering role at
  the shell level.
- Interactive or long-lived process management (servers, watchers, REPLs); this
  contract covers one-shot command execution.

## Acceptance Criteria

1. A `compact-terminal-api` crate owns the execution, inline/spill decision, and
   spill-reading behavior, with transport-independent request and response types.
2. `compact-terminal-mcp` delegates to that crate and keeps its current `run`
   and `read_spill` tool names and response shapes.
3. The CLI MUST expose `run` and `read-spill`, with request fields, response fields, exit status, and error code equivalent to the API/MCP operation of the same name.
4. Output at or below `inline_limit` is returned inline; output above it is spilled. When a response is spilled, `spill_file` MUST name an existing readable file whose bytes equal the omitted stdout and stderr payload until the documented cleanup TTL expires.
5. `read_spill` supports both explicit line ranges and pattern matching, and
   returns bounded results in both modes.
6. Non-zero exit codes, timeouts, and missing or expired spill handles produce
   distinct, transport-appropriate responses from one shared error model.
7. The tool remains reachable to delegated agents: the server is registered in
   `.vscode/mcp.json` and `.github/mcp.json` and named in the `tools:` wildcard
   lists of the `.agents/agents/` templates that need command execution.

## Traceability

- Parent design call: `agent-tooling/default-tool-suite` (spec `7c9757a7`)
- Epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
- Implementation ticket: `.ticket/tickets/bd5e9aee-f89b-4d38-be80-80d6c8c1a3b5`
- Layering reference: `agent-tooling/peek-api`
  (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
- Implementation: `memory-api/crates/compact-terminal-api`,
  `memory-api/tools/cli/compact-terminal-cli`,
  `memory-api/tools/mcp/compact-terminal-mcp`
- Guidance: `.agents/instructions/orchestration/tool-output.instructions.md`

## Validation Evidence

Verified 2026-07-27:

- `cargo build` -- warning-free for all three crates
- `cargo test -p compact-terminal-api` -- 8 tests passed
- `cargo test -p compact-terminal-mcp` -- 6 tests passed  
- `cargo test -p compact-terminal-cli` -- 6 tests passed
- `./target/debug/spec.exe health --all` -- 0 issues across 220 specs
- MCP capability check: `tools/list` correctly returns `run` and `read_spill`;
  binary initialize includes `"capabilities":{"tools":{}}`

Coverage includes inline boundary cases (at, just below, just above
`inline_limit`), spill handle round-trip through `read_spill` by range and by
pattern, non-zero exit propagation, timeout behavior, missing-handle error, and
the `enable_tools()` capability requirement.
