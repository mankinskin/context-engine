# Implement a bounded filesystem operations tool suite

Parent epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
Spec: `agent-tooling/filesystem-operations` (`58a1d32c-2643-455c-bf3b-e0ccf0eecd9f`)

## Problem

No `*-filesystem*` tool exists under `memory-api/tools/mcp/` or
`memory-api/tools/cli/`. Listing, stat, move, copy, and delete are done through
raw shell, which returns unbounded output, has no shared conflict model, and
offers no preflight for destructive operations -- unlike the entity stores,
which already have `*_move_preflight` / `*_move_apply` / `*_move_rollback`
families.

## Work

1. Create the `*-api` behavior crate owning listing, stat, and mutation, with
   transport-independent request/response types and one error model.
2. Bounded listing with explicit depth and entry caps plus explicit truncation
   reporting.
3. Include/exclude glob filters and workspace ignore-rule honoring.
4. Metadata-only `stat` that never returns content.
5. Move, rename, copy, and delete with destination-conflict detection and an
   explicit overwrite opt-in.
6. Bounded mutation responses listing only affected paths and conflicts.
7. Add thin `*-cli` and `*-mcp` transports.
8. Register the MCP server in `.vscode/mcp.json` and `.github/mcp.json`, and add
   it to the relevant `.agents/agents/` template `tools:` wildcard lists.

## Acceptance

Mirrors the acceptance criteria of spec `agent-tooling/filesystem-operations`.

## Validation

- `cargo test -p <fs>-api`
- `cargo test -p <fs>-mcp`
- Focused cases: depth/entry-capped listing with truncation flag, include and
  exclude filters, ignore-rule honoring, stat on missing path, move onto an
  existing destination with and without overwrite, delete of a missing path, and
  response size bound on a large directory.
