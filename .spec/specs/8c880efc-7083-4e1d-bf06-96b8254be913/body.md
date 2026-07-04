# Goal
Turn `session-api` from a capture/archive-only store into a runtime "cognitive workspace" that lets an agent bootstrap every session, proactively gather selective context (rules, specs, tickets) across stores, and pin only what the current task needs — so per-turn static instruction load shrinks dramatically and entity usage is curated over time.

Source design: `DESIGN_SESSION_BOOTSTRAPPING.md`.

# Problem
Today every turn drags ~1,190 lines (~8-10k tokens) of always-on guidance (four `applyTo: "**"` instruction files plus `AGENTS.md` and copilot-instructions), regardless of task. `session-api` only records transcripts after the fact; there is no proactive session state, no selective context surface, and no usage/quality signal, even though Tantivy search already exists (`rule_search`, `spec_search`, ticket search). This causes context dilution, state amnesia, full metacognition cost on trivial prompts, and no way to learn which entities are actually useful.

# Resolved Decisions (frozen)
1. **D1 — Session identity & resume.** The agent carries the `session_id`; the runtime context is persisted in the existing capture session directory under a well-known filename (`session_context.json`). `session_init` is **load-or-create and idempotent**: the first turn creates it, later turns resume the same session and keep mutating it. Rationale: the MCP server is stateless across turns, so the id the agent already holds is the only stable handle.
2. **D2 — Cross-store references are a hard prerequisite.** Pinned entities live in different stores (`.rule`, `.spec`, `.ticket`, nested workspaces). References are stored as **URNs** `ce://<workspace>/<store>/<entity>` and resolved through the cross-store resolver. The cascade's "hard links" are sourced from spec↔ticket cross-entity edges. Rationale: the cascade cannot pull related entities until cross-store resolution is robust. This gates implementation (see Dependencies).
3. **D3 — Client-side rendering.** `session_view` returns structured data; the agent injects it into its own prompt. The server never rewrites instruction files. Rationale: keeps the filesystem clean and avoids race conditions between parallel agents.
4. **D4 — File-backed persistence.** Every mutating call flushes `session_context.json` to disk before returning; `session_init` creates the session directory if absent. Rationale: crash-safe under server restarts; the JSON is tiny so IO cost is negligible.
5. **D5 — Cascade aggressiveness.** Auto-pin **only hard ID-linked** entities; semantic matches stay agent-driven. Rationale: a vague query (e.g. "fix the button") must not auto-pin 50 loosely matching rules.
6. **D6 — Headers-only rendering.** Pins and `session_view` carry only **short entity headers** (urn/id, type, title or slug, relation, reason). Full bodies are fetched explicitly by the agent via existing get/peek tools when needed. Rationale: prevents re-bloating the context we are trying to shrink.
7. **D7 — Remove always-on generated instructions.** Only a minimal **bootstrapper** instruction stays always-on. All other guidance becomes **discoverable rule entries** that the agent gathers (search), pins, and renders into its own per-session instruction set. Rule *filters/scopes* may be pinned; individual rule bodies are fetched on demand. Rationale: eliminate the per-turn metacognition load at its source.
8. **D8 — No mode field.** There is no `current_mode`. Every session performs a bootstrap; general chat passes a **minimal bootstrap** with zero pinned entities. Rationale: a coarse mode flag is unnecessary once a uniform, cheap bootstrap path exists.
9. **D9 — Usage counting & feedback.** Each pin records a **usage event** (frequency) against the entity. Before responding, agents may attach a **rating** (`helpful`/`mixed`/`not-helpful`) and optional note to pinned entities. The capability is **generic over entity type** in memory-api (specs and rules now; tickets supported by the same model later). Rationale: curate frequently-useful entities and detect obsolete low-value ones to improve rules and specs.

# Scope
- Runtime session contract: `session_context.json` holding `pinned_entities` for tickets, specs, and rules as URNs with `relation`/`reason`, plus timestamps.
- Bootstrap surface: `session_init`, `session_pin`, `session_unpin`, `session_view` (core lib + CLI + MCP), all headers-only.
- Cascade: given a ticket at `session_init`, auto-pin only hard ID-linked entities as suggestions, resolved via URNs.
- Usage + feedback hooks: pin → usage event; end-of-session rating on pinned entities.
- Preserve the existing capture/archive/worktree path unchanged; the runtime path is additive.

# Non-goals
- Replacing or rewriting the existing capture/transcript/worktree workflow.
- Semantic auto-pinning of vague matches (agent-driven only).
- A server-resident, stateful MCP process (assume the server may restart between turns).
- Building the full heavyweight `feedback-api` ingestion/search/SLO program — the runtime path consumes only the lightweight per-entity usage+rating capability.

# Dependencies (must land before session-bootstrap implementation)
- **Cross-store references (robust):** URN model + resolver and recursive multi-store discovery.
  - default store: [82d6ada4 URN cross-store reference model and resolver](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/82d6ada4-ac35-45a7-9df6-7b7501d58e70/ticket.toml), [6bd67a7a dynamic multi-store discovery and cross-store references](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/6bd67a7a-2a76-4dd7-a897-b4d325476621/ticket.toml) (program tracker [671d4e47](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/671d4e47-b53d-4a04-aa1d-30f2aa8a2bbe/ticket.toml)).
  - hard-link source (memory-api store, **cannot be graph-edged from this store yet** — recorded textually): [b03be2d5 cross-entity edges spec↔ticket](C:/Users/linus/git/graph_app/context-engine/memory-api/.ticket/tickets/b03be2d5-5293-4dc7-ad11-cca2dbf32c8b/ticket.toml), [f00291a3 ticket↔spec integration](C:/Users/linus/git/graph_app/context-engine/memory-api/.ticket/tickets/f00291a3-bd61-469e-a737-c44cb3911e3b/ticket.toml).
- **Feedback ratings + usage counting (core):** rule-entry feedback (done) + direct spec feedback ([29bf9628](C:/Users/linus/git/graph_app/context-engine/memory-api/.ticket/tickets/29bf9628-1dc5-4bb4-ae00-b7410dd52db5/ticket.toml), memory-api store) + a new generic usage-counting capability (see epic).

# Acceptance Criteria
1. A frozen `session_context.json` schema (URN refs, no `current_mode`) is referenced by every child spec.
2. `session_init`/`pin`/`unpin`/`view` signatures are defined headers-only and implemented verbatim by CLI and MCP.
3. Pinning records a usage event and supports a per-entity rating before session end, using the generic memory-api feedback model.
4. The existing capture/archive path has a regression test proving runtime additions do not change its on-disk output.
5. Cross-store entity references are stored and resolved as URNs.

# Traceability
- Epic: [effba966 session-bootstrap epic](C:/Users/linus/git/graph_app/context-engine/memory-api/memory-api/.ticket/tickets/effba966-f0a8-4d7d-b289-b7feba826cf8/ticket.toml)
- Design/owner: [afa00b5c bootstrap contract & ADRs](C:/Users/linus/git/graph_app/context-engine/memory-api/memory-api/.ticket/tickets/afa00b5c-c736-4d75-b157-d3e9ce90d819/ticket.toml)
- Runtime model: [412964a3 runtime session-context model](C:/Users/linus/git/graph_app/context-engine/memory-api/memory-api/.ticket/tickets/412964a3-e1c3-47da-94ad-268ff20441c0/ticket.toml)
- Cascade: [d8f76965 cascade context gathering](C:/Users/linus/git/graph_app/context-engine/memory-api/memory-api/.ticket/tickets/d8f76965-1ff3-4a0a-bb24-773b9637fae4/ticket.toml)
- CLI/MCP: [6b2dc497 init/pin/unpin/view surfaces](C:/Users/linus/git/graph_app/context-engine/memory-api/memory-api/.ticket/tickets/6b2dc497-188c-44f5-9106-bf35deecb7a1/ticket.toml)
- Rule rendering: [b4a8dc5e minimal bootstrapper + selective loading](C:/Users/linus/git/graph_app/context-engine/memory-api/memory-api/.ticket/tickets/b4a8dc5e-9d80-4fea-bb42-0c30aba0ecd6/ticket.toml)

# Related Specs
- `memory-api/session-api/persistence-writer` (existing capture write path that must stay intact)
- `memory-api/session-api/hook-ingestion-read-query` (existing read/query surface to reuse)
- `context-engine/session-worktree-default-workflow` (worktree check-in that init must compose with)