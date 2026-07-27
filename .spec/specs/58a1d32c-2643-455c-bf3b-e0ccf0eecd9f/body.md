# agent-tooling/filesystem-operations

## Goal

Specify a token-bounded filesystem surface -- list, stat, move, rename, copy,
delete -- so agents inspect and mutate the workspace tree through one contract
with bounded output and explicit conflict detection, instead of through raw
shell commands with unbounded output.

## Problem

There is no `*-filesystem*` tool in `memory-api/tools/mcp/` or
`memory-api/tools/cli/`. Filesystem work is done with raw `ls`, `find`, `mv`,
`cp`, and `rm` through the terminal. This has three costs:

- **Unbounded output.** A recursive listing of a large tree returns thousands of
  entries when the agent needed the first level. `peek --repo-map` gives a
  compact map of the repository but is not a general listing primitive.
- **No shared conflict model.** Whether `mv` overwrote an existing file, or `rm`
  removed something that was already gone, is inferred from exit codes and
  stderr text that differ per platform.
- **Destructive operations without a preflight.** The repository already treats
  moves as needing a preflight/apply/rollback cycle in other stores (see the
  `*_move_preflight` / `*_move_apply` / `*_move_rollback` tool families), but
  plain workspace-file moves have no equivalent.

## Scope

- Bounded directory listing with an explicit depth cap and an explicit entry
  cap, reporting truncation rather than silently omitting entries.
- Metadata-only `stat` (existence, kind, size, modified time) that never returns
  file content.
- Path filtering on listing: include and exclude globs, and honoring ignore
  rules so vendored and build directories do not dominate results.
- Mutating operations -- move, rename, copy, delete -- with explicit
  destination-conflict detection and a defined overwrite opt-in.
- Bounded responses for mutating operations: what changed and what conflicted,
  not a full tree dump.
- The `*-api` behavior crate plus thin `*-cli` and `*-mcp` transports.

## Non-goals

- File content reading, which belongs to `agent-tooling/peek-api`.
- File content editing, which belongs to `agent-tooling/file-editing`.
- Repo-wide content search, which belongs to `agent-tooling/repo-wide-search`.
- Entity-store moves (ticket, spec, rule, session), which already have their own
  preflight/apply/rollback tool families.
- Version control operations.

## Acceptance Criteria

1. A `*-api` crate owns listing, stat, and mutation behavior with
   transport-independent request and response types and one error model.
2. Listing is bounded by default with explicit depth and entry caps, and a
   truncated result is flagged as truncated with the total count where known.
3. Listing supports include and exclude path filters and honors workspace ignore
   rules so build and vendor directories can be excluded.
4. `stat` returns metadata only and never returns file content.
5. Move, rename, copy, and delete detect destination conflicts and fail rather
   than overwrite unless overwrite is explicitly requested.
6. Mutating responses are bounded and enumerate only the affected paths and
   conflicts.
7. Thin `*-cli` and `*-mcp` transports delegate to the API crate without
   reimplementing traversal or validation.
8. The MCP server is registered in `.vscode/mcp.json` and `.github/mcp.json` and
   named in the `tools:` wildcard lists of the relevant `.agents/agents/`
   templates.

## Traceability

- Parent design call: `agent-tooling/default-tool-suite`
- Epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
- Layering reference: `agent-tooling/peek-api`
  (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
- Guidance: `.agents/instructions/orchestration/file-inspection.instructions.md`

## Validation Evidence

Expected before review:

- `cargo test -p <fs>-api`
- `cargo test -p <fs>-mcp`
- Focused cases: depth-capped and entry-capped listing with truncation flag,
  include/exclude filter behavior, ignore-rule honoring, stat on missing path,
  move onto an existing destination without and with overwrite, delete of a
  missing path, and response size bound on a large directory.
