# Implement a context-anchored differential editing tool suite

**Crate name**: `edit-api` (api crate); `edit-cli` (CLI transport); `edit-mcp` (MCP transport).

**Implementation note**: This is a **net-new implementation**, not an extraction. The precedent ticket `bd5e9aee` extracted an already-existing `compact-terminal-mcp` into layered api/cli/mcp crates. There is no existing editing abstraction in the workspace today. What transfers from that precedent is **only** the three-crate layout and workspace wiring pattern — not any logic. Sizing must reflect net-new implementation cost.

Parent epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
Spec: `agent-tooling/file-editing` (`4f5ad264-8e8d-4681-9551-4ec14b73c3b1`)

## Problem

No `*-edit*` tool exists under `memory-api/tools/mcp/` or
`memory-api/tools/cli/`. Agents either use host-provided built-in edit tools
whose contract this repository does not own, or rewrite whole files. Full-file
rewrites cost tokens proportional to file size and silently discard concurrent
edits; line-anchored edits break when earlier edits shift the file; and without
a defined ambiguity failure mode a non-unique anchor can be applied to the wrong
occurrence.

## Work

1. Create the `*-api` behavior crate owning anchoring, matching, and patch
   application, with transport-independent request/response types and one error
   model.
2. Implement context-anchored replacement: locate by surrounding context text,
   not line number.
3. Enforce uniqueness as a precondition: zero or multiple matches fail with the
   match count and candidate locations, applying nothing.
4. Support multi-edit batching across one or more files with documented ordering
   and failure-isolation semantics.
5. Keep responses bounded: report changed paths, anchors, and failures; never
   echo full file contents.
6. Add thin `*-cli` and `*-mcp` transports.
7. Register the MCP server in `.vscode/mcp.json` and `.github/mcp.json`, and add
   it to the `tools:` wildcard lists of `.agents/agents/` templates that grant
   edit capability.

## Acceptance

Mirrors the acceptance criteria of spec `agent-tooling/file-editing`.

## Validation

- `cargo test -p <edit>-api`
- `cargo test -p <edit>-mcp`
- Focused cases: unique-anchor success, zero-match rejection, multi-match
  rejection with candidate reporting, multi-edit batch across two files, edit
  anchored across a previously edited region, and response size bound.
