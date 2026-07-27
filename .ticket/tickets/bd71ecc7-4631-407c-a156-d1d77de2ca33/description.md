# Implement a repo-wide bounded search tool suite

**Crate name**: `search-api` (api crate); `search-cli` (CLI transport); `search-mcp` (MCP transport).

**Implementation note**: This is a **net-new implementation**, not an extraction. The precedent ticket `bd5e9aee` extracted an already-existing `compact-terminal-mcp` into layered api/cli/mcp crates. There is no existing repo-wide search abstraction in the workspace today. What transfers from that precedent is **only** the three-crate layout and workspace wiring pattern — not any logic. Sizing must reflect net-new implementation cost.

**Boundary with `peek-api`**: `search-api` is repo-wide/counts-first; `peek-api` remains single-file. The overlap is accepted, and this boundary should be documented in both tools' guidance.

**Dependency on filesystem operations**: This ticket depends on [244c3113 (filesystem operations)](../244c3113-e28f-44d7-b9a8-f5dd45d2895c/ticket.toml) for traversal and ignore-rule handling. Filesystem must be implemented first.

Parent epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
Spec: `agent-tooling/repo-wide-search` (`af9ebba9-6de4-4290-ab4a-319c432ded4c`)

## Problem

No `*-search*` tool exists under `memory-api/tools/mcp/` or
`memory-api/tools/cli/` covering repo-wide scans. `peek_grep` is single-file by
design. Raw `grep -r` / `rg` return uncapped hit sets, and `semantic_search` is
approximate. The most frequent discovery operation in an agent loop is the one
operation with no bounded default.

## Work

1. Create the `*-api` behavior crate owning traversal, matching, capping, and
   truncation reporting, reusing the `peek-api` regex validation and error model
   where possible.
2. Repo-root-scoped search by default, with an optional narrower subtree scope.
3. A counts-only triage mode returning per-file match counts and no content.
4. Content mode bounded by a total match cap and a per-file match cap, with
   documented defaults and explicit truncation reporting including known totals.
5. Opt-in per-match context window, defaulting to minimal or none.
6. Include/exclude globs and workspace ignore-rule honoring with explicit opt-out.
7. Add thin `*-cli` and `*-mcp` transports.
8. Register the MCP server in `.vscode/mcp.json` and `.github/mcp.json`, and add
   it to the `tools:` wildcard lists of `.agents/agents/` templates that grant
   search capability.

## Acceptance

Mirrors the acceptance criteria of spec `agent-tooling/repo-wide-search`.

## Validation

- `cargo test -p <search>-api`
- `cargo test -p <search>-mcp`
- Focused cases: counts-only returns no content, total-cap and per-file-cap
  truncation with reported totals, context window opt-in vs default,
  include/exclude glob behavior, ignore-rule honoring and opt-out, invalid regex
  rejection parity with `peek-api`, and subtree scope narrowing.
