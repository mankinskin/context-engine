## Review Outcome (2026-07-27)

**Verdict: PASS.** Reviewer independently re-verified the inventory (peek-api/peek-cli/peek-mcp, compact-terminal-mcp) and all three gap claims (file editing/differential patching, filesystem operations, repo-wide bounded search) against the actual `memory-api/tools/mcp/` and `memory-api/tools/cli/` trees — all confirmed accurate. The embedded design-call deliverable was judged well-scoped and sufficient; the epic is accepted as-is rather than split into child specs/tickets in this pass.

Resolutions recorded:
- `peek-api` spec (`3ccdde3a-368c-4655-a6c8-20a58822c83d`) promoted `draft` → `reviewed` → `approved`, since it documents fully implemented, tested behavior. Its draft status referenced in Acceptance Criterion 2 is now resolved.
- A dedicated `compact-terminal` spec and the per-gap-category specs/child tickets are deliberately **deferred** to the epic's own design-call phase (do not front-run child-ticket work from a review pass).
- A formal ticket-graph edge from this epic to the `peek-api` spec was **not created**: ticket-mcp `add_edge` links tickets to tickets, not specs, and no ticket<->spec edge tool exists in the current MCP surface. The textual reference in this description (spec id + path) remains the traceability link until/unless a cross-store link mechanism exists; note this as a follow-up if spec<->ticket edges are added to the tool surface later.
- Epic state transitioned `new` → `ready` (its only next-state options were `ready`/`cancelled`; `ready` reflects "accepted, actionable via its design-call path").

---

## Epic: Token-Optimized Default Agent Tool Suite (peek + compact-terminal + design call)

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

### Reading files — `peek` family (implemented; spec now approved)
- Behavior crate: `memory-api/crates/peek-api` — owns bounded file inspection
  (count, grep, head, tail, explicit ranges) and skeleton/structural rendering.
- CLI transport: `memory-api/tools/cli/peek-cli` (`peek`) — "coordinates first,
  content second"; `--all` is an explicit, visible opt-out.
- MCP transport: `memory-api/tools/mcp/peek-mcp` — named tools `peek_read`,
  `peek_grep`, `peek_count`, `peek_skeleton`.
- Spec: `agent-tooling/peek-api` (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
  — component `agent-tooling`, state `approved` (promoted from `draft` during
  review). Establishes the transport-layering contract (api owns behavior;
  cli/mcp stay thin).

### Executing terminal commands — `compact-terminal-mcp` (implemented; no spec)
- `memory-api/tools/mcp/compact-terminal-mcp` — `run` truncates long output and
  spills full streams to a transient file, returning a bounded preview plus
  `read_spill` (windowed / grep) inspection. No dedicated spec yet; deferred to
  this epic's design-call phase.

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
   `agent-tooling` component, plus a new spec covering `compact-terminal`.
   (`peek-api` spec is now `approved`.)
3. Child implementation tickets for the identified gaps.
4. Registration + agent-template wiring so the full suite is available to
   delegated sub-agents behind the cost gate.

## Acceptance Criteria

1. A design doc / design call exists covering all five default-tool categories
   (read, execute, edit, filesystem, search) with token-bounded contracts and
   transport layering, and is linked from this epic.
2. `compact-terminal` has a spec under the `agent-tooling` component. (`peek-api`
   spec is linked by textual reference in this description and is `approved`.)
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

- `.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d` — peek-api spec (agent-tooling), state `approved`.
- `memory-api/crates/peek-api`, `memory-api/tools/cli/peek-cli`, `memory-api/tools/mcp/peek-mcp`.
- `memory-api/tools/mcp/compact-terminal-mcp`.
- `.agents/instructions/orchestration/` — token-efficiency guidance (file-inspection, tool-output, orchestrator-delegation).
