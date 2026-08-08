## Goal

Persistent stores and generated output directories must never appear as a side effect of a tool merely starting up in a workspace. Every MCP server and viewer covered by this contract must leave a fresh, storeless workspace filesystem-identical before and after startup, unless the caller explicitly initializes or configures a store/log sink.

## Authoritative Behavior (approved)

1. `.ticket` and equivalent persistent stores (`.spec`, `.session`, `.feedback`, `.test`, or any store-backed tool) require explicit initialization or configuration; mere process startup, read-only queries, or no-op tool calls against a storeless workspace must never create the store directory.
2. Generated output directories, including `test-logs/` and `target/test-logs/`, are created lazily, only on the first actual configured write. They must never be created as a side effect of process startup, tracing/logging initialization, or a read-only call.
3. Absent an explicit filesystem log destination (no configured log file/dir), filesystem logging remains disabled — no log file or directory is created as a side effect of enabling tracing/logging infrastructure.
4. Any operation that requires a store to exist, invoked against a workspace where that store is missing and not configured, must fail with a clear, non-mutating error. The tool must not silently create the missing store to satisfy the failed operation.
5. Explicit initialization (`init`) and explicit configured first writes are unaffected — they must still create their intended artifact exactly as today.

## Scope

Covers startup-time behavior of every installed MCP server and viewer binary, discovered by direct investigation rather than assumed, including at minimum:

- `memory-api/tools/mcp/ticket-mcp`
- `memory-api/tools/mcp/session-mcp` (and `session-cli`)
- `context-stack/tools/mcp/context-mcp`
- `memory-viewers/ticket-viewer`
- `memory-viewers/log-viewer`
- `memory-viewers/doc-viewer`
- `memory-viewers/spec-viewer`
- any `spec-mcp`, `test-mcp`, `audit-mcp`, `feedback-mcp`, `fs-mcp` binaries present in `memory-api/tools/mcp/` or equivalent locations

The actual installed tool matrix must be enumerated during implementation (do not hardcode a fixed list without verifying it against the repository at implementation time).

## Non-Goals

- Does not ban or restrict intentional explicit initialization (`init`) or explicit configured writes — those must keep creating their artifacts.
- Does not change the contents of a log file/store once filesystem logging or store creation is legitimately configured/enabled.
- Does not fold in or restate pre-dispatch agent-gate requirements from orchestration instructions; that is a separate concern.
- Does not resolve the session-anchored workspace-resolution/store-divergence defect tracked in ticket fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3 (cwd-vs-server-cwd store resolution is a related but distinct concern from artifact-creation timing).

## Acceptance Criteria

1. A reusable fixture produces a fresh, temporary, storeless workspace directory with no `.ticket`/`.spec`/`.session`/`.feedback`/`.test`/log-output artifacts, usable both by focused per-package tests and by a repository-level command.
2. The fixture snapshots the complete filesystem state before startup, spawns each matrix entry with captured exit code/stdout/stderr, snapshots again, and asserts the pre/post snapshots are byte-for-byte identical (exact delta assertion, not absence-of-error).
3. A parameterized startup matrix exists over the actual discovered set of installed MCP servers/viewers (see Scope) — the matrix is not hardcoded to `ticket-mcp` and `log-viewer` only.
4. For any tool whose invoked operation requires a missing store, the operation returns a clear, non-mutating error; the fixture confirms no store directory was created as a side effect of that failed call.
5. A positive-path case exists per relevant tool proving that explicit init or an explicit configured write still creates the intended artifact (store directory, log file/dir), confirming the contract does not regress legitimate lazy-creation behavior.
6. Regression evidence identifies the exact code path/creator previously responsible for eager `test-logs/`/`target/test-logs/` creation on startup — a test that fails again specifically at that creator if reintroduced, not merely a pass/fail assertion at the fixture boundary.
7. Focused package-level tests pass for every affected crate/tool.
8. A repository-level matrix command exists that runs the full startup matrix in one invocation and reports the filesystem-delta result per tool.

## Validation Plan

- Design validation before implementation, per the fixture and assertion shape in Acceptance Criteria 1-2.
- Focused, affected-package test suites per tool discovered in the matrix.
- Repository-level fixture/matrix command (Acceptance Criterion 8) as the single source of truth for the full startup contract.
- Regression test targeting the exact prior `test-logs/`/`target/test-logs/` creator (Acceptance Criterion 6).

## Related

- Ticket 52724aed-7215-471b-b2d8-7fb425f5ed61 — "Startup artifact pollution: MCP servers/viewers must not create stores or log dirs on mere startup in storeless workspaces" (primary tracking ticket for this spec).
- Ticket fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3 — "Session-anchored MCP workspace resolution: require session_id and resolve every proxied call to the session's active worktree" (linked, not a hard dependency; addresses a related but distinct cwd-vs-server-cwd store-resolution concern).
