# agent-tooling/default-tool-suite

Design call for epic `e342cc4c-a7a4-42de-81fc-572d0497d12b`.

## Goal

Define one coherent suite of token-bounded default agent tools covering five
categories -- read, execute, edit, filesystem, search -- so that every agent,
including delegated sub-agents that inherit no orchestrator context, has a
compact-by-default path for each category reachable through the workspace
agent-template MCP wildcards.

## Problem

Two of the five categories are implemented (`peek` for read, `compact-terminal`
for execute). The other three have no token-bounded MCP surface, so sub-agents
fall back to unbounded built-in `edit`, filesystem, and `search` tools. Because
each delegated spawn is a full agent loop with no inherited context, unbounded
fallbacks cancel out the token savings that motivated the delegation.

## Shared Contract (all five categories)

Every suite tool must satisfy the following, regardless of category:

1. **Behavior lives in an `*-api` crate.** `*-cli` and `*-mcp` stay thin
   transports over one request/response model and one error model. This is the
   layering already established by `agent-tooling/peek-api`.
2. **Bounded by default, unbounded by explicit opt-out.** The default response
   is a bounded window or a summary. Any full/unbounded mode is named so the
   cost is visible in command history (the `peek --all` precedent).
3. **Coordinates before content.** Responses lead with addressing information
   (line numbers, paths, counts, match offsets) so a follow-up call can request
   exactly the needed slice instead of re-reading everything.
4. **Spill-and-peek for oversized payloads.** When output exceeds the inline
   budget, persist the full payload to a transient file and return a preview
   plus a handle; provide a windowed/grep reader over that handle. This is the
   `compact-terminal-mcp` `run` / `read_spill` precedent.
5. **Compact encoding.** Prefer TOON over JSON for machine-readable output where
   the transport supports it.
6. **Registered and reachable.** Each MCP server is registered in both
   `.vscode/mcp.json` and `.github/mcp.json` behind `mcp-cost-gate`, AND named
   in the `tools:` wildcard list of the agent templates under `.agents/agents/`.
   Registration without a template wildcard leaves the tool unreachable for
   delegated agents.

## The Five Categories

### 1. Read -- `peek` (implemented)

- `memory-api/crates/peek-api`, `memory-api/tools/cli/peek-cli`,
  `memory-api/tools/mcp/peek-mcp`.
- Tools: `peek_read`, `peek_grep`, `peek_count`, `peek_skeleton`.
- Spec: `agent-tooling/peek-api` (`3ccdde3a-368c-4655-a6c8-20a58822c83d`, approved).
- Status: contract satisfied. Serves as the reference implementation for the
  `*-api` layering rule.

### 2. Execute -- `compact-terminal` (implemented, spec gap)

- `memory-api/tools/mcp/compact-terminal-mcp`.
- Tools: `run`, `read_spill`.
- Gap: MCP-only. No `compact-terminal-api` crate and no CLI transport, so the
  spill/preview behavior is not reusable outside MCP and diverges from the
  layering rule. No spec existed before this design call.
- Follow-on spec: `agent-tooling/compact-terminal`.

### 3. Edit -- differential patching (gap)

- No MCP surface exists. Sub-agents either rewrite whole files or rely on the
  built-in `edit` tool, whose contract is not owned by this repository.
- Required contract: context-anchored replacement (match on surrounding context
  rather than line numbers), multi-edit batching in a single call, and a
  precondition failure mode that reports ambiguity rather than guessing.
- Follow-on spec: `agent-tooling/file-editing`.

### 4. Filesystem -- bounded operations (gap)

- No MCP surface exists. Directory listing, stat, move, rename, and delete are
  done through raw shell, which returns unbounded output and has no shared
  conflict/error model.
- Required contract: bounded listing with depth and entry caps, stat without
  content, and mutating operations with explicit conflict detection.
- Follow-on spec: `agent-tooling/filesystem-operations`.

### 5. Search -- repo-wide bounded search (gap)

- `peek_grep` is single-file only. Repo-wide search falls back to raw `grep`,
  `rg`, or `semantic_search`, all of which can return unbounded hit sets.
- Required contract: repo-root-scoped search with a match cap, per-match context
  window, path include/exclude filters, and a "counts only" mode for triage
  before any content is returned.
- Follow-on spec: `agent-tooling/repo-wide-search`.

## Transport Layering Decision

All three gap categories follow the `peek` pattern:

```
<name>-api   (crate: behavior, validation, error model)
  |- <name>-cli   (thin CLI transport; fallback when MCP is unavailable)
  `- <name>-mcp   (thin MCP transport; named tools, one per operation)
```

`compact-terminal` is retrofitted to the same shape as a follow-on, not as a
prerequisite: its current MCP-only implementation stays behaviorally stable
while the spec records the layering debt.

## Non-goals

- Reimplementing `peek` or `compact-terminal` behavior.
- Changing user-facing semantics of existing `peek-cli` modes.
- Replacing VS Code built-in editing and terminal tools for interactive human use.

## Acceptance Criteria

1. Each of the five categories has a spec under the `agent-tooling` component at
   state `draft` or higher.
2. Each gap category (edit, filesystem, search) has a child implementation
   ticket linked to epic `e342cc4c` by a `depends_on` edge.
3. Every suite MCP server name in `.vscode/mcp.json` and `.github/mcp.json`
   matches the wildcard entries in `.agents/agents/*.agent.md`, with no
   registered-but-unreferenced server and no wildcard that resolves to nothing.
4. `.agents/instructions/orchestration/file-inspection.instructions.md` and
   `.agents/instructions/orchestration/tool-output.instructions.md` name the
   suite as the default path for delegated agents.

## Agent Template Tool Registration

### Per-Template Scoped Tool Grants

Each agent template (`.agents/agents/*.agent.md`) declares its accessible tool set via MCP wildcard patterns in its `tools:` list. Tool availability is scoped per template: delegated sub-agents inherit only the tools their own template names, not the orchestrator's tools.

**tool_search availability:** The `tool_search` tool is itself a deferred tool that must be granted explicitly via wildcard or name if sub-agents are expected to discover and load additional deferred tools at runtime. If `tool_search` is absent from a template's grant list, that agent cannot load deferred tools, limiting it to pre-expanded tools only.

**Contract:** Tool grants are enforced at dispatch time. A sub-agent spawned from template T sees only the union of tools matching T's wildcard list. Lazy tool discovery (via `tool_search`) is available only when `tool_search` itself is granted.

**Related ticket:** [cd19fed4 Scope MCP tool grants per agent template](.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0/ticket.toml)

## Traceability

- Epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
- Epic: [79c4ac3e Delegation cost](.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml)
- Tickets: [cd19fed4 Scope MCP tool grants per agent template](.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0/ticket.toml)
- Child specs: `agent-tooling/compact-terminal`, `agent-tooling/file-editing`,
  `agent-tooling/filesystem-operations`, `agent-tooling/repo-wide-search`
- Reference spec: `agent-tooling/peek-api`
  (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)

## Known Tooling Gap

There is no ticket-to-spec edge type in the current MCP surface; `add_edge`
links tickets to tickets only. Spec/ticket traceability in this design call is
therefore textual (ids and store paths recorded in both directions). If a
cross-store link mechanism is added, these references should be promoted to
real edges.
