# agent-tooling/repo-wide-search

## Goal

Specify a repo-root-scoped, token-bounded search surface so agents locate code
across the workspace through one contract with match caps, per-match context
windows, and a counts-first triage mode -- instead of through raw `grep` or
`rg` invocations whose hit sets are unbounded.

## Problem

There is no `*-search*` tool in `memory-api/tools/mcp/` or
`memory-api/tools/cli/` covering repo-wide scans.

- `peek_grep` is deliberately single-file: it takes one path and returns
  matching line numbers within it. It cannot answer "where in this repository is
  this symbol used?"
- Raw `grep -r` and `rg` return every match with no cap. A common identifier can
  return thousands of lines, which is precisely the 50-90% irrelevant-context
  waste the tool suite exists to eliminate.
- `semantic_search` is approximate and non-deterministic, which makes it a poor
  fit for exhaustive reference-finding.

The result is that the most frequent discovery operation in an agent loop is the
one operation with no bounded default.

## Scope

- Repo-root-scoped literal and regex search with an explicit match cap and an
  explicit per-file match cap.
- A counts-only triage mode returning per-file match counts and no content, so
  the agent can narrow the target set before requesting any lines.
- A per-match context window, defaulting to zero or a small window, so the caller
  opts into context rather than receiving it by default.
- Path include and exclude globs, plus honoring workspace ignore rules so build
  artifacts and vendored trees do not dominate results.
- Explicit truncation reporting: when the cap is hit, say so and report the known
  total, so the caller can narrow rather than assume completeness.
- The `*-api` behavior crate plus thin `*-cli` and `*-mcp` transports, sharing
  the regex validation and error model already used by `peek-api` where possible.

## Non-goals

- Replacing `peek_grep` for single-file searching.
- Semantic or embedding-based search; this contract is exact-match search.
- Symbol-aware or type-aware reference finding, which is language-server work.
- Search-and-replace, which belongs to `agent-tooling/file-editing`.

## Acceptance Criteria

1. A `*-api` crate owns traversal, matching, capping, and truncation reporting
   with transport-independent request and response types and one error model.
2. Search is scoped to the repository root by default and accepts a narrower
   subtree scope.
3. A counts-only mode returns per-file match counts with no line content.
4. Content mode is bounded by a total match cap and a per-file match cap, both
   with documented defaults; exceeding either is reported as truncation with the
   known total.
5. Per-match context is opt-in via an explicit window size, and defaults to
   minimal or no surrounding context.
6. Include and exclude globs are supported, and workspace ignore rules are
   honored by default with an explicit opt-out.
7. Invalid regex input produces the same shared, transport-appropriate error
   behavior as `peek-api` regex validation.
8. Thin `*-cli` and `*-mcp` transports delegate to the API crate.
9. The MCP server is registered in `.vscode/mcp.json` and `.github/mcp.json` and
   named in the `tools:` wildcard lists of the `.agents/agents/` templates that
   grant search capability.

## Traceability

- Parent design call: `agent-tooling/default-tool-suite`
- Epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
- Layering and regex-validation reference: `agent-tooling/peek-api`
  (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
- Guidance: `.agents/instructions/orchestration/file-inspection.instructions.md`

## Validation Evidence

Expected before review:

- `cargo test -p <search>-api`
- `cargo test -p <search>-mcp`
- Focused cases: counts-only mode returns no content, total cap truncation with
  reported total, per-file cap truncation, context window opt-in versus default,
  include/exclude glob behavior, ignore-rule honoring and opt-out, invalid regex
  rejection parity with `peek-api`, and scope narrowing to a subtree.
