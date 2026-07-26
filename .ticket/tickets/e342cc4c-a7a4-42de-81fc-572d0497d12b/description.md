# Epic: Token-Optimized Default Agent Tool Suite (peek + compact-terminal + design call)

## Goal

Establish a coherent suite of **token-optimized default agent MCP tools** — for
reading files, executing terminal commands, editing files, filesystem
operations, and search — that every agent (including **delegated sub-agents**)
can rely on to keep token consumption bounded. The suite must be exposed as
MCP tools so it is reachable through the workspace `.agents/agents/*.agent.md`
templates (via MCP wildcards) rather than only through raw, unbounded built-in
tools that spend 50-90% of the token budget on irrelevant context.

## Why

Delegated agents inherit none of the orchestrator's context and each spawn is a
full agent loop. If those sub-agents fall back to unbounded file reads and raw
terminal capture, the token savings from delegation are lost. A shared,
bounded-by-default tool suite makes efficient inspection the path of least
resistance for orchestrator and sub-agents alike, and keeps the cost gate and
MCP integration intact.

## Existing Planning and Implementation (inventory)

The suite is partially built. This epic consolidates the scattered work and
identifies the gaps.

### Reading files — `peek` family (implemented; spec in draft)
- Behavior crate: `memory-api/crates/peek-api` — owns bounded file inspection
  (count, grep, head, tail, explicit ranges) and skeleton/structural rendering.
- CLI transport: `memory-api/tools/cli/peek-cli` (`peek`) — "coordinates first,
  content second"; `--all` is an explicit, visible opt-out.
- MCP transport: `memory-api/tools/mcp/peek-mcp` — named tools `peek_read`,
  `peek_grep`, `peek_count`, `peek_skeleton`.
- Spec: `agent-tooling/peek-api` (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
  — component `agent-tooling`, state `draft`. Establishes the transport-layering
  contract (api owns behavior; cli/mcp stay thin).

### Executing terminal commands — `compact-terminal-mcp` (implemented; no spec)
- `memory-api/tools/mcp/compact-terminal-mcp` — `run` truncates long output and
  spills full streams to a transient file, returning a bounded preview plus
  `read_spill` (windowed / grep) inspection. No dedicated spec or epic yet.

### Gap — no token-optimized MCP tool exists for:
1. **File editing / differential patching** — an MCP surface for context-anchored
   edits (replace-with-context, multi-edit) so sub-agents patch instead of
   rewriting whole files.
2. **Filesystem operations** — bounded list / move / rename / stat as MCP tools
   sharing one contract.
3. **Search** — a first-class token-bounded search/grep MCP surface across files
   (peek-grep is per-file only; repo-wide bounded search has no dedicated tool).

## Scope

- Bring `compact-terminal` and the `peek` family under one "default agent tool
  suite" umbrella with shared token-bounded conventions (compact-by-default,
  bounded windows, spill-and-peek, TOON over JSON).
- Author a **design call** (design doc + specs) for the missing categories:
  file editing / differential patching, filesystem operations, and repo-wide
  search — following the established `*-api` behavior + thin `*-cli`/`*-mcp`
  transport layering already used by `peek`.
- Ensure every suite tool is registered so it is reachable through the workspace
  agent templates' MCP wildcards and the `mcp-cost-gate` boundary, so delegated
  agents keep full access.

## Non-goals

- Reimplementing existing `peek` or `compact-terminal` behavior; this epic
  organizes and extends, it does not rewrite working transports.
- Changing user-facing semantics of existing `peek-cli` modes unless a
  separately tracked spec/bug requires it.
- Replacing the VS Code built-in editing/terminal tools for interactive human
  use; the suite targets agent-to-agent token efficiency.

## Deliverables

1. A design document (design call) enumerating the five default-tool categories
   (read, execute, edit, filesystem, search), their token-bounded contracts, and
   the transport layering for each, reusing the `peek-api` pattern.
2. A spec per new capability category (edit, filesystem, search) under the
   `agent-tooling` component, plus promotion of the `peek-api` spec out of draft
   and a new spec covering `compact-terminal`.
3. Child implementation tickets for the identified gaps.
4. Registration + agent-template wiring so the full suite is available to
   delegated sub-agents behind the cost gate.

## Acceptance Criteria

1. A design doc / design call exists covering all five default-tool categories
   (read, execute, edit, filesystem, search) with token-bounded contracts and
   transport layering, and is linked from this epic.
2. `compact-terminal` has a spec under the `agent-tooling` component; the
   `peek-api` spec is linked to this epic and its draft status is resolved.
3. Specs exist (state at least draft) for the three gap categories: file
   editing / differential patching, filesystem operations, and repo-wide search.
4. Child implementation tickets exist for each gap category and are linked to
   this epic.
5. The suite's MCP tool naming is verified against registered server names in
   `.vscode/mcp.json` and `.github/mcp.json` so workspace agent-template
   wildcards actually resolve to the tools (no silently-empty wildcard).
6. Guidance in the orchestration instruction set
   (`.agents/instructions/orchestration/file-inspection.instructions.md` and
   `.agents/instructions/orchestration/tool-output.instructions.md`) references
   the suite as the default path for delegated agents. (The former single
   `token-efficiency.instructions.md` was split into the orchestration/session
   instruction files.)

## References

- `.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d` — peek-api spec (agent-tooling).
- `memory-api/crates/peek-api`, `memory-api/tools/cli/peek-cli`, `memory-api/tools/mcp/peek-mcp`.
- `memory-api/tools/mcp/compact-terminal-mcp`.
- `.agents/instructions/orchestration/` — token-efficiency guidance (file-inspection, tool-output, orchestrator-delegation).
