# agent-tooling/file-editing

## Goal

Specify a token-bounded, context-anchored file editing surface so agents --
especially delegated sub-agents -- patch files differentially instead of
rewriting them, and so edit failures are reported as explicit precondition
violations rather than silently applied to the wrong location.

## Problem

There is no `*-edit*` tool in `memory-api/tools/mcp/` or
`memory-api/tools/cli/`. Agents rely on host-provided built-in edit tools whose
contract this repository does not own, or fall back to writing whole files.

Both fallbacks are expensive and unsafe:

- A full-file rewrite costs tokens proportional to file size for a change that
  may be three lines, and it silently discards concurrent edits by other agents.
- Line-number-anchored edits break as soon as an earlier edit shifts the file.
- Without a defined ambiguity failure mode, a non-unique anchor can be applied
  to the wrong occurrence with no signal to the caller.

The repository's own guidance already mandates differential patching (see
`.agents/instructions/orchestration/differential-patching.instructions.md`), but
there is no owned tool that enforces it.

## Scope

- A context-anchored replacement operation: locate the target by surrounding
  context text rather than by line number, and replace it.
- Uniqueness as a precondition: an anchor that matches zero or more than one
  location fails with a diagnostic naming the match count and candidate
  locations, and applies nothing.
- Multi-edit batching: several independent replacements applied in one call,
  with defined ordering and defined all-or-nothing versus per-edit semantics.
- Insert and delete expressed through the same anchored contract.
- A bounded response: report what changed (path, anchor, applied/failed) without
  echoing the whole file back.
- The `*-api` behavior crate plus thin `*-cli` and `*-mcp` transports.

## Non-goals

- Replacing the VS Code built-in editing tools for interactive human use.
- Semantic or AST-aware refactoring; that is symbol-rename territory and is
  served by language-server tooling.
- File creation and deletion as filesystem entities, which belong to
  `agent-tooling/filesystem-operations`.
- Version control operations.

## Acceptance Criteria

1. A `*-api` crate owns the anchoring, matching, and patch-application behavior
   with transport-independent request and response types and one error model.
2. Replacement is anchored by surrounding context text, not by line number, so
   an edit stays valid when unrelated parts of the file shift.
3. An anchor matching zero locations, or more than one location, fails the
   operation and returns the match count and candidate locations; no partial or
   guessed application occurs.
4. A single call can apply multiple independent edits, across one or more files,
   with documented ordering and failure-isolation semantics.
5. Responses are bounded: they identify what was changed and any failures, and
   never return full file contents as confirmation.
6. Thin `*-cli` and `*-mcp` transports delegate to the API crate without
   reimplementing matching or validation.
7. The MCP server is registered in `.vscode/mcp.json` and `.github/mcp.json` and
   named in the `tools:` wildcard lists of the `.agents/agents/` templates that
   grant edit capability.

## Traceability

- Parent design call: `agent-tooling/default-tool-suite`
- Epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
- Layering reference: `agent-tooling/peek-api`
  (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
- Guidance:
  `.agents/instructions/orchestration/differential-patching.instructions.md`

## Validation Evidence

Expected before review:

- `cargo test -p <edit>-api`
- `cargo test -p <edit>-mcp`
- Focused cases: unique-anchor success, zero-match rejection, multi-match
  rejection with candidate reporting, multi-edit batch across two files,
  edit whose anchor spans a previously edited region, and response size bound.
