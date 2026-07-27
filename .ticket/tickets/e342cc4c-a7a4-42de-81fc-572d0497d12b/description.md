# Epic: Token-Optimized Default Agent Tool Suite (peek + compact-terminal + design call)

## Implementation Status & Sequencing (2026-07-27)

**Implementation note (all three child tickets)**: These are **net-new implementations**, not extractions. The precedent ticket `bd5e9aee` extracted an already-existing `compact-terminal-mcp` into layered api/cli/mcp crates. There is no existing filesystem, editing, or repo-wide search abstraction in the workspace today. What transfers from that precedent is **only** the three-crate layout and workspace wiring pattern — not any logic. Sizing must reflect net-new implementation cost, so the phrase "the extraction pattern is proven and reusable" must not be used to justify optimistic estimates.

**Implementation progress**:
- [244c3113 (filesystem operations)](../244c3113-e28f-44d7-b9a8-f5dd45d2895c/ticket.toml) — **in-review** (moved 2026-07-27). Implementation complete, passed independent audit (GO verdict after three NO-GO rounds), 56 tests passing. Awaiting human review.
- [bd71ecc7 (repo-wide search)](../bd71ecc7-4631-407c-a156-d1d77de2ca33/ticket.toml) — blocked on 244c3113 review.

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
  list/stat/move tool suite (api + cli + mcp) — **in-review**
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

Establish a coherent suite of **token-optimized default agent MCP tools** — for reading, executing, editing, filesystem, and repo-wide search — so delegated agents and the primary agent both share one bounded, cost-aware interaction surface with the workspace, rather than every agent assembling its own mix of unbounded raw shell commands and selective MCP tool access.

These tools form the default agent toolkit once complete. All support TOON output (via `rtk` prefix in CLI, via MCP `CompactToolResult` wrapper in the MCP server) and all report truncation/spilling explicitly (see `.agents/instructions/orchestration/tool-output.instructions.md` and `.agents/instructions/orchestration/compact-output.instructions.md`).

---

## Problem

The repository already has strong implementations of certain categories:
- **Read (file inspection):** `peek-api` + `peek-cli` + `peek-mcp` — complete, tested, bounded reads with skeleton, grep, count.
- **Execute (shell/terminal):** `compact-terminal-mcp` with output spilling — MCP server exists; CLI transport was missing but has now been added via ticket `bd5e9aee`.

But three categories are missing entirely from the MCP and CLI tool tree:
1. **Edit (file mutations):** No differential patching tool. Edits are performed via raw `sed` or multi-step `read → rewrite → replace` through the terminal or the unbounded `editFile` vscode tool, with no shared conflict model or bounded preview.
2. **Filesystem (workspace tree operations):** No listing/stat/move/copy/delete tool. Tree operations are raw shell `ls`/`find`/`mv`/`cp`/`rm` with unbounded output and no structured conflict detection.
3. **Search (repo-wide content search):** No counts-first or bounded-result search tool. Search is either a vscode-only ripgrep call or raw `grep -r`, neither of which reports hit-count-before-results or offers a structured way to stop at N hits.

This means:
- Delegated sub-agents cannot access edit, filesystem, or search functionality in a bounded, cost-aware way; they fall back to unbounded shell commands.
- The primary agent has inconsistent tool availability: bounded for read/execute, unbounded for edit/filesystem/search.
- No shared contract exists across categories for what "bounded" means, what "spill-and-peek" looks like, or how conflict detection surfaces errors.

---

## Scope

This epic encompasses a design call plus three implementation tickets:

1. **Design call (this epic's embedded deliverable):**
   - Define the shared contract for all five tool categories: behavior in an `*-api` crate; bounded by default with a named opt-out; coordinates before content (count/size/metadata first); spill-and-peek for oversized payloads; TOON over JSON; registered in mcp.json **and** named in agent-template wildcard lists.
   - Specify each of the three missing categories (edit, filesystem, search).
   - Document the transport layering: `*-api` owns behavior, `*-cli` and `*-mcp` are thin adapters.
   - Author the design doc as a spec (`agent-tooling/default-tool-suite`), plus per-category child specs (`agent-tooling/file-editing`, `agent-tooling/filesystem-operations`, `agent-tooling/repo-wide-search`).
   - Create child implementation tickets for each of the three gaps.

2. **Three child implementation tickets** (to be created during design call):
   - File editing (differential patching): api + cli + mcp.
   - Filesystem operations (list/stat/move/copy/delete): api + cli + mcp.
   - Repo-wide search (counts-first bounded search): api + cli + mcp.

Each child ticket must:
- Build the `*-api` crate with transport-independent request/response types and one error model.
- Add thin `*-cli` and `*-mcp` transports that delegate to the api crate.
- Register the MCP server in `.vscode/mcp.json` and `.github/mcp.json` and add its wildcard `'<server>-mcp/*'` entry to relevant agent templates.
- Pass focused unit/integration tests that validate bounded behavior, conflict detection, and TOON output.

---

## Non-goals

- Changing the existing `peek-api`, `peek-cli`, or `peek-mcp` implementations (read category is complete).
- Changing the existing `compact-terminal-mcp` implementation (execute category is complete except CLI transport, which is ticket `bd5e9aee`).
- Implementing entity-store-specific tools (ticket, spec, rule, session stores have their own MCP servers).
- Version control operations (git commands stay in the terminal or vscode's git integration).
- IDE-specific integrations (language servers, debuggers, test runners).

---

## Acceptance Criteria

1. **Inventory verification (already satisfied):**
   - [x] Verified: `peek-api`, `peek-cli`, `peek-mcp` exist and cover the read category.
   - [x] Verified: `compact-terminal-mcp` exists and covers the execute category.
   - [x] Gap confirmed: no file-editing tool exists (`agent-tooling/differential-patching`).
   - [x] Gap confirmed: no filesystem tool exists (`agent-tooling/filesystem-operations`).
   - [x] Gap confirmed: no repo-wide-search tool exists (`agent-tooling/bounded-search`).

2. **Design doc authored (already satisfied):**
   - [x] Spec created: `agent-tooling/default-tool-suite` (`7c9757a7-739f-4dfe-a4de-26f187f3b5aa`, state `draft`). Defines the shared contract and specifies each of the five categories.
   - [x] Child specs created for the three gaps:
     - `agent-tooling/file-editing` (`4f5ad264-8e8d-4681-9551-4ec14b73c3b1`)
     - `agent-tooling/filesystem-operations` (`58a1d32c-2643-455c-bf3b-e0ccf0eecd9f`)
     - `agent-tooling/repo-wide-search` (`af9ebba9-6de4-4290-ab4a-319c432ded4c`)
   - [x] All specs reference this epic as parent (`e342cc4c`).
   - [x] Existing `peek-api` spec (`3ccdde3a-368c-4655-a6c8-20a58822c83d`) promoted from `draft` to `approved` to reflect its fully-implemented status.

3. **Child tickets created (already satisfied):**
   - [x] Ticket created: `b8ce7cd8-50d0-4233-8584-3af2a27c07d1` — file editing: context-anchored differential patching tool suite (api + cli + mcp).
   - [x] Ticket created: `244c3113-e28f-44d7-b9a8-f5dd45d2895c` — filesystem operations: bounded list/stat/move tool suite (api + cli + mcp) — **in-review**.
   - [x] Ticket created: `bd71ecc7-4631-407c-a156-d1d77de2ca33` — repo-wide search: bounded counts-first search tool suite (api + cli + mcp).
   - [x] All child tickets reference this epic in their description.
   - [x] All child tickets have `depends_on` edges pointing to this epic.

4. **Layering and contract consistency verified (already satisfied):**
   - [x] Each category spec (`file-editing`, `filesystem-operations`, `repo-wide-search`) states that the `*-api` crate owns behavior with transport-independent types and one error model, and that `*-cli` and `*-mcp` are thin adapters.
   - [x] Each category spec requires bounded behavior by default with a named opt-out for unbounded cases.
   - [x] Each category spec requires coordinates-before-content (counts/sizes/metadata first).
   - [x] Each category spec requires spill-and-peek for oversized payloads.
   - [x] Each category spec requires TOON output support (`--toon` flag in CLI, `CompactToolResult` in MCP).
   - [x] Each category spec requires registration in `.vscode/mcp.json` and `.github/mcp.json` **and** naming in agent-template wildcard lists.

5. **MCP naming verification (already satisfied):**
   - [x] Verified: `peek-mcp` and `compact-terminal-mcp` are registered in both `.vscode/mcp.json` and `.github/mcp.json`.
   - [x] Verified: `peek-mcp` and `compact-terminal-mcp` wildcards appear in relevant agent templates (default, explore, implement, research, etc.).
   - [x] Gap found and fixed: `compact-terminal-mcp` was registered but not referenced by any agent templates. Added `'compact-terminal-mcp/*'` to all 14 templates that grant `execute` capability.
   - [x] Gap found and fixed: `ticket-refinement` had `execute` and `read` but no `'peek-mcp/*'` wildcard. Added.

6. **Guidance updates (already satisfied):**
   - [x] `.agents/instructions/orchestration/file-inspection.instructions.md` updated with a "Default Agent Tool Suite" section naming the read category (`peek-mcp` first, `peek-cli` fallback) and a "Known Suite Gaps" section for the not-yet-built filesystem and search categories.
   - [x] `.agents/instructions/orchestration/tool-output.instructions.md` updated with a "Default Agent Tool Suite" section naming the execute category (`compact-terminal-mcp` first, `rtk` fallback) and promoting compact-terminal to the default for long-running commands.

---

## Validation

Design call complete:
- Specs created and linked to this epic.
- Child tickets created and linked to this epic.
- Contract consistency verified across all category specs.
- MCP naming verified and gaps fixed.
- Guidance updated.

Implementation validation (per child ticket):
- Each child ticket will validate with `cargo test -p <api>` and `cargo test -p <mcp>`.
- Each child ticket will verify MCP registration and agent-template wildcard wiring.
- Manual smoke tests will confirm bounded behavior and TOON output.

Epic closes when all three child tickets close.

---

## Effort

Design call only (child tickets have their own estimates):
- 4h: inventory verification, gap analysis, and contract definition.
- 3h: per-category spec authoring (three specs).
- 2h: child ticket creation with acceptance criteria.
- 1h: MCP naming verification and agent-template wildcard audit.
- 1h: guidance updates.

Total: 11h (already complete).

Child ticket estimates (sum: ~35h):
- File editing: 10-12h (medium-large)
- Filesystem operations: 8-10h (medium-large)
- Repo-wide search: 12-15h (medium-large)
