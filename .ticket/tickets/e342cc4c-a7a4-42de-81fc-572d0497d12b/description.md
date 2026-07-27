# Epic: Token-Optimized Default Agent Tool Suite (peek + compact-terminal + design call)

## Design-Call Outcome (2026-07-27)

The design call is complete. All four remaining acceptance criteria are now
satisfied; see "Acceptance Criteria" below for per-criterion status.

**Design doc.** Authored as spec `agent-tooling/default-tool-suite`
(`7c9757a7-739f-4dfe-a4de-26f187f3b5aa`, state `draft`). It defines the shared
contract binding all five categories (behavior in an `*-api` crate; bounded by
default with a named opt-out; coordinates before content; spill-and-peek for
oversized payloads; TOON over JSON; registered *and* wildcard-referenced), then
specifies each of the five categories and the transport layering for the three
gaps. The four category specs below are children of it.

**Specs created** (all state `draft`, component `agent-tooling`):

| Category | Spec | Id |
|---|---|---|
| design call (parent) | `agent-tooling/default-tool-suite` | `7c9757a7-739f-4dfe-a4de-26f187f3b5aa` |
| execute | `agent-tooling/compact-terminal` | `63c60c9d-adbe-4ddb-8c1d-6156610d0753` |
| edit | `agent-tooling/file-editing` | `4f5ad264-8e8d-4681-9551-4ec14b73c3b1` |
| filesystem | `agent-tooling/filesystem-operations` | `58a1d32c-2643-455c-bf3b-e0ccf0eecd9f` |
| search | `agent-tooling/repo-wide-search` | `af9ebba9-6de4-4290-ab4a-319c432ded4c` |

The read category is already covered by `agent-tooling/peek-api`
(`3ccdde3a-368c-4655-a6c8-20a58822c83d`, `approved`).

**Child tickets created**, each linked to this epic by a `depends_on` edge:

- `bd5e9aee-f89b-4d38-be80-80d6c8c1a3b5` — compact-terminal: extract
  `compact-terminal-api` and add CLI transport
- `b8ce7cd8-50d0-4233-8584-3af2a27c07d1` — file editing: context-anchored
  differential patching tool suite (api + cli + mcp)
- `244c3113-e28f-44d7-b9a8-f5dd45d2895c` — filesystem operations: bounded
  list/stat/move tool suite (api + cli + mcp)
- `bd71ecc7-4631-407c-a156-d1d77de2ca33` — repo-wide search: bounded
  counts-first search tool suite (api + cli + mcp)

### MCP naming verification (criterion 5) — gap found and fixed

Verified suite server names across `.vscode/mcp.json`, `.github/mcp.json`, and
the `tools:` wildcard lists in `.agents/agents/*.agent.md`.

- Names are consistent across both mcp.json files: `peek-mcp` and
  `compact-terminal-mcp` are registered in each, both behind `mcp-cost-gate`.
- No silently-empty wildcard exists: every `'<server>-mcp/*'` entry in the agent
  templates resolves to a registered server.
- **Gap found:** the inverse failure. `compact-terminal-mcp` was registered in
  both mcp.json files but referenced by **zero** agent templates, so the suite's
  execute tool was unreachable for every delegated sub-agent — exactly the
  failure mode criterion 5 exists to catch.
- **Fixed:** added `'compact-terminal-mcp/*'` to the `tools:` list of all 14
  agent templates that grant `execute` capability (audit, commit, default,
  explore, handoff, implement, interview, research, review, roast, spec,
  testing, ticket-refinement, transcription). `iteration` and `orchestrator`
  were left alone: neither grants `execute`.
- Also added `'peek-mcp/*'` to `ticket-refinement`, which had `execute` and
  `read` but no bounded-read tool.

This makes "registered in mcp.json **and** named in an agent-template wildcard"
an explicit requirement in the design-call spec and in each category spec, so
future suite tools cannot repeat the omission.

### Guidance updates (criterion 6)

- `.agents/instructions/orchestration/file-inspection.instructions.md` — added a
  "Default Agent Tool Suite" section naming the read category (`peek-mcp` first,
  `peek-cli` fallback, `peek-api` as behavior owner) as the default path for all
  agents including delegated sub-agents; added a "Known Suite Gaps" section for
  the not-yet-built filesystem and repo-wide-search categories.
- `.agents/instructions/orchestration/tool-output.instructions.md` — added a
  "Default Agent Tool Suite" section naming the execute category
  (`compact-terminal-mcp` first, `rtk` as shell fallback) as the default path,
  and promoted the compact-terminal pattern from "when available" to the default
  for long-running commands now that the wildcard wiring exists.

---

## Review Outcome (2026-07-27)

**Verdict: PASS.** Reviewer independently re-verified the inventory (peek-api/peek-cli/peek-mcp, compact-terminal-mcp) and all three gap claims (file editing/differential patching, filesystem operations, repo-wide bounded search) against the actual `memory-api/tools/mcp/` and `memory-api/tools/cli/` trees — all confirmed accurate. The embedded design-call deliverable was judged well-scoped and sufficient; the epic is accepted as-is rather than split into child specs/tickets in this pass.

Resolutions recorded:
- `peek-api` spec (`3ccdde3a-368c-4655-a6c8-20a58822c83d`) promoted `draft` → `reviewed` → `approved`, since it documents fully implemented, tested behavior. Its draft status referenced in Acceptance Criterion 2 is now resolved.
- A dedicated `compact-terminal` spec and the per-gap-category specs/child tickets were deliberately **deferred** to the epic's own design-call phase (do not front-run child-ticket work from a review pass). That design-call phase has now run; see "Design-Call Outcome" above.
- A formal ticket-graph edge from this epic to the `peek-api` spec was **not created**: ticket-mcp `add_edge` links tickets to tickets, not specs, and no ticket<->spec edge tool exists in the current MCP surface. The textual reference in this description (spec id + path) remains the traceability link until/unless a cross-store link mechanism exists; note this as a follow-up if spec<->ticket edges are added to the tool surface later.
- Epic state transitioned `new` → `ready` (its only next-state options were `ready`/`cancelled`; `ready` reflects "accepted, actionable via its design-call path").

---

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

### Reading files — `peek` family (implemented; spec approved)
- Behavior crate: `memory-api/crates/peek-api` — owns bounded file inspection
  (count, grep, head, tail, explicit ranges) and skeleton/structural rendering.
- CLI transport: `memory-api/tools/cli/peek-cli` (`peek`) — "coordinates first,
  content second"; `--all` is an explicit, visible opt-out.
- MCP transport: `memory-api/tools/mcp/peek-mcp` — named tools `peek_read`,
  `peek_grep`, `peek_count`, `peek_skeleton`.
- Spec: `agent-tooling/peek-api` (`.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d`)
  — component `agent-tooling`, state `approved`. Establishes the
  transport-layering contract (api owns behavior; cli/mcp stay thin).

### Executing terminal commands — `compact-terminal-mcp` (implemented; spec now drafted)
- `memory-api/tools/mcp/compact-terminal-mcp` — `run` truncates long output and
  spills full streams to a transient file, returning a bounded preview plus
  `read_spill` (windowed / grep) inspection.
- Spec: `agent-tooling/compact-terminal` (`63c60c9d-adbe-4ddb-8c1d-6156610d0753`),
  state `draft`. Records the layering debt: MCP-only, no `*-api` crate, no CLI.

### Gap — no token-optimized MCP tool exists for:
1. **File editing / differential patching** — an MCP surface for context-anchored
   edits (replace-with-context, multi-edit) so sub-agents patch instead of
   rewriting whole files. Spec `agent-tooling/file-editing`
   (`4f5ad264-8e8d-4681-9551-4ec14b73c3b1`), ticket
   `b8ce7cd8-50d0-4233-8584-3af2a27c07d1`.
2. **Filesystem operations** — bounded list / move / rename / stat as MCP tools
   sharing one contract. Spec `agent-tooling/filesystem-operations`
   (`58a1d32c-2643-455c-bf3b-e0ccf0eecd9f`), ticket
   `244c3113-e28f-44d7-b9a8-f5dd45d2895c`.
3. **Search** — a first-class token-bounded search/grep MCP surface across files
   (peek-grep is per-file only; repo-wide bounded search has no dedicated tool).
   Spec `agent-tooling/repo-wide-search`
   (`af9ebba9-6de4-4290-ab4a-319c432ded4c`), ticket
   `bd71ecc7-4631-407c-a156-d1d77de2ca33`.

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
   the transport layering for each, reusing the `peek-api` pattern. — **done**,
   spec `7c9757a7-739f-4dfe-a4de-26f187f3b5aa`.
2. A spec per new capability category (edit, filesystem, search) under the
   `agent-tooling` component, plus a new spec covering `compact-terminal`. —
   **done**, four specs at `draft`.
3. Child implementation tickets for the identified gaps. — **done**, four
   tickets linked by `depends_on`.
4. Registration + agent-template wiring so the full suite is available to
   delegated sub-agents. — **done for the implemented categories**;
   the three gap categories wire themselves up as part of their own tickets.

## Acceptance Criteria

1. ✅ A design doc / design call exists covering all five default-tool categories
   (read, execute, edit, filesystem, search) with token-bounded contracts and
   transport layering, and is linked from this epic.
   → spec `agent-tooling/default-tool-suite` (`7c9757a7-739f-4dfe-a4de-26f187f3b5aa`).
2. ✅ `compact-terminal` has a spec under the `agent-tooling` component.
   → `agent-tooling/compact-terminal` (`63c60c9d-adbe-4ddb-8c1d-6156610d0753`), `draft`.
   `peek-api` spec is linked by textual reference and is `approved`.
3. ✅ Specs exist (state at least draft) for the three gap categories: file
   editing / differential patching, filesystem operations, and repo-wide search.
   → `4f5ad264`, `58a1d32c`, `af9ebba9`, all `draft`.
4. ✅ Child implementation tickets exist for each gap category and are linked to
   this epic. → `bd5e9aee`, `b8ce7cd8`, `244c3113`, `bd71ecc7`, each with a
   `depends_on` edge from this epic.
5. ✅ The suite's MCP tool naming is verified against registered server names in
   `.vscode/mcp.json` and `.github/mcp.json` so workspace agent-template
   wildcards actually resolve to the tools (no silently-empty wildcard).
   → verified; found and fixed the inverse gap (`compact-terminal-mcp`
   registered but referenced by zero templates). See "MCP naming verification".
6. ✅ Guidance in the orchestration instruction set
   (`.agents/instructions/orchestration/file-inspection.instructions.md` and
   `.agents/instructions/orchestration/tool-output.instructions.md`) references
   the suite as the default path for delegated agents.
   → both files updated with a "Default Agent Tool Suite" section.

## References

- `.spec/specs/7c9757a7-739f-4dfe-a4de-26f187f3b5aa` — design call: default agent tool suite (`draft`).
- `.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d` — peek-api spec (`approved`).
- `.spec/specs/63c60c9d-adbe-4ddb-8c1d-6156610d0753` — compact-terminal spec (`draft`).
- `.spec/specs/4f5ad264-8e8d-4681-9551-4ec14b73c3b1` — file-editing spec (`draft`).
- `.spec/specs/58a1d32c-2643-455c-bf3b-e0ccf0eecd9f` — filesystem-operations spec (`draft`).
- `.spec/specs/af9ebba9-6de4-4290-ab4a-319c432ded4c` — repo-wide-search spec (`draft`).
- `memory-api/crates/peek-api`, `memory-api/tools/cli/peek-cli`, `memory-api/tools/mcp/peek-mcp`.
- `memory-api/tools/mcp/compact-terminal-mcp`.
- `.agents/instructions/orchestration/` — token-efficiency guidance (file-inspection, tool-output, orchestrator-delegation).

## Follow-up Tooling Gap

There is still no ticket↔spec edge type: `add_edge` links tickets to tickets
only. All spec/ticket traceability in this epic is therefore textual (ids and
store paths recorded in both directions). If a cross-store link mechanism is
added to the MCP surface, these references should be promoted to real edges.
This is recorded in the design-call spec under "Known Tooling Gap".
