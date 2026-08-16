<!-- ticket-index:file generated=true -->

# Ticket Catalog

Generated ticket index grouped by state and component. Use this before scanning raw `.ticket/tickets/` folders.

## State: backlog

### Component: memory-api,rule-api

<!-- ticket-index:entry id=818e894a-09ad-462a-9283-7895f0962600 slug=backlog/memory-api,rule-api digest=34410c883170 -->
#### [818e894a] Stray ambient `.rule` store under OS Temp root pollutes ancestor-scan discovery in tests
- priority: `low`
- summary: A stray `.rule` directory exists directly under the OS Temp root on this dev machine (`C:/Users/linus/AppData/Local/Temp/.rule/rules/`, created 2026-07-27 20:40, containing rules with slugs `shared/t...
- ref: `memory-api/.ticket/tickets/818e894a-09ad-462a-9283-7895f0962600/ticket.toml`


### Component: pdf

<!-- ticket-index:entry id=ab1a7235-cae2-4393-91ea-864e0b2c71d4 slug=backlog/pdf digest=1e4187e18155 -->
#### [ab1a7235] [superseded by 7f39fae3] pdf-http transport (follow-up, out of v1 scope)
- summary: Backlog follow-up: add an `http` feature/bin to the `pdf` facade crate, following the same `transport-harness` HTTP wiring pattern used by other domain crates (e.g. `workflow-tools-contract-reference...
- ref: `.ticket/tickets/ab1a7235-cae2-4393-91ea-864e0b2c71d4/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=1ff57502-ad4e-4c40-a852-18752c18f44c slug=backlog/unspecified digest=4ad53d7ab40e -->
#### [1ff57502] [ticket-api] Defer inverse "sessions that worked on this ticket" query to a follow-up ticket
- summary: Track the Ticket-API-side inverse query ("which sessions worked on me", queried from the ticket rather than the session), explicitly deferred out of the initial context-enrichment workflow scope per ...
- ref: `.ticket/tickets/1ff57502-ad4e-4c40-a852-18752c18f44c/ticket.toml`


## State: blocked

### Component: session-api

<!-- ticket-index:entry id=25dd26cb-5768-4c01-8b88-a65c906a432a slug=blocked/session-api digest=bae72fc0a522 -->
#### [25dd26cb] Parallel sessions within a track
- summary: Only sequential sessions are supported within a track. Parallel sessions would enable concurrent work on different aspects of the same goal.
- ref: `.ticket/tickets/25dd26cb-5768-4c01-8b88-a65c906a432a/ticket.toml`


## State: cancelled

### Component: cli

<!-- ticket-index:entry id=48ea4df8-25f5-46ce-b2cc-ff00d32ddd47 slug=cancelled/cli digest=d50365a0abb2 -->
#### [48ea4df8] [bootstrap] run one-week dogfood trial and publish go-no-go report
- summary: Status: PLANNED
- ref: `.ticket/tickets/48ea4df8-25f5-46ce-b2cc-ff00d32ddd47/ticket.toml`


### Component: context-tasks

<!-- ticket-index:entry id=d5ced7e2-fc67-4a37-a621-96a54a71e51f slug=cancelled/context-tasks digest=8bd4334e8504 -->
#### [d5ced7e2] Plan: Copilot API execution layer — isolated sub-agents + review orchestration
- summary: Build a Rust execution layer that can:
- ref: `.ticket/tickets/d5ced7e2-fc67-4a37-a621-96a54a71e51f/ticket.toml`

<!-- ticket-index:entry id=a8632357-fce3-4191-9283-3de2b53c2e82 slug=cancelled/context-tasks digest=743b8ab16201 -->
#### [a8632357] [AOH][Impl] Assignment Runner -- concurrent sub-agent execution and progress watcher
- summary: Drive agent sessions from kickoff through completion, streaming progress events and managing the session lifecycle state machine. The assignment runner sits between the sandbox manager (which provisi...
- ref: `.ticket/tickets/a8632357-fce3-4191-9283-3de2b53c2e82/ticket.toml`

<!-- ticket-index:entry id=8c185de3-88f9-4565-915e-220d5656d9ac slug=cancelled/context-tasks digest=bca13eae207c -->
#### [8c185de3] [AOH][Impl] Copilot API client -- execution provider contracts and auth
- summary: Provide the typed HTTP client that all agent sessions use to communicate with GitHub Copilot's API. Per ADR-5, Copilot is the **only** LLM provider in v1 — there is no provider abstraction layer. Thi...
- ref: `.ticket/tickets/8c185de3-88f9-4565-915e-220d5656d9ac/ticket.toml`

<!-- ticket-index:entry id=0135d961-c76b-44d2-97d6-c3f08ee7d806 slug=cancelled/context-tasks digest=c0f59b4ff40c -->
#### [0135d961] [AOH][Impl] End-to-end executor integration and fault-injection suite
- summary: Validate that the full AOH execution stack works correctly when all components are composed: Copilot client → sandbox manager → assignment runner → review coordinator → notifier → TUI event flow. Thi...
- ref: `.ticket/tickets/0135d961-c76b-44d2-97d6-c3f08ee7d806/ticket.toml`

<!-- ticket-index:entry id=8db8ef2f-e33c-4234-a39a-64a481b27984 slug=cancelled/context-tasks digest=a3ca091e5675 -->
#### [8db8ef2f] [AOH][Impl] Notifier Adapters -- desktop and messenger delivery for assignment and review events
- summary: Deliver actionable notifications to the human operator when agent sessions produce events that require attention: review requests, budget warnings, failures, and merge completions. Per ADR-2, v1 supp...
- ref: `.ticket/tickets/8db8ef2f-e33c-4234-a39a-64a481b27984/ticket.toml`

<!-- ticket-index:entry id=d0cc3c8b-efc8-44c4-bbca-5daf4ddcdb8b slug=cancelled/context-tasks digest=f966894e1109 -->
#### [d0cc3c8b] [AOH][Impl] Review Coordinator -- validator handoff and state guards
- summary: Enforce separation-of-duties between agent implementation and human review. The review coordinator manages the handoff from a completed agent session (`Reporting` / `PROpen` state) through human revi...
- ref: `.ticket/tickets/d0cc3c8b-efc8-44c4-bbca-5daf4ddcdb8b/ticket.toml`

<!-- ticket-index:entry id=51471c3e-a088-47d4-9922-ba49d914af17 slug=cancelled/context-tasks digest=f002fe399f61 -->
#### [51471c3e] [AOH][Impl] Sandbox Manager -- per-assignment worktree and branch isolation
- summary: Provision and manage isolated execution environments for agent sessions. Per ADR-1, v1 uses **Docker containers** (primary) with **Podman** as a Linux CI alternative, orchestrated through the `bollar...
- ref: `.ticket/tickets/51471c3e-a088-47d4-9922-ba49d914af17/ticket.toml`

<!-- ticket-index:entry id=5af54f6c-6192-49d8-8a35-c8581066a586 slug=cancelled/context-tasks digest=ab0b4b3d2883 -->
#### [5af54f6c] [AOH][Impl] Terminal UI -- queue view, assignment status, and review workflow
- summary: Provide the primary human interface for the AOH orchestrator in v1. Per ADR-4, the entrypoint is a **Rust daemon with a `ratatui` TUI** — no VS Code extension in v1. The TUI is the operator's console...
- ref: `.ticket/tickets/5af54f6c-6192-49d8-8a35-c8581066a586/ticket.toml`


### Component: doc-viewer

<!-- ticket-index:entry id=06a194e8-d883-45a4-9693-6a4b9123ec5a slug=cancelled/doc-viewer digest=f01a78a11625 -->
#### [06a194e8] Port: doc-viewer Leptos frontend
- summary: The doc-viewer currently uses a Preact/TS frontend with marked + highlight.js for markdown rendering and a tree-based crate browser. This needs a Leptos/WASM port that renders markdown as native DOM ...
- ref: `.ticket/tickets/06a194e8-d883-45a4-9693-6a4b9123ec5a/ticket.toml`


### Component: graph

<!-- ticket-index:entry id=be1a3de7-f44f-496d-b4c6-b4f8a120dc97 slug=cancelled/graph digest=c2f4669d9ddf -->
#### [be1a3de7] [bootstrap] add merge queue scheduler with lease conflict overlay
- summary: `TaskCommand` is the canonical machine protocol.
- ref: `.ticket/tickets/be1a3de7-f44f-496d-b4c6-b4f8a120dc97/ticket.toml`

<!-- ticket-index:entry id=5e4727f9-53a6-4d36-a98f-4c9a6db81216 slug=cancelled/graph digest=b16913755c41 -->
#### [5e4727f9] [bootstrap] implement deps, blocked-by, and validate-graph commands
- summary: Status:** DONE (formally closed — see EXECUTION_CHECKLIST.md for handoff)
- ref: `.ticket/tickets/5e4727f9-53a6-4d36-a98f-4c9a6db81216/ticket.toml`


### Component: lease

<!-- ticket-index:entry id=2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2 slug=cancelled/lease digest=ec9ef3a381e4 -->
#### [2a1fa2f2] [bootstrap] implement lease lifecycle with stale recovery
- summary: Status:** BLOCKED (requires Phase 1 CRUD stable)
- ref: `memory-api/.ticket/tickets/2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2/ticket.toml`


### Component: log-viewer-leptos

<!-- ticket-index:entry id=ee6e2d37-60b0-434f-8b8e-d2ccbb2f7624 slug=cancelled/log-viewer-leptos digest=3b6e9c510636 -->
#### [ee6e2d37] Bug: GPU canvas should cover the full page, not just hypergraph view
- summary: The GPU canvas is currently scoped to the `.lv-hypergraph-view` container instead of covering the full viewport. It uses `position: absolute; inset: 0` within its parent, so it only renders in the hy...
- ref: `.ticket/tickets/ee6e2d37-60b0-434f-8b8e-d2ccbb2f7624/ticket.toml`

<!-- ticket-index:entry id=92eac6aa-d560-4436-beab-0de86b806c9f slug=cancelled/log-viewer-leptos digest=c92140cd48d3 -->
#### [92eac6aa] Bug: Node projection broken — nodes cluster in top-left corner
- summary: D hypergraph nodes are projected into a small area in the top-left corner instead of spreading across the viewport. Nodes should overflow behind glass sidebar/header panels (treated as overlays). Ful...
- ref: `.ticket/tickets/92eac6aa-d560-4436-beab-0de86b806c9f/ticket.toml`

<!-- ticket-index:entry id=868a3690-2269-4733-877f-9c53e196a819 slug=cancelled/log-viewer-leptos digest=eec0e071c3ec -->
#### [868a3690] Feature: Code viewer with source file references in log entries
- summary: Log entries carry source file paths and line numbers, but the Leptos frontend has no way to view source code. The TS version has a right panel CodeViewer with Prism.js syntax highlighting, clickable ...
- ref: `.ticket/tickets/868a3690-2269-4733-877f-9c53e196a819/ticket.toml`

<!-- ticket-index:entry id=17358907-1f1c-4c3f-b250-3d8220b6c363 slug=cancelled/log-viewer-leptos digest=83be216c1e80 -->
#### [17358907] Feature: Theme save/load/export/import
- priority: `low`
- summary: T3 builds the theme editing UI with 49 color pickers and effect sliders, but custom themes are lost on refresh. Users need persistent storage (save/load), sharing (export/import as JSON files), and m...
- ref: `.ticket/tickets/17358907-1f1c-4c3f-b250-3d8220b6c363/ticket.toml`


### Component: pdf

<!-- ticket-index:entry id=84a9f497-fe5a-4c04-b1e1-ab99245e6ea0 slug=cancelled/pdf digest=1216f02615d1 -->
#### [84a9f497] PDF domain capability (pdf-api/pdf) exposed via CLI + MCP
- summary: Add a new `pdf` domain capability so agents can, via MCP named tools (and CLI fallback):
- ref: `.ticket/tickets/84a9f497-fe5a-4c04-b1e1-ab99245e6ea0/ticket.toml`

<!-- ticket-index:entry id=01ebb34b-7559-4b87-b535-ad6f44354b57 slug=cancelled/pdf digest=e927c4675ec3 -->
#### [01ebb34b] T0: PDF crate verification spike (versions, licenses, capability coverage)
- summary: Bounded research spike: verify the actual state of candidate pure-Rust PDF crates before any implementation ticket locks in an API. No web access was available when this epic/track was authored, so e...
- ref: `.ticket/tickets/01ebb34b-7559-4b87-b535-ad6f44354b57/ticket.toml`

<!-- ticket-index:entry id=21402279-40cd-45ed-94dc-ff13e0e2da35 slug=cancelled/pdf digest=67f7aefd5efb -->
#### [21402279] T1: Scaffold pdf-api + pdf facade crates (feature-gated cli/mcp bins, workspace wiring)
- summary: Scaffold the `pdf-api` internal crate and `pdf` public facade crate per `WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md`, with feature-gated `cli`/`mcp` binaries and root workspace wiring. No domain logic i...
- ref: `.ticket/tickets/21402279-40cd-45ed-94dc-ff13e0e2da35/ticket.toml`

<!-- ticket-index:entry id=1c255835-8ace-4fe8-8827-ac4520279128 slug=cancelled/pdf digest=974d8992a6f7 -->
#### [1c255835] T2: pdf-api core types, request/response dispatch, error enum, sandboxing + write-safety layer
- summary: Build the `pdf-api` core: a tagged request/response type system covering all six v1 operations, a `thiserror` error enum, a single `execute()` dispatch function, and the shared write-safety + sandbox...
- ref: `.ticket/tickets/1c255835-8ace-4fe8-8827-ac4520279128/ticket.toml`

<!-- ticket-index:entry id=0b7497a2-18ec-4e5a-b1d6-4c8a4cfac45d slug=cancelled/pdf digest=5c43bff2dae6 -->
#### [0b7497a2] T3: PDF text extraction
- summary: Implement text extraction from PDF files using the crate selected by T0.
- ref: `.ticket/tickets/0b7497a2-18ec-4e5a-b1d6-4c8a4cfac45d/ticket.toml`

<!-- ticket-index:entry id=4ff956f5-8eac-442c-ba25-5bbe2dc3b05a slug=cancelled/pdf digest=71db4b38501e -->
#### [4ff956f5] T4: PDF page operations (merge, split, reorder, delete) + metadata read/write
- summary: Implement page/document-level operations: merge multiple PDFs into one, split a PDF into per-page (or per-range) output files, reorder/delete pages within a PDF, and read/write document metadata (tit...
- ref: `.ticket/tickets/4ff956f5-8eac-442c-ba25-5bbe2dc3b05a/ticket.toml`

<!-- ticket-index:entry id=ea2ff01e-7a7d-4be5-980d-d3f2a4fa731d slug=cancelled/pdf digest=a23db3105846 -->
#### [ea2ff01e] T5: PDF creation — programmatic primitive + optional typst-cli path
- summary: Implement PDF creation in two modes: (a) a programmatic primitive using a pure-Rust PDF-generation crate (candidate: `printpdf`, to be confirmed by T0), and (b) an optional path that shells out to `t...
- ref: `.ticket/tickets/ea2ff01e-7a7d-4be5-980d-d3f2a4fa731d/ticket.toml`

<!-- ticket-index:entry id=903976ff-d9fc-4a6b-9f98-4f2c11d75f18 slug=cancelled/pdf digest=ee48efa715c2 -->
#### [903976ff] T6: pdf-cli transport (transport-harness wiring, one subcommand per operation)
- summary: Wire the `pdf-cli` binary transport using `transport-harness`, exposing every `pdf-api` operation as a CLI subcommand.
- ref: `.ticket/tickets/903976ff-d9fc-4a6b-9f98-4f2c11d75f18/ticket.toml`

<!-- ticket-index:entry id=57cd6a98-cee4-4c0e-a85c-667bef3d8085 slug=cancelled/pdf digest=103529c19c6c -->
#### [57cd6a98] T7: pdf-mcp transport (named tools, error mapping, ServerHandler)
- summary: Wire the `pdf-mcp` binary transport following the canonical named-tool MCP pattern from `memory-api/tools/mcp/peek-mcp/src/server.rs`, exposing every `pdf-api` operation as a named MCP tool.
- ref: `.ticket/tickets/57cd6a98-cee4-4c0e-a85c-667bef3d8085/ticket.toml`

<!-- ticket-index:entry id=527cc1ad-1603-49d6-b0ae-97c4063daebd slug=cancelled/pdf digest=ded118abff22 -->
#### [527cc1ad] T8: Author .agents/skills/pdf/SKILL.md + register in skills README Master Index
- summary: Author the hand-owned `.agents/skills/pdf/SKILL.md` skill doc and register it in the skills Master Index, instructing agents to prefer `pdf-mcp` named tools with `pdf-cli` documented as fallback.
- ref: `.ticket/tickets/527cc1ad-1603-49d6-b0ae-97c4063daebd/ticket.toml`

<!-- ticket-index:entry id=3ea5d521-981c-4fbc-ac58-78fd92181d17 slug=cancelled/pdf digest=a80c00fb33aa -->
#### [3ea5d521] T9: PDF embedded image extraction (last, cuttable)
- summary: Implement embedded image extraction from PDFs: given a PDF, extract each embedded raster image to a file in an output directory. Sequenced last so it can be cut from v1 without blocking any other cap...
- ref: `.ticket/tickets/3ea5d521-981c-4fbc-ac58-78fd92181d17/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=13ca22b3-db4e-4d91-8515-ac0e90785201 slug=cancelled/session-api digest=dd268bb07c46 -->
#### [13ca22b3] Link d1b3a6c9 (workflow diagnostics routing + structural validation) to a spec with explicit acceptance criteria
- priority: `medium`
- summary: Ticket d1b3a6c9 "Route workflow diagnostics upward and add structural workflow-graph validation" has no `acceptance_criteria` field and no linked spec. During review, spec c737328d was checked as a c...
- ref: `memory-api/.ticket/tickets/13ca22b3-db4e-4d91-8515-ac0e90785201/ticket.toml`

<!-- ticket-index:entry id=968e863b-8469-4de8-ba12-dff1c927ca24 slug=cancelled/session-api digest=0b741ebe2428 -->
#### [968e863b] Make session assignments and MCP routing independent of deleted worktrees
- priority: `high`
- summary: Preserve session-anchored MCP routing without persisting a root-store dependency on another worktree's path or contents.
- ref: `.ticket/tickets/968e863b-8469-4de8-ba12-dff1c927ca24/ticket.toml`

<!-- ticket-index:entry id=674e8e44-55ee-472f-8044-5d6e473438cf slug=cancelled/session-api digest=8aa5f135fe22 -->
#### [674e8e44] Session-to-worktree assignment is never persisted, and provisioning outcomes are unobservable per hook event
- priority: `high`
- summary: The session-to-worktree assignment is never persisted after worktree provisioning. A session with a matching registered worktree therefore resolves to the main checkout, breaking ownership routing fo...
- ref: `.ticket/tickets/674e8e44-55ee-472f-8044-5d6e473438cf/ticket.toml`


### Component: test-api

<!-- ticket-index:entry id=57a13857-b80e-4d91-99e8-452ccebe4c38 slug=cancelled/test-api digest=30e0f1f56164 -->
#### [57a13857] [tools] Close transport surface gaps (or document intentional absences)
- priority: `medium`
- summary: The transport surfaces are heterogeneous, which forces the e2e matrix (#1) to blanket-block whole domains. Decide and act: build the missing surfaces, or formally document them as out-of-scope so the...
- ref: `memory-api/.ticket/tickets/57a13857-b80e-4d91-99e8-452ccebe4c38/ticket.toml`


### Component: ticket

<!-- ticket-index:entry id=52f2b14f-fd64-5eb2-83a3-a2c99c5680ce slug=cancelled/ticket digest=45004fc10ead -->
#### [52f2b14f] [feedback-followup][ticket] Address not-helpful feedback on ce://default/ticket/6a47ab0f-7e42-463e-afe0-bf51b85249c9
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/6a47ab0f-7e42-463e-afe0-bf51b85249c9` during session `e31bd0e5-ab29-4e76-9284-5f3d2067f40c` (tool call `toolu_01Gkjst5pUyW8UfkdQgamdPA`).
- ref: `.ticket/tickets/52f2b14f-fd64-5eb2-83a3-a2c99c5680ce/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=b458cba7-54b1-45d8-8c86-17b920416b8b slug=cancelled/ticket-http digest=12b418df5336 -->
#### [b458cba7] API: Batch mutation endpoint for transactional multi-command execution
- priority: `high`
- ref: `memory-api/.ticket/tickets/b458cba7-54b1-45d8-8c86-17b920416b8b/ticket.toml`

<!-- ticket-index:entry id=3fd32109-7122-4fdf-80f2-b741db5d3b30 slug=cancelled/ticket-http digest=d0386b24ec0b -->
#### [3fd32109] [ticket-http][ticket-viewer] Expose workspace graph payload for focused full-graph navigation
- priority: `high`
- summary: Provide infrastructure for a ticket-viewer graph mode that can keep the whole workspace graph visible while focusing the selected ticket.
- ref: `memory-api/.ticket/tickets/3fd32109-7122-4fdf-80f2-b741db5d3b30/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=2b3a6e2e-4911-4b33-a3a9-9ace11f26637 slug=cancelled/ticket-viewer digest=03e32a803ce8 -->
#### [2b3a6e2e] Bug: TicketDetail right panel hardcoded colors — port to theme variables
- priority: `high`
- summary: `tools/viewer/ticket-viewer/frontend/dioxus/src/components/ticket_detail.rs` builds the right-side ticket detail panel from inline hardcoded hex colors. When the theme is set to PAPER (light) the pan...
- ref: `memory-viewers/.ticket/tickets/2b3a6e2e-4911-4b33-a3a9-9ace11f26637/ticket.toml`

<!-- ticket-index:entry id=fea28293-5494-49e1-bdb4-8165457b59ca slug=cancelled/ticket-viewer digest=48b4e37b89a4 -->
#### [fea28293] Feature: Batch operations — multi-select, queue, bulk apply, filter-based updates
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/fea28293-5494-49e1-bdb4-8165457b59ca/ticket.toml`

<!-- ticket-index:entry id=e42d8e0a-c210-4efe-a22c-2565079e67b8 slug=cancelled/ticket-viewer digest=d61fbaac65e5 -->
#### [e42d8e0a] Feature: History timeline — revision viewer with field diffs
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/e42d8e0a-c210-4efe-a22c-2565079e67b8/ticket.toml`

<!-- ticket-index:entry id=5711c397-9f0e-442e-a65d-e4295f735593 slug=cancelled/ticket-viewer digest=54fba65d6df9 -->
#### [5711c397] Port: SVG dependency graph fallback view
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/5711c397-9f0e-442e-a65d-e4295f735593/ticket.toml`

<!-- ticket-index:entry id=d83f8e52-090e-42d7-9fcf-f389afdbb90c slug=cancelled/ticket-viewer digest=4edc243fa4f0 -->
#### [d83f8e52] Port: ticket-viewer Leptos frontend
- summary: The ticket-viewer currently uses a Preact/TS frontend with WebGPU 3D dependency graph visualization, SVG fallback, SSE live streaming, and a ticket list grouped by state. This needs a Leptos/WASM por...
- ref: `memory-viewers/.ticket/tickets/d83f8e52-090e-42d7-9fcf-f389afdbb90c/ticket.toml`

<!-- ticket-index:entry id=b1592d19-82c4-44b5-8633-8788a202b438 slug=cancelled/ticket-viewer digest=38bf1d1cfcf6 -->
#### [b1592d19] [probe] ticket root target
- ref: `memory-viewers/.ticket/tickets/b1592d19-82c4-44b5-8633-8788a202b438/ticket.toml`

<!-- ticket-index:entry id=2dcc66b5-c061-45fa-a55f-63b731727bb6 slug=cancelled/ticket-viewer digest=61c2386b4dd8 -->
#### [2dcc66b5] [ticket-viewer] Build integrated ticket document panel
- priority: `high`
- summary: Replace the split metadata/content treatment with a single compact ticket document area in the main layout.
- ref: `memory-api/.ticket/tickets/2dcc66b5-c061-45fa-a55f-63b731727bb6/ticket.toml`

<!-- ticket-index:entry id=379ff931-7e0a-4069-a1d7-86cc3ef73e9e slug=cancelled/ticket-viewer digest=00cc0d0d49b1 -->
#### [379ff931] [ticket-viewer] Fix graph layout defaults and isometric settings
- priority: `high`
- summary: Fix graph layout defaults and settings so dependency hierarchy reads cleanly from top to bottom on a 2D plane optimized for isometric viewing.
- ref: `memory-api/.ticket/tickets/379ff931-7e0a-4069-a1d7-86cc3ef73e9e/ticket.toml`

<!-- ticket-index:entry id=3526cce3-c934-4c37-b7a8-c7c0979f308d slug=cancelled/ticket-viewer digest=07fdde5f2cb2 -->
#### [3526cce3] [ticket-viewer] Keep full workspace graph visible with focused navigation
- priority: `high`
- summary: Change the ticket-viewer graph mode so the full graph stays visible while the selected ticket becomes the active focus anchor.
- ref: `memory-api/.ticket/tickets/3526cce3-c934-4c37-b7a8-c7c0979f308d/ticket.toml`

<!-- ticket-index:entry id=6e7a15c9-d8e6-4bbe-bb34-b83bd651896b slug=cancelled/ticket-viewer digest=ab4fc7c5afc9 -->
#### [6e7a15c9] [ticket-viewer] Keep full workspace graph visible with focused navigation
- priority: `high`
- summary: Change the ticket-viewer graph mode so the full graph stays visible while the selected ticket becomes the active focus anchor.
- ref: `memory-viewers/.ticket/tickets/6e7a15c9-d8e6-4bbe-bb34-b83bd651896b/ticket.toml`

<!-- ticket-index:entry id=0d2e5a7d-f76b-474a-8991-b3a56ea73ac5 slug=cancelled/ticket-viewer digest=51e5b8f0218a -->
#### [0d2e5a7d] [ticket-viewer][ticket-http][viewer-api] Improve main layout ticket documents and focused full-graph navigation
- priority: `high`
- summary: Upgrade the ticket-viewer main layout so ticket details render as a compact integrated document and the graph view becomes a focused full-workspace navigation surface with better layout, settings, an...
- ref: `memory-api/.ticket/tickets/0d2e5a7d-f76b-474a-8991-b3a56ea73ac5/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=7a351e71-52dd-484b-8a51-1e15744dcfb6 slug=cancelled/unspecified digest=06d5b00f1ef6 -->
#### [7a351e71] Gitignore tools/model-prices/__pycache__ (tracked bytecode drift)
- summary: `tools/model-prices/__pycache__/cost_gate.cpython-314.pyc` is tracked in git. Running `test_cost_gate.py` or `sync_model_prices.py` regenerates the bytecode cache, producing a spurious binary diff on...
- ref: `.ticket/tickets/7a351e71-52dd-484b-8a51-1e15744dcfb6/ticket.toml`

<!-- ticket-index:entry id=b7f8e991-2db0-4499-869c-c41bf92ff0da slug=cancelled/unspecified digest=56fbeaf5bd54 -->
#### [b7f8e991] Test ticket from Dioxus form
- ref: `.ticket/tickets/b7f8e991-2db0-4499-869c-c41bf92ff0da/ticket.toml`

<!-- ticket-index:entry id=4e28bf38-bd3c-466c-9eee-cd618d5f45fe slug=cancelled/unspecified digest=f01b7895d49c -->
#### [4e28bf38] [AOH] Epic: Agent Orchestration Harness — Complete Agentic Workflow System
- priority: `high`
- summary: Design and implement a full-stack agent orchestration harness that covers the full development lifecycle: user-driven research, ticket refinement, autonomous parallel implementation, local review/mer...
- ref: `.ticket/tickets/4e28bf38-bd3c-466c-9eee-cd618d5f45fe/ticket.toml`

<!-- ticket-index:entry id=a92569e5-3582-4191-9513-80ce6938cda4 slug=cancelled/unspecified digest=a0b60f0e5790 -->
#### [a92569e5] [AOH][Impl] Agent Identity — Persona Store, LRU Assignment, Trait Injection
- summary: Manage the pool of reusable agent personas and handle assignment to sessions. Per ADR-8, agents are identified by nature-vocabulary personas (e.g., "Basalt", "Coral", "Zephyr") drawn from a configura...
- ref: `.ticket/tickets/a92569e5-3582-4191-9513-80ce6938cda4/ticket.toml`

<!-- ticket-index:entry id=6e6b8cf6-3dd8-4b82-939e-a879248271ce slug=cancelled/unspecified digest=a4794419c70b -->
#### [6e6b8cf6] [AOH][Impl] Orchestrator Core — Daemon, Scheduler, Secret Server, Conflict Detector
- summary: Central daemon process that hosts all AOH orchestration services. The orchestrator core is the main binary (`orchestrator-tui` crate) that wires together the session scheduler, secret server, conflic...
- ref: `.ticket/tickets/6e6b8cf6-3dd8-4b82-939e-a879248271ce/ticket.toml`

<!-- ticket-index:entry id=dd5872f4-0267-42f2-a896-29403db2f47a slug=cancelled/unspecified digest=dfb49a82751a -->
#### [dd5872f4] [AOH][Research] WhatsApp Business API and multi-messenger adapter design
- priority: `high`
- summary: User decision (Q2):** Prefers WhatsApp as primary channel. Telegram, Slack, Discord also viable. System must support multiple simultaneously active messengers (user chooses preferred channel; orchest...
- ref: `.ticket/tickets/dd5872f4-0267-42f2-a896-29403db2f47a/ticket.toml`

<!-- ticket-index:entry id=283c2bc7-adb3-45c8-ae74-432709b3511c slug=cancelled/unspecified digest=f48360bb12d8 -->
#### [283c2bc7] [context-editor] Phase 5: Z-Prepass Depth Buffer for Early-Z Voxel Culling
- summary: The tiled rasterizer currently has `depth_stencil: None` in its `RenderPipelineDescriptor` and
- ref: `.ticket/tickets/283c2bc7-adb3-45c8-ae74-432709b3511c/ticket.toml`

<!-- ticket-index:entry id=f8b447b7-aa3d-498d-8900-672c6c8ba064 slug=cancelled/unspecified digest=856f2e7fd849 -->
#### [f8b447b7] [memory-api][curation] Generic entity usage counting & feedback ratings
- summary: Prerequisite for the [session-bootstrap] epic. Provides the generic, entity-type-agnostic curation primitive that session bootstrapping feeds.
- ref: `.ticket/tickets/f8b447b7-aa3d-498d-8900-672c6c8ba064/ticket.toml`

<!-- ticket-index:entry id=2bb8b3e1-cf28-431d-9f00-5f4fe90ba5b0 slug=cancelled/unspecified digest=b99450083655 -->
#### [2bb8b3e1] [memory-matrix] Close failure-bundle gaps for review readiness
- ref: `.ticket/tickets/2bb8b3e1-cf28-431d-9f00-5f4fe90ba5b0/ticket.toml`

<!-- ticket-index:entry id=3afaab37-b228-4051-bc46-618db4e0b82b slug=cancelled/unspecified digest=1db9822ae405 -->
#### [3afaab37] [ticket-api][bug] update_ticket resets state to `new` on field/description patch; transition_states silently no-ops
- priority: `high`
- summary: High — silently corrupts ticket lifecycle state. Any agent that edits a ticket's `description` or `fields` via the update API loses the ticket's current state (reset to `new`), and the documented mul...
- ref: `.ticket/tickets/3afaab37-b228-4051-bc46-618db4e0b82b/ticket.toml`

<!-- ticket-index:entry id=e9c7a153-c1ef-444d-b1fb-3e4d57bab7ff slug=cancelled/unspecified digest=9df17debd275 -->
#### [e9c7a153] throwaway bidi validation
- ref: `.ticket/tickets/e9c7a153-c1ef-444d-b1fb-3e4d57bab7ff/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=ca0f6ccc-545d-45df-bbbb-74a3daf0d18c slug=cancelled/viewer-api digest=d0984715e7bd -->
#### [ca0f6ccc] Arch: Extract viewer-api-leptos shared crate for all Leptos viewers
- summary: The log-viewer Leptos frontend currently lives as a monolith in `tools/viewer/log-viewer/frontend-leptos/`. Shared UI primitives (ResizeHandle, TreeView, TabBar, CodeViewer, ThemeSettings, WgpuOverla...
- ref: `viewer-api/.ticket/tickets/ca0f6ccc-545d-45df-bbbb-74a3daf0d18c/ticket.toml`

<!-- ticket-index:entry id=7f41940d-617a-495d-aad8-5a19111bdab9 slug=cancelled/viewer-api digest=39d3029d437b -->
#### [7f41940d] Epic: Leptos Viewer Platform — port viewer-api + all viewers to Leptos/Rust
- ref: `viewer-api/.ticket/tickets/7f41940d-617a-495d-aad8-5a19111bdab9/ticket.toml`

<!-- ticket-index:entry id=7b33f98e-9572-4ceb-8379-189621e4ae74 slug=cancelled/viewer-api digest=a0e277512281 -->
#### [7b33f98e] [viewer-api] Extract a reusable interactive chip button for Dioxus explorer filters
- priority: `high`
- ref: `viewer-api/.ticket/tickets/7b33f98e-9572-4ceb-8379-189621e4ae74/ticket.toml`

<!-- ticket-index:entry id=d3fb343c-1fea-47b1-8137-5ac7a37a95e1 slug=cancelled/viewer-api digest=4096be9d963c -->
#### [d3fb343c] [viewer-api][ticket-viewer] Add multi-level graph node detail rendering
- priority: `high`
- summary: Introduce multiple graph node detail levels so zoomed-out views stay legible and zoomed-in views can show rich ticket content.
- ref: `memory-api/.ticket/tickets/d3fb343c-1fea-47b1-8137-5ac7a37a95e1/ticket.toml`


### Component: watcher

<!-- ticket-index:entry id=de6c3391-27c2-4e27-bde8-1456f0eb3f43 slug=cancelled/watcher digest=f595b7a95fea -->
#### [de6c3391] [bootstrap] add crash-recovery test for atomic write plus reconcile
- summary: Status:** DONE (formally closed — see EXECUTION_CHECKLIST.md for handoff)
- ref: `.ticket/tickets/de6c3391-27c2-4e27-bde8-1456f0eb3f43/ticket.toml`

<!-- ticket-index:entry id=c91a334e-26cf-4cf2-9212-4288a07bbf09 slug=cancelled/watcher digest=437f1c7be6e5 -->
#### [c91a334e] [bootstrap] establish observability and failure diagnostics standard
- summary: Status: ACTIVE
- ref: `.ticket/tickets/c91a334e-26cf-4cf2-9212-4288a07bbf09/ticket.toml`


## State: done

### Component: agent-config

<!-- ticket-index:entry id=9dadecaa-4ec1-4110-aedf-2771e7189cb5 slug=done/agent-config digest=5b95935ae838 -->
#### [9dadecaa] Delegation policy: prefer workspace agent templates over VS Code built-in agents
- summary: Fresh-Eyes Review | 2026-07-26**
- ref: `.ticket/tickets/9dadecaa-4ec1-4110-aedf-2771e7189cb5/ticket.toml`


### Component: agent-guidance

<!-- ticket-index:entry id=9cd886d5-330e-40b4-b972-071e2609fe35 slug=done/agent-guidance digest=63550d1e1bb5 -->
#### [9cd886d5] Define applyTo-class scheme across .agents/instructions/**
- priority: `low`
- summary: Sub-agents currently receive the full applicable instruction corpus for their mode regardless of the specific ticket's domain, paying token cost for irrelevant sections (e.g. a database-migration wor...
- ref: `.ticket/tickets/9cd886d5-330e-40b4-b972-071e2609fe35/ticket.toml`

<!-- ticket-index:entry id=95403a71-cb2a-45ea-b48d-3444904e66dd slug=done/agent-guidance digest=11aecea9ed0f -->
#### [95403a71] Extend entity disambiguation protocol to all agent responses
- priority: `high`
- summary: question-quality.instructions.md line 16 already bans ambiguous pronouns, but only for interview questions. General agent chat responses still use "this", "that", "the engine" without establishing wh...
- ref: `.ticket/tickets/95403a71-cb2a-45ea-b48d-3444904e66dd/ticket.toml`

<!-- ticket-index:entry id=6426c891-8d9b-40cc-a163-1ec421ce9d62 slug=done/agent-guidance digest=66811f075caa -->
#### [6426c891] Simplify Agent: audit and condense instruction/guidance corpus
- priority: `high`
- summary: Guidance files under .agents/instructions/** have grown organically; sub-agents pay a token tax on every call reading noisy, possibly outdated, or over-specific rules. There is no repeatable process ...
- ref: `.ticket/tickets/6426c891-8d9b-40cc-a163-1ec421ce9d62/ticket.toml`

<!-- ticket-index:entry id=e38c258e-4502-4a92-95c7-1dac38fd24b7 slug=done/agent-guidance digest=b34f9ad4da73 -->
#### [e38c258e] [agent-guidance] One worktree + one branch per implementation agent, with root-orchestrator merge monopoly
- priority: `high`
- summary: Problem: every agent works in the same root checkout on `main`. Concurrent sessions overwrite each other's uncommitted edits, a `cargo fmt` or `git add -A` from one session swallows another's in-prog...
- ref: `.ticket/tickets/e38c258e-4502-4a92-95c7-1dac38fd24b7/ticket.toml`


### Component: agent-orchestration

<!-- ticket-index:entry id=1a240fdc-7de2-4494-8714-b2c81de09158 slug=done/agent-orchestration digest=231bbffc93d9 -->
#### [1a240fdc] Retry-limit escalation policy for worker-tier test failures
- priority: `medium`
- summary: Small/worker-tier agents can burn large amounts of tokens attempting repeated self-fixes after a failing test run, with no hard stop today. Existing fail-fast semantics in pre-dispatch-gates.instruct...
- ref: `.ticket/tickets/1a240fdc-7de2-4494-8714-b2c81de09158/ticket.toml`

<!-- ticket-index:entry id=feb5784c-ece7-40d3-9617-3faee2f6a753 slug=done/agent-orchestration digest=38ef0c59897c -->
#### [feb5784c] Spec: two-tier Planner/Worker model routing architecture
- priority: `medium`
- summary: Current orchestration allows a multi-tier chain (large model -> smaller model -> smaller model) via orchestrator-delegation.instructions.md and model-routing.instructions.md. Each extra hop dilutes i...
- ref: `.ticket/tickets/feb5784c-ece7-40d3-9617-3faee2f6a753/ticket.toml`

<!-- ticket-index:entry id=44c6cc5c-35c0-4cf0-b607-186914c21e5d slug=done/agent-orchestration digest=ecc7c32089ae -->
#### [44c6cc5c] Split-responsibility testing: frontier-authored tests, worker-only implementation
- priority: `medium`
- summary: Worker/small-model tiers are weak at designing comprehensive, correctly-scoped tests. Today nothing prevents a worker-tier agent from writing (and potentially weakening) its own test file to make its...
- ref: `.ticket/tickets/44c6cc5c-35c0-4cf0-b607-186914c21e5d/ticket.toml`

<!-- ticket-index:entry id=7563ce30-bf9e-43fe-bca5-68473b1d9d79 slug=done/agent-orchestration digest=31273b9df445 -->
#### [7563ce30] Write-and-die pattern for worker sub-agent dispatch
- priority: `medium`
- summary: Worker-tier sub-agents currently can remain conversational across multiple steps within one session, which burns tokens as they re-derive "what to do next" instead of executing a single predetermined...
- ref: `.ticket/tickets/7563ce30-bf9e-43fe-bca5-68473b1d9d79/ticket.toml`


### Component: agent-tooling

<!-- ticket-index:entry id=1fbf2d84-4a6b-4d8e-a69e-45aec87ff95f slug=done/agent-tooling digest=4aec0c423e99 -->
#### [1fbf2d84] Close the loop: Iteration Agent orchestrates review→interview→commit→handoff into a self-contained next handoff
- summary: We have deep tooling for *implementing* work but almost none for the post-implementation transition: turning a finished, validated implementation into the next self-contained handoff. Today an implem...
- ref: `.ticket/tickets/1fbf2d84-4a6b-4d8e-a69e-45aec87ff95f/ticket.toml`

<!-- ticket-index:entry id=373072a9-ddc6-42c3-b709-50a7d9659297 slug=done/agent-tooling digest=84b996697d29 -->
#### [373072a9] Delegation decision policy: case → cost-class mapping and allocation strategy
- summary: Author explicit agent instructions that specify, in an orchestrated session, **in which cases work is delegated and to which model class**, where classes are defined by **capability role** rather tha...
- ref: `.ticket/tickets/373072a9-ddc6-42c3-b709-50a7d9659297/ticket.toml`

<!-- ticket-index:entry id=ea12b7b0-11c2-42c0-b0a9-d2942093525b slug=done/agent-tooling digest=633d966b4244 -->
#### [ea12b7b0] Durable cross-session rules: loop-closure + escalation-gate instruction files
- summary: Add durable cross-session instruction rules so the loop discipline applies to **all** sessions, not just when the Iteration Agent runs.
- ref: `.ticket/tickets/ea12b7b0-11c2-42c0-b0a9-d2942093525b/ticket.toml`

<!-- ticket-index:entry id=d3af78d7-9486-43c0-aae7-ddd5681d9807 slug=done/agent-tooling digest=2f9117326a22 -->
#### [d3af78d7] Handoff-package schema spec + session_handoff required-field enforcement
- summary: Define the **handoff-package schema**: the required fields that make the next implementation session fully self-contained (zero discovery, zero user clarification). Document it as a spec and enforce ...
- ref: `.ticket/tickets/d3af78d7-9486-43c0-aae7-ddd5681d9807/ticket.toml`

<!-- ticket-index:entry id=858f731c-3d85-4bab-8e0f-73afc098ead0 slug=done/agent-tooling digest=8f441e04a160 -->
#### [858f731c] Implement Agent phase isolation: strip search + askQuestions tools + phase-separation rule
- summary: Isolate the implementation phase so it needs **no search and no user-clarification** tooling. Enforce it on the Implement Agent's tool surface and document it as a durable phase-separation rule.
- ref: `.ticket/tickets/858f731c-3d85-4bab-8e0f-73afc098ead0/ticket.toml`

<!-- ticket-index:entry id=76e831f2-a0fe-4078-be91-681b6fa371c7 slug=done/agent-tooling digest=712ea15123af -->
#### [76e831f2] Iteration Agent template + prompt (thin orchestrator: review→interview→commit→handoff)
- summary: Author a new **Iteration Agent** template (`.agents/agents/iteration.agent.md`) and matching prompt (`.agents/prompts/iteration.prompt.md`). It is a **thin orchestrator** for the post-implementation ...
- ref: `.ticket/tickets/76e831f2-a0fe-4078-be91-681b6fa371c7/ticket.toml`

<!-- ticket-index:entry id=5755b694-b8bc-42c6-869d-34761e8a822c slug=done/agent-tooling digest=a1c89afc52b1 -->
#### [5755b694] Iteration-loop workflow spec (phases, review→interview→commit→handoff ordering, gates)
- summary: Author the **iteration-loop workflow spec**: the phase model, canonical ordering, and gates that govern how a finished implementation becomes the next handoff.
- ref: `.ticket/tickets/5755b694-b8bc-42c6-869d-34761e8a822c/ticket.toml`

<!-- ticket-index:entry id=37b5026f-add9-4568-8953-fd5607fb91dc slug=done/agent-tooling digest=6a32fa270e86 -->
#### [37b5026f] Migrate .agents/instructions into nested workflow folders
- summary: Tracking ticket for migrating .agents/instructions/ into nested workflow folders.
- ref: `.ticket/tickets/37b5026f-add9-4568-8953-fd5607fb91dc/ticket.toml`

<!-- ticket-index:entry id=9d527ad1-616b-45fb-b67c-64e0396841fe slug=done/agent-tooling digest=144b56cf9896 -->
#### [9d527ad1] Per-tool-call token-load telemetry via mcp-cost-gate (proxy observes payloads, not usage)
- summary: Recovered from the `forward_handoff_package` snapshot in `history.ndjson` — these had fallen out of the live description, which is unacceptable on a ticket with this history.
- ref: `.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml`

<!-- ticket-index:entry id=41ff230b-cedf-4ec3-86cf-9b48a89b8325 slug=done/agent-tooling digest=fe422c85eeb7 -->
#### [41ff230b] Quality gates and session/tool-call data collection for delegated sessions
- summary: Establish **quality gates before and after delegated sessions** and collect the underlying data (sessions, tool calls, delegated sessions) needed to understand how often delegated sessions produce sa...
- ref: `.ticket/tickets/41ff230b-cedf-4ec3-86cf-9b48a89b8325/ticket.toml`

<!-- ticket-index:entry id=9185d8f2-1080-46b1-84da-485f9ad839f6 slug=done/agent-tooling digest=0e28b789da55 -->
#### [9185d8f2] Remove hardcoded token-heavy tool categorization; single default cost + empirical bootstrap
- summary: `Gate::tool_cost()` in memory-api/tools/mcp/mcp-cost-gate/src/gate.rs falls back to a hardcoded static classification whenever the empirical rollup lacks data for a tool. Tools matching `TOKEN_HEAVY_...
- ref: `.ticket/tickets/9185d8f2-1080-46b1-84da-485f9ad839f6/ticket.toml`

<!-- ticket-index:entry id=8c67b96a-b88f-4710-b8e2-f65e59c4e61e slug=done/agent-tooling digest=384df9fae9b8 -->
#### [8c67b96a] Session handoff record should own the full handoff package; ticket should only reference it
- summary: This ticket originally claimed that `session_handoff` accepts only `workspace`, `workspace_session_id`, and `validation[]`. **That is no longer true and must not be implemented against.**
- ref: `.ticket/tickets/8c67b96a-b88f-4710-b8e2-f65e59c4e61e/ticket.toml`

<!-- ticket-index:entry id=84c7757d-9819-4b06-a669-e62366db250a slug=done/agent-tooling digest=3854ff3722ad -->
#### [84c7757d] Session store: populate tool-metrics, capture tool error text, and count timeouts/hangs as non-success
- summary: The session store's failure signal is misleading. In session 51701334, only **10 of 554** tool executions (1.8%) recorded `tool_success: false`, yet the session was dominated by friction — 5-minute t...
- ref: `.ticket/tickets/84c7757d-9819-4b06-a669-e62366db250a/ticket.toml`

<!-- ticket-index:entry id=6549b6a7-8957-4df0-ada5-8fefb49c015c slug=done/agent-tooling digest=4bc946c9ad59 -->
#### [6549b6a7] Session store: record per-turn/per-sub-agent token and cost with model attribution
- summary: Session 51701334 (instruction-file migration) cost ~$9 total; the orchestrator model used only ~$0.90, so ~90% of spend went to sub-agents. This spend is currently **unattributable from the session s...
- ref: `.ticket/tickets/6549b6a7-8957-4df0-ada5-8fefb49c015c/ticket.toml`

<!-- ticket-index:entry id=77eb143b-0322-4c91-b3c4-deccc2b2927c slug=done/agent-tooling digest=b29bb464a65c -->
#### [77eb143b] [delegation-cost] Enforce MCP-over-shell in agent templates: 116 of 298 terminal calls were substitutable
- priority: `high`
- summary: `run_in_terminal` was the single most-used tool in both analysed sessions — 177 calls in `3e9bc20b`, 121 in `41966513`. Classifying every command shows most of it duplicated capabilities already load...
- ref: `.ticket/tickets/77eb143b-0322-4c91-b3c4-deccc2b2927c/ticket.toml`

<!-- ticket-index:entry id=46d8b25d-e80c-4170-9601-1c26a7a0bcb8 slug=done/agent-tooling digest=a2ae94e6684c -->
#### [46d8b25d] [delegation-cost] Move quality gates before dispatch: ~130 turns of rework came from post-delegation blocks
- priority: `high`
- summary: Mechanism chosen (user-mandated)**: cheap gate sub-agent, MANDATORY — the "narrow orchestrator tool grant" fallback is explicitly rejected in [pre-dispatch-gates.instructions.md](../../.agents/instru...
- ref: `.ticket/tickets/46d8b25d-e80c-4170-9601-1c26a7a0bcb8/ticket.toml`

<!-- ticket-index:entry id=cc3324c9-1da4-4a21-90d0-4128074108b2 slug=done/agent-tooling digest=6d99512cc454 -->
#### [cc3324c9] [delegation-cost] Pass a shared context bundle to fan-out siblings instead of each rediscovering it
- priority: `high`
- summary: Sub-agents are spawned with no shared context, so each rediscovers the same artifacts independently. The orchestrator already holds the digest and does not pass it down.
- ref: `.ticket/tickets/cc3324c9-1da4-4a21-90d0-4128074108b2/ticket.toml`

<!-- ticket-index:entry id=cd19fed4-44d5-4ef0-848c-19753f1539b0 slug=done/agent-tooling digest=7697e5e388d6 -->
#### [cd19fed4] [delegation-cost] Scope MCP tool grants per agent template and restore lazy tool discovery for sub-agents
- priority: `high`
- summary: Review pass: all 5 ACs met; template MCP tool grants are verified.
- ref: `.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0/ticket.toml`

<!-- ticket-index:entry id=fb14754e-2be8-40a5-a995-488842ba6367 slug=done/agent-tooling digest=3bf8f9f608f6 -->
#### [fb14754e] [delegation-cost][handoff] Carry verified physical repo paths in handoff packages and delegation prompts
- priority: `high`
- summary: Validated against committed tree (memory-api fcef868 + root f960037c). `cargo test -p session-api`: 195 passed (10 suites), 0 failed.
- ref: `.ticket/tickets/fb14754e-2be8-40a5-a995-488842ba6367/ticket.toml`

<!-- ticket-index:entry id=66acb737-71d6-4585-a921-b597f7c88e8e slug=done/agent-tooling digest=ae9f60715027 -->
#### [66acb737] [delegation-cost][model-routing] Declare `model:` per agent template; all 24 delegations ran on the same tier
- priority: `high`
- summary: Added a `model:` frontmatter field (bare model_id, no vendor suffix) to all 16 `.agents/agents/*.agent.md` templates:
- ref: `.ticket/tickets/66acb737-71d6-4585-a921-b597f7c88e8e/ticket.toml`

<!-- ticket-index:entry id=10d21210-7168-4ed4-8e99-f6fb0e6e08db slug=done/agent-tooling digest=087e6c48816d -->
#### [10d21210] [delegation-cost][verification] Define a synthetic benchmark session with a checked-in baseline
- priority: `high`
- summary: Nine acceptance criteria across epic `79c4ac3e` and its children are phrased as *"on a comparable follow-up session"* or *"a replayed equivalent of either session"*. Neither phrase is defined. There ...
- ref: `.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml`

<!-- ticket-index:entry id=9c29d697-f782-4737-aea1-645abf75cfb9 slug=done/agent-tooling digest=cb53ee55e481 -->
#### [9c29d697] [mcp-cost-gate] Fix substring over-match in tool_cost() (exact-match-first)
- summary: `tool_cost()` in memory-api/tools/mcp/mcp-cost-gate/src/gate.rs (lines 197-200) uses bidirectional substring matching between the requested tool name and rollup tool names:
- ref: `.ticket/tickets/9c29d697-f782-4737-aea1-645abf75cfb9/ticket.toml`

<!-- ticket-index:entry id=56be2eaa-6b32-4291-857a-b2c1aa24f273 slug=done/agent-tooling digest=9dd7ec9e16aa -->
#### [56be2eaa] [mcp-cost-gate] Integration test harness for live expensive-model gating verification
- summary: Add a live/integration verification path for mcp-cost-gate so that expensive-model gating can be verified end-to-end. Today mcp-cost-gate is an MCP stdio proxy with no standalone verdict CLI, which i...
- ref: `.ticket/tickets/56be2eaa-6b32-4291-857a-b2c1aa24f273/ticket.toml`

<!-- ticket-index:entry id=574560bf-6675-49a1-847c-769b9215071e slug=done/agent-tooling digest=febf4a1791ff -->
#### [574560bf] [session-api] session_tool_metrics hard-fails the whole store on one session dir missing session.json
- priority: `high`
- summary: `session_tool_metrics` hard-fails for the entire workspace store when any single session directory is missing `session.json`.
- ref: `.ticket/tickets/574560bf-6675-49a1-847c-769b9215071e/ticket.toml`

<!-- ticket-index:entry id=4f066c96-b398-4aba-93f1-7d0fd4da39ba slug=done/agent-tooling digest=89dcf8617345 -->
#### [4f066c96] [token-efficiency] Add compact terminal MCP tool
- priority: `high`
- summary: Implement a compact terminal MCP tool that returns short outputs inline and truncates long outputs automatically.
- ref: `.ticket/tickets/4f066c96-b398-4aba-93f1-7d0fd4da39ba/ticket.toml`

<!-- ticket-index:entry id=65819900-1d16-4c53-8b5d-7548c64a75ef slug=done/agent-tooling digest=dec11fb52e3a -->
#### [65819900] [token-efficiency] Add interface skeletonization utility
- priority: `medium`
- summary: Create an interface skeletonization utility that strips implementation bodies and returns only structural information.
- ref: `.ticket/tickets/65819900-1d16-4c53-8b5d-7548c64a75ef/ticket.toml`

<!-- ticket-index:entry id=d4605cc0-5901-4b68-94d5-e7e3e6cac06f slug=done/agent-tooling digest=2c6721660bea -->
#### [d4605cc0] [token-efficiency] Add token-bounded file inspection utility
- priority: `medium`
- summary: Create a token-bounded file inspection utility that defaults to narrow line windows instead of whole-file reads.
- ref: `.ticket/tickets/d4605cc0-5901-4b68-94d5-e7e3e6cac06f/ticket.toml`

<!-- ticket-index:entry id=72c1e92d-65e1-445b-9365-e3384d9da088 slug=done/agent-tooling digest=00ab8fe6747d -->
#### [72c1e92d] [token-efficiency] Generate static `.agent/repo_map.toon`
- priority: `high`
- summary: Add a generated root-level `repo_map.toon` workspace map for low-token structural awareness.
- ref: `.ticket/tickets/72c1e92d-65e1-445b-9365-e3384d9da088/ticket.toml`

<!-- ticket-index:entry id=06cfe998-c2e1-48a4-83e9-11e85e7c40f4 slug=done/agent-tooling digest=66f0b91f9560 -->
#### [06cfe998] [token-efficiency] Introduce peek-api with peek-cli and peek-mcp transport layers
- priority: `high`
- summary: Introduce a proper `peek-api` library crate and move the current `peek-cli` logic behind the repository’s standard `*-api` layering so `peek-cli` and a new `peek-mcp` become thin transport adapters.
- ref: `.ticket/tickets/06cfe998-c2e1-48a4-83e9-11e85e7c40f4/ticket.toml`

<!-- ticket-index:entry id=f93e5db5-4f20-4e23-8832-498c4591938f slug=done/agent-tooling digest=3063b2e7060f -->
#### [f93e5db5] [token-efficiency] Replace repo_map Python generation with peek-api folder skeleton tree output
- priority: `high`
- summary: Replace the current Python-based repo-map generation flow with a repo-aware `peek-api` skeleton/tree renderer that can accept a folder path, apply compaction/filtering rules, and emit a tree-shaped s...
- ref: `.ticket/tickets/f93e5db5-4f20-4e23-8832-498c4591938f/ticket.toml`

<!-- ticket-index:entry id=bd5e9aee-f89b-4d38-be80-80d6c8c1a3b5 slug=done/agent-tooling digest=1eada76da987 -->
#### [bd5e9aee] compact-terminal: extract compact-terminal-api and add CLI transport
- summary: Parent epic: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
- ref: `.ticket/tickets/bd5e9aee-f89b-4d38-be80-80d6c8c1a3b5/ticket.toml`

<!-- ticket-index:entry id=244c3113-e28f-44d7-b9a8-f5dd45d2895c slug=done/agent-tooling digest=c6fd4bc942b6 -->
#### [244c3113] filesystem operations: bounded list/stat/move tool suite (api + cli + mcp)
- summary: Crate names**: `fs-api` (api crate); `fs-cli` (CLI transport); `fs-mcp` (MCP transport).
- ref: `.ticket/tickets/244c3113-e28f-44d7-b9a8-f5dd45d2895c/ticket.toml`


### Component: agent-workflow

<!-- ticket-index:entry id=f3cc69a4-03de-4b45-8b87-a548d5669afe slug=done/agent-workflow digest=1beb26706446 -->
#### [f3cc69a4] [agents] Add Teacher Agent lesson guidance
- priority: `high`
- summary: Create a user-invocable Teacher Agent that turns a problem, system, or learning
- ref: `.ticket/tickets/f3cc69a4-03de-4b45-8b87-a548d5669afe/ticket.toml`

<!-- ticket-index:entry id=9f617940-a3fd-4990-b3fd-a3fa95c10ae7 slug=done/agent-workflow digest=d62c2c9733cb -->
#### [9f617940] [agents] Make Explainer Agent German-first and interactive
- priority: `high`
- summary: Evolve the existing Explainer Agent into a German-first, language-adaptive,
- ref: `.ticket/tickets/9f617940-a3fd-4990-b3fd-a3fa95c10ae7/ticket.toml`

<!-- ticket-index:entry id=9577b114-ec11-431b-8740-c488bef05fc9 slug=done/agent-workflow digest=a435c5c2c30a -->
#### [9577b114] [session-bootstrap][handoff] Require durable session identity and exact resume flow in /handoff
- priority: `high`
- summary: Update the generated `/handoff` prompt contract so every handoff persists and carries durable session identity into the receiving run.
- ref: `.ticket/tickets/9577b114-ec11-431b-8740-c488bef05fc9/ticket.toml`


### Component: audit-api

<!-- ticket-index:entry id=95d4f986-b81c-4951-bae5-4227f2d72a6d slug=done/audit-api digest=1d183fd45113 -->
#### [95d4f986] [audit-api] Include dependency convergence findings in default repo audit
- priority: `high`
- summary: The default repo audit currently validates only a narrow slice of ticket dependency topology. The shipped audit spec covers orphan tickets, and this ticket started as a follow-up to surface raw depen...
- ref: `memory-viewers/.ticket/tickets/95d4f986-b81c-4951-bae5-4227f2d72a6d/ticket.toml`

<!-- ticket-index:entry id=a762448e-464c-43da-95b8-e49eb07814ed slug=done/audit-api digest=5aa3e5c27e0b -->
#### [a762448e] [audit-api] Require every ticket to participate in dependency graph
- priority: `high`
- summary: Add an audit validation rule that flags tickets with neither outgoing depends_on edges nor incoming dependees so every ticket participates in the ticket graph. For legitimately standalone work, creat...
- ref: `memory-api/.ticket/tickets/a762448e-464c-43da-95b8-e49eb07814ed/ticket.toml`

<!-- ticket-index:entry id=855a1e5d-d998-4caf-b60c-d75a13ca3264 slug=done/audit-api digest=b19a14b6f372 -->
#### [855a1e5d] [memory-index] Audit store status summary generator
- priority: `medium`
- summary: Build a generator that reads the audit-api and emits a compact markdown summary of the current audit status at `.audit/README.md` along with its TOON sidecar at `.audit/index.toon`. The purpose is to...
- ref: `.ticket/tickets/855a1e5d-d998-4caf-b60c-d75a13ca3264/ticket.toml`


### Component: cli

<!-- ticket-index:entry id=b5a42a5f-c7f9-4f5b-95f3-f416f958ea3f slug=done/cli digest=597bf709767f -->
#### [b5a42a5f] Plan: integration test harness — tools/context-cli/tests/ (75 tests)
- summary: tags: `#plan` `#testing` `#integration` `#context-api` `#context-cli` `#context-read`
- ref: `.ticket/tickets/b5a42a5f-c7f9-4f5b-95f3-f416f958ea3f/ticket.toml`

<!-- ticket-index:entry id=dde37b0b-6e67-4af0-b04c-ce2e81dac529 slug=done/cli digest=a4d6d156d237 -->
#### [dde37b0b] [bootstrap] add first-class batch commands for bulk ticket/edge operations
- ref: `.ticket/tickets/dde37b0b-6e67-4af0-b04c-ce2e81dac529/ticket.toml`


### Component: context-editor

<!-- ticket-index:entry id=55e30448-e34d-423f-9625-b32425cfbfdc slug=done/context-editor digest=fcd4fca2c0e5 -->
#### [55e30448] 3D UI Panels: Glass SDF Elements in Voxel-Splatted World
- priority: `high`
- summary: > **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
- ref: `.ticket/tickets/55e30448-e34d-423f-9625-b32425cfbfdc/ticket.toml`

<!-- ticket-index:entry id=0cfa8404-ca21-4761-b754-54bfd3c174f1 slug=done/context-editor digest=cb9d73dfaade -->
#### [0cfa8404] Arch: context-editor crate scaffold — Kernel vs World Crate Split
- priority: `critical`
- summary: The context-editor must not be a monolithic crate. It requires a high-performance separation between the **Engine (Kernel)** and the **Game Logic (World-Crate)**. The Kernel provides heavy infrastruc...
- ref: `.ticket/tickets/0cfa8404-ca21-4761-b754-54bfd3c174f1/ticket.toml`

<!-- ticket-index:entry id=4cf4eeb7-2421-438a-a05c-f081125a3617 slug=done/context-editor digest=d71d7d897b08 -->
#### [4cf4eeb7] Character: First-Person Camera Controller with SVO-Derived Rapier Collision
- priority: `high`
- summary: The user navigates the 3D Voxel-splatted world via a first-person character controller. Physics (gravity, ground detection, collision response) is handled by bevy_rapier3d with collision geometry der...
- ref: `.ticket/tickets/4cf4eeb7-2421-438a-a05c-f081125a3617/ticket.toml`

<!-- ticket-index:entry id=c945101a-68b3-48a8-9b03-051241fa2683 slug=done/context-editor digest=f67ca26acbce -->
#### [c945101a] Context Graph 3D: Hypergraph Nodes as Voxel Clusters Generating Splats
- priority: `high`
- summary: The context-engine hypergraph is visualized in 3D. Each graph node becomes a voxel cluster in the SVO, and the splat generation pipeline produces splats from these voxels — nodes appear as soft, volu...
- ref: `.ticket/tickets/c945101a-68b3-48a8-9b03-051241fa2683/ticket.toml`

<!-- ticket-index:entry id=656c3673-f066-4241-a21f-b020c427fc27 slug=done/context-editor digest=50d7ac0f1782 -->
#### [656c3673] Dioxus–Taffy Bridge: 2D UI Panels Composited Over Voxel-Splatted Scene
- priority: `high`
- summary: The 2D HUD/panel layer (Dioxus virtual DOM → Taffy layout) must composite over the 3D Voxel-splatted scene. The bridge renders 2D UI to a texture that is alpha-blended on top of the final tiled raste...
- ref: `.ticket/tickets/656c3673-f066-4241-a21f-b020c427fc27/ticket.toml`

<!-- ticket-index:entry id=0bc0d12f-0670-4a56-8498-c1d5a0237f5e slug=done/context-editor digest=2f16316248d0 -->
#### [0bc0d12f] Impl: Code file viewer — syntax highlighting, source navigation
- priority: `high`
- summary: Source code files are displayed as glass panels in the 3D Voxel-splatted world. Code panels use moderate roughness — enough frosting to keep syntax-highlighted text readable, but transparent enough t...
- ref: `.ticket/tickets/0bc0d12f-0670-4a56-8498-c1d5a0237f5e/ticket.toml`

<!-- ticket-index:entry id=c0386fb6-2f5d-4f13-a189-5d068b72ba26 slug=done/context-editor digest=324d6a81fd8a -->
#### [c0386fb6] Impl: Documentation editor — markdown, doc-viewer API integration
- priority: `high`
- summary: Documentation pages (from doc-viewer / MCP doc sources) are displayed as frosted glass panels in the 3D Voxel-splatted scene. Docs are read-heavy, so they use higher roughness for readability — the m...
- ref: `.ticket/tickets/c0386fb6-2f5d-4f13-a189-5d068b72ba26/ticket.toml`

<!-- ticket-index:entry id=f7340416-1d2f-4ff9-b27e-237958ac00ca slug=done/context-editor digest=550467dceff0 -->
#### [f7340416] Impl: Physics simulation and world environment system
- priority: `high`
- summary: > **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
- ref: `.ticket/tickets/f7340416-1d2f-4ff9-b27e-237958ac00ca/ticket.toml`

<!-- ticket-index:entry id=7132eaae-039c-4ad6-8cfb-29e6694c9ff1 slug=done/context-editor digest=286712c8fa4e -->
#### [7132eaae] Impl: Ticket editor — ticket-api CRUD, SSE, dependency graph 3D
- priority: `high`
- summary: Ticket data from the ticket-api is rendered as interactive 3D panels within the Voxel-splatted world. Each ticket becomes a glass SDF panel (T10) displaying ticket fields, with dependency edges visua...
- ref: `.ticket/tickets/7132eaae-039c-4ad6-8cfb-29e6694c9ff1/ticket.toml`

<!-- ticket-index:entry id=2da63ae2-49a4-47fc-9fd7-72f27f3a3a33 slug=done/context-editor digest=ede083656073 -->
#### [2da63ae2] Multiplayer Latency Compensation: Client-Side Prediction, Rollback & Hermite SDF Ghosting
- priority: `high`
- ref: `.ticket/tickets/2da63ae2-49a4-47fc-9fd7-72f27f3a3a33/ticket.toml`

<!-- ticket-index:entry id=4a7a4323-594e-437f-bf78-db9f92a112fb slug=done/context-editor digest=9e469f5035c2 -->
#### [4a7a4323] Particle System & Physics Effects (Rollup)
- priority: `high`
- summary: The engine needs a highly optimized particle system to depict voxel destruction, magic skills, and environmental effects. It must be governed by a generalized Force Compute scheme (explosions, vortic...
- ref: `.ticket/tickets/4a7a4323-594e-437f-bf78-db9f92a112fb/ticket.toml`

<!-- ticket-index:entry id=530e1bea-7502-45e9-93c8-83a7a48ba124 slug=done/context-editor digest=942339d75025 -->
#### [530e1bea] Rendering: Bevy Render Graph, Double-Buffered SVO, and Voxel Splatting Pipeline
- priority: `high`
- summary: > **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
- ref: `.ticket/tickets/530e1bea-7502-45e9-93c8-83a7a48ba124/ticket.toml`

<!-- ticket-index:entry id=14cf7364-f03c-4ecf-9771-4f7bc81a5bcd slug=done/context-editor digest=96a6ffaa7309 -->
#### [14cf7364] Rendering: Voxel Splatting from SVO — Ray-Box SDF Kernel, EWA Filtering, PBR & Tiled Forward+ Rasterization
- priority: `high`
- summary: > **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
- ref: `.ticket/tickets/14cf7364-f03c-4ecf-9771-4f7bc81a5bcd/ticket.toml`

<!-- ticket-index:entry id=de697c71-9328-4133-8c77-d01e8b885940 slug=done/context-editor digest=096a24ae54f6 -->
#### [de697c71] Runtime Parameters: Voxel Splatting, Tiling, Sorting, and Double Buffer Config
- priority: `high`
- summary: All rendering pipeline parameters must be tweakable at runtime via a Bevy resource. This includes SVO parameters, splat generation, AABB screen projection, GPU radix sort, tiled rasterizer, glass eff...
- ref: `.ticket/tickets/de697c71-9328-4133-8c77-d01e8b885940/ticket.toml`

<!-- ticket-index:entry id=947d0a30-1731-445e-a438-2eeab6b1c5bf slug=done/context-editor digest=20e39400acc9 -->
#### [947d0a30] Style: Theme Palette Driving SVO Materials, PBR Parameters, and Glass Tints
- priority: `high`
- summary: All visual elements — SVO voxel colors, PBR roughness/metallic parameters, glass panel tints, particle colors, lighting — must be driven by a single `ThemePalette` Bevy resource for runtime re-themin...
- ref: `.ticket/tickets/947d0a30-1731-445e-a438-2eeab6b1c5bf/ticket.toml`

<!-- ticket-index:entry id=1e7a290c-0571-490e-9010-48840f28159f slug=done/context-editor digest=c8150146d17b -->
#### [1e7a290c] VFX: Liquid Glass — SDF Refraction of Voxel Splats with Chromatic Aberration, Caustics, and Mipmap Blur
- priority: `high`
- summary: > **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
- ref: `.ticket/tickets/1e7a290c-0571-490e-9010-48840f28159f/ticket.toml`

<!-- ticket-index:entry id=6851d03f-692e-4fc0-ada3-08480ecced6e slug=done/context-editor digest=9a7a12657425 -->
#### [6851d03f] Voxel Splat PBR Material System: Cook-Torrance/GGX, Compact u32 Material Encoding, BRDF LUT & Soft Shadows
- priority: `high`
- summary: Every visible voxel splat needs physically-based shading. The material is packed into a single `u32` in the SVO's `color_data` field. This ticket provides the shared WGSL functions that the tiled ras...
- ref: `.ticket/tickets/6851d03f-692e-4fc0-ada3-08480ecced6e/ticket.toml`

<!-- ticket-index:entry id=6d7ddc8e-ba62-4ee9-9358-9d6d0642ab1e slug=done/context-editor digest=9770d32d3b07 -->
#### [6d7ddc8e] World Editor: Voxel Paint/Carve with Live Splat Regeneration
- priority: `high`
- summary: > **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
- ref: `.ticket/tickets/6d7ddc8e-ba62-4ee9-9358-9d6d0642ab1e/ticket.toml`

<!-- ticket-index:entry id=5e87d2e3-d1cd-4b6c-932a-15b5d1393651 slug=done/context-editor digest=db403e0897b4 -->
#### [5e87d2e3] [context-editor][SVO-RM] Phase 3a: Full PBR Lighting and Depth Buffer Integration
- summary: Phase 1b uses simplified Lambertian + ambient shading. This ticket upgrades to full Cook-Torrance PBR (matching the quality of the old tiled rasterizer) and integrates the ray march output with Bevy'...
- ref: `.ticket/tickets/5e87d2e3-d1cd-4b6c-932a-15b5d1393651/ticket.toml`

<!-- ticket-index:entry id=5eea3447-151b-4f92-9dbf-e89280122d29 slug=done/context-editor digest=ac60427822af -->
#### [5eea3447] [context-editor][SVO-RM] Phase 3b: Remove Tiled Forward+ Pipeline
- summary: Once the SVO ray march pipeline is fully functional — with PBR lighting, depth buffer output, and secondary rays all working (Phases 1b through 3a) — the old Tiled Forward+ pipeline becomes dead code...
- ref: `.ticket/tickets/5eea3447-151b-4f92-9dbf-e89280122d29/ticket.toml`

<!-- ticket-index:entry id=86de425a-f7cc-4cfb-9a1d-582bc4e352cb slug=done/context-editor digest=cc650b60f5f0 -->
#### [86de425a] [context-editor][SVO-RM] Phase 4a: Frustum Culling, Paged SVO Upload and Virtual Address Table
- summary: After the old pipeline is removed (Phase 3b), the renderer runs entirely on the SVO ray march shader using the existing `SvoDoubleBuffer`, whose GPU buffer is sized for the **entire** octree — all no...
- ref: `.ticket/tickets/86de425a-f7cc-4cfb-9a1d-582bc4e352cb/ticket.toml`

<!-- ticket-index:entry id=70d37471-bb9e-4100-b239-c5da156cbe2a slug=done/context-editor digest=0f855537f086 -->
#### [70d37471] [context-editor][SVO-RM] Phase 4b: LOD Cutoff
- summary: With paging in place (Phase 4a), the ray march shader can handle arbitrarily large worlds. However, distant terrain is still traversed to full depth, wasting GPU cycles on sub-pixel detail. This tick...
- ref: `.ticket/tickets/70d37471-bb9e-4100-b239-c5da156cbe2a/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=46d89aa2-043a-4c94-8213-2f365aa2d517 slug=done/context-engine digest=45fcc6da15c7 -->
#### [46d89aa2] Add handoff workflow prompts
- priority: `medium`
- summary: Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session jumpstart handoffs. Scope includes rule-target config, canonical prompt rule entries, generated pr...
- ref: `.ticket/tickets/46d89aa2-043a-4c94-8213-2f365aa2d517/ticket.toml`

<!-- ticket-index:entry id=f9f46954-0a11-4450-a8a9-f3be6ec969a1 slug=done/context-engine digest=5c1ecda45cc4 -->
#### [f9f46954] [context-engine] Fix VS Code Copilot hook file path
- priority: `medium`
- summary: Restore the VS Code Copilot hook configuration after the hook file was renamed from `.github/hooks/docs-validation.json` to `.github/hooks/hooks.json` in this checkout.
- ref: `.ticket/tickets/f9f46954-0a11-4450-a8a9-f3be6ec969a1/ticket.toml`

<!-- ticket-index:entry id=0dba399a-4691-4173-b921-17e5e6f6ebb8 slug=done/context-engine digest=4acbba52ee72 -->
#### [0dba399a] [memory-index] Define IndexEntry schema and serde contract
- priority: `high`
- summary: Define the canonical `IndexEntry` schema used by every generated memory-api store index artifact. `IndexEntry` represents a single entity captured in a domain index — a ticket, spec, rule, audit find...
- ref: `.ticket/tickets/0dba399a-4691-4173-b921-17e5e6f6ebb8/ticket.toml`

<!-- ticket-index:entry id=98bc6b1c-fe7e-4c5f-b0a3-b05586f442e0 slug=done/context-engine digest=788e6a1b03e8 -->
#### [98bc6b1c] [memory-index] Define benchmarking and profiling plan for store-index generation
- priority: `high`
- summary: The current track mentions profiling in passing, but there is no dedicated plan for how generator latency will be benchmarked and profiled across domains. Without a concrete performance plan, pre-com...
- ref: `.ticket/tickets/98bc6b1c-fe7e-4c5f-b0a3-b05586f442e0/ticket.toml`

<!-- ticket-index:entry id=94c56f3d-774a-4b55-a13e-69c782ce9707 slug=done/context-engine digest=e991a79e1ba1 -->
#### [94c56f3d] [memory-index] Define domain-owned thin generator architecture for store indexes
- priority: `high`
- summary: The current memory-index track still reads as if generator logic can live centrally inside `memory-api`. That violates separation of concerns: `memory-api` is the generic backend library and must not...
- ref: `.ticket/tickets/94c56f3d-774a-4b55-a13e-69c782ce9707/ticket.toml`

<!-- ticket-index:entry id=52dfd793-6fd4-463f-8c0e-7a8e5c67dd48 slug=done/context-engine digest=b46b4981ebc8 -->
#### [52dfd793] [memory-index] Define git hook automation for store-index regeneration
- priority: `high`
- summary: The memory-index track requires automatic regeneration of `.ticket/README.md`, `.ticket/index.toon`, `.spec/index.toon`, `.rule/index.toon`, `.audit/index.toon`, workspace summaries, and `.agents/` h...
- ref: `.ticket/tickets/52dfd793-6fd4-463f-8c0e-7a8e5c67dd48/ticket.toml`

<!-- ticket-index:entry id=d3a95908-fc43-4bbe-9572-998cc61d9102 slug=done/context-engine digest=9405fd1ef3d6 -->
#### [d3a95908] [memory-index] Define peek-cli and level-of-detail validation for generated indexes
- priority: `high`
- summary: The current track does not define how generated index artifacts will be validated for efficient agent consumption with `peek-cli`. That leaves a major integration question unanswered: whether the gen...
- ref: `.ticket/tickets/d3a95908-fc43-4bbe-9572-998cc61d9102/ticket.toml`

<!-- ticket-index:entry id=db667eed-f507-49ee-b1b6-b7b3edca98ce slug=done/context-engine digest=03e6b5f120b7 -->
#### [db667eed] [memory-index] Define shared rendering pipeline integration for generated indexes
- priority: `high`
- summary: The track currently treats generated index rendering as an isolated effort. There is no plan for how store-index generation should integrate with the existing `rule-api` rendering pipeline or with fu...
- ref: `.ticket/tickets/db667eed-f507-49ee-b1b6-b7b3edca98ce/ticket.toml`

<!-- ticket-index:entry id=e7a0ee3c-dc2f-42dd-8c02-5070a747c156 slug=done/context-engine digest=1d2f52f7781f -->
#### [e7a0ee3c] [memory-index] IndexEntry TOON sidecar format and validator
- priority: `high`
- summary: Design and implement the compact machine-readable TOON sidecar emitted alongside every memory-api store index README. This sidecar is the primary surface for similarity search, RAG retrieval, and aut...
- ref: `.ticket/tickets/e7a0ee3c-dc2f-42dd-8c02-5070a747c156/ticket.toml`

<!-- ticket-index:entry id=456d9b69-ec43-4746-b47d-20704da01be9 slug=done/context-engine digest=c00f19283b6d -->
#### [456d9b69] [sandbox-v1][plan] define functional sandbox orchestration v1
- priority: `high`
- summary: Define and refine the first functional sandbox orchestration slice before implementation begins.
- ref: `.ticket/tickets/456d9b69-ec43-4746-b47d-20704da01be9/ticket.toml`

<!-- ticket-index:entry id=b6af9f40-e1f7-4f68-92e7-0a063a4ac020 slug=done/context-engine digest=4b070f38af22 -->
#### [b6af9f40] [workflow][session-worktree] Default worktree-backed session workflow
- priority: `high`
- summary: Track the migration to a default worktree-backed session workflow so parallel agent sessions no longer share one staging area.
- ref: `.ticket/tickets/b6af9f40-e1f7-4f68-92e7-0a063a4ac020/ticket.toml`

<!-- ticket-index:entry id=68a49ca7-a6f6-42a8-b820-0a86e6a4de2e slug=done/context-engine digest=8819349d5c1a -->
#### [68a49ca7] [workflow][session-worktree] Plan default worktree-backed session workflow
- priority: `high`
- summary: Turn `context-engine/session-worktree-default-workflow` (`2860a8db-0c4e-4e94-984a-c10a72a67ffc`) into the concrete planning contract for the default worktree-backed session path.
- ref: `.ticket/tickets/68a49ca7-a6f6-42a8-b820-0a86e6a4de2e/ticket.toml`


### Component: context-read

<!-- ticket-index:entry id=ad29a401-f4c5-4f76-bcaa-905dcfe5a966 slug=done/context-read digest=aca4c1474b5f -->
#### [ad29a401] Design: Root update steps — ExpansionCtx commit/overlap
- summary: tags: `#design` `#RootManager` `#ExpansionCtx` `#commit` `#overlap` `#reading`
- ref: `.ticket/tickets/ad29a401-f4c5-4f76-bcaa-905dcfe5a966/ticket.toml`

<!-- ticket-index:entry id=73d1bf88-8f61-431b-bc2d-d36c523f8f03 slug=done/context-read digest=ccd6809596cf -->
#### [73d1bf88] Plan: align context-read normalization/materialization tests with clarified contract
- summary: Align the context-read failing tests with the clarified normalization/materialization contract before wider algorithm edits proceed.
- ref: `.ticket/tickets/73d1bf88-8f61-431b-bc2d-d36c523f8f03/ticket.toml`

<!-- ticket-index:entry id=51cfdc74-4454-481d-91af-1d94b4934dec slug=done/context-read digest=174eebb04f7a -->
#### [51cfdc74] Spec: context-read worked traces for overlap progression
- summary: Add step-by-step worked traces to the existing `read_sequence` / `context-read pipeline` spec chain so the overlap algorithm is specified through concrete iterations, variable transitions, and commit...
- ref: `.ticket/tickets/51cfdc74-4454-481d-91af-1d94b4934dec/ticket.toml`

<!-- ticket-index:entry id=b78a21bd-de8f-407f-a9b8-2664019240da slug=done/context-read digest=898e88bb3316 -->
#### [b78a21bd] [context-read] Replace bespoke postfix search with trace-owned longest postfix paths
- priority: `high`
- summary: `ExpansionCtx` reimplements postfix search with `collect_postfix_candidates`, `collect_postfix_candidates_inner`, and `find_postfix_path`. That search ignores the graph invariant that each node alrea...
- ref: `.ticket/tickets/b78a21bd-de8f-407f-a9b8-2664019240da/ticket.toml`


### Component: context-stack

<!-- ticket-index:entry id=7937930a-e184-41eb-9732-7ac39897d263 slug=done/context-stack digest=0bf1056f892c -->
#### [7937930a] Add branch-root rewrite mode to crane-cli
- priority: `medium`
- summary: The current `crane-cli` mapping model requires a non-empty destination path. That is sufficient for direct source-to-destination transplants, but it does not yet support collapsing a filtered subtree...
- ref: `.ticket/tickets/7937930a-e184-41eb-9732-7ac39897d263/ticket.toml`

<!-- ticket-index:entry id=c330a47f-8983-4f9b-b0f1-3beafd118e22 slug=done/context-stack digest=0842a35ac113 -->
#### [c330a47f] Finalize context-stack tool migration handoff
- priority: `high`
- summary: The context-stack-related tools already exist in the standalone `../context-stack` repository under `tools/**`, and the standalone repo has an in-progress integration slice that makes those tools bui...
- ref: `.ticket/tickets/c330a47f-8983-4f9b-b0f1-3beafd118e22/ticket.toml`

<!-- ticket-index:entry id=400f92ff-0f93-46de-a79d-14bf4e2b2ce7 slug=done/context-stack digest=37321d616191 -->
#### [400f92ff] Retarget imported context-stack tool manifests for standalone layout
- priority: `high`
- ref: `.ticket/tickets/400f92ff-0f93-46de-a79d-14bf4e2b2ce7/ticket.toml`

<!-- ticket-index:entry id=17c99c98-9127-4bd0-90b5-c47f990b56de slug=done/context-stack digest=e1a1686a5675 -->
#### [17c99c98] Verify crane-cli against controlled and real dry-run flows
- priority: `high`
- summary: The verification slice now covers the real context-stack migration shape more directly.
- ref: `.ticket/tickets/17c99c98-9127-4bd0-90b5-c47f990b56de/ticket.toml`

<!-- ticket-index:entry id=1dffcf23-8a95-4f45-8163-27e4e58048c7 slug=done/context-stack digest=1a848d8149c9 -->
#### [1dffcf23] [context-stack] Define replayable graph-operation journal format for log-viewer
- priority: `high`
- summary: Define a replayable graph-operation format for context-stack operations that fits log-viewer visualizations and links to trace logs and journals.
- ref: `.ticket/tickets/1dffcf23-8a95-4f45-8163-27e4e58048c7/ticket.toml`


### Component: context-tasks

<!-- ticket-index:entry id=8fe78950-a043-4f23-a5ef-8e4d7fc5c322 slug=done/context-tasks digest=1e279b8583d6 -->
#### [8fe78950] Bug: ticket list search results use synthetic updated_at
- ref: `.ticket/tickets/8fe78950-a043-4f23-a5ef-8e4d7fc5c322/ticket.toml`

<!-- ticket-index:entry id=9676914b-ffbc-41d1-b3dd-f3c8de863a61 slug=done/context-tasks digest=9d718bf73a09 -->
#### [9676914b] Bug: ticket-serve multi-workspace lazy-open misses SSE hook and reconcile
- ref: `.ticket/tickets/9676914b-ffbc-41d1-b3dd-f3c8de863a61/ticket.toml`

<!-- ticket-index:entry id=ab663230-f0af-49eb-9f6e-dac7f819626b slug=done/context-tasks digest=43eb9b4fd1d8 -->
#### [ab663230] Bug: workspace=default returned 404 due to redb double-open in serve dispatch
- ref: `.ticket/tickets/ab663230-f0af-49eb-9f6e-dac7f819626b/ticket.toml`

<!-- ticket-index:entry id=09a32876-665c-476c-9587-8dcb3acd6e6a slug=done/context-tasks digest=e724253bb144 -->
#### [09a32876] Design: SSE event schema freeze for ticket graph updates
- summary: Freeze the SSE payload contract so backend and frontend can implement independently.
- ref: `.ticket/tickets/09a32876-665c-476c-9587-8dcb3acd6e6a/ticket.toml`

<!-- ticket-index:entry id=68dfc679-9eb7-48cd-ade5-a452fdc0f01d slug=done/context-tasks digest=6c67c3c5b2f6 -->
#### [68dfc679] Design: auth token lifecycle and rotation/reload behavior for ticket serve
- summary: Define how ticket serve token auth can rotate safely without process restarts.
- ref: `.ticket/tickets/68dfc679-9eb7-48cd-ade5-a452fdc0f01d/ticket.toml`

<!-- ticket-index:entry id=24aa7e5e-1d62-4f35-a4f7-b056a0b8abce slug=done/context-tasks digest=8ec7e09613b8 -->
#### [24aa7e5e] Design: command-hook emission contract and fallback reconciliation
- summary: Define hook events emitted by ticket mutations and fallback behavior when hooks fail.
- ref: `.ticket/tickets/24aa7e5e-1d62-4f35-a4f7-b056a0b8abce/ticket.toml`

<!-- ticket-index:entry id=e79fdc1f-2bfb-410f-931c-dbb744cd209e slug=done/context-tasks digest=dcbf88212dda -->
#### [e79fdc1f] Design: server-side subgraph query API and pagination semantics
- summary: Define scalable subgraph query semantics for large ticket dependency graphs.
- ref: `.ticket/tickets/e79fdc1f-2bfb-410f-931c-dbb744cd209e/ticket.toml`

<!-- ticket-index:entry id=21a1b9ca-c053-4709-8785-e41fb0661c31 slug=done/context-tasks digest=a5c858822e56 -->
#### [21a1b9ca] Design: ticket HTTP API + SSE event contract + auth model
- summary: Define the contract for ticket server APIs and live graph updates with required auth.
- ref: `.ticket/tickets/21a1b9ca-c053-4709-8785-e41fb0661c31/ticket.toml`

<!-- ticket-index:entry id=dd6e20bb-e4ac-4b86-ae1c-9090f3d7fd11 slug=done/context-tasks digest=51b299f3a2c1 -->
#### [dd6e20bb] Feature: edge removal and dependency inversion support for parent-child tickets
- ref: `.ticket/tickets/dd6e20bb-e4ac-4b86-ae1c-9090f3d7fd11/ticket.toml`

<!-- ticket-index:entry id=00ee9f46-7d24-4c3e-8961-00ed760e7ca2 slug=done/context-tasks digest=05ca8b1e9c6d -->
#### [00ee9f46] Impl: auth token reload and runtime reconfiguration for ticket serve
- summary: Wave 1 / Track C2** | Component: `context-tasks`
- ref: `.ticket/tickets/00ee9f46-7d24-4c3e-8961-00ed760e7ca2/ticket.toml`

<!-- ticket-index:entry id=5e68c2e1-e93e-415f-a3c3-c1a396f36395 slug=done/context-tasks digest=85055e87f7d3 -->
#### [5e68c2e1] Impl: live ticket graph stream pipeline (SSE + hooks + conflict events)
- summary: Wave 1 / Track D** | Component: `context-tasks`
- ref: `.ticket/tickets/5e68c2e1-e93e-415f-a3c3-c1a396f36395/ticket.toml`

<!-- ticket-index:entry id=ce8ba16b-db39-44af-a583-3cf830f94d6f slug=done/context-tasks digest=27d36c9714cc -->
#### [ce8ba16b] Impl: remove inbound HTTP auth from ticket serve
- summary: > **Key insight:** The bearer token is not for protecting the ticket HTTP API.
- ref: `.ticket/tickets/ce8ba16b-db39-44af-a583-3cf830f94d6f/ticket.toml`

<!-- ticket-index:entry id=43dedd9b-46cd-46c7-96f8-6683ded2cc4d slug=done/context-tasks digest=08ec9dc1420c -->
#### [43dedd9b] Impl: ticket serve mode (HTTP + auth + workspace-aware ticket endpoints)
- summary: Wave 1 / Track C** | Component: `context-tasks`
- ref: `.ticket/tickets/43dedd9b-46cd-46c7-96f8-6683ded2cc4d/ticket.toml`

<!-- ticket-index:entry id=15dbf903-b97f-4f74-ad03-93e44188eaf0 slug=done/context-tasks digest=ed86667bdfd8 -->
#### [15dbf903] Tech debt: registry.get holds sync mutex during blocking store open
- ref: `.ticket/tickets/15dbf903-b97f-4f74-ad03-93e44188eaf0/ticket.toml`


### Component: doc-api

<!-- ticket-index:entry id=609099ac-c5b5-4fe2-8072-a7b19ff8d75c slug=done/doc-api digest=c842445cdb02 -->
#### [609099ac] [doc-api] Support cargo metadata outputs as docs workspace inputs
- priority: `high`
- summary: Add `cargo metadata` output support to the new docs surface so `doc-api` can use Cargo's workspace and package graph as a docs workspace input.
- ref: `memory-api/.ticket/tickets/609099ac-c5b5-4fe2-8072-a7b19ff8d75c/ticket.toml`


### Component: doc-http

<!-- ticket-index:entry id=4e99c7dd-6e1b-4bce-a8c9-67e5182a4dc3 slug=done/doc-http digest=7df5a3a5e7e8 -->
#### [4e99c7dd] [doc-http] Support cargo doc generated HTML and JSON outputs
- priority: `high`
- summary: Add support for generated `cargo doc` outputs so the docs family can register, describe, and serve Rust documentation HTML and rustdoc JSON artifacts.
- ref: `memory-api/.ticket/tickets/4e99c7dd-6e1b-4bce-a8c9-67e5182a4dc3/ticket.toml`


### Component: doc-viewer

<!-- ticket-index:entry id=391fcd15-0da6-4b39-86f3-19afca688377 slug=done/doc-viewer digest=cda7e1f6c490 -->
#### [391fcd15] [doc-viewer] Rewrite doc-viewer as a Dioxus viewer over doc-http
- priority: `high`
- summary: Implement a concrete doc-viewer migration path that replaces the current Preact-first shell with a Dioxus frontend built on `viewer-api-dioxus` and backed by `doc-http` for its server-facing document...
- ref: `viewer-api/.ticket/tickets/391fcd15-0da6-4b39-86f3-19afca688377/ticket.toml`


### Component: docs

<!-- ticket-index:entry id=71e13480-4f92-418a-a9e6-155f3274f180 slug=done/docs digest=2db7dfc86d09 -->
#### [71e13480] Update agent guidance and rule entries for parts, freezing, and projected reads
- priority: `medium`
- summary: Make the new structure the documented default across agent-facing guidance, so agents stop writing reviews into objectives and start using part-addressed writes and projected reads.
- ref: `memory-api/.ticket/tickets/71e13480-4f92-418a-a9e6-155f3274f180/ticket.toml`


### Component: documentation

<!-- ticket-index:entry id=2fb3adb0-fa3a-41a6-8fd6-38096635a38b slug=done/documentation digest=527c343c6dbd -->
#### [2fb3adb0] [readmes] Smooth repository README surfaces
- priority: `high`
- ref: `.ticket/tickets/2fb3adb0-fa3a-41a6-8fd6-38096635a38b/ticket.toml`


### Component: feedback-api

<!-- ticket-index:entry id=c7542933-3052-45c8-99e6-3e09f40cc9b9 slug=done/feedback-api digest=566e579cc7b9 -->
#### [c7542933] [feedback-api] Core curation surface — URN usage counting + entity ratings (bootstrap gate)
- priority: `high`
- summary: This is the **minimal, self-contained core** of the feedback-api program that session bootstrapping gates on — NOT the full program. It exists so the session-bootstrap epic can depend on a small, shi...
- ref: `memory-api/.ticket/tickets/c7542933-3052-45c8-99e6-3e09f40cc9b9/ticket.toml`

<!-- ticket-index:entry id=6b0002bf-c15b-4cc7-ac38-fbf66a07d1bc slug=done/feedback-api digest=697b598b4427 -->
#### [6b0002bf] [feedback-api][activation] Wire feedback discovery, collection, and analyzer loop (ring activation)
- priority: `high`
- summary: Make the feedback system actually usable and self-driving. The feedback-api crate, schema, transports, and redistributed ring edges exist and compile, but the ring has never fired: 3 of 4 edges have ...
- ref: `memory-api/.ticket/tickets/6b0002bf-c15b-4cc7-ac38-fbf66a07d1bc/ticket.toml`

<!-- ticket-index:entry id=3d4c4739-3138-40be-947d-556e5f7de53a slug=done/feedback-api digest=c39a7493173d -->
#### [3d4c4739] [feedback-api][analyzer] Backtraceable, verifiable follow-up ticket format; re-enable synthesis behind it
- priority: `high`
- summary: The stop-hook (`crates/session-api/src/bin/copilot-capture-hook.rs`) currently only logs structured signals — auto-ticket synthesis was paused after the naive miner created 522 false-positive tickets...
- ref: `memory-api/.ticket/tickets/3d4c4739-3138-40be-947d-556e5f7de53a/ticket.toml`

<!-- ticket-index:entry id=fb6aa078-1f8b-4a76-99bd-3a26190b1208 slug=done/feedback-api digest=1ad44fc537d8 -->
#### [fb6aa078] [feedback-api][design] First-class feedback-api crate boundary and unified FeedbackEntry schema v1
- priority: `high`
- summary: Define and enforce feedback-api crate boundaries, a single versioned FeedbackEntry schema, AND the correct topology of the feedback ring: the ring is NOT a module — it is an emergent distributed loop...
- ref: `memory-api/.ticket/tickets/fb6aa078-1f8b-4a76-99bd-3a26190b1208/ticket.toml`

<!-- ticket-index:entry id=b4954d6c-0451-49ed-8939-11f6568558f5 slug=done/feedback-api digest=a9e746fca8b4 -->
#### [b4954d6c] [feedback-api][ingestion] Mine explicit feedback-ingestion tool calls from structured session metadata
- priority: `high`
- summary: The highest-fidelity feedback signal is an explicit feedback-ingestion tool call the agent already made during a session. It carries an unambiguous target entity + rating + note in structured argumen...
- ref: `memory-api/.ticket/tickets/b4954d6c-0451-49ed-8939-11f6568558f5/ticket.toml`

<!-- ticket-index:entry id=a7601cb7-6c92-4891-aa55-07ab46125bb8 slug=done/feedback-api digest=1840b471c9ea -->
#### [a7601cb7] [feedback-api][provenance] Extend FeedbackProvenance with session/turn/tool-call backtrace refs
- priority: `high`
- summary: `FeedbackProvenance` currently carries only `session_id`, `author`, `executed_at` (crates/feedback-api/src/lib.rs). The removed transcript miner even dropped `session_id` entirely, so mined entries w...
- ref: `memory-api/.ticket/tickets/a7601cb7-6c92-4891-aa55-07ab46125bb8/ticket.toml`

<!-- ticket-index:entry id=16e112a7-5ab0-45a5-87c8-7d89d07ffd16 slug=done/feedback-api digest=94797df4ff08 -->
#### [16e112a7] [feedback-api][signals] Define failed-tool-call to entity mapping and recording policy
- priority: `medium`
- summary: `mine_structured_feedback_signals` now detects `FailedToolCall` signals (`event_meta.tool_success == Some(false)`), but a failed tool call has no inherent target entity. We must decide, with evidence...
- ref: `memory-api/.ticket/tickets/16e112a7-5ab0-45a5-87c8-7d89d07ffd16/ticket.toml`

<!-- ticket-index:entry id=3fa60398-3154-46ee-aca5-8d87541bac1e slug=done/feedback-api digest=0543ab7a41b4 -->
#### [3fa60398] [feedback-api][signals] Deterministic BFS entity queue for structured feedback mining
- priority: `medium`
- summary: Structured feedback mining must iterate discovered entities deterministically. Per the agreed design (decision #2): iterate breadth-first, and only queue newly-discovered entities for detection after...
- ref: `memory-api/.ticket/tickets/3fa60398-3154-46ee-aca5-8d87541bac1e/ticket.toml`


### Component: history

<!-- ticket-index:entry id=77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4 slug=done/history digest=a2caa19385ab -->
#### [77f1eb5c] [bootstrap] wire history, diff, and revert end-to-end
- ref: `memory-api/.ticket/tickets/77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4/ticket.toml`


### Component: instructions

<!-- ticket-index:entry id=33565741-c3ce-4697-91d3-092a803aaac0 slug=done/instructions digest=ec0c2f6acb0b -->
#### [33565741] [ticket-system] Instruction updates: mandatory review gate and diligent state progression
- priority: `high`
- ref: `.ticket/tickets/33565741-c3ce-4697-91d3-092a803aaac0/ticket.toml`


### Component: log-api

<!-- ticket-index:entry id=d3349747-b2f2-4dd4-b73c-dc016fec80d6 slug=done/log-api digest=305c85ebcc99 -->
#### [d3349747] [log-api] Add runtime log session model and cross-store links
- priority: `high`
- summary: Extend `log-api` beyond validation-only captures so it can represent runtime log sessions for tools, servers, tests, benchmarks, graph operations, journals, and agent sessions.
- ref: `.ticket/tickets/d3349747-b2f2-4dd4-b73c-dc016fec80d6/ticket.toml`

<!-- ticket-index:entry id=db9bad13-ae43-4300-8037-7165c0e9a7b0 slug=done/log-api digest=3189966a6eb6 -->
#### [db9bad13] [log-api][test-api][journal] Normalize artifact routing for executions, runtime sessions, and journals
- priority: `high`
- summary: Normalize the artifact-routing contract between `test-api` executions, `log-api` runtime sessions, and operation journals so validation runners, benchmark harnesses, and transport diagnostics all emi...
- ref: `.ticket/tickets/db9bad13-ae43-4300-8037-7165c0e9a7b0/ticket.toml`


### Component: mcp-cost-gate

<!-- ticket-index:entry id=32067e83-7c60-40b7-9d2e-4c419020adcf slug=done/mcp-cost-gate digest=75c7dd510bf2 -->
#### [32067e83] [mcp-cost-gate] Tolerate common caller_model naming deviations instead of rejecting the call
- priority: `high`
- summary: Files touched:
- ref: `.ticket/tickets/32067e83-7c60-40b7-9d2e-4c419020adcf/ticket.toml`


### Component: mcp-cost-gate,model-prices

<!-- ticket-index:entry id=4e7e53f5-b3de-477f-8cbd-f88b6c103bb5 slug=done/mcp-cost-gate,model-prices digest=4c45dda121c9 -->
#### [4e7e53f5] graded cost + model budget + offset resolution
- summary: Implement the graded cost model (1–100 scale) for tool classification and budget-based gating decisions.
- ref: `.ticket/tickets/4e7e53f5-b3de-477f-8cbd-f88b6c103bb5/ticket.toml`


### Component: mcp-toolmon

<!-- ticket-index:entry id=e70471d4-611a-4e8b-a612-5b4573868986 slug=done/mcp-toolmon digest=c87c0cebcae2 -->
#### [e70471d4] Generalize mcp-toolmon path rewriting beyond the workspace argument
- priority: `high`
- summary: `memory-api/tools/mcp/mcp-toolmon/src/proxy.rs` rewrites only `params.arguments.workspace` in the `Decision::Allow` block at lines 452-492. Every other path-bearing argument still resolves against th...
- ref: `.ticket/tickets/e70471d4-611a-4e8b-a612-5b4573868986/ticket.toml`


### Component: mcp-transports

<!-- ticket-index:entry id=2318f63f-7113-4987-87b7-ca26afa04d11 slug=done/mcp-transports digest=ec7d7910300d -->
#### [2318f63f] [mcp] Report unique server identities for local Copilot registrations
- priority: `high`
- summary: Make feedback-mcp, session-mcp, peek-mcp, rule-mcp, audit-mcp, and compact-terminal-mcp report their true package identities in MCP initialize responses so VS Code Copilot can distinguish and activat...
- ref: `memory-api/.ticket/tickets/2318f63f-7113-4987-87b7-ca26afa04d11/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=9faa3f5f-e2e1-469d-994e-1bb8b90d5ab4 slug=done/memory-api digest=bc2d9b11f827 -->
#### [9faa3f5f] [delegation-cost][mcp] Unify `workspace` parameter semantics and make rejections actionable across spec-mcp/test-mcp
- priority: `high`
- summary: `memory-api/src/workspace.rs:32-46` - `InvalidWorkspaceSelector` Display impl
- ref: `.ticket/tickets/9faa3f5f-e2e1-469d-994e-1bb8b90d5ab4/ticket.toml`

<!-- ticket-index:entry id=6c859ac3-14c9-4d9d-b428-5b0cca03e23a slug=done/memory-api digest=803b55472417 -->
#### [6c859ac3] [journal] Define generic operation journal schema and store contract
- priority: `high`
- summary: Generalize the move kernel's journaling concept into a reusable operation journal contract for memory stores.
- ref: `.ticket/tickets/6c859ac3-14c9-4d9d-b428-5b0cca03e23a/ticket.toml`

<!-- ticket-index:entry id=8affb65d-605b-4225-819a-af951e0bd318 slug=done/memory-api digest=5e69dd6272cf -->
#### [8affb65d] [memory-api] Add shared store bootstrap open_or_init helpers
- priority: `high`
- summary: `ticket-viewer` needed a viewer-local workaround to start from a checkout where
- ref: `memory-api/.ticket/tickets/8affb65d-605b-4225-819a-af951e0bd318/ticket.toml`

<!-- ticket-index:entry id=6124971a-0775-455f-a7b8-840766a43ce3 slug=done/memory-api digest=b3830acfd764 -->
#### [6124971a] [memory-api] Canonicalize local store root resolution
- priority: `high`
- summary: Implemented shared workspace/store root normalization for ticket/spec/rule stores, validated ticket create target roots so repo/store paths resolve into .ticket/tickets, removed the ticket workspace ...
- ref: `memory-api/.ticket/tickets/6124971a-0775-455f-a7b8-840766a43ce3/ticket.toml`

<!-- ticket-index:entry id=2e41c96d-fe9f-4cf2-b941-6f0d452f237c slug=done/memory-api digest=61c23af3a350 -->
#### [2e41c96d] [memory-api] Create domain instrumentation and journaling coverage map
- priority: `high`
- summary: Map major operations across memory-domain crates to required spans, events, result summaries, and journal requirements before broad instrumentation begins.
- ref: `.ticket/tickets/2e41c96d-fe9f-4cf2-b941-6f0d452f237c/ticket.toml`

<!-- ticket-index:entry id=026b2eb6-17c6-4d02-b46b-79758f1237a1 slug=done/memory-api digest=8e5481cfb3b9 -->
#### [026b2eb6] [memory-api] E2E test workspace fixture repository — multi-store, multi-submodule
- priority: `high`
- summary: Provide a dedicated, version-controlled **fixture repository** that materializes a realistic multi-store, multi-worktree workspace so every memory tool (ticket, spec, rule, audit, session, test, doc,...
- ref: `memory-api/.ticket/tickets/026b2eb6-17c6-4d02-b46b-79758f1237a1/ticket.toml`

<!-- ticket-index:entry id=756fed27-96b3-4572-a986-a4f70986984a slug=done/memory-api digest=869c6d558273 -->
#### [756fed27] [memory-api] Extract shared tracing initialization for all transports
- priority: `high`
- summary: Starting shared transport tracing implementation: extract a reusable memory-api tracing initializer from current ticket-http/main patterns and wire it into representative HTTP and MCP tools first.
- ref: `.ticket/tickets/756fed27-96b3-4572-a986-a4f70986984a/ticket.toml`

<!-- ticket-index:entry id=0a510279-5482-4c4f-8cb5-fad3baa57427 slug=done/memory-api digest=3f1223f8e32b -->
#### [0a510279] [memory-api] Generalize cross-workspace move into a domain-neutral kernel with per-domain trait specialization
- priority: `high`
- summary: Promote the proven ticket-only cross-workspace move (delivered by `505b2cd4`) into a **domain-neutral generic move kernel** in `memory-api` that every domain store reuses through a trait, with **no d...
- ref: `memory-api/.ticket/tickets/0a510279-5482-4c4f-8cb5-fad3baa57427/ticket.toml`

<!-- ticket-index:entry id=404ddde3-cf6c-4d33-9bb0-43d65c12a3e1 slug=done/memory-api digest=8dd4e497af38 -->
#### [404ddde3] [memory-api] Normalize move journal paths and shrink rewrite rollback payload
- priority: `high`
- summary: A generated move journal mixes Windows and Unix separators in persisted path fields and stores extremely large `previous_content` snapshots for rewritten tracked files.
- ref: `memory-api/.ticket/tickets/404ddde3-cf6c-4d33-9bb0-43d65c12a3e1/ticket.toml`

<!-- ticket-index:entry id=21e6c015-55c6-4807-8d55-16193ed687ed slug=done/memory-api digest=1978a1068fc3 -->
#### [21e6c015] [memory-api] Support cross-git-worktree (submodule) entity moves
- priority: `high`
- summary: Extend the cross-workspace move contract so an entity can be moved between two stores that live in **different git worktrees** — most importantly across **git submodule boundaries**. This repository ...
- ref: `memory-api/.ticket/tickets/21e6c015-55c6-4807-8d55-16193ed687ed/ticket.toml`

<!-- ticket-index:entry id=e6bdafbe-3538-47a3-8837-1f8e74fb13e8 slug=done/memory-api digest=9c98ae8b6a74 -->
#### [e6bdafbe] [memory-api] Track explicit-init-only store creation validation
- priority: `high`
- summary: Resumed explicit-init-only validation track and started active execution.
- ref: `memory-api/.ticket/tickets/e6bdafbe-3538-47a3-8837-1f8e74fb13e8/ticket.toml`

<!-- ticket-index:entry id=0fdce225-9cef-46ed-92d9-83c852c2d084 slug=done/memory-api digest=cec37b954476 -->
#### [0fdce225] [memory-api][cli][mcp] Require explicit workspace for entity creation
- priority: `high`
- ref: `memory-api/.ticket/tickets/0fdce225-9cef-46ed-92d9-83c852c2d084/ticket.toml`

<!-- ticket-index:entry id=94a51f30-8c37-4ea6-b49a-97206d28add3 slug=done/memory-api digest=d61ec73f108d -->
#### [94a51f30] [memory-api][spec-api][rule-api][audit-api] Adopt generic move kernel across domains and expose move on all transports
- priority: `high`
- summary: Now that the domain-neutral move kernel (`0a510279`, `memory_api::storage::move_kernel`) has landed and is proven by the `ticket-api` adapter plus a `spec-api` demonstration adapter, make cross-works...
- ref: `memory-api/.ticket/tickets/94a51f30-8c37-4ea6-b49a-97206d28add3/ticket.toml`

<!-- ticket-index:entry id=a7f19a7d-42d0-48b7-b89b-98de3c6fa3b4 slug=done/memory-api digest=3960869661e9 -->
#### [a7f19a7d] [memory-api][test] Refresh cross-domain move matrix + add move benchmark now that the kernel landed
- priority: `medium`
- summary: The generic move kernel (`0a510279`) has landed, but the cross-domain operation matrix (`memory-matrix`) still reports `move` as **blocked-with-reason** and its test asserts the blocked state. Update...
- ref: `memory-api/.ticket/tickets/a7f19a7d-42d0-48b7-b89b-98de3c6fa3b4/ticket.toml`

<!-- ticket-index:entry id=e6e09d6f-a41c-49f7-bc6a-c6d8e822598b slug=done/memory-api digest=c421b7b32f64 -->
#### [e6e09d6f] [memory-api][ticket-cli][spec-cli][rule-cli] Normalize nested workspace option semantics
- priority: `high`
- summary: The ticket direction is right, but the existing plan was too broad to execute safely. The core contract already lives in the workspace design spec, and the remaining implementation should be treated ...
- ref: `memory-api/.ticket/tickets/e6e09d6f-a41c-49f7-bc6a-c6d8e822598b/ticket.toml`

<!-- ticket-index:entry id=ef0ebf38-7f55-4bd7-bf0c-0b416650ee0b slug=done/memory-api digest=725465787cfa -->
#### [ef0ebf38] [memory-api][ticket-cli][spec-cli][rule-cli] Unify child-workspace resolution across CLI tools
- priority: `high`
- summary: Current child-workspace resolution across the CLI tools is inconsistent even though store selection already uses a shared workspace-root/index-root resolver.
- ref: `memory-api/.ticket/tickets/ef0ebf38-7f55-4bd7-bf0c-0b416650ee0b/ticket.toml`

<!-- ticket-index:entry id=7f7fe4a8-a1d6-44b4-baf9-9500f6db40a5 slug=done/memory-api digest=67637f4a5e0f -->
#### [7f7fe4a8] [memory-index] Define domain digest input contract for generated index entries
- priority: `high`
- summary: `IndexEntry` and `IndexSidecar` already define a generic digest algorithm, but the generator tickets do not define how each domain produces the stable input fields that feed that digest. In particula...
- ref: `.ticket/tickets/7f7fe4a8-a1d6-44b4-baf9-9500f6db40a5/ticket.toml`

<!-- ticket-index:entry id=3ae150e4-8e85-42c0-93cf-971f814cff65 slug=done/memory-api digest=b2bcd2689890 -->
#### [3ae150e4] [rule-cli] Fix repo prompt contract test root resolution
- priority: `medium`
- ref: `memory-api/.ticket/tickets/3ae150e4-8e85-42c0-93cf-971f814cff65/ticket.toml`

<!-- ticket-index:entry id=9491f6b7-c11b-4d94-aed6-f5c6ea004e8a slug=done/memory-api digest=61636de9f0ef -->
#### [9491f6b7] [session-api] Plan and scaffold Copilot chat-session capture in memory-api
- priority: `high`
- summary: Plan and scaffold a bounded first `session-api` slice under `memory-viewers/memory-api` for saving Copilot chat sessions into a memory-api-backed store.
- ref: `.ticket/tickets/9491f6b7-c11b-4d94-aed6-f5c6ea004e8a/ticket.toml`

<!-- ticket-index:entry id=459789f8-12b7-4013-be11-521d5ca23e49 slug=done/memory-api digest=4721c83db800 -->
#### [459789f8] [session-mcp][rule-mcp][feedback-mcp] Workspace-validation test parity across all 5 MCP servers
- summary: Review pass: all 3 ACs met; workspace-validation parity is verified.
- ref: `memory-api/.ticket/tickets/459789f8-12b7-4013-be11-521d5ca23e49/ticket.toml`

<!-- ticket-index:entry id=d5722e8e-4932-4ccc-9cee-480ada710202 slug=done/memory-api digest=eeb582438cdc -->
#### [d5722e8e] [spec][P0.5] memory-api — EntityStore convenience facade
- priority: `high`
- summary: Add an `EntityStore` struct to `memory-api` that composes `RedbIndexStore`, `EntityFs`, and `TantivySearchIndex` into a single convenient type. This gives downstream crates (spec-api, ticket-api) a u...
- ref: `memory-api/.ticket/tickets/d5722e8e-4932-4ccc-9cee-480ada710202/ticket.toml`

<!-- ticket-index:entry id=e0b3e9a8-bd43-472a-8222-f8c5e3321dbd slug=done/memory-api digest=1097c0fa8f3c -->
#### [e0b3e9a8] [spec][P0] Extract memory-api crate — generic entity storage, index, search, schema engine
- priority: `critical`
- summary: Extract ~75% of ticket-api into a generic `memory-api` crate that provides filesystem-backed entity storage with schema validation, indexing, search, and graph edges. Both ticket-api and the new spec...
- ref: `memory-api/.ticket/tickets/e0b3e9a8-bd43-472a-8222-f8c5e3321dbd/ticket.toml`

<!-- ticket-index:entry id=d187d817-d3f5-49ca-8925-8d06b5824912 slug=done/memory-api digest=e2cb6c151293 -->
#### [d187d817] [ticket-cli][spec-cli][rule-cli][audit-cli] Add TOON input and output support
- priority: `medium`
- summary: Implemented TOON machine-readable output across the memory-api CLI suite and extended spec-cli structured field decoding to accept TOON next to JSON.
- ref: `.ticket/tickets/d187d817-d3f5-49ca-8925-8d06b5824912/ticket.toml`

<!-- ticket-index:entry id=3041d7e3-2b34-4597-b354-e0aa6ffb0459 slug=done/memory-api digest=7f5b920602d7 -->
#### [3041d7e3] [transports] Correlate CLI/MCP/HTTP spans with log sessions and journals
- priority: `medium`
- summary: Starting transport correlation implementation: inspect ticket-http, ticket-mcp, and ticket-cli lifecycle entrypoints and propagate stable request/tool correlation plus journal ids through transport s...
- ref: `.ticket/tickets/3041d7e3-2b34-4597-b354-e0aa6ffb0459/ticket.toml`


### Component: memory-kernel

<!-- ticket-index:entry id=e12d8343-24f2-4b5d-8023-5a071238904a slug=done/memory-kernel digest=7d7f7ef72fde -->
#### [e12d8343] [workflow-tools][foundations] Extract shared storage kernel into memory-kernel repository
- priority: `high`
- summary: Phase A extraction: create standalone `memory-kernel` from `memory-api/crates/memory-api`, freeing the legacy package name and supplying the neutral shared substrate for workflow domains.
- ref: `.ticket/tickets/e12d8343-24f2-4b5d-8023-5a071238904a/ticket.toml`


### Component: memory-matrix

<!-- ticket-index:entry id=51e2210c-829b-4f7f-865e-99d120d8fd7d slug=done/memory-matrix digest=bc684f50f868 -->
#### [51e2210c] [memory-matrix] Add missing-store explicit-init policy coverage
- priority: `high`
- summary: The matrix needs explicit missing-store coverage to prove strict read/search/scan paths do not recreate hidden store roots, with explicit create/init as separate positive controls.
- ref: `memory-api/.ticket/tickets/51e2210c-829b-4f7f-865e-99d120d8fd7d/ticket.toml`

<!-- ticket-index:entry id=cc78d33d-1744-4945-bb77-f0fd1142568e slug=done/memory-matrix digest=5ed61e34b7e6 -->
#### [cc78d33d] [memory-matrix] Subprocess failure bundle capture for transport cells
- priority: `high`
- summary: Make matrix harness failures for subprocess-driven transport cells actionable on first failure, without ad hoc stderr plumbing.
- ref: `memory-api/.ticket/tickets/cc78d33d-1744-4945-bb77-f0fd1142568e/ticket.toml`


### Component: model-prices

<!-- ticket-index:entry id=b0d6bb1c-1a74-478a-aac1-1943b5454e96 slug=done/model-prices digest=803931a1e213 -->
#### [b0d6bb1c] Extend sync_model_prices.py with GitHub Copilot pricing source
- priority: `medium`
- summary: `tools/model-prices/sync_model_prices.py` sources prices only from pydantic/genai-prices `prices/data_slim.json`, which is a vendor catalogue. Models offered by the Copilot `runSubagent` surface can ...
- ref: `.ticket/tickets/b0d6bb1c-1a74-478a-aac1-1943b5454e96/ticket.toml`


### Component: observability

<!-- ticket-index:entry id=ff6637f5-01f6-46c3-b727-e1a19ee0f202 slug=done/observability digest=c776b0f646c2 -->
#### [ff6637f5] [benchmarks] Capture profiling timings through logs and journals
- priority: `medium`
- summary: Define how benchmarks and profiling runs use tracing logs and journal metadata without mixing timing data into deterministic replay state.
- ref: `.ticket/tickets/ff6637f5-01f6-46c3-b727-e1a19ee0f202/ticket.toml`

<!-- ticket-index:entry id=84673399-75e6-4f36-8a17-4c666001e530 slug=done/observability digest=7e5377bc8ee3 -->
#### [84673399] [observability] Resolve logging, journaling, and replay architecture boundaries
- priority: `high`
- summary: Resolve the core architecture decisions for unified logging, operation journaling, and replayable visualization before implementation proceeds.
- ref: `.ticket/tickets/84673399-75e6-4f36-8a17-4c666001e530/ticket.toml`

<!-- ticket-index:entry id=529844ac-f7e5-4265-b087-5bd2b597155f slug=done/observability digest=2ce0cf71c7ae -->
#### [529844ac] [observability][contract] Define cross-store correlation-id contract
- priority: `high`
- summary: Define and publish the cross-store correlation-id contract for observability artifacts so logs, journals, replay events, and transport spans can be joined deterministically.
- ref: `.ticket/tickets/529844ac-f7e5-4265-b087-5bd2b597155f/ticket.toml`

<!-- ticket-index:entry id=8b1eab26-389b-4125-86ec-886c9d48702b slug=done/observability digest=fa3fe591a766 -->
#### [8b1eab26] [observability][contract] Define deterministic replay vs profiling evidence boundary
- priority: `high`
- summary: Define and publish the deterministic replay versus profiling evidence boundary so replay/rollback state remains deterministic while performance diagnostics remain queryable and linked.
- ref: `.ticket/tickets/8b1eab26-389b-4125-86ec-886c9d48702b/ticket.toml`

<!-- ticket-index:entry id=1c56033e-5c30-46bd-a0bd-2209b8841876 slug=done/observability digest=fb6cd5b1990b -->
#### [1c56033e] [observability][contract] Publish canonical profiling/tracing phase taxonomy
- priority: `high`
- summary: Publish a canonical profiling/tracing phase taxonomy so phase-level timings and completion events are queryable and comparable across observability surfaces.
- ref: `.ticket/tickets/1c56033e-5c30-46bd-a0bd-2209b8841876/ticket.toml`

<!-- ticket-index:entry id=72b3545c-ceb9-4cb2-a8d4-c146fc9b460a slug=done/observability digest=3b338fd5292b -->
#### [72b3545c] [observability][governance] Define profiling metadata retention and redaction policy
- priority: `medium`
- summary: Define and publish the governance policy for profiling metadata retention, rotation/sampling, and redaction across observability artifacts.
- ref: `.ticket/tickets/72b3545c-ceb9-4cb2-a8d4-c146fc9b460a/ticket.toml`

<!-- ticket-index:entry id=de8719bf-a58a-41d1-891e-2b87894e6c02 slug=done/observability digest=0009fbf8aea5 -->
#### [de8719bf] [profiling][benchmarks] Emit standardized run metadata and percentile summaries
- priority: `high`
- ref: `.ticket/tickets/de8719bf-a58a-41d1-891e-2b87894e6c02/ticket.toml`

<!-- ticket-index:entry id=1b34dbe7-7055-45ae-8c3c-068adef1ca84 slug=done/observability digest=b79a84303f52 -->
#### [1b34dbe7] [profiling][ticket-api] Decompose integration and workflow sub-phase timings
- priority: `high`
- summary: Decompose ticket-api integration and workflow recompute profiling into explicit sub-phase timings aligned with observability phase taxonomy.
- ref: `.ticket/tickets/1b34dbe7-7055-45ae-8c3c-068adef1ca84/ticket.toml`

<!-- ticket-index:entry id=87ff70d7-36a8-453d-9ce2-3fec830b163f slug=done/observability digest=ac47f60a4814 -->
#### [87ff70d7] [profiling][validation] Validate profile evidence linkage across ticket/spec/journal
- priority: `medium`
- summary: Validate that profiling evidence produced in the ff6637f5 track is linkable across ticket, spec, and journal artifacts while keeping replay payload deterministic.
- ref: `.ticket/tickets/87ff70d7-36a8-453d-9ce2-3fec830b163f/ticket.toml`


### Component: onboarding

<!-- ticket-index:entry id=8f94b367-da11-4c59-9dbd-783700f36056 slug=done/onboarding digest=a84c84b649be -->
#### [8f94b367] [onboarding] Fresh-clone bootstrap must work in one documented step without errors
- priority: `high`
- summary: A first-time user must be able to go from a fresh clone to a fully usable repository in a minimal, documented sequence of steps, with **zero error messages** along the way.
- ref: `.ticket/tickets/8f94b367-da11-4c59-9dbd-783700f36056/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=762d9ac9-e0e0-4f02-b60f-21c79e3c26f6 slug=done/repo-guidance digest=fe3bfd7a0020 -->
#### [762d9ac9] Enforce ticket-spec-validation-doc-review workflow across generated guidance
- priority: `high`
- summary: The repository guidance does not consistently enforce one workflow for normal engineering work.
- ref: `.ticket/tickets/762d9ac9-e0e0-4f02-b60f-21c79e3c26f6/ticket.toml`

<!-- ticket-index:entry id=14ff41fa-818e-4a4e-8747-f79a33d174c2 slug=done/repo-guidance digest=86a52e7f0957 -->
#### [14ff41fa] [agents][rule] Add token-optimized agentic engineering skill target
- priority: `medium`
- summary: Implemented generated rule target for `.agents/skills/token-optimized-agentic-engineering.SKILL.md`, created canonical `.skill` rule entry, translated source guidance to English, and verified target ...
- ref: `.ticket/tickets/14ff41fa-818e-4a4e-8747-f79a33d174c2/ticket.toml`

<!-- ticket-index:entry id=088c8c40-7615-486c-88bb-1534902377d1 slug=done/repo-guidance digest=a185bc2bd00c -->
#### [088c8c40] [memory-api] Adopt shared README schema and parent-linked tool READMEs
- priority: `high`
- summary: `memory-api` is already generated, but its README targets still encode structure locally and its first-level tool READMEs do not provide the parent-link blocks required for a navigable repo tree.
- ref: `.ticket/tickets/088c8c40-7615-486c-88bb-1534902377d1/ticket.toml`

<!-- ticket-index:entry id=9c6fd645-3c50-47f2-b9bd-6de323de0ecc slug=done/repo-guidance digest=81765d97dc64 -->
#### [9c6fd645] [readmes][rule-api] Add shared README schema and validation primitives
- priority: `high`
- summary: The current `rule-targets` model supports imports and explicit node lists, but it does not provide a reusable README schema that multiple workspaces can inherit. That forces each repo to hand-author ...
- ref: `.ticket/tickets/9c6fd645-3c50-47f2-b9bd-6de323de0ecc/ticket.toml`

<!-- ticket-index:entry id=6d4c4777-3dd9-4aa3-9c7b-1780cf1175ee slug=done/repo-guidance digest=296fff664ea5 -->
#### [6d4c4777] [repo-guidance] Generate ticket-system and ticket prompts from canonical rule targets
- priority: `high`
- summary: The repository workflow guidance spec says the guidance under `.agents/` and `.github/` is regenerated from canonical rule content, but three key files are still effectively being treated as directly...
- ref: `.ticket/tickets/6d4c4777-3dd9-4aa3-9c7b-1780cf1175ee/ticket.toml`

<!-- ticket-index:entry id=5d3cd5da-99e5-4320-979c-595fedf24a88 slug=done/repo-guidance digest=36b8bbb58a22 -->
#### [5d3cd5da] [repo-guidance] Link ticket references to ticket.toml in generated guidance
- priority: `medium`
- summary: Update canonical ticket-reference guidance so chat/spec outputs keep the authoritative ticket folder path from ticket-api output for traceability, but append the concrete ticket file path when render...
- ref: `.ticket/tickets/5d3cd5da-99e5-4320-979c-595fedf24a88/ticket.toml`

<!-- ticket-index:entry id=321f6a3a-8bfb-4a8e-95bc-64ff845812ed slug=done/repo-guidance digest=c2d9e6e135ac -->
#### [321f6a3a] [repo-guidance] Model cost-awareness and tiered model-routing guidance
- priority: `high`
- summary: Add durable agent-policy guidance that encourages model cost awareness and delegation of cheap, routine work to smaller/cheaper models via subagents — especially inside sessions driven by large, expe...
- ref: `.ticket/tickets/321f6a3a-8bfb-4a8e-95bc-64ff845812ed/ticket.toml`

<!-- ticket-index:entry id=eaa42703-11f8-42dc-8c18-aec48101ed5e slug=done/repo-guidance digest=eaad48f31f40 -->
#### [eaa42703] [repo-guidance][rule-api] Generate workflow prompt and agent files from canonical rules
- priority: `high`
- summary: Generate the requested workflow prompt and agent files from canonical rule-api entries, wire them into root rule-target configs, generate the outputs, and keep the implementation traceability explici...
- ref: `.ticket/tickets/eaa42703-11f8-42dc-8c18-aec48101ed5e/ticket.toml`

<!-- ticket-index:entry id=e4f6e712-b3b6-493a-9ca2-d5f0d91f61b9 slug=done/repo-guidance digest=6aaaf322df66 -->
#### [e4f6e712] [repo-guidance][rule-api] Import child rule-target configs and generate nested workspace agent files
- priority: `high`
- summary: Root guidance generation still duplicates child workspace target definitions, and several nested-workspace guidance files under `.github/agents/` are still hand-written instead of being rendered from...
- ref: `.ticket/tickets/e4f6e712-b3b6-493a-9ca2-d5f0d91f61b9/ticket.toml`

<!-- ticket-index:entry id=45379405-d7c3-41bf-bd6d-059354c4291b slug=done/repo-guidance digest=63628ae96493 -->
#### [45379405] [repo-guidance][rule-api] Split rule-target configs into thematic folders across nested workspaces
- priority: `high`
- summary: Split rule-target configs into thematic files under rule-targets/ directories across the root, memory-viewers, memory-api, and viewer-api workspaces, and extend rule-api imports so parent workspaces ...
- ref: `.ticket/tickets/45379405-d7c3-41bf-bd6d-059354c4291b/ticket.toml`

<!-- ticket-index:entry id=2750018f-ed82-4a3a-9347-1fc47e9658c8 slug=done/repo-guidance digest=9e4a070e0bda -->
#### [2750018f] [rule-api] Implement shared README schema inheritance and validation
- priority: `high`
- summary: Once the failing tests exist, `rule-api` still needs real schema support and validation behavior. The current model in `targets.rs` only supports explicit node lists, which leaves the README rollout ...
- ref: `.ticket/tickets/2750018f-ed82-4a3a-9347-1fc47e9658c8/ticket.toml`

<!-- ticket-index:entry id=ba37c1c6-a853-4596-bf91-ab0b02f493ef slug=done/repo-guidance digest=5bb9eb916dde -->
#### [ba37c1c6] [rule-api][tests] Add failing README schema fixtures and coverage
- priority: `high`
- summary: The README schema work needs a stable failing test surface before any parser or renderer changes land. Without that, schema inheritance and required-block behavior will be guessed rather than proven.
- ref: `.ticket/tickets/ba37c1c6-a853-4596-bf91-ab0b02f493ef/ticket.toml`

<!-- ticket-index:entry id=d7d582c2-5734-4818-acf1-382f67bfdb89 slug=done/repo-guidance digest=5a6f74148335 -->
#### [d7d582c2] [viewer-api] Adopt shared README schema and parent-linked child READMEs
- priority: `high`
- summary: `viewer-api` is already generated, but its README targets still use a bespoke structure and its first-level generated child READMEs do not currently provide the repo-internal parent-link chain requir...
- ref: `.ticket/tickets/d7d582c2-5734-4818-acf1-382f67bfdb89/ticket.toml`

<!-- ticket-index:entry id=d6f5f59e-3955-443f-9381-afc486d0b8ad slug=done/repo-guidance digest=7c7aeb1bd757 -->
#### [d6f5f59e] [workflow] Generate guidance surfaces and seed session-api planning
- priority: `high`
- summary: Create generated workflow prompts and agents from canonical rule entries, then plan and scaffold a first session-api slice for saving Copilot chat sessions into the memory-api store.
- ref: `.ticket/tickets/d6f5f59e-3955-443f-9381-afc486d0b8ad/ticket.toml`

<!-- ticket-index:entry id=326bfe38-6f5e-4000-9ffc-e5be0839194f slug=done/repo-guidance digest=1bfd406cfa4c -->
#### [326bfe38] [workflow][session-worktree] Add worktree-first session guidance and hooks
- priority: `high`
- summary: Adopt the new worktree-backed session path in repository guidance and hooks after the planning contract and `session-api` assignment surfaces are in place.
- ref: `.ticket/tickets/326bfe38-6f5e-4000-9ffc-e5be0839194f/ticket.toml`


### Component: repo-workflow-tooling

<!-- ticket-index:entry id=74b32430-cd23-43ad-94dd-086ff752e2b4 slug=done/repo-workflow-tooling digest=515e7805478c -->
#### [74b32430] Prototype cross-store workflow traceability capture
- priority: `high`
- summary: The repository workflow expects first-class traceability across tickets, specs, docs, validation records, and logs, but the corrected architecture is store-owned metadata rather than wrapper-owned li...
- ref: `.ticket/tickets/74b32430-cd23-43ad-94dd-086ff752e2b4/ticket.toml`

<!-- ticket-index:entry id=042efd55-80a7-4a79-a821-75972f8886e3 slug=done/repo-workflow-tooling digest=0e2fdc612c1f -->
#### [042efd55] Prototype documentation validation capture for doc-api redesign
- priority: `medium`
- summary: The repository workflow requires documentation validation for authored docs and generated guidance surfaces, but the corrected architecture is native workflow metadata owned by `doc-api` and surfaced...
- ref: `.ticket/tickets/042efd55-80a7-4a79-a821-75972f8886e3/ticket.toml`

<!-- ticket-index:entry id=02bf9cf0-7e14-46f8-b80a-9e66b38878f9 slug=done/repo-workflow-tooling digest=ed28c7f70a0f -->
#### [02bf9cf0] Prototype validation capture for workflow metadata redesign
- priority: `high`
- summary: The repository workflow requires reusable validation capture, but the corrected architecture is embedded workflow metadata in the ticket/spec/doc layers plus future first-class test/log stores.
- ref: `.ticket/tickets/02bf9cf0-7e14-46f8-b80a-9e66b38878f9/ticket.toml`

<!-- ticket-index:entry id=1031c748-ba1e-43c0-ab4a-8ad2a0e9e97b slug=done/repo-workflow-tooling digest=9a25a72f1e57 -->
#### [1031c748] Remove workflow-cli prototype from live repo surfaces
- priority: `high`
- summary: Remove the remaining live `workflow-cli` prototype surface from the repository.
- ref: `.ticket/tickets/1031c748-ba1e-43c0-ab4a-8ad2a0e9e97b/ticket.toml`

<!-- ticket-index:entry id=06778dd8-a894-4759-b8fc-f00f6dd21fa5 slug=done/repo-workflow-tooling digest=6f6be0883ccc -->
#### [06778dd8] Rewrite doc validation spec around doc-api and doc-cli
- priority: `high`
- summary: Rewrite the documentation validation spec so it targets `doc-api` and a future `doc-cli` instead of a separate wrapper documentation command path.
- ref: `.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5/ticket.toml`

<!-- ticket-index:entry id=0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274 slug=done/repo-workflow-tooling digest=16c71ef59658 -->
#### [0fb5a2e5] Rewrite workflow traceability spec around first-class metadata links
- priority: `high`
- summary: Rewrite the workflow traceability spec so cross-store links are modeled as first-class metadata across the memory stores instead of wrapper-owned path artifacts.
- ref: `.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274/ticket.toml`

<!-- ticket-index:entry id=75e9fef3-b624-4e12-9709-5d800222908c slug=done/repo-workflow-tooling digest=a75a5f7a3657 -->
#### [75e9fef3] Rewrite workflow validation spec around embedded memory-api behavior
- priority: `high`
- summary: Rewrite the current workflow validation spec so it describes embedded memory-system behavior instead of a separate wrapper validation path.
- ref: `.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c/ticket.toml`


### Component: rule-api

<!-- ticket-index:entry id=af7ee01c-d649-4ed2-898c-d4f2e148f00f slug=done/rule-api digest=7880ff66846d -->
#### [af7ee01c] Add explain and preview tooling for rule target composition
- priority: `high`
- summary: The current generation flow does not explain why a file contains the entries it does. That makes target construction difficult to review, debug, and evolve.
- ref: `memory-api/.ticket/tickets/af7ee01c-d649-4ed2-898c-d4f2e148f00f/ticket.toml`

<!-- ticket-index:entry id=48b1cefb-dcc5-4cd4-ac41-568e57c97aca slug=done/rule-api digest=1a7dbc481de2 -->
#### [48b1cefb] Add rule-api tools to edit and generate agent markdown
- priority: `high`
- summary: A canonical `rule-api` store is not enough by itself. The team needs tools to import duplicated markdown, edit canonical rule entries, and generate repo-local files so manual file editing is no longe...
- ref: `memory-api/.ticket/tickets/48b1cefb-dcc5-4cd4-ac41-568e57c97aca/ticket.toml`

<!-- ticket-index:entry id=88800b2e-74f5-4d65-958a-1423d18072e3 slug=done/rule-api digest=554e6b5e9ce9 -->
#### [88800b2e] Attach ratings and feedback to rule entries
- priority: `high`
- summary: The current markdown files do not provide a structured, uniform way for agents to record whether a rule entry was helpful, outdated, conflicting, or in need of revision.
- ref: `memory-api/.ticket/tickets/88800b2e-74f5-4d65-958a-1423d18072e3/ticket.toml`

<!-- ticket-index:entry id=dee7de7a-4af0-468e-b779-309192e2e4db slug=done/rule-api digest=b1f2e68b75fa -->
#### [dee7de7a] Create rule-api storage model and stable rule IDs
- priority: `high`
- summary: We need a concrete `rule-api` domain on top of `memory-api` storage primitives so canonical rule entries can be stored, indexed, searched, versioned, rendered into markdown, and annotated with rating...
- ref: `memory-api/.ticket/tickets/dee7de7a-4af0-468e-b779-309192e2e4db/ticket.toml`

<!-- ticket-index:entry id=18eb59ee-05f6-4a03-b522-438b67556141 slug=done/rule-api digest=3bdc4c98c24f -->
#### [18eb59ee] Design hierarchical rule target schema
- priority: `high`
- summary: `RenderTarget` currently acts as a flat filter plus output path. That is not expressive enough to describe a document outline, per-section composition, or explicit ordering within a file.
- ref: `memory-api/.ticket/tickets/18eb59ee-05f6-4a03-b522-438b67556141/ticket.toml`

<!-- ticket-index:entry id=84ee1a9b-e0e8-4990-a9c7-af0e7b336d0e slug=done/rule-api digest=874ac3c90d80 -->
#### [84ee1a9b] Implement deterministic hierarchical rule target evaluation
- priority: `high`
- summary: Even with a better schema, the current generation path still assumes one file-wide filter and one flat ordered list of rule entries. The evaluator needs to understand a hierarchical target tree and r...
- ref: `memory-api/.ticket/tickets/84ee1a9b-e0e8-4990-a9c7-af0e7b336d0e/ticket.toml`

<!-- ticket-index:entry id=050c5441-1d3a-46bc-9748-cfb7030a93bd slug=done/rule-api digest=ac4e9e4c582b -->
#### [050c5441] Implement nested rule workspaces across submodule repositories
- priority: `high`
- summary: `rule-api` currently operates on one workspace root and one target config per invocation. That is enough for the top-level `context-engine` workflow, but it does not let `memory-viewers/`, `memory-ap...
- ref: `memory-api/.ticket/tickets/050c5441-1d3a-46bc-9748-cfb7030a93bd/ticket.toml`

<!-- ticket-index:entry id=c809ae33-a4fa-4e5f-b920-5d269466a11c slug=done/rule-api digest=b11813db1e28 -->
#### [c809ae33] Improve rule target construction for hierarchical document outlines
- priority: `high`
- summary: The current `rule-targets.toml` model builds each output file from one flat filter. That keeps the implementation small, but it makes file composition hard to reason about, encourages repeating rule ...
- ref: `memory-api/.ticket/tickets/c809ae33-a4fa-4e5f-b920-5d269466a11c/ticket.toml`

<!-- ticket-index:entry id=454405a2-a37e-4be6-b7a7-b96008afa974 slug=done/rule-api digest=1905c165c162 -->
#### [454405a2] Migrate duplicated agent docs into generated rule-api outputs
- priority: `high`
- summary: The existing `AGENTS.md` and `.github` markdown files are duplicated across context-engine, memory-viewers, memory-api, and viewer-api. Shared text is currently owned by copy-paste instead of by a ca...
- ref: `memory-api/.ticket/tickets/454405a2-a37e-4be6-b7a7-b96008afa974/ticket.toml`

<!-- ticket-index:entry id=f76169f7-239d-4993-a0a2-0709414acb7f slug=done/rule-api digest=707a35eeda73 -->
#### [f76169f7] Preserve existing line endings in generated outputs
- priority: `medium`
- summary: Implemented shared generated-output newline preparation in rule-api so rewrites adapt to the existing file's newline sequence while new files stay canonical LF. Wired the behavior into rule-cli and r...
- ref: `memory-api/.ticket/tickets/f76169f7-239d-4993-a0a2-0709414acb7f/ticket.toml`

<!-- ticket-index:entry id=9336a096-4399-467e-a7d8-fac30080d71f slug=done/rule-api digest=9e2810955fce -->
#### [9336a096] [memory-index] Rule store catalog generator
- priority: `high`
- summary: Build a generator that reads the rule store (rule-api) and emits a grouped catalog at `.rule/README.md` with its TOON sidecar at `.rule/index.toon`. The purpose is to give agents a compact, browsable...
- ref: `.ticket/tickets/9336a096-4399-467e-a7d8-fac30080d71f/ticket.toml`

<!-- ticket-index:entry id=e057932b-aaa8-43f5-be33-91dbf7399057 slug=done/rule-api digest=d70d4d805805 -->
#### [e057932b] [rule-api] Backfill existing rule workspaces to body.md storage
- priority: `high`
- summary: Once `rule-api` can understand the new storage contract, the repository still has 543 existing rule folders spread across four workspaces that need to be brought into the new layout. Leaving them mix...
- ref: `memory-api/.ticket/tickets/e057932b-aaa8-43f5-be33-91dbf7399057/ticket.toml`

<!-- ticket-index:entry id=d8581db8-ab3b-4445-8f1b-1b5dbf801b5e slug=done/rule-api digest=c0735987b727 -->
#### [d8581db8] [rule-api] Define body.md rule storage contract and migration plan
- priority: `high`
- summary: `rule-api` currently stores canonical rule body text in two places:
- ref: `memory-api/.ticket/tickets/d8581db8-ab3b-4445-8f1b-1b5dbf801b5e/ticket.toml`

<!-- ticket-index:entry id=e395bad6-c70c-4957-80da-412491304c84 slug=done/rule-api digest=824cbf424852 -->
#### [e395bad6] [rule-api] Implement canonical body.md storage with legacy rule compatibility
- priority: `high`
- summary: Even after the desired `body.md` contract is clear, `rule-api` still depends on shared storage helpers that hardcode `description.md`, and the rule schema still requires a manifest-level `body` field...
- ref: `memory-api/.ticket/tickets/e395bad6-c70c-4957-80da-412491304c84/ticket.toml`


### Component: session

<!-- ticket-index:entry id=72314c5e-6cc6-40f9-af58-96b66004d4d7 slug=done/session digest=ae9e858f1e69 -->
#### [72314c5e] Add a rename subcommand for topic-slug worktrees
- priority: `medium`
- summary: Provide a safe, agent-facing `rename <old-name> <new-name>` command for topic-slug worktree renames.
- ref: `.ticket/tickets/72314c5e-6cc6-40f9-af58-96b66004d4d7/ticket.toml`

<!-- ticket-index:entry id=a1b911ab-9394-4ba8-9134-1b2687e96ccd slug=done/session digest=edc4d686e510 -->
#### [a1b911ab] Discover a session's worktree from session_id, and recycle worktrees on session completion
- priority: `high`
- summary: Let an MCP server resolve `session_id` to the worktree that session should write
- ref: `.ticket/tickets/a1b911ab-9394-4ba8-9134-1b2687e96ccd/ticket.toml`

<!-- ticket-index:entry id=723c2bea-3b5d-4cf2-a519-958bcab036a3 slug=done/session digest=ec3b493c6cba -->
#### [723c2bea] Fix shared-config core.worktree hijack in submodule worktrees
- priority: `high`
- summary: Prevent worktree submodule population from writing `core.worktree` into shared submodule configuration, where the most recently populated worktree can hijack every checkout.
- ref: `.ticket/tickets/723c2bea-3b5d-4cf2-a519-958bcab036a3/ticket.toml`

<!-- ticket-index:entry id=5e6cf4f8-120c-4674-95de-d7b79c99f5b3 slug=done/session digest=029017b9541e -->
#### [5e6cf4f8] Rewrite worktree.sh as a Rust binary and add worktree lifecycle recycling
- priority: `high`
- summary: Replace `tools/worktree/worktree.sh` with a Rust binary that drives git through
- ref: `.ticket/tickets/5e6cf4f8-120c-4674-95de-d7b79c99f5b3/ticket.toml`

<!-- ticket-index:entry id=0afe45b5-9ec8-4f4a-af74-f46f06cc7516 slug=done/session digest=5d7509372284 -->
#### [0afe45b5] [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store
- priority: `high`
- summary: Make ticket-api and session-api store resolution deterministically anchor to the
- ref: `.ticket/tickets/0afe45b5-9ec8-4f4a-af74-f46f06cc7516/ticket.toml`

<!-- ticket-index:entry id=fd374421-f72f-4175-9daf-c47d387e7a01 slug=done/session digest=d27a319cb465 -->
#### [fd374421] compact-terminal-mcp hangs: spawned shell inherits the server's MCP stdin
- priority: `high`
- summary: `compact-terminal-mcp` hangs forever on every request, returning no output at all, including for a trivial `echo alive`. Reproduced repeatedly on 2026-08-07 after rebuilding the binary from source, s...
- ref: `.ticket/tickets/fd374421-f72f-4175-9daf-c47d387e7a01/ticket.toml`

<!-- ticket-index:entry id=23e67e65-c2d6-420a-8988-4c07a64e2235 slug=done/session digest=92ff463345ef -->
#### [23e67e65] worktree-ctl does not enforce bottom-up submodule integration or the gitlink containment invariant
- priority: `high`
- summary: The context-engine superproject has five submodules: `context-stack`, `memory-api`, `memory-kernel`, `memory-viewers`, and `viewer-api`. A session merged several `agent/*` branches into superproject ...
- ref: `.ticket/tickets/23e67e65-c2d6-420a-8988-4c07a64e2235/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=ab02e15a-df68-41bc-a2dc-638f1fd01694 slug=done/session-api digest=fbd977430ad7 -->
#### [ab02e15a] Add track fields to session schema (schema-only)
- summary: Add track_id, anchor_ticket_id, parent_session_id (all NULLABLE) to SessionRecord. Activate spawned_session_id. No migration work in this ticket.
- ref: `.ticket/tickets/ab02e15a-df68-41bc-a2dc-638f1fd01694/ticket.toml`

<!-- ticket-index:entry id=76c64b38-25e9-484c-818c-365f15114c89 slug=done/session-api digest=090d11fd32c5 -->
#### [76c64b38] Decouple Copilot session UUID from workspace runtime context identity
- priority: `high`
- summary: Root `session.exe init --workspace . --toon` returns the same value for both `session_id` and `workspace_session_id` — a stale slug (`epic-kickoff-8fdfe135`), not a fresh UUID. This is not a display ...
- ref: `memory-api/.ticket/tickets/76c64b38-25e9-484c-818c-365f15114c89/ticket.toml`

<!-- ticket-index:entry id=565ae4b1-dd93-4685-955d-58490a0dd3fb slug=done/session-api digest=4e07c493002f -->
#### [565ae4b1] Make copilot-capture-hook provisioning outcome observable
- priority: `medium`
- summary: `copilot-capture-hook` exits 0 and emits `{}` for successful provisioning, reuse, skip, and failure alike. Diagnostics go only to stderr, which VS Code discards, leaving silent success and silent fai...
- ref: `.ticket/tickets/565ae4b1-dd93-4685-955d-58490a0dd3fb/ticket.toml`

<!-- ticket-index:entry id=99e040a2-f7fb-40cf-8714-2bc487076d72 slug=done/session-api digest=0eb345155e74 -->
#### [99e040a2] Remove main-checkout session registry with dual-layout worktree discovery
- priority: `high`
- summary: Session-to-worktree assignment state is written to the main checkout at `.session/sessions/<uuid>/session.json`, although transcript capture already writes each session record inside the assigned wor...
- ref: `.ticket/tickets/99e040a2-f7fb-40cf-8714-2bc487076d72/ticket.toml`

<!-- ticket-index:entry id=e4f84414-ef2e-4012-9cfe-da08fe2c077c slug=done/session-api digest=8b7ad7ec5d58 -->
#### [e4f84414] Render workflow mermaid graph in handoff markdown
- priority: `medium`
- summary: Problem:
- ref: `memory-api/.ticket/tickets/e4f84414-ef2e-4012-9cfe-da08fe2c077c/ticket.toml`

<!-- ticket-index:entry id=d1b3a6c9-5f2e-4f6b-9b3c-8fa1e2d3c4b5 slug=done/session-api digest=448cd0304811 -->
#### [d1b3a6c9] Route workflow diagnostics upward and add structural workflow-graph validation
- priority: `high`
- summary: session-api: removed diag_* bubble-node emission from render_workflow_mermaid (store.rs), dropped its now-unused diagnostics param, updated both call sites (store.rs, store/config/workflow.rs). Added...
- ref: `memory-api/.ticket/tickets/d1b3a6c9-5f2e-4f6b-9b3c-8fa1e2d3c4b5/ticket.toml`

<!-- ticket-index:entry id=fd7737ec-814b-4ba1-a9da-33db36cdea94 slug=done/session-api digest=ed4ce2cf49e1 -->
#### [fd7737ec] Session track migration and reconciliation
- summary: FAIL-OPEN migration strategy: pre-existing sessions have track_id = null, NO backfill pass.
- ref: `.ticket/tickets/fd7737ec-814b-4ba1-a9da-33db36cdea94/ticket.toml`

<!-- ticket-index:entry id=c2d9b8f1-6a3b-4c5d-9e7f-1a2b3c4d5e6f slug=done/session-api digest=9feae7098631 -->
#### [c2d9b8f1] Support cross-workspace ticket/spec URN resolution in session workflow nodes
- priority: `high`
- summary: `DefaultTicketStateResolver` rejects any entity URN whose workspace slug differs from the session's own workspace slug. The rejection occurs in memory-api/crates/session-api/src/store.rs where code r...
- ref: `memory-api/.ticket/tickets/c2d9b8f1-6a3b-4c5d-9e7f-1a2b3c4d5e6f/ticket.toml`

<!-- ticket-index:entry id=b7c61f0e-ed42-4eef-8d3b-da934d7c0628 slug=done/session-api digest=650196aa04c9 -->
#### [b7c61f0e] [delegation-cost][session-api] Promote the sub-agent cost analyzer into session-api with real token attribution
- priority: `high`
- summary: Validated against committed tree (memory-api 98e1fa6). `cargo test -p session-api`: 195 passed (10 suites), 0 failed — matches implementer's report, still green.
- ref: `.ticket/tickets/b7c61f0e-ed42-4eef-8d3b-da934d7c0628/ticket.toml`

<!-- ticket-index:entry id=959c94bd-4a42-47d6-bee4-a12332a23b52 slug=done/session-api digest=4a388674d67b -->
#### [959c94bd] [session-api] Add hook ingestion and read/query support
- priority: `high`
- summary: Implement the next `session-api` batch in the nested `memory-api` workspace by making transcript persistence append only and adding the first read/query plus hook-facing capture APIs.
- ref: `memory-api/.ticket/tickets/959c94bd-4a42-47d6-bee4-a12332a23b52/ticket.toml`

<!-- ticket-index:entry id=f76b0fa9-d880-45da-b039-b483e904ee2f slug=done/session-api digest=d330ca154756 -->
#### [f76b0fa9] [session-api] Add session-cli and session-mcp for session subcommands
- priority: `high`
- summary: Expose the `session-api` capabilities (check-in, lookup, query, range peeking, and skeleton peeking) through dedicated CLI and MCP surfaces so that agents and users can interact with sessions cleanly.
- ref: `.ticket/tickets/f76b0fa9-d880-45da-b039-b483e904ee2f/ticket.toml`

<!-- ticket-index:entry id=908ed6a4-d4f2-4cd1-bce0-d2d804b19b62 slug=done/session-api digest=90dd5a796d97 -->
#### [908ed6a4] [session-api] Complete handoff round-trip assertions (risk_notes, predecessor_handoff) and verify 8c67b96a AC4/AC5
- summary: Review pass: all 4 ACs met; round-trip assertions are verified.
- ref: `memory-api/.ticket/tickets/908ed6a4-d4f2-4cd1-bce0-d2d804b19b62/ticket.toml`

<!-- ticket-index:entry id=70cd7056-c342-4433-ad60-5bc798f61aa6 slug=done/session-api digest=81d5d258de92 -->
#### [70cd7056] [session-api] Durable session workflow persistence and mutation
- priority: `high`
- summary: Implement the durable logical session workspace and mutable workflow graph defined by spec `memory-api/session-api/durable-session-workflow` (`c677182e-90da-4ac3-8b94-9e2e97c825cf`).
- ref: `memory-api/.ticket/tickets/70cd7056-c342-4433-ad60-5bc798f61aa6/ticket.toml`

<!-- ticket-index:entry id=cf4d1e1a-5315-4aa8-b836-5a90996e63c4 slug=done/session-api digest=f27894a9b59f -->
#### [cf4d1e1a] [session-api] Fix: Resolve session workspace relative to tool execution
- priority: `high`
- summary: Fix a bug where the `.memory-api` folder is created inside the nested `memory-viewers/memory-api` folder even when the ticket tool is run from the `context-engine` root. The session workspace should ...
- ref: `.ticket/tickets/cf4d1e1a-5315-4aa8-b836-5a90996e63c4/ticket.toml`

<!-- ticket-index:entry id=c8f79641-6f99-4401-9b08-ad960a8d785c slug=done/session-api digest=c4710403fa5a -->
#### [c8f79641] [session-api] Persist session captures to filesystem store
- priority: `high`
- summary: Implement the next `session-api` batch in the nested `memory-api` workspace by turning the current store plan into a real filesystem write path.
- ref: `memory-api/.ticket/tickets/c8f79641-6f99-4401-9b08-ad960a8d785c/ticket.toml`

<!-- ticket-index:entry id=11d3b412-7d70-4144-932d-589256af488a slug=done/session-api digest=ac0ce320515c -->
#### [11d3b412] [session-api] Record active model per transcript turn
- priority: `medium`
- summary: Session metadata already carries a single `model` field, but a session may route across multiple models (a large model delegating subtasks to cheaper ones). To make model routing observable, capture ...
- ref: `memory-api/.ticket/tickets/11d3b412-7d70-4144-932d-589256af488a/ticket.toml`

<!-- ticket-index:entry id=cc4b0289-b6fd-412f-a97a-497f05f572f4 slug=done/session-api digest=111f7c588392 -->
#### [cc4b0289] [session-api] Render session workflow as terminal and Mermaid graphs
- priority: `high`
- summary: Render the durable session workflow as a compact terminal dependency graph or deterministic Mermaid flowchart.
- ref: `memory-api/.ticket/tickets/cc4b0289-b6fd-412f-a97a-497f05f572f4/ticket.toml`

<!-- ticket-index:entry id=e663f9e9-ac52-4c0e-8e07-d17c8a15b48d slug=done/session-api digest=e47192ef4337 -->
#### [e663f9e9] [session-api] Wire VS Code Copilot stop-hook session capture
- priority: `high`
- summary: Implement the first external integration slice for session capture by wiring VS Code GitHub Copilot chat hooks to the existing `session-api` persistence path.
- ref: `.ticket/tickets/e663f9e9-ac52-4c0e-8e07-d17c8a15b48d/ticket.toml`

<!-- ticket-index:entry id=e4d4c667-6d51-41c2-bd73-098911def78e slug=done/session-api digest=cbaa2c361dd0 -->
#### [e4d4c667] [session-api] sessions_for_ticket aborts the whole scan on a malformed/corrupt session store entry
- summary: `sessions_for_ticket` fails hard (aborts the entire scan) when the live `.session` store contains a malformed entry. Two such entries currently exist in the real store:
- ref: `.ticket/tickets/e4d4c667-6d51-41c2-bd73-098911def78e/ticket.toml`

<!-- ticket-index:entry id=2b75bac2-ff14-43c3-8e87-1e801772f309 slug=done/session-api digest=8f372feee973 -->
#### [2b75bac2] [session-api] sessions_for_ticket is inert: capture ticket linkage at check-in and decide a structured-data backfill
- priority: `high`
- summary: `sessions_for_ticket` (added in ticket bba9b313-ff13-4fd1-91d4-6485a6c2f4de) is functionally inert against the real `.session` store:
- ref: `.ticket/tickets/2b75bac2-ff14-43c3-8e87-1e801772f309/ticket.toml`

<!-- ticket-index:entry id=d8cb1b87-48a2-4a99-b741-48cfaed44711 slug=done/session-api digest=c4bda312eef5 -->
#### [d8cb1b87] [session-api][audit] Remediate default file-length findings
- priority: `high`
- summary: The repository audit default is `max_file_lines = 400`. A prior tracker update incorrectly used a non-default per-run override as the acceptance criterion. That override has been removed from the liv...
- ref: `memory-api/.ticket/tickets/d8cb1b87-48a2-4a99-b741-48cfaed44711/ticket.toml`

<!-- ticket-index:entry id=25b5f3e7-cace-4822-a955-bc2e3202be77 slug=done/session-api digest=06049e01a109 -->
#### [25b5f3e7] [session-api][handoff] Make upward context and ticket narrative reproducible in handoff markdown
- summary: `render_handoff_record_markdown` currently renders `target_tickets` as bare backticked short IDs and cannot explain the higher-level goal or an upward program context. A prior handoff required a manu...
- ref: `.ticket/tickets/25b5f3e7-cace-4822-a955-bc2e3202be77/ticket.toml`

<!-- ticket-index:entry id=742dbc65-a100-4278-9274-7d99a3e2afc4 slug=done/session-api digest=9014cf20488e -->
#### [742dbc65] [session-api][handoff] Model and enforce upward context for implementation-ready handoffs
- summary: `SessionHandoffPackage` has no durable higher-level objective or structured upward-context data. `create_handoff_record` therefore cannot distinguish an implementation-ready handoff with missing prog...
- ref: `.ticket/tickets/742dbc65-a100-4278-9274-7d99a3e2afc4/ticket.toml`

<!-- ticket-index:entry id=0647a212-9d2e-4943-9627-f854ce3f14c4 slug=done/session-api digest=52adfa31f6e3 -->
#### [0647a212] [session-api][handoff] Persist handoff records and resume durable session workspaces
- priority: `high`
- summary: Persist structured session handoffs and resume the same durable workspace under a new capture run.
- ref: `memory-api/.ticket/tickets/0647a212-9d2e-4943-9627-f854ce3f14c4/ticket.toml`

<!-- ticket-index:entry id=ba8f5528-5af3-4de2-8904-442a4691854a slug=done/session-api digest=3da71d01b095 -->
#### [ba8f5528] [session-api][handoff] Render resolved ticket narrative and upward context in handoff markdown
- summary: `render_handoff_record_markdown` renders target tickets as bare short IDs and omits both a higher-level goal and upward program context. Manual edits to the generated exemplar are destroyed by the ne...
- ref: `.ticket/tickets/ba8f5528-5af3-4de2-8904-442a4691854a/ticket.toml`

<!-- ticket-index:entry id=41ed4585-5b1e-4681-96e8-4883ed140c18 slug=done/session-api digest=cf2df70cddb4 -->
#### [41ed4585] [session-api][handoff] Store handoffs as folders with handoff.json + rendered handoff.md
- priority: `high`
- summary: Store each handoff as a folder containing the canonical `handoff.json` plus a rendered `handoff.md`, so markdown handoffs sit next to the JSON for human review, feedback, and research loops (subticke...
- ref: `memory-api/.ticket/tickets/41ed4585-5b1e-4681-96e8-4883ed140c18/ticket.toml`

<!-- ticket-index:entry id=580019e8-b253-42ee-80fc-990f6d26baf6 slug=done/session-api digest=f7ab1a650d59 -->
#### [580019e8] [session-api][repo] Untrack 76 committed events.json files (~99MB) with git rm --cached, no history rewrite
- summary: `git ls-files | grep events.json` returns **76 tracked files totalling ~99MB** under `.session/sessions/**`. These were committed before the current `.gitignore` rule was added. New `events.json` fil...
- ref: `memory-api/.ticket/tickets/580019e8-b253-42ee-80fc-990f6d26baf6/ticket.toml`

<!-- ticket-index:entry id=980cf1fa-45e1-4bf3-96c0-305a5d6f709f slug=done/session-api digest=117162bd49b5 -->
#### [980cf1fa] [session-api][session-mcp] Validation node with null validation_spec_id permanently wedges session handoff/finish, with no repair path
- summary: Review pass: all 7 ACs met; validation node wedge repair is complete.
- ref: `memory-api/.ticket/tickets/980cf1fa-45e1-4bf3-96c0-305a5d6f709f/ticket.toml`

<!-- ticket-index:entry id=7a4f9c3d-bf5f-4849-93c7-b8c2706dac61 slug=done/session-api digest=439d23f52915 -->
#### [7a4f9c3d] [session-api][store] Flatten .session layout under sessions/<session_id>/ and relocate local pointers/locks
- priority: `high`
- summary: Relocate runtime context/handoffs/finish to be owned directly by `sessions/<session_id>/`, move local-only pointers/locks to `.session/local/`, and remove the `runtime/workspaces/` nesting (subticket...
- ref: `memory-api/.ticket/tickets/7a4f9c3d-bf5f-4849-93c7-b8c2706dac61/ticket.toml`

<!-- ticket-index:entry id=0a45bedb-6dfe-466e-893f-fddfd225f1f6 slug=done/session-api digest=cc726f2fc68e -->
#### [0a45bedb] [session-api][store] Flatten session store layout, unify identity, and git-track durable artifacts
- priority: `high`
- summary: Collapse the `.session/runtime/workspaces/<workspace_session_id>/` layer so a session **owns** its runtime context and handoffs directly, all sessions share the single `.session/` store, and durable ...
- ref: `memory-api/.ticket/tickets/0a45bedb-6dfe-466e-893f-fddfd225f1f6/ticket.toml`

<!-- ticket-index:entry id=4817a5cc-5e91-4280-b7ed-aed296a480b3 slug=done/session-api digest=46cf3090a40b -->
#### [4817a5cc] [session-api][store] Git-tracking policy: track durable session artifacts, ignore local pointers/locks
- priority: `high`
- summary: Make durable session artifacts persist in git for later feedback/research loops, while keeping machine-local pointers/locks ignored (subticket 4 of the flattening tracker). Depends on the flattened l...
- ref: `memory-api/.ticket/tickets/4817a5cc-5e91-4280-b7ed-aed296a480b3/ticket.toml`

<!-- ticket-index:entry id=7fabc77a-7704-448b-aef8-1f3e22dd18dd slug=done/session-api digest=fc275a74970b -->
#### [7fabc77a] [session-api][store] Remove legacy runtime-path fallback shims
- priority: `medium`
- summary: Remove the legacy `.session/runtime/` fallback shims now that the flattened `.session/sessions/<id>/` layout is canonical and the legacy tree has been deleted after byte-and-hash verification.
- ref: `memory-api/.ticket/tickets/7fabc77a-7704-448b-aef8-1f3e22dd18dd/ticket.toml`

<!-- ticket-index:entry id=fc86f42d-3fc0-4f07-911a-525098248dcf slug=done/session-api digest=d9a987b19ed2 -->
#### [fc86f42d] [session-api][store] Unify session identity: link runtime context to session_id and stamp transcripts
- priority: `high`
- summary: Establish the missing join between a runtime continuity thread and its captured transcripts so handoffs and context can be session-owned (subticket 1 of the flattening tracker). Foundational — no dir...
- ref: `memory-api/.ticket/tickets/fc86f42d-3fc0-4f07-911a-525098248dcf/ticket.toml`

<!-- ticket-index:entry id=e2189e9d-8ea7-4747-bda9-51e573ba51ca slug=done/session-api digest=40d7654b3f3c -->
#### [e2189e9d] [session-api][worktree] Implement session check-in and worktree assignment surfaces
- priority: `high`
- summary: Implement the first executable slice of the default worktree-backed session workflow after `68a49ca7` locks the contract.
- ref: `.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml`

<!-- ticket-index:entry id=67d7c279-6661-461b-8204-7a1bd7e028c5 slug=done/session-api digest=6cdd41994f55 -->
#### [67d7c279] [session-optimization] Deduplicate captured events (raw_event_json vs data_json, complete vs result)
- priority: `medium`
- summary: Stop storing each captured event's payload twice, which makes session review and replay disproportionately expensive.
- ref: `memory-api/.ticket/tickets/67d7c279-6661-461b-8204-7a1bd7e028c5/ticket.toml`

<!-- ticket-index:entry id=6737a239-60fa-44af-8bf3-a60f8eb1e8a8 slug=done/session-api digest=01288c618eaf -->
#### [6737a239] budget-offset grants
- summary: Implement durable, auditable budget-offset grants in session-api with create/list/revoke operations and grant_id-based resolution.
- ref: `.ticket/tickets/6737a239-60fa-44af-8bf3-a60f8eb1e8a8/ticket.toml`

<!-- ticket-index:entry id=b64cc71d-8594-4617-b3fb-3057fca0b56b slug=done/session-api digest=e3b838dc5397 -->
#### [b64cc71d] session-api tool_metrics core
- summary: Implement the core data model and computation primitives for empirical tool token metrics derived from session transcripts.
- ref: `.ticket/tickets/b64cc71d-8594-4617-b3fb-3057fca0b56b/ticket.toml`

<!-- ticket-index:entry id=c81f3938-0b4b-42a0-bbf1-888ddd9d2262 slug=done/session-api digest=677ef3885c68 -->
#### [c81f3938] upward escalation workflow
- summary: Implement durable escalation record and workflow for sub-agents to request capability/offset upward, with async resolution queue.
- ref: `.ticket/tickets/c81f3938-0b4b-42a0-bbf1-888ddd9d2262/ticket.toml`


### Component: session-cli,session-mcp

<!-- ticket-index:entry id=66c85d65-98bc-4432-b6f6-6c41664645f8 slug=done/session-cli,session-mcp digest=ad985221789b -->
#### [66c85d65] tool_metrics surfaces + rollup writer
- summary: Expose tool_metrics computation via CLI and MCP surfaces, and wire automatic rollup refresh on session persist.
- ref: `.ticket/tickets/66c85d65-98bc-4432-b6f6-6c41664645f8/ticket.toml`


### Component: session-workflow

<!-- ticket-index:entry id=47cc50db-8efa-4945-87fe-d30fe1f6bc61 slug=done/session-workflow digest=b3762c5e0742 -->
#### [47cc50db] Implement upstream tool-result guards and compact prompt views
- priority: `p1`
- summary: Build coded support that reduces what GitHub Copilot sends or reuses in model-facing context by guarding tool results upstream and emitting compact prompt-facing state views.
- ref: `.ticket/tickets/47cc50db-8efa-4945-87fe-d30fe1f6bc61/ticket.toml`

<!-- ticket-index:entry id=1c1ebfd1-4478-401f-a9ad-efcc2ff53b16 slug=done/session-workflow digest=d00b08f64312 -->
#### [1c1ebfd1] Reduce model-bound Copilot context in bootstrap and handoff flows
- priority: `p1`
- summary: Reduce low-value context that reaches GitHub Copilot model APIs by tightening upstream workflow guidance, tool-result handling, and bootstrap or handoff behavior.
- ref: `.ticket/tickets/1c1ebfd1-4478-401f-a9ad-efcc2ff53b16/ticket.toml`


### Component: spec

<!-- ticket-index:entry id=a9514081-35c2-4162-b62d-3baf4a14ec8b slug=done/spec digest=2eb0d2da5fb7 -->
#### [a9514081] [spec] Define explicit-init-only memory store contract
- priority: `high`
- summary: Owning spec updated: explicit-init-only contract now forbids implicit root creation and defines validation matrix.
- ref: `memory-api/.ticket/tickets/a9514081-35c2-4162-b62d-3baf4a14ec8b/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=09641443-a8f2-479d-85cb-ea44a963595b slug=done/spec-api digest=289b217e4396 -->
#### [09641443] Add spec-local target mapping for generated spec artifacts
- priority: `high`
- summary: `spec-api` has generated artifact write paths, but a spec folder has no explicit way to declare which `rule-api` target should produce `body.md` or any named section file. If that mapping is left imp...
- ref: `memory-api/.ticket/tickets/09641443-a8f2-479d-85cb-ea44a963595b/ticket.toml`

<!-- ticket-index:entry id=a5fe4c58-f59c-4d97-8ee6-3447724b5fac slug=done/spec-api digest=762c96605a74 -->
#### [a5fe4c58] Adopt rule targets for generated spec artifacts
- priority: `high`
- summary: The shared generated-markdown builder and `spec-api` generated body/section update paths now exist, but there is still no end-to-end workflow for a spec to declare that `body.md` or `sections/*.md` s...
- ref: `memory-api/.ticket/tickets/a5fe4c58-f59c-4d97-8ee6-3447724b5fac/ticket.toml`

<!-- ticket-index:entry id=f4b0be64-a2f5-4cb5-a476-b2b921d6ff02 slug=done/spec-api digest=d9292367583c -->
#### [f4b0be64] Generate spec documents from canonical snippets via shared builder
- priority: `high`
- summary: `rule-api` already knows how to collect ordered snippet content from a database-backed store, render generated markdown outputs, and rewrite files without losing the current newline convention. `spec...
- ref: `memory-api/.ticket/tickets/f4b0be64-a2f5-4cb5-a476-b2b921d6ff02/ticket.toml`

<!-- ticket-index:entry id=f147eb0e-c758-459b-a956-a1162c3e1af6 slug=done/spec-api digest=74d543b69ad0 -->
#### [f147eb0e] Migrate recurring spec principles to canonical rule entries via spec sync-generated
- priority: `high`
- summary: Cross-cutting design principles that recur across many specs (workspace identifiers, typed errors, JSON contracts, browser validation, ticket traceability link format, generated-file markers, `<x>-ap...
- ref: `.ticket/tickets/f147eb0e-c758-459b-a956-a1162c3e1af6/ticket.toml`

<!-- ticket-index:entry id=7f869c33-15ff-4959-8161-731844eef21b slug=done/spec-api digest=4090aa060acb -->
#### [7f869c33] Pilot migration for rule-target-backed spec artifacts
- priority: `high`
- summary: The proposed rule-target-backed spec workflow is still theoretical until at least one real spec stops duplicating canonical prose and proves the migration path end to end.
- ref: `memory-api/.ticket/tickets/7f869c33-15ff-4959-8161-731844eef21b/ticket.toml`

<!-- ticket-index:entry id=b9757ba7-3b2c-4f92-919d-f3c443ceb69c slug=done/spec-api digest=6a2497793fe2 -->
#### [b9757ba7] [memory-index] Spec store hierarchy generator
- priority: `high`
- summary: Build a generator that reads the spec store (spec-api) and emits a hierarchical markdown folder tree under `.spec/`, with `.spec/index.toon` as the machine-readable TOON sidecar. The purpose is to gi...
- ref: `.ticket/tickets/b9757ba7-3b2c-4f92-919d-f3c443ceb69c/ticket.toml`

<!-- ticket-index:entry id=87a35ccb-d91c-4ce8-93b3-e150bb5afe1d slug=done/spec-api digest=78ae0c9182cc -->
#### [87a35ccb] [rule-cli][rule-mcp] Route spec-doc targets through spec-owned generation
- priority: `high`
- ref: `.ticket/tickets/87a35ccb-d91c-4ce8-93b3-e150bb5afe1d/ticket.toml`

<!-- ticket-index:entry id=f986e666-d8db-4845-ba86-eb4bb89484ce slug=done/spec-api digest=b129fbbad16e -->
#### [f986e666] [spec-api] Reject empty and no-op spec body updates so a successful update guarantees a change
- priority: `high`
- summary: Closing evidence: Governing spec 351389c0-0873-4c3c-bc46-3551459ba1cd (spec-api/store, state=reviewed, links this ticket). Validation: `rtk cargo test -p spec-api` → 79 passed, 0 failed. Landed in me...
- ref: `.ticket/tickets/f986e666-d8db-4845-ba86-eb4bb89484ce/ticket.toml`

<!-- ticket-index:entry id=55a1b302-9b33-4389-8962-65362b9b3eb0 slug=done/spec-api digest=d83cfaed9104 -->
#### [55a1b302] [spec][P1] spec-api code references — symbol-level links to implementation files with line ranges
- priority: `high`
- summary: Implement the `CodeRef` system that links spec features to exact symbols in the implementation code with file paths and line ranges.
- ref: `memory-api/.ticket/tickets/55a1b302-9b33-4389-8962-65362b9b3eb0/ticket.toml`

<!-- ticket-index:entry id=dc0df24e-075c-4147-b96c-3b26b428b0a2 slug=done/spec-api digest=6504e5750750 -->
#### [dc0df24e] [spec][P1] spec-api crate — umbrella ticket (manifest, slugs, folders, schema, code refs, storage)
- priority: `high`
- summary: This is the parent ticket for the spec-api crate. It tracks the execution order and dependencies of all P1 sub-tickets.
- ref: `memory-api/.ticket/tickets/dc0df24e-075c-4147-b96c-3b26b428b0a2/ticket.toml`

<!-- ticket-index:entry id=4b6dc9d5-4932-4573-8635-d477804538ac slug=done/spec-api digest=e12e80cfb6ba -->
#### [4b6dc9d5] [spec][P1] spec-api schema — draft/reviewed/approved/implemented/verified state machine
- priority: `high`
- summary: Define the specification type schema with a full lifecycle from draft to verified implementation.
- ref: `memory-api/.ticket/tickets/4b6dc9d5-4932-4573-8635-d477804538ac/ticket.toml`

<!-- ticket-index:entry id=ab47648c-d1f8-4ad5-a652-08ef97f76ccd slug=done/spec-api digest=29b1f075db18 -->
#### [ab47648c] [spec][P1] spec-api storage — SpecStore on memory-api EntityStore with parent-child hierarchy
- priority: `high`
- summary: Build `SpecStore` on top of `memory_api::EntityStore` adding spec-specific features: parent-child hierarchy, slug uniqueness, multi-file folder support, and section management.
- ref: `memory-api/.ticket/tickets/ab47648c-d1f8-4ad5-a652-08ef97f76ccd/ticket.toml`

<!-- ticket-index:entry id=ad531f63-124b-4edd-b2b3-1f8a35173649 slug=done/spec-api digest=024bf3c81685 -->
#### [ad531f63] [spec][P1a] spec-api crate scaffold + SpecManifest model
- priority: `high`
- summary: Create the `crates/spec-api/` crate with its Cargo.toml and define the SpecManifest model using the same `extra: BTreeMap<String, Value>` pattern as EntityManifest/TicketManifest.
- ref: `memory-api/.ticket/tickets/ad531f63-124b-4edd-b2b3-1f8a35173649/ticket.toml`

<!-- ticket-index:entry id=90c88ead-86cc-4aba-ac36-85e7355bdcce slug=done/spec-api digest=8a2a1833fb3a -->
#### [90c88ead] [spec][P1b] spec-api slug system — validation, uniqueness, resolution
- priority: `high`
- summary: Implement the slug validation and resolution system for specs. Slugs are hierarchical, human-readable identifiers (e.g. `ticket-api/storage/store`) that provide a user-friendly alternative to UUIDs.
- ref: `memory-api/.ticket/tickets/90c88ead-86cc-4aba-ac36-85e7355bdcce/ticket.toml`

<!-- ticket-index:entry id=614f5f2a-3e86-412a-b4f0-d36f73935907 slug=done/spec-api digest=3a03cd178a0d -->
#### [614f5f2a] [spec][P1c] spec-api multi-file folder structure
- priority: `high`
- summary: Extend the EntityFs pattern to support the multi-file spec folder layout. Each spec lives in a `<scan_root>/<uuid>/` directory with a defined set of files.
- ref: `memory-api/.ticket/tickets/614f5f2a-3e86-412a-b4f0-d36f73935907/ticket.toml`

<!-- ticket-index:entry id=d12e6ca5-a83f-41c6-b612-219d4c2e82e3 slug=done/spec-api digest=8d465a55b02e -->
#### [d12e6ca5] [spec][P3] Spec creation — bootstrap specs for existing interfaces from code analysis
- priority: `high`
- summary: Build tooling to analyze existing Rust crate source code and generate initial spec files documenting the current implementation. This is for interfaces that are already implemented but not yet docume...
- ref: `memory-api/.ticket/tickets/d12e6ca5-a83f-41c6-b612-219d4c2e82e3/ticket.toml`

<!-- ticket-index:entry id=a0b59873-abe9-4e62-84a3-c233635b4cd6 slug=done/spec-api digest=e8a4d76979d6 -->
#### [a0b59873] spec + validation
- summary: Create or update spec and validation for the complete tool-metrics → graded-cost-gate → grant → escalation workflow.
- ref: `.ticket/tickets/a0b59873-abe9-4e62-84a3-c233635b4cd6/ticket.toml`


### Component: spec-cli

<!-- ticket-index:entry id=b2ef1de1-5801-47c6-97c6-e3c5cd8d7dae slug=done/spec-cli digest=2f861838ba7f -->
#### [b2ef1de1] Add spec sync-generated orchestration for rule-target-backed artifacts
- priority: `high`
- summary: Even with generated body/section APIs, there is no supported command that evaluates the declared rule targets for a spec and updates the spec store consistently. Running `rule-api` generation directl...
- ref: `memory-api/.ticket/tickets/b2ef1de1-5801-47c6-97c6-e3c5cd8d7dae/ticket.toml`

<!-- ticket-index:entry id=090b6db9-f88e-418b-888e-94641d347432 slug=done/spec-cli digest=3f3e04b66376 -->
#### [090b6db9] [spec][P2] spec-cli — CRUD, search, hierarchy, health commands
- priority: `high`
- summary: Create a `spec` CLI binary with CRUD, search, hierarchy navigation, and health check commands.
- ref: `memory-api/.ticket/tickets/090b6db9-f88e-418b-888e-94641d347432/ticket.toml`


### Component: spec-http

<!-- ticket-index:entry id=fc18c607-2147-4481-8a44-19bdc754f366 slug=done/spec-http digest=e507ecd4c511 -->
#### [fc18c607] [spec][P2] spec-http — HTTP endpoints for spec-api (alongside ticket-http)
- priority: `medium`
- summary: Add HTTP endpoints for spec-api, either as part of ticket-http or as a separate spec-http crate. Routes follow the same pattern as ticket-http.
- ref: `memory-api/.ticket/tickets/fc18c607-2147-4481-8a44-19bdc754f366/ticket.toml`


### Component: spec-mcp

<!-- ticket-index:entry id=20717710-a18d-4640-bea8-f7ee5b593327 slug=done/spec-mcp digest=a90f8c2dc913 -->
#### [20717710] [spec-api] code_ref kind deserializer rejects shorthand/legacy values (fn, block, field, enum_variant)
- summary: Repro
- ref: `.ticket/tickets/20717710-a18d-4640-bea8-f7ee5b593327/ticket.toml`

<!-- ticket-index:entry id=fbb5a87d-44b5-4a92-8c6c-79f8302dcba5 slug=done/spec-mcp digest=8fad78e963a0 -->
#### [fbb5a87d] [spec][P2] spec-mcp — MCP tool surface for spec-api
- priority: `high`
- summary: Create MCP tools for spec-api, following the same pattern as ticket-mcp. Tools for creating, reading, updating, searching, and generating skills from specs.
- ref: `memory-api/.ticket/tickets/fbb5a87d-44b5-4a92-8c6c-79f8302dcba5/ticket.toml`


### Component: spec-viewer

<!-- ticket-index:entry id=75c843e1-d380-40b1-8053-14e4658c42a3 slug=done/spec-viewer digest=5cbd7a49b058 -->
#### [75c843e1] [spec-viewer] Bug: sidebar overlay blocks spec list clicks
- priority: `high`
- summary: Opening the spec list sidebar in `spec-viewer` places an overlay above the list, which prevents users from clicking visible spec entries.
- ref: `memory-viewers/.ticket/tickets/75c843e1-d380-40b1-8053-14e4658c42a3/ticket.toml`

<!-- ticket-index:entry id=06399bb2-eb06-40a9-8de9-26f6d0854753 slug=done/spec-viewer digest=f4a6f8bb91ab -->
#### [06399bb2] [spec-viewer] GPU-accelerated spec browser — Dioxus SPA + spec-http backend
- priority: `high`
- summary: A single-process, GPU-accelerated web application for **reading and navigating** the
- ref: `memory-viewers/.ticket/tickets/06399bb2-eb06-40a9-8de9-26f6d0854753/ticket.toml`


### Component: storage

<!-- ticket-index:entry id=995c5394-e892-4b33-870b-f53c2cff9e05 slug=done/storage digest=17c4940cc282 -->
#### [995c5394] Impl: FsWatcher background daemon loop — auto-reconcile on FS events
- ref: `memory-api/.ticket/tickets/995c5394-e892-4b33-870b-f53c2cff9e05/ticket.toml`

<!-- ticket-index:entry id=02a79934-3782-4840-bfb6-caec08ee00c7 slug=done/storage digest=d7230b9909cd -->
#### [02a79934] Impl: enforce explicit index_root for exec/exec-batch agent protocol
- ref: `memory-api/.ticket/tickets/02a79934-3782-4840-bfb6-caec08ee00c7/ticket.toml`

<!-- ticket-index:entry id=62c04e04-e976-4464-b7c9-f31bda78c0d5 slug=done/storage digest=b8981985d27c -->
#### [62c04e04] Impl: expose integrate_orphan(path) as public API + wire to reconciler
- ref: `memory-api/.ticket/tickets/62c04e04-e976-4464-b7c9-f31bda78c0d5/ticket.toml`

<!-- ticket-index:entry id=834fc3eb-589e-4f83-b5cd-501187aa4d5f slug=done/storage digest=9ef15ac9f809 -->
#### [834fc3eb] Impl: fix exec --batch — true rollback semantics on first failure
- ref: `memory-api/.ticket/tickets/834fc3eb-589e-4f83-b5cd-501187aa4d5f/ticket.toml`

<!-- ticket-index:entry id=ec355bad-85cc-4ceb-8d3f-57222934e871 slug=done/storage digest=2f5632c5a67e -->
#### [ec355bad] Impl: fix scan --reindex — clear stale Tantivy entries before rebuild
- ref: `memory-api/.ticket/tickets/ec355bad-85cc-4ceb-8d3f-57222934e871/ticket.toml`

<!-- ticket-index:entry id=9d0258de-bf87-4b7e-b8f0-e78f4fdf0b58 slug=done/storage digest=88467fe5324f -->
#### [9d0258de] [bootstrap] define backup and restore procedure for index plus history
- summary: The `.ticket/` directory has two distinct layers:
- ref: `memory-api/.ticket/tickets/9d0258de-bf87-4b7e-b8f0-e78f4fdf0b58/ticket.toml`

<!-- ticket-index:entry id=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1 slug=done/storage digest=37167ac9edf3 -->
#### [4f2d2a5e] [bootstrap] wire create/get/update/list/delete to storage backend
- summary: Status:** READY (Phase 0 formally closed)
- ref: `memory-api/.ticket/tickets/4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1/ticket.toml`


### Component: test-api

<!-- ticket-index:entry id=03ed4121-ec7e-4d5f-adb4-4d3846af8031 slug=done/test-api digest=a94e564d7969 -->
#### [03ed4121] [bench] Cross-domain benchmark matrix with per-operation latency budgets
- priority: `high`
- summary: Provide Criterion benchmarks for the same `domain × operation` matrix as the test suite, each asserting a reasonable maximum-latency budget, and ingest the results into `test-api`.
- ref: `memory-api/.ticket/tickets/03ed4121-ec7e-4d5f-adb4-4d3846af8031/ticket.toml`

<!-- ticket-index:entry id=93b8a331-da80-4fef-b13d-7f277cadb15f slug=done/test-api digest=43ffc073dac0 -->
#### [93b8a331] [design][test] Browser & TypeScript automated test integration strategy (design session)
- priority: `high`
- summary: Created `.spec/specs/9e823b76-cd60-4689-b772-649ebb3a34a1/` defining the repository-native subprocess runner, structured reporter/result adapter, provenance, retry/outcome, blocked-capability, artifa...
- ref: `memory-api/.ticket/tickets/93b8a331-da80-4fef-b13d-7f277cadb15f/ticket.toml`

<!-- ticket-index:entry id=9138f4e7-2757-4d23-9676-3306608a429e slug=done/test-api digest=572af78a7c21 -->
#### [9138f4e7] [memory-api] Representative fixture population — all domains, realistic volumes
- priority: `high`
- summary: Replace the 4-entity stub fixture with a **synthesized** representative workspace so operations run against real data instead of self-seeded throwaway stores.
- ref: `memory-api/.ticket/tickets/9138f4e7-2757-4d23-9676-3306608a429e/ticket.toml`

<!-- ticket-index:entry id=2b0f31e5-1067-45e9-93ff-ef273c26020e slug=done/test-api digest=e586a4bb2e11 -->
#### [2b0f31e5] [test-api] Benchmark result model + Criterion ingest + latency budgets
- priority: `high`
- summary: Let `test-api` record benchmark results (not just pass/fail validations) and compare them against per-operation maximum-latency budgets, ingesting existing Criterion output.
- ref: `memory-api/.ticket/tickets/2b0f31e5-1067-45e9-93ff-ef273c26020e/ticket.toml`

<!-- ticket-index:entry id=a03d8a97-2b31-4f1a-a2cb-b33848af2f2a slug=done/test-api digest=2195e29be310 -->
#### [a03d8a97] [test-api] Execution provenance + persisted per-cell evidence
- priority: `high`
- summary: Make stored executions individually discoverable and traceable to the exact test. Today the per-cell matrix executions are written to the ephemeral fixture tempdir and discarded; the committed roll-u...
- ref: `memory-api/.ticket/tickets/a03d8a97-2b31-4f1a-a2cb-b33848af2f2a/ticket.toml`

<!-- ticket-index:entry id=124c6621-b39d-4009-b599-a3e5503d08f6 slug=done/test-api digest=c0507082ba9e -->
#### [124c6621] [test-api] Execution timing model + slow-test query
- priority: `high`
- summary: Make `test-api` capture how long each validation took and let callers query for slow runs, so the unified surface can flag "unreasonably slow" operations.
- ref: `memory-api/.ticket/tickets/124c6621-b39d-4009-b599-a3e5503d08f6/ticket.toml`

<!-- ticket-index:entry id=90de77b1-7784-4b26-81d1-9039277a9bec slug=done/test-api digest=eed4c3e5c462 -->
#### [90de77b1] [test-api] Store-index generator — run/status/issue/timing summary
- priority: `high`
- summary: Generate a committed `test-api` store-index that summarizes all validation and benchmark runs — statuses, issues, and timings — following the domain-owned thin-generator architecture used by the othe...
- ref: `memory-api/.ticket/tickets/90de77b1-7784-4b26-81d1-9039277a9bec/ticket.toml`

<!-- ticket-index:entry id=1e8f6866-9dda-4b2c-9f41-27ac83ee61d5 slug=done/test-api digest=ac0ea71a84fa -->
#### [1e8f6866] [test-api] prune_execution_runs silently deletes unrelated ticket-linked evidence (global keep-2-runs policy)
- priority: `critical`
- summary: `TestStoreConfig::record_execution` (memory-api/crates/test-api/src/store.rs) unconditionally calls `self.prune_execution_runs(2)` after every write. This function computes, across **every** executio...
- ref: `memory-api/.ticket/tickets/1e8f6866-9dda-4b2c-9f41-27ac83ee61d5/ticket.toml`

<!-- ticket-index:entry id=7a524627-bb48-47c8-a3d8-9c8b9303f0f3 slug=done/test-api digest=4ccecab3ef4c -->
#### [7a524627] [test-api][log-api] Validation runner harness — cargo test/bench to executions + log capture
- priority: `high`
- summary: Provide a runner that executes test/benchmark suites, captures their output and timing, and records them into `test-api` (executions/benchmarks) and `log-api` (stdout/stderr/summary) — the "execute" ...
- ref: `memory-api/.ticket/tickets/7a524627-bb48-47c8-a3d8-9c8b9303f0f3/ticket.toml`

<!-- ticket-index:entry id=905d05ae-b367-44f8-9988-a671702d8a32 slug=done/test-api digest=418f6bb3a930 -->
#### [905d05ae] [test-api][test-mcp] Fix record/list .test store routing split; aggregate discoverable stores on read
- priority: `high`
- summary: `test-mcp`'s write tools (`test_record_spec`, `test_record_execution`) resolve their `.test` store root per-call from an explicit `workspace` argument via `config_for_workspace` (walks upward from th...
- ref: `memory-api/.ticket/tickets/905d05ae-b367-44f8-9988-a671702d8a32/ticket.toml`

<!-- ticket-index:entry id=26d6353a-6613-48ad-96c5-d4bd8ad5180f slug=done/test-api digest=9cb003af54c2 -->
#### [26d6353a] [test-cli][log] test/log CLI + audit failed & slow query surface
- priority: `medium`
- summary: Give `test-api` and `log-api` a user-facing surface (CLI, optional HTTP) and an audit query that answers: "which validations failed, and which are unreasonably slow?".
- ref: `memory-api/.ticket/tickets/26d6353a-6613-48ad-96c5-d4bd8ad5180f/ticket.toml`

<!-- ticket-index:entry id=260e37d7-ae90-43e4-871b-3c18937189ca slug=done/test-api digest=bb7241a067d9 -->
#### [260e37d7] [test] Audit hygiene for memory-matrix — split lib.rs, fix llvm-cov
- priority: `medium`
- summary: Clear the audit findings against `memory-matrix` so the crate passes a clean audit.
- ref: `memory-api/.ticket/tickets/260e37d7-ae90-43e4-871b-3c18937189ca/ticket.toml`

<!-- ticket-index:entry id=751f0e71-a857-484f-a45e-09717f086321 slug=done/test-api digest=571722d154cc -->
#### [751f0e71] [test] Cross-domain operation test matrix — get/search/CRUD/move/scan x all domains
- priority: `high`
- summary: Implement an end-to-end test matrix that exercises the basic operations of **every memory domain** against the representative fixture, recording each run as a `test-api` `ValidationExecution` with a ...
- ref: `memory-api/.ticket/tickets/751f0e71-a857-484f-a45e-09717f086321/ticket.toml`

<!-- ticket-index:entry id=387843e4-815e-4424-97fa-9855a464b5e6 slug=done/test-api digest=14e078374dbb -->
#### [387843e4] [test] Transport-layer e2e matrix — CLI/HTTP/MCP x domains
- priority: `high`
- summary: Validate each basic operation (`get`/`search`/CRUD/`move`/`scan`) through the **real transport surfaces** of every domain, not just in-process storage, so regressions in `ticket-http`, `spec-mcp`, `t...
- ref: `memory-api/.ticket/tickets/387843e4-815e-4424-97fa-9855a464b5e6/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=261e7567-e234-43d5-881b-c481e34131f8 slug=done/ticket-api digest=139137109d54 -->
#### [261e7567] API: Add author field to ticket history revisions in ticket-api
- priority: `medium`
- ref: `memory-api/.ticket/tickets/261e7567-e234-43d5-881b-c481e34131f8/ticket.toml`

<!-- ticket-index:entry id=09c5e822-740c-453e-91ae-07d01d897e15 slug=done/ticket-api digest=69844678a3fc -->
#### [09c5e822] Bug: scan --force does not prune orphan entries from redb index
- priority: `medium`
- ref: `memory-api/.ticket/tickets/09c5e822-740c-453e-91ae-07d01d897e15/ticket.toml`

<!-- ticket-index:entry id=bc74e91f-d9ef-42df-8b72-94bd4743944c slug=done/ticket-api digest=8d42e3c12d9a -->
#### [bc74e91f] Fix objective-part staleness on combined freeze+description write
- priority: `medium`
- summary: On a single combined call that both transitions a ticket `--to-state planned` and writes `--description ... --description-mode ...` in the same request, the newly materialized `objective` part file i...
- ref: `memory-api/.ticket/tickets/bc74e91f-d9ef-42df-8b72-94bd4743944c/ticket.toml`

<!-- ticket-index:entry id=f65f2b32-9297-4360-9ad7-deb75e7ea401 slug=done/ticket-api digest=61f469fe37e1 -->
#### [f65f2b32] Migrate existing descriptions into typed parts (dry-run then apply, lossless)
- priority: `medium`
- summary: Split the existing ticket corpus's monolithic descriptions into typed parts, moving only content that can be confidently classified and leaving everything else in `objective` verbatim, with a dry-run...
- ref: `memory-api/.ticket/tickets/f65f2b32-9297-4360-9ad7-deb75e7ea401/ticket.toml`

<!-- ticket-index:entry id=3d952036-efd4-4f36-a77f-6b7f5058a0a0 slug=done/ticket-api digest=cb894c3647ed -->
#### [3d952036] Part-addressed writes and mandatory description_mode (remove the replace default)
- priority: `high`
- summary: Make every ticket content write state its intent explicitly. Remove the `replace` default from `description_mode`, and expose part-addressed writes so recording a review never touches the objective.
- ref: `memory-api/.ticket/tickets/3d952036-efd4-4f36-a77f-6b7f5058a0a0/ticket.toml`

<!-- ticket-index:entry id=b88b1fc0-eabe-444e-8511-e3467a699849 slug=done/ticket-api digest=89742c26c796 -->
#### [b88b1fc0] Phase 1: Add schema fields (doc_category, tags, workflow_stage, priority, source_agent_files, bug_validity, phase)
- summary: Add 7 new fields to `crates/ticket-api/schemas/tracker-improvement.toml` to replace 30+ ad-hoc fields with structured, filterable schema fields.
- ref: `memory-api/.ticket/tickets/b88b1fc0-eabe-444e-8511-e3467a699849/ticket.toml`

<!-- ticket-index:entry id=1600e55e-1def-4e84-9f09-7b866b8ac99a slug=done/ticket-api digest=f4c472106b6a -->
#### [1600e55e] Phase 2: Copy plan descriptions into open plan tickets
- summary: For each open ticket that has a matching agent plan file but no `description.md`, copy the primary plan file as the ticket's description and set structured metadata fields.
- ref: `memory-api/.ticket/tickets/1600e55e-1def-4e84-9f09-7b866b8ac99a/ticket.toml`

<!-- ticket-index:entry id=a2ebab34-3001-4fec-8454-1f74421c3049 slug=done/ticket-api digest=3c5eb94b546f -->
#### [a2ebab34] Phase 3: Attach interview files as ticket assets
- summary: Copy interview files into `assets/interviews/` for their parent plan tickets. Interviews are supplementary to plans and should not be standalone tickets.
- ref: `memory-api/.ticket/tickets/a2ebab34-3001-4fec-8454-1f74421c3049/ticket.toml`

<!-- ticket-index:entry id=56d080d3-011b-4eea-86a2-bb528b2d683f slug=done/ticket-api digest=f8b14dcbc8ec -->
#### [56d080d3] Phase 4: Copy descriptions for bootstrap tickets
- summary: Copy research phase docs as descriptions for the 13 bootstrap tickets that lack them. Set `doc_category=research`, `workflow_stage=plan`.
- ref: `memory-api/.ticket/tickets/56d080d3-011b-4eea-86a2-bb528b2d683f/ticket.toml`

<!-- ticket-index:entry id=b682a57c-8c5c-4763-af50-1c70cff2df46 slug=done/ticket-api digest=2f6f7af56e00 -->
#### [b682a57c] Phase 5: Enrich bug tickets with bug_validity and reproduction tracking
- summary: Enrich bug tickets with structured validity tracking and reproduction status, replacing the informal confidence emoji system.
- ref: `memory-api/.ticket/tickets/b682a57c-8c5c-4763-af50-1c70cff2df46/ticket.toml`

<!-- ticket-index:entry id=5afd39bf-276c-4b3a-a1e4-f9b3b6643483 slug=done/ticket-api digest=9f014524a092 -->
#### [5afd39bf] Phase 6: Cleanup stale tickets and deduplicate
- summary: Clean up the ticket store by cancelling stale tickets whose agent files were deleted, merging duplicates, and ensuring all tickets with descriptions have `doc_category` set.
- ref: `memory-api/.ticket/tickets/5afd39bf-276c-4b3a-a1e4-f9b3b6643483/ticket.toml`

<!-- ticket-index:entry id=f9e70385-adb7-4942-a8fb-6a383863cc7e slug=done/ticket-api digest=375ed585d994 -->
#### [f9e70385] Plan freezing at `planned`: hard reject, amendment parts, unfreeze by state transition
- priority: `high`
- summary: Freeze a ticket's planning parts when it enters `planned`, hard-reject writes to frozen parts, route corrections into `amendment` parts, and unfreeze only by transitioning the ticket back to a pre-`p...
- ref: `memory-api/.ticket/tickets/f9e70385-adb7-4942-a8fb-6a383863cc7e/ticket.toml`

<!-- ticket-index:entry id=6bb1e3fd-646d-424c-a216-826cf5f06867 slug=done/ticket-api digest=2b208d823861 -->
#### [6bb1e3fd] Plan: Migrate agent files into ticket system — schema improvements + content migration
- summary: Migrate the 201 agent documentation files from `agents/` into the ticket system as structured, filterable ticket content. Add schema fields so all tickets can be queried by category, tags, workflow s...
- ref: `memory-api/.ticket/tickets/6bb1e3fd-646d-424c-a216-826cf5f06867/ticket.toml`

<!-- ticket-index:entry id=4c7b884e-fd9b-4967-9599-5b55495d6e52 slug=done/ticket-api digest=17f8a8766735 -->
#### [4c7b884e] Projected ticket reads: summary/plan/review/full profiles and explicit part lists
- priority: `high`
- summary: Let an agent read only the parts of a ticket its role needs, via four named view profiles or an explicit part list, across the API, CLI, and MCP surfaces.
- ref: `memory-api/.ticket/tickets/4c7b884e-fd9b-4967-9599-5b55495d6e52/ticket.toml`

<!-- ticket-index:entry id=5b3da351-1c87-4619-a0bc-6d7abe147d60 slug=done/ticket-api digest=3c22e9d362a0 -->
#### [5b3da351] Rename ticket lifecycle states: new -> open, ready -> planned, with store migration
- priority: `high`
- summary: Renamed ticket lifecycle states `new`->`open` and `ready`->`planned` across the schema, code, and both real ticket stores.
- ref: `memory-api/.ticket/tickets/5b3da351-1c87-4619-a0bc-6d7abe147d60/ticket.toml`

<!-- ticket-index:entry id=5a3d152c-faf7-4d33-8a4e-7ed19cf6b142 slug=done/ticket-api digest=638d269fb4b9 -->
#### [5a3d152c] Ticket parts: parts/ storage, [[parts]] manifest index, and core kind vocabulary
- priority: `high`
- summary: Give a ticket directory a `parts/` folder holding one markdown file per typed content part, indexed by a `[[parts]]` table in `ticket.toml`, with a schema-validated core kind vocabulary and lossless ...
- ref: `memory-api/.ticket/tickets/5a3d152c-faf7-4d33-8a4e-7ed19cf6b142/ticket.toml`

<!-- ticket-index:entry id=9d69e93d-b7ab-4f88-a88c-40ec76d5206b slug=done/ticket-api digest=63556dc593ab -->
#### [9d69e93d] Typed [[refs]] manifest table for external entity references
- priority: `medium`
- summary: Give tickets a typed `[[refs]]` manifest table for references to non-ticket entities, so a ticket reaches external context by pointer instead of inlining it, and absorb the existing untyped `related_...
- ref: `memory-api/.ticket/tickets/9d69e93d-b7ab-4f88-a88c-40ec76d5206b/ticket.toml`

<!-- ticket-index:entry id=c5e9bb39-d784-4d0c-8de1-3885013cddce slug=done/ticket-api digest=b7f00924d6aa -->
#### [c5e9bb39] [memory-index] Ticket store index generator with git hook integration
- priority: `high`
- summary: Build a generator that reads the ticket store (ticket-api) and emits a committed markdown index co-located in `.ticket/README.md` along with its TOON sidecar at `.ticket/index.toon`. The purpose is t...
- ref: `.ticket/tickets/c5e9bb39-d784-4d0c-8de1-3885013cddce/ticket.toml`

<!-- ticket-index:entry id=82652305-ab94-4270-847c-a5209c2dcd44 slug=done/ticket-api digest=2960a64a0614 -->
#### [82652305] [spec][P0] Refactor ticket-api to depend on memory-api
- priority: `critical`
- summary: After memory-api is extracted, refactor ticket-api to be a thin domain layer on top of memory-api, keeping only ticket-specific logic.
- ref: `memory-api/.ticket/tickets/82652305-ab94-4270-847c-a5209c2dcd44/ticket.toml`

<!-- ticket-index:entry id=731f3579-0678-4eb8-a45c-53020830fba8 slug=done/ticket-api digest=a451f6217d0c -->
#### [731f3579] [ticket-api] Add built-in `task` schema and health check for ticket types missing a schema
- summary: Many tickets have `type = "task"`, but no schema was registered for the `task` type. Transitions/close operations failed with `no schema for type 'task'` (see the schema lookup in `memory-api/crates/...
- ref: `.ticket/tickets/731f3579-0678-4eb8-a45c-53020830fba8/ticket.toml`

<!-- ticket-index:entry id=bf62e2f9-7bdb-471d-a8c3-e160fe87e610 slug=done/ticket-api digest=5eda1c4b4833 -->
#### [bf62e2f9] [ticket-api] Add explicit replace/append mode to description update and always capture pre-overwrite description in history
- priority: `high`
- summary: Closing evidence: Governing spec 1f77f652-f883-4782-940a-39874dfe1382 (ticket-api/storage/description-update-modes, state=reviewed, links this ticket). Validation: `rtk cargo test -p ticket-api` → 13...
- ref: `.ticket/tickets/bf62e2f9-7bdb-471d-a8c3-e160fe87e610/ticket.toml`

<!-- ticket-index:entry id=bc691249-5a2d-409e-8e7b-2602d80cf61e slug=done/ticket-api digest=2212dca3509c -->
#### [bc691249] [ticket-api] Add journaled storage-layer execution for cross-workspace ticket moves
- priority: `high`
- summary: Execute a supported move safely at the storage layer, with resumable journal state and rollback when a step fails.
- ref: `memory-api/.ticket/tickets/bc691249-5a2d-409e-8e7b-2602d80cf61e/ticket.toml`

<!-- ticket-index:entry id=eb6033a8-f15b-4024-952e-5c86dc108939 slug=done/ticket-api digest=2794eb95fff0 -->
#### [eb6033a8] [ticket-api] Add move preflight planner and destination-visibility validation
- priority: `high`
- summary: Build the read-only planning layer for `ticket move` that decides whether a move is supported and enumerates every object the execution phase would touch.
- ref: `memory-api/.ticket/tickets/eb6033a8-f15b-4024-952e-5c86dc108939/ticket.toml`

<!-- ticket-index:entry id=013b57bd-2e8c-4d4d-87c8-6f8687a195c8 slug=done/ticket-api digest=9a7c4da33c6a -->
#### [013b57bd] [ticket-api] Add targeted incremental reconcile modes for move and maintenance tooling
- priority: `high`
- summary: Provide targeted reconcile/scan modes so move flows and internal tooling can update only the touched ticket set instead of walking unrelated roots.
- ref: `.ticket/tickets/013b57bd-2e8c-4d4d-87c8-6f8687a195c8/ticket.toml`

<!-- ticket-index:entry id=429f6f1d-6429-4601-bfac-b572fdb4dbff slug=done/ticket-api digest=d18f72cf7629 -->
#### [429f6f1d] [ticket-api] Child workspaces surface parent dependency entries
- priority: `high`
- summary: Child memory workspaces such as `ticket-api` can open their own local store, but cross-workspace dependency and graph views still assume both ends of a relationship can be resolved inside the active ...
- ref: `memory-api/.ticket/tickets/429f6f1d-6429-4601-bfac-b572fdb4dbff/ticket.toml`

<!-- ticket-index:entry id=13e9ce28-ff35-4898-8dda-6d333dc1f222 slug=done/ticket-api digest=5d38acbbfc63 -->
#### [13e9ce28] [ticket-api] Cross-workspace move + automatic reference re-linking for store entries
- priority: `high`
- summary: Provide a first-class, safe operation to move a ticket from one `memory-api` workspace store to another and automatically preserve the references that can be preserved safely, while refusing unsuppor...
- ref: `memory-api/.ticket/tickets/13e9ce28-ff35-4898-8dda-6d333dc1f222/ticket.toml`

<!-- ticket-index:entry id=505b2cd4-f21d-4e8d-8e6a-ae06a5b69854 slug=done/ticket-api digest=295425d7f353 -->
#### [505b2cd4] [ticket-api] Deliver safe cross-workspace ticket move for git-backed stores
- priority: `high`
- summary: Implement a safe, reviewable `ticket move` capability for git-backed `memory-api` workspaces that relocates a ticket into a different workspace store and preserves correctness of the owning store, re...
- ref: `memory-api/.ticket/tickets/505b2cd4-f21d-4e8d-8e6a-ae06a5b69854/ticket.toml`

<!-- ticket-index:entry id=3b6a2a26-bd4e-44ce-ba15-41594b809b9a slug=done/ticket-api digest=c618c5822a2f -->
#### [3b6a2a26] [ticket-api] Derive blocker and unlock trees with frontier leaf metrics
- priority: `high`
- summary: Extend the shared workflow layer with explicit blocker and unlock tree derivation.
- ref: `.ticket/tickets/3b6a2a26-bd4e-44ce-ba15-41594b809b9a/ticket.toml`

<!-- ticket-index:entry id=22cd3001-0127-4a27-8834-721250ff39ff slug=done/ticket-api digest=f97b379fe44e -->
#### [22cd3001] [ticket-api] Enforce board safety and migrate historical board rows during ticket moves
- priority: `medium`
- summary: Handle board-state references safely during a ticket move so live ownership is never silently lost.
- ref: `memory-api/.ticket/tickets/22cd3001-0127-4a27-8834-721250ff39ff/ticket.toml`

<!-- ticket-index:entry id=a4c31280-66d3-44a3-9a5d-13d4fbde1bfe slug=done/ticket-api digest=e0a6cf803f70 -->
#### [a4c31280] [ticket-api] Fix health false positives for description and resolved dependencies
- priority: `high`
- summary: Fix ticket health false positives across CLI, HTTP, and MCP surfaces.
- ref: `.ticket/tickets/a4c31280-66d3-44a3-9a5d-13d4fbde1bfe/ticket.toml`

<!-- ticket-index:entry id=3e4718af-3fd3-40a4-ac89-d298c99c806a slug=done/ticket-api digest=7de841de9209 -->
#### [3e4718af] [ticket-api] Make workflow-facts recompute incremental during scan and move reconcile
- priority: `high`
- summary: Reduce `rebuild_workflow_facts_ms` by recomputing only the workflow facts touched by a changed ticket or move operation.
- ref: `.ticket/tickets/3e4718af-3fd3-40a4-ac89-d298c99c806a/ticket.toml`

<!-- ticket-index:entry id=3d72029b-cf2d-49bb-9dde-00587304b857 slug=done/ticket-api digest=b722d9631b9a -->
#### [3d72029b] [ticket-api] Materialize recent-unblock and blocker-progress facts
- priority: `high`
- summary: Materialize recent-unblock and blocker-progress workflow facts for scalable ordering.
- ref: `.ticket/tickets/3d72029b-cf2d-49bb-9dde-00587304b857/ticket.toml`

<!-- ticket-index:entry id=deeeb26d-cb73-46c5-bf2a-1778caa7f82a slug=done/ticket-api digest=9dad278f6d66 -->
#### [deeeb26d] [ticket-api] Persist dependency edges in tracked ticket files
- priority: `high`
- summary: `ticket link` and `ticket unlink` currently mutate dependency edges only in the ignored `.ticket/tickets.db` SQLite index.
- ref: `memory-api/.ticket/tickets/deeeb26d-cb73-46c5-bf2a-1778caa7f82a/ticket.toml`

<!-- ticket-index:entry id=dd2947da-d4d2-4c8a-9a9a-3633060ff4c5 slug=done/ticket-api digest=f4b8498bffc3 -->
#### [dd2947da] [ticket-api] Reconcile aggregate scan, prune, and search visibility
- priority: `high`
- summary: Make aggregate scan the single source of truth for both index visibility and search visibility.
- ref: `.ticket/tickets/dd2947da-d4d2-4c8a-9a9a-3633060ff4c5/ticket.toml`

<!-- ticket-index:entry id=3a26572a-5e1a-4a57-aefa-9b342886a5ca slug=done/ticket-api digest=ac65c045a86e -->
#### [3a26572a] [ticket-api] Rewrite repo path references that cite the moved ticket folder
- priority: `medium`
- summary: Automatically update repo-local text references that cite the moved ticket's old folder path so specs, tests, and docs do not point at a stale store location after the move.
- ref: `memory-api/.ticket/tickets/3a26572a-5e1a-4a57-aefa-9b342886a5ca/ticket.toml`

<!-- ticket-index:entry id=385f2521-b318-403b-a4ea-195a47e5c453 slug=done/ticket-api digest=b4e288cf05ff -->
#### [385f2521] [ticket-api] Unify multi-step state transitions across update and close flows
- priority: `high`
- summary: `ticket update` currently enforces a single-step state transition and optionally accepts `from_state`, which duplicates the current store state and rejects legitimate fast-forward workflows such as `...
- ref: `.ticket/tickets/385f2521-b318-403b-a4ea-195a47e5c453/ticket.toml`

<!-- ticket-index:entry id=da27c074-8c9e-4613-b8b9-bf02c72b50f7 slug=done/ticket-api digest=5f4d5fd7e878 -->
#### [da27c074] [ticket-api] Validate cross-workspace ticket move flows end to end
- priority: `high`
- summary: Validated the injected-failure recovery and transport smoke coverage for cross-workspace ticket move flows.
- ref: `memory-api/.ticket/tickets/da27c074-8c9e-4613-b8b9-bf02c72b50f7/ticket.toml`

<!-- ticket-index:entry id=27a697db-ffa4-491f-b489-e3ed3a1ae261 slug=done/ticket-api digest=873fc5eaa7f7 -->
#### [27a697db] [ticket-api] `cargo test -p ticket` broken: TicketStore has no Default impl (since ae09e93)
- summary: `cargo test -p ticket` does not compile. [memory-api/crates/ticket/src/lib.rs](memory-api/crates/ticket/src/lib.rs#L10) contains a `#[cfg(test)]` assertion `let _ = storage::TicketStore::default;`, b...
- ref: `memory-api/.ticket/tickets/27a697db-ffa4-491f-b489-e3ed3a1ae261/ticket.toml`

<!-- ticket-index:entry id=c060bf94-2435-4cc5-8016-ca1d2c8264f5 slug=done/ticket-api digest=c97979f12772 -->
#### [c060bf94] [ticket-api][board][session-api] Bind board entries to sessions and worktrees; add active-worktree discovery
- priority: `high`
- summary: Problem: the branch-and-worktree isolation protocol is guidance-only. The draftboard cannot answer "which worktree is this ticket being worked in", "which session owns that worktree", or "which workt...
- ref: `.ticket/tickets/c060bf94-2435-4cc5-8016-ca1d2c8264f5/ticket.toml`

<!-- ticket-index:entry id=7f4aaa05-584a-42a8-a06f-d0b263ad929e slug=done/ticket-api digest=1bf195f536a9 -->
#### [7f4aaa05] [ticket-api][bug] update_ticket resets state to new on field/description patch; transition_states silently no-ops
- priority: `high`
- summary: Fixed the state machine regression in ticket-api `update()` function where:
- ref: `memory-api/.ticket/tickets/7f4aaa05-584a-42a8-a06f-d0b263ad929e/ticket.toml`

<!-- ticket-index:entry id=875919d5-558c-46a8-a83f-02a6756a1e0e slug=done/ticket-api digest=a6fc3c6cc990 -->
#### [875919d5] [ticket-api][memory-api] Batch residual SQLite and index writes during scan reconciliation
- priority: `high`
- summary: Remove avoidable per-entity SQLite and remaining index-write churn from scan reconciliation after the Tantivy batching fix.
- ref: `.ticket/tickets/875919d5-558c-46a8-a83f-02a6756a1e0e/ticket.toml`

<!-- ticket-index:entry id=bf094901-cdb6-4b25-8ccd-3eb7716f9d20 slug=done/ticket-api digest=de2ae1225e6f -->
#### [bf094901] [ticket-api][memory-api] Eliminate full-store rescans and per-document search commits from scan/move hot paths
- priority: `high`
- summary: Remove the major performance regressions surfaced by the move-domain E2E and perf scan tests in the ticket store hot path.
- ref: `.ticket/tickets/bf094901-cdb6-4b25-8ccd-3eb7716f9d20/ticket.toml`

<!-- ticket-index:entry id=23f4e2eb-e916-401e-b398-9ad59c06c5e3 slug=done/ticket-api digest=f4da12a03e25 -->
#### [23f4e2eb] [ticket-api][memory-api] Investigate & fix ~100s ticket get latency
- priority: `critical`
- summary: Root-cause and fix the severe per-invocation latency of `ticket get` (and likely all CLI store operations), measured at **96–107s** for a single get in this workspace, and establish a defended latenc...
- ref: `memory-api/.ticket/tickets/23f4e2eb-e916-401e-b398-9ad59c06c5e3/ticket.toml`

<!-- ticket-index:entry id=cadf78e8-a243-4d1c-8c1b-451978bb05ea slug=done/ticket-api digest=28c1bf104e7c -->
#### [cadf78e8] [ticket-api][memory-api] Track remaining store scan/index/move performance follow-ups
- priority: `high`
- summary: Tracks remaining store scan/index/move performance follow-ups after baseline hot-path fixes.
- ref: `.ticket/tickets/cadf78e8-a243-4d1c-8c1b-451978bb05ea/ticket.toml`

<!-- ticket-index:entry id=dc70628a-3a84-4441-ad46-f59b7757e1f7 slug=done/ticket-api digest=e13b15268d00 -->
#### [dc70628a] [ticket-api][spec] Extend workspace-ownership specs with move/relink contract
- priority: `high`
- summary: Update the existing `memory-api` recurring-principles specs for `nested-workspace-resolution` and `workspace-identifiers` so the move operation has an explicit contract before implementation lands.
- ref: `memory-api/.ticket/tickets/dc70628a-3a84-4441-ad46-f59b7757e1f7/ticket.toml`

<!-- ticket-index:entry id=d74412e4-1d0e-4679-8725-e5da6f266fe9 slug=done/ticket-api digest=a508ffe796b4 -->
#### [d74412e4] [ticket-api][ticket-cli] Blueprint blocker trees and recently-unblocked workflow ordering
- priority: `high`
- summary: Create the full implementation blueprint for blocker-tree workflow exploration and recently-unblocked ordering.
- ref: `.ticket/tickets/d74412e4-1d0e-4679-8725-e5da6f266fe9/ticket.toml`

<!-- ticket-index:entry id=d1f9f390-dda0-4762-a14c-9ce339abc393 slug=done/ticket-api digest=a45ddb596a8c -->
#### [d1f9f390] [ticket-api][ticket-cli][ticket-mcp] Redesign best-next ranking around dependency convergence
- priority: `high`
- summary: `ticket next`, `ticket board show`, and `ticket-mcp next_tickets` currently use the dependees-first contract documented in the current best-next spec: candidate workflow progress, then priority, then...
- ref: `memory-api/.ticket/tickets/d1f9f390-dda0-4762-a14c-9ce339abc393/ticket.toml`

<!-- ticket-index:entry id=c031aeb0-f374-4d57-9d46-2463dfa8571d slug=done/ticket-api digest=cf1e688e9cff -->
#### [c031aeb0] [ticket-api][ticket-cli][ticket-mcp][ticket-http] Define minimal workflow and health core plus adapter responsibilities
- priority: `high`
- summary: The same ticket store can still produce different workflow and health answers because parity-critical domain behavior is split across the interface crates.
- ref: `.ticket/tickets/c031aeb0-f374-4d57-9d46-2463dfa8571d/ticket.toml`

<!-- ticket-index:entry id=0e375356-b74e-48c4-8f1d-77cd28e055bc slug=done/ticket-api digest=3fd6252087da -->
#### [0e375356] [ticket-api][ticket-cli][ticket-mcp][ticket-http] Implement scoped selectors for board and next
- priority: `high`
- summary: The workflow discovery surfaces currently build scope locally and inconsistently.
- ref: `.ticket/tickets/0e375356-b74e-48c4-8f1d-77cd28e055bc/ticket.toml`

<!-- ticket-index:entry id=cf4246c3-6539-4f1c-a876-6d34073db7b3 slug=done/ticket-api digest=0befd5086500 -->
#### [cf4246c3] [ticket-api][ticket-cli][ticket-mcp][ticket-http] Track workflow and health surface convergence
- priority: `high`
- summary: Turn the current workflow and health surface inconsistency diagnosis into one coordinated implementation track with a ticket-api-owned minimal core, explicit adapter-boundary cleanup, and reproducibl...
- ref: `.ticket/tickets/cf4246c3-6539-4f1c-a876-6d34073db7b3/ticket.toml`

<!-- ticket-index:entry id=4a48b371-7dc0-4bf2-badb-747a8f00a0fc slug=done/ticket-api digest=b0d6c7ff5ca3 -->
#### [4a48b371] [ticket-api][ticket-cli][ticket-mcp][ticket-http] Unify board-aware next filtering across workflow surfaces
- priority: `high`
- summary: `ticket next` and MCP `next_tickets` already apply board-aware filtering: tickets
- ref: `.ticket/tickets/4a48b371-7dc0-4bf2-badb-747a8f00a0fc/ticket.toml`

<!-- ticket-index:entry id=cb562da4-a873-4537-9301-b800b2ab660d slug=done/ticket-api digest=6d74385afb86 -->
#### [cb562da4] [ticket-api][ticket-http][ticket-viewer] Design: canonical workspace identity and search consistency
- priority: `high`
- summary: Recent fixes closed two local failures but left the larger design problem intact:
- ref: `.ticket/tickets/cb562da4-a873-4537-9301-b800b2ab660d/ticket.toml`

<!-- ticket-index:entry id=11450369-0d45-4922-988f-49bc88fd4079 slug=done/ticket-api digest=b7657c2eff9d -->
#### [11450369] [ticket-cli] Render board show recommendations as pretty cards
- priority: `high`
- summary: Render recommendation lists with the same compact pretty-card layout anywhere the CLI shows human-readable next-work candidates.
- ref: `memory-api/.ticket/tickets/11450369-0d45-4922-988f-49bc88fd4079/ticket.toml`

<!-- ticket-index:entry id=6484d4b7-e24b-4c13-999c-d0b00928d97c slug=done/ticket-api digest=5d0d8e5daefc -->
#### [6484d4b7] [ticket-cli][ticket-http][ticket-mcp] Build larger-integration parity routine for workflow and health surfaces
- priority: `high`
- summary: The current validation approach mostly proves ticket-cli, ticket-http, and ticket-mcp in isolation.
- ref: `.ticket/tickets/6484d4b7-e24b-4c13-999c-d0b00928d97c/ticket.toml`

<!-- ticket-index:entry id=27558fde-37b0-43eb-86c6-cfbe2d99a0b8 slug=done/ticket-api digest=2a5bc42147b1 -->
#### [27558fde] [ticket-mcp][ticket-http] Workspace-resolution parity — nested-root awareness + pure transport (first run)
- priority: `high`
- summary: The first parity run. Establishes the shared-resolver adoption + pure-transport audit pattern that the spec/rule/audit domains will reuse.
- ref: `memory-api/.ticket/tickets/27558fde-37b0-43eb-86c6-cfbe2d99a0b8/ticket.toml`

<!-- ticket-index:entry id=7599ed31-598e-458e-8651-9bfe6c57ffd9 slug=done/ticket-api digest=a24fa3ffb489 -->
#### [7599ed31] [ticket-store][cleanup] Migrate misplaced context-engine-workspace tickets into the memory-api store
- priority: `high`
- summary: Every entity must live in the **lowest-level store that contains all of the code the entity is concerned with**:
- ref: `memory-api/.ticket/tickets/7599ed31-598e-458e-8651-9bfe6c57ffd9/ticket.toml`

<!-- ticket-index:entry id=2d85467b-23a3-4a70-a376-70ef5370d9f8 slug=done/ticket-api digest=2cae667c79fc -->
#### [2d85467b] [ticket-system] Add dependees to next-ticket ordering
- priority: `high`
- summary: Add an incoming-dependees ranking key to best-next ticket selection. Compute dependees as the count of depends_on edges whose target is the candidate ticket. Keep workflow progress first and priority...
- ref: `memory-api/.ticket/tickets/2d85467b-23a3-4a70-a376-70ef5370d9f8/ticket.toml`

<!-- ticket-index:entry id=77629631-8076-4fca-9640-316583ff290c slug=done/ticket-api digest=5c92267323a5 -->
#### [77629631] [ticket-system] Expose ordering keys on priority-sorted outputs
- priority: `high`
- summary: Expose the full best-next ordering metadata anywhere the CLI surfaces the priority-sorted board recommendation list. Preserve dependees and created_at when board show rewraps next candidates, and ren...
- ref: `memory-api/.ticket/tickets/77629631-8076-4fca-9640-316583ff290c/ticket.toml`

<!-- ticket-index:entry id=a62f28bd-8600-473f-a831-de4736ffc219 slug=done/ticket-api digest=11602bac6d43 -->
#### [a62f28bd] [ticket-system] Per-ticket workflow paths: required_states field
- priority: `high`
- ref: `memory-api/.ticket/tickets/a62f28bd-8600-473f-a831-de4736ffc219/ticket.toml`


### Component: ticket-api,spec-api

<!-- ticket-index:entry id=e82b4f88-45e1-402b-ab59-de845c4930e0 slug=done/ticket-api,spec-api digest=c5bc96839b34 -->
#### [e82b4f88] Structured ticket↔spec linking with validation to replace prose-only references
- priority: `high`
- summary: Review pass: all 7 ACs met; structured ticket-spec links are verified.
- ref: `.ticket/tickets/e82b4f88-45e1-402b-ab59-de845c4930e0/ticket.toml`


### Component: ticket-api,ticket-cli

<!-- ticket-index:entry id=a98ea0e1-d3e8-47e4-aa28-b6a39296cd45 slug=done/ticket-api,ticket-cli digest=a94ee7ed8999 -->
#### [a98ea0e1] [ticket-system] Force sync: reconcile index from disk ticket.toml
- priority: `medium`
- ref: `.ticket/tickets/a98ea0e1-d3e8-47e4-aa28-b6a39296cd45/ticket.toml`

<!-- ticket-index:entry id=a3cc8e3e-7cb9-413c-a4df-966df77859d5 slug=done/ticket-api,ticket-cli digest=676950c55d03 -->
#### [a3cc8e3e] [ticket-system] Undo support: revert last update via --undo flag
- priority: `high`
- ref: `.ticket/tickets/a3cc8e3e-7cb9-413c-a4df-966df77859d5/ticket.toml`


### Component: ticket-cli

<!-- ticket-index:entry id=6f8bcf0a-c5e3-423c-b3cd-190a5bb0b18f slug=done/ticket-cli digest=f97a4235ec8b -->
#### [6f8bcf0a] Plan: ticket attach — asset file management for tickets
- priority: `medium`
- summary: Tickets can have an `assets/` directory for supplementary files (interview transcripts,
- ref: `memory-api/.ticket/tickets/6f8bcf0a-c5e3-423c-b3cd-190a5bb0b18f/ticket.toml`

<!-- ticket-index:entry id=b0056fa6-bdb3-40a6-acfb-9c96dd1ca82f slug=done/ticket-cli digest=32e85be926de -->
#### [b0056fa6] Plan: ticket audit — store health check and statistics
- priority: `medium`
- summary: After the migration, a 90-line Python script was needed to audit the ticket store:
- ref: `memory-api/.ticket/tickets/b0056fa6-bdb3-40a6-acfb-9c96dd1ca82f/ticket.toml`

<!-- ticket-index:entry id=b11cde49-100f-4443-95d9-6d9c30d21622 slug=done/ticket-cli digest=5c2c56c7ae56 -->
#### [b11cde49] Plan: ticket batch-update — bulk field updates with filter
- priority: `medium`
- summary: The existing `batch` command takes NDJSON `TaskCommand` objects, which is powerful
- ref: `memory-api/.ticket/tickets/b11cde49-100f-4443-95d9-6d9c30d21622/ticket.toml`

<!-- ticket-index:entry id=f77ff07e-c250-4740-8da8-8cf065564f8a slug=done/ticket-cli digest=b466f8b97c77 -->
#### [f77ff07e] Plan: ticket close / state fast-forward — skip intermediate states
- priority: `high`
- summary: The tracker-improvement schema has 11 states with a strict transition chain:
- ref: `memory-api/.ticket/tickets/f77ff07e-c250-4740-8da8-8cf065564f8a/ticket.toml`

<!-- ticket-index:entry id=a48475e3-42fc-44f6-88b2-0f4a86930a31 slug=done/ticket-cli digest=afabcf2b29eb -->
#### [a48475e3] Plan: ticket list --where — structured field-value filtering
- priority: `high`
- summary: `ticket list` currently supports `--state` and `--type` filters only.
- ref: `memory-api/.ticket/tickets/a48475e3-42fc-44f6-88b2-0f4a86930a31/ticket.toml`

<!-- ticket-index:entry id=53176121-eb55-4aa9-a1d6-5075db1c163b slug=done/ticket-cli digest=56b5f9bedded -->
#### [53176121] [ticket-cli] Add `ticket move` CLI with dry-run and recovery guidance
- priority: `high`
- summary: Expose the move planner and executor through `ticket-cli` so operators can preview, apply, resume, and roll back a move from the command line.
- ref: `memory-api/.ticket/tickets/53176121-eb55-4aa9-a1d6-5075db1c163b/ticket.toml`

<!-- ticket-index:entry id=d39e9e08-5104-461b-83ff-bd4361e967d9 slug=done/ticket-cli digest=5fc71c9f4ab9 -->
#### [d39e9e08] [ticket-cli] Add blockers command and nested tree rendering
- priority: `high`
- summary: Add an upstream `ticket blockers <id>` command and upgrade `ticket unblocked-by <id>` to nested tree output.
- ref: `.ticket/tickets/d39e9e08-5104-461b-83ff-bd4361e967d9/ticket.toml`

<!-- ticket-index:entry id=40282486-bd98-4f3b-8bb5-96cfe853e247 slug=done/ticket-cli digest=0c589097b675 -->
#### [40282486] [ticket-cli] Add reverse-dependency follow-up queries for next and unblocked-by
- priority: `high`
- summary: Users can ask which tickets were unblocked by finishing a dependency, but the current CLI requires a manual topgraph plus per-ticket health fan-out to answer that question.
- ref: `memory-api/.ticket/tickets/40282486-bd98-4f3b-8bb5-96cfe853e247/ticket.toml`

<!-- ticket-index:entry id=8de93812-3a8c-4937-9f09-05a9a9b86309 slug=done/ticket-cli digest=5ae851523a53 -->
#### [8de93812] [ticket-cli] Canonicalize board subcommand option naming
- priority: `medium`
- ref: `.ticket/tickets/8de93812-3a8c-4937-9f09-05a9a9b86309/ticket.toml`

<!-- ticket-index:entry id=74fd59ca-8253-4e18-99bd-0b1fa204c6d6 slug=done/ticket-cli digest=6e7d0da49165 -->
#### [74fd59ca] [ticket-cli] Remove constant blocker-progress field from board show JSON recommendations
- priority: `medium`
- summary: `ticket board show --json` currently includes `recommended_next[].last_blocker_progress_at`, but recommended-next items are sourced from the actionable queue where that field is always null by contra...
- ref: `.ticket/tickets/74fd59ca-8253-4e18-99bd-0b1fa204c6d6/ticket.toml`

<!-- ticket-index:entry id=91011568-ae0b-4b23-b060-b0c018e1e912 slug=done/ticket-cli digest=ae11687e5eab -->
#### [91011568] [ticket-cli][ticket-mcp] Expose authoritative ticket folder paths in query output
- priority: `high`
- ref: `memory-api/.ticket/tickets/91011568-ae0b-4b23-b060-b0c018e1e912/ticket.toml`

<!-- ticket-index:entry id=15837e16-8755-4eb1-8b36-6c4453899e46 slug=done/ticket-cli digest=c40a48b4a4f8 -->
#### [15837e16] [ticket-cli][ticket-mcp] Integrate recent-unblock ordering into workflow surfaces
- priority: `high`
- summary: Integrate recent-unblock ordering and tree metadata into prioritized workflow surfaces.
- ref: `.ticket/tickets/15837e16-8755-4eb1-8b36-6c4453899e46/ticket.toml`

<!-- ticket-index:entry id=129d4f4e-7db8-4c3b-87d5-de8ed12c0b09 slug=done/ticket-cli digest=8413e08031bd -->
#### [129d4f4e] [ticket-system] Next command: sort by workflow progress
- priority: `medium`
- ref: `memory-api/.ticket/tickets/129d4f4e-7db8-4c3b-87d5-de8ed12c0b09/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=8034efd8-e165-4798-afe1-3445026345d9 slug=done/ticket-http digest=70c236741191 -->
#### [8034efd8] API: Batch mutation endpoint for transactional multi-command execution
- priority: `high`
- ref: `memory-api/.ticket/tickets/8034efd8-e165-4798-afe1-3445026345d9/ticket.toml`

<!-- ticket-index:entry id=15871ee6-8e6b-40a0-8293-46d31deae3e8 slug=done/ticket-http digest=62b9e74b9a15 -->
#### [15871ee6] API: Edge mutation endpoints — add and remove edges
- priority: `high`
- ref: `memory-api/.ticket/tickets/15871ee6-8e6b-40a0-8293-46d31deae3e8/ticket.toml`

<!-- ticket-index:entry id=3fda11c3-978b-4f7c-9ee1-934a97debb12 slug=done/ticket-http digest=dbd45651894d -->
#### [3fda11c3] API: History and revert endpoints
- priority: `medium`
- ref: `memory-api/.ticket/tickets/3fda11c3-978b-4f7c-9ee1-934a97debb12/ticket.toml`

<!-- ticket-index:entry id=189a6068-7ccc-4daf-808e-6b0b82e97ef5 slug=done/ticket-http digest=2addc1effbb4 -->
#### [189a6068] API: Schema endpoint — types, states, transitions, fields
- priority: `high`
- ref: `memory-api/.ticket/tickets/189a6068-7ccc-4daf-808e-6b0b82e97ef5/ticket.toml`

<!-- ticket-index:entry id=69abd1c7-15a9-4d56-8156-0f09ff90783f slug=done/ticket-http digest=ffbd065a3138 -->
#### [69abd1c7] API: Ticket mutation endpoints — create, update, close, cancel, delete
- priority: `critical`
- ref: `memory-api/.ticket/tickets/69abd1c7-15a9-4d56-8156-0f09ff90783f/ticket.toml`

<!-- ticket-index:entry id=d3a8b66a-8efc-493e-9993-3b5a68b0a7f7 slug=done/ticket-http digest=098252654295 -->
#### [d3a8b66a] Impl: Add created_at to TicketSummary HTTP response
- summary: Add `created_at` field to the `TicketSummary` struct in the ticket-http handler so the frontend can sort tickets by creation date.
- ref: `memory-api/.ticket/tickets/d3a8b66a-8efc-493e-9993-3b5a68b0a7f7/ticket.toml`

<!-- ticket-index:entry id=23f1c81b-3c71-4b4b-9e6f-81ee7c43a30b slug=done/ticket-http digest=56ed4fd265d2 -->
#### [23f1c81b] [ticket-http] Add no-auto-init E2E for missing .ticket workspace
- priority: `high`
- summary: HTTP read probes against a configured workspace path with no on-disk `.ticket` root must not auto-create stores. The regression target is workspace-resolved read/list probes.
- ref: `memory-api/.ticket/tickets/23f1c81b-3c71-4b4b-9e6f-81ee7c43a30b/ticket.toml`

<!-- ticket-index:entry id=373a3317-4dfd-456a-a86e-523f4e7692f0 slug=done/ticket-http digest=214ebc5891a6 -->
#### [373a3317] [ticket-http] Add ticket move endpoint for workspace relocation
- priority: `medium`
- summary: Expose the move capability over HTTP for remote tooling and UI-driven workflows.
- ref: `memory-api/.ticket/tickets/373a3317-4dfd-456a-a86e-523f4e7692f0/ticket.toml`

<!-- ticket-index:entry id=700b9763-17f8-436e-ace0-45b88bedd1d7 slug=done/ticket-http digest=ecd2af160357 -->
#### [700b9763] [ticket-http] Design: workspace-aware ticket references for child-workspace frontend endpoints
- priority: `high`
- summary: The ticket-viewer and ticket-vscode frontends still model `workspace` as one route or query string that owns every returned record. On the server side both `ticket-viewer` and `ticket-http` resolve o...
- ref: `memory-api/.ticket/tickets/700b9763-17f8-436e-ace0-45b88bedd1d7/ticket.toml`

<!-- ticket-index:entry id=10cf2a19-356c-4e69-b0f3-b930d68dc0ce slug=done/ticket-http digest=4782d78fb596 -->
#### [10cf2a19] [ticket-http] Expose workflow trees and actionable ordering metadata
- priority: `high`
- summary: Add ticket-http parity for the blocker-tree and recently-unblocked workflow surfaces so browser clients can consume the same ordering and tree data as the CLI and MCP surfaces. This work should defin...
- ref: `.ticket/tickets/10cf2a19-356c-4e69-b0f3-b930d68dc0ce/ticket.toml`

<!-- ticket-index:entry id=8d95b98c-df79-46a7-affa-afa061c0dfff slug=done/ticket-http digest=8980fb345245 -->
#### [8d95b98c] [ticket-http] Fix child-owned workspace refs for viewer follow-up requests
- priority: `high`
- summary: Implemented child-workspace ownership fixes for ticket refs and follow-up reads.
- ref: `.ticket/tickets/8d95b98c-df79-46a7-affa-afa061c0dfff/ticket.toml`

<!-- ticket-index:entry id=fcf9eb04-394e-4b1b-acf2-4da54f3d3f6c slug=done/ticket-http digest=683a751e91d4 -->
#### [fcf9eb04] [ticket-http] Remove special default workspace naming and replace opaque server errors
- priority: `high`
- ref: `.ticket/tickets/fcf9eb04-394e-4b1b-acf2-4da54f3d3f6c/ticket.toml`

<!-- ticket-index:entry id=416ebd52-447d-44e4-a4ad-23162d37e0b1 slug=done/ticket-http digest=cc1014f2d06e -->
#### [416ebd52] [ticket-http] Return only authoritative resolved hits in workspace-aware search
- priority: `high`
- summary: HTTP query responses must only expose tickets that resolve to authoritative indexed paths and workspace ownership.
- ref: `.ticket/tickets/416ebd52-447d-44e4-a4ad-23162d37e0b1/ticket.toml`

<!-- ticket-index:entry id=cccf5d99-d7e9-43e6-8aea-90480ad3cf0d slug=done/ticket-http digest=e52c5feaf9a4 -->
#### [cccf5d99] [ticket-http][ticket-viewer] Bug: query results ignore active state filter
- priority: `high`
- summary: The ticket explorer currently fails to honor the active state filter once the user types a search query.
- ref: `memory-api/.ticket/tickets/cccf5d99-d7e9-43e6-8aea-90480ad3cf0d/ticket.toml`

<!-- ticket-index:entry id=397fa45b-a0bd-43d2-b430-2dfa44d80c5c slug=done/ticket-http digest=3450bceb26b7 -->
#### [397fa45b] [ticket-http][ticket-viewer] Expose workspace graph payload for focused full-graph navigation
- priority: `high`
- summary: Start implementing a workspace-scoped graph payload and frontend fetch contract for focused navigation.
- ref: `memory-viewers/.ticket/tickets/397fa45b-a0bd-43d2-b430-2dfa44d80c5c/ticket.toml`

<!-- ticket-index:entry id=3554ee9e-35fb-447d-8905-258298c37ef6 slug=done/ticket-http digest=138882017467 -->
#### [3554ee9e] [ticket-http][ticket-viewer] Introduce collision-safe public workspace identity
- priority: `high`
- summary: Replace basename-only workspace identity with a collision-safe public contract.
- ref: `.ticket/tickets/3554ee9e-35fb-447d-8905-258298c37ef6/ticket.toml`


### Component: ticket-mcp

<!-- ticket-index:entry id=58fe9f39-50c2-4e1c-8bdc-336ed5d6da6e slug=done/ticket-mcp digest=4ce4a63c5ddd -->
#### [58fe9f39] Plan: ticket-mcp write tools — update, close, and batch operations via MCP
- priority: `high`
- summary: The ticket-mcp server currently exposes only read-only tools:
- ref: `memory-api/.ticket/tickets/58fe9f39-50c2-4e1c-8bdc-336ed5d6da6e/ticket.toml`

<!-- ticket-index:entry id=84d19fab-9086-4eb2-9d1b-f6bbbae62ceb slug=done/ticket-mcp digest=eebcb606bf77 -->
#### [84d19fab] [ticket-mcp] Expose ticket move planning and execution over MCP
- priority: `medium`
- summary: Expose the move capability to MCP clients so agents can dry-run and apply safe workspace moves without shelling out to the CLI.
- ref: `memory-api/.ticket/tickets/84d19fab-9086-4eb2-9d1b-f6bbbae62ceb/ticket.toml`


### Component: ticket-query

<!-- ticket-index:entry id=f6aa9048-c300-4f64-bf20-157d439dd7ca slug=done/ticket-query digest=fbc267470674 -->
#### [f6aa9048] [spec][ticket-query] Specify expressive query and ordering contract
- priority: `high`
- summary: The current ticket query contract is not expressive enough for focused discovery.
- ref: `.ticket/tickets/f6aa9048-c300-4f64-bf20-157d439dd7ca/ticket.toml`


### Component: ticket-system

<!-- ticket-index:entry id=6848ffa2-4e31-4985-beff-cba01af9b7ca slug=done/ticket-system digest=9f570fb7ecc7 -->
#### [6848ffa2] [ticket-system] Add effort field for token-budget estimates
- priority: `medium`
- summary: Extend ticket ordering so `board`, `next`, `list`, and similar listing surfaces account for the new `effort` field.
- ref: `.ticket/tickets/6848ffa2-4e31-4985-beff-cba01af9b7ca/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=44d22e8f-bdc5-4268-b678-023dc0154c0f slug=done/ticket-viewer digest=7b15508ee349 -->
#### [44d22e8f] Arch: ticket-viewer dioxus-frontend crate scaffold with trunk serve
- priority: `critical`
- ref: `memory-viewers/.ticket/tickets/44d22e8f-bdc5-4268-b678-023dc0154c0f/ticket.toml`

<!-- ticket-index:entry id=b8ae615d-ea03-4d63-bcce-01ab0b0942b3 slug=done/ticket-viewer digest=77731e92dbda -->
#### [b8ae615d] Bug: WorkspacesResponse field mismatch items vs workspaces
- ref: `memory-viewers/.ticket/tickets/b8ae615d-ea03-4d63-bcce-01ab0b0942b3/ticket.toml`

<!-- ticket-index:entry id=a4d0f88f-8b04-48b4-afae-7d16566997ae slug=done/ticket-viewer digest=03072c5121a8 -->
#### [a4d0f88f] Bug: ticket-viewer tree SVG icons render at massive size
- ref: `memory-viewers/.ticket/tickets/a4d0f88f-8b04-48b4-afae-7d16566997ae/ticket.toml`

<!-- ticket-index:entry id=049480c4-c363-4a54-ab34-cb3025313781 slug=done/ticket-viewer digest=efe599ee3c8a -->
#### [049480c4] Composite: Ticket-Viewer Feature Bundle (sorting, 3D graph, themes)
- summary: Composite ticket tracking three feature tracks for ticket-viewer:
- ref: `memory-viewers/.ticket/tickets/049480c4-c363-4a54-ab34-cb3025313781/ticket.toml`

<!-- ticket-index:entry id=08e3f042-f690-4d0e-907a-b4ffb9508e50 slug=done/ticket-viewer digest=90aa8aa649fe -->
#### [08e3f042] Design: ticket-viewer UX wireframes + interaction spec
- summary: Define the baseline UX for tree/file/graph views with workspace switching and state styling.
- ref: `memory-viewers/.ticket/tickets/08e3f042-f690-4d0e-907a-b4ffb9508e50/ticket.toml`

<!-- ticket-index:entry id=b21604c1-cefe-4479-ae48-b56c9a985dd0 slug=done/ticket-viewer digest=ad56e3eac23b -->
#### [b21604c1] Feature: Batch operations — multi-select, queue, bulk apply, filter-based updates
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/b21604c1-cefe-4479-ae48-b56c9a985dd0/ticket.toml`

<!-- ticket-index:entry id=9d0c7931-3fab-4176-a209-d1b4dafc904c slug=done/ticket-viewer digest=62ec33429634 -->
#### [9d0c7931] Feature: Description editor — Markdown textarea with live preview
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/9d0c7931-3fab-4176-a209-d1b4dafc904c/ticket.toml`

<!-- ticket-index:entry id=19383fed-b739-4d10-b097-cf09a616348e slug=done/ticket-viewer digest=d6616dc6b72e -->
#### [19383fed] Feature: Edge management — add/remove dependencies from graph
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/19383fed-b739-4d10-b097-cf09a616348e/ticket.toml`

<!-- ticket-index:entry id=4a228c24-d466-4782-9160-c492f727007a slug=done/ticket-viewer digest=b1cd39a89d63 -->
#### [4a228c24] Feature: Full-text search UI with field predicates
- priority: `medium`
- summary: The ticket-viewer sidebar works well for browsing known tickets, but it is too slow when users only know part of a title, body phrase, or ticket id. The viewer needs an in-app search surface that can...
- ref: `memory-viewers/.ticket/tickets/4a228c24-d466-4782-9160-c492f727007a/ticket.toml`

<!-- ticket-index:entry id=12d3c38b-0172-49a3-9e42-ba2a5a9b8eb4 slug=done/ticket-viewer digest=c4c378cdd11f -->
#### [12d3c38b] Feature: GPU 3D dependency graph via WebGPU
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/12d3c38b-0172-49a3-9e42-ba2a5a9b8eb4/ticket.toml`

<!-- ticket-index:entry id=4f399974-0ad6-4da6-848a-954cccea4943 slug=done/ticket-viewer digest=333a2ccaca7d -->
#### [4f399974] Feature: Share themes and effects from log-viewer to ticket-viewer
- summary: Make the theme and effects system available in ticket-viewer so it can use themed presets, particle effects, CRT overlays, smoke, and other visual effects from the shared GPU pipeline.
- ref: `memory-viewers/.ticket/tickets/4f399974-0ad6-4da6-848a-954cccea4943/ticket.toml`

<!-- ticket-index:entry id=4143b314-357d-400b-b2b7-9bf588a98d90 slug=done/ticket-viewer digest=c67e01f8be3b -->
#### [4143b314] Feature: State transition UI — visual state machine with advance/undo
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/4143b314-357d-400b-b2b7-9bf588a98d90/ticket.toml`

<!-- ticket-index:entry id=3e069173-179f-4d4c-97bd-ecade44956f1 slug=done/ticket-viewer digest=7ddca8e7ba22 -->
#### [3e069173] Feature: Ticket creation form with type selection and required fields
- priority: `critical`
- ref: `memory-viewers/.ticket/tickets/3e069173-179f-4d4c-97bd-ecade44956f1/ticket.toml`

<!-- ticket-index:entry id=15ee34c6-60d9-487a-aba0-cffeb435c031 slug=done/ticket-viewer digest=d9a5daff8c9f -->
#### [15ee34c6] Feature: Ticket inline editing — title, priority, component, custom fields
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/15ee34c6-60d9-487a-aba0-cffeb435c031/ticket.toml`

<!-- ticket-index:entry id=6f71ca0b-b922-4f6b-9b09-bf0cbd9b228c slug=done/ticket-viewer digest=4910c9b0d864 -->
#### [6f71ca0b] Impl: GPU dependency graph in ticket-viewer replacing SVG GraphView
- summary: Build a GPU-rendered dependency graph in ticket-viewer using the shared `Graph3DView` component from viewer-api, replacing the current SVG-based `GraphView.tsx`.
- ref: `memory-viewers/.ticket/tickets/6f71ca0b-b922-4f6b-9b09-bf0cbd9b228c/ticket.toml`

<!-- ticket-index:entry id=2772fe5d-3f29-4116-82fe-bf611ea54c58 slug=done/ticket-viewer digest=55a399774eb0 -->
#### [2772fe5d] Impl: hypergraph dependency view reusing log-viewer graph patterns
- ref: `memory-viewers/.ticket/tickets/2772fe5d-3f29-4116-82fe-bf611ea54c58/ticket.toml`

<!-- ticket-index:entry id=b594864a-008c-423d-bf86-df940ed9dc54 slug=done/ticket-viewer digest=8c52c22a579a -->
#### [b594864a] Impl: state styling baseline and per-workspace UI state persistence
- ref: `memory-viewers/.ticket/tickets/b594864a-008c-423d-bf86-df940ed9dc54/ticket.toml`

<!-- ticket-index:entry id=02dea1fa-828e-4173-aed3-7a0e654e9d81 slug=done/ticket-viewer digest=45920fc1529e -->
#### [02dea1fa] Impl: ticket-viewer shell reusing doc-viewer tree and file display
- ref: `memory-viewers/.ticket/tickets/02dea1fa-828e-4173-aed3-7a0e654e9d81/ticket.toml`

<!-- ticket-index:entry id=8d861d64-4d3b-4c1c-82db-13fa1221cd04 slug=done/ticket-viewer digest=1dfa92bab2d2 -->
#### [8d861d64] Port: Dependency graph — DOM Element Graph via HypergraphView pipeline
- priority: `critical`
- ref: `memory-viewers/.ticket/tickets/8d861d64-4d3b-4c1c-82db-13fa1221cd04/ticket.toml`

<!-- ticket-index:entry id=8672684c-da0a-4e45-9b7d-0ce6c6d4182a slug=done/ticket-viewer digest=fc4ef3fba786 -->
#### [8672684c] Port: SSE integration for real-time ticket updates
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/8672684c-da0a-4e45-9b7d-0ce6c6d4182a/ticket.toml`

<!-- ticket-index:entry id=c2f04936-8871-4ee6-9b08-febe671fba2f slug=done/ticket-viewer digest=5396bb2a37ae -->
#### [c2f04936] Port: State persistence — localStorage per-workspace + URL routing
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/c2f04936-8871-4ee6-9b08-febe671fba2f/ticket.toml`

<!-- ticket-index:entry id=af19b0f6-b6b4-48fb-8b70-09a70f4868f8 slug=done/ticket-viewer digest=b985524aac15 -->
#### [af19b0f6] Port: TicketContent viewer — Markdown + TOML tabs
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/af19b0f6-b6b4-48fb-8b70-09a70f4868f8/ticket.toml`

<!-- ticket-index:entry id=3e79be12-cf02-4976-9d66-23cafb3276eb slug=done/ticket-viewer digest=8ebeddb9c878 -->
#### [3e79be12] Port: TicketTree with state grouping, search, filter, sort
- priority: `critical`
- ref: `memory-viewers/.ticket/tickets/3e79be12-cf02-4976-9d66-23cafb3276eb/ticket.toml`

<!-- ticket-index:entry id=80b4b77f-3fd6-4fab-98ab-028c6f6d6ef6 slug=done/ticket-viewer digest=f13636889506 -->
#### [80b4b77f] Port: WorkspacePicker with auth token management
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/80b4b77f-3fd6-4fab-98ab-028c6f6d6ef6/ticket.toml`

<!-- ticket-index:entry id=1f39ba8f-650b-417d-b664-1878f08af669 slug=done/ticket-viewer digest=bae8c9c36af2 -->
#### [1f39ba8f] [ticket-viewer] Add graph review E2E coverage
- priority: `high`
- summary: Add release Playwright coverage for the graph review checklist items around layout restoration and zoom-driven node detail interactions.
- ref: `memory-viewers/.ticket/tickets/1f39ba8f-650b-417d-b664-1878f08af669/ticket.toml`

<!-- ticket-index:entry id=a08a6153-126e-4e4a-8333-0e651817d8ea slug=done/ticket-viewer digest=eff93193c914 -->
#### [a08a6153] [ticket-viewer] Add workflow ordering and blocker-tree surfaces
- priority: `high`
- summary: Add workflow-focused ticket-viewer UI surfaces that consume the new workflow payloads. The viewer should keep generic field sorting for ad hoc browsing, but it should introduce workflow-specific next...
- ref: `.ticket/tickets/a08a6153-126e-4e4a-8333-0e651817d8ea/ticket.toml`

<!-- ticket-index:entry id=d7a27192-6c67-4446-9450-c946bf58747e slug=done/ticket-viewer digest=67d7f014c10c -->
#### [d7a27192] [ticket-viewer] Bootstrap the local ticket store on server startup
- priority: `high`
- summary: `ticket-viewer` currently resolves the local `.ticket` workspace and calls
- ref: `memory-viewers/.ticket/tickets/d7a27192-6c67-4446-9450-c946bf58747e/ticket.toml`

<!-- ticket-index:entry id=6ea2c97c-0b41-4b90-91db-f0de9e8e4b8e slug=done/ticket-viewer digest=f027ee2e6d25 -->
#### [6ea2c97c] [ticket-viewer] Bug: header actions cleanup for theme/info/home/filter
- priority: `high`
- summary: The ticket-viewer list route exposes a confusing header action set. The route wires `PageHeader.on_home` and `on_theme_toggle`, but the shared `HeaderActions` render the theme toggle as an `InfoIcon`...
- ref: `memory-viewers/.ticket/tickets/6ea2c97c-0b41-4b90-91db-f0de9e8e4b8e/ticket.toml`

<!-- ticket-index:entry id=c10cc92e-03b5-423b-a7ef-93879c253f7d slug=done/ticket-viewer digest=31bc86eaf52d -->
#### [c10cc92e] [ticket-viewer] Bug: sidebar tree parity with spec-viewer + long-list scrolling
- priority: `high`
- summary: The ticket-viewer list route still mounts a local `TicketTree` inside a shell that aggressively hides overflow, and in practice long sidebar trees are not reachable by scrolling. The result diverges ...
- ref: `memory-viewers/.ticket/tickets/c10cc92e-03b5-423b-a7ef-93879c253f7d/ticket.toml`

<!-- ticket-index:entry id=8f5d611f-0033-423e-b2f6-17683feb8e34 slug=done/ticket-viewer digest=8f9c50409c38 -->
#### [8f5d611f] [ticket-viewer] Build integrated ticket document panel
- priority: `high`
- summary: Replace the split metadata/content treatment with a single compact ticket document area in the main layout.
- ref: `memory-viewers/.ticket/tickets/8f5d611f-0033-423e-b2f6-17683feb8e34/ticket.toml`

<!-- ticket-index:entry id=a34d9891-5cd3-403c-8576-f3a55a50047e slug=done/ticket-viewer digest=b2a93cfeca5a -->
#### [a34d9891] [ticket-viewer] Consolidate detail panel into content panel (compact header, inline edit, metadata footer)
- priority: `high`
- summary: W1. The right details panel duplicates the floating content panel, but the content panel lacks the details panel's interactive buttons and edit capability. Remove the details panel and extend the con...
- ref: `memory-viewers/.ticket/tickets/a34d9891-5cd3-403c-8576-f3a55a50047e/ticket.toml`

<!-- ticket-index:entry id=f121b24b-61b0-41b4-9567-8ffc2417d7cb slug=done/ticket-viewer digest=39610c04178f -->
#### [f121b24b] [ticket-viewer] Feature: keyboard navigation in explorer + quick-search
- priority: `high`
- summary: Ticket selection in the current ticket-viewer is still predominantly mouse-driven.
- ref: `memory-viewers/.ticket/tickets/f121b24b-61b0-41b4-9567-8ffc2417d7cb/ticket.toml`

<!-- ticket-index:entry id=60092819-f725-48ec-93f0-aba195ef81eb slug=done/ticket-viewer digest=e634ddb06c9f -->
#### [60092819] [ticket-viewer] Fix graph layout defaults and isometric settings
- priority: `high`
- summary: Record graph layout/defaults implementation and move the ticket to review.
- ref: `memory-viewers/.ticket/tickets/60092819-f725-48ec-93f0-aba195ef81eb/ticket.toml`

<!-- ticket-index:entry id=b00b945b-045f-4124-9c69-ea15346b144f slug=done/ticket-viewer digest=8f214c52400c -->
#### [b00b945b] [ticket-viewer] Fix list-driven content panel selection sync
- priority: `high`
- summary: Opening a different ticket from the sidebar could leave the content panel showing the previous ticket body because the content component kept its state across list-driven selection changes.
- ref: `memory-api/.ticket/tickets/b00b945b-045f-4124-9c69-ea15346b144f/ticket.toml`

<!-- ticket-index:entry id=f2d19fdc-9d3a-4eb0-ad62-cc42ded75541 slug=done/ticket-viewer digest=1358227076e3 -->
#### [f2d19fdc] [ticket-viewer] Fix wasm32 build regression from mechanical unused cleanup (dep_graph/page.rs, store.rs)
- priority: `high`
- summary: Commit 4bb0961 ("chore(warnings): apply mechanical unused cleanup in dioxus viewers") was applied under native-target checking only and broke the wasm32 build of ticket-viewer-dioxus. It:
- ref: `memory-viewers/.ticket/tickets/f2d19fdc-9d3a-4eb0-ad62-cc42ded75541/ticket.toml`

<!-- ticket-index:entry id=4a9b49fd-58e0-404c-a120-47ef277dcf9f slug=done/ticket-viewer digest=e4a4074af620 -->
#### [4a9b49fd] [ticket-viewer] Keep filtered explorer state authoritative under live refresh
- priority: `high`
- summary: Keep the filtered explorer authoritative under overlapping requests, SSE updates, snapshot refreshes, and workspace switches, and lock the full redesign with focused tests.
- ref: `.ticket/tickets/4a9b49fd-58e0-404c-a120-47ef277dcf9f/ticket.toml`

<!-- ticket-index:entry id=80631f3c-4a75-491e-876a-8bf2a5e0ab4f slug=done/ticket-viewer digest=623a5b49385c -->
#### [80631f3c] [ticket-viewer] Move Playwright E2E ownership into memory-viewers
- priority: `medium`
- summary: Ticket-viewer browser coverage is split across two ownership boundaries.
- ref: `memory-viewers/.ticket/tickets/80631f3c-4a75-491e-876a-8bf2a5e0ab4f/ticket.toml`

<!-- ticket-index:entry id=8ea2d687-ca80-4631-9a9b-583bed22c4ca slug=done/ticket-viewer digest=b10ce170c651 -->
#### [8ea2d687] [ticket-viewer] Prevent offline click panics in the Dioxus frontend
- priority: `high`
- summary: When the ticket-viewer backend is offline, clicking anywhere in the ticket-viewer frontend can throw `Uncaught RuntimeError: unreachable` and leave the UI unresponsive.
- ref: `memory-viewers/.ticket/tickets/8ea2d687-ca80-4631-9a9b-583bed22c4ca/ticket.toml`

<!-- ticket-index:entry id=fcced2f3-c32c-4533-9743-56543f428222 slug=done/ticket-viewer digest=2cbb107e6636 -->
#### [fcced2f3] [ticket-viewer][ticket-http] Search syntax hints and pattern contract
- priority: `high`
- summary: The ticket-viewer currently exposes text search, but the intended contract is not visible enough to users:
- ref: `.ticket/tickets/fcced2f3-c32c-4533-9743-56543f428222/ticket.toml`

<!-- ticket-index:entry id=eeda4039-d82d-4573-9d79-0bc89e152a76 slug=done/ticket-viewer digest=abceac7206cc -->
#### [eeda4039] [ticket-viewer][viewer-api] Add kanban table graph layout mode
- priority: `high`
- summary: The current default graph layouts are tuned for planar hierarchy views, but they do not present ticket dependency trees in a way that matches ticket workflow review. Dense ticket sets with mixed stat...
- ref: `memory-api/.ticket/tickets/eeda4039-d82d-4573-9d79-0bc89e152a76/ticket.toml`

<!-- ticket-index:entry id=d1d38010-08b8-4a06-ad2b-0bbed453c941 slug=done/ticket-viewer digest=4c7aba9c1266 -->
#### [d1d38010] [ticket-viewer][viewer-api] Preserve graph layout and camera across same-graph refreshes
- priority: `high`
- summary: The shared Graph3D surface preserves dragged node positions only until the next same-graph re-render. Once the ticket-viewer reuses the same workspace graph payload or changes node focus, the shared ...
- ref: `memory-api/.ticket/tickets/d1d38010-08b8-4a06-ad2b-0bbed453c941/ticket.toml`

<!-- ticket-index:entry id=800f09ed-beb0-4a12-be93-1392e45eadb8 slug=done/ticket-viewer digest=b2385f22e3d5 -->
#### [800f09ed] [ticket-viewer][viewer-api] Tighten graph layout and enlarge rich nodes
- priority: `high`
- summary: Make the default ticket graph easier to read by tightening the layout, enlarging visible node tiers, and giving rich ticket nodes a more cubic high-LOD presentation.
- ref: `memory-viewers/.ticket/tickets/800f09ed-beb0-4a12-be93-1392e45eadb8/ticket.toml`

<!-- ticket-index:entry id=89fa0c25-a9ee-4f2d-a341-09fd9707946a slug=done/ticket-viewer digest=9c1ad62f81ea -->
#### [89fa0c25] ticket-viewer: render parts, frozen state, amendments, and typed refs
- priority: `medium`
- summary: Surface parts, refs, and frozen state in ticket-viewer so a human can read the full structured ticket, collapse or expand individual parts, see what is frozen, and follow typed references.
- ref: `memory-api/.ticket/tickets/89fa0c25-a9ee-4f2d-a341-09fd9707946a/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=0b231549-d029-4465-997e-0ba4d5e0529e slug=done/ticket-vscode digest=1e9f6f4a4ac6 -->
#### [0b231549] [spec][vscode] Write specification for ticket-vscode extension
- priority: `high`
- summary: Produce a complete design specification for the existing `ticket-vscode` VS Code extension (`tools/ticket-vscode/`). The spec must be detailed enough to serve as the architectural reference model whe...
- ref: `memory-api/.ticket/tickets/0b231549-d029-4465-997e-0ba4d5e0529e/ticket.toml`

<!-- ticket-index:entry id=362448d4-ccf1-4b9d-90f3-d4577da83a65 slug=done/ticket-vscode digest=b4ff99585d25 -->
#### [362448d4] [ticket-vscode] Add dual-host packaging, bundling, and extension test harnesses
- priority: `high`
- summary: Package the ported extension so it activates in both the desktop/remote Node host and the web extension host, and add the harnesses that validate both.
- ref: `memory-api/.ticket/tickets/362448d4-ccf1-4b9d-90f3-d4577da83a65/ticket.toml`

<!-- ticket-index:entry id=50307cce-5a93-4668-9481-a3af5985ea1b slug=done/ticket-vscode digest=5b1e0d93638e -->
#### [50307cce] [ticket-vscode] Cover no-.ticket server launch without implicit init
- priority: `high`
- summary: The VS Code extension starts the ticket server without `--index-root` when no `.ticket` workspace is detected. That extension-equivalent path must be covered so future changes cannot reintroduce impl...
- ref: `memory-api/.ticket/tickets/50307cce-5a93-4668-9481-a3af5985ea1b/ticket.toml`

<!-- ticket-index:entry id=011563c2-59e7-48f1-a61f-d8fdc80d2f6e slug=done/ticket-vscode digest=1ad587cc30c7 -->
#### [011563c2] [ticket-vscode] Extract portable Rust core for ticket/domain logic
- priority: `high`
- summary: Move deterministic, serializable logic out of the current TypeScript extension into a new Rust core that is compiled to WASM and driven by host-provided data.
- ref: `memory-api/.ticket/tickets/011563c2-59e7-48f1-a61f-d8fdc80d2f6e/ticket.toml`

<!-- ticket-index:entry id=93f7e422-1e41-4145-b8ba-0dcf7fc730ac slug=done/ticket-vscode digest=6daee40a6c88 -->
#### [93f7e422] [ticket-vscode] Freeze Rust/WASM architecture spec and feature matrix
- priority: `high`
- summary: Use the new planning spec `ticket-vscode/rust-wasm-port` as the canonical design surface for the migration.
- ref: `memory-api/.ticket/tickets/93f7e422-1e41-4145-b8ba-0dcf7fc730ac/ticket.toml`

<!-- ticket-index:entry id=694d74b4-028b-4602-8090-d6200d577d4a slug=done/ticket-vscode digest=4ba264e8c4e7 -->
#### [694d74b4] [ticket-vscode] Integrate Rust/WASM core into TS hosts and remove replaced legacy logic
- priority: `high`
- summary: Removed the remaining TS-only fallback paths from ticket-vscode host/provider code, switched ticket navigation to VS Code external-URI routing, and updated provider tests to inject a core mock so the...
- ref: `memory-api/.ticket/tickets/694d74b4-028b-4602-8090-d6200d577d4a/ticket.toml`

<!-- ticket-index:entry id=44abe1d4-5727-45f8-be3b-d1ca5bf4c1ae slug=done/ticket-vscode digest=bc5387a0573a -->
#### [44abe1d4] [ticket-vscode] Move ticket 694d74b4 into the memory-api workspace store
- priority: `medium`
- summary: Relocate ticket `694d74b4-028b-4602-8090-d6200d577d4a` ("[ticket-vscode] Integrate Rust/WASM core into TS hosts and remove replaced legacy logic") from the root store into the memory-api store so it ...
- ref: `memory-api/.ticket/tickets/44abe1d4-5727-45f8-be3b-d1ca5bf4c1ae/ticket.toml`

<!-- ticket-index:entry id=8735fa5d-0550-40f1-9ee8-7b83a44a7fd1 slug=done/ticket-vscode digest=20334d246d75 -->
#### [8735fa5d] [ticket-vscode] Prefer PATH ticket-viewer before debug fallback
- priority: `medium`
- summary: Auto-start in `tools/ticket-vscode` should prefer the `ticket-viewer` executable on `PATH` before falling back to a workspace-local `target/debug/ticket-viewer(.exe)` binary. This keeps the extension...
- ref: `.ticket/tickets/8735fa5d-0550-40f1-9ee8-7b83a44a7fd1/ticket.toml`

<!-- ticket-index:entry id=14047b99-41d6-4899-bec6-4a919bffcc2d slug=done/ticket-vscode digest=4e82d12a9693 -->
#### [14047b99] [ticket-vscode] Prove dual-host WASM activation
- priority: `high`
- summary: Build a narrow architecture spike that proves a Rust/WASM module can be loaded by both VS Code extension hosts used by this port.
- ref: `memory-api/.ticket/tickets/14047b99-41d6-4899-bec6-4a919bffcc2d/ticket.toml`

<!-- ticket-index:entry id=bfafde19-ddf7-47ef-966e-a1135be4efd6 slug=done/ticket-vscode digest=45f5d53b9a57 -->
#### [bfafde19] [ticket-vscode] Replace Node-bound behaviors with host capability adapters
- priority: `high`
- summary: Refactor the extension host layer so runtime-specific behavior is isolated behind explicit capabilities instead of being embedded throughout `extensionSupport.ts`, `extensionCommands.ts`, and `ticket...
- ref: `memory-api/.ticket/tickets/bfafde19-ddf7-47ef-966e-a1135be4efd6/ticket.toml`

<!-- ticket-index:entry id=6de424b0-68ec-43c7-9d70-eb8d17305ab3 slug=done/ticket-vscode digest=37fdaed24d32 -->
#### [6de424b0] [ticket-vscode] Validate Rust/WASM parity across desktop, web, and remote hosts
- priority: `high`
- summary: This ticket closes the track by validating the implemented port against the spec and the current user-visible workflows.
- ref: `memory-api/.ticket/tickets/6de424b0-68ec-43c7-9d70-eb8d17305ab3/ticket.toml`

<!-- ticket-index:entry id=4842a2bd-e94d-4066-801e-8883cbc18cab slug=done/ticket-vscode digest=f2a9050764d3 -->
#### [4842a2bd] ticket-vscode: Auto-detect .ticket workspace from open VS Code folders
- priority: `high`
- summary: The ticket-vscode extension currently resolves the ticket workspace through a hardcoded chain:
- ref: `memory-api/.ticket/tickets/4842a2bd-e94d-4066-801e-8883cbc18cab/ticket.toml`

<!-- ticket-index:entry id=dbca2bab-77bb-4f58-8460-3714f3d07004 slug=done/ticket-vscode digest=6bcf198b12b9 -->
#### [dbca2bab] ticket-vscode: Auto-start ticket-viewer server on extension activation
- priority: `medium`
- summary: Currently, users must manually click the ▶ button in the Tickets sidebar or run the "Start Ticket Viewer Server" command before the tree view can display tickets. If the server is not running, the tr...
- ref: `memory-api/.ticket/tickets/dbca2bab-77bb-4f58-8460-3714f3d07004/ticket.toml`

<!-- ticket-index:entry id=576c5f77-b261-42aa-a3f0-fd2f9597520e slug=done/ticket-vscode digest=882d3a5db166 -->
#### [576c5f77] ticket-vscode: Navigate to ticket URL in Simple Browser on click
- priority: `medium`
- summary: When a user clicks a ticket in the tree view, the current behavior opens the ticket-viewer root URL and copies the ticket ID to the clipboard. The user must then manually paste and search for the tic...
- ref: `memory-api/.ticket/tickets/576c5f77-b261-42aa-a3f0-fd2f9597520e/ticket.toml`

<!-- ticket-index:entry id=5b330dd5-2dcc-4460-b468-43ff4c35bfba slug=done/ticket-vscode digest=73016ade284d -->
#### [5b330dd5] ticket-vscode: Open clicked tickets in viewer and expose Copy ID in context menu
- priority: `medium`
- summary: The current `ticket-vscode` tree click behavior still routes through `ticket-viewer.openTicket`, which prefers opening `description.md` when the local ticket folder exists. That means clicking a tick...
- ref: `memory-api/.ticket/tickets/5b330dd5-2dcc-4460-b468-43ff4c35bfba/ticket.toml`

<!-- ticket-index:entry id=a7d6ba2d-ea15-498f-9195-6ee775ea69a4 slug=done/ticket-vscode digest=27a2596027a2 -->
#### [a7d6ba2d] ticket-vscode: Replace native tooltip with debounced webview panel beside sidebar
- priority: `high`
- summary: The current hover tooltip on ticket tree items appears too quickly (VS Code's default ~500ms) and is positioned by VS Code at the cursor, which can obscure the tree view. Users want a more relaxed ho...
- ref: `memory-api/.ticket/tickets/a7d6ba2d-ea15-498f-9195-6ee775ea69a4/ticket.toml`

<!-- ticket-index:entry id=207b70d9-de61-4de4-af80-69732ff5b892 slug=done/ticket-vscode digest=e0aabbba8047 -->
#### [207b70d9] ticket-vscode: Show ticket description in hover tooltip
- priority: `medium`
- summary: Currently, hovering over a ticket in the tree view shows only basic metadata: title, ID, state, and type. The ticket description (which contains context, acceptance criteria, and implementation detai...
- ref: `memory-api/.ticket/tickets/207b70d9-de61-4de4-af80-69732ff5b892/ticket.toml`


### Component: ticket-workflow

<!-- ticket-index:entry id=790df512-d8a9-42bd-b3d6-6e2b4d5eda9c slug=done/ticket-workflow digest=b8af2ffd0348 -->
#### [790df512] [spec][ticket-workflow] Specify scoped selector contract for board and next
- priority: `high`
- summary: `ticket board show`, `ticket next`, MCP `board_show` / `next_tickets`, and HTTP `/api/workflow/next` expose only narrow, inconsistent scoping knobs.
- ref: `.ticket/tickets/790df512-d8a9-42bd-b3d6-6e2b4d5eda9c/ticket.toml`


### Component: tooling

<!-- ticket-index:entry id=4ef88dbc-cb39-4724-9f2c-53ab09cf90c5 slug=done/tooling digest=1325d050c322 -->
#### [4ef88dbc] Add a git-worktree helper script for the agent isolation protocol
- priority: `high`
- summary: Problem: the git-worktree isolation protocol used by agents (bootstrap/rebase/merge/teardown) exists only as prose in .agents/instructions/commit/branch-worktree.instructions.md, .agents/agents/orche...
- ref: `.ticket/tickets/4ef88dbc-cb39-4724-9f2c-53ab09cf90c5/ticket.toml`

<!-- ticket-index:entry id=d30e13e1-3304-4128-9653-be7c47679f9f slug=done/tooling digest=45f4455dada8 -->
#### [d30e13e1] [install-tools] Install all viewer binaries
- priority: `high`
- summary: Update install-tools.sh so the root installer refreshes doc-viewer, log-viewer, spec-viewer, and ticket-viewer PATH binaries. Keep ticket-vscode PATH-first launch behavior unchanged; solve the stale-...
- ref: `.ticket/tickets/d30e13e1-3304-4128-9653-be7c47679f9f/ticket.toml`

<!-- ticket-index:entry id=f52cc8e5-9faf-4a41-9c5b-ad7c2a381dd9 slug=done/tooling digest=a3bc21e058fd -->
#### [f52cc8e5] [tooling] Define executable and hook registry schema with Markdown catalog generation
- priority: `high`
- summary: Design and implement the versioned registry schema, repository inventory, Markdown catalog generator, and freshness validation. The registry covers Cargo binaries, executable scripts, services, exten...
- ref: `.ticket/tickets/f52cc8e5-9faf-4a41-9c5b-ad7c2a381dd9/ticket.toml`


### Component: tools/viewer/log-viewer/frontend/dioxus

<!-- ticket-index:entry id=4c1167a0-9262-4850-bf47-2a6729eb6e76 slug=done/tools/viewer/log-viewer/frontend/dioxus digest=e209f2648b78 -->
#### [4c1167a0] [LOG-5d] Port log-viewer app shell, URL state, and per-file store to Dioxus
- priority: `p1`
- summary: The current log-viewer frontend is not just a three-pane browser. `tools/viewer/log-viewer/frontend/src/App.tsx` coordinates the header, advanced filter panel, sidebar file tree, tabbed center pane, ...
- ref: `.ticket/tickets/4c1167a0-9262-4850-bf47-2a6729eb6e76/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=122cd156-3b25-4c36-8eb9-fc658f1bbedf slug=done/unspecified digest=3fa8a3e892bd -->
#### [122cd156] Add CLI surface for repository QA tool
- summary: Add a command-line surface for the repository QA tool so it can be used outside MCP.
- ref: `.ticket/tickets/122cd156-3b25-4c36-8eb9-fc658f1bbedf/ticket.toml`

<!-- ticket-index:entry id=f2afb9ab-e15e-4943-a5d8-c0c9b5628eb0 slug=done/unspecified digest=a0c6c7983080 -->
#### [f2afb9ab] Add exclude-path config for repository QA tool
- summary: Add a repository-level configuration file for the repo QA tool so users can exclude paths from audits.
- ref: `.ticket/tickets/f2afb9ab-e15e-4943-a5d8-c0c9b5628eb0/ticket.toml`

<!-- ticket-index:entry id=6cc88405-62f2-4e36-b95e-a37f498175eb slug=done/unspecified digest=e7a293b36b27 -->
#### [6cc88405] Add literal task/category example to MCP workflow schema
- summary: Review of ticket 3c6da958-f494-408f-b7dd-cc43997b8ead found AC1 incomplete: `WorkflowAddNodeInput.category` explains the custom-label redirect in prose but does not show the literal copy-ready field ...
- ref: `.ticket/tickets/6cc88405-62f2-4e36-b95e-a37f498175eb/ticket.toml`

<!-- ticket-index:entry id=b2392ff7-6b7f-4b48-99b7-adf1142a3fc0 slug=done/unspecified digest=d1262c69a676 -->
#### [b2392ff7] Add repository QA MCP audit tool
- summary: Add a single-endpoint Rust tool for agents to audit repository quality and track findings in a synchronized local database.
- ref: `memory-api/.ticket/tickets/b2392ff7-6b7f-4b48-99b7-adf1142a3fc0/ticket.toml`

<!-- ticket-index:entry id=4c937720-4e35-4db6-bce7-608fdad5b6c5 slug=done/unspecified digest=f213adaa6c71 -->
#### [4c937720] Add reviews prompt for ranked in-review ticket review
- priority: `medium`
- summary: Create a reusable rule-generated prompt in `.agents/prompts/reviews.prompt.md` that reviews the highest-ranked `in-review` tickets using the ticket system.
- ref: `.ticket/tickets/4c937720-4e35-4db6-bce7-608fdad5b6c5/ticket.toml`

<!-- ticket-index:entry id=729b72ac-9688-48be-847d-c864ffa037be slug=done/unspecified digest=ac276888a216 -->
#### [729b72ac] Add session-workflow authoring instruction guidance file
- summary: No guidance file documents how to author session workflows, so agents rediscover the node-kind model and URN rules by trial and error (session `aedf210d`, turns 60→63). The Review Agent mode even ins...
- ref: `.ticket/tickets/729b72ac-9688-48be-847d-c864ffa037be/ticket.toml`

<!-- ticket-index:entry id=dcc6ad3f-6266-4940-96e1-2f26656cae57 slug=done/unspecified digest=f98c32730e0f -->
#### [dcc6ad3f] Advanced Voxel Tools: Fill, Smooth, Extrude, Clone
- priority: `high`
- summary: Beyond basic paint/carve, the world editor needs advanced manipulation tools: flood-fill enclosed regions, smooth surfaces (averaging neighbors), extrude faces outward, and clone/stamp regions.
- ref: `.ticket/tickets/dcc6ad3f-6266-4940-96e1-2f26656cae57/ticket.toml`

<!-- ticket-index:entry id=524bee64-3ba0-480b-80c7-4ab56cd45784 slug=done/unspecified digest=a33c02768b80 -->
#### [524bee64] Batch session workflow node/edge creation tools
- summary: Every workflow node and edge is created one call at a time. Building a densely-linked graph (e.g. 4 review criteria + edges) multiplies both round-trips and the cost of any single schema mistake. In ...
- ref: `.ticket/tickets/524bee64-3ba0-480b-80c7-4ab56cd45784/ticket.toml`

<!-- ticket-index:entry id=40349f3f-8d04-4bf6-9241-b79425c10a97 slug=done/unspecified digest=f0d90690b2a3 -->
#### [40349f3f] Copilot capture hook does not record worktree assignment, leaving every passively captured session unlinked
- summary: `sessions_for_ticket` (spec `e5f8a2c1`) answers "which sessions worked on this ticket" via three cumulative tiers strict ⊆ linked ⊆ mentioned, using ONLY structured signals (never transcript text). T...
- ref: `.ticket/tickets/40349f3f-8d04-4bf6-9241-b79425c10a97/ticket.toml`

<!-- ticket-index:entry id=86b0a60e-9e9b-41c5-ba1e-a0f372587dbe slug=done/unspecified digest=8a1b6e687c3f -->
#### [86b0a60e] Core Voxel Editor: Paint, Carve, and Ray-Octree Intersection
- priority: `high`
- summary: The minimum viable world editor: paint voxels onto surfaces, carve voxels away, and ray-cast to find which voxel the cursor is pointing at. Edits flow through VoxelWorld → double buffer → splat regen...
- ref: `.ticket/tickets/86b0a60e-9e9b-41c5-ba1e-a0f372587dbe/ticket.toml`

<!-- ticket-index:entry id=468cb1c1-7756-48f6-85b8-e696d9065c01 slug=done/unspecified digest=45ac360699cd -->
#### [468cb1c1] Document multi-transcript refinement workflow
- summary: Add a reusable transcription workflow for creating numbered raw/clean pairs, merging cleaned artifacts, and folding later cleaned transcripts into a maintained merged artifact.
- ref: `.ticket/tickets/468cb1c1-7756-48f6-85b8-e696d9065c01/ticket.toml`

<!-- ticket-index:entry id=b29d49db-f9d9-4e53-a9ab-60de8bd25f80 slug=done/unspecified digest=1947726fcda1 -->
#### [b29d49db] Double-Buffered SVO Upload: BACK-Buffer Write + Swap System
- priority: `high`
- summary: The VoxelWorld's dirty regions must be uploaded to the GPU without stalling the render loop. WASM writes to the BACK buffer while the GPU reads the FRONT buffer. After upload, swap makes new data ava...
- ref: `.ticket/tickets/b29d49db-f9d9-4e53-a9ab-60de8bd25f80/ticket.toml`

<!-- ticket-index:entry id=abbc1175-bb52-4871-af41-c6aaa2f04b19 slug=done/unspecified digest=548692e22c76 -->
#### [abbc1175] Editor UX: Undo/Redo, Symmetry, Live Preview, Material Picker
- priority: `high`
- summary: The voxel editor needs UX features for a productive editing workflow: undo/redo for all operations, symmetry modes for mirrored sculpting, live brush preview, and a material picker connected to the t...
- ref: `.ticket/tickets/abbc1175-bb52-4871-af41-c6aaa2f04b19/ticket.toml`

<!-- ticket-index:entry id=2df2a9e7-5755-43e3-b143-3b4d19c8a5e7 slug=done/unspecified digest=ddd977b5328c -->
#### [2df2a9e7] Expand board next-up and remove duplicate next board summary
- priority: `high`
- ref: `.ticket/tickets/2df2a9e7-5755-43e3-b143-3b4d19c8a5e7/ticket.toml`

<!-- ticket-index:entry id=d7141fac-8c1f-42c9-a392-7e400bb5f2ba slug=done/unspecified digest=d3f35c976c18 -->
#### [d7141fac] Expose ticket part read/write (list/get/write/amend/undo) over ticket-cli, ticket-mcp, ticket-http
- summary: Expose TicketStore::write_part, write_amendment_part, and undo_part (landed in 5a3d152c/3d952036/f9e70385) over ticket-cli, ticket-mcp, and ticket-http transports. Currently zero transport surface ex...
- ref: `memory-api/.ticket/tickets/d7141fac-8c1f-42c9-a392-7e400bb5f2ba/ticket.toml`

<!-- ticket-index:entry id=5162dae2-8222-49af-a2e0-f508541ef678 slug=done/unspecified digest=027f360d7413 -->
#### [5162dae2] Feature: Interaction Bridge: 2D Unprojection & Dioxus-to-WASM Pipeline
- priority: `high`
- summary: Clicks and hovers originating inside the generic Dioxus UI layer (`kernel-root`) must translate to 3D physical world events without stalling the main browser thread. The translation from 2D DOM coord...
- ref: `.ticket/tickets/5162dae2-8222-49af-a2e0-f508541ef678/ticket.toml`

<!-- ticket-index:entry id=4c58e8fb-73ad-4173-b5ee-3b87eafd3940 slug=done/unspecified digest=eac68fb3b137 -->
#### [4c58e8fb] Feature: SVO LOD Management & GPU Streaming
- priority: `high`
- summary: Rendering millions of voxels simultaneously via ray marching limits world size. We need a Level of Detail (LOD) architecture that leverages the SVO tree structure to dynamically group distant geometr...
- ref: `.ticket/tickets/4c58e8fb-73ad-4173-b5ee-3b87eafd3940/ticket.toml`

<!-- ticket-index:entry id=0c02b304-de04-4e29-818f-fb1e6797bdc0 slug=done/unspecified digest=2b62468bed8e -->
#### [0c02b304] Fix silent history-append failures and manifest null-deletion corruption
- priority: `high`
- ref: `memory-api/.ticket/tickets/0c02b304-de04-4e29-818f-fb1e6797bdc0/ticket.toml`

<!-- ticket-index:entry id=08b3c22c-eefc-48d4-b2ed-64a9a7b53c98 slug=done/unspecified digest=b1bf2b57075a -->
#### [08b3c22c] Fresh-clone bootstrap fails: memory-api submodule pinned to unreachable commit
- summary: First-time understanding of this repository: make `context-engine` fully
- ref: `.ticket/tickets/08b3c22c-eefc-48d4-b2ed-64a9a7b53c98/ticket.toml`

<!-- ticket-index:entry id=c6c150d9-eba5-4370-8283-c759bac302ef slug=done/unspecified digest=801c2abffab1 -->
#### [c6c150d9] GPU Buffer Infrastructure: Double-Buffered SVO, Splat Buffers, and Bind Groups
- priority: `high`
- summary: All GPU storage buffers and bind groups must be created before the render graph can execute. This ticket covers the buffer allocation layer — double-buffered SVO, voxel splatting buffers, bind group ...
- ref: `.ticket/tickets/c6c150d9-eba5-4370-8283-c759bac302ef/ticket.toml`

<!-- ticket-index:entry id=cf71418d-038b-4fc1-879d-0a302b681f84 slug=done/unspecified digest=c6f0027fd018 -->
#### [cf71418d] GPU Radix Sort: 8-Pass Parallel Sort for Voxel Splat Depth+Tile Ordering
- priority: `high`
- summary: After sort key construction (T6b), ~1M voxel splats must be sorted by composite key `(tile_id | depth)` for correct front-to-back compositing in the tiled rasterizer (T6d). This ticket implements an ...
- ref: `.ticket/tickets/cf71418d-038b-4fc1-879d-0a302b681f84/ticket.toml`

<!-- ticket-index:entry id=a87f450a-f703-4773-8467-44718d5ba70f slug=done/unspecified digest=78f23116b5fc -->
#### [a87f450a] Glass SDF Core: Analytical SDF Evaluation + Snell's Refraction in Tiled Rasterizer
- priority: `high`
- summary: UI panels must appear as physically realistic glass floating in 3D space. This ticket implements the core glass system: analytical SDF evaluation per-pixel, Snell's law refraction to bend splat looku...
- ref: `.ticket/tickets/a87f450a-f703-4773-8467-44718d5ba70f/ticket.toml`

<!-- ticket-index:entry id=5008909b-e6f2-40a4-897c-8cd359efc292 slug=done/unspecified digest=0b5731184a8b -->
#### [5008909b] Glass VFX: Chromatic Aberration, Pseudo-Caustics, and Frosted Mipmap Blur
- priority: `high`
- summary: Once basic glass refraction works (T3a), three visual effects add realism: chromatic aberration (spectral RGB split), pseudo-caustics (refraction divergence → brightness), and mipmap-based frosted bl...
- ref: `.ticket/tickets/5008909b-e6f2-40a4-897c-8cd359efc292/ticket.toml`

<!-- ticket-index:entry id=15c39147-4dac-4bdf-9b4f-b8b51e2a6c6e slug=done/unspecified digest=e1ceb102fdf3 -->
#### [15c39147] Global clickable-reference policy rendered across agent targets
- summary: Define one global, format-switchable clickable-reference policy and render it into the global agent contract.
- ref: `.ticket/tickets/15c39147-4dac-4bdf-9b4f-b8b51e2a6c6e/ticket.toml`

<!-- ticket-index:entry id=bce0d2fb-8ac3-4be3-af42-47f5b6928caa slug=done/unspecified digest=7dfd2c12ec11 -->
#### [bce0d2fb] Impl: Extract HypergraphViewCore to viewer-api as primary graph component
- ref: `viewer-api/.ticket/tickets/bce0d2fb-8ac3-4be3-af42-47f5b6928caa/ticket.toml`

<!-- ticket-index:entry id=367335ee-22bd-4cde-aa6a-312e80702c19 slug=done/unspecified digest=a9c73e500cf6 -->
#### [367335ee] Impl: URL hash routing for ticket-viewer (workspace + ticket ID)
- ref: `memory-viewers/.ticket/tickets/367335ee-22bd-4cde-aa6a-312e80702c19/ticket.toml`

<!-- ticket-index:entry id=f4f5da07-8889-42ed-b32d-8638e811be76 slug=done/unspecified digest=f83232b4f22c -->
#### [f4f5da07] Improve board immediate action wording
- summary: Adjust `ticket board show` immediate action text so the suggested ticket includes its current state immediately after `Start`, wraps the title in quotes, and escapes inner quotes correctly.
- ref: `.ticket/tickets/f4f5da07-8889-42ed-b32d-8638e811be76/ticket.toml`

<!-- ticket-index:entry id=1a7ae853-7588-409d-b172-b2d2bfbed3fb slug=done/unspecified digest=5b768b0a5ff3 -->
#### [1a7ae853] Make session workflow tools flexible and self-correcting for dynamic workflow authoring
- summary: Session workflow tools force agents through restrictive, single-item, closed-schema calls that produce avoidable failure cascades. In session `aedf210d-134c-4a8d-ab7c-2060f82f95d4` (turns 60→62→63), ...
- ref: `.ticket/tickets/1a7ae853-7588-409d-b172-b2d2bfbed3fb/ticket.toml`

<!-- ticket-index:entry id=445a2d76-5795-4d7a-aec8-d1536ec61416 slug=done/unspecified digest=cd1f338961bc -->
#### [445a2d76] Model price awareness: enforce orchestrator mode for expensive models
- summary: Implement "context stack price awareness" so that an agent automatically switches into an **orchestrator mode** based on the cost of its own underlying model. In orchestrator mode the agent delegates...
- ref: `.ticket/tickets/445a2d76-5795-4d7a-aec8-d1536ec61416/ticket.toml`

<!-- ticket-index:entry id=75b12c5a-a0cf-4810-b5b6-a65319dc95a7 slug=done/unspecified digest=73fa6f9a04d0 -->
#### [75b12c5a] Motion-Blurred Particle Splatting
- priority: `high`
- summary: Rendering hundreds of thousands of particles conventionally as point sprites creates hard edges that don't mix gracefully within the soft, liquid-glass/Voxel SDF aesthetic. Particles need volumetric ...
- ref: `.ticket/tickets/75b12c5a-a0cf-4810-b5b6-a65319dc95a7/ticket.toml`

<!-- ticket-index:entry id=b4f444ee-4858-4d13-8cdb-690a33115611 slug=done/unspecified digest=ca75492299e5 -->
#### [b4f444ee] Move context-stack to repo root and remove deprecated folders
- summary: Move the context-stack submodule from crates/context-stack to context-stack at the repository root, remove the deprecated humans/, agents/, scripts/, and tools/http/ directories, and update the works...
- ref: `.ticket/tickets/b4f444ee-4858-4d13-8cdb-690a33115611/ticket.toml`

<!-- ticket-index:entry id=6cc7dbbd-ccb2-4262-9a5b-40d9e8bb4b75 slug=done/unspecified digest=c35ef87afe7a -->
#### [6cc7dbbd] Multiplayer Characters: SDF Capsule Sync, Interpolation & Voxel Splat Rendering
- priority: `high`
- summary: Other players must appear in the local client's 3D world as physical entities that cast shadows, refract through Liquid Glass, and move smoothly despite network latency. We render remote players as S...
- ref: `.ticket/tickets/6cc7dbbd-ccb2-4262-9a5b-40d9e8bb4b75/ticket.toml`

<!-- ticket-index:entry id=286cb564-7406-44e7-911f-509a1ea5144d slug=done/unspecified digest=b7f1be3a016d -->
#### [286cb564] Multiplayer Networking: WebTransport, Spatial Subscriptions & Chunk Sync
- priority: `high`
- summary: The client must efficiently synchronize with SpacetimeDB in real time. In an open world with millions of voxels, the client cannot load everything — it needs spatial subscriptions that load/unload ch...
- ref: `.ticket/tickets/286cb564-7406-44e7-911f-509a1ea5144d/ticket.toml`

<!-- ticket-index:entry id=8d289d6c-941b-41e2-aaa3-58465bcba3d3 slug=done/unspecified digest=f0da55845639 -->
#### [8d289d6c] Panel Interaction: 3D Ray-Cast Hit Testing and Input Handling
- priority: `high`
- summary: World panels must respond to mouse clicks, hovers, and drags. This requires ray-casting from the camera through mouse position, intersecting with panel planes, and dispatching input events to the cor...
- ref: `.ticket/tickets/8d289d6c-941b-41e2-aaa3-58465bcba3d3/ticket.toml`

<!-- ticket-index:entry id=a5ab9013-94ce-4055-8d03-400236209958 slug=done/unspecified digest=ec802a94d80f -->
#### [a5ab9013] Rapier Collision Bridge: SVO → Chunk Colliders for Physics
- priority: `high`
- summary: bevy_rapier3d needs collision shapes derived from the SVO for character physics and rigid body dynamics. This ticket implements the bridge that converts dirty SVO chunks into Rapier colliders using g...
- ref: `.ticket/tickets/a5ab9013-94ce-4055-8d03-400236209958/ticket.toml`

<!-- ticket-index:entry id=dbb8fc83-a319-4f40-8124-149a24361068 slug=done/unspecified digest=64522b92b475 -->
#### [dbb8fc83] Register task and epic ticket workflow schemas
- summary: The ticket store can create `task` and `epic` records but cannot transition or update them because no workflow schema is registered for either type. Independent review of the session-workflow flexibi...
- ref: `.ticket/tickets/dbb8fc83-a319-4f40-8124-149a24361068/ticket.toml`

<!-- ticket-index:entry id=c7659e24-3687-4581-bc4c-54bfc7e19267 slug=done/unspecified digest=b1739a933d95 -->
#### [c7659e24] Render Graph + Pipeline: Custom Nodes, Canvas Setup, and Mipmap Generation
- priority: `high`
- summary: Bevy's render graph must host 7 custom nodes executing in sequence, plus canvas/WebGPU initialization and mipmap generation for frosted glass. This ticket wires the render graph — it does NOT impleme...
- ref: `.ticket/tickets/c7659e24-3687-4581-bc4c-54bfc7e19267/ticket.toml`

<!-- ticket-index:entry id=85012858-cbf3-40df-b55e-b82e89f72434 slug=done/unspecified digest=9d09f0ec2488 -->
#### [85012858] Research lifecycle engine design surfaces
- priority: `high`
- summary: Research the existing shared schema, ticket registry, spec registry, and rule-schema surfaces needed for Track 1.
- ref: `.ticket/tickets/85012858-cbf3-40df-b55e-b82e89f72434/ticket.toml`

<!-- ticket-index:entry id=e2f25c12-cc03-4d45-9af1-144902783883 slug=done/unspecified digest=15a088d4b242 -->
#### [e2f25c12] Retire the rule system: delete rule stores and rule-targets, freeze generated docs
- summary: Approved scope for retiring the rule system:
- ref: `.ticket/tickets/e2f25c12-cc03-4d45-9af1-144902783883/ticket.toml`

<!-- ticket-index:entry id=781dd9ad-74a6-41a1-9dac-4fda185780d7 slug=done/unspecified digest=ceb13c821b77 -->
#### [781dd9ad] Self-correcting session workflow rejection errors (embed alternative pattern)
- summary: When a session workflow mutation is rejected, the error lists allowed values but does not tell the agent what to do instead. Observed in session `aedf210d` (turns 60, 62): the `kind` rejection and th...
- ref: `.ticket/tickets/781dd9ad-74a6-41a1-9dac-4fda185780d7/ticket.toml`

<!-- ticket-index:entry id=d54f034c-b6ab-4c8d-bb81-a287d05834a1 slug=done/unspecified digest=c33be0af621c -->
#### [d54f034c] Simplify ticket state machine: drop in-refinement and in-validation, enforce e2e testing and explicit user review
- priority: `high`
- summary: The current `tracker-improvement` state machine has **8 states** with 6
- ref: `.ticket/tickets/d54f034c-b6ab-4c8d-bb81-a287d05834a1/ticket.toml`

<!-- ticket-index:entry id=5070c6b3-a37a-47fa-8dcf-69f805c1a2d2 slug=done/unspecified digest=f2a0d9db1b30 -->
#### [5070c6b3] Sort Key Construction & Tiled Depth Ordering for Voxel Splats
- priority: `high`
- summary: The second rendering stage: project each `VoxelSplat`'s bounding box to screen-space, compute tile membership, and construct composite sort keys `(tile_id << 12) | depth` for the GPU radix sort (T6c)...
- ref: `.ticket/tickets/5070c6b3-a37a-47fa-8dcf-69f805c1a2d2/ticket.toml`

<!-- ticket-index:entry id=dac1c75b-e738-4211-8869-30e9ee12f313 slug=done/unspecified digest=e51ab32552b9 -->
#### [dac1c75b] Strip ticket/spec transition authority from Review Agent
- summary: Enforce the D2 policy decision: **the Iteration Agent owns ALL ticket state transitions.** The Review Agent is now strictly verdict-only and must never call `close_ticket`, never pass `to_state` to `...
- ref: `.ticket/tickets/dac1c75b-e738-4211-8869-30e9ee12f313/ticket.toml`

<!-- ticket-index:entry id=3c6da958-f494-408f-b7dd-cc43997b8ead slug=done/unspecified digest=9427c597cb8c -->
#### [3c6da958] Surface `category` and add non-gating anchor URN for any workflow node kind
- summary: The workflow node model already carries a free-text `category` field that exists "so agents never hit an expressiveness wall for labels that do not drive behavior" (memory-api/crates/session-api/src/...
- ref: `.ticket/tickets/3c6da958-f494-408f-b7dd-cc43997b8ead/ticket.toml`

<!-- ticket-index:entry id=be9dd542-7e47-4898-9499-d8f268589058 slug=done/unspecified digest=99b29106fbf4 -->
#### [be9dd542] T1: Rename mcp-cost-gate crate/binary to mcp-toolmon (behavior-neutral)
- summary: The crate/binary name `mcp-cost-gate` no longer reflects its intended scope (general-purpose MCP proxy with pluggable policy + reload). Part of epic 25780944. This ticket is behavior-neutral and owns...
- ref: `.ticket/tickets/be9dd542-7e47-4898-9499-d8f268589058/ticket.toml`

<!-- ticket-index:entry id=9c3a4aff-276a-47a4-affb-6ebf06196625 slug=done/unspecified digest=58ef1e9701ea -->
#### [9c3a4aff] T2: Extract transport/proxy core from cost policy behind a Policy trait
- summary: `gate.rs` cost logic is hardwired into the proxy path in `proxy.rs`. To make mcp-toolmon a general-purpose proxy (reload/lifecycle features are policy-agnostic), the cost gate must become one pluggab...
- ref: `.ticket/tickets/9c3a4aff-276a-47a4-affb-6ebf06196625/ticket.toml`

<!-- ticket-index:entry id=3ee52eb7-a7f2-42dd-aad3-f09d80b8392a slug=done/unspecified digest=b6aa8ea4676a -->
#### [3ee52eb7] T3: Shadow-copy child execution + canonical-path resolution
- summary: `main.rs` spawns the child directly from its resolved PATH location, which holds a Windows file lock (`os error 5`) on `~/.cargo/bin/<server>.exe` for the process lifetime, blocking `cargo install --...
- ref: `.ticket/tickets/3ee52eb7-a7f2-42dd-aad3-f09d80b8392a/ticket.toml`

<!-- ticket-index:entry id=060fe66c-6f1d-4e57-8d46-d9a073899505 slug=done/unspecified digest=d7ac444e1b46 -->
#### [060fe66c] T4: Child lifecycle supervisor — swap, pending-request failure synthesis, never-exit fallback
- summary: There is currently no way to replace the running child process in place. A future watcher (T6) needs a supervisor that can kill/respawn the child, fail in-flight requests safely, and never let a bad ...
- ref: `.ticket/tickets/060fe66c-6f1d-4e57-8d46-d9a073899505/ticket.toml`

<!-- ticket-index:entry id=41da7827-1e45-445c-8583-1d00dc7ec5bd slug=done/unspecified digest=1c097628d0b0 -->
#### [41da7827] T5: Handshake replay cache — initialize + notifications/initialized, response suppression
- summary: When a child is respawned (T4), it needs a fresh MCP `initialize` handshake to function, but the client already completed its own handshake and must not see a second one. Part of epic 25780944; depen...
- ref: `.ticket/tickets/41da7827-1e45-445c-8583-1d00dc7ec5bd/ticket.toml`

<!-- ticket-index:entry id=7e242382-4445-48b6-9936-c8ae9bd5f2f3 slug=done/unspecified digest=a52bc3ffaf1d -->
#### [7e242382] T6: Binary watcher — debounced mtime/hash polling, triggers swap, tools/list_changed notification
- summary: Nothing currently detects that the canonical child binary P has been rebuilt. Part of epic 25780944; depends on T4 (swap target) and T5 (handshake replay must run on every triggered swap).
- ref: `.ticket/tickets/7e242382-4445-48b6-9936-c8ae9bd5f2f3/ticket.toml`

<!-- ticket-index:entry id=15c66e77-379b-4a1f-b2a2-4cf1cb808af9 slug=done/unspecified digest=78444d05f9b5 -->
#### [15c66e77] T7: Validation — transparent-reload integration test + Windows lock-freedom test
- summary: The whole feature's value is that a live reload is invisible to the client and the Windows lock problem is actually gone. Needs end-to-end proof, not just unit tests of individual parts. Part of epic...
- ref: `.ticket/tickets/15c66e77-379b-4a1f-b2a2-4cf1cb808af9/ticket.toml`

<!-- ticket-index:entry id=c7ed2ad2-8283-45cf-bda7-6b4c79c8eb45 slug=done/unspecified digest=73948a2982e3 -->
#### [c7ed2ad2] T8: Split policy API and cost gate into separate crates (thin transport core)
- summary: An audit found the mcp-toolmon `Policy` trait boundary is leaky:
- ref: `.ticket/tickets/c7ed2ad2-8283-45cf-bda7-6b4c79c8eb45/ticket.toml`

<!-- ticket-index:entry id=c5115990-529c-47ad-91bf-4dd1f8602d44 slug=done/unspecified digest=2544456ac2a5 -->
#### [c5115990] Voxel Inventory: Mini-SVO Items, Glass UI Rendering & Drag-to-World
- priority: `high`
- summary: Items in this RPG are physical voxel objects — each item is a small SVO (e.g., 8³) that can be inspected in 3D inside the inventory UI, rotated, and dragged into the world where it materializes as ac...
- ref: `.ticket/tickets/c5115990-529c-47ad-91bf-4dd1f8602d44/ticket.toml`

<!-- ticket-index:entry id=f0ac6e8b-4e12-4765-9a9a-6b3e107f6779 slug=done/unspecified digest=29ed96171e6e -->
#### [f0ac6e8b] Voxel Splat Kernel: Ray-Box SDF Splatting with Screen-Space EWA Filtering and LOD Blend
- priority: `high`
- summary: The first rendering stage: each occupied SVO leaf is projected to screen as a **voxel splat**. Instead of emitting 3D Gaussians with SH, each splat is evaluated analytically in screen-space using a *...
- ref: `.ticket/tickets/f0ac6e8b-4e12-4765-9a9a-6b3e107f6779/ticket.toml`

<!-- ticket-index:entry id=c241f246-2fc2-47dc-8742-684f5b23f08f slug=done/unspecified digest=4828671ee3fe -->
#### [c241f246] VoxelWorld API: Octree Data Structure, Manipulation, and Dirty-Range Tracking
- priority: `high`
- summary: All rendering, physics, and editing flows through the VoxelWorld resource — the Sparse Voxel Octree that stores world structure. This ticket implements the core data structure, manipulation API, and ...
- ref: `.ticket/tickets/c241f246-2fc2-47dc-8742-684f5b23f08f/ticket.toml`

<!-- ticket-index:entry id=c44e3bcf-18d1-4b8a-b2ee-e709c1c248c5 slug=done/unspecified digest=6f11d6944b41 -->
#### [c44e3bcf] World Generation: Procedural Noise SVO, Delta Persistence & Resource Regrowth
- priority: `high`
- summary: The open world needs an initial terrain generated procedurally from noise functions. SpacetimeDB stores only player-made modifications (deltas) against the deterministic base terrain. This means: sam...
- ref: `.ticket/tickets/c44e3bcf-18d1-4b8a-b2ee-e709c1c248c5/ticket.toml`

<!-- ticket-index:entry id=d02afc1f-7e0c-483f-a70d-86c7b1e088ad slug=done/unspecified digest=0cf1f95daa39 -->
#### [d02afc1f] WorldPanel Rendering: Glass SDF Panels with Content Textures in 3D Scene
- priority: `high`
- summary: In-world UI panels (floating labels, menus, information displays) must render as glass SDF shapes integrated into the tiled voxel splat rasterizer. Each panel has a content texture (rendered by Dioxu...
- ref: `.ticket/tickets/d02afc1f-7e0c-483f-a70d-86c7b1e088ad/ticket.toml`

<!-- ticket-index:entry id=34bc4938-fe4a-4ab1-94da-9d8d3697b268 slug=done/unspecified digest=069897010f2b -->
#### [34bc4938] [AOH][Design] Full system architecture — orchestrator, sessions, sandbox, messaging, PR
- priority: `high`
- summary: `COMPLETE` — All 15 ADRs are locked. All design blockers have been resolved:
- ref: `.ticket/tickets/34bc4938-fe4a-4ab1-94da-9d8d3697b268/ticket.toml`

<!-- ticket-index:entry id=d3f76335-15de-40e7-97ef-18c400e32268 slug=done/unspecified digest=ca986a5db7a2 -->
#### [d3f76335] [AOH][Design] Local-first git management — branch lifecycle without remote dependency
- priority: `high`
- summary: User decision (Q3):** GitHub is the remote, but PR management should be **local**. Only push to the remote when explicitly merging/sharing. No automatic remote pushes during agent implementation.
- ref: `.ticket/tickets/d3f76335-15de-40e7-97ef-18c400e32268/ticket.toml`

<!-- ticket-index:entry id=db784443-2e6c-4665-8e29-8e334ff74ffc slug=done/unspecified digest=d4ece621c0fe -->
#### [db784443] [AOH][Design] Operator authorization, secret lifecycle, and trust boundaries
- priority: `high`
- summary: Define the security and trust model for AOH before implementation starts.
- ref: `.ticket/tickets/db784443-2e6c-4665-8e29-8e334ff74ffc/ticket.toml`

<!-- ticket-index:entry id=d45826cd-18dd-446c-a4dc-cc94050ad780 slug=done/unspecified digest=094169162efa -->
#### [d45826cd] [AOH][Design] Reusable agent persona store — identity assignment and lifecycle
- priority: `medium`
- summary: User decision (Q8):** Unique generated personas per session, **reusable** — the same persona can be revived across multiple sessions. A persona is a persistent identity with a name, email, and a char...
- ref: `.ticket/tickets/d45826cd-18dd-446c-a4dc-cc94050ad780/ticket.toml`

<!-- ticket-index:entry id=ffa5361a-892f-4e9d-9aa7-f79ed8f97638 slug=done/unspecified digest=b3865cd05675 -->
#### [ffa5361a] [AOH][Design] Session archive, artifact retention, and revival schema
- priority: `high`
- summary: Turn ADR-9 into an implementation-ready contract.
- ref: `.ticket/tickets/ffa5361a-892f-4e9d-9aa7-f79ed8f97638/ticket.toml`

<!-- ticket-index:entry id=f345b954-7b4b-4d90-84cf-c6d7099dfa4f slug=done/unspecified digest=0e8a6b271b76 -->
#### [f345b954] [AOH][Interview] Requirements refinement — sandbox, messaging, git host, API, and scale
- priority: `high`
- summary: Q**: What execution environment should agent sessions run in?
- ref: `.ticket/tickets/f345b954-7b4b-4d90-84cf-c6d7099dfa4f/ticket.toml`

<!-- ticket-index:entry id=02412b9a-bccd-46f7-bded-0fbd7a067478 slug=done/unspecified digest=50e4228fa124 -->
#### [02412b9a] [AOH][Refinement] Reconcile AOH architecture with existing Phase 2 execution tickets
- priority: `high`
- summary: Normalize the AOH planning tree so there is **one canonical implementation decomposition**, not a second parallel tree beside the existing Phase 2 execution-layer tickets.
- ref: `.ticket/tickets/02412b9a-bccd-46f7-bded-0fbd7a067478/ticket.toml`

<!-- ticket-index:entry id=65d8e6c7-78ea-48ce-a6bd-8bc1eb712c4f slug=done/unspecified digest=f0bce9daf363 -->
#### [65d8e6c7] [AOH][Research] Cloud Hypervisor — Rust microVM with browser support and fast boot
- priority: `high`
- summary: Finding (2026-04-09):** Both `cloud-hypervisor` and Firecracker lack `virtio-gpu` (paravirtualised GPU) support. Without GPU access inside the guest, Chromium requires software rendering (SwiftShader...
- ref: `.ticket/tickets/65d8e6c7-78ea-48ce-a6bd-8bc1eb712c4f/ticket.toml`

<!-- ticket-index:entry id=49d6fe2e-e205-402a-84a6-7acb9c61e27c slug=done/unspecified digest=ea7e44023d49 -->
#### [49d6fe2e] [AOH][Research] Container BaaS — Podman/Docker, GPU passthrough, bollard, network namespaces
- priority: `high`
- summary: ADR-1 (revised 2026-04-09):** MicroVM approach (cloud-hypervisor/Firecracker) ruled out for browser workloads — no virtio-gpu support. Container-based Browser-as-a-Service (BaaS) is the adopted strat...
- ref: `.ticket/tickets/49d6fe2e-e205-402a-84a6-7acb9c61e27c/ticket.toml`

<!-- ticket-index:entry id=f3c6ed90-18a5-4bdb-8d76-9070ec24d3aa slug=done/unspecified digest=2276942bda16 -->
#### [f3c6ed90] [AOH][Research] GitHub API — PR lifecycle, branch management, code review
- priority: `medium`
- summary: | Decision | Resolution |
- ref: `.ticket/tickets/f3c6ed90-18a5-4bdb-8d76-9070ec24d3aa/ticket.toml`

<!-- ticket-index:entry id=89701593-7e97-470e-b836-ee28866515fd slug=done/unspecified digest=a125b4d81472 -->
#### [89701593] [AOH][Research] Messaging service APIs for async user interaction
- priority: `medium`
- summary: Updated 2026-04-09**: WhatsApp removed from candidates — requires paid Meta Business account. Active candidates: **Telegram** (primary), **Discord**, **Slack**.
- ref: `.ticket/tickets/89701593-7e97-470e-b836-ee28866515fd/ticket.toml`

<!-- ticket-index:entry id=09b68366-486e-4e39-a610-1d14676368aa slug=done/unspecified digest=2379bd0d9d0a -->
#### [09b68366] [AOH][Research] Multi-agent coordination and cross-agent communication protocols
- priority: `medium`
- summary: | Decision | Resolution |
- ref: `.ticket/tickets/09b68366-486e-4e39-a610-1d14676368aa/ticket.toml`

<!-- ticket-index:entry id=1b681754-84bf-4d4c-934a-e31c67eb91f4 slug=done/unspecified digest=84f1a318bfdc -->
#### [1b681754] [AOH][Research] Open-source agentic coding frameworks survey
- priority: `high`
- summary: Survey the open-source landscape for agentic coding frameworks that could be reused, adapted, or serve as reference implementations for the AOH project. Focus on: session lifecycle management, sandbo...
- ref: `.ticket/tickets/1b681754-84bf-4d4c-934a-e31c67eb91f4/ticket.toml`

<!-- ticket-index:entry id=7cf1044a-ae6a-4d08-a96f-436a5d4d9863 slug=done/unspecified digest=2c48c7fe6fca -->
#### [7cf1044a] [AOH][Research] Sandbox isolation technologies for agent code execution
- priority: `medium`
- summary: > **ADR-1 v1 Selection: Tier 2 — Container (Docker / Podman via `bollard`)**
- ref: `.ticket/tickets/7cf1044a-ae6a-4d08-a96f-436a5d4d9863/ticket.toml`

<!-- ticket-index:entry id=cba080b5-3c38-495d-8b67-d690b52de4d6 slug=done/unspecified digest=9e389abb4dde -->
#### [cba080b5] [AOH][Research] VS Code + GitHub Copilot agent API and MCP integration
- priority: `high`
- summary: Determine what programmatic control surface exists for creating and supervising GitHub Copilot agent sessions from a Rust orchestrator, and how MCP tools can be shared across multiple agent sessions ...
- ref: `.ticket/tickets/cba080b5-3c38-495d-8b67-d690b52de4d6/ticket.toml`

<!-- ticket-index:entry id=854f0e8f-c881-48a5-a8bc-a6f7ac3092a9 slug=done/unspecified digest=9f42cb4d9164 -->
#### [854f0e8f] [Board] Draftboard — workspace WIP coordination for concurrent agents
- priority: `high`
- summary: Provide a workspace-global, short-term "daily planning board" that tracks the current state of all active work across concurrent agent sessions. The draftboard fills the gap between ephemeral user pr...
- ref: `memory-api/.ticket/tickets/854f0e8f-c881-48a5-a8bc-a6f7ac3092a9/ticket.toml`

<!-- ticket-index:entry id=74160bb8-ac9c-4fd6-82e4-2e392d96e48b slug=done/unspecified digest=48ba89c540bd -->
#### [74160bb8] [Board] Integrate draftboard state into next and status commands
- priority: `medium`
- summary: Make the existing `ticket next` and `ticket status` commands draftboard-aware so that agents receive board context automatically, without needing to call `board show` separately. This is the integrat...
- ref: `memory-api/.ticket/tickets/74160bb8-ac9c-4fd6-82e4-2e392d96e48b/ticket.toml`

<!-- ticket-index:entry id=b72b0a40-496e-43d0-a5b3-ec358d85802b slug=done/unspecified digest=f5feb8b9cd8b -->
#### [b72b0a40] [Board] ticket-api: Cleanup, file ops, reconciliation, claim deprecation
- priority: `high`
- summary: Add the operational maintenance layer to the draftboard in `crates/ticket-api/`. This builds on the core board storage (types, tables, check-in/out/heartbeat/show/configure) established by `0db86ac1`...
- ref: `memory-api/.ticket/tickets/b72b0a40-496e-43d0-a5b3-ec358d85802b/ticket.toml`

<!-- ticket-index:entry id=0db86ac1-45ca-49a6-abc7-dd30b5adbee7 slug=done/unspecified digest=70ed41bc6828 -->
#### [0db86ac1] [Board] ticket-api: Core board storage — types, tables, CRUD
- priority: `high`
- summary: Implement the foundational draftboard data layer in `crates/ticket-api/`. This ticket covers core types, redb tables, and the primary board operations (check-in, check-out, heartbeat, show, configure...
- ref: `memory-api/.ticket/tickets/0db86ac1-45ca-49a6-abc7-dd30b5adbee7/ticket.toml`

<!-- ticket-index:entry id=bcc111c6-5034-4259-b8cd-3a4dacf3113a slug=done/unspecified digest=d25b0808a112 -->
#### [bcc111c6] [Board] ticket-cli: board subcommand family (show, check-in, check-out, heartbeat, clean)
- priority: `medium`
- summary: Expose all draftboard operations as `ticket board <subcommand>` in the CLI. This is the primary human and agent interface for draftboard coordination. All subcommands follow the existing CLI conventi...
- ref: `memory-api/.ticket/tickets/bcc111c6-5034-4259-b8cd-3a4dacf3113a/ticket.toml`

<!-- ticket-index:entry id=ec52f7cb-7c5e-4854-84d3-80618167762d slug=done/unspecified digest=fc19850aa8f4 -->
#### [ec52f7cb] [Board] ticket-mcp: Board tool endpoints for agent coordination
- priority: `medium`
- summary: Expose the draftboard as MCP tools so that agent sessions can coordinate through the MCP protocol without shelling out to the CLI. This is the primary machine interface for agent-to-board interaction...
- ref: `memory-api/.ticket/tickets/ec52f7cb-7c5e-4854-84d3-80618167762d/ticket.toml`

<!-- ticket-index:entry id=8aff39cb-2480-4610-9593-2e4e6d96d265 slug=done/unspecified digest=8f070dd4fdc8 -->
#### [8aff39cb] [Board][Design] Draftboard data model, API contract, and CLI/MCP surface
- priority: `high`
- summary: Produce the implementation-ready contract for the draftboard system: data model, store API, CLI subcommand surface, and MCP tool definitions. This design must be approved before any implementation be...
- ref: `memory-api/.ticket/tickets/8aff39cb-2480-4610-9593-2e4e6d96d265/ticket.toml`

<!-- ticket-index:entry id=84ceb9ce-ce68-4473-ac11-9724a20283ce slug=done/unspecified digest=600ac588edca -->
#### [84ceb9ce] [Board][Design] Entry identity, resume flow, and synchronization invariants
- priority: `high`
- summary: Close the remaining correctness gaps around what a draftboard entry actually represents, how an agent resumes existing work, and how board state stays synchronized with leases and ticket state transi...
- ref: `memory-api/.ticket/tickets/84ceb9ce-ce68-4473-ac11-9724a20283ce/ticket.toml`

<!-- ticket-index:entry id=c3143e3c-2d16-447a-9062-14305a31b786 slug=done/unspecified digest=c653114c4955 -->
#### [c3143e3c] [Board][Design] Stale-entry review, cleanup approval, and conflict resolution workflow
- priority: `high`
- summary: Define the human-in-the-loop workflow for stale entries, explicit cleanup, and file ownership conflicts.
- ref: `memory-api/.ticket/tickets/c3143e3c-2d16-447a-9062-14305a31b786/ticket.toml`

<!-- ticket-index:entry id=4c29acf5-df06-44b5-9f1a-890d574b7e75 slug=done/unspecified digest=29e36217135f -->
#### [4c29acf5] [Board][Docs] Add board workflow guidance to .github agent files
- priority: `medium`
- summary: The Draftboard feature (epic 854f0e8f) is now fully implemented across all
- ref: `memory-api/.ticket/tickets/4c29acf5-df06-44b5-9f1a-890d574b7e75/ticket.toml`

<!-- ticket-index:entry id=be38e809-781f-498c-915e-afaca1d1d3e0 slug=done/unspecified digest=2b1f63960cf5 -->
#### [be38e809] [Board][Validation] Concurrent check-in, crash recovery, and cross-interface consistency
- priority: `medium`
- summary: Validate that the draftboard behaves correctly under the failure modes and concurrency patterns it is explicitly meant to manage.
- ref: `memory-api/.ticket/tickets/be38e809-781f-498c-915e-afaca1d1d3e0/ticket.toml`

<!-- ticket-index:entry id=60a2a388-c8b6-4e25-a80a-0ba686f11bf9 slug=done/unspecified digest=4a8e467d8f4a -->
#### [60a2a388] [LOG-1b] doc-viewer + spec-viewer: wire init_tracing_full with file logging
- summary: `doc-viewer` and `spec-viewer` (if they have `main.rs` entry points) currently use `init_tracing()` (console-only) or have no explicit tracing setup. Logs are lost in detached mode.
- ref: `memory-viewers/.ticket/tickets/60a2a388-c8b6-4e25-a80a-0ba686f11bf9/ticket.toml`

<!-- ticket-index:entry id=12197242-b7b4-4212-83a8-4b0b65a4bd7b slug=done/unspecified digest=b03615113fd0 -->
#### [12197242] [LOG-2a] Audit and normalise context-* tracing field names for log-viewer compatibility
- summary: The `crates/context-{insert,read,search,trace}` crates emit `tracing` spans and events but field names, targets, and event shapes are not uniform. The log-viewer parser (`crates/context-api/src/log_p...
- ref: `memory-api/.ticket/tickets/12197242-b7b4-4212-83a8-4b0b65a4bd7b/ticket.toml`

<!-- ticket-index:entry id=c179ef57-6866-451d-ba7f-f7923ad1374b slug=done/unspecified digest=707c689fe677 -->
#### [c179ef57] [LOG-5a] Scaffold log-viewer-dioxus crate with trunk build and API client
- summary: The current log-viewer frontend is a Preact/Vite application. Per the Dioxus Viewer Platform epic (`35a6d14b`), all viewer frontends should be ported to Rust/Dioxus 0.7 compiled to WASM via `trunk`. ...
- ref: `.ticket/tickets/c179ef57-6866-451d-ba7f-f7923ad1374b/ticket.toml`

<!-- ticket-index:entry id=fe5232d9-537a-4217-b8c0-b8e3ca81d95b slug=done/unspecified digest=a67eaff97975 -->
#### [fe5232d9] [agent-rules] Prefer MCP Playwright tools in browser frontend testing guidance
- priority: `medium`
- summary: Browser-hosted frontend guidance requires Playwright coverage, but it does not explicitly tell agents to try the MCP Playwright/browser tools first before falling back to repo-local wrappers or manua...
- ref: `.ticket/tickets/fe5232d9-537a-4217-b8c0-b8e3ca81d95b/ticket.toml`

<!-- ticket-index:entry id=7c74f2fe-2bfd-477c-847e-bc02200a4819 slug=done/unspecified digest=863b2394dfe8 -->
#### [7c74f2fe] [agents] Add dedicated context-enrichment.agent.md template for enrich-review-close workflow
- summary: Author a new dedicated agent template, `.agents/agents/context-enrichment.agent.md`, whose sole job is to enrich context for an in-review ticket by reconstructing its history via `sessions_for_ticket...
- ref: `.ticket/tickets/7c74f2fe-2bfd-477c-847e-bc02200a4819/ticket.toml`

<!-- ticket-index:entry id=6bd67a7a-2a76-4dd7-a897-b4d325476621 slug=done/unspecified digest=a6fa80239691 -->
#### [6bd67a7a] [architecture][workspace] Dynamic multi-store discovery and cross-store references
- priority: `high`
- summary: Implement recursive multi-store workspace discovery and cross-store reference integration with URN-based identities across local and nested workspaces.
- ref: `.ticket/tickets/6bd67a7a-2a76-4dd7-a897-b4d325476621/ticket.toml`

<!-- ticket-index:entry id=7e318b2a-a381-49a1-aee9-18758a4b80fd slug=done/unspecified digest=9208cbcaa38a -->
#### [7e318b2a] [architecture][workspace] Late store onboarding reconciliation
- priority: `high`
- summary: Support absent-then-present store integration and late onboarding reconciliation.
- ref: `.ticket/tickets/7e318b2a-a381-49a1-aee9-18758a4b80fd/ticket.toml`

<!-- ticket-index:entry id=fa3e0a51-0caa-4a33-bfe2-1b173feaa979 slug=done/unspecified digest=48d76a77e0ec -->
#### [fa3e0a51] [architecture][workspace] Recursive automatic store discovery
- priority: `high`
- summary: Implement fully automatic recursive store discovery across local and nested workspaces.
- ref: `.ticket/tickets/fa3e0a51-0caa-4a33-bfe2-1b173feaa979/ticket.toml`

<!-- ticket-index:entry id=82d6ada4-ac35-45a7-9df6-7b7501d58e70 slug=done/unspecified digest=fab4165eab77 -->
#### [82d6ada4] [architecture][workspace] URN cross-store reference model and resolver
- priority: `high`
- summary: Implement URN-based cross-store reference model and resolver APIs.
- ref: `.ticket/tickets/82d6ada4-ac35-45a7-9df6-7b7501d58e70/ticket.toml`

<!-- ticket-index:entry id=c96f325f-5b45-4f2d-aed6-85648106d3ea slug=done/unspecified digest=ee45fa79e00a -->
#### [c96f325f] [audit instructions] Focus audit guidance on executing target-context audits and canonical findings summaries
- summary: Refocus the audit agent guidance on executing audit tools against the requested target context, surfacing findings, and summarizing results in a canonical findings-and-recommendations format.
- ref: `.ticket/tickets/c96f325f-5b45-4f2d-aed6-85648106d3ea/ticket.toml`

<!-- ticket-index:entry id=635b7e37-8bed-4622-a38d-ef87bb08f46c slug=done/unspecified digest=4ee06b7b3049 -->
#### [635b7e37] [audit-api] Derive spec fulfillment rollups from store-owned evidence
- summary: Teach `audit-api` to report derived spec fulfillment status by reading store-owned expectation and evidence metadata.
- ref: `.ticket/tickets/635b7e37-8bed-4622-a38d-ef87bb08f46c/ticket.toml`

<!-- ticket-index:entry id=2663a981-d279-45dc-abc0-42270491dca6 slug=done/unspecified digest=76287e764cd4 -->
#### [2663a981] [audit-cli] Add unified session_audit interface for latest or explicit session
- summary: Expose session audit through audit-cli unified audit interface.
- ref: `.ticket/tickets/2663a981-d279-45dc-abc0-42270491dca6/ticket.toml`

<!-- ticket-index:entry id=c991d769-27b4-4567-b9d1-95173af02c5a slug=done/unspecified digest=e104eed1fb8a -->
#### [c991d769] [audit-roadmap][file_length][batch-1] memory-api (88)
- summary: Resolve the current file_length batch for memory-api and reduce findings from baseline using strict largest-first splits.
- ref: `.ticket/tickets/c991d769-27b4-4567-b9d1-95173af02c5a/ticket.toml`

<!-- ticket-index:entry id=e92dd945-9607-4001-88d7-634a8ab28b5c slug=done/unspecified digest=6f13d892c7d9 -->
#### [e92dd945] [audit-roadmap][file_length][batch-1a] memory-api core crates (7)
- priority: `high`
- summary: Split remaining **core crate** file_length offenders with behavior-preserving module extractions and focused validation.
- ref: `.ticket/tickets/e92dd945-9607-4001-88d7-634a8ab28b5c/ticket.toml`

<!-- ticket-index:entry id=3e11a5fa-498b-47ae-80aa-1c7cb9a79be4 slug=done/unspecified digest=2525e6597270 -->
#### [3e11a5fa] [audit-roadmap][file_length][batch-1b] memory-api rules+fixtures (3)
- priority: `high`
- summary: Split remaining **rule + fixture** file_length offenders with narrow helper/test-module extraction.
- ref: `.ticket/tickets/3e11a5fa-498b-47ae-80aa-1c7cb9a79be4/ticket.toml`

<!-- ticket-index:entry id=22d67e25-9766-4cab-aed8-638d6fce222c slug=done/unspecified digest=f278ba74eb17 -->
#### [22d67e25] [audit-roadmap][file_length][batch-1c] memory-api ticket surfaces (5)
- priority: `high`
- summary: Split remaining **ticket surfaces (HTTP/CLI/MCP)** file_length offenders while preserving external behavior.
- ref: `.ticket/tickets/22d67e25-9766-4cab-aed8-638d6fce222c/ticket.toml`

<!-- ticket-index:entry id=5c7296f6-533f-47d9-8fe8-ffd4c80d8ca8 slug=done/unspecified digest=e37bc6d27892 -->
#### [5c7296f6] [audit-roadmap][file_length][batch-1d] memory-api tools+matrix tail (3)
- priority: `medium`
- summary: Split remaining **cross-cutting tooling/matrix** file_length offenders to close out the tail of batch-1.
- ref: `.ticket/tickets/5c7296f6-533f-47d9-8fe8-ffd4c80d8ca8/ticket.toml`

<!-- ticket-index:entry id=ac990dfe-083d-413c-ba4a-7cfbbda677b0 slug=done/unspecified digest=3e5e0ab5fe8e -->
#### [ac990dfe] [audit-roadmap][stability] Clear compiler/test/coverage findings
- summary: Clear the audit-roadmap stability category (compiler_warning, test_execution, coverage) for this roadmap slice.
- ref: `.ticket/tickets/ac990dfe-083d-413c-ba4a-7cfbbda677b0/ticket.toml`

<!-- ticket-index:entry id=9347c9f8-f3f0-49e7-8ca1-df77a0cca499 slug=done/unspecified digest=205fcae20924 -->
#### [9347c9f8] [audit-roadmap][stability][batch-1] compiler_warning (1)
- summary: Resolve the stability compiler_warning finding. This is an aggregate metric finding representing ALL compiler warnings, not a single warning.
- ref: `.ticket/tickets/9347c9f8-f3f0-49e7-8ca1-df77a0cca499/ticket.toml`

<!-- ticket-index:entry id=f2d8f807-447e-41a2-80db-2fca03d5b9ee slug=done/unspecified digest=8dc2a6c202f2 -->
#### [f2d8f807] [audit-roadmap][stability][batch-2] test_execution (1)
- summary: Resolve the current stability batch for test_execution and reduce 1 findings from the baseline.
- ref: `.ticket/tickets/f2d8f807-447e-41a2-80db-2fca03d5b9ee/ticket.toml`

<!-- ticket-index:entry id=1ff5c55a-d279-45a6-9451-7dbfb191c0e7 slug=done/unspecified digest=fc6c599d2c2b -->
#### [1ff5c55a] [audit-roadmap][stability][batch-3] coverage (1)
- summary: Resolve the current stability batch for coverage and reduce 1 findings from the baseline.
- ref: `.ticket/tickets/1ff5c55a-d279-45a6-9451-7dbfb191c0e7/ticket.toml`

<!-- ticket-index:entry id=40edd5d1-a02f-4c47-b791-1a5212641085 slug=done/unspecified digest=99c200abdaaa -->
#### [40edd5d1] [audit-roadmap][stability][dead_code] context-stack dead_code triage
- summary: Triage `dead_code` compiler warnings in the `context-stack` submodule (split off from parent `9347c9f8` mechanical pass).
- ref: `.ticket/tickets/40edd5d1-a02f-4c47-b791-1a5212641085/ticket.toml`

<!-- ticket-index:entry id=ef9eb7fb-1140-402b-983e-0ab5ba2bfaff slug=done/unspecified digest=cac964e0b56e -->
#### [ef9eb7fb] [audit-roadmap][stability][dead_code] log-viewer dead_code triage
- summary: Triage `dead_code` compiler warnings in the `log-viewer` frontend (`tools/viewer/log-viewer`, root repo — split off from parent `9347c9f8` mechanical pass).
- ref: `.ticket/tickets/ef9eb7fb-1140-402b-983e-0ab5ba2bfaff/ticket.toml`

<!-- ticket-index:entry id=cde503fd-3042-4ab3-8cff-dcb605e09af8 slug=done/unspecified digest=204e753a2ce2 -->
#### [cde503fd] [audit-roadmap][stability][dead_code] memory-viewers dead_code triage
- summary: Triage `dead_code` compiler warnings in the `memory-viewers` submodule (split off from parent `9347c9f8` mechanical pass).
- ref: `.ticket/tickets/cde503fd-3042-4ab3-8cff-dcb605e09af8/ticket.toml`

<!-- ticket-index:entry id=9c329f10-b2b0-412c-ab5a-14a52bddec76 slug=done/unspecified digest=dc43dc02bdeb -->
#### [9c329f10] [audit-roadmap][stability][dead_code] viewer-api dead_code triage
- summary: Triage `dead_code` compiler warnings in the `viewer-api` submodule (split off from parent `9347c9f8` mechanical pass). Largest concentration.
- ref: `.ticket/tickets/9c329f10-b2b0-412c-ab5a-14a52bddec76/ticket.toml`

<!-- ticket-index:entry id=15cf86fd-66ef-483e-b934-1f7c72352f67 slug=done/unspecified digest=3f5d0a31b66c -->
#### [15cf86fd] [audit-roadmap][static_complexity][batch-1 follow-up] context-cli output cluster (2)
- summary: Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.
- ref: `.ticket/tickets/15cf86fd-66ef-483e-b934-1f7c72352f67/ticket.toml`

<!-- ticket-index:entry id=7cfc8996-3dbc-49c5-b359-be493c094a4a slug=done/unspecified digest=a2ec8369bcbe -->
#### [7cfc8996] [audit-roadmap][static_complexity][batch-1 follow-up] context-cli repl cluster (2)
- summary: Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.
- ref: `.ticket/tickets/7cfc8996-3dbc-49c5-b359-be493c094a4a/ticket.toml`

<!-- ticket-index:entry id=409a3919-296e-4a62-bd1a-77a8212bf43f slug=done/unspecified digest=27d90b79e731 -->
#### [409a3919] [audit-roadmap][static_complexity][batch-1 follow-up] context-insert vertex cluster (4)
- summary: Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.
- ref: `.ticket/tickets/409a3919-296e-4a62-bd1a-77a8212bf43f/ticket.toml`

<!-- ticket-index:entry id=ebf4b601-0f9e-471c-9283-1390cda1eb41 slug=done/unspecified digest=0bcc67969012 -->
#### [ebf4b601] [audit-roadmap][static_complexity][batch-1 follow-up] context-trace debug_to_json cluster (5)
- summary: Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.
- ref: `.ticket/tickets/ebf4b601-0f9e-471c-9283-1390cda1eb41/ticket.toml`

<!-- ticket-index:entry id=3661f22c-ebc8-4f6d-ab66-6bada28d6f03 slug=done/unspecified digest=5fb08bcc576e -->
#### [3661f22c] [audit-roadmap][static_complexity][batch-1 follow-up] context-trace formatter cluster (2)
- summary: Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.
- ref: `.ticket/tickets/3661f22c-ebc8-4f6d-ab66-6bada28d6f03/ticket.toml`

<!-- ticket-index:entry id=a8721506-7528-468b-a756-1e21f210fbbe slug=done/unspecified digest=65a388d8c872 -->
#### [a8721506] [audit-roadmap][static_complexity][batch-1 follow-up] context-trace search_path cluster (2)
- summary: Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.
- ref: `.ticket/tickets/a8721506-7528-468b-a756-1e21f210fbbe/ticket.toml`

<!-- ticket-index:entry id=1c9e7b3e-32e8-4233-8fd4-0244c3d9aab0 slug=done/unspecified digest=7b0b668202b7 -->
#### [1c9e7b3e] [audit-roadmap][static_complexity][batch-1] context-stack (38)
- summary: Resolve the current static_complexity batch for context-stack and reduce 38 findings from the baseline.
- ref: `.ticket/tickets/1c9e7b3e-32e8-4233-8fd4-0244c3d9aab0/ticket.toml`

<!-- ticket-index:entry id=d1ef4001-a2a3-4ef4-a1a1-bdfac49c68e2 slug=done/unspecified digest=2ca9f612a282 -->
#### [d1ef4001] [audit-roadmap][static_complexity][batch-2] tools (29)
- summary: Resolve the current static_complexity batch for tools and reduce 29 findings from the baseline.
- ref: `.ticket/tickets/d1ef4001-a2a3-4ef4-a1a1-bdfac49c68e2/ticket.toml`

<!-- ticket-index:entry id=e179f11a-52b5-432e-a13a-330bb3fc5c92 slug=done/unspecified digest=47abc5d3625e -->
#### [e179f11a] [audit-roadmap][static_complexity][batch-3] memory-api (28)
- summary: Resolve the current static_complexity batch for memory-api and reduce 28 findings from the baseline.
- ref: `.ticket/tickets/e179f11a-52b5-432e-a13a-330bb3fc5c92/ticket.toml`

<!-- ticket-index:entry id=1a2b326d-cf0c-4935-83a7-b80cf0ff0f32 slug=done/unspecified digest=4f99afe15ee8 -->
#### [1a2b326d] [audit-roadmap][ticket_graph] Clear ticket graph findings
- summary: Eliminate ticket_graph findings by repairing dependency graph hygiene, orphan handling, and lifecycle consistency across all ticket stores.
- ref: `.ticket/tickets/1a2b326d-cf0c-4935-83a7-b80cf0ff0f32/ticket.toml`

<!-- ticket-index:entry id=6e89260d-2b89-4e0d-9f03-ed10907de19d slug=done/unspecified digest=e9a1cf6985ff -->
#### [6e89260d] [audit-roadmap][ticket_graph] Resolve context-engine dependency_convergence findings (21)
- summary: After audit-roadmap batch-1 (ca788fe3) cleared all 104 orphan_ticket_count findings in the context-engine `.ticket` store, 21 `dependency_convergence_count` findings remain. Each flags a dependent ti...
- ref: `.ticket/tickets/6e89260d-2b89-4e0d-9f03-ed10907de19d/ticket.toml`

<!-- ticket-index:entry id=ca788fe3-67e9-4b5f-97d4-521bbc657bd6 slug=done/unspecified digest=79e694303b67 -->
#### [ca788fe3] [audit-roadmap][ticket_graph][batch-1] context-engine .ticket store (125)
- summary: Resolve the current ticket_graph batch for context-engine .ticket store and reduce 125 findings from the baseline.
- ref: `.ticket/tickets/ca788fe3-67e9-4b5f-97d4-521bbc657bd6/ticket.toml`

<!-- ticket-index:entry id=ba482d65-2443-4a85-964b-c9c605c6ee75 slug=done/unspecified digest=8e24068d4b28 -->
#### [ba482d65] [audit-roadmap][ticket_graph][batch-2] memory-api .ticket store (54)
- summary: Resolve the current ticket_graph batch for memory-api .ticket store and reduce 54 findings from the baseline.
- ref: `.ticket/tickets/ba482d65-2443-4a85-964b-c9c605c6ee75/ticket.toml`

<!-- ticket-index:entry id=22877505-d20f-481d-9ae6-34f7b812901d slug=done/unspecified digest=63998173cba9 -->
#### [22877505] [audit-roadmap][ticket_graph][batch-2][blocker] memory-api orphan findings persist after explicit graph links (38)
- summary: Explain why memory-api orphan findings persisted at 38 after explicit depends_on links in chunk-2, and determine whether the issue is graph data, audit interpretation, or index staleness.
- ref: `.ticket/tickets/22877505-d20f-481d-9ae6-34f7b812901d/ticket.toml`

<!-- ticket-index:entry id=024bcc29-284e-4726-a1a2-96360608865a slug=done/unspecified digest=7dac481504b3 -->
#### [024bcc29] [audit-roadmap][ticket_graph][batch-3] viewer-api .ticket store (50)
- summary: Resolve the current ticket_graph batch for viewer-api .ticket store and reduce 50 findings from the baseline.
- ref: `.ticket/tickets/024bcc29-284e-4726-a1a2-96360608865a/ticket.toml`

<!-- ticket-index:entry id=4e68dc40-a953-45e5-a1aa-e0aecdbcf696 slug=done/unspecified digest=72a5b3b7b71f -->
#### [4e68dc40] [audit-roadmap][ticket_graph][batch-4] memory-viewers .ticket store (23)
- summary: Resolve the current ticket_graph batch for memory-viewers .ticket store and reduce 23 findings from the baseline.
- ref: `.ticket/tickets/4e68dc40-a953-45e5-a1aa-e0aecdbcf696/ticket.toml`

<!-- ticket-index:entry id=cc8e9a36-6fd7-4fb9-adc2-d5456917de34 slug=done/unspecified digest=b051c6001059 -->
#### [cc8e9a36] [audit-roadmap][ticket_graph][batch-5] context-stack/context-editor/misc stores (6)
- summary: Resolve the current ticket_graph batch for context-stack/context-editor/misc stores and reduce 6 findings from the baseline.
- ref: `.ticket/tickets/cc8e9a36-6fd7-4fb9-adc2-d5456917de34/ticket.toml`

<!-- ticket-index:entry id=d760a9bb-b970-4a97-8e38-fb4d78a5ea10 slug=done/unspecified digest=ffb47835e381 -->
#### [d760a9bb] [benchmarks] Capture structured tracing for Criterion and Rust perf harnesses
- summary: Starting benchmark/perf tracing implementation: inspect ticket-api perf tests and representative Criterion benches, then add structured run-level tracing that preserves benchmark usefulness.
- ref: `.ticket/tickets/d760a9bb-b970-4a97-8e38-fb4d78a5ea10/ticket.toml`

<!-- ticket-index:entry id=37dc83ab-af5c-4746-9c02-b27ffb8215a9 slug=done/unspecified digest=c4852e23dc19 -->
#### [37dc83ab] [bug] Tantivy 0.22.1 fastfield panic breaks spec/store full-text search
- summary: Full-text search across the spec store (and any store backed by the same Tantivy index path) is non-functional. `spec scan --force` panics inside Tantivy and incremental scans silently fail to popula...
- ref: `.ticket/tickets/37dc83ab-af5c-4746-9c02-b27ffb8215a9/ticket.toml`

<!-- ticket-index:entry id=cf77062c-f663-4ed4-beca-9303795cf973 slug=done/unspecified digest=bbfca086bff4 -->
#### [cf77062c] [ci] Split viewer workflows to memory-viewers
- ref: `.ticket/tickets/cf77062c-f663-4ed4-beca-9303795cf973/ticket.toml`

<!-- ticket-index:entry id=7c281aec-092c-448d-bf26-94522e07b4e8 slug=done/unspecified digest=a37ed2650a34 -->
#### [7c281aec] [content-materialization][memory-api] G-B.1 retro-fix: typed MoveError variant for interoperability-contract violation (replace string marker)
- summary: Replace the stringly-typed journal contract marker with a typed error variant.
- ref: `memory-api/.ticket/tickets/7c281aec-092c-448d-bf26-94522e07b4e8/ticket.toml`

<!-- ticket-index:entry id=fa6e2c19-5fd8-4305-90c7-f1f4bea8df36 slug=done/unspecified digest=9837e31bec03 -->
#### [fa6e2c19] [content-materialization][memory-api] G-B.2 retro-fix: trait-based interoperability contract (replace runtime validate_interoperability_contract)
- summary: Convert the runtime `validate_interoperability_contract` / `interoperability_gaps` plumbing into a trait-based contract where the type system can enforce the shared minimum interoperability set plus ...
- ref: `memory-api/.ticket/tickets/fa6e2c19-5fd8-4305-90c7-f1f4bea8df36/ticket.toml`

<!-- ticket-index:entry id=17483930-36a1-4bf7-864b-e30ded445db3 slug=done/unspecified digest=b796d61feaaa -->
#### [17483930] [content-materialization][rule-api] G-B: Rust code-design policy — typed errors, trait contracts, trait inheritance
- summary: Author canonical Rust code-design policy as rule-api content and drive the first retro-fixes. Learnings to encode:
- ref: `memory-api/.ticket/tickets/17483930-36a1-4bf7-864b-e30ded445db3/ticket.toml`

<!-- ticket-index:entry id=6875dff6-7191-46aa-9358-9567430969f1 slug=done/unspecified digest=4cf452548d9c -->
#### [6875dff6] [content-materialization][rule-api] G-C: Rule-introduces-spec — status-conditioned spec presentation in session construction
- summary: Require that every spec is introduced/explained in-session by a governing PolicyRule, conditioned on implementation status:
- ref: `memory-api/.ticket/tickets/6875dff6-7191-46aa-9358-9567430969f1/ticket.toml`

<!-- ticket-index:entry id=633a38b8-7200-4979-9ce9-8c296ccc78bd slug=done/unspecified digest=d76b57eb484f -->
#### [633a38b8] [content-materialization][spec-api] G-A: Spec-contract v2 — motivation, dependent-expectation, computed guards, positions
- summary: Extend the `aligned-structure:v1` spec template into a real contract shape. Every spec must declare:
- ref: `memory-api/.ticket/tickets/633a38b8-7200-4979-9ce9-8c296ccc78bd/ticket.toml`

<!-- ticket-index:entry id=e3340271-557d-4d1c-bef7-db73712f468e slug=done/unspecified digest=5e7f1da4e9f2 -->
#### [e3340271] [context-editor] SVO-Accelerated Ray Marching: Per-Voxel SDF Evaluation, Hi-Z Occlusion & Fragment Optimizations
- priority: `high`
- summary: > **Interview:** [interview.md](interview.md) — all 11 design questions answered and finalized.
- ref: `.ticket/tickets/e3340271-557d-4d1c-bef7-db73712f468e/ticket.toml`

<!-- ticket-index:entry id=194ade77-6922-4be8-8c5b-4423173abcf6 slug=done/unspecified digest=c4873d5adc58 -->
#### [194ade77] [context-editor] Tiled Forward+ Rasterizer (Tile Binning + Fragment Compositing)
- priority: `high`
- summary: The final rendering stage: bin sorted voxel splats into 16×16 pixel tiles, then composite them per-pixel with front-to-back alpha blending in a fragment shader. Each pixel evaluates a **ray-box SDF**...
- ref: `.ticket/tickets/194ade77-6922-4be8-8c5b-4423173abcf6/ticket.toml`

<!-- ticket-index:entry id=33463861-ffba-4ead-905e-5d867b707936 slug=done/unspecified digest=7a8060c64d2a -->
#### [33463861] [context-enrichment] Dogfood sessions_for_ticket against ticket 06cfe998 and record findings
- summary: Run a manual dogfood pass of `sessions_for_ticket` against ticket `06cfe998` ([token-efficiency] Introduce peek-api with peek-cli and peek-mcp transport layers, currently `in-review`), then record fi...
- ref: `.ticket/tickets/33463861-ffba-4ead-905e-5d867b707936/ticket.toml`

<!-- ticket-index:entry id=87001cb8-46c4-4921-a336-dc0cf0c1f66a slug=done/unspecified digest=4224a515c768 -->
#### [87001cb8] [doc-api] Add documentation-validation evidence identities for spec fulfillment
- summary: Extend `doc-api` with documentation-validation identities and coverage metadata that can satisfy or block spec acceptance clauses.
- ref: `.ticket/tickets/87001cb8-46c4-4921-a336-dc0cf0c1f66a/ticket.toml`

<!-- ticket-index:entry id=363e26d6-0b4d-469e-a53b-5c3424262085 slug=done/unspecified digest=e9463ef5bda4 -->
#### [363e26d6] [e2e] Correlate Playwright browser runs with backend tracing sessions
- summary: Starting Playwright/backend tracing correlation work: inspect shared viewer E2E runtime, server log surfaces, and existing client-log/tracing helpers to introduce a stable per-test correlation id.
- ref: `.ticket/tickets/363e26d6-0b4d-469e-a53b-5c3424262085/ticket.toml`

<!-- ticket-index:entry id=7e8bc1c3-c4dd-431a-965d-adf3ba0a1ad6 slug=done/unspecified digest=f1b07ac89ec1 -->
#### [7e8bc1c3] [epic] Guidance corpus quick-win track
- priority: `high`
- summary: Batches the low-architecture-risk guidance-corpus tickets that can be executed without waiting on the Planner/Worker spec track. Closes when its three children are done.
- ref: `.ticket/tickets/7e8bc1c3-c4dd-431a-965d-adf3ba0a1ad6/ticket.toml`

<!-- ticket-index:entry id=f13c9836-5b74-433b-b3d2-e1475d080ad0 slug=done/unspecified digest=9e3db4517834 -->
#### [f13c9836] [epic] Worker-tier dispatch contract
- priority: `medium`
- summary: Batches the tickets that define the Planner/Worker dispatch contract: the architecture spec plus the three instruction-level policies that depend on its vocabulary. Closes when all four children are ...
- ref: `.ticket/tickets/f13c9836-5b74-433b-b3d2-e1475d080ad0/ticket.toml`

<!-- ticket-index:entry id=9c95c1e4-3cdb-428e-b9de-800684651226 slug=done/unspecified digest=7ed1595e3efc -->
#### [9c95c1e4] [feedback-api] Event ingestion, metadata normalization, and retention policy
- priority: `high`
- summary: Define feedback event ingestion for human and privileged-agent authors, normalize metadata, and establish retention/privacy boundaries.
- ref: `memory-api/.ticket/tickets/9c95c1e4-3cdb-428e-b9de-800684651226/ticket.toml`

<!-- ticket-index:entry id=2e52bd26-1a93-4c62-b712-024a567a934a slug=done/unspecified digest=24cba0cc7e55 -->
#### [2e52bd26] [handoff][ticket-workflow] Work package: regression-resistant best-next-ticket workflow
- summary: Bundle the "best next ticket to implement" hardening work into a single handoff package that another engineer or agent can pick up without needing to reconstruct the backlog from chat history.
- ref: `.ticket/tickets/2e52bd26-1a93-4c62-b712-024a567a934a/ticket.toml`

<!-- ticket-index:entry id=90279c46-6c9b-42a5-a60e-3ac8bfad346a slug=done/unspecified digest=dbba78b25ba9 -->
#### [90279c46] [hooks][rule] Make pre-commit validate only repo-local rule targets
- ref: `.ticket/tickets/90279c46-6c9b-42a5-a60e-3ac8bfad346a/ticket.toml`

<!-- ticket-index:entry id=1f7a7d60-e7ea-49c4-80c4-dee78e8862be slug=done/unspecified digest=8c9cfbb99531 -->
#### [1f7a7d60] [hooks][rule] Show sync-targets failure output in pre-commit
- ref: `.ticket/tickets/1f7a7d60-e7ea-49c4-80c4-dee78e8862be/ticket.toml`

<!-- ticket-index:entry id=30606247-06cd-4246-905b-bad49d2dd289 slug=done/unspecified digest=486713c22325 -->
#### [30606247] [instruction-governance] Add instruction precedence + exception matrix in AGENTS.md
- summary: Added an explicit instruction precedence + exception matrix to AGENTS through the canonical rule system, then regenerated AGENTS via target-scoped generation.
- ref: `.ticket/tickets/30606247-06cd-4246-905b-bad49d2dd289/ticket.toml`

<!-- ticket-index:entry id=e416e4e8-cab2-41ad-9fcf-63da5f444e0a slug=done/unspecified digest=0fe6cece2524 -->
#### [e416e4e8] [instruction-governance] Consolidate duplicated instruction guidance and keep pointer surfaces minimal
- summary: Final consolidation pass completed for instruction-governance track.
- ref: `.ticket/tickets/e416e4e8-cab2-41ad-9fcf-63da5f444e0a/ticket.toml`

<!-- ticket-index:entry id=f19dcafa-8349-4375-96ab-401a7d0eb17c slug=done/unspecified digest=19533d9ca918 -->
#### [f19dcafa] [instruction-governance] Deduplicate spec-system.instructions and add explicit exceptions
- summary: Deduplicated `spec-system.instructions.md` and added explicit precedence/exception handling aligned with the AGENTS instruction-precedence matrix.
- ref: `.ticket/tickets/f19dcafa-8349-4375-96ab-401a7d0eb17c/ticket.toml`

<!-- ticket-index:entry id=18e7a4d1-6ac3-402a-be3f-9f588795f006 slug=done/unspecified digest=576ecda9b569 -->
#### [18e7a4d1] [instruction-governance] Narrow over-broad applyTo patterns in instruction files
- summary: Narrowed over-broad `applyTo` selectors in instruction frontmatter to reduce unintended rule activation.
- ref: `.ticket/tickets/18e7a4d1-6ac3-402a-be3f-9f588795f006/ticket.toml`

<!-- ticket-index:entry id=cf7f79a6-6011-4060-811d-a3831de4c3b7 slug=done/unspecified digest=366e17535267 -->
#### [cf7f79a6] [instruction-governance] Resolve formatting-rule conflict (backticks vs linkified paths)
- summary: Resolved the formatting-rule conflict by making precedence/exception behavior explicit and adding a canonical formatting policy that selects linkified file references over backticks for file/path cit...
- ref: `.ticket/tickets/cf7f79a6-6011-4060-811d-a3831de4c3b7/ticket.toml`

<!-- ticket-index:entry id=e1d8be15-0542-4804-bb0f-6a4b4fb4b073 slug=done/unspecified digest=9d11809c9bfe -->
#### [e1d8be15] [instruction-governance] Tracker: instruction-system cleanup and conflict resolution
- summary: Instruction-governance cleanup track completed.
- ref: `.ticket/tickets/e1d8be15-0542-4804-bb0f-6a4b4fb4b073/ticket.toml`

<!-- ticket-index:entry id=0805fb76-f99b-45a5-87c6-5a8e65bdb2da slug=done/unspecified digest=d9a2d959d859 -->
#### [0805fb76] [log-api] Bootstrap validation-log identities for spec fulfillment
- summary: Bootstrap the first `log-api` entities for validation-log capture and retrieval linked from `test-api` executions.
- ref: `.ticket/tickets/0805fb76-f99b-45a5-87c6-5a8e65bdb2da/ticket.toml`

<!-- ticket-index:entry id=f3a58d3c-5961-4ee4-a32d-b51343a5a275 slug=done/unspecified digest=b0deaf99646c -->
#### [f3a58d3c] [memory-api] Cross-store edge health: shared resolver + policy-aware parent-workspace warning (parity across all stores)
- summary: Store-scoped health checks report `dangling_edge` ERRORS for `depends_on` edges that point to entities in a DIFFERENT store/workspace (e.g. memory-api tickets depending on root-store architecture tic...
- ref: `memory-api/.ticket/tickets/f3a58d3c-5961-4ee4-a32d-b51343a5a275/ticket.toml`

<!-- ticket-index:entry id=e8e3ef17-313f-4cb7-aa9c-6447a18d36a3 slug=done/unspecified digest=afc26b34f0e3 -->
#### [e8e3ef17] [memory-api] Implement path normalization kernel and migrate CLI/MCP/HTTP path surfaces
- summary: Implement the path normalization utility kernel specified in the design spec and migrate all transport-facing path surfaces to it.
- ref: `memory-api/.ticket/tickets/e8e3ef17-313f-4cb7-aa9c-6447a18d36a3/ticket.toml`

<!-- ticket-index:entry id=45f5f58e-4a6e-40b9-bf03-1bc9dc5ca43d slug=done/unspecified digest=8a22d8a15f8f -->
#### [45f5f58e] [memory-api] Incremental entity-store scan (mtime/hash short-circuit)
- summary: Make `EntityStore::scan` skip re-integration for unchanged entities so sync-targets (and other store consumers) do not re-read+re-index every rule every run.
- ref: `memory-api/.ticket/tickets/45f5f58e-4a6e-40b9-bf03-1bc9dc5ca43d/ticket.toml`

<!-- ticket-index:entry id=e3961a54-ea4c-4ce6-aee9-da67a15bf2c7 slug=done/unspecified digest=f263daf129b3 -->
#### [e3961a54] [memory-api] Path normalization kernel design + UNC/verbatim regression guard tests
- summary: Implementation-ready design for a single resilient path normalization utility kernel in memory-api, focused on Unix-style canonical path rendering across Windows/Unix environments, including explicit...
- ref: `memory-api/.ticket/tickets/e3961a54-ea4c-4ce6-aee9-da67a15bf2c7/ticket.toml`

<!-- ticket-index:entry id=64a7cb3a-b35f-4953-9368-0d7afc89fb53 slug=done/unspecified digest=92ac42bb7213 -->
#### [64a7cb3a] [memory-api] Sync install contract README section with canonical rule entry
- ref: `.ticket/tickets/64a7cb3a-b35f-4953-9368-0d7afc89fb53/ticket.toml`

<!-- ticket-index:entry id=15ce7eab-4048-48f2-9296-4c427f23455d slug=done/unspecified digest=0897e0d392bf -->
#### [15ce7eab] [memory-api][move-kernel] Emit tracing spans for journaled move execution
- summary: Starting implementation on move-kernel tracing spans/events so journal phase timings are visible through the tracing pipeline as well as the MoveJournal artifact.
- ref: `.ticket/tickets/15ce7eab-4048-48f2-9296-4c427f23455d/ticket.toml`

<!-- ticket-index:entry id=8ad77570-6dbe-4647-9d58-bf324a82b4fc slug=done/unspecified digest=8744ff37f03d -->
#### [8ad77570] [memory-matrix] Promote subprocess failure-bundle log_session_ids linkage
- summary: Close the remaining observability gap where subprocess failure bundles still emit empty `linkage.log_session_ids` when runtime log session capture is unavailable.
- ref: `.ticket/tickets/8ad77570-6dbe-4647-9d58-bf324a82b4fc/ticket.toml`

<!-- ticket-index:entry id=f97d7086-999b-41e0-8ded-7829251223cd slug=done/unspecified digest=c4e713bb150f -->
#### [f97d7086] [planning][workspace-policy] unify recovery hints across memory-api stores
- ref: `.ticket/tickets/f97d7086-999b-41e0-8ded-7829251223cd/ticket.toml`

<!-- ticket-index:entry id=bf8ef22e-ea06-45de-9f90-a2fee0e4cc6e slug=done/unspecified digest=e0e97ef7572d -->
#### [bf8ef22e] [repo-guidance][rule-api] Add implement agent target from canonical rules
- summary: The repository generates custom agents for research, testing, interview, and audit, but it does not provide a dedicated implement agent that is optimized for surgical execution once the scope is clea...
- ref: `.ticket/tickets/bf8ef22e-ea06-45de-9f90-a2fee0e4cc6e/ticket.toml`

<!-- ticket-index:entry id=37d7fac3-cc7d-44b9-b6e1-f199fca8e901 slug=done/unspecified digest=ba8209fcb3da -->
#### [37d7fac3] [repo-guidance][spec-api] Overhaul behavior-first spec contract and guidance
- summary: Overhaul the repository guidance for describing target behavior so specs become concise behavior contracts anchored by the spec store, entity references, validation triangulation, and related impleme...
- ref: `.ticket/tickets/37d7fac3-cc7d-44b9-b6e1-f199fca8e901/ticket.toml`

<!-- ticket-index:entry id=0e1dca8b-2869-4c43-b62b-79eb5f6f3a17 slug=done/unspecified digest=0bca153a6e6e -->
#### [0e1dca8b] [repo] Keep submodules on main instead of detached HEADs
- priority: `high`
- summary: Top-level submodules `memory-viewers` and `context-stack` are checked out in detached HEAD state, and local commits in those submodules are not naturally advancing `origin/main`. Update repository-ow...
- ref: `.ticket/tickets/0e1dca8b-2869-4c43-b62b-79eb5f6f3a17/ticket.toml`

<!-- ticket-index:entry id=efdcbb83-b198-4444-ba92-df39079d3004 slug=done/unspecified digest=7a4a2fe658aa -->
#### [efdcbb83] [rule-api] Add structured tracing for RuleStore runtime operations
- summary: Reopening RuleStore tracing ticket to resolve the remaining rule-api test failures blocking closure, then rerun full crate validation.
- ref: `.ticket/tickets/efdcbb83-b198-4444-ba92-df39079d3004/ticket.toml`

<!-- ticket-index:entry id=66df8683-1e3e-44d5-9052-4bb72277dc57 slug=done/unspecified digest=dc8dc9162c9a -->
#### [66df8683] [rule-audit][prompts] Deduplicate high-overlap rules from fresh overlap audit
- summary: Goal: reduce the high-overlap rule pairs surfaced by the fresh rule-overlap audit and leave a cleaner, more canonical prompt/instruction rule set for future regenerations.
- ref: `.ticket/tickets/66df8683-1e3e-44d5-9052-4bb72277dc57/ticket.toml`

<!-- ticket-index:entry id=68c61b92-af6b-4331-99fd-5a77dd3512e1 slug=done/unspecified digest=191197b6fc2e -->
#### [68c61b92] [rule-cli] Improve generate-target errors for config directories and output-path targets
- ref: `.ticket/tickets/68c61b92-af6b-4331-99fd-5a77dd3512e1/ticket.toml`

<!-- ticket-index:entry id=f4f955b0-a827-4fce-882d-4df2f5903891 slug=done/unspecified digest=8887f9974548 -->
#### [f4f955b0] [rule-cli] Make scan output explain diagnostics and counters
- ref: `.ticket/tickets/f4f955b0-a827-4fce-882d-4df2f5903891/ticket.toml`

<!-- ticket-index:entry id=2dc02b9b-7e94-4cb4-82c4-83e4359792cb slug=done/unspecified digest=6337fc445d49 -->
#### [2dc02b9b] [rule-cli] Normalize path separators in sync-targets output
- summary: Every path field in sync-targets output must use `/` separators on all hosts.
- ref: `memory-api/.ticket/tickets/2dc02b9b-7e94-4cb4-82c4-83e4359792cb/ticket.toml`

<!-- ticket-index:entry id=6e7f9bd6-80e8-4f0d-86d9-128eb257eb32 slug=done/unspecified digest=489c08e31475 -->
#### [6e7f9bd6] [rule-cli] sync-targets: incremental work + normalized path output
- summary: Tracker for fixing two `rule sync-targets` defects.
- ref: `memory-api/.ticket/tickets/6e7f9bd6-80e8-4f0d-86d9-128eb257eb32/ticket.toml`

<!-- ticket-index:entry id=69b38924-485a-41fb-bdc8-1423b5d82cc2 slug=done/unspecified digest=124ecd999cba -->
#### [69b38924] [rule-cli] sync-targets: skip unchanged writes, single collection pass, reuse SpecStore
- summary: Reduce redundant per-run work in sync_targets_payload without changing generated content.
- ref: `memory-api/.ticket/tickets/69b38924-485a-41fb-bdc8-1423b5d82cc2/ticket.toml`

<!-- ticket-index:entry id=331f331d-b618-42db-9284-195c2e410a11 slug=done/unspecified digest=0e47f17fdeb8 -->
#### [331f331d] [rule-cli] v1 reverse-sync from generated artifacts (file-only sync-rules)
- summary: Implement v1 reverse-sync for generated rule artifacts using canonical `rule-api:entry` ids so generated-file edits can update existing rule bodies in place.
- ref: `.ticket/tickets/331f331d-b618-42db-9284-195c2e410a11/ticket.toml`

<!-- ticket-index:entry id=665b727c-09b1-43e0-8795-eb67e2758aea slug=done/unspecified digest=a43e6d2629fb -->
#### [665b727c] [rule-cli][rule-api] Allow rule-targets directories for generate-target configs
- ref: `.ticket/tickets/665b727c-09b1-43e0-8795-eb67e2758aea/ticket.toml`

<!-- ticket-index:entry id=e7a31e70-e8f8-4369-aae4-98cc7f35db7c slug=done/unspecified digest=6c3366e98f96 -->
#### [e7a31e70] [rule-cli][rule-api] Require explicit child-workspace scans for render commands
- ref: `.ticket/tickets/e7a31e70-e8f8-4369-aae4-98cc7f35db7c/ticket.toml`

<!-- ticket-index:entry id=0da01943-4bab-44eb-bc4b-c803f6526b26 slug=done/unspecified digest=e12677c9ed71 -->
#### [0da01943] [rules][copilot] Integrate RTK section into generated copilot instructions
- ref: `.ticket/tickets/0da01943-4bab-44eb-bc4b-c803f6526b26/ticket.toml`

<!-- ticket-index:entry id=6b2dc497-188c-44f5-9106-bf35deecb7a1 slug=done/unspecified digest=0864e58b2e88 -->
#### [6b2dc497] [session-api] Add init/pin/unpin/view to session-cli and session-mcp
- summary: Expose runtime context, workflow, rendering, handoff/resume, and finish operations through CLI and MCP.
- ref: `memory-api/.ticket/tickets/6b2dc497-188c-44f5-9106-bf35deecb7a1/ticket.toml`

<!-- ticket-index:entry id=627d4152-36a7-4b24-9a9c-5f047abcac60 slug=done/unspecified digest=37467b16d5f4 -->
#### [627d4152] [session-api] Implement session_audit and schema_version tagging for persisted sessions
- summary: Implement `session_audit` in session-api and tag persisted session records with a schema version.
- ref: `.ticket/tickets/627d4152-36a7-4b24-9a9c-5f047abcac60/ticket.toml`

<!-- ticket-index:entry id=412964a3-e1c3-47da-94ad-268ff20441c0 slug=done/unspecified digest=427b4605dd02 -->
#### [412964a3] [session-api] Runtime session-context model (pinned_entities, init/pin/unpin)
- priority: `high`
- summary: Extend `session-api` from a write/archive store into the durable read/runtime foundation used by pinned context and the session workflow.
- ref: `memory-api/.ticket/tickets/412964a3-e1c3-47da-94ad-268ff20441c0/ticket.toml`

<!-- ticket-index:entry id=2d8d0487-1680-424c-816d-01925e187e62 slug=done/unspecified digest=67bb265d3133 -->
#### [2d8d0487] [session-api][audit-cli] Add session_audit in unified audit interface with schema-versioned sessions
- summary: Plan and deliver a `session_audit` feature in session-api and expose it through audit-cli's unified audit interface for reviewing a specific session id or the latest persisted session.
- ref: `.ticket/tickets/2d8d0487-1680-424c-816d-01925e187e62/ticket.toml`

<!-- ticket-index:entry id=5e99cc3e-5b9e-4ca1-a54c-cbdf82444b50 slug=done/unspecified digest=401f120aeeda -->
#### [5e99cc3e] [session-api][audit-cli] Define spec contract for session_audit and session schema version
- summary: Create/update the owning spec contract for session audit reporting and schema-versioned persisted session artifacts.
- ref: `.ticket/tickets/5e99cc3e-5b9e-4ca1-a54c-cbdf82444b50/ticket.toml`

<!-- ticket-index:entry id=474eb962-b68f-4651-b980-c4c9233b2710 slug=done/unspecified digest=e42d44969192 -->
#### [474eb962] [session-api][audit-cli] Plan policy-file updates for session_audit and schema-version guidance
- summary: Updated handoff policy sources and regenerated prompts to require upfront shorthand/placeholder legends, explicit ticket legends, and unresolved-reference guards. Validated generated handoff and hand...
- ref: `.ticket/tickets/474eb962-b68f-4651-b980-c4c9233b2710/ticket.toml`

<!-- ticket-index:entry id=f1161fae-b1fc-4cc1-baae-18c0eb7e7868 slug=done/unspecified digest=a63031a6260e -->
#### [f1161fae] [session-api][audit-cli] Validate session_audit feature and evidence package
- summary: Validate and package review evidence for session audit + schema versioning changes.
- ref: `.ticket/tickets/f1161fae-b1fc-4cc1-baae-18c0eb7e7868/ticket.toml`

<!-- ticket-index:entry id=b3155a94-230e-416b-be0e-5948d6d2193a slug=done/unspecified digest=39d556ca0c35 -->
#### [b3155a94] [session-api][hook] Fix Stop hook transcript capture robustness and workspace slug consistency
- summary: Validation passed: cargo test -p session-api (28 passed). End-to-end hook invocation with transcriptPath/workspaceSlug + modern transcript events persists turns and is readable via session-cli peek-s...
- ref: `memory-api/.ticket/tickets/b3155a94-230e-416b-be0e-5948d6d2193a/ticket.toml`

<!-- ticket-index:entry id=6b1edff1-bc32-40c7-b3a9-fb1292b0213f slug=done/unspecified digest=b5f0d092ffb8 -->
#### [6b1edff1] [session-api][remediation] Authoritative finish, live ticket state, atomic durability, and CLI contract
- priority: `high`
- summary: Repairs correctness holes found during independent review of the durable
- ref: `memory-api/.ticket/tickets/6b1edff1-bc32-40c7-b3a9-fb1292b0213f/ticket.toml`

<!-- ticket-index:entry id=bba9b313-ff13-4fd1-91d4-6485a6c2f4de slug=done/unspecified digest=fe846bc6869e -->
#### [bba9b313] [session-api][session-cli][session-mcp] Add sessions_for_ticket query with selectable relation-strength tiers
- summary: Add a dedicated query capability to the Session API that answers "which sessions worked on ticket X", using relation signals that already exist in the session data model but are not queryable today.
- ref: `.ticket/tickets/bba9b313-ff13-4fd1-91d4-6485a6c2f4de/ticket.toml`

<!-- ticket-index:entry id=203248cb-0694-481b-a634-ba7d70962750 slug=done/unspecified digest=0050f675cbfc -->
#### [203248cb] [session-api][session-mcp] Separate behavioral vs descriptive workflow node kinds; add Spec kind
- summary: The session workflow `kind` field conflates two orthogonal concerns and gets the restriction
- ref: `.ticket/tickets/203248cb-0694-481b-a634-ba7d70962750/ticket.toml`

<!-- ticket-index:entry id=afa00b5c-c736-4d75-b157-d3e9ce90d819 slug=done/unspecified digest=7d950a775240 -->
#### [afa00b5c] [session-bootstrap][design] Define bootstrap contract, session_context schema, and ADRs
- summary: Planning/design ticket for the [session-bootstrap] epic. Produces the specs and the resolved decisions.
- ref: `memory-api/.ticket/tickets/afa00b5c-c736-4d75-b157-d3e9ce90d819/ticket.toml`

<!-- ticket-index:entry id=b4a8dc5e-9d80-4fea-bb42-0c30aba0ecd6 slug=done/unspecified digest=35654bc6a361 -->
#### [b4a8dc5e] [session-bootstrap][rules] Minimal bootstrapper + selective instruction loading
- summary: Operationalizes decision **D7**: stop force-loading static guidance; make it discoverable and agent-rendered.
- ref: `memory-api/.ticket/tickets/b4a8dc5e-9d80-4fea-bb42-0c30aba0ecd6/ticket.toml`

<!-- ticket-index:entry id=7f1ed44f-73f3-40c9-9647-d899c64ec507 slug=done/unspecified digest=40270f17875f -->
#### [7f1ed44f] [session-mcp][schema] Enum-constrain and document workflow mutation parameters
- summary: The session-mcp workflow mutation tools accept enum-valued parameters typed as bare `String`,
- ref: `.ticket/tickets/7f1ed44f-73f3-40c9-9647-d899c64ec507/ticket.toml`

<!-- ticket-index:entry id=3eaceaae-254e-4a9f-ab19-c1eed2080931 slug=done/unspecified digest=b81b9572868e -->
#### [3eaceaae] [session-mcp][session-api] Surface and echo workspace_session_id inline
- summary: `workspace_session_id` is a mandatory parameter on every session-mcp workflow call
- ref: `.ticket/tickets/3eaceaae-254e-4a9f-ab19-c1eed2080931/ticket.toml`

<!-- ticket-index:entry id=1400919a-84b9-49ff-8e8a-92a7d9068594 slug=done/unspecified digest=c5c80510bba2 -->
#### [1400919a] [session-optimization] Resolve tool-calling reliability and session evidence gaps
- priority: `high`
- summary: Track and resolve high-impact tool-calling reliability and session-evidence quality issues observed in session `a0228f9f-bbac-4c82-b1e6-8a628aa91ec1`.
- ref: `.ticket/tickets/1400919a-84b9-49ff-8e8a-92a7d9068594/ticket.toml`

<!-- ticket-index:entry id=c851f3af-433a-496e-a586-28631de142ce slug=done/unspecified digest=8c852ea6c9b5 -->
#### [c851f3af] [session-optimization] empty assistant.message events create session-noise
- priority: `medium`
- summary: Raw session events include high-volume empty assistant-message records during tool-heavy phases, inflating artifacts and obscuring meaningful narration.
- ref: `.ticket/tickets/c851f3af-433a-496e-a586-28631de142ce/ticket.toml`

<!-- ticket-index:entry id=b6cdc89d-30fc-4303-aaba-f959abfeda4b slug=done/unspecified digest=cecadb185a32 -->
#### [b6cdc89d] [session-optimization] run_in_terminal sync enters ambiguous background state
- priority: `high`
- summary: `run_in_terminal` in `mode=sync` moved a command to background despite no explicit timeout and no clear completion signal, then required repeated polling and eventual manual termination.
- ref: `.ticket/tickets/b6cdc89d-30fc-4303-aaba-f959abfeda4b/ticket.toml`

<!-- ticket-index:entry id=7769da57-a8f6-4e72-a860-c8263d5a360e slug=done/unspecified digest=6359ba32dbeb -->
#### [7769da57] [session-optimization] session events miss structured tool result payloads
- priority: `high`
- summary: Session events capture tool start/complete envelopes but omit structured tool result payloads, reducing post-session debuggability for tool-calling failures.
- ref: `.ticket/tickets/7769da57-a8f6-4e72-a860-c8263d5a360e/ticket.toml`

<!-- ticket-index:entry id=c73d4a6b-2610-4e69-9fc3-bfedcf2ec53d slug=done/unspecified digest=743f415d95a1 -->
#### [c73d4a6b] [spec-api] Add native expectation, acceptance, and evidence fields
- summary: Extend `spec-api` with native fields and validation for expected properties, acceptance clauses, and evidence requirements.
- ref: `.ticket/tickets/c73d4a6b-2610-4e69-9fc3-bfedcf2ec53d/ticket.toml`

<!-- ticket-index:entry id=e813f958-f0ee-42c3-a732-e26516eef311 slug=done/unspecified digest=160ba182c0c3 -->
#### [e813f958] [spec-api] Add structured tracing for SpecStore runtime operations
- summary: Starting SpecStore tracing implementation: add structured spans/events for open/init/open_or_init/scan and slug-index rebuild flows, then validate targeted spec-api tests.
- ref: `.ticket/tickets/e813f958-f0ee-42c3-a732-e26516eef311/ticket.toml`

<!-- ticket-index:entry id=b744bcf5-05a5-4601-bbe1-caae9d42ea5f slug=done/unspecified digest=88a806f355b5 -->
#### [b744bcf5] [spec-api] Expectation-oriented spec contract and model
- summary: Redefine the specification contract and the native `spec-api` model so the repository can represent expected properties, acceptance clauses, and evidence requirements without relying on free-form pro...
- ref: `.ticket/tickets/b744bcf5-05a5-4601-bbe1-caae9d42ea5f/ticket.toml`

<!-- ticket-index:entry id=830de529-2818-49fd-a792-3b59dd99a748 slug=done/unspecified digest=50361b8570ac -->
#### [830de529] [spec-api] Normalize spec create target roots into .spec/specs
- summary: Fix spec creation target-root handling so spec entities are always created inside the canonical .spec/specs store when the caller passes a workspace root, the .spec store root, or a path inside that ...
- ref: `memory-api/.ticket/tickets/830de529-2818-49fd-a792-3b59dd99a748/ticket.toml`

<!-- ticket-index:entry id=0b6e1bf3-2478-40a5-a619-085d8691835a slug=done/unspecified digest=3f8096cb7cbe -->
#### [0b6e1bf3] [spec-api][rules] Define blackbox contract and authoring guidance for expectation-oriented specs
- summary: Add failing blackbox tests and update the concrete authoring guidance surfaces so expectation-oriented specs are defined by intended properties, acceptance criteria, and evidence requirements rather ...
- ref: `.ticket/tickets/0b6e1bf3-2478-40a5-a619-085d8691835a/ticket.toml`

<!-- ticket-index:entry id=eaeaf157-85c3-4caf-a538-4f6ebb2a5ec7 slug=done/unspecified digest=4e8c7a10ca2d -->
#### [eaeaf157] [spec-cli] Normalize refs validate workspace_root JSON paths on Windows
- summary: Fix `spec refs <id> validate` so its JSON `workspace_root` output uses slash-normalized paths on Windows, matching existing expectations in the command tests and the output contract already used by `...
- ref: `.ticket/tickets/eaeaf157-85c3-4caf-a538-4f6ebb2a5ec7/ticket.toml`

<!-- ticket-index:entry id=59d96577-09a8-44a7-b0ea-3d51b3a6fb05 slug=done/unspecified digest=54ef3a6048a5 -->
#### [59d96577] [spec-cli][spec-mcp] Make spec workflows root-aware across nested .spec stores
- summary: Fixed Windows workspace-root normalization in spec-cli refs payload rendering. Focused validation now passes (spec-cli refs tests plus package slices). Recorded native .test evidence under vt-spec-ro...
- ref: `memory-api/.ticket/tickets/59d96577-09a8-44a7-b0ea-3d51b3a6fb05/ticket.toml`

<!-- ticket-index:entry id=c666f0b3-f1e6-4073-852f-e494bf5c1272 slug=done/unspecified digest=1719a4365d3a -->
#### [c666f0b3] [spec-cli][spec-mcp][spec-http] Expose expectation and evidence parity across transports
- summary: Expose the expectation and evidence model consistently through `spec-cli`, `spec-mcp`, and `spec-http`, with one shared parity contract.
- ref: `.ticket/tickets/c666f0b3-f1e6-4073-852f-e494bf5c1272/ticket.toml`

<!-- ticket-index:entry id=38bbb8e2-19d1-4522-8577-dd922e70d6b7 slug=done/unspecified digest=88d91be48ee3 -->
#### [38bbb8e2] [spec-system] Migrate all specs to aligned behavior-first structure
- summary: Migrate all existing specs in the workspace store to the aligned behavior-first structure.
- ref: `.ticket/tickets/38bbb8e2-19d1-4522-8577-dd922e70d6b7/ticket.toml`

<!-- ticket-index:entry id=690578c4-1c47-430b-9258-4edaa0a82d73 slug=done/unspecified digest=751615159027 -->
#### [690578c4] [spec-system] Repair six missed aligned behavior-first spec bodies
- ref: `.ticket/tickets/690578c4-1c47-430b-9258-4edaa0a82d73/ticket.toml`

<!-- ticket-index:entry id=2d2c3e94-3d55-457f-8c06-ace46f4b3d89 slug=done/unspecified digest=349156548330 -->
#### [2d2c3e94] [spec-viewer][P5.1] Multi-spec tabs via TabsStore
- summary: Replace the single-spec right panel in spec-viewer with `TabsStore<SpecId>` (from viewer-api P2).
- ref: `memory-viewers/.ticket/tickets/2d2c3e94-3d55-457f-8c06-ace46f4b3d89/ticket.toml`

<!-- ticket-index:entry id=b2b02558-4620-49ed-b3db-092e5cee840c slug=done/unspecified digest=e07268a130af -->
#### [b2b02558] [spec-viewer][P5.2] Breadcrumbs above spec detail tabs
- summary: Render the viewer-api `Breadcrumbs` (P1) above the spec detail tabs.
- ref: `memory-viewers/.ticket/tickets/b2b02558-4620-49ed-b3db-092e5cee840c/ticket.toml`

<!-- ticket-index:entry id=b2dc000c-3e4e-44fd-980b-0728895d0177 slug=done/unspecified digest=4dc7ee21b3a5 -->
#### [b2dc000c] [spec-viewer][P5.3] CategoryPage uses CardGrid/CardSection
- summary: Replace the hand-rolled spec list cards with viewer-api `CardGrid`/`Card`/`CardSection` (P1).
- ref: `memory-viewers/.ticket/tickets/b2dc000c-3e4e-44fd-980b-0728895d0177/ticket.toml`

<!-- ticket-index:entry id=74424d6f-327d-4de3-8dd2-3f26962c171d slug=done/unspecified digest=3bc793c00ad5 -->
#### [74424d6f] [spec-viewer][P5.4] Theme settings in Modal overlay
- summary: Move the floating theme settings panel (`theme-settings-floating`) into the viewer-api `Modal` overlay (P1).
- ref: `memory-viewers/.ticket/tickets/74424d6f-327d-4de3-8dd2-3f26962c171d/ticket.toml`

<!-- ticket-index:entry id=d075e565-3463-410f-82ee-fb82f34cffc9 slug=done/unspecified digest=bd5e7b7f8166 -->
#### [d075e565] [spec-viewer][P5.5] HeaderActions replaces manual nav buttons
- summary: Replace the manual nav buttons in `tools/viewer/spec-viewer/frontend/dioxus/src/routes.rs` with the viewer-api `HeaderActions` component (P3).
- ref: `memory-viewers/.ticket/tickets/d075e565-3463-410f-82ee-fb82f34cffc9/ticket.toml`

<!-- ticket-index:entry id=4f69b73e-8352-4b4a-8a8b-93ad6b65c056 slug=done/unspecified digest=ce4dfc3dbf06 -->
#### [4f69b73e] [spec-viewer][P5.6] URL routing via PathCodec + tree expand
- summary: Adopt the viewer-api `PathCodec`/`expand_path_to` (P2) so the active spec id roundtrips through the URL.
- ref: `memory-viewers/.ticket/tickets/4f69b73e-8352-4b4a-8a8b-93ad6b65c056/ticket.toml`

<!-- ticket-index:entry id=10009542-af00-44e8-9b43-ea5d12bf1d6c slug=done/unspecified digest=7fa293cf2a9c -->
#### [10009542] [spec-viewer][P5.7] Prefetcher for sibling specs
- summary: Wrap the spec-fetch API in the viewer-api `Prefetcher` (P2) so that siblings of the active spec are eagerly loaded.
- ref: `memory-viewers/.ticket/tickets/10009542-af00-44e8-9b43-ea5d12bf1d6c/ticket.toml`

<!-- ticket-index:entry id=19bb3b4c-61fe-4270-91db-3ea27a819445 slug=done/unspecified digest=e3361d82a11b -->
#### [19bb3b4c] [spec] Migrate shared workflow validation sections to spec-doc rules
- summary: Migrate a first set of root workflow architecture specs to rule-backed `spec-doc` generation so repeated important sections stop drifting. Start with the three workflow metadata specs that share a pl...
- ref: `.ticket/tickets/19bb3b4c-61fe-4270-91db-3ea27a819445/ticket.toml`

<!-- ticket-index:entry id=86bf3da2-b6cc-4fc7-898d-044403283550 slug=done/unspecified digest=22a5f823664a -->
#### [86bf3da2] [test-api] Bootstrap validation specification and execution identities for spec fulfillment
- summary: Bootstrap the first `test-api` entities for validation specifications, executions, and outcomes used by expectation-oriented spec fulfillment.
- ref: `.ticket/tickets/86bf3da2-b6cc-4fc7-898d-044403283550/ticket.toml`

<!-- ticket-index:entry id=6f3dcdfc-bf2f-45d7-9776-0f0a360ac199 slug=done/unspecified digest=583e875f214a -->
#### [6f3dcdfc] [test-cli] Add test-result store and `test` CLI for validation evidence
- summary: `test-api` currently only defines validation identities (`ValidationSpec`, `ValidationExecution`) with no persistence and no CLI. Validation results are being written verbatim into ticket description...
- ref: `.ticket/tickets/6f3dcdfc-bf2f-45d7-9776-0f0a360ac199/ticket.toml`

<!-- ticket-index:entry id=185419e0-bea4-4c7b-abda-1e92193f32e7 slug=done/unspecified digest=e9316c86a556 -->
#### [185419e0] [ticket-api] Allow bidirectional ticket state transitions by default
- summary: Ticket state transitions should work in both directions by default using the same state transition interface. We should not require every schema to spell out reverse edges when the validator can trea...
- ref: `memory-api/.ticket/tickets/185419e0-bea4-4c7b-abda-1e92193f32e7/ticket.toml`

<!-- ticket-index:entry id=835b332b-8c1f-4423-bf21-4fac7aa7c8f7 slug=done/unspecified digest=9de011370ebf -->
#### [835b332b] [ticket-api] Health + state-guard: stop warning on ready→ready; flag/guard only tickets progressed ahead of their dependencies
- summary: The health check emits `unblocked_with_deps` (info) whenever a `ready` ticket has non-terminal dependencies. But `ready` does not mean "unblocked" — it means "groomed". A `ready → ready` dependency c...
- ref: `memory-api/.ticket/tickets/835b332b-8c1f-4423-bf21-4fac7aa7c8f7/ticket.toml`

<!-- ticket-index:entry id=ca7e0388-ee5c-41c7-8385-7de6817ce261 slug=done/unspecified digest=128a3a8a6b65 -->
#### [ca7e0388] [ticket-api] Lease-release capability — owner/stale release semantics + transport parity + orphaned-lease cleanup
- summary: `TicketStore::release_lease(ticket_id, requester)` added in memory-api/crates/ticket-api/src/storage/store/board.rs with the semantics:
- ref: `memory-api/.ticket/tickets/ca7e0388-ee5c-41c7-8385-7de6817ce261/ticket.toml`

<!-- ticket-index:entry id=c680b137-01a1-4d13-b219-39af86eaa71b slug=done/unspecified digest=eae0ece43162 -->
#### [c680b137] [ticket-api] Promote health_check to a ticket-api call with CLI/MCP/HTTP transport parity
- summary: The original premise ("health_check has no CLI parity") is outdated. `ticket-api::health::collect_findings` already exists and a `ticket health` CLI subcommand, MCP `health_check`, and HTTP `/api/gra...
- ref: `memory-api/.ticket/tickets/c680b137-01a1-4d13-b219-39af86eaa71b/ticket.toml`

<!-- ticket-index:entry id=161454bd-af80-4f21-a491-be41d3ed196c slug=done/unspecified digest=38da656e0f48 -->
#### [161454bd] [ticket-api] Reject ticket creation when `type` has no registered schema
- summary: Ticket creation (`TicketStore::create`) previously validated against the type's schema only when one was registered, silently persisting tickets whose `type` had no schema. The failure only surfaced ...
- ref: `.ticket/tickets/161454bd-af80-4f21-a491-be41d3ed196c/ticket.toml`

<!-- ticket-index:entry id=d5771b88-ca1d-41b2-8b59-0c911a34b37f slug=done/unspecified digest=b9d40f82f7e2 -->
#### [d5771b88] [ticket-api] Repair move-planner invisible-reference fixture visibility
- summary: `storage::move_planner::tests::preflight_reports_invisible_reference_visibility_and_path_refs` failed during setup with `StorageError::NotFound` for its target-only fixture ticket UUID. The fixture a...
- ref: `memory-api/.ticket/tickets/d5771b88-ca1d-41b2-8b59-0c911a34b37f/ticket.toml`

<!-- ticket-index:entry id=16d8aed9-cc29-4820-bfc7-4ae2f202f262 slug=done/unspecified digest=38d455667452 -->
#### [16d8aed9] [ticket-api][ticket-cli] Auto-walk allowed transition paths by default; make strict single-hop an opt-out flag
- summary: Review finding on fdf53556 (FUP-ERR). The reviewer rejected the strict single-hop default: FUP-ERR made `update --to-state` block skipped-waypoint transitions (e.g. `new -> in-implementation`) with a...
- ref: `.ticket/tickets/16d8aed9-cc29-4820-bfc7-4ae2f202f262/ticket.toml`

<!-- ticket-index:entry id=fdf53556-2d84-4964-9575-f40032a02e85 slug=done/unspecified digest=aee3fbe66779 -->
#### [fdf53556] [ticket-api][ticket-cli][ticket-mcp] Ticket state-transition recovery contract: report current + allowed next states, intermediate states, HTTP parity, inspection command
- summary: Carved from the **original scope** of 8bb97b73. That parent session delivered and validated only the **session-mcp** enum-rejection slice; its REVIEWER NOTE flags the ticket-transition scope as NOT d...
- ref: `.ticket/tickets/fdf53556-2d84-4964-9575-f40032a02e85/ticket.toml`

<!-- ticket-index:entry id=dda04f91-3e03-46bb-a553-d9a172139027 slug=done/unspecified digest=8d592f5234f9 -->
#### [dda04f91] [ticket-api][watcher] Add reconcile lifecycle tracing
- summary: Starting watcher reconcile tracing implementation: instrument reconcile_once and run_watch_loop with lifecycle spans/events and focused validation.
- ref: `.ticket/tickets/dda04f91-3e03-46bb-a553-d9a172139027/ticket.toml`

<!-- ticket-index:entry id=07836f41-7fa5-4e41-8411-1c7cf8aeee75 slug=done/unspecified digest=c9e1a27aa724 -->
#### [07836f41] [ticket-cli] Make get/search/list workspace-aware across nested roots
- summary: `ticket get <id>` was not workspace-aware and failed with a raw path error when the ticket lived under a different ticket root.
- ref: `memory-api/.ticket/tickets/07836f41-7fa5-4e41-8411-1c7cf8aeee75/ticket.toml`

<!-- ticket-index:entry id=68a08b34-000b-4585-8354-4b1a26a15f4b slug=done/unspecified digest=acad30e56d98 -->
#### [68a08b34] [ticket-cli] Scope-aware board and next for multi-root workspaces
- summary: `ticket board show` and `ticket next` are not scope-aware enough for multi-root repositories.
- ref: `.ticket/tickets/68a08b34-000b-4585-8354-4b1a26a15f4b/ticket.toml`

<!-- ticket-index:entry id=8a3ad90a-eaf3-4638-ad85-51c98549f581 slug=done/unspecified digest=3134224f6b38 -->
#### [8a3ad90a] [ticket-cli][ticket-api] Safe dangling-edge remediation workflow
- priority: `high`
- summary: Add a safer, first-class workflow to remediate dangling ticket edges reported by `ticket health`.
- ref: `.ticket/tickets/8a3ad90a-eaf3-4638-ad85-51c98549f581/ticket.toml`

<!-- ticket-index:entry id=8bb97b73-9dbc-43ee-9939-46b3ddf2612f slug=done/unspecified digest=bf19f3fc305e -->
#### [8bb97b73] [ticket-cli][ticket-mcp][session-mcp] Explain invalid state/enum transitions with allowed values
- summary: Invalid transitions and enum-valued parameters are enforced, but they are not explained
- ref: `.ticket/tickets/8bb97b73-9dbc-43ee-9939-46b3ddf2612f/ticket.toml`

<!-- ticket-index:entry id=02723a9b-23ff-47b1-8306-0480be087ddd slug=done/unspecified digest=52f036ff4924 -->
#### [02723a9b] [ticket-cli][ticket-viewer] Fix nested-workspace discovery and stale list races
- summary: Two ticket discovery paths are still unreliable for nested child workspaces.
- ref: `.ticket/tickets/02723a9b-23ff-47b1-8306-0480be087ddd/ticket.toml`

<!-- ticket-index:entry id=2ffd479a-ca4b-4265-a1c5-f0081b2e531e slug=done/unspecified digest=2f5860f3a95f -->
#### [2ffd479a] [ticket-mcp] Canonicalize workspace resolution across all tools
- summary: `ticket-mcp` accepted an explicit checkout path but did not refresh its root workspace policy at startup. A root `.ticket` index could therefore omit child stores that its `workspace-policy.toml` exp...
- ref: `memory-api/.ticket/tickets/2ffd479a-ca4b-4265-a1c5-f0081b2e531e/ticket.toml`

<!-- ticket-index:entry id=7d857543-be6b-42ba-8b7e-608b5bd7c046 slug=done/unspecified digest=3d8b86f27b9e -->
#### [7d857543] [ticket-mcp][spec-mcp][rule-api] Self-describing capability catalog for ticket/spec/rule surfaces (CLI + MCP parity)
- summary: Carved from the **original scope** of 5ad77aba. That parent session delivered and validated only the **session-mcp** `session_capabilities` slice; its REVIEWER NOTE flags the broader ticket/spec/rule...
- ref: `.ticket/tickets/7d857543-be6b-42ba-8b7e-608b5bd7c046/ticket.toml`

<!-- ticket-index:entry id=5ad77aba-c7f7-4058-854e-dd0412746c7c slug=done/unspecified digest=5c8819771d15 -->
#### [5ad77aba] [ticket-mcp][spec-mcp][rule-api][session-mcp] Add self-describing capability catalog and help surfaces
- summary: The ticket/spec/rule/session tool surfaces are not self-describing enough for operators or agents.
- ref: `.ticket/tickets/5ad77aba-c7f7-4058-854e-dd0412746c7c/ticket.toml`

<!-- ticket-index:entry id=d1770bd5-dc7e-42ca-a5d0-2bc0cbc91110 slug=done/unspecified digest=b3cbba43ab47 -->
#### [d1770bd5] [ticket-store] Relocate misplaced ticket and spec directories
- summary: Generated: 2026-06-28
- ref: `.ticket/tickets/d1770bd5-dc7e-42ca-a5d0-2bc0cbc91110/ticket.toml`

<!-- ticket-index:entry id=5d5c7bbb-fac2-49ba-aa19-37bf6e2aac34 slug=done/unspecified digest=0dfdb9071e20 -->
#### [5d5c7bbb] [ticket-viewer] Add cache invalidation for graph layout on ticket updates
- summary: Depends on: [111510f4 Fix graph reactivity: ticket state changes don't update graph nodes](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/111510f4-c74b-4819-800b...
- ref: `.ticket/tickets/5d5c7bbb-fac2-49ba-aa19-37bf6e2aac34/ticket.toml`

<!-- ticket-index:entry id=884ad295-9b75-4ad6-938d-6ab73c8efa6b slug=done/unspecified digest=7f6c90917380 -->
#### [884ad295] [ticket-viewer] Avoid click panic when backend is offline
- ref: `.ticket/tickets/884ad295-9b75-4ad6-938d-6ab73c8efa6b/ticket.toml`

<!-- ticket-index:entry id=75fde4f5-ca1c-4bcf-9530-36a3da59a8f1 slug=done/unspecified digest=fc0c39020661 -->
#### [75fde4f5] [ticket-viewer] Targeted node update on ticket.upsert + fix invalidate_workspace version no-op
- summary: Make `ticket.upsert` update node visual state without a full workspace layout cache wipe, and fix the version-signal no-op so the intended reactive update actually fires.
- ref: `.ticket/tickets/75fde4f5-ca1c-4bcf-9530-36a3da59a8f1/ticket.toml`

<!-- ticket-index:entry id=fe7effea-6b70-4b16-8c00-bc7e910a0fde slug=done/unspecified digest=78379bbf0b86 -->
#### [fe7effea] [ticket-viewer] Test graph reactivity with ticket state updates
- summary: Depends on: [111510f4 Fix graph reactivity: ticket state changes don't update graph nodes](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/111510f4-c74b-4819-800b...
- ref: `.ticket/tickets/fe7effea-6b70-4b16-8c00-bc7e910a0fde/ticket.toml`

<!-- ticket-index:entry id=1e119a99-375a-479b-80ce-98cb63d92436 slug=done/unspecified digest=523ec70aa5ee -->
#### [1e119a99] [ticket-viewer] Update graph SSE subscription to listen for ticket.upsert events
- summary: Depends on: [111510f4 Fix graph reactivity: ticket state changes don't update graph nodes](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/111510f4-c74b-4819-800b...
- ref: `.ticket/tickets/1e119a99-375a-479b-80ce-98cb63d92436/ticket.toml`

<!-- ticket-index:entry id=aac045c2-9455-4338-9942-32466ff2e6b1 slug=done/unspecified digest=5ef141e84dd8 -->
#### [aac045c2] [ticket-viewer][viewer-api] Fix graph node-card lag during camera/drag (per-frame DOM thrashing)
- summary: Eliminate the per-frame inefficiency that makes DOM node cards visibly lag behind the GPU-drawn edge endpoints while orbiting/panning the camera or dragging nodes in the WebGPU graph. Reactivity must...
- ref: `.ticket/tickets/aac045c2-9455-4338-9942-32466ff2e6b1/ticket.toml`

<!-- ticket-index:entry id=46d16755-309b-479f-aab2-624c3fa7ce9b slug=done/unspecified digest=17de0c602375 -->
#### [46d16755] [ticket-vscode] Fix canonical workspace selection when server exposes path-based or shared workspace ids
- summary: Implemented and validated canonical workspace resolution for ticket-vscode. The extension now maps detected local .ticket roots to canonical server workspace ids by label/path and otherwise prefers a...
- ref: `.ticket/tickets/46d16755-309b-479f-aab2-624c3fa7ce9b/ticket.toml`

<!-- ticket-index:entry id=e83264db-e634-4c7c-811d-4413a1e3416a slug=done/unspecified digest=feb03668e428 -->
#### [e83264db] [ticket-vscode] Prevent aborted list tickets request after server start
- priority: `high`
- ref: `memory-api/.ticket/tickets/e83264db-e634-4c7c-811d-4413a1e3416a/ticket.toml`

<!-- ticket-index:entry id=20b6a09a-080a-480b-8f09-79cbf7bc20bd slug=done/unspecified digest=1930b9ca83ed -->
#### [20b6a09a] [token-efficiency] Omit default workspace and schema from ticket outputs
- priority: `high`
- summary: Implemented shared ticket output normalization to omit default workspace/schema metadata across CLI and MCP responses; validated with focused ticket-api, ticket-cli, and ticket-mcp tests.
- ref: `.ticket/tickets/20b6a09a-080a-480b-8f09-79cbf7bc20bd/ticket.toml`

<!-- ticket-index:entry id=76941e78-f812-440c-9fbc-04d3bb88f11a slug=done/unspecified digest=04add9174810 -->
#### [76941e78] [tool-metrics][T1] Probe: establish which source actually carries tool output size
- summary: Tool output size is the one number the entire graded-cost design depends on (`graded-cost-scale.md`: "LINEAR map of empirical `est-output-tokens`"), and it has never been captured. The raw Copilot tr...
- ref: `.ticket/tickets/76941e78-f812-440c-9fbc-04d3bb88f11a/ticket.toml`

<!-- ticket-index:entry id=44119807-53af-41b0-920a-ffbc985d425d slug=done/unspecified digest=c5e4616e510a -->
#### [44119807] [tool-metrics][T2] Capture tool output size at capture time with per-call source attribution
- summary: Review verdict (3rd pass, 2026-07-30): AC1 conditional pass (independently confirmed real evidence, but coverage measured at only ~24%, 7/29 real calls), AC2 pass, AC3 pass, AC4 pass — but overall FA...
- ref: `.ticket/tickets/44119807-53af-41b0-920a-ffbc985d425d/ticket.toml`

<!-- ticket-index:entry id=ce7b7bde-fa29-4ad0-9524-70629dcddd19 slug=done/unspecified digest=88d0d914c27c -->
#### [ce7b7bde] [tool-metrics][T4] Recurrence guardrail: data-capture ACs must be verified by artifact read-back
- summary: Three independent post-mortem agents (review, roast, session forensics) converged on one recurrence mechanism: **proxy evidence was accepted in place of outcome evidence**, repeatedly, for months.
- ref: `.ticket/tickets/ce7b7bde-fa29-4ad0-9524-70629dcddd19/ticket.toml`

<!-- ticket-index:entry id=6dc44fbb-4480-4bad-853c-79b8171dd73b slug=done/unspecified digest=79dca26288dd -->
#### [6dc44fbb] [viewer-api] Anchor SVG edge endpoints to world_to_screen instead of getBoundingClientRect
- summary: Decouple SVG edge-overlay geometry from the DOM layout pass so edges and node cards are positioned from the same projection source.
- ref: `.ticket/tickets/6dc44fbb-4480-4bad-853c-79b8171dd73b/ticket.toml`

<!-- ticket-index:entry id=c79e2630-3d49-454b-998f-fb52c24303f4 slug=done/unspecified digest=877e2e9b1fa4 -->
#### [c79e2630] [viewer-api] Default visual validation to external fullscreen Chromium
- ref: `.ticket/tickets/c79e2630-3d49-454b-998f-fb52c24303f4/ticket.toml`

<!-- ticket-index:entry id=f685dca9-1a67-4b0b-bc14-a88d6ef1226d slug=done/unspecified digest=c4519b13925d -->
#### [f685dca9] [viewer-api] Guard sync_render_state against resetting layout during active interaction
- summary: Prevent reactive re-renders (e.g. SSE `ticket.upsert`, hover changes) from resetting an in-progress drag or camera interaction.
- ref: `.ticket/tickets/f685dca9-1a67-4b0b-bc14-a88d6ef1226d/ticket.toml`

<!-- ticket-index:entry id=97a9ed0b-4442-4514-8c67-09e3393f79a7 slug=done/unspecified digest=a8f4305f7e0b -->
#### [97a9ed0b] [viewer-api] render_frame: compute VP once and collect node rects once per frame
- summary: Remove the redundant per-frame work in `render_frame` that thrashes layout and inflates the rAF callback, causing DOM node cards to lag behind GPU edges.
- ref: `.ticket/tickets/97a9ed0b-4442-4514-8c67-09e3393f79a7/ticket.toml`

<!-- ticket-index:entry id=b005a8fe-9971-4e59-a3df-171f81b6d3f7 slug=done/unspecified digest=36920ecff534 -->
#### [b005a8fe] [viewer-api][P1] Visual primitives: Breadcrumbs, Overlay/Modal, MetaHeader+Chip, Card/CardGrid
- summary: Add foundational visual primitives to `viewer-api-dioxus` that doc-viewer relies on but spec-viewer currently lacks.
- ref: `viewer-api/.ticket/tickets/b005a8fe-9971-4e59-a3df-171f81b6d3f7/ticket.toml`

<!-- ticket-index:entry id=da16dada-e245-4fdd-868a-c3691e6c351a slug=done/unspecified digest=87af4fb5e6b6 -->
#### [da16dada] [viewer-api][P2] State containers: TabsStore, PathCodec/url_path, Prefetcher
- summary: Add reusable state containers to `viewer-api-dioxus` for cross-cutting viewer concerns.
- ref: `viewer-api/.ticket/tickets/da16dada-e245-4fdd-868a-c3691e6c351a/ticket.toml`

<!-- ticket-index:entry id=8bf5edd2-4fe6-4580-ac87-73843f0206f0 slug=done/unspecified digest=8880a35b6c56 -->
#### [8bf5edd2] [viewer-api][P3] Widget extensions: TreeNode rich tooltip, mobile sidebar audit, HeaderActions
- summary: Extend the existing shared widgets with capabilities doc-viewer uses but Dioxus viewers cannot express today.
- ref: `viewer-api/.ticket/tickets/8bf5edd2-4fe6-4580-ac87-73843f0206f0/ticket.toml`

<!-- ticket-index:entry id=b4127011-4e08-47bc-ac73-3d3761f29587 slug=done/unspecified digest=6537b72f151f -->
#### [b4127011] [viewer-api][P4] FilterPanel shell with JQ presets and results list
- summary: Port doc-viewer's filter panel (basic dropdown filters + JQ presets + custom JQ input + results list) to a generic Dioxus shell.
- ref: `viewer-api/.ticket/tickets/b4127011-4e08-47bc-ac73-3d3761f29587/ticket.toml`

<!-- ticket-index:entry id=8d6895a5-dce8-47c1-98ce-212fd0ae2e08 slug=done/unspecified digest=3adaac4517c2 -->
#### [8d6895a5] [viewer-api][audit] Fix viewer-api-dioxus compile failure and restore llvm-cov coverage collection
- ref: `.ticket/tickets/8d6895a5-dce8-47c1-98ce-212fd0ae2e08/ticket.toml`

<!-- ticket-index:entry id=53b14bf8-3243-4cd6-909e-17c431812441 slug=done/unspecified digest=1fa590c99f59 -->
#### [53b14bf8] [workflow-tools] Rename reference CLI binary to bare domain name
- ref: `.ticket/tickets/53b14bf8-3243-4cd6-909e-17c431812441/ticket.toml`

<!-- ticket-index:entry id=618f6ce4-e7b3-48f2-9c9e-840247a119da slug=done/unspecified digest=53fddfb5b258 -->
#### [618f6ce4] [workflow] Bootstrap doc-api, test-api, and log-api evidence stores for spec fulfillment
- summary: Coordinate the bounded evidence-store bootstrap across `doc-api`, `test-api`, and `log-api` so spec acceptance clauses can resolve authoritative documentation, validation, and log evidence without wr...
- ref: `.ticket/tickets/618f6ce4-e7b3-48f2-9c9e-840247a119da/ticket.toml`

<!-- ticket-index:entry id=bc19467f-b4d4-48c3-be92-b551d4fe6679 slug=done/unspecified digest=b1d9a098598f -->
#### [bc19467f] [workflow] Expectation-oriented spec contract rollout
- summary: Coordinate the rollout that redefines specifications around intended properties, acceptance criteria, and store-owned evidence while migrating the affected specs and tickets homogeneously.
- ref: `.ticket/tickets/bc19467f-b4d4-48c3-be92-b551d4fe6679/ticket.toml`

<!-- ticket-index:entry id=577df498-d468-448f-afc1-3e35e48e5f12 slug=done/unspecified digest=996613c3c179 -->
#### [577df498] [workflow] Homogeneously migrate remaining expectation-oriented specs and tickets
- summary: Homogeneously migrate the remaining affected expectation-oriented specs and tickets after the pilot proves the contract.
- ref: `.ticket/tickets/577df498-d468-448f-afc1-3e35e48e5f12/ticket.toml`

<!-- ticket-index:entry id=6e5306fb-c1b3-4aec-991d-fabaf3096e23 slug=done/unspecified digest=e21c4c77192a -->
#### [6e5306fb] [workflow] Pilot expectation-oriented spec contract on one workflow spec and one README-rollout spec
- summary: Pilot the expectation-oriented spec contract on one workflow spec and one README-rollout spec.
- ref: `.ticket/tickets/6e5306fb-c1b3-4aec-991d-fabaf3096e23/ticket.toml`

<!-- ticket-index:entry id=aaa90ee6-1358-41ad-b19e-61abdc3f1dc2 slug=done/unspecified digest=c3f6d403847b -->
#### [aaa90ee6] [workflow] Store-owned spec evidence integration
- summary: Integrate store-owned acceptance evidence so specs can be satisfied or blocked by native documentation, validation, and log records rather than by wrapper-owned artifact payloads.
- ref: `.ticket/tickets/aaa90ee6-1358-41ad-b19e-61abdc3f1dc2/ticket.toml`

<!-- ticket-index:entry id=51d53f8f-5636-4185-b629-806816059caa slug=done/unspecified digest=9cab133f7e06 -->
#### [51d53f8f] [workspace-policy] 1/6 Policy file parser + in-memory WorkspacePolicy object
- priority: `high`
- summary: Add a `WorkspacePolicy` type parsed from `.ticket/workspace-policy.toml`, with documented defaults and an absent-file compatibility mode.
- ref: `.ticket/tickets/51d53f8f-5636-4185-b629-806816059caa/ticket.toml`

<!-- ticket-index:entry id=6312c5c4-858a-4953-98cf-2f1258de2093 slug=done/unspecified digest=f98b0e13fedf -->
#### [6312c5c4] [workspace-policy] 2/6 Apply policy in discover_workspace_scan_roots
- priority: `high`
- summary: Make `discover_workspace_scan_roots` policy-aware so unwanted workspaces are never added as scan roots.
- ref: `.ticket/tickets/6312c5c4-858a-4953-98cf-2f1258de2093/ticket.toml`

<!-- ticket-index:entry id=eecbcee9-d82d-469f-b728-56f7e5598006 slug=done/unspecified digest=7c5f2a0cde6c -->
#### [eecbcee9] [workspace-policy] 3/6 Scan-root policy metadata + scan-time enforcement
- priority: `high`
- summary: Extend scan-root persistence with auditability metadata and enforce policy at scan time so roots not allowed by policy are skipped.
- ref: `.ticket/tickets/eecbcee9-d82d-469f-b728-56f7e5598006/ticket.toml`

<!-- ticket-index:entry id=42094bd4-f818-460e-b48f-b1be3bcc1c80 slug=done/unspecified digest=465aa2704062 -->
#### [42094bd4] [workspace-policy] 4/6 Query-time final guard for policy-allowed roots
- priority: `high`
- summary: Harden the existing query visibility guard so it only surfaces tickets under active, policy-allowed roots — the final defense even if stale/ignored roots exist in the index.
- ref: `.ticket/tickets/42094bd4-f818-460e-b48f-b1be3bcc1c80/ticket.toml`

<!-- ticket-index:entry id=c5ff717e-8b7e-480c-ab9a-fda0527d6966 slug=done/unspecified digest=d22d938067e7 -->
#### [c5ff717e] [workspace-policy] 5/6 CLI/API surfaces for policy management
- priority: `medium`
- summary: Expose commands to inspect and edit the workspace policy and to rescan with policy applied.
- ref: `.ticket/tickets/c5ff717e-8b7e-480c-ab9a-fda0527d6966/ticket.toml`

<!-- ticket-index:entry id=25677720-ec54-4f95-915d-0049a67942cb slug=done/unspecified digest=e478fc11ae41 -->
#### [25677720] [workspace-policy] 6/6 Regression tests across discovery/scan/query
- priority: `high`
- summary: Prove the policy contract end to end with focused regression tests.
- ref: `.ticket/tickets/25677720-ec54-4f95-915d-0049a67942cb/ticket.toml`

<!-- ticket-index:entry id=528af270-a65c-4311-88ed-7a2e87d468bf slug=done/unspecified digest=a2dc8c24bb2e -->
#### [528af270] [workspace-policy] 7/6 Cleanup retro fixture references + audit boundary rule
- priority: `high`
- summary: Workspace-policy rollout should prevent future indexing/visibility of excluded workspaces, but the store currently contains historical "retro" tickets and cross-workspace edges that reference fixture...
- ref: `.ticket/tickets/528af270-a65c-4311-88ed-7a2e87d468bf/ticket.toml`

<!-- ticket-index:entry id=65d5885b-ec09-450e-b6c8-1607ec3e51c3 slug=done/unspecified digest=3c4e5c4f024b -->
#### [65d5885b] [workspace-policy] Explicit workspace-policy layer for ticket scan-root discovery
- priority: `high`
- summary: There is no explicit workspace-policy layer today — only discovery behavior plus scan roots. Because of this, indexed tickets can leak when discovery or stored roots pull in workspaces that were not ...
- ref: `.ticket/tickets/65d5885b-ec09-450e-b6c8-1607ec3e51c3/ticket.toml`

<!-- ticket-index:entry id=25780944-a784-4373-8991-88c2902b1556 slug=done/unspecified digest=945ad1d3d81f -->
#### [25780944] mcp-toolmon: rename mcp-cost-gate, add pluggable policy + live binary reload
- summary: MCP servers run from prebuilt binaries in `~/.cargo/bin/`. After a source fix + rebuild, the running server keeps serving stale code, with no reload path short of a full VS Code MCP restart. Worse, o...
- ref: `.ticket/tickets/25780944-a784-4373-8991-88c2902b1556/ticket.toml`

<!-- ticket-index:entry id=e14f893f-042d-45cc-b748-f48860a640c5 slug=done/unspecified digest=ae374d0e92eb -->
#### [e14f893f] ticket-viewer WgpuOverlay panics with `unreachable` when overlay enabled (default-ON regression)
- priority: `high`
- summary: Surfaced by the e2e test
- ref: `memory-viewers/.ticket/tickets/e14f893f-042d-45cc-b748-f48860a640c5/ticket.toml`

<!-- ticket-index:entry id=5bf1951a-dce4-4efb-80d6-89fe4fa01573 slug=done/unspecified digest=f0c8645afd53 -->
#### [5bf1951a] ticket-vscode: Fix tree view state grouping — show only same-state tickets per folder with dependency hierarchy
- priority: `high`
- summary: The current `buildStateGroups()` in `ticketProvider.ts` extends each state folder
- ref: `memory-api/.ticket/tickets/5bf1951a-dce4-4efb-80d6-89fe4fa01573/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=1789cdfa-cd7e-45c4-a683-815b80c39970 slug=done/viewer-api digest=10b45771de29 -->
#### [1789cdfa] Feature: Extract WgpuOverlay/effects to viewer-api and GPU dependency graph in ticket-viewer
- summary: Extract the entire GPU rendering pipeline from log-viewer into viewer-api as shared infrastructure, then use it to build a GPU-rendered dependency graph in ticket-viewer.
- ref: `viewer-api/.ticket/tickets/1789cdfa-cd7e-45c4-a683-815b80c39970/ticket.toml`

<!-- ticket-index:entry id=d7971816-6f84-419e-abd8-0f84d5f7b82f slug=done/viewer-api digest=da10da4406cd -->
#### [d7971816] Feature: Sortable FileTree with generic sorting header
- summary: Add a generic sorting header to the shared `FileTree` component in `viewer-api/frontend` and integrate it in ticket-viewer.
- ref: `viewer-api/.ticket/tickets/d7971816-6f84-419e-abd8-0f84d5f7b82f/ticket.toml`

<!-- ticket-index:entry id=c826869a-d40c-425d-ba5c-4003c222cfde slug=done/viewer-api digest=330d33186d86 -->
#### [c826869a] Impl: Extract generic Graph3DView component to viewer-api
- summary: Extract a fully self-contained `Graph3DView` component into `viewer-api/frontend`. This is not a thin wrapper — it owns camera, layout, interaction, animation, and GPU rendering. Log-viewer only adds...
- ref: `viewer-api/.ticket/tickets/c826869a-d40c-425d-ba5c-4003c222cfde/ticket.toml`

<!-- ticket-index:entry id=b3d250d5-dd28-44e6-aaf4-47bee9dea56e slug=done/viewer-api digest=5f5c3006b215 -->
#### [b3d250d5] Impl: Move WgpuOverlay + shaders + effects to viewer-api
- summary: Move the WgpuOverlay component, WGSL shaders, effects, and 3D math utilities from `log-viewer/frontend/src/` to `viewer-api/frontend/src/` so they become shared infrastructure. Also moves the SVG Gra...
- ref: `viewer-api/.ticket/tickets/b3d250d5-dd28-44e6-aaf4-47bee9dea56e/ticket.toml`

<!-- ticket-index:entry id=a1259318-f992-44e3-9cdf-0ea4c224f6f3 slug=done/viewer-api digest=6f0a801db490 -->
#### [a1259318] Impl: viewer-api extraction for shared tree/file/graph server primitives
- summary: Wave 1 / Track E** | Component: `viewer-api`
- ref: `viewer-api/.ticket/tickets/a1259318-f992-44e3-9cdf-0ea4c224f6f3/ticket.toml`

<!-- ticket-index:entry id=a39f7805-c6bd-47d3-9b1b-fa29215bdf9e slug=done/viewer-api digest=7bcc1a25c09d -->
#### [a39f7805] Plan: Graph edge visual polish -- lighting, focus colors, and particles
- priority: `medium`
- summary: tags: `#plan` `#viewer-api` `#graph` `#ux` `#rendering` `#webgpu` `#lighting` `#particles`
- ref: `viewer-api/.ticket/tickets/a39f7805-c6bd-47d3-9b1b-fa29215bdf9e/ticket.toml`

<!-- ticket-index:entry id=21a2e8f4-4bd8-4436-be52-c2c4a07bb692 slug=done/viewer-api digest=38f175370989 -->
#### [21a2e8f4] [viewer-api] Adopt rich tree tooltips in Dioxus spec-viewer and doc-viewer
- priority: `high`
- summary: Adopt the existing shared `TreeNode::tooltip_render` capability in current Dioxus tree consumers so the shared tree surface actually exposes richer doc-viewer-style metadata.
- ref: `viewer-api/.ticket/tickets/21a2e8f4-4bd8-4436-be52-c2c4a07bb692/ticket.toml`

<!-- ticket-index:entry id=4d9293ab-b7a8-4113-b80a-bfe39297bad2 slug=done/viewer-api digest=4872f984cbfc -->
#### [4d9293ab] [viewer-api] Adopt shared TabsStore in Dioxus doc-viewer
- priority: `high`
- summary: Replace the Dioxus doc-viewer's ad-hoc tab-state signals with the existing shared `viewer_api_dioxus::TabsStore<OpenArtifactTab>` so the frontend actually consumes the tab-state primitive that alread...
- ref: `viewer-api/.ticket/tickets/4d9293ab-b7a8-4113-b80a-bfe39297bad2/ticket.toml`

<!-- ticket-index:entry id=c6bf5b7a-f822-44bb-8d2b-86c966031ca6 slug=done/viewer-api digest=d26b0019e0ce -->
#### [c6bf5b7a] [viewer-api] Enlarge Graph3D directed edge arrow tips
- priority: `medium`
- summary: The shared Graph3D edge overlay in `viewer-api/viewer-api/frontend/dioxus/src/graph3d/mod.rs` renders directed-edge arrow markers that are too small to read comfortably in ticket-viewer and other vie...
- ref: `.ticket/tickets/c6bf5b7a-f822-44bb-8d2b-86c966031ca6/ticket.toml`

<!-- ticket-index:entry id=763f8c13-a4bd-47af-8894-3e95a63fde8d slug=done/viewer-api digest=5a9ec90f971a -->
#### [763f8c13] [viewer-api] Extract a reusable Dioxus explorer shell around FileTree
- priority: `high`
- summary: Extract a reusable Dioxus explorer shell around FileTree so viewers stop duplicating sidebar search and tree-control chrome.
- ref: `viewer-api/.ticket/tickets/763f8c13-a4bd-47af-8894-3e95a63fde8d/ticket.toml`

<!-- ticket-index:entry id=735502cd-3aec-4772-b2a8-2184aaaf3c21 slug=done/viewer-api digest=180ed67ae868 -->
#### [735502cd] [viewer-api] Extract a reusable interactive chip button for Dioxus explorer filters
- priority: `high`
- summary: Extract a shared clickable chip button in viewer-api so explorer filter/state toggles stop duplicating button markup and state wiring across FileTree and ticket-viewer.
- ref: `viewer-api/.ticket/tickets/735502cd-3aec-4772-b2a8-2184aaaf3c21/ticket.toml`

<!-- ticket-index:entry id=fecbd4d8-b863-4821-bd7d-d6bd16f9356c slug=done/viewer-api digest=ae78a54bec5b -->
#### [fecbd4d8] [viewer-api] Preserve frontend build diagnostics in viewer-ctl failures
- priority: `medium`
- summary: `viewer-ctl prepare <viewer>` shells out to `trunk` and other frontend build
- ref: `viewer-api/.ticket/tickets/fecbd4d8-b863-4821-bd7d-d6bd16f9356c/ticket.toml`

<!-- ticket-index:entry id=9a81d3e5-82ca-4fd0-84bf-c0a54f6716e5 slug=done/viewer-api digest=08e794518f6f -->
#### [9a81d3e5] [viewer-api] Reuse the shared toggle contract for Dioxus explorer sort controls
- priority: `high`
- summary: Reuse the shared toggle button contract for FileTree sort controls so explorer sort rows stop duplicating active/inactive button markup.
- ref: `viewer-api/.ticket/tickets/9a81d3e5-82ca-4fd0-84bf-c0a54f6716e5/ticket.toml`

<!-- ticket-index:entry id=379ac56a-4580-4ed6-a571-eb76282ef375 slug=done/viewer-api digest=71fd28fde1ba -->
#### [379ac56a] [viewer-api][bug] Standalone manifest cannot resolve context-api
- priority: `high`
- summary: The viewer-api remote at SHA `52456b47` declares an in-tree dependency in `viewer-api/Cargo.toml` (local superproject path: `viewer-api/viewer-api/Cargo.toml`):
- ref: `.ticket/tickets/379ac56a-4580-4ed6-a571-eb76282ef375/ticket.toml`

<!-- ticket-index:entry id=322ba030-160c-41d3-8a12-42936ae92858 slug=done/viewer-api digest=d6cfcdcf59a2 -->
#### [322ba030] [viewer-api][ticket-viewer] Add multi-level graph node detail rendering
- priority: `high`
- summary: Introduce multiple graph node detail levels so zoomed-out views stay legible and zoomed-in views can show rich ticket content.
- ref: `memory-viewers/.ticket/tickets/322ba030-160c-41d3-8a12-42936ae92858/ticket.toml`

<!-- ticket-index:entry id=6ccbe0b7-0b6c-44fc-989b-db66c963b623 slug=done/viewer-api digest=52c85875dce8 -->
#### [6ccbe0b7] [viewer-api][ticket-viewer] Fix FileTree left-panel formatting and resize/render performance
- priority: `high`
- summary: W3. The left file-tree panel is badly formatted and expensive to render; widening the panel is sluggish and laggy.
- ref: `memory-viewers/.ticket/tickets/6ccbe0b7-0b6c-44fc-989b-db66c963b623/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=7346feae-045f-4da9-bf1c-47535132ffa1 slug=done/viewer-api-dioxus digest=c1837c005ad6 -->
#### [7346feae] Arch: viewer-api-dioxus crate scaffold and build system
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/7346feae-045f-4da9-bf1c-47535132ffa1/ticket.toml`

<!-- ticket-index:entry id=512986e0-9f0e-483c-8201-5c316bffdeb2 slug=done/viewer-api-dioxus digest=7373f823202b -->
#### [512986e0] Feature: Theme settings UI panel with live preview
- priority: `high`
- ref: `viewer-api/.ticket/tickets/512986e0-9f0e-483c-8201-5c316bffdeb2/ticket.toml`

<!-- ticket-index:entry id=2405a83e-e3b5-47ad-8d88-8c12f507d252 slug=done/viewer-api-dioxus digest=d5770c5b8f2a -->
#### [2405a83e] Port: CSS stylesheets — base, layout, buttons, tabs, tree, code-viewer
- priority: `high`
- ref: `viewer-api/.ticket/tickets/2405a83e-e3b5-47ad-8d88-8c12f507d252/ticket.toml`

<!-- ticket-index:entry id=7330aa36-102d-452c-b61d-6f4c8651b422 slug=done/viewer-api-dioxus digest=5da73ad78664 -->
#### [7330aa36] Port: CodeViewer and FileContentViewer
- priority: `high`
- ref: `viewer-api/.ticket/tickets/7330aa36-102d-452c-b61d-6f4c8651b422/ticket.toml`

<!-- ticket-index:entry id=b3f9878d-5839-4a87-989c-aa3101ee38aa slug=done/viewer-api-dioxus digest=58dac46016b5 -->
#### [b3f9878d] Port: Layout components — Header, Layout, Sidebar, Panel, GlassPanel
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/b3f9878d-5839-4a87-989c-aa3101ee38aa/ticket.toml`

<!-- ticket-index:entry id=9dec4f23-4e92-4c14-b085-b9f625589228 slug=done/viewer-api-dioxus digest=a4e93f15e615 -->
#### [9dec4f23] Port: ResizeHandle with rAF-batched drag
- priority: `high`
- ref: `viewer-api/.ticket/tickets/9dec4f23-4e92-4c14-b085-b9f625589228/ticket.toml`

<!-- ticket-index:entry id=11f77899-6def-4140-b6bf-e84035a9264e slug=done/viewer-api-dioxus digest=cff476869051 -->
#### [11f77899] Port: TabBar, Spinner, Icons
- priority: `high`
- ref: `viewer-api/.ticket/tickets/11f77899-6def-4140-b6bf-e84035a9264e/ticket.toml`

<!-- ticket-index:entry id=46864375-0923-420c-b9db-67ce23056e52 slug=done/viewer-api-dioxus digest=33c7c51ad11a -->
#### [46864375] Port: Theme system — ThemeStore, CSS variables, presets, save/load
- priority: `high`
- ref: `viewer-api/.ticket/tickets/46864375-0923-420c-b9db-67ce23056e52/ticket.toml`

<!-- ticket-index:entry id=31739fc3-bb79-4b56-8dd6-ea789340ac8a slug=done/viewer-api-dioxus digest=485a1436726d -->
#### [31739fc3] Port: TreeView and FileTree with sort/filter
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/31739fc3-bb79-4b56-8dd6-ea789340ac8a/ticket.toml`

<!-- ticket-index:entry id=503eecc9-c8d6-4932-93df-e40018805818 slug=done/viewer-api-dioxus digest=5a87ce6e8fb5 -->
#### [503eecc9] Port: URL state management and session utilities
- priority: `medium`
- ref: `viewer-api/.ticket/tickets/503eecc9-c8d6-4932-93df-e40018805818/ticket.toml`

<!-- ticket-index:entry id=5f668df8-82e8-4d3c-b3a7-95052a04d688 slug=done/viewer-api-dioxus digest=866bac583f74 -->
#### [5f668df8] [ticket-viewer][spec-viewer] Bug: theme settings action does not open the modal
- priority: `high`
- summary: Root cause: the Dioxus Trunk entrypoints for ticket-viewer, spec-viewer, and viewer-api omitted the shared `modal.css` bundle, so the Theme Settings overlay mounted without the fixed backdrop and sta...
- ref: `.ticket/tickets/5f668df8-82e8-4d3c-b3a7-95052a04d688/ticket.toml`


### Component: viewer-api-leptos

<!-- ticket-index:entry id=29897f92-59bf-45f9-b963-caa7bfad71c8 slug=done/viewer-api-leptos digest=3292d3b8829c -->
#### [29897f92] Feature: UI polish — tab bar, sidebar, and resizable panels
- summary: The Leptos frontend has a minimal tab bar (20px, uppercase, no icons) and a flat sidebar (220px, no tree indentation, no resize). The TS version has a polished tab bar (32px, icons, active accents), ...
- ref: `viewer-api/.ticket/tickets/29897f92-59bf-45f9-b963-caa7bfad71c8/ticket.toml`


### Component: viewer-platform-spec

<!-- ticket-index:entry id=7d951620-76c5-4b24-90ce-e7d08d2dd188 slug=done/viewer-platform-spec digest=1e91a937eaf3 -->
#### [7d951620] [viewer-platform][spec] Define measurable browser and GPU validation contract
- priority: `high`
- summary: Created the focused `aligned-structure:v2` cross-viewer browser and GPU validation contract in `.spec/specs/e302d4c3-c24f-4778-bef0-453d3c1997bb/`.
- ref: `memory-viewers/.ticket/tickets/7d951620-76c5-4b24-90ce-e7d08d2dd188/ticket.toml`


### Component: viewer-tools

<!-- ticket-index:entry id=46d94c15-7a2d-4190-a1dc-93cd2b3a293b slug=done/viewer-tools digest=04e5fbc65667 -->
#### [46d94c15] Bug: ticket-viewer proxy URL encoding breaks non-ASCII query values
- ref: `.ticket/tickets/46d94c15-7a2d-4190-a1dc-93cd2b3a293b/ticket.toml`

<!-- ticket-index:entry id=91e22471-4895-4fcf-bab2-63efd7d9262d slug=done/viewer-tools digest=3ed75ab50a31 -->
#### [91e22471] Plan: ticket-viewer + ticket HTTP server mode + shared viewer-api graph architecture
- summary: Create a dedicated ticket-viewer (derived from doc-viewer structure), add HTTP server mode to `ticket` for live updates, and render dependency/state topology using the hypergraph display approach fro...
- ref: `.ticket/tickets/91e22471-4895-4fcf-bab2-63efd7d9262d/ticket.toml`

<!-- ticket-index:entry id=6d4d9a66-ed28-45e1-93f6-a6595c4593b3 slug=done/viewer-tools digest=f2d38c872391 -->
#### [6d4d9a66] Validation: ticket-viewer + ticket-serve E2E, scale envelope, and regression suite
- ref: `.ticket/tickets/6d4d9a66-ed28-45e1-93f6-a6595c4593b3/ticket.toml`


### Component: workflow-policy

<!-- ticket-index:entry id=a2c469c4-c62b-444d-81ed-6c936bee8ba3 slug=done/workflow-policy digest=87baeb91c70c -->
#### [a2c469c4] [workflow-policy][benchmarks] Research and define end-to-end benchmark and execution-evidence policy
- priority: `high`
- summary: Research and codify one repository workflow for end-to-end tool benchmarking that measures latency, throughput, and load behavior, records executable anchors, and produces durable metrics/evidence su...
- ref: `.ticket/tickets/a2c469c4-c62b-444d-81ed-6c936bee8ba3/ticket.toml`

<!-- ticket-index:entry id=a71c2da8-0972-4c2d-9754-0a0e06db5272 slug=done/workflow-policy digest=5b5071878052 -->
#### [a71c2da8] [workflow-policy][tracing][log-api] Research and define tracing instrumentation and log execution policy
- priority: `high`
- summary: Research and codify the repository policy for tracing instrumentation, runtime log capture, and managing generated logs plus executions through `log-api` without collapsing distinct stores into one o...
- ref: `.ticket/tickets/a71c2da8-0972-4c2d-9754-0a0e06db5272/ticket.toml`


### Component: workflow-tools

<!-- ticket-index:entry id=26da8f59-5d2d-41d9-9e87-7c74f200a9ce slug=done/workflow-tools digest=367974d7e50b -->
#### [26da8f59] [workflow-tools][consolidation] Retire legacy ticket-cli/ticket-mcp/ticket-http crates onto the pilot `ticket` crate bins
- priority: `high`
- summary: The `ticket` pilot crate (61ce77f9, done) **duplicated** rather than replaced the legacy ticket transports. Both surfaces exist and build today:
- ref: `.ticket/tickets/26da8f59-5d2d-41d9-9e87-7c74f200a9ce/ticket.toml`

<!-- ticket-index:entry id=0da6894c-dcbb-4196-8ac7-b6fae7c40ec9 slug=done/workflow-tools digest=9b46551fa103 -->
#### [0da6894c] [workflow-tools][design] Single domain crate per tool: unify api + transports as one crate with transport binary targets
- priority: `high`
- summary: Phase A design/contract. Define the canonical per-domain crate layout that every tool extraction must follow: a single domain crate (named after the domain, e.g. `ticket`) that unifies the domain API...
- ref: `.ticket/tickets/0da6894c-dcbb-4196-8ac7-b6fae7c40ec9/ticket.toml`

<!-- ticket-index:entry id=61ce77f9-b6be-4667-9727-ffbd6bf6b8f4 slug=done/workflow-tools digest=fafad25bacda -->
#### [61ce77f9] [workflow-tools][extraction] Extract ticket as first per-tool {domain} crate (pilot)
- priority: `high`
- summary: Action 3 of the workflow-tools extraction epic (69eb4118). Reviewer selected
- ref: `.ticket/tickets/61ce77f9-b6be-4667-9727-ffbd6bf6b8f4/ticket.toml`

<!-- ticket-index:entry id=1b7e0c3d-4e7e-4d70-a744-ab1bbe0fd34f slug=done/workflow-tools digest=fa769799150a -->
#### [1b7e0c3d] [workflow-tools][foundations] Cut over legacy memory-api consumers to memory-kernel
- priority: `high`
- summary: Workflow-tools repository extraction is blocked by a repository-level dependency cycle. `ticket-api` depends non-optionally on the legacy base crate at `memory-api/crates/memory-api`, while seven cra...
- ref: `.ticket/tickets/1b7e0c3d-4e7e-4d70-a744-ab1bbe0fd34f/ticket.toml`

<!-- ticket-index:entry id=60114a17-c0ad-43eb-8df6-4741a59d83ce slug=done/workflow-tools digest=5e4b7bafdc00 -->
#### [60114a17] [workflow-tools][foundations] Design memory-kernel transport-harness reference-proof integration tests
- priority: `high`
- summary: Research/design precursor for ticket 2cc7680c-7f19-4ad7-8658-29920e60ce1c, created from the 2026-07-25 review (verdict: Needs changes — "do more research and design before implementing").
- ref: `.ticket/tickets/60114a17-c0ad-43eb-8df6-4741a59d83ce/ticket.toml`

<!-- ticket-index:entry id=15e632f1-515b-43e1-9149-02163056d13d slug=done/workflow-tools digest=5cef4bf4580f -->
#### [15e632f1] [workflow-tools][foundations] Extract memory-matrix + rewire in-tree consumers to external memory-fixtures
- priority: `medium`
- summary: C:/Users/linus/AppData/Local/Temp/ticket-15e632f1-desc.md
- ref: `.ticket/tickets/15e632f1-515b-43e1-9149-02163056d13d/ticket.toml`

<!-- ticket-index:entry id=f10f52e4-baec-4fd8-9ac8-2e683ad05ee8 slug=done/workflow-tools digest=8f16ff903783 -->
#### [f10f52e4] [workflow-tools][foundations] Move the transport-harness contract into a memory-kernel spec
- priority: `high`
- summary: Review follow-up from ticket dbe0e955-c1b4-414d-820c-10c3fbbb5d3d.
- ref: `.ticket/tickets/f10f52e4-baec-4fd8-9ac8-2e683ad05ee8/ticket.toml`

<!-- ticket-index:entry id=9451f439-9a08-45bb-810e-771721ac1189 slug=done/workflow-tools digest=fd20db67f60d -->
#### [9451f439] [workflow-tools][foundations] Register memory-kernel as a context-engine submodule
- priority: `high`
- summary: Review follow-up from ticket dbe0e955-c1b4-414d-820c-10c3fbbb5d3d.
- ref: `.ticket/tickets/9451f439-9a08-45bb-810e-771721ac1189/ticket.toml`

<!-- ticket-index:entry id=2cc7680c-7f19-4ad7-8658-29920e60ce1c slug=done/workflow-tools digest=c4369c2247f8 -->
#### [2cc7680c] [workflow-tools][foundations] Replace trivial transport reference with realistic domain wiring
- priority: `high`
- summary: Review follow-up from ticket dbe0e955-c1b4-414d-820c-10c3fbbb5d3d.
- ref: `.ticket/tickets/2cc7680c-7f19-4ad7-8658-29920e60ce1c/ticket.toml`

<!-- ticket-index:entry id=dbe0e955-c1b4-414d-820c-10c3fbbb5d3d slug=done/workflow-tools digest=7aafb4cf1509 -->
#### [dbe0e955] [workflow-tools][foundations] Shared transport-harness crate for cli/mcp/http scaffolding reused by domain-crate binaries
- priority: `high`
- summary: Phase A foundation. Provide a shared transport-harness library crate that factors out the common cli / mcp / http scaffolding (argument parsing, MCP server setup, HTTP router/error mapping, output fo...
- ref: `.ticket/tickets/dbe0e955-c1b4-414d-820c-10c3fbbb5d3d/ticket.toml`

<!-- ticket-index:entry id=1c452ff1-6edc-47c8-a73d-d07ce9b4d473 slug=done/workflow-tools digest=3f58df108d6e -->
#### [1c452ff1] [workflow-tools][foundations] Stabilize viewer-api and memory-fixtures as standalone shared dependencies
- priority: `high`
- summary: Phase A. Stabilize the shared, non-domain support libraries as standalone dependencies consumable by every per-tool repo: the `viewer-api` framework (+ viewer-ctl, already its own repo) and shared te...
- ref: `.ticket/tickets/1c452ff1-6edc-47c8-a73d-d07ce9b4d473/ticket.toml`

<!-- ticket-index:entry id=a78ec124-1540-4d18-b3f0-cb02bca7764c slug=done/workflow-tools digest=881fa30d1793 -->
#### [a78ec124] [workflow-tools][pilot] Align ticket crate memory-fixtures dep to git pin (gates PILOT closure)
- priority: `medium`
- summary: Reviewer approved PILOT-TICKET (61ce77f9) on substance but gated closure on a convention fix.
- ref: `.ticket/tickets/a78ec124-1540-4d18-b3f0-cb02bca7764c/ticket.toml`

<!-- ticket-index:entry id=4f284d45-e520-4d22-945e-f54bdd3c0175 slug=done/workflow-tools digest=367e6f799aa8 -->
#### [4f284d45] [workflow-tools][provisioning] Create GitHub repositories for extracted workflow tooling (user action)
- priority: `high`
- summary: Phase 0 provisioning is complete as of 2026-07-25.
- ref: `.ticket/tickets/4f284d45-e520-4d22-945e-f54bdd3c0175/ticket.toml`


### Component: workspace-ownership

<!-- ticket-index:entry id=6ded2540-206b-4ffd-bdb6-23459a16ab1d slug=done/workspace-ownership digest=26047d47e44e -->
#### [6ded2540] [workspace] Relocate viewer and peek crates into owning repositories
- priority: `high`
- summary: Move repository-owned packages to their canonical Git submodule owners without changing package names or runtime behavior.
- ref: `.ticket/tickets/6ded2540-206b-4ffd-bdb6-23459a16ab1d/ticket.toml`


### Component: workspace-tooling

<!-- ticket-index:entry id=d505282b-ba78-4ddc-9071-20060630a86f slug=done/workspace-tooling digest=f4f59398d52d -->
#### [d505282b] [mcp-config] Register and activate local agent MCP servers in VS Code Copilot
- priority: `high`
- summary: Make locally installed MCP binaries discoverable and correctly identified from GitHub Copilot Chat in this VS Code workspace.
- ref: `.ticket/tickets/d505282b-ba78-4ddc-9071-20060630a86f/ticket.toml`


## State: in-implementation

### Component: agent-workflow

<!-- ticket-index:entry id=79449c3f-2f49-4925-b8fd-3751face53b5 slug=in-implementation/agent-workflow digest=28b76f0a7914 -->
#### [79449c3f] Define Explainer Agent version-one contract
- priority: `high`
- ref: `.ticket/tickets/79449c3f-2f49-4925-b8fd-3751face53b5/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=52724aed-7215-471b-b2d8-7fb425f5ed61 slug=in-implementation/memory-api digest=4590d6591c29 -->
#### [52724aed] Startup artifact pollution: MCP servers/viewers must not create stores or log dirs on mere startup in storeless workspaces
- priority: `high`
- summary: Eliminate startup-time filesystem artifact pollution across MCP servers and viewers: no tool may create a `.ticket` store, `test-logs/`/`target/test-logs/` directories, or any other persistent artifa...
- ref: `.ticket/tickets/52724aed-7215-471b-b2d8-7fb425f5ed61/ticket.toml`


### Component: ticket

<!-- ticket-index:entry id=ba4aaa9c-d270-4cfc-a1e2-395634608371 slug=in-implementation/ticket digest=fa7bffbfffd3 -->
#### [ba4aaa9c] [workflow-tools][per-tool] Extract ticket tool as a single `ticket` domain crate (api + transport bins) + viewer/vscode frontends
- priority: `high`
- summary: Phase B. Extract the ticket tool into its own `ticket` repository (owner mankinskin), built as a single `ticket` domain crate per contract `0da6894c`: the crate lib re-exports the internal `ticket-ap...
- ref: `.ticket/tickets/ba4aaa9c-d270-4cfc-a1e2-395634608371/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3 slug=in-implementation/ticket-api digest=84150bf41317 -->
#### [fa2ba34b] Session-anchored MCP workspace resolution: require session_id and resolve every proxied call to the session's active worktree
- priority: `high`
- summary: Eliminate silent ticket-store divergence between a worktree agent's file/CLI edits and ticket-mcp MCP writes.
- ref: `.ticket/tickets/fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3/ticket.toml`


### Component: viewer-api-performance

<!-- ticket-index:entry id=099ac71e-bffa-4a5b-89f3-2ca3bc875bac slug=in-implementation/viewer-api-performance digest=d69ee45d6c2d -->
#### [099ac71e] [profiling] Validate browser profiling pipeline (trace capture + wasm benches)
- priority: `high`
- summary: Phase-1 implementation is landed and compiles; this ticket covers the remaining
- ref: `.ticket/tickets/099ac71e-bffa-4a5b-89f3-2ca3bc875bac/ticket.toml`


### Component: workflow-policy

<!-- ticket-index:entry id=6e72756f-11c6-405f-8d74-0ab608172871 slug=in-implementation/workflow-policy digest=b5e2665b4d5c -->
#### [6e72756f] [workflow-policy] Tracker: durable store bootstrap, benchmark, and tracing/log guidance
- priority: `high`
- summary: Create one durable policy track for repository workflows that are currently too expensive to rediscover ad hoc:
- ref: `.ticket/tickets/6e72756f-11c6-405f-8d74-0ab608172871/ticket.toml`

<!-- ticket-index:entry id=79dd2d35-267b-4395-8316-0761df45f3c5 slug=in-implementation/workflow-policy digest=908d771ce74b -->
#### [79dd2d35] [workflow-policy][memory-store] Research and define minimal domain-store bootstrap policy
- priority: `high`
- summary: Research and codify the minimal, repeatable bootstrap policy for creating a new memory-backed store that can immediately perform CRUD, validate entity schemas, participate in workspace hierarchy reso...
- ref: `.ticket/tickets/79dd2d35-267b-4395-8316-0761df45f3c5/ticket.toml`


## State: in-review

### Component: agent-guidance

<!-- ticket-index:entry id=1d6a63aa-37ad-4374-a4bd-cc693d4b8ffd slug=in-review/agent-guidance digest=c70cd5edb1d7 -->
#### [1d6a63aa] Add rtk exception for path-stream command output
- priority: `low`
- summary: `.agents/instructions/orchestration/compact-output.instructions.md` is 28 lines with `applyTo: "**/*.sh,**/*.ps1"`. Under `Compact-by-Default Output` at line 6, line 17 says `Prefer rtk <cmd> over ba...
- ref: `.ticket/tickets/1d6a63aa-37ad-4374-a4bd-cc693d4b8ffd/ticket.toml`

<!-- ticket-index:entry id=7be23bd8-9793-4f86-a96d-403824f8af94 slug=in-review/agent-guidance digest=c093093ece7f -->
#### [7be23bd8] Agent session identity, worktree traceability, and prior-session inspection protocol
- priority: `high`
- summary: Agents work in isolated Git worktrees and hand work between sessions, but repository guidance does not make session lineage legible. The implementation scope is guidance only.
- ref: `.ticket/tickets/7be23bd8-9793-4f86-a96d-403824f8af94/ticket.toml`

<!-- ticket-index:entry id=900ea258-7203-4e5a-aeef-4cfe35c7ce49 slug=in-review/agent-guidance digest=022596ef0d21 -->
#### [900ea258] Allow small main-checkout changes in agent guidance
- priority: `medium`
- ref: `.ticket/tickets/900ea258-7203-4e5a-aeef-4cfe35c7ce49/ticket.toml`

<!-- ticket-index:entry id=dc13ffb4-f172-469c-a0ad-454354aa4f28 slug=in-review/agent-guidance digest=c28ac0633e60 -->
#### [dc13ffb4] Define a shared terminal return contract for all sub-agents
- priority: `high`
- summary: No shared instruction currently makes a dispatched sub-agent terminate with a usable deliverable. Create the proposed `.agents/instructions/orchestration/subagent-return-contract.instructions.md` and...
- ref: `.ticket/tickets/dc13ffb4-f172-469c-a0ad-454354aa4f28/ticket.toml`

<!-- ticket-index:entry id=cd7aceca-8df9-4ccc-8a13-935ad84fbb6d slug=in-review/agent-guidance digest=5c1c1a4179e1 -->
#### [cd7aceca] Define mermaid graph rendering conventions for agent responses
- priority: `medium`
- summary: When an agent renders a ticket dependency or ordering graph in a response, the diagram must make labels, edge semantics, and reading direction immediately understandable.
- ref: `.ticket/tickets/cd7aceca-8df9-4ccc-8a13-935ad84fbb6d/ticket.toml`

<!-- ticket-index:entry id=659a39a4-8c84-470b-8a3f-9fff3dd86acb slug=in-review/agent-guidance digest=6bff08621294 -->
#### [659a39a4] Delegation-quality failure taxonomy and orchestrator pre-dispatch command verification
- priority: `medium`
- summary: Measured in one real orchestrator session: of about 58 sub-agent dispatches, at least 6 failed outright and 9 units required re-dispatch. The failures fall into three root-cause classes.
- ref: `.ticket/tickets/659a39a4-8c84-470b-8a3f-9fff3dd86acb/ticket.toml`

<!-- ticket-index:entry id=f66f30d9-5fab-4b6f-b6e8-6379a7e046cb slug=in-review/agent-guidance digest=ebdf36315d39 -->
#### [f66f30d9] Replace literal runtime IDs in agent instructions
- ref: `.ticket/tickets/f66f30d9-5fab-4b6f-b6e8-6379a7e046cb/ticket.toml`


### Component: agent-orchestration

<!-- ticket-index:entry id=3c18e5b7-1c08-46fe-a752-bf810d3980a3 slug=in-review/agent-orchestration digest=d27f5482f188 -->
#### [3c18e5b7] Add a pre-dispatch gate set for read-only research delegations
- priority: `medium`
- summary: `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` is 168 lines and says gates apply to every delegation. `Per-Delegation-Class Gate Sets` at line 34 defines exactly four gate se...
- ref: `.ticket/tickets/3c18e5b7-1c08-46fe-a752-bf810d3980a3/ticket.toml`

<!-- ticket-index:entry id=b8cc3ff3-ba80-4da1-95c8-94da089cb2c0 slug=in-review/agent-orchestration digest=9f4130a3fa04 -->
#### [b8cc3ff3] Distinguish compile failures from test failures in the retry cap
- priority: `medium`
- summary: `.agents/instructions/orchestration/retry-limit.instructions.md` is 43 lines with `applyTo: "**/*.rs,**/*.ts,**/tests/**"`. Its retry cap is stated purely for test failures: a step that fails a test ...
- ref: `.ticket/tickets/b8cc3ff3-ba80-4da1-95c8-94da089cb2c0/ticket.toml`


### Component: agent-templates

<!-- ticket-index:entry id=b7731dd0-1c9c-4c05-8abe-6bd95e129255 slug=in-review/agent-templates digest=a8a66a4960fc -->
#### [b7731dd0] Add remote dependency resolution proof to live validation agent
- priority: `medium`
- summary: `.agents/agents/live-validation.agent.md` has Constraints at line 29 but has no proof contract for a cross-repository dependency resolving from its remote source.
- ref: `.ticket/tickets/b7731dd0-1c9c-4c05-8abe-6bd95e129255/ticket.toml`

<!-- ticket-index:entry id=378e4e57-22bb-40ee-bc72-623eb330aa55 slug=in-review/agent-templates digest=5a98048b8bfc -->
#### [378e4e57] Delete the Default agent template and its routing entry
- priority: `medium`
- summary: `.agents/agents/Default.agent.md` is nine lines total. Its frontmatter description is the literal placeholder `Describe what this custom agent does and when to use it.`, it declares `model: "GPT-5.6 ...
- ref: `.ticket/tickets/378e4e57-22bb-40ee-bc72-623eb330aa55/ticket.toml`

<!-- ticket-index:entry id=4578f53b-2ee2-457e-94bc-0f720e900c5a slug=in-review/agent-templates digest=fb652fef124e -->
#### [4578f53b] Enforce state-store write-target discipline in implement and commit agents
- priority: `high`
- summary: `.agents/instructions/commit/branch-worktree.instructions.md` already contains `### Entity stores are worktree-local` at line 127, so the state-store rule is partial rather than absent. No agent temp...
- ref: `.ticket/tickets/4578f53b-2ee2-457e-94bc-0f720e900c5a/ticket.toml`

<!-- ticket-index:entry id=d597fdf0-2bc9-4a30-bd80-19b9e0414dfe slug=in-review/agent-templates digest=edd0f060996e -->
#### [d597fdf0] Harden research agent return contract and output fields
- priority: `high`
- summary: `.agents/agents/research.agent.md` is 49 lines. Line 29 explicitly permits questions: `Ask concise follow-up questions only when a focused search still leaves a material ambiguity.` The permission co...
- ref: `.ticket/tickets/d597fdf0-2bc9-4a30-bd80-19b9e0414dfe/ticket.toml`

<!-- ticket-index:entry id=967c3cf6-e73c-4701-9bf0-f51d30914d70 slug=in-review/agent-templates digest=de55c6c8a114 -->
#### [967c3cf6] Require architectural decisions to be settled before implementation planning
- priority: `medium`
- summary: `.agents/agents/ticket-refinement.agent.md` has Required Workflow at line 34 but no mandatory pre-plan decision checklist.
- ref: `.ticket/tickets/967c3cf6-e73c-4701-9bf0-f51d30914d70/ticket.toml`

<!-- ticket-index:entry id=48c3a5cb-98a1-4c2c-a5c8-fd7cd7dc7b66 slug=in-review/agent-templates digest=44219652ab16 -->
#### [48c3a5cb] Require migration preflight inventory before bulk file moves
- priority: `high`
- summary: `.agents/agents/implement.agent.md` has its Required Workflow at line 53. The workflow has no pre-migration inventory rule before bulk file moves.
- ref: `.ticket/tickets/48c3a5cb-98a1-4c2c-a5c8-fd7cd7dc7b66/ticket.toml`


### Component: agent-tooling

<!-- ticket-index:entry id=4bf9b3b4-0185-4dce-a9a9-5a5e8f762e11 slug=in-review/agent-tooling digest=25f96fb20e14 -->
#### [4bf9b3b4] Agent template roster redesign: 15 new specialized agents, orchestrator rewrite
- summary: Specification: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/4bf9b3b4-0185-4dce-a9a9-5a5e8f762e11/ticket.toml`

<!-- ticket-index:entry id=b51c3460-a699-4748-9436-1fd45a4d621e slug=in-review/agent-tooling digest=f5fd32ff61e7 -->
#### [b51c3460] Audit ticket and session graph renderers for the same labelling gaps
- priority: `low`
- summary: The repository ships graph renderers of its own: the ticket CLI/MCP `topgraph` and `subgraph` commands, the session workflow renderers `session_workflow_render_mermaid` and `session_workflow_render_t...
- ref: `.ticket/tickets/b51c3460-a699-4748-9436-1fd45a4d621e/ticket.toml`

<!-- ticket-index:entry id=4c0d1d46-86ea-4b80-8f4c-3d9d8d3f0ad1 slug=in-review/agent-tooling digest=8cb0431592b9 -->
#### [4c0d1d46] Batch 1: session and worktree agents
- summary: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/4c0d1d46-86ea-4b80-8f4c-3d9d8d3f0ad1/ticket.toml`

<!-- ticket-index:entry id=f1eaa1b9-37c8-41f7-8d0e-862f0f7ebd5b slug=in-review/agent-tooling digest=f7708946dfaa -->
#### [f1eaa1b9] Batch 2: research and writing agents
- summary: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/f1eaa1b9-37c8-41f7-8d0e-862f0f7ebd5b/ticket.toml`

<!-- ticket-index:entry id=8d837ab6-a3d7-41f4-aefc-f3ad78477a2c slug=in-review/agent-tooling digest=4ba59d306c94 -->
#### [8d837ab6] Batch 3: quality and architecture agents
- summary: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/8d837ab6-a3d7-41f4-aefc-f3ad78477a2c/ticket.toml`

<!-- ticket-index:entry id=a243a5ba-1b8d-4888-a44f-d484c9de39ea slug=in-review/agent-tooling digest=8833e017be7c -->
#### [a243a5ba] Batch 4: ops and intake agents
- summary: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/a243a5ba-1b8d-4888-a44f-d484c9de39ea/ticket.toml`

<!-- ticket-index:entry id=6e9f3a9c-7d3e-49c6-9a53-17fca59f7a4b slug=in-review/agent-tooling digest=b7b53566e46f -->
#### [6e9f3a9c] Batch 5: orchestrator rewrite, simplify extension, command agent removal
- summary: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/6e9f3a9c-7d3e-49c6-9a53-17fca59f7a4b/ticket.toml`

<!-- ticket-index:entry id=a070fc1c-c407-4c8c-9bd4-fdf7a1f4fa97 slug=in-review/agent-tooling digest=e6d3db682ed0 -->
#### [a070fc1c] Update AGENTS.md and instruction cross-references for the new agent roster
- summary: [Agent template roster and authoring contract](.spec/specs/88413517-1d93-4582-8328-71a417dde3a1/spec.toml)
- ref: `.ticket/tickets/a070fc1c-c407-4c8c-9bd4-fdf7a1f4fa97/ticket.toml`


### Component: documentation

<!-- ticket-index:entry id=0c4298da-ccba-45f8-9759-fed686d28d7f slug=in-review/documentation digest=edc34f7c5978 -->
#### [0c4298da] Update ticket-cli references in COMMANDS.md and memory-api README
- priority: `medium`
- summary: `COMMANDS.md` line 25 has a `## ticket-cli` heading and line 29 declares source `memory-api/tools/cli/ticket-cli`. `memory-api/README.md` line 9 links `tools/cli/ticket-cli/README.md`; line 106 insta...
- ref: `.ticket/tickets/0c4298da-ccba-45f8-9759-fed686d28d7f/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=8130027d-557b-4797-9fc2-a57cb51fd01e slug=in-review/memory-api digest=32c155eca81e -->
#### [8130027d] Contain CLI ticket-store resolution and purge cross-worktree scan roots
- priority: `high`
- summary: Make direct ticket CLI execution checkout-contained: an explicit selector always wins, and discovery remains inside the caller's checkout.
- ref: `.ticket/tickets/8130027d-557b-4797-9fc2-a57cb51fd01e/ticket.toml`


### Component: peek

<!-- ticket-index:entry id=b24ffddf-eb6a-48b3-ada7-984c13436447 slug=in-review/peek digest=9fb7ebb1835a -->
#### [b24ffddf] Refresh repo map and hardcoded ticket-cli tool description
- priority: `low`
- summary: `memory-api/crates/peek-api/src/lib.rs` line 544 hardcodes the description `"ticket-cli (state machine, board, deps)"`. The stale source text propagates into generated `repo_map.toon` line 295. `repo...
- ref: `.ticket/tickets/b24ffddf-eb6a-48b3-ada7-984c13436447/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=7063311c-a380-4269-adcf-7d1388ab5f39 slug=in-review/repo-guidance digest=c76f6c61aabd -->
#### [7063311c] Author spec for repository architecture and dependency policies
- priority: `high`
- summary: Document the repository-level architecture and dependency policies that now govern workflow-tool extraction: cross-repository git dependency resolution, the `memory-kernel` neutrality boundary, the d...
- ref: `.ticket/tickets/7063311c-a380-4269-adcf-7d1388ab5f39/ticket.toml`

<!-- ticket-index:entry id=d2bf768f-4011-42fa-9149-97d6adb0c322 slug=in-review/repo-guidance digest=1216e22a3cb3 -->
#### [d2bf768f] Document cross-repo git-URL dependency and patch-override policy
- priority: `high`
- summary: Create `.agents/instructions/commit/cross-repo-dependencies.instructions.md` as the repository-wide policy for dependencies that move into another repository. The instruction must state all of the fo...
- ref: `.ticket/tickets/d2bf768f-4011-42fa-9149-97d6adb0c322/ticket.toml`

<!-- ticket-index:entry id=665a5df8-eed9-4adb-8022-fe7f07955062 slug=in-review/repo-guidance digest=9bead7ccbebd -->
#### [665a5df8] Document kernel neutrality boundary and extension-trait pattern
- priority: `high`
- summary: Create `.agents/instructions/engine/kernel-layering.instructions.md` to state that `memory-kernel` contains neutral data and neutral contracts only. Domain-specific types, keys, and semantics must no...
- ref: `.ticket/tickets/665a5df8-eed9-4adb-8022-fe7f07955062/ticket.toml`

<!-- ticket-index:entry id=9a1bffce-b825-4f58-a078-2351d9bdaa16 slug=in-review/repo-guidance digest=0cee751803e2 -->
#### [9a1bffce] Document the {domain}-api plus public {domain} crate architecture
- priority: `high`
- summary: `.github/copilot-instructions.md` lines 25-27 say each domain crate exposes transports at `tools/cli/*`, `tools/mcp/*`, and sometimes `tools/http/*`, with business logic in a `-api` crate. The passag...
- ref: `.ticket/tickets/9a1bffce-b825-4f58-a078-2351d9bdaa16/ticket.toml`

<!-- ticket-index:entry id=33aee765-7c5e-4e86-ad97-29dbf15c7259 slug=in-review/repo-guidance digest=934a9ee85566 -->
#### [33aee765] Fix stale memory-kernel description in copilot-instructions
- priority: `high`
- summary: `.github/copilot-instructions.md` lines 33-34 describe `memory-kernel/` as a development-only submodule for `transport-harness` that is not needed for ordinary work. The statement appears in the `Rep...
- ref: `.ticket/tickets/33aee765-7c5e-4e86-ad97-29dbf15c7259/ticket.toml`

<!-- ticket-index:entry id=95ca18e4-6b4c-48ec-a3f9-f3ec0bc3cbd7 slug=in-review/repo-guidance digest=ae4351b72d33 -->
#### [95ca18e4] Remove CHEAT_SHEET.md references from the guidance corpus
- priority: `medium`
- summary: `AGENTS.md` lists `CHEAT_SHEET.md` under `Canonical Sources` and cites the file in the Discovery Protocol for type-level gotchas and common patterns. `.github/copilot-instructions.md` also references...
- ref: `.ticket/tickets/95ca18e4-6b4c-48ec-a3f9-f3ec0bc3cbd7/ticket.toml`

<!-- ticket-index:entry id=a74f09cf-2c4b-4c13-9247-cd74519b6b7e slug=in-review/repo-guidance digest=bc4d54e170ac -->
#### [a74f09cf] State CLI binary naming policy as an explicit rule
- priority: `medium`
- summary: Add an explicit CLI binary naming rule to `.github/copilot-instructions.md`, near the existing transport naming examples around line 88. The rule must state that the command-line binary uses the bare...
- ref: `.ticket/tickets/a74f09cf-2c4b-4c13-9247-cd74519b6b7e/ticket.toml`


### Component: session

<!-- ticket-index:entry id=85bfbb06-bf3a-4ea4-b57d-a829b8cd2545 slug=in-review/session digest=7f653000ff4b -->
#### [85bfbb06] Record sub-agent runs: SubagentStart/SubagentStop hook capture and delegation-quality rollups
- priority: `high`
- summary: An orchestrator-mode session delegates every unit of work to sub-agents, but the repository can currently observe almost nothing about those sub-agents.
- ref: `.ticket/tickets/85bfbb06-bf3a-4ea4-b57d-a829b8cd2545/ticket.toml`


### Component: testing

<!-- ticket-index:entry id=81d52e34-b73a-4be3-a7ce-3986b2a823b2 slug=in-review/testing digest=4321d35cefdc -->
#### [81d52e34] Add latent-versus-introduced failure triage to test debugging guidance
- priority: `high`
- summary: Add a `Regression Triage` section to `.agents/instructions/testing/test-debugging.instructions.md` immediately after `Debug Workflow` (currently line 15 in the 22-line instruction). The section must ...
- ref: `.ticket/tickets/81d52e34-b73a-4be3-a7ce-3986b2a823b2/ticket.toml`

<!-- ticket-index:entry id=df27707f-3f61-4707-9aae-f6614d767510 slug=in-review/testing digest=fbdfad58a796 -->
#### [df27707f] Add order-sensitive API assertion rule
- priority: `medium`
- summary: Extend `.agents/instructions/testing/assertions.instructions.md`, whose only heading is `Assertions` at line 5, with a rule for APIs whose contract includes ordering. Tests must use stable ordered fi...
- ref: `.ticket/tickets/df27707f-3f61-4707-9aae-f6614d767510/ticket.toml`


### Component: ticket

<!-- ticket-index:entry id=ea44ba5f-2db1-420a-9285-5c01b5654151 slug=in-review/ticket digest=eb37664e8c45 -->
#### [ea44ba5f] Derive and render latent edges: transitive dependencies and execution-order edges
- priority: `medium`
- summary: The ticket store persists only direct `depends_on` edges. Two useful edge
- ref: `.ticket/tickets/ea44ba5f-2db1-420a-9285-5c01b5654151/ticket.toml`

<!-- ticket-index:entry id=2e07430b-5ade-4384-8b4f-93dd6de73203 slug=in-review/ticket digest=efcd6090ebf2 -->
#### [2e07430b] Ticket CLI panics when stdout is closed early
- priority: `medium`
- summary: `./target/debug/ticket.exe list --toon | head -40` panics when `head` closes stdout, exits with code 130, and produces no usable output. The same outcome occurs with ordinary early-closing consumers ...
- ref: `.ticket/tickets/2e07430b-5ade-4384-8b4f-93dd6de73203/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=29a56eef-d1ac-4bae-8c1f-817b33a232b5 slug=in-review/ticket-api digest=26bdb953de65 -->
#### [29a56eef] [ticket-api] create_ticket accepts off-schema state values, and health_check does not detect the resulting frozen tickets
- priority: `high`
- summary: `create_ticket` accepts a `state` value that is not a member of the ticket
- ref: `.ticket/tickets/29a56eef-d1ac-4bae-8c1f-817b33a232b5/ticket.toml`


### Component: tooling

<!-- ticket-index:entry id=25f01a1e-81c2-45e6-a2d3-d0c8777c0db3 slug=in-review/tooling digest=4d9c67206e2d -->
#### [25f01a1e] Extend gitattributes to stop CRLF churn in tracked source files
- priority: `medium`
- summary: Extend `.gitattributes`, which currently contains only `*.sh text eol=lf`, to normalize tracked source files including `*.toml`, `*.rs`, and `*.md` so CRLF and whitespace-only churn does not appear a...
- ref: `.ticket/tickets/25f01a1e-81c2-45e6-a2d3-d0c8777c0db3/ticket.toml`

<!-- ticket-index:entry id=07a3eb2d-8868-4c36-a60a-e93cc787c065 slug=in-review/tooling digest=9433090dc31f -->
#### [07a3eb2d] Repair build and install tooling referencing removed ticket-cli package
- priority: `high`
- summary: Three tooling files still target the deleted `ticket-cli` package, causing real build/install failures:
- ref: `.ticket/tickets/07a3eb2d-8868-4c36-a60a-e93cc787c065/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=2606f325-b029-4fae-8442-94b1793b786e slug=in-review/unspecified digest=19b3df00da95 -->
#### [2606f325] Sync-copilot-surfaces should try symlinks with read-only-copy fallback
- summary: tools/install/sync-copilot-surfaces.sh currently always deep-copies .agents/{agents,prompts,instructions} into .github/{agents,prompts,instructions}. This risks silent data loss: someone edits the ge...
- ref: `.ticket/tickets/2606f325-b029-4fae-8442-94b1793b786e/ticket.toml`

<!-- ticket-index:entry id=edb92a7d-b735-4c7b-b339-36847df68f76 slug=in-review/unspecified digest=65eb3364687d -->
#### [edb92a7d] Unify bootstrap into one script and wire up Copilot CLI agent surfaces (mcp/agents/prompts/instructions)
- summary: Bootstrapping `context-engine` today needs 5 separate manual commands
- ref: `.ticket/tickets/edb92a7d-b735-4c7b-b339-36847df68f76/ticket.toml`

<!-- ticket-index:entry id=5d4078fa-d7eb-4f0d-bf84-e21029f5ad5d slug=in-review/unspecified digest=3fab1af9f0fa -->
#### [5d4078fa] [content-materialization][feedback-api] G-D: Close the feedback ring — execution→verified, transcript mining, missing-rule tickets, ticket-entity feedback
- summary: Close the open loop so the system improves itself. The ring is an **emergent distributed loop**, not a module: every domain writes feedback into the feedback-api hub and reacts to outcomes. Extends t...
- ref: `memory-api/.ticket/tickets/5d4078fa-d7eb-4f0d-bf84-e21029f5ad5d/ticket.toml`


### Component: workflow-tools

<!-- ticket-index:entry id=e8a5c061-d474-4752-b063-3a8b730f6765 slug=in-review/workflow-tools digest=860bb1bd9d57 -->
#### [e8a5c061] Require repo-level dependency-cycle check before crate extraction
- priority: `high`
- summary: Document a mandatory pre-extraction dependency-cycle check that inspects cycles across repository boundaries, not only a single Cargo crate graph. The guidance must record two concrete discoveries:
- ref: `.ticket/tickets/e8a5c061-d474-4752-b063-3a8b730f6765/ticket.toml`


### Component: worktree-lifecycle

<!-- ticket-index:entry id=5f075124-402c-4a47-a549-5f522c4d95d1 slug=in-review/worktree-lifecycle digest=e3e675f7ee30 -->
#### [5f075124] Bootstrap worktree-local repository stores
- priority: `high`
- summary: Add a one-command worktree bootstrap path that creates or reuses the assigned worktree and runs the repository initializer in that worktree. The command must leave `new` as Git-only, support dry runs...
- ref: `.ticket/tickets/5f075124-402c-4a47-a549-5f522c4d95d1/ticket.toml`


## State: new

### Component: memory-api

<!-- ticket-index:entry id=d893ed4c-a6a2-4fe4-94c5-175d4da13a9b slug=new/memory-api digest=e0111595caac -->
#### [d893ed4c] [ticket-mcp][memory-api] Enumerate discovered descendant stores in list_workspaces for viewer domain selection
- priority: `medium`
- summary: W7. Descendant .ticket stores are already discovered and folded into the aggregated default index (verified: memory-viewers/viewer-api/memory-api tickets appear in .ticket/index.toon), but list_works...
- ref: `memory-viewers/.ticket/tickets/d893ed4c-a6a2-4fe4-94c5-175d4da13a9b/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=9a7c3f5b-2d1e-4f6a-8b0c-3d2f1e4b6a7c slug=new/session-api digest=826ae89a8b01 -->
#### [9a7c3f5b] [session-api][handoff] Implementation-ready handoff: concise target-ticket & session summaries
- summary: Implementation-ready handoff support: concise target-ticket and session summaries
- ref: `.ticket/tickets/9a7c3f5b-2d1e-4f6a-8b0c-3d2f1e4b6a7c/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=f7efc6f8-78c4-4f2a-bcb9-95ef1c21bb67 slug=new/ticket-viewer digest=f30f4cbdf478 -->
#### [f7efc6f8] Arch: ticket-viewer dioxus-frontend crate scaffold with Trunk
- priority: `critical`
- ref: `memory-viewers/.ticket/tickets/f7efc6f8-78c4-4f2a-bcb9-95ef1c21bb67/ticket.toml`

<!-- ticket-index:entry id=4e2b2b0b-9f56-4786-991c-8f10e653f4c3 slug=new/ticket-viewer digest=173119b6cbd7 -->
#### [4e2b2b0b] Epic: ticket-viewer UI polish — theme consistency, transparent buttons, shared crates, tiling panels
- priority: `high`
- summary: Visual inspection of the running ticket-viewer (http://localhost:3002) revealed four classes of UI defects, ranging from a CSS variable bug to missing platform-level infrastructure. This epic groups ...
- ref: `memory-viewers/.ticket/tickets/4e2b2b0b-9f56-4786-991c-8f10e653f4c3/ticket.toml`

<!-- ticket-index:entry id=a60ccc7f-c8cd-4eb1-aa8e-5e127e98383e slug=new/ticket-viewer digest=ffe66a1c04cd -->
#### [a60ccc7f] Feature: Description editor — Markdown textarea with live preview
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/a60ccc7f-c8cd-4eb1-aa8e-5e127e98383e/ticket.toml`

<!-- ticket-index:entry id=859a1174-1c91-49d7-bc05-28beb39047ef slug=new/ticket-viewer digest=b42b4fac253f -->
#### [859a1174] Feature: Edge management — add/remove dependencies from graph
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/859a1174-1c91-49d7-bc05-28beb39047ef/ticket.toml`

<!-- ticket-index:entry id=b4679179-a65c-4eb4-82ea-590a1ecdf1ca slug=new/ticket-viewer digest=8c72fb5635bb -->
#### [b4679179] Feature: Full-text search UI with field predicates
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/b4679179-a65c-4eb4-82ea-590a1ecdf1ca/ticket.toml`

<!-- ticket-index:entry id=593b094d-fcaa-43c4-a693-2ccec4fbc0b4 slug=new/ticket-viewer digest=bdc33416ec9f -->
#### [593b094d] Feature: GPU 3D dependency graph via WebGPU
- priority: `low`
- ref: `memory-viewers/.ticket/tickets/593b094d-fcaa-43c4-a693-2ccec4fbc0b4/ticket.toml`

<!-- ticket-index:entry id=dd80a182-8d0d-4439-bb59-668b9e6a5672 slug=new/ticket-viewer digest=862da9c67f61 -->
#### [dd80a182] Feature: History timeline — revision viewer with field diffs
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/dd80a182-8d0d-4439-bb59-668b9e6a5672/ticket.toml`

<!-- ticket-index:entry id=dc1a8740-d808-4c4f-ac82-1bea9e22183c slug=new/ticket-viewer digest=21cd9dd6578e -->
#### [dc1a8740] Feature: State transition UI — visual state machine with advance/undo
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/dc1a8740-d808-4c4f-ac82-1bea9e22183c/ticket.toml`

<!-- ticket-index:entry id=5185921d-1fea-409d-98eb-6d57e4b5502a slug=new/ticket-viewer digest=9652edc9bd01 -->
#### [5185921d] Feature: Ticket creation form with type selection and required fields
- priority: `critical`
- ref: `memory-viewers/.ticket/tickets/5185921d-1fea-409d-98eb-6d57e4b5502a/ticket.toml`

<!-- ticket-index:entry id=3e7f4202-13f9-4daf-be91-3875fde8fce8 slug=new/ticket-viewer digest=3932b00300b1 -->
#### [3e7f4202] Feature: Ticket inline editing — title, priority, component, custom fields
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/3e7f4202-13f9-4daf-be91-3875fde8fce8/ticket.toml`

<!-- ticket-index:entry id=d264a42c-500f-43fa-be5d-3e832679fe67 slug=new/ticket-viewer digest=e90fa4ff631f -->
#### [d264a42c] Port: SSE integration for real-time ticket updates
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/d264a42c-500f-43fa-be5d-3e832679fe67/ticket.toml`

<!-- ticket-index:entry id=ec5d383a-1fa6-4e15-8090-6e5e3c1d94fa slug=new/ticket-viewer digest=e376d2b7521f -->
#### [ec5d383a] Port: SVG dependency graph fallback view
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/ec5d383a-1fa6-4e15-8090-6e5e3c1d94fa/ticket.toml`

<!-- ticket-index:entry id=e2a6ad44-a58d-4a85-b976-bece05ce3a9d slug=new/ticket-viewer digest=1fefbcf73b17 -->
#### [e2a6ad44] Port: State persistence — localStorage per-workspace + URL routing
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/e2a6ad44-a58d-4a85-b976-bece05ce3a9d/ticket.toml`

<!-- ticket-index:entry id=af7a881d-b5f6-459d-bd55-31b999057c33 slug=new/ticket-viewer digest=8110b3f30e41 -->
#### [af7a881d] Port: TicketContent viewer — Markdown + TOML tabs
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/af7a881d-b5f6-459d-bd55-31b999057c33/ticket.toml`

<!-- ticket-index:entry id=57efb581-3b7e-4c94-a0ec-798dbfc49527 slug=new/ticket-viewer digest=d37930efa062 -->
#### [57efb581] Port: TicketTree with state grouping, search, filter, sort
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/57efb581-3b7e-4c94-a0ec-798dbfc49527/ticket.toml`

<!-- ticket-index:entry id=d8694ff6-b6dc-4707-8f00-51bbdaf11c20 slug=new/ticket-viewer digest=ad9d5da0a7c8 -->
#### [d8694ff6] Port: WorkspacePicker with auth token management
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/d8694ff6-b6dc-4707-8f00-51bbdaf11c20/ticket.toml`

<!-- ticket-index:entry id=c892904f-6aff-4892-baf1-1f832837cde5 slug=new/ticket-viewer digest=02889df5830a -->
#### [c892904f] Tracker: ticket-viewer store observability roadmap
- priority: `high`
- summary: High-priority roadmap to improve observability of the internal stores through the ticket-viewer and shared viewer-api. Consolidates and refines fragmented existing planning into one tracked program.
- ref: `memory-viewers/.ticket/tickets/c892904f-6aff-4892-baf1-1f832837cde5/ticket.toml`

<!-- ticket-index:entry id=1b0dbe8a-1828-41e7-b33f-4a066c3622bf slug=new/ticket-viewer digest=207423d08ee1 -->
#### [1b0dbe8a] [ticket-viewer] Clickable parent/child neighbours in content panel with cache-driven graph retarget
- priority: `high`
- summary: W4. List immediate parents and children in the content panel; make them clickable to jump the selection to the linked node; the rendered graph must update from cache instead of fetching at click time.
- ref: `memory-viewers/.ticket/tickets/1b0dbe8a-1828-41e7-b33f-4a066c3622bf/ticket.toml`

<!-- ticket-index:entry id=a2f5460c-1e7e-481b-a250-e9def213ba55 slug=new/ticket-viewer digest=3714293cf92f -->
#### [a2f5460c] [ticket-viewer] Explorer filtering + local keyboard follow-up
- priority: `high`
- summary: Research against the current Dioxus ticket-viewer shows the explorer still has three concrete gaps:
- ref: `memory-viewers/.ticket/tickets/a2f5460c-1e7e-481b-a250-e9def213ba55/ticket.toml`

<!-- ticket-index:entry id=0b7da330-a38d-49e5-853c-cf1d40633b6f slug=new/ticket-viewer digest=e410859d685a -->
#### [0b7da330] [ticket-viewer] Feature: multi-select state filter in explorer
- priority: `high`
- summary: The explorer still models ticket state filtering as a single selected value.
- ref: `memory-viewers/.ticket/tickets/0b7da330-a38d-49e5-853c-cf1d40633b6f/ticket.toml`

<!-- ticket-index:entry id=a889ec06-62f0-4933-a313-c74eb0c896ce slug=new/ticket-viewer digest=69c2051efa75 -->
#### [a889ec06] [ticket-viewer] Fix Trunk dev websocket handshake
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/a889ec06-62f0-4933-a313-c74eb0c896ce/ticket.toml`

<!-- ticket-index:entry id=e978d833-bbaa-4cb8-8c1a-52a282a079d9 slug=new/ticket-viewer digest=04939cb08c16 -->
#### [e978d833] [ticket-viewer] Render bounded neighbourhood subgraph from cached full graph
- priority: `high`
- summary: W2. The immediate neighbourhood is hard to track when the full graph is rendered. Render only a focused set of nodes/edges within a limited distance of the selected node(s); load the full graph into ...
- ref: `memory-viewers/.ticket/tickets/e978d833-bbaa-4cb8-8c1a-52a282a079d9/ticket.toml`

<!-- ticket-index:entry id=05dae5fd-1a1d-4a64-be62-f29ca0771a4d slug=new/ticket-viewer digest=e7858d7e6e8c -->
#### [05dae5fd] [ticket-viewer][ticket-http][viewer-api] Improve main layout ticket documents and focused full-graph navigation
- priority: `high`
- summary: Upgrade the ticket-viewer main layout so ticket details render as a compact integrated document and the graph view becomes a focused full-workspace navigation surface with better layout, settings, an...
- ref: `memory-viewers/.ticket/tickets/05dae5fd-1a1d-4a64-be62-f29ca0771a4d/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=30347ae7-c182-49a8-93fd-41819aa7848c slug=new/unspecified digest=25adcd98698c -->
#### [30347ae7] A — Opt-in legacy handoff migration & re-render support
- summary: Goal: Add opt-in migration and re-render support for legacy handoff records (preserve historical handoff `a9519525`).
- ref: `.ticket/tickets/30347ae7-c182-49a8-93fd-41819aa7848c/ticket.toml`

<!-- ticket-index:entry id=c321dc52-e846-4e6f-8d7e-af08ea3c2e87 slug=new/unspecified digest=3041ec50feee -->
#### [c321dc52] B — Visible non-fatal ticket-mirror failure reporting, retry, and alerting
- summary: Goal: Implement visible, non-fatal reporting and retry/alerting for ticket-mirror failures.
- ref: `.ticket/tickets/c321dc52-e846-4e6f-8d7e-af08ea3c2e87/ticket.toml`

<!-- ticket-index:entry id=ba602b61-5216-44bf-af04-87b6132547a7 slug=new/unspecified digest=830e06b43db6 -->
#### [ba602b61] C — Coordination: overlap reconciliation for tickets e4f84414, 25b5f3e7, 742dbc65, ba8f5528 (non-blocking)
- summary: Goal: Create a coordination-only ticket to reconcile overlapping scope across tickets `e4f84414`, `25b5f3e7`, `742dbc65`, and `ba8f5528` without blocking their current progress.
- ref: `.ticket/tickets/ba602b61-5216-44bf-af04-87b6132547a7/ticket.toml`

<!-- ticket-index:entry id=59abe2da-d7fe-4acb-b154-7186336fcd4c slug=new/unspecified digest=92660798b70d -->
#### [59abe2da] D — Spec cleanup for 5e52039d: manifest traceability and readiness wording
- summary: Goal: Clean up spec `5e52039d` to ensure manifest traceability links and reconcile spec wording with `SessionHandoffPackage::is_implementation_ready` package-list predicates.
- ref: `.ticket/tickets/59abe2da-d7fe-4acb-b154-7186336fcd4c/ticket.toml`

<!-- ticket-index:entry id=1d8d82b5-8e40-463f-adaf-30d2f5625844 slug=new/unspecified digest=c742de2de502 -->
#### [1d8d82b5] [LOG-1a] ticket-viewer: wire init_tracing_full with file logging to target/logs/"
- summary: `tools/viewer/ticket-viewer/src/main.rs` calls `init_tracing("info,ticket_http::serve::handlers=debug")`. All structured log output goes to stderr only. When started in detached mode (default `viewer...
- ref: `memory-viewers/.ticket/tickets/1d8d82b5-8e40-463f-adaf-30d2f5625844/ticket.toml`

<!-- ticket-index:entry id=3b1345eb-9983-4567-a3ab-c2e00c7cec1e slug=new/unspecified digest=c50d7c9be473 -->
#### [3b1345eb] [LOG-1c] viewer-ctl: add --log-dir and --log-level flags to start/restart
- summary: There is no way to control the log directory or level for a viewer server launched via `viewer-ctl start` without modifying the binary's default env vars by hand. Operators cannot redirect logs to a ...
- ref: `viewer-api/.ticket/tickets/3b1345eb-9983-4567-a3ab-c2e00c7cec1e/ticket.toml`

<!-- ticket-index:entry id=aaaf1d50-e2ee-4352-b9b4-495c7eadb117 slug=new/unspecified digest=3aa17a8f9f0c -->
#### [aaaf1d50] [context-editor][SVO-RM] Phase 1 tracker (de-retro cluster migration)
- ref: `context-stack/tools/context-editor/.ticket/tickets/aaaf1d50-e2ee-4352-b9b4-495c7eadb117/ticket.toml`

<!-- ticket-index:entry id=8afb299f-4a14-4eaf-8cbc-d9472f69c84b slug=new/unspecified digest=aac3156255d4 -->
#### [8afb299f] [context-read] Expansion loop redesign tracker (de-retro cluster migration)
- ref: `context-stack/.ticket/tickets/8afb299f-4a14-4eaf-8cbc-d9472f69c84b/ticket.toml`

<!-- ticket-index:entry id=a6fd15f6-f9c3-407a-af99-3febee5b2557 slug=new/unspecified digest=02f3ce1881b9 -->
#### [a6fd15f6] [doc-viewer][P6] (Deferred) Migrate doc-viewer onto shared viewer-api
- summary: Optional / deferred.** Once the Dioxus shared crate is stable and proven in spec-viewer, expose its components to doc-viewer (Preact/TS) via thin TypeScript bindings or a wasm-bindgen surface, elimin...
- ref: `viewer-api/.ticket/tickets/a6fd15f6-f9c3-407a-af99-3febee5b2557/ticket.toml`

<!-- ticket-index:entry id=0feb20f3-205a-4e71-9902-31c8c5bb13eb slug=new/unspecified digest=9dd019cf5844 -->
#### [0feb20f3] [spec-viewer][P5] Adopt new viewer-api primitives in spec-viewer
- summary: Adopt the new viewer-api widgets and stores in spec-viewer, replacing ad-hoc inline implementations.
- ref: `memory-viewers/.ticket/tickets/0feb20f3-205a-4e71-9902-31c8c5bb13eb/ticket.toml`

<!-- ticket-index:entry id=521e18b7-bed4-4588-886c-e25d6c8ddc8b slug=new/unspecified digest=cc1c4e4e0309 -->
#### [521e18b7] [spec-viewer][design] reachable page graph + entry×view navigation
- summary: Design the next navigation model for `spec-viewer` so users can reach the important UI surfaces by clicking through the app and can switch specs without losing the current view context.
- ref: `memory-viewers/.ticket/tickets/521e18b7-bed4-4588-886c-e25d6c8ddc8b/ticket.toml`

<!-- ticket-index:entry id=a76ce0b4-e906-4ecd-8513-0cb763ec305c slug=new/unspecified digest=edc43508898b -->
#### [a76ce0b4] [spec-viewer][nav-1] canonical entry×view route contract + URL normalization
- summary: Implement the canonical `entry × view` navigation contract for `spec-viewer`.
- ref: `memory-viewers/.ticket/tickets/a76ce0b4-e906-4ecd-8513-0cb763ec305c/ticket.toml`

<!-- ticket-index:entry id=8a3fe2eb-511a-4d2d-9e98-c17b9b812399 slug=new/unspecified digest=d90580d020ce -->
#### [8a3fe2eb] [spec-viewer][nav-2] fold tree into /specs + restore click-through reachability
- summary: Fold the old tree route into the root browse page and make the primary page graph reachable by clicks only.
- ref: `memory-viewers/.ticket/tickets/8a3fe2eb-511a-4d2d-9e98-c17b9b812399/ticket.toml`

<!-- ticket-index:entry id=57db7e0f-1189-4b06-8cd2-718f8d9beace slug=new/unspecified digest=776960a6992e -->
#### [57db7e0f] [spec-viewer][nav-3] preserve active view while switching specs
- summary: Preserve the active spec view while switching between specs, using deterministic per-spec fallback when needed.
- ref: `memory-viewers/.ticket/tickets/57db7e0f-1189-4b06-8cd2-718f8d9beace/ticket.toml`

<!-- ticket-index:entry id=bf250aa9-ca23-4686-83ca-c1395b1e3d1e slug=new/unspecified digest=92a4213f6ef9 -->
#### [bf250aa9] [spec-viewer][nav-4] browser verification for reachable page graph + entry×view
- summary: Add browser regression coverage for the new spec-viewer reachable page graph and `entry × view` navigation semantics.
- ref: `memory-viewers/.ticket/tickets/bf250aa9-ca23-4686-83ca-c1395b1e3d1e/ticket.toml`

<!-- ticket-index:entry id=936d38d6-a238-4cb9-b00a-1b2a4b65dc04 slug=new/unspecified digest=558e1abaa6fc -->
#### [936d38d6] [viewer-api] Port doc-viewer features to shared Dioxus viewer-api
- summary: Track the remaining Dioxus adoption work needed after the shared doc-viewer-inspired primitives landed in `viewer-api-dioxus`, so current viewers reuse the shared shells and stores instead of carryin...
- ref: `viewer-api/.ticket/tickets/936d38d6-a238-4cb9-b00a-1b2a4b65dc04/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=35a6d14b-25b0-4b24-b59f-d0d733cacd20 slug=new/viewer-api digest=08194136043d -->
#### [35a6d14b] Epic: Dioxus Viewer Platform — viewer-api-dioxus + ticket-viewer Dioxus frontend
- priority: `critical`
- summary: Port the viewer-api frontend library and ticket-viewer SPA from TypeScript/Preact to Rust/Dioxus 0.7, compiled to WASM via `trunk` (Trunk WASM bundler). Adds full ticket mutation capabilities powered...
- ref: `viewer-api/.ticket/tickets/35a6d14b-25b0-4b24-b59f-d0d733cacd20/ticket.toml`

<!-- ticket-index:entry id=92161adb-d06a-47b6-a065-4fb652e764ea slug=new/viewer-api digest=fdac883368c9 -->
#### [92161adb] Epic: Dioxus Viewer Platform — viewer-api-dioxus + ticket-viewer Dioxus frontend
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/92161adb-d06a-47b6-a065-4fb652e764ea/ticket.toml`

<!-- ticket-index:entry id=81a6a595-7426-478d-9487-17142dcfa8a0 slug=new/viewer-api digest=ad562c724110 -->
#### [81a6a595] Plan: Context API phase 4.1 — viewer-api + log-viewer as thin frontend layers
- summary: tags: `#context-api` `#phase4.1` `#refactor` `#viewer-api` `#log-viewer` `#context-http` `#frontend`
- ref: `viewer-api/.ticket/tickets/81a6a595-7426-478d-9487-17142dcfa8a0/ticket.toml`

<!-- ticket-index:entry id=301dc3ce-38b6-4b28-bd84-266e33b46c90 slug=new/viewer-api digest=bf14f62e1312 -->
#### [301dc3ce] Plan: DOM 3D integration for graph viewer
- summary: tags: `#plan` `#rendering` `#3d` `#webgpu` `#dom`
- ref: `viewer-api/.ticket/tickets/301dc3ce-38b6-4b28-bd84-266e33b46c90/ticket.toml`

<!-- ticket-index:entry id=ee7aa0cd-04ae-423e-83be-6edf58eeaf41 slug=new/viewer-api digest=c087463b6260 -->
#### [ee7aa0cd] Plan: nesting view mode for graph viewer
- summary: Date:** 2026-03-07
- ref: `viewer-api/.ticket/tickets/ee7aa0cd-04ae-423e-83be-6edf58eeaf41/ticket.toml`

<!-- ticket-index:entry id=608bb106-f22d-4bb4-bbde-d87ec33fd6e6 slug=new/viewer-api digest=4eef291b5757 -->
#### [608bb106] Plan: search visualisation in graph viewer
- summary: tags: `#plan` `#visualization` `#search` `#logging` `#frontend`
- ref: `viewer-api/.ticket/tickets/608bb106-f22d-4bb4-bbde-d87ec33fd6e6/ticket.toml`

<!-- ticket-index:entry id=20c4d807-042f-4c4b-a683-3d84658094c3 slug=new/viewer-api digest=ed49ba3dd87e -->
#### [20c4d807] Plan: viewer refactoring and mobile support — HypergraphView extraction
- summary: Date:** 2026-03-04
- ref: `viewer-api/.ticket/tickets/20c4d807-042f-4c4b-a683-3d84658094c3/ticket.toml`

<!-- ticket-index:entry id=68912b00-e189-4dc1-8124-ca41d9aab953 slug=new/viewer-api digest=4e7398a024a0 -->
#### [68912b00] Plan: viewer tools features (2026-03-01 batch)
- summary: Date:** 2026-03-01
- ref: `viewer-api/.ticket/tickets/68912b00-e189-4dc1-8124-ca41d9aab953/ticket.toml`

<!-- ticket-index:entry id=97c757b1-3c58-4b54-ab4b-35b7d0ea9ece slug=new/viewer-api digest=1f132d4b7333 -->
#### [97c757b1] Plan: viewer-api refactoring — extract shared server infrastructure
- summary: tags: `#plan` `#refactoring` `#tools` `#viewer-api`
- ref: `viewer-api/.ticket/tickets/97c757b1-3c58-4b54-ab4b-35b7d0ea9ece/ticket.toml`

<!-- ticket-index:entry id=966a7714-0bf4-443b-b778-8e0f383f99d8 slug=new/viewer-api digest=9dddbb7bd585 -->
#### [966a7714] [viewer-api] Build demo-viewer exercising all viewer components with root-level domain selection
- priority: `medium`
- summary: W5b. A complete demo-viewer that exercises all components from all viewers in a single application with arbitrary data, and a root-level domain selection to switch between domains.
- ref: `memory-viewers/.ticket/tickets/966a7714-0bf4-443b-b778-8e0f383f99d8/ticket.toml`

<!-- ticket-index:entry id=d1e4ab96-52e1-4b80-ad7c-bfff459d3fac slug=new/viewer-api digest=bfc977d73aff -->
#### [d1e4ab96] [viewer-api] Converge shared Dioxus viewer shells across frontends
- priority: `high`
- summary: Converge the duplicated Dioxus viewer shell patterns across the current frontend implementations so each viewer stays thin and generic behavior lives in viewer-api.
- ref: `viewer-api/.ticket/tickets/d1e4ab96-52e1-4b80-ad7c-bfff459d3fac/ticket.toml`

<!-- ticket-index:entry id=0d995e82-26b8-4f71-afc0-f5de75d5a468 slug=new/viewer-api digest=2d8a4d9160d2 -->
#### [0d995e82] [viewer-api] Create minimal viewer-template bootstrap (tree explorer, tabbed main, right/bottom panels, floating panels)
- priority: `medium`
- summary: W5a. We need a minimal viewer-template that serves as the bootstrap for new viewers (rule-viewer, audit-viewer).
- ref: `memory-viewers/.ticket/tickets/0d995e82-26b8-4f71-afc0-f5de75d5a468/ticket.toml`

<!-- ticket-index:entry id=e0d8ccef-49d8-4446-bd63-78626a4163dc slug=new/viewer-api digest=f22f18416fc4 -->
#### [e0d8ccef] [viewer-api] Optimize Graph3D renderer for performance and extensibility
- priority: `high`
- summary: W6. The viewer-api 3D graph view needs optimization to efficiently provide the implemented features and to allow future extensibility in different contexts.
- ref: `memory-viewers/.ticket/tickets/e0d8ccef-49d8-4446-bd63-78626a4163dc/ticket.toml`

<!-- ticket-index:entry id=4e0dc8fb-18fa-4be1-a43d-37008d0453e3 slug=new/viewer-api digest=141e16f66967 -->
#### [4e0dc8fb] [viewer-api][ticket-viewer][design] viewer-wide keyboard support model
- priority: `medium`
- summary: Design a viewer-wide keyboard interaction model for the Dioxus viewer stack without bundling it into the immediate explorer fixes.
- ref: `viewer-api/.ticket/tickets/4e0dc8fb-18fa-4be1-a43d-37008d0453e3/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=7a2cc57e-aeed-4302-a483-6eb68955cbd0 slug=new/viewer-api-dioxus digest=645f04a46ae4 -->
#### [7a2cc57e] Arch: viewer-api-dioxus crate scaffold and build system
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/7a2cc57e-aeed-4302-a483-6eb68955cbd0/ticket.toml`

<!-- ticket-index:entry id=88c694f8-fded-46c5-b09e-6bf7f381562f slug=new/viewer-api-dioxus digest=6b1d510abf48 -->
#### [88c694f8] Feature: Theme settings UI panel with live preview
- priority: `medium`
- ref: `viewer-api/.ticket/tickets/88c694f8-fded-46c5-b09e-6bf7f381562f/ticket.toml`

<!-- ticket-index:entry id=f2f062b9-6b32-49b0-a465-f16fc0f9a362 slug=new/viewer-api-dioxus digest=ada9b547bc87 -->
#### [f2f062b9] Feature: WgpuOverlay effects system port — particles, CRT, vignette
- priority: `low`
- ref: `viewer-api/.ticket/tickets/f2f062b9-6b32-49b0-a465-f16fc0f9a362/ticket.toml`

<!-- ticket-index:entry id=01932eb7-54e5-441b-87bc-db3013a0882c slug=new/viewer-api-dioxus digest=93800e82d9a9 -->
#### [01932eb7] Feature: tiling + tabbed panel system replacing flat Sidebar/Panel primitives
- priority: `medium`
- summary: The current `viewer-api-dioxus::components::layout::{Sidebar, Panel}` primitives are flat: each viewer hardcodes a single left `Sidebar` + one optional right `Panel`. There is no support for:
- ref: `viewer-api/.ticket/tickets/01932eb7-54e5-441b-87bc-db3013a0882c/ticket.toml`

<!-- ticket-index:entry id=7149d59e-d303-4a5a-aad3-5b0100b9c5f7 slug=new/viewer-api-dioxus digest=a95eae0cc83d -->
#### [7149d59e] Port: CSS stylesheets — base, layout, buttons, tabs, tree, code-viewer
- priority: `high`
- ref: `viewer-api/.ticket/tickets/7149d59e-d303-4a5a-aad3-5b0100b9c5f7/ticket.toml`

<!-- ticket-index:entry id=ee302615-4916-417a-8074-1b84e92e5967 slug=new/viewer-api-dioxus digest=65c60d27dd34 -->
#### [ee302615] Port: CodeViewer and FileContentViewer
- priority: `high`
- ref: `viewer-api/.ticket/tickets/ee302615-4916-417a-8074-1b84e92e5967/ticket.toml`

<!-- ticket-index:entry id=f1e31f8a-af8a-4d0b-bd83-9ca8c682cfa6 slug=new/viewer-api-dioxus digest=4068b37e97f2 -->
#### [f1e31f8a] Port: Layout components — Header, Layout, Sidebar, Panel, GlassPanel
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/f1e31f8a-af8a-4d0b-bd83-9ca8c682cfa6/ticket.toml`

<!-- ticket-index:entry id=cf5c7199-2aa2-4a6f-ab3b-5f7aa5bd4e75 slug=new/viewer-api-dioxus digest=02f60c768e95 -->
#### [cf5c7199] Port: ResizeHandle with rAF-batched drag
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/cf5c7199-2aa2-4a6f-ab3b-5f7aa5bd4e75/ticket.toml`

<!-- ticket-index:entry id=b106752e-4112-44dd-8359-5586a18d1292 slug=new/viewer-api-dioxus digest=fa283b53e73c -->
#### [b106752e] Port: TabBar, Spinner, Icons
- priority: `high`
- ref: `viewer-api/.ticket/tickets/b106752e-4112-44dd-8359-5586a18d1292/ticket.toml`

<!-- ticket-index:entry id=edd5de82-858a-410d-a71c-efdcfc967b64 slug=new/viewer-api-dioxus digest=4484fe914755 -->
#### [edd5de82] Port: Theme system — ThemeStore, CSS variables, presets, save/load
- priority: `high`
- ref: `viewer-api/.ticket/tickets/edd5de82-858a-410d-a71c-efdcfc967b64/ticket.toml`

<!-- ticket-index:entry id=4102096d-cdba-4001-9fa2-266511b4f353 slug=new/viewer-api-dioxus digest=a14614a06f6a -->
#### [4102096d] Port: TreeView and FileTree with sort/filter
- priority: `critical`
- ref: `viewer-api/.ticket/tickets/4102096d-cdba-4001-9fa2-266511b4f353/ticket.toml`

<!-- ticket-index:entry id=5f910868-ce90-4166-9c0a-ba1e6a5f1711 slug=new/viewer-api-dioxus digest=5c32dc4e134e -->
#### [5f910868] Port: URL state management and session utilities
- priority: `medium`
- ref: `viewer-api/.ticket/tickets/5f910868-ce90-4166-9c0a-ba1e6a5f1711/ticket.toml`

<!-- ticket-index:entry id=92964ada-4ab5-4fe1-ab29-5bfd55583ad2 slug=new/viewer-api-dioxus digest=d54d71d024f9 -->
#### [92964ada] Refactor: extract viewer-theme and viewer-widgets crates from viewer-api-dioxus
- priority: `medium`
- summary: `viewer-api-dioxus` currently bundles three logically distinct concerns:
- ref: `viewer-api/.ticket/tickets/92964ada-4ab5-4fe1-ab29-5bfd55583ad2/ticket.toml`


### Component: viewer-api-e2e

<!-- ticket-index:entry id=6002e996-dcb8-4dad-a830-20346ce9d8cc slug=new/viewer-api-e2e digest=b642587417eb -->
#### [6002e996] [viewer-platform][e2e] Canonical shared Playwright harness and all-viewer runner
- priority: `high`
- summary: Playwright ownership and configuration are fragmented. Ticket-viewer forks shared suites and viewer definitions, GPU launch profiles differ, and `cargo make test-e2e` omits ticket-viewer while retain...
- ref: `memory-viewers/.ticket/tickets/6002e996-dcb8-4dad-a830-20346ce9d8cc/ticket.toml`


### Component: viewer-api-gpu-validation

<!-- ticket-index:entry id=26a73130-c631-4168-8030-fade31c5cf55 slug=new/viewer-api-gpu-validation digest=835c3c91f88c -->
#### [26a73130] [viewer-platform][gpu-test] Reproducible software and hardware browser lanes
- priority: `medium`
- summary: Spec-viewer and ticket-viewer currently launch materially different browser/GPU configurations. SwiftShader, headless bundled Chromium, headed system Chrome, and native hardware results are not compa...
- ref: `memory-viewers/.ticket/tickets/26a73130-c631-4168-8030-fade31c5cf55/ticket.toml`


### Component: viewer-api-observability

<!-- ticket-index:entry id=9202bc21-b11f-4464-b66b-af19dd6e7078 slug=new/viewer-api-observability digest=1a3ef8ad2cc7 -->
#### [9202bc21] [viewer-platform][observability] Correlate and attach WASM, browser, and backend logs per test
- priority: `high`
- summary: The server persists `x-session-id`, but the WASM `NetworkLayer` currently constructs its own request headers and does not visibly use the shared session helper. Ticket-viewer also carries an older tr...
- ref: `memory-viewers/.ticket/tickets/9202bc21-b11f-4464-b66b-af19dd6e7078/ticket.toml`

<!-- ticket-index:entry id=8f349d96-a307-400b-a90e-3aceb2250166 slug=new/viewer-api-observability digest=77588acac53e -->
#### [8f349d96] viewer-api-dioxus: ship WASM tracing logs to a server file sink
- priority: `high`
- summary: Ship structured tracing records from the Dioxus WASM frontend to a bounded server-side JSONL sink so browser records are queryable alongside backend logs.
- ref: `viewer-api/.ticket/tickets/8f349d96-a307-400b-a90e-3aceb2250166/ticket.toml`

<!-- ticket-index:entry id=b480632a-8605-4b5b-a4e8-f2988b2565a0 slug=new/viewer-api-observability digest=03f58122bc2e -->
#### [b480632a] viewer-api-dioxus: structured tracing for WASM frontend
- priority: `high`
- summary: Replace ad-hoc browser console calls in viewer-api Dioxus/WASM code with structured tracing that supports levels, fields, spans, runtime filtering, console diagnostics, and the separate persisted sin...
- ref: `viewer-api/.ticket/tickets/b480632a-8605-4b5b-a4e8-f2988b2565a0/ticket.toml`


### Component: viewer-api-performance

<!-- ticket-index:entry id=459022a5-3900-4f7b-8fbd-2de1fbb79bdd slug=new/viewer-api-performance digest=da702bfd9be1 -->
#### [459022a5] [viewer-platform][perf] Add browser/WASM soak and resource-leak detection
- priority: `medium`
- summary: Short FPS samples cannot detect retained event listeners, leaked wasm-bindgen closures, unbounded tracing buffers, DOM growth, texture/resource accumulation, or gradual frame-time degradation.
- ref: `memory-viewers/.ticket/tickets/459022a5-3900-4f7b-8fbd-2de1fbb79bdd/ticket.toml`

<!-- ticket-index:entry id=09bef250-b7a4-4ffa-91a0-52292ba907fa slug=new/viewer-api-performance digest=354d77ae4d31 -->
#### [09bef250] [viewer-platform][perf] Enforce browser and WASM regression budgets
- priority: `high`
- summary: The current Chromium profile suite only proves that a trace file was written, while the WASM micro-benchmarks assert only that elapsed time is finite. Neither fails when performance regresses.
- ref: `memory-viewers/.ticket/tickets/09bef250-b7a4-4ffa-91a0-52292ba907fa/ticket.toml`


### Component: viewer-platform-ci

<!-- ticket-index:entry id=0556ed59-57a6-4ca1-a8a4-93fdb1549c22 slug=new/viewer-platform-ci digest=3a64437d1fa4 -->
#### [0556ed59] [viewer-platform][ci] Enforce browser lanes and retain diagnostic artifacts
- priority: `high`
- summary: Repository policy requires Playwright for browser-facing changes, but no checked-in workflow currently runs the viewer Playwright or WASM browser suites. Reports, traces, screenshots, logs, and bench...
- ref: `memory-viewers/.ticket/tickets/0556ed59-57a6-4ca1-a8a4-93fdb1549c22/ticket.toml`


### Component: viewer-platform-tauri

<!-- ticket-index:entry id=431707b1-e512-4e47-9926-01407230e6db slug=new/viewer-platform-tauri digest=a913669771aa -->
#### [431707b1] [viewer-platform][tauri-test] Native desktop E2E validation lane
- priority: `low`
- summary: Add a deferred native desktop validation lane after the browser/WASM contract is stable. This lane proves only behavior that Chromium Playwright cannot establish: Tauri IPC, native window lifecycle, ...
- ref: `memory-viewers/.ticket/tickets/431707b1-e512-4e47-9926-01407230e6db/ticket.toml`


### Component: viewer-platform-testing

<!-- ticket-index:entry id=956485ad-2e80-4a4c-b5ec-42bac2c7c295 slug=new/viewer-platform-testing digest=3cd5ef7ded4f -->
#### [956485ad] [viewer-platform][testing] Robust browser, observability, and performance validation strategy
- priority: `high`
- summary: Establish one implementation-ready validation program for the shared viewer-api platform and the ticket-viewer, spec-viewer, log-viewer, and doc-viewer operational frontends. The program must prove b...
- ref: `memory-viewers/.ticket/tickets/956485ad-2e80-4a4c-b5ec-42bac2c7c295/ticket.toml`


### Component: viewer-platform-ux-validation

<!-- ticket-index:entry id=40110a1a-40e8-4345-8407-577bd5f4d602 slug=new/viewer-platform-ux-validation digest=d91b0dd90f56 -->
#### [40110a1a] [viewer-platform][ux-test] Accessibility, responsive, focus, and visual regression gates
- priority: `high`
- summary: Current suites include useful screenshots, keyboard checks, and isolated responsive assertions, but there is no shared accessibility scanner or maintained visual baseline. Screenshot attachments alon...
- ref: `memory-viewers/.ticket/tickets/40110a1a-40e8-4345-8407-577bd5f4d602/ticket.toml`


## State: on-hold

### Component: agent-workflow

<!-- ticket-index:entry id=f227c217-6fda-452e-ae35-4208eb3974f5 slug=on-hold/agent-workflow digest=6e74cb210ca5 -->
#### [f227c217] [token-efficiency] Update guidance for compact agent workflows
- priority: `medium`
- summary: Update the repository guidance so agent workflows consistently prefer compact/default outputs and verbose-on-demand expansion.
- ref: `.ticket/tickets/f227c217-6fda-452e-ae35-4208eb3974f5/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=47914c71-bb3c-4b95-9120-6121dd42ae2b slug=on-hold/unspecified digest=230209341bb1 -->
#### [47914c71] Multiplayer Backend: SpacetimeDB Server Module — Tables, Reducers, Auth & Tick Loop
- priority: `high`
- summary: The context-editor is currently a single-player application. To enable multiplayer open-world gameplay, we need an authoritative server that stores world state, validates player actions, manages auth...
- ref: `context-stack/tools/context-editor/.ticket/tickets/47914c71-bb3c-4b95-9120-6121dd42ae2b/ticket.toml`

<!-- ticket-index:entry id=c3e01552-88d2-446c-955d-f3902a224a56 slug=on-hold/unspecified digest=68429d0fbd30 -->
#### [c3e01552] [ticket-api][ticket-cli][ticket-mcp] Generate capability catalog programmatically from live CLI/MCP shapes (drift-checked)
- summary: Deferred until the workflow-tools architectural migration lands. Direction change: rather than binding a hand-maintained catalog to live CLI/MCP shapes via a drift check within the current per-transp...
- ref: `.ticket/tickets/c3e01552-88d2-446c-955d-f3902a224a56/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=bb1c32f5-5275-4e4f-85ae-a0fba09c522a slug=on-hold/viewer-api digest=589422205a99 -->
#### [bb1c32f5] [viewer-api] Extract a reusable Dioxus page header shell
- priority: `high`
- summary: Extract a reusable Dioxus page-header shell in viewer-api-dioxus so viewer routes stop composing ad-hoc header behavior inline.
- ref: `viewer-api/.ticket/tickets/bb1c32f5-5275-4e4f-85ae-a0fba09c522a/ticket.toml`


## State: open

### Component: agent-guidance

<!-- ticket-index:entry id=1eb03085-125b-4bab-a82f-ac15079d8dc5 slug=open/agent-guidance digest=bdc5976625e8 -->
#### [1eb03085] Author recurring test-audit agent template
- priority: `medium`
- summary: Build reusable recurring test-audit agent/instruction template after manual inventory work.
- ref: `.ticket/tickets/1eb03085-125b-4bab-a82f-ac15079d8dc5/ticket.toml`

<!-- ticket-index:entry id=b8d14fe8-d607-45cc-9281-cf4e113a5c56 slug=open/agent-guidance digest=aa88f74ca3ca -->
#### [b8d14fe8] Author repository test-writing template
- priority: `medium`
- summary: Author guidance for writing strong tests in this repository.
- ref: `.ticket/tickets/b8d14fe8-d607-45cc-9281-cf4e113a5c56/ticket.toml`

<!-- ticket-index:entry id=b0f0cde0-9fbe-474e-b2b3-ea393571bc11 slug=open/agent-guidance digest=8a5b9b6e3a44 -->
#### [b0f0cde0] Phase 7: Institutionalize coverage practice
- priority: `medium`
- summary: Capture the manual audit methodology before automating recurring coverage work.
- ref: `.ticket/tickets/b0f0cde0-9fbe-474e-b2b3-ea393571bc11/ticket.toml`

<!-- ticket-index:entry id=bd044e88-1aea-45c9-8823-8f879467d0e4 slug=open/agent-guidance digest=2cdf0e6bb5c4 -->
#### [bd044e88] Require a capable model tier for worktree and git-state verification
- priority: `medium`
- summary: Four separate cheap-tier (T3) sub-agent reports about worktree and submodule state were internally contradictory or wrong:
- ref: `.ticket/tickets/bd044e88-1aea-45c9-8823-8f879467d0e4/ticket.toml`

<!-- ticket-index:entry id=9515b7db-6bbe-4ec4-acae-494d51ae9c99 slug=open/agent-guidance digest=19357c5ca311 -->
#### [9515b7db] Retire browser-tool preference spec 347b6f97
- priority: `medium`
- summary: Move browser-tool preference from product specification into contributor instructions and retire spec `347b6f97`.
- ref: `.ticket/tickets/9515b7db-6bbe-4ec4-acae-494d51ae9c99/ticket.toml`

<!-- ticket-index:entry id=527fc2cf-7796-4320-a340-0161941f3154 slug=open/agent-guidance digest=698bbfabcd74 -->
#### [527fc2cf] Runtime tag-driven dynamic guidance injection (rule-api backed)
- priority: `low`
- summary: Static `applyTo`-glob classing (see ticket 9cd886d5) covers the near-term case: a fixed set of always-on general rules plus domain-scoped specialized files. It does not cover selecting guidance dynam...
- ref: `.ticket/tickets/527fc2cf-7796-4320-a340-0161941f3154/ticket.toml`


### Component: agent-harness

<!-- ticket-index:entry id=5bb96360-90e3-43d5-b03b-d568d163eeff slug=open/agent-harness digest=a4495f5a85a3 -->
#### [5bb96360] [agent-harness] Agent-driven interactive chat UI (UI sandbox interaction protocol + skill)
- priority: `high`
- summary: Make the agent a first-class driver of the shared chat UI ("UI sandbox") — the common interface between agent and user. Through the chat surface the agent controls a virtual world: rendering UI scene...
- ref: `.ticket/tickets/5bb96360-90e3-43d5-b03b-d568d163eeff/ticket.toml`

<!-- ticket-index:entry id=b947e0d3-2f1b-4b88-b885-990367ac8563 slug=open/agent-harness digest=19bbd4afeaa1 -->
#### [b947e0d3] [agent-harness][UI1] agent-shared: agent->UI interaction protocol + response envelope
- priority: `high`
- summary: Define a typed, versioned agent->UI interaction protocol in `agent-shared`, enumerating every interaction the agent may perform in the UI sandbox. This is the foundation all other chat-UI-interaction...
- ref: `.ticket/tickets/b947e0d3-2f1b-4b88-b885-990367ac8563/ticket.toml`

<!-- ticket-index:entry id=3dd84de0-474a-4797-ba83-787c907ae55b slug=open/agent-harness digest=73329b938abb -->
#### [3dd84de0] [agent-harness][UI2] agent-core: emit interaction directives + correlate user responses
- priority: `high`
- summary: Let the `agent-core` ReAct loop emit UI-sandbox interaction directives as part of a normal turn, and correlate user responses back into the same session.
- ref: `.ticket/tickets/3dd84de0-474a-4797-ba83-787c907ae55b/ticket.toml`

<!-- ticket-index:entry id=11db5933-2a39-4fa1-adcc-119ec572e8bd slug=open/agent-harness digest=f9060b80f7de -->
#### [11db5933] [agent-harness][UI3] agent-uapi: render agent-driven interactions in Dioxus WASM chat UI
- priority: `high`
- summary: Render every agent-driven interaction kind in the Dioxus WASM browser chat UI (the OpenCode-style target surface), and send user responses back over the websocket.
- ref: `.ticket/tickets/11db5933-2a39-4fa1-adcc-119ec572e8bd/ticket.toml`

<!-- ticket-index:entry id=cdf1e535-335a-4ffb-88ba-17fd72c05b69 slug=open/agent-harness digest=eebfd796d90e -->
#### [cdf1e535] [agent-harness][UI4] agent-uapi: render/degrade agent-driven interactions in Ratatui TUI
- priority: `medium`
- summary: Handle every agent-driven interaction kind in the Ratatui TUI chat surface, with explicit, observable graceful degradation where the terminal cannot render an interaction (e.g. images, rich scene/col...
- ref: `.ticket/tickets/cdf1e535-335a-4ffb-88ba-17fd72c05b69/ticket.toml`

<!-- ticket-index:entry id=f834173b-8524-4702-8dea-7f865ba060c7 slug=open/agent-harness digest=ffaa0b6e0305 -->
#### [f834173b] [agent-harness][UI5] Skill: UI-sandbox interactions the agent can perform
- priority: `medium`
- summary: Author a skill that documents the UI-sandbox interactions the agent can perform, so any agent loop can discover and correctly use the chat-UI interaction protocol.
- ref: `.ticket/tickets/f834173b-8524-4702-8dea-7f865ba060c7/ticket.toml`


### Component: agent-hooks

<!-- ticket-index:entry id=7f4dcf79-c37c-4234-9ec7-99f35491b8b9 slug=open/agent-hooks digest=4cafecd92951 -->
#### [7f4dcf79] Repair hook-capture E2E payload drift and add a subagent fixture
- summary: The existing hook-capture E2E harness reaches `copilot -p` but has reported exit 1 from live payload schema drift that predates Subagent capture work. The pre-existing assertion contract is common ke...
- ref: `.ticket/tickets/7f4dcf79-c37c-4234-9ec7-99f35491b8b9/ticket.toml`


### Component: agent-orchestration

<!-- ticket-index:entry id=780a6aa1-e4d6-43fb-a923-9fbb275fcece slug=open/agent-orchestration digest=89024fd8f521 -->
#### [780a6aa1] Add a pre-dispatch gate set for read-only research delegations
- priority: `medium`
- summary: `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` is 168 lines and says gates apply to every delegation. `Per-Delegation-Class Gate Sets` at line 34 defines exactly four gate se...
- ref: `.ticket/tickets/780a6aa1-e4d6-43fb-a923-9fbb275fcece/ticket.toml`

<!-- ticket-index:entry id=54114fc9-1a6f-4f9b-a0b5-0d13ac5188b7 slug=open/agent-orchestration digest=8e7f31481d71 -->
#### [54114fc9] Spec: deterministic Rust execution controller for ticket-driven workflows
- priority: `low`
- summary: Session/step sequencing today is agent-driven (LLMs decide control flow via MCP tool calls), which allows overstepping scope, indefinite iteration loops, and unpredictable termination. Ticket depende...
- ref: `.ticket/tickets/54114fc9-1a6f-4f9b-a0b5-0d13ac5188b7/ticket.toml`


### Component: agent-templates

<!-- ticket-index:entry id=3df54f79-71bd-4262-a711-8eb260153c4f slug=open/agent-templates digest=2c6c854c406c -->
#### [3df54f79] [agent-templates] Grant web/online-search tooling to the Research Agent
- priority: `medium`
- summary: The repo needs an agent capable of online search. `.agents/agents/research.agent.md` (line 4 tools list) and `.agents/agents/explore.agent.md` are both repo-scoped: neither carries a `web` grant. `we...
- ref: `.ticket/tickets/3df54f79-71bd-4262-a711-8eb260153c4f/ticket.toml`


### Component: agent-tooling

<!-- ticket-index:entry id=e07eb28b-77e8-44bf-86eb-d3b43c17a61c slug=open/agent-tooling digest=dad389a94061 -->
#### [e07eb28b] Align ticket and session graph renderers with graph presentation contract
- summary: Align ticket CLI/MCP graph output, session Mermaid and terminal workflow renderers, and the ticket-viewer dependency graph with the rendering contract: typed labels, label-safe presentation, legend o...
- ref: `.ticket/tickets/e07eb28b-77e8-44bf-86eb-d3b43c17a61c/ticket.toml`

<!-- ticket-index:entry id=7de9f4f0-0189-40c7-ac0a-0669e2aab57c slug=open/agent-tooling digest=762d9c14442f -->
#### [7de9f4f0] Completion-claim audit: require verified-by evidence before a ticket may reach done
- summary: Ticket 9d527ad1 was found in state `done` with a description asserting AC1-AC6 were satisfied, while the claimed emission path and `duration_ms` field did not exist in the code at all. `history.ndjso...
- ref: `.ticket/tickets/7de9f4f0-0189-40c7-ac0a-0669e2aab57c/ticket.toml`

<!-- ticket-index:entry id=8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb slug=open/agent-tooling digest=b7ea9634fba5 -->
#### [8ad2581e] Delegation quality/cost metric and self-optimization loop
- summary: Define and record the **delegation quality/cost metric** that lets the system compare expensive and cheaper models and find the cheapest model that meets our standards, closing the self-optimization ...
- ref: `.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml`

<!-- ticket-index:entry id=9c9e2edc-81fc-489e-9153-bf4ac0bf1a13 slug=open/agent-tooling digest=57416860239d -->
#### [9c9e2edc] Dynamic argument-based cost estimation in cost gate
- summary: The cost gate assigns each tool a single per-tool cost that does not vary with the call. Under the single-default model (ticket 9185d8f2), every not-yet-measured tool shares one default cost. This ig...
- ref: `.ticket/tickets/9c9e2edc-81fc-489e-9153-bf4ac0bf1a13/ticket.toml`

<!-- ticket-index:entry id=22c55989-2e69-4457-8474-9583714771e0 slug=open/agent-tooling digest=c215d379a1d7 -->
#### [22c55989] Escalation policy: triggers for higher cost class vs. user consultation
- summary: Define the escalation half of the policy: when the orchestrating agent should stop delegating downward and instead **escalate to a higher capability role / cost class**, and when it should **consult ...
- ref: `.ticket/tickets/22c55989-2e69-4457-8474-9583714771e0/ticket.toml`

<!-- ticket-index:entry id=6a47ab0f-7e42-463e-afe0-bf51b85249c9 slug=open/agent-tooling digest=9a47a4ee0fe7 -->
#### [6a47ab0f] Orchestration policy: delegation floor, verify-output enforcement, timeout playbook, sanctioned validation primitive
- summary: Session 51701334 delegated effectively at the top level (orchestrator ~$0.90) but sub-agents thrashed, driving total cost to ~$9. Four design gaps in the current orchestration guidance (`.agents/inst...
- ref: `.ticket/tickets/6a47ab0f-7e42-463e-afe0-bf51b85249c9/ticket.toml`

<!-- ticket-index:entry id=bc52c543-8c3f-49de-a08b-6791caa65523 slug=open/agent-tooling digest=4f499eb48948 -->
#### [bc52c543] Phase 2: automated auto-tuning of delegation thresholds from the quality/cost metric
- summary: Phase 2 of the self-optimization loop: **automatically auto-tune the delegation cost-class thresholds** (ticket 373072a9) from the recorded quality/cost metric (ticket 8ad2581e), removing the human-a...
- ref: `.ticket/tickets/bc52c543-8c3f-49de-a08b-6791caa65523/ticket.toml`

<!-- ticket-index:entry id=ae313316-b790-4035-888c-d9f07499adb5 slug=open/agent-tooling digest=56760f8ccfff -->
#### [ae313316] Self-optimizing delegation & escalation policy for orchestrated sessions
- summary: Extend the model price-awareness / orchestrator design (feature 445a2d76) with an explicit, self-optimizing policy that governs *when and how* an orchestrating agent delegates versus escalates inside...
- ref: `.ticket/tickets/ae313316-b790-4035-888c-d9f07499adb5/ticket.toml`

<!-- ticket-index:entry id=9b0147e3-2a8e-437d-b839-6be6edcfa2aa slug=open/agent-tooling digest=757d3730b902 -->
#### [9b0147e3] Session-coupled feedback signal integration + implementation-status audit
- summary: Integrate the **session-coupled feedback signal** into the delegation loop: the ability to flag specific problem spots and draw attention to particular scenarios, which is mostly coupled to a session.
- ref: `.ticket/tickets/9b0147e3-2a8e-437d-b839-6be6edcfa2aa/ticket.toml`

<!-- ticket-index:entry id=bac71cc5-d636-4878-b0c1-a404d2257430 slug=open/agent-tooling digest=3290d270b9e9 -->
#### [bac71cc5] [agent-guidance][epic] The instruction corpus costs every turn and did not change behaviour
- priority: `medium`
- summary: A roast of the instruction corpus, commissioned during the review of epic `79c4ac3e`, found that the guidance layer is itself a material per-turn cost and that it demonstrably failed to change behavi...
- ref: `.ticket/tickets/bac71cc5-d636-4878-b0c1-a404d2257430/ticket.toml`

<!-- ticket-index:entry id=7200f269-fde3-4efc-9edd-208c36d6189e slug=open/agent-tooling digest=1226b6979329 -->
#### [7200f269] [agent-tooling] Add an explicit gate-mode marker for pre-dispatch quality-gate agent invocations
- priority: `medium`
- summary: `explore.agent.md` now serves two roles: the general-purpose exploration
- ref: `.ticket/tickets/7200f269-fde3-4efc-9edd-208c36d6189e/ticket.toml`

<!-- ticket-index:entry id=0231bfca-d9ce-4d42-83f0-2ce7befa0f3e slug=open/agent-tooling digest=9b1ea5a393ca -->
#### [0231bfca] [delegation-cost] Propose an MCP-over-shell enforcement mechanism from the 77eb143b classifier trend
- priority: `medium`
- summary: `77eb143b` shipped the classifier and measured — not enforced — CLI-over-MCP shell usage. On the two checked-in baseline sessions (`3e9bc20b…`, `41966513…`), replayed through `compute_delegation_cost...
- ref: `.ticket/tickets/0231bfca-d9ce-4d42-83f0-2ce7befa0f3e/ticket.toml`

<!-- ticket-index:entry id=79c4ac3e-fd53-48bf-babb-43d27555c4bd slug=open/agent-tooling digest=1eefbe80e71b -->
#### [79c4ac3e] [delegation-cost][epic] Sub-agent delegation is more expensive than the orchestration it replaces
- priority: `high`
- summary: Two orchestrated sessions ran in parallel on 2026-07-27 and were unexpectedly expensive. The cost was **not** in the top-level orchestrator — it made **zero** tool calls in both sessions and only pla...
- ref: `.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml`

<!-- ticket-index:entry id=94c61173-254d-40b6-9ee5-439050ce24a7 slug=open/agent-tooling digest=d70697fcd4e0 -->
#### [94c61173] [delegation-cost][model-routing] Verify runSubagent model-tier resolution and override-audit recording empirically
- priority: `medium`
- summary: Ticket 66acb737 declared `model:` frontmatter on all 16 `.agents/agents/*.agent.md` templates and documented (in model-routing.instructions.md, "Per-Template `model:` Declaration") two behavioral rul...
- ref: `.ticket/tickets/94c61173-254d-40b6-9ee5-439050ce24a7/ticket.toml`

<!-- ticket-index:entry id=4aa13ba7-5bdb-4ac6-8581-2f86427f0980 slug=open/agent-tooling digest=fa7b7d19944b -->
#### [4aa13ba7] [mcp-cost-gate] Tool-call telemetry is never collected: COST_GATE_TELEMETRY_LOG unset, rollup file absent, no aggregator
- priority: `high`
- summary: Ticket 9d527ad1 delivered per-tool-call token-load *measurement* in `mcp-cost-gate`, but nothing ever turned collection on. Every telemetry record produced in production since it landed has been sile...
- ref: `.ticket/tickets/4aa13ba7-5bdb-4ac6-8581-2f86427f0980/ticket.toml`

<!-- ticket-index:entry id=ea52bd6f-aa48-43f5-9228-0bff7190abf8 slug=open/agent-tooling digest=bd061facff77 -->
#### [ea52bd6f] [terminal] Add human-owned observer terminal sessions
- priority: `high`
- summary: Provide a secure observer-terminal capability in which a human enters terminal
- ref: `.ticket/tickets/ea52bd6f-aa48-43f5-9228-0bff7190abf8/ticket.toml`

<!-- ticket-index:entry id=0dd23fe6-6892-4d21-9927-4a81584dc77a slug=open/agent-tooling digest=eb55a913162a -->
#### [0dd23fe6] [token-efficiency] Audit execute MCP tools for terminal reuse and input continuation features
- priority: `medium`
- summary: Audit the currently active execute-style MCP surfaces and adjacent terminal-execution tooling to determine whether they already support terminal reuse, follow-up input, resumable execution, or persis...
- ref: `.ticket/tickets/0dd23fe6-6892-4d21-9927-4a81584dc77a/ticket.toml`

<!-- ticket-index:entry id=9b9df133-d809-4900-b56a-afae4efcdd08 slug=open/agent-tooling digest=0bb97c55163c -->
#### [9b9df133] [token-efficiency] Track token-efficient agent tooling rollout
- priority: `high`
- summary: Goal: coordinate the workspace-wide token-efficiency rollout for agent-facing tooling and guidance.
- ref: `.ticket/tickets/9b9df133-d809-4900-b56a-afae4efcdd08/ticket.toml`

<!-- ticket-index:entry id=b8ce7cd8-50d0-4233-8584-3af2a27c07d1 slug=open/agent-tooling digest=4ae975cf4bdf -->
#### [b8ce7cd8] file editing: context-anchored differential patching tool suite (api + cli + mcp)
- summary: Crate name**: `edit-api` (api crate); `edit-cli` (CLI transport); `edit-mcp` (MCP transport).
- ref: `.ticket/tickets/b8ce7cd8-50d0-4233-8584-3af2a27c07d1/ticket.toml`

<!-- ticket-index:entry id=bd71ecc7-4631-407c-a156-d1d77de2ca33 slug=open/agent-tooling digest=2f19f904bc82 -->
#### [bd71ecc7] repo-wide search: bounded counts-first search tool suite (api + cli + mcp)
- summary: Crate name**: `search-api` (api crate); `search-cli` (CLI transport); `search-mcp` (MCP transport).
- ref: `.ticket/tickets/bd71ecc7-4631-407c-a156-d1d77de2ca33/ticket.toml`


### Component: agent-workflow

<!-- ticket-index:entry id=f18e6885-c193-4159-82c5-d164e470437b slug=open/agent-workflow digest=3232f5bbfc40 -->
#### [f18e6885] Add spec-system guidance and Spec Agent rule targets
- priority: `medium`
- summary: Add generated spec-system guidance instructions and a Spec Agent workflow so spec creation/update work consistently links tests, tickets, and related specs. Update rule targets and canonical rule ent...
- ref: `.ticket/tickets/f18e6885-c193-4159-82c5-d164e470437b/ticket.toml`

<!-- ticket-index:entry id=2c019bce-08a8-4d70-9e17-82f42e69fdcc slug=open/agent-workflow digest=ce4be0f8d398 -->
#### [2c019bce] [agent-workflow] Add an explicit pre-create entity gate so agents apply deltas instead of duplicating tickets and specs
- priority: `high`
- summary: Multiple orchestrated agents created the same tickets in sequence: an orchestrator reviewed an existing ticket track, then a dispatched sub-agent duplicated many of the same tickets one-to-one. Two c...
- ref: `.ticket/tickets/2c019bce-08a8-4d70-9e17-82f42e69fdcc/ticket.toml`

<!-- ticket-index:entry id=4ca4ce83-3baa-4676-8001-bc72b2c99352 slug=open/agent-workflow digest=826b2af21d6a -->
#### [4ca4ce83] [agent-workflow] Correct stale cost-gate caller_model behavior described in orchestration instruction docs
- priority: `medium`
- summary: Two instruction files still describe cost-gate behavior that does not match the code, and the drift predates ticket `32067e83`:
- ref: `.ticket/tickets/4ca4ce83-3baa-4676-8001-bc72b2c99352/ticket.toml`

<!-- ticket-index:entry id=36b04541-9bf1-433c-b100-77c3b7cb5855 slug=open/agent-workflow digest=c8116e877b1f -->
#### [36b04541] [agent-workflow] Interruption-recovery instructions and a resume-interrupted prompt template
- priority: `high`
- summary: A sub-agent's run was interrupted. When the orchestrator was told to continue, it assumed the sub-agent had finished and moved on to the next step, silently dropping the unfinished work.
- ref: `.ticket/tickets/36b04541-9bf1-433c-b100-77c3b7cb5855/ticket.toml`

<!-- ticket-index:entry id=b7a3c75e-a97a-4864-95db-cbc14ec2b829 slug=open/agent-workflow digest=8519356ea763 -->
#### [b7a3c75e] [agent-workflow] Make research a precondition of ticket authoring in AGENTS.md task routing and tickets.prompt.md
- priority: `high`
- summary: Ticket authoring currently can run **before** research. In an observed incident, a PDF-domain ticket track locked in `printpdf` as the backend crate; research performed later showed this was the wron...
- ref: `.ticket/tickets/b7a3c75e-a97a-4864-95db-cbc14ec2b829/ticket.toml`

<!-- ticket-index:entry id=fe9019df-1dc9-47a7-864e-334588d00820 slug=open/agent-workflow digest=4b797144db8f -->
#### [fe9019df] [agent-workflow] Require review and audit agents to re-verify every file path they cite
- priority: `high`
- summary: Two mis-attributed citations occurred in the same implementation track, both from agents asked to review rather than implement:
- ref: `.ticket/tickets/fe9019df-1dc9-47a7-864e-334588d00820/ticket.toml`

<!-- ticket-index:entry id=2558a279-8819-4682-8db5-c2a4aa30aa0e slug=open/agent-workflow digest=35baea765edf -->
#### [2558a279] [epic] Workflow and tooling reliability: update semantics, handoff step graph, delta-not-duplicate, interruption recovery, cost-gate model resolution
- priority: `high`
- summary: Tracker for eight workflow/tooling defects reported by the user and verified against repo state on 2026-07-28.
- ref: `.ticket/tickets/2558a279-8819-4682-8db5-c2a4aa30aa0e/ticket.toml`


### Component: agents

<!-- ticket-index:entry id=3c3b42f3-1412-4c73-a531-4567add92a33 slug=open/agents digest=03c3fe8863bd -->
#### [3c3b42f3] [agents] C1: Define 14-role taxonomy + deterministic routing table in AGENTS.md
- summary: Routing today is semantic-similarity guessing; AGENTS.md routes by prompt text, never by agent template or role. No canonical role taxonomy exists to disambiguate the 15 colliding template pairs.
- ref: `.ticket/tickets/3c3b42f3-1412-4c73-a531-4567add92a33/ticket.toml`

<!-- ticket-index:entry id=1c850547-c76a-4d65-83c6-133289552661 slug=open/agents digest=b8e331a078fa -->
#### [1c850547] [agents] C2: Author 8 consolidated templates + new Telemetry template (ec3b13f1-compliant)
- summary: agent templates + 24 prompts have 15 colliding pairs and 10 orphan files. Need 8 merged templates plus one new standalone Telemetry template, each declaring an explicit MCP tool list or a wildcard + ...
- ref: `.ticket/tickets/1c850547-c76a-4d65-83c6-133289552661/ticket.toml`

<!-- ticket-index:entry id=fb241a6c-165f-4a5e-bad7-9ac0ab63348b slug=open/agents digest=1be73371944d -->
#### [fb241a6c] [agents] C3: Delete/merge superseded templates and prompts, preserving load-bearing slash-commands
- summary: After C2 lands the 9 new templates, the 17 superseded `.agent.md` files and the folded-in `.prompt.md` files remain, duplicating guidance and re-introducing the exact collisions this epic removes.
- ref: `.ticket/tickets/fb241a6c-165f-4a5e-bad7-9ac0ab63348b/ticket.toml`

<!-- ticket-index:entry id=46d423d8-0a7e-4dc8-b701-b5c2768f34f7 slug=open/agents digest=d2a4e27b7d26 -->
#### [46d423d8] [agents] C4: Add R2 global state-overview mode to Explorer (board_show + next_tickets + list_tickets)
- summary: R2 (global state overview: cross-workspace board, state distribution, highest-priority next) has no template today. It is folded into Explorer per the merge map but needs its own concrete workflow, n...
- ref: `.ticket/tickets/46d423d8-0a7e-4dc8-b701-b5c2768f34f7/ticket.toml`

<!-- ticket-index:entry id=ce9edc5b-cb27-4cb8-8802-68a8714c686c slug=open/agents digest=cb14cf9a775e -->
#### [ce9edc5b] [agents] C5: Compress 5 orchestration instruction files (16,898 tokens) into one
- summary: orchestration instruction files alone total 16,898 tokens of unconditionally-relevant guidance. Once C1 (routing contract) and C2 (consolidated templates) land, several of these files' rules become r...
- ref: `.ticket/tickets/ce9edc5b-cb27-4cb8-8802-68a8714c686c/ticket.toml`

<!-- ticket-index:entry id=ea80712b-3506-4b8f-bb36-fc2618aa7b82 slug=open/agents digest=47a42aef7313 -->
#### [ea80712b] [agents] C6: Validation — prompt-replay/routing determinism check for role resolution
- summary: The epic's core claim is that request->role routing becomes deterministic (first-match-wins). This must be validated, not assumed, once the routing table (C1), templates (C2), and deletions (C3) are ...
- ref: `.ticket/tickets/ea80712b-3506-4b8f-bb36-fc2618aa7b82/ticket.toml`

<!-- ticket-index:entry id=c608f5ac-cb7f-424f-ae99-22e75a9477d7 slug=open/agents digest=68a7749e2a01 -->
#### [c608f5ac] [agents] Consolidate 17 agent templates + 24 prompts into 8 role-based templates with deterministic routing
- summary: Guidance corpus: 101 files, ~103,750 tokens. `.agents/agents/**` = 24,168 tokens; only 3,641 tokens load unconditionally per session; 5 orchestration instruction files alone = 16,898 tokens. 15 colli...
- ref: `.ticket/tickets/c608f5ac-cb7f-424f-ae99-22e75a9477d7/ticket.toml`


### Component: audit

<!-- ticket-index:entry id=afb71e41-95a2-4881-bf6c-7e7e3c96056d slug=open/audit digest=9de867c94d95 -->
#### [afb71e41] [workflow-tools][per-tool] Extract audit tool as a single `audit` domain crate (api + transport bins)
- priority: `high`
- summary: Phase B. Extract the audit tool into its own `audit` repository (owner mankinskin), built as a single `audit` domain crate per contract `0da6894c`: the crate lib re-exports the internal `audit-api` c...
- ref: `.ticket/tickets/afb71e41-95a2-4881-bf6c-7e7e3c96056d/ticket.toml`


### Component: audit-api

<!-- ticket-index:entry id=e5e3b293-cd5d-4e19-875f-13e8f486bf92 slug=open/audit-api digest=bf8bff22c45f -->
#### [e5e3b293] Fix audit-api rule_overlap DuplicateSlug test failure and correct d1b3a6c9 validation evidence
- priority: `medium`
- summary: Review of ticket d1b3a6c9 found `cargo test -p audit-api` currently reports 13 passed / 1 failed, not the "14 passed, 0 failed" claimed in d1b3a6c9's description:
- ref: `memory-api/.ticket/tickets/e5e3b293-cd5d-4e19-875f-13e8f486bf92/ticket.toml`

<!-- ticket-index:entry id=632974d1-ce70-446a-b210-068840041116 slug=open/audit-api digest=06d757a69388 -->
#### [632974d1] [audit-mcp][audit-http] Workspace-resolution parity — nested-root awareness + pure transport
- priority: `medium`
- summary: Follow-up after the ticket-domain first run. Adopt the shared memory-api resolver + pure-transport pattern for the audit transports.
- ref: `memory-api/.ticket/tickets/632974d1-ce70-446a-b210-068840041116/ticket.toml`

<!-- ticket-index:entry id=0d601ffd-73c9-4b1e-8f6f-05e32fd8c0ef slug=open/audit-api digest=90c60821fb42 -->
#### [0d601ffd] [memory-index] Audit index auto-rerun on file changes via git hooks
- priority: `medium`
- summary: Make the audit index responsive to file changes so that when relevant files are modified, a git hook automatically re-runs the audit and updates `.audit/README.md`.
- ref: `.ticket/tickets/0d601ffd-73c9-4b1e-8f6f-05e32fd8c0ef/ticket.toml`

<!-- ticket-index:entry id=1aa6119a-1059-4412-9d21-77c125608d22 slug=open/audit-api digest=114e5d9c9dea -->
#### [1aa6119a] [memory-index] Audit index: per-finding entries sorted by severity, clickable file links, collapsible severity sections
- priority: `medium`
- summary: Improve the `.audit/README.md` generated index so findings are listed per-file at severity-ordered entries with clickable file paths, instead of aggregated per-category summary blocks.
- ref: `.ticket/tickets/1aa6119a-1059-4412-9d21-77c125608d22/ticket.toml`


### Component: cli

<!-- ticket-index:entry id=c01ace60-4794-48fd-a22c-f4745ad2ca3c slug=open/cli digest=55eecdf690b9 -->
#### [c01ace60] Plan: end-to-end test registry
- summary: tags: `#plan` `#context-trace` `#context-search` `#context-insert` `#algorithm` `#debugging` `#testing` `#api` `#performance`
- ref: `.ticket/tickets/c01ace60-4794-48fd-a22c-f4745ad2ca3c/ticket.toml`


### Component: context-api

<!-- ticket-index:entry id=974e6e37-f414-4ac3-8f5c-e867c709b775 slug=open/context-api digest=37d1a6a4dd1d -->
#### [974e6e37] Design: Instruction Language DSL for graph operations
- summary: tags: `#context-api` `#design` `#instruction-language` `#dsl` `#future`
- ref: `.ticket/tickets/974e6e37-f414-4ac3-8f5c-e867c709b775/ticket.toml`

<!-- ticket-index:entry id=b786f1f5-8d04-4586-8e30-a532069bbd81 slug=open/context-api digest=083d294e8ce6 -->
#### [b786f1f5] Plan: CLI read UX improvement — ReadSequence, ReadFile, REPL parsing
- summary: tags: `#plan` `#context-api` `#context-cli` `#ux` `#read` `#cli` `#repl`
- ref: `.ticket/tickets/b786f1f5-8d04-4586-8e30-a532069bbd81/ticket.toml`

<!-- ticket-index:entry id=0727b7dd-b90b-4edb-8c16-2d6220506585 slug=open/context-api digest=495581d8bd2c -->
#### [0727b7dd] Plan: Context API — master multi-phase architecture plan
- summary: tags: `#context-api` `#architecture` `#multi-phase` `#api-design` `#plan`
- ref: `.ticket/tickets/0727b7dd-b90b-4edb-8c16-2d6220506585/ticket.toml`


### Component: context-editor

<!-- ticket-index:entry id=e7da478e-b18e-4551-a385-d39e81d09a41 slug=open/context-editor digest=aaba4f9d64b5 -->
#### [e7da478e] Plan: context-editor — unified GPU-accelerated 3D world editor tool
- priority: `critical`
- summary: A single-binary, GPU-accelerated tool that merges the log-viewer, doc-viewer,
- ref: `.ticket/tickets/e7da478e-b18e-4551-a385-d39e81d09a41/ticket.toml`

<!-- ticket-index:entry id=1b65d658-07d0-4d31-881b-6111321b5752 slug=open/context-editor digest=8878baad76ce -->
#### [1b65d658] SDF Item Cutting: CSG Shader Subtraction, Cut Particles & Liquid Glass Impact Feedback
- priority: `high`
- ref: `.ticket/tickets/1b65d658-07d0-4d31-881b-6111321b5752/ticket.toml`

<!-- ticket-index:entry id=8922e00c-98ac-4604-ae01-29acca066b61 slug=open/context-editor digest=9b1c2f8a5b19 -->
#### [8922e00c] [context-editor] Epic: Direct SVO Ray Marching — Replace Tiled Forward+ Pipeline
- priority: `critical`
- summary: The current rendering pipeline uses a multi-stage GPU-driven splatting approach:
- ref: `.ticket/tickets/8922e00c-98ac-4604-ae01-29acca066b61/ticket.toml`

<!-- ticket-index:entry id=febe05b2-ab03-4309-9d84-39aae471e27a slug=open/context-editor digest=30d87754d399 -->
#### [febe05b2] [context-editor][SVO-RM] Phase 1a: World-to-SVO Transform and Layout Validation
- summary: The ray march shader needs to transform world-space rays into the SVO's normalized $[0,1]^3$ coordinate space. Currently, `compute_node_positions()` outputs world-space centers and half-extents, but ...
- ref: `context-stack/tools/context-editor/.ticket/tickets/febe05b2-ab03-4309-9d84-39aae471e27a/ticket.toml`

<!-- ticket-index:entry id=9ef831d0-0f1d-46db-88bb-e537a37b9606 slug=open/context-editor digest=e8d1cf9b00aa -->
#### [9ef831d0] [context-editor][SVO-RM] Phase 1b: Core SVO Ray March Compute Shader
- summary: This is the centrepiece of the rendering rewrite. We need a compute shader that, for each pixel, casts a ray through the world-space SVO and finds the nearest voxel intersection by hierarchical trave...
- ref: `context-stack/tools/context-editor/.ticket/tickets/9ef831d0-0f1d-46db-88bb-e537a37b9606/ticket.toml`

<!-- ticket-index:entry id=22801e4f-36bd-43fc-b765-6e456b2bc63a slug=open/context-editor digest=d507aad17ec3 -->
#### [22801e4f] [context-editor][SVO-RM] Phase 2a: SDF Blending and Front-to-Back Alpha Compositing
- summary: Phase 1b establishes basic ray-AABB leaf hits with box SDF. This ticket refines the SDF evaluation to support:
- ref: `context-stack/tools/context-editor/.ticket/tickets/22801e4f-36bd-43fc-b765-6e456b2bc63a/ticket.toml`

<!-- ticket-index:entry id=8c2f1575-e704-44cb-bfc0-1f908bfc4855 slug=open/context-editor digest=338b48f17100 -->
#### [8c2f1575] [context-editor][SVO-RM] Phase 2b: Secondary Rays -- Reflections, Refractions, Shadows
- summary: The old pipeline required special-case code for glass panels (refraction, chromatic aberration, caustics) as a pre-pass before the splat loop. With SVO ray marching, secondary rays (reflections, refr...
- ref: `context-stack/tools/context-editor/.ticket/tickets/8c2f1575-e704-44cb-bfc0-1f908bfc4855/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=c2409055-c489-441b-9a60-f3b3aa608522 slug=open/context-engine digest=345c82ab09ea -->
#### [c2409055] [memory-index] Memory workspace DAG indexing
- priority: `medium`
- summary: Build a workspace summary capability locally inside each tool/domain (e.g. ticket-cli, spec-cli, rule-cli). Under this contract, each store folder (like `.ticket/` or `.spec/`) acts as the root ancho...
- ref: `.ticket/tickets/c2409055-c489-441b-9a60-f3b3aa608522/ticket.toml`

<!-- ticket-index:entry id=fe098673-f7fa-43ba-af66-047578861596 slug=open/context-engine digest=7dcbafca7800 -->
#### [fe098673] [memory-index] Roadmap: sequential implementation of domain-owned store indexes
- priority: `high`
- summary: Provide one canonical roadmap tracker for the memory-index work so implementation proceeds in a single explicit order instead of a loose set of related tickets.
- ref: `.ticket/tickets/fe098673-f7fa-43ba-af66-047578861596/ticket.toml`

<!-- ticket-index:entry id=2a3ad242-8c01-4779-94ec-9e4d5595f538 slug=open/context-engine digest=c6f728aec9e9 -->
#### [2a3ad242] [sandbox-v1][impl] memory-stack traceability, archive linking, and runbook docs
- priority: `high`
- summary: Record workflow metadata in ticket, spec, and doc owned surfaces.
- ref: `.ticket/tickets/2a3ad242-8c01-4779-94ec-9e4d5595f538/ticket.toml`

<!-- ticket-index:entry id=0884ab64-e54d-4f9c-abbf-de61990773eb slug=open/context-engine digest=35f86a426c4e -->
#### [0884ab64] [sandbox-v1][impl] session execution, per-session MCP, and artifact capture
- priority: `high`
- summary: Copilot completions client and session runner.
- ref: `.ticket/tickets/0884ab64-e54d-4f9c-abbf-de61990773eb/ticket.toml`

<!-- ticket-index:entry id=5ed70069-b080-4a95-8dc5-ddf495007bdd slug=open/context-engine digest=c8b218f2b368 -->
#### [5ed70069] [sandbox-v1][impl] validation and hardening gates
- priority: `high`
- summary: Integration harness for end-to-end Firecracker-backed sandbox execution.
- ref: `.ticket/tickets/5ed70069-b080-4a95-8dc5-ddf495007bdd/ticket.toml`

<!-- ticket-index:entry id=6bebc161-63e6-4177-9958-0e36ffcd92bc slug=open/context-engine digest=c940eb13d58b -->
#### [6bebc161] [sandbox-v1][track] functional sandbox orchestration implementation
- priority: `high`
- summary: Track completion of the v1 sandbox orchestration implementation after planning and design are complete.
- ref: `.ticket/tickets/6bebc161-63e6-4177-9958-0e36ffcd92bc/ticket.toml`

<!-- ticket-index:entry id=92741a14-d718-4f49-8843-040432a3d8da slug=open/context-engine digest=274cdfadbdf1 -->
#### [92741a14] [workflow-tools][context-engine] Reframe context-engine as a consuming example with workflow-tools as an installed dependency
- priority: `high`
- summary: Phase E. Reframe context-engine as an instantiated example of a target environment: it retains only the context-stack (context-api/insert/read/search/trace) plus its own generated artifacts, and cons...
- ref: `.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml`


### Component: context-insert

<!-- ticket-index:entry id=a4210ebf-208c-48b7-814f-da0d3269e236 slug=open/context-insert digest=f6d1aadbc8e0 -->
#### [a4210ebf] Plan: integration test remediation — RC-1, RC-2, RC-3 fix rounds
- summary: tags: `#plan` `#testing` `#integration` `#context-api` `#context-read` `#context-insert` `#bug-fix` `#refactoring`
- ref: `.ticket/tickets/a4210ebf-208c-48b7-814f-da0d3269e236/ticket.toml`


### Component: context-mcp

<!-- ticket-index:entry id=61f78a57-6896-4ad7-9daa-0e9e805aa397 slug=open/context-mcp digest=4373bb7e161f -->
#### [61f78a57] Plan: Context API phase 3.1 — per-command tracing log capture + log query tools
- summary: tags: `#context-api` `#phase3.1` `#tracing` `#logs` `#cli` `#mcp` `#jq`
- ref: `.ticket/tickets/61f78a57-6896-4ad7-9daa-0e9e805aa397/ticket.toml`


### Component: context-read

<!-- ticket-index:entry id=3125d4c5-5eb1-48a0-a935-a5d686408a72 slug=open/context-read digest=21926c13cdf7 -->
#### [3125d4c5] Bug: context-read crate 28 compilation errors — API mismatch with context-trace
- summary: tags: `#bug-report` `#context-trace` `#context-read` `#debugging` `#refactoring` `#api`
- ref: `.ticket/tickets/3125d4c5-5eb1-48a0-a935-a5d686408a72/ticket.toml`

<!-- ticket-index:entry id=6432858e-0e7c-4a0c-bc59-96b04f932391 slug=open/context-read digest=65674d16cdc6 -->
#### [6432858e] Plan: context-read completion — text indexing crate
- summary: tags: `#plan` `#context-read` `#algorithm` `#cursor` `#expansion` `#overlap`
- ref: `.ticket/tickets/6432858e-0e7c-4a0c-bc59-96b04f932391/ticket.toml`

<!-- ticket-index:entry id=668743ea-497b-46a2-b7f7-f136684acc8c slug=open/context-read digest=62572ffa6100 -->
#### [668743ea] Plan: context-read restructure — migrate bands/, delete dead code, rename pipeline/
- summary: tags: `#plan` `#context-read` `#architecture` `#restructuring` `#api` `#refactoring`
- ref: `.ticket/tickets/668743ea-497b-46a2-b7f7-f136684acc8c/ticket.toml`

<!-- ticket-index:entry id=fe81b165-113f-43fc-87c2-dc7f44170152 slug=open/context-read digest=a69bc3530e85 -->
#### [fe81b165] Tracker: context-read final test remediation for context-stack integration
- summary: Remediate the remaining `context-read` crate test failures that block full `context-stack` integration, while aligning the ticket set and spec language with the clarified read algorithm.
- ref: `.ticket/tickets/fe81b165-113f-43fc-87c2-dc7f44170152/ticket.toml`

<!-- ticket-index:entry id=c6cc7d5a-dcfb-4ae0-bfc7-d8682462503b slug=open/context-read digest=f797daa4aeb6 -->
#### [c6cc7d5a] [Bug] context-read normalization of embedded paths is inconsistent across API layers
- summary: The current failing assertions around infix and overlap matches assume that lower-level path results must always normalize to an `EntireRoot` materialized token.
- ref: `.ticket/tickets/c6cc7d5a-dcfb-4ae0-bfc7-d8682462503b/ticket.toml`

<!-- ticket-index:entry id=05925875-000c-4af3-913e-e4121ab223ca slug=open/context-read digest=cc3cd95d6bf2 -->
#### [05925875] [Bug] context-read overlap-step materialization breaks retention policy or graph invariants
- summary: `context-read` should materialize graph state after each overlap expansion step to keep progression safe, but that materialized state must still obey retention policy and structural invariants.
- ref: `.ticket/tickets/05925875-000c-4af3-913e-e4121ab223ca/ticket.toml`

<!-- ticket-index:entry id=f8dfcd09-0e29-4ee6-a61f-de64aed1098f slug=open/context-read digest=f32083ddde42 -->
#### [f8dfcd09] [context-read] Revisit prior roots when new overlap subparts materialize
- priority: `high`
- summary: The current pipeline materializes overlap products locally but does not reliably revisit already-known roots when new subparts appear later. That is why `bcdea` still misses `[bc, dea]` after later r...
- ref: `.ticket/tickets/f8dfcd09-0e29-4ee6-a61f-de64aed1098f/ticket.toml`


### Component: context-search

<!-- ticket-index:entry id=346573c1-2711-407d-a50f-a2cbce53b965 slug=open/context-search digest=3a723774f1e7 -->
#### [346573c1] Bug: TraceCache root token mismatch causes insert panics
- summary: tags: `#context-search` `#context-insert` `#TraceCache` `#InitInterval` `#panic` `#critical`
- ref: `.ticket/tickets/346573c1-2711-407d-a50f-a2cbce53b965/ticket.toml`

<!-- ticket-index:entry id=d265e603-feac-4cec-86e0-a323acd990b1 slug=open/context-search digest=a4d1bfb6b5fc -->
#### [d265e603] Plan: search event refactoring — PathNode, IntoTransition, tentative root
- summary: tags: `#plan` `#refactoring` `#visualization` `#events` `#search`
- ref: `.ticket/tickets/d265e603-feac-4cec-86e0-a323acd990b1/ticket.toml`


### Component: context-tasks

<!-- ticket-index:entry id=4470de7b-8c04-4c06-ae29-af411ade5db5 slug=open/context-tasks digest=0a730a8c59dc -->
#### [4470de7b] Design backlog: stable dependency semantics with state-derived readiness
- priority: `backlog`
- ref: `.ticket/tickets/4470de7b-8c04-4c06-ae29-af411ade5db5/ticket.toml`


### Component: context-trace

<!-- ticket-index:entry id=619e49fc-951a-4e14-bc33-e831525c3002 slug=open/context-trace digest=f94adf522c52 -->
#### [619e49fc] Plan: fine-grained locking design for context-trace
- summary: tags: `#plan` `#context-trace` `#context-search` `#context-insert` `#context-read` `#debugging` `#testing` `#refactoring` `#api`
- ref: `.ticket/tickets/619e49fc-951a-4e14-bc33-e831525c3002/ticket.toml`

<!-- ticket-index:entry id=19990e37-b5c2-41bc-af39-d649559a8885 slug=open/context-trace digest=edbff6511b8e -->
#### [19990e37] Plan: graph diff command — diff two graph states
- summary: tags: `#plan` `#cli` `#api` `#context-api` `#context-cli` `#graph-diff` `#comparison`
- ref: `.ticket/tickets/19990e37-b5c2-41bc-af39-d649559a8885/ticket.toml`

<!-- ticket-index:entry id=164549c4-1050-4fb6-9bc0-57077cbf2667 slug=open/context-trace digest=9920a037d77c -->
#### [164549c4] Plan: position-annotated paths — path structures with position metadata
- summary: tags: `#plan` `#context-trace` `#context-search` `#debugging` `#testing` `#performance`
- ref: `.ticket/tickets/164549c4-1050-4fb6-9bc0-57077cbf2667/ticket.toml`

<!-- ticket-index:entry id=0d61b9df-544d-453c-9a8f-68078ec5163f slug=open/context-trace digest=9694324694ce -->
#### [0d61b9df] Plan: selective partition merge — avoid full-graph merge
- summary: tags: `#plan` `#context-insert` `#algorithm` `#testing` `#api`
- ref: `.ticket/tickets/0d61b9df-544d-453c-9a8f-68078ec5163f/ticket.toml`

<!-- ticket-index:entry id=f8afe331-41e2-4563-ad6a-456837afb1f8 slug=open/context-trace digest=f8cae7e3f59f -->
#### [f8afe331] [Bug] dedup_atoms_not_duplicated: regression panic in vertex/data/children.rs
- summary: Regression** — this test was previously passing. It now panics.
- ref: `.ticket/tickets/f8afe331-41e2-4563-ad6a-456837afb1f8/ticket.toml`

<!-- ticket-index:entry id=f41f08a8-fad9-4a20-b3a4-58bc1cc4d6ef slug=open/context-trace digest=0a36bd231287 -->
#### [f41f08a8] [Bug] edge_repeated_single_char: panic — pattern width mismatch in T2w4 token (RC-3)
- summary: The **public `context-cli` integration test now passes** because `context-api::read_sequence` was redirected through the corrected public exact-root insert path before reading the root back.
- ref: `.ticket/tickets/f41f08a8-fad9-4a20-b3a4-58bc1cc4d6ef/ticket.toml`


### Component: context-trace-macros

<!-- ticket-index:entry id=35a54e96-772a-4a44-9523-ec11b81d8a4f slug=open/context-trace-macros digest=1545ea0c6ea2 -->
#### [35a54e96] Add context-trace-macros compile-fail tests
- priority: `high`
- summary: Add compile-fail coverage for `context-stack/context-trace-macros`.
- ref: `.ticket/tickets/35a54e96-772a-4a44-9523-ec11b81d8a4f/ticket.toml`


### Component: doc

<!-- ticket-index:entry id=a57886e4-e076-4542-8b16-87dde43d62b0 slug=open/doc digest=e952b8b6e465 -->
#### [a57886e4] [workflow-tools][per-tool] Extract doc tool as a single `doc` domain crate (api + transport bins) + viewer frontend
- priority: `high`
- summary: Phase B. Extract the doc tool into its own `doc` repository (owner mankinskin), built as a single `doc` domain crate per contract `0da6894c`: the crate lib re-exports the internal `doc-api` crate and...
- ref: `.ticket/tickets/a57886e4-e076-4542-8b16-87dde43d62b0/ticket.toml`


### Component: doc-cli

<!-- ticket-index:entry id=ad9f6e52-2147-4b25-be2c-9e59dd58a876 slug=open/doc-cli digest=eafd00bfbac2 -->
#### [ad9f6e52] [doc-cli] Add CLI surface for doc-api
- priority: `high`
- summary: Create `doc-cli` as the CLI interface for `doc-api`.
- ref: `.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876/ticket.toml`


### Component: doc-viewer

<!-- ticket-index:entry id=0515479f-a5c2-47c6-b8c2-3961dfa6dcf7 slug=open/doc-viewer digest=ce3e3dc0e6fa -->
#### [0515479f] Plan: MCP crate docs — extend MCP server for crate API documentation
- summary: tags: `#plan` `#context-trace` `#debugging` `#api`
- ref: `.ticket/tickets/0515479f-a5c2-47c6-b8c2-3961dfa6dcf7/ticket.toml`


### Component: docs

<!-- ticket-index:entry id=88cd4cb8-8b31-48b6-9c13-28522d939b0b slug=open/docs digest=fef40cb3b891 -->
#### [88cd4cb8] Plan: dungeon crawler skill docs (4 skill documents in docs/skills/)
- summary: tags: `#plan` `#documentation` `#skills` `#dungeon-crawler` `#hypergraph` `#educational` `#external-facing`
- ref: `.ticket/tickets/88cd4cb8-8b31-48b6-9c13-28522d939b0b/ticket.toml`


### Component: feedback

<!-- ticket-index:entry id=bdd35984-235a-45eb-971d-a57719bf2c74 slug=open/feedback digest=033703d8f9f4 -->
#### [bdd35984] [workflow-tools][per-tool] Extract feedback tool as a single `feedback` domain crate (api + transport bins)
- priority: `high`
- summary: Phase B. Extract the feedback tool into its own `feedback` repository (owner mankinskin), built as a single `feedback` domain crate per contract `0da6894c`: the crate lib re-exports the internal `fee...
- ref: `.ticket/tickets/bdd35984-235a-45eb-971d-a57719bf2c74/ticket.toml`


### Component: feedback-api

<!-- ticket-index:entry id=274c3fcf-1502-41b3-af5b-cd1e7e599e79 slug=open/feedback-api digest=f96aaa508b65 -->
#### [274c3fcf] Author shared feedback API specification
- priority: `high`
- summary: Author the repository’s missing shared feedback API specification.
- ref: `.ticket/tickets/274c3fcf-1502-41b3-af5b-cd1e7e599e79/ticket.toml`

<!-- ticket-index:entry id=020503a4-5b93-426d-9a05-6ff9e1f60276 slug=open/feedback-api digest=477938649141 -->
#### [020503a4] Test shared feedback API contract
- priority: `high`
- summary: Test the shared feedback API contract across feedback-cli and feedback-mcp.
- ref: `.ticket/tickets/020503a4-5b93-426d-9a05-6ff9e1f60276/ticket.toml`


### Component: fs-api

<!-- ticket-index:entry id=40da867e-4ccb-496c-bfda-f8e9d26db75c slug=open/fs-api digest=c1a91d0b9545 -->
#### [40da867e] Define symlink confinement for spec 58a1d32c
- priority: `high`
- summary: Refine spec `58a1d32c` with symlink confinement and race semantics.
- ref: `.ticket/tickets/40da867e-4ccb-496c-bfda-f8e9d26db75c/ticket.toml`

<!-- ticket-index:entry id=10f3fe34-5564-40b9-b5e0-9f3e231b2993 slug=open/fs-api digest=bd5af9bb025f -->
#### [10f3fe34] Test filesystem symlink and TOCTOU matrix
- priority: `high`
- summary: Test fs-api, fs-cli, and fs-mcp symlink and TOCTOU behavior.
- ref: `.ticket/tickets/10f3fe34-5564-40b9-b5e0-9f3e231b2993/ticket.toml`


### Component: install-ctl

<!-- ticket-index:entry id=fde76de2-1543-4eca-9d43-b40f24ec8241 slug=open/install-ctl digest=202c83950cb8 -->
#### [fde76de2] Make viewer frontend assets checkout-local
- priority: `high`
- summary: Eliminate the global frontend asset cache so builds and served assets are isolated per checkout/worktree.
- ref: `.ticket/tickets/fde76de2-1543-4eca-9d43-b40f24ec8241/ticket.toml`


### Component: interview

<!-- ticket-index:entry id=f2882a13-58bd-4f42-b25c-092b564373c9 slug=open/interview digest=62f706c2819a -->
#### [f2882a13] [workflow-tools][per-tool] Pre-create interview tool repo as a single `interview` domain crate skeleton (placeholder)
- priority: `medium`
- summary: Phase B. Pre-create the `interview` repository (owner mankinskin) as a placeholder for the interview tool, scaffolded per contract `0da6894c`: a single `interview` domain crate skeleton whose lib wil...
- ref: `.ticket/tickets/f2882a13-58bd-4f42-b25c-092b564373c9/ticket.toml`


### Component: log

<!-- ticket-index:entry id=2736c3dc-ac19-4095-8a4a-e0a61340c58b slug=open/log digest=35f3028b3f44 -->
#### [2736c3dc] [workflow-tools][per-tool] Extract log tool as a single `log` domain crate (api + transport bins) + viewer frontend
- priority: `high`
- summary: Phase B. Extract the log tool into its own `log` repository (owner mankinskin), built as a single `log` domain crate per contract `0da6894c`: the crate lib re-exports the internal `log-api` crate and...
- ref: `.ticket/tickets/2736c3dc-ac19-4095-8a4a-e0a61340c58b/ticket.toml`


### Component: log-api

<!-- ticket-index:entry id=501d4932-a48e-4c8a-a4f3-8c31be0bdd23 slug=open/log-api digest=0031af6cfa2f -->
#### [501d4932] [log-api] Add first-class validation log capture and retrieval
- priority: `high`
- summary: Add a first-class `log-api` for workflow validation log capture, indexing, and retrieval in the memory system.
- ref: `.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23/ticket.toml`

<!-- ticket-index:entry id=aa94d02e-9620-4db6-9974-36699cd56537 slug=open/log-api digest=9cdb23f4c08f -->
#### [aa94d02e] [log-api] Add live indexing and search for active logs and journals
- priority: `high`
- summary: Add incremental indexing/search over active JSONL logs and operation journal metadata.
- ref: `.ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml`


### Component: log-viewer

<!-- ticket-index:entry id=06e00e0b-42ce-4a74-aea2-392302dd68f7 slug=open/log-viewer digest=9d6ad53f3923 -->
#### [06e00e0b] [log-viewer] Integrate GraphOpEvent replay with 3D graph visualization
- priority: `medium`
- summary: Integrate GraphOpEvent replay system with the enhanced 3D graph visualization in log-viewer:
- ref: `.ticket/tickets/06e00e0b-42ce-4a74-aea2-392302dd68f7/ticket.toml`

<!-- ticket-index:entry id=bf295665-a075-4cfb-9a86-f54e96918695 slug=open/log-viewer digest=e08943b26fd2 -->
#### [bf295665] [log-viewer] Integrate graph improvements (selection, rendering tiers, panel framing, 2D mode)
- priority: `high`
- summary: Integrate the four graph improvements into log-viewer:
- ref: `.ticket/tickets/bf295665-a075-4cfb-9a86-f54e96918695/ticket.toml`


### Component: mcp-cost-gate

<!-- ticket-index:entry id=8c4d1d9c-1004-4539-9880-0a0e8aa03dd3 slug=open/mcp-cost-gate digest=a0ece36f48f5 -->
#### [8c4d1d9c] [mcp-cost-gate] Re-tune graded-cost calibration from real rollup data
- summary: The graded cost-gate calibration constants shipped as provisional placeholders:
- ref: `.ticket/tickets/8c4d1d9c-1004-4539-9880-0a0e8aa03dd3/ticket.toml`


### Component: mcp-toolmon

<!-- ticket-index:entry id=1d08620b-1ef6-4e62-9cb5-6ccf8386c3ed slug=open/mcp-toolmon digest=691c2943a16d -->
#### [1d08620b] Author mcp-toolmon session-guard specification
- priority: `high`
- summary: Author a specification for mcp-toolmon session validation before guarded tool calls.
- ref: `.ticket/tickets/1d08620b-1ef6-4e62-9cb5-6ccf8386c3ed/ticket.toml`

<!-- ticket-index:entry id=ee2b23b5-fc16-4bc4-900d-da7ec216e1c9 slug=open/mcp-toolmon digest=7b97ee50d975 -->
#### [ee2b23b5] Define graded cost policy for spec 39983ddf
- priority: `high`
- summary: Refine spec `39983ddf` around a versioned graded-budget cost policy.
- ref: `.ticket/tickets/ee2b23b5-fc16-4bc4-900d-da7ec216e1c9/ticket.toml`

<!-- ticket-index:entry id=1f166356-0d54-4c08-bfee-5a9eac2e06b7 slug=open/mcp-toolmon digest=8f7b9ce3b900 -->
#### [1f166356] Define model rejection guidance for spec 9f0b9e30
- priority: `high`
- summary: Refine spec `9f0b9e30` with deterministic model-rejection guidance.
- ref: `.ticket/tickets/1f166356-0d54-4c08-bfee-5a9eac2e06b7/ticket.toml`

<!-- ticket-index:entry id=3283b554-8cc2-4518-84af-a2ba95ea9e50 slug=open/mcp-toolmon digest=7f39e3abcff2 -->
#### [3283b554] Test mcp-toolmon cost gate and session guard
- priority: `high`
- summary: Test cost-gate, caller-model resolution, rejection guidance, and session-guard behavior.
- ref: `.ticket/tickets/3283b554-8cc2-4518-84af-a2ba95ea9e50/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=55403b85-104c-4559-afd0-9af63fb30a75 slug=open/memory-api digest=253b4054e571 -->
#### [55403b85] Add two-worktree deletion isolation regression harness
- priority: `high`
- summary: Provide end-to-end evidence that deleting one worktree cannot affect another checkout's source, artifacts, stores, CLI, or MCP operations.
- ref: `.ticket/tickets/55403b85-104c-4559-afd0-9af63fb30a75/ticket.toml`

<!-- ticket-index:entry id=461ddbb1-f88f-4585-bf65-b7e721f1101e slug=open/memory-api digest=231aefd21fd5 -->
#### [461ddbb1] Contain feedback, spec, and test stores within the caller checkout
- priority: `high`
- summary: Apply the worktree isolation contract to feedback, spec, and test stores.
- ref: `.ticket/tickets/461ddbb1-f88f-4585-bf65-b7e721f1101e/ticket.toml`

<!-- ticket-index:entry id=7ef3f8db-d4a9-4135-99eb-3c006070a328 slug=open/memory-api digest=ca342d083533 -->
#### [7ef3f8db] Implement directed inherited schema lifecycle engine
- priority: `high`
- summary: Implement the shared directed schema engine required by [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).
- ref: `.ticket/tickets/7ef3f8db-d4a9-4135-99eb-3c006070a328/ticket.toml`

<!-- ticket-index:entry id=3a624bf6-f240-4ea8-b0bb-1bceac883b6b slug=open/memory-api digest=db219d714a24 -->
#### [3a624bf6] Worktree stores must be isolated: no cross-worktree path persistence, discovery, or shared caches
- priority: `high`
- summary: Each git worktree and every folder within a worktree MUST be completely isolated and independent from the content of other worktrees. No component may persist a dependency on another worktree's path ...
- ref: `.ticket/tickets/3a624bf6-f240-4ea8-b0bb-1bceac883b6b/ticket.toml`

<!-- ticket-index:entry id=35cd05c1-45f7-4d65-b943-7c000570928f slug=open/memory-api digest=0659220caf0d -->
#### [35cd05c1] [journal] Adapt move kernel journals to the generic operation-journal envelope
- priority: `medium`
- summary: Bridge the existing domain-neutral move kernel journal to the generalized operation-journal model without breaking current move APIs.
- ref: `.ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml`

<!-- ticket-index:entry id=39239e48-828a-41d8-a697-9cf02e980da9 slug=open/memory-api digest=db7d96349cbd -->
#### [39239e48] [memory-api] Transport-layer workspace-resolution parity (tracker)
- priority: `high`
- summary: Make workspace resolution a generic memory-api capability consumed identically by every transport (cli, mcp, http) across every entity domain, and keep transports pure (parse + dispatch only). Fix th...
- ref: `memory-api/.ticket/tickets/39239e48-828a-41d8-a697-9cf02e980da9/ticket.toml`

<!-- ticket-index:entry id=e268a1e8-3f3a-433f-b4a0-d58c590b8d29 slug=open/memory-api digest=9a8304531347 -->
#### [e268a1e8] [memory-api][bootstrap] Implement core-profile minimal-store fixture and template smoke path
- priority: `high`
- summary: Implement one concrete core-profile fixture and template smoke path for newly bootstrapped durable stores so the foundational memory-store contract is exercised by code, not only by policy text.
- ref: `.ticket/tickets/e268a1e8-3f3a-433f-b4a0-d58c590b8d29/ticket.toml`

<!-- ticket-index:entry id=8ab31960-f3fa-4a2b-b2ac-f807e1a15fdc slug=open/memory-api digest=8d1b728fa292 -->
#### [8ab31960] [memory-api][ticket-api][ticket-cli][ticket-mcp][ticket-http] Implement expressive ticket query and ordering
- priority: `high`
- summary: The repository has the pieces of a query engine, but not the complete interface needed for focused ticket discovery.
- ref: `.ticket/tickets/8ab31960-f3fa-4a2b-b2ac-f807e1a15fdc/ticket.toml`


### Component: memory-index

<!-- ticket-index:entry id=64bffa35-b622-452e-a469-5330d912ae8c slug=open/memory-index digest=f69d78601f74 -->
#### [64bffa35] [memory-index] Generic rendering abstraction: trait-based IndexEntryFormatter for all domains
- priority: `high`
- summary: Create a single, domain-parameterized rendering abstraction that all store indexes (ticket, spec, rule, audit) can use, eliminating duplication and enabling consistent, testable formatting across dom...
- ref: `.ticket/tickets/64bffa35-b622-452e-a469-5330d912ae8c/ticket.toml`


### Component: model-prices

<!-- ticket-index:entry id=2ac80674-fee8-43a9-b858-5f75f0221e41 slug=open/model-prices digest=9595fe51f4a0 -->
#### [2ac80674] [repo-hygiene] Untrack committed cost_gate .pyc and gitignore __pycache__
- summary: `tools/model-prices/__pycache__/cost_gate.cpython-314.pyc` is tracked in git (committed in 086f8b62 and updated in 9af02308). There is no `.pyc` / `__pycache__` entry in `.gitignore`, so compiled Pyt...
- ref: `.ticket/tickets/2ac80674-fee8-43a9-b858-5f75f0221e41/ticket.toml`


### Component: ngrams

<!-- ticket-index:entry id=a25e3cad-5bf2-4432-9c89-3a6dd67ee774 slug=open/ngrams digest=e07e182dddc3 -->
#### [a25e3cad] Plan: ngrams oracle validation — compare ngrams against oracle
- summary: tags: `#plan` `#testing` `#validation` `#ngrams` `#context-read` `#context-api` `#integration`
- ref: `.ticket/tickets/a25e3cad-5bf2-4432-9c89-3a6dd67ee774/ticket.toml`


### Component: observability

<!-- ticket-index:entry id=bce26d30-0a79-40b4-812a-c14b4a246de5 slug=open/observability digest=cc970e7eb85f -->
#### [bce26d30] [docs-tests] Validate unified logging and journaling architecture end to end
- priority: `medium`
- summary: Create the documentation and validation matrix for the unified logging and journaling design.
- ref: `.ticket/tickets/bce26d30-0a79-40b4-812a-c14b4a246de5/ticket.toml`


### Component: onboarding

<!-- ticket-index:entry id=d9e1c624-d422-4c1e-b221-daef5a557765 slug=open/onboarding digest=4bb53bde04df -->
#### [d9e1c624] [onboarding] Verify fresh-clone bootstrap end-to-end in an isolated container
- priority: `high`
- summary: Ticket 8f94b367 fixed the documented bootstrap path, but every completed check ran on an already-provisioned developer machine. Binaries and domain stores were already present, so those passing check...
- ref: `.ticket/tickets/d9e1c624-d422-4c1e-b221-daef5a557765/ticket.toml`


### Component: peek

<!-- ticket-index:entry id=6af0deab-3390-4b64-a6ae-35bed5d29730 slug=open/peek digest=41d75c0b1aae -->
#### [6af0deab] [workflow-tools][per-tool] Extract peek tool as a single `peek` domain crate (api + transport bins incl. compact-terminal)
- priority: `medium`
- summary: Phase B. Extract the peek tool into its own `peek` repository (owner mankinskin), built as a single `peek` domain crate per contract `0da6894c`: the crate lib re-exports the internal `peek-api` crate...
- ref: `.ticket/tickets/6af0deab-3390-4b64-a6ae-35bed5d29730/ticket.toml`


### Component: performance

<!-- ticket-index:entry id=631ebadb-6bbf-4fa2-9a71-f6b47d24c6a9 slug=open/performance digest=e3033c3d715c -->
#### [631ebadb] Build hardware-adaptive index performance harness
- priority: `medium`
- summary: Build index-generation performance harness using the adaptive baseline contract.
- ref: `.ticket/tickets/631ebadb-6bbf-4fa2-9a71-f6b47d24c6a9/ticket.toml`

<!-- ticket-index:entry id=f49fbb27-8b76-4f56-85d3-6e7365aa4a45 slug=open/performance digest=d908be79e3e6 -->
#### [f49fbb27] Define adaptive performance baseline for spec 18b6a9c5
- priority: `high`
- summary: Refine spec `18b6a9c5` with hardware-adaptive latency expectations.
- ref: `.ticket/tickets/f49fbb27-8b76-4f56-85d3-6e7365aa4a45/ticket.toml`


### Component: performance-validation

<!-- ticket-index:entry id=ef3f4a91-734f-47aa-a9cf-fdfdb60ac2db slug=open/performance-validation digest=2884f8dec47c -->
#### [ef3f4a91] [profiling] Performance profiling & benchmark matrix (tracker)
- priority: `high`
- summary: Parent tracker for performance profiling and benchmarking across context-engine and the viewer platform. It combines native storage/transport measurement with browser/WASM rendering evidence without ...
- ref: `.ticket/tickets/ef3f4a91-734f-47aa-a9cf-fdfdb60ac2db/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=7bc328d7-5168-4f93-938d-09e4e09dcdb4 slug=open/repo-guidance digest=9cd17d2f435e -->
#### [7bc328d7] Repository guidance and agent-template learnings from the workflow-tools restructuring
- priority: `high`
- summary: This epic collects every documentation and agent-template correction identified by a post-hoc analysis of the workflow-tools restructuring session.
- ref: `.ticket/tickets/7bc328d7-5168-4f93-938d-09e4e09dcdb4/ticket.toml`

<!-- ticket-index:entry id=ce0beb35-fc60-45ae-b26b-3cd06a282476 slug=open/repo-guidance digest=5f7d2b889b1a -->
#### [ce0beb35] [context-engine] Generate root README and root-owned child READMEs from rules
- priority: `high`
- summary: The root workspace already has a `.rule` store, but its `README.md` and the root-owned child READMEs are still manual. That leaves the most visible repository entry points outside the generation pipe...
- ref: `.ticket/tickets/ce0beb35-fc60-45ae-b26b-3cd06a282476/ticket.toml`

<!-- ticket-index:entry id=3f62f10e-6f7d-4fa1-b205-97fe62babaf2 slug=open/repo-guidance digest=49471b94b84e -->
#### [3f62f10e] [context-stack] Add local .rule store and generate root README
- priority: `high`
- summary: `context-stack` still lacks a repo-local `.rule` store and local README targets, so its root README cannot participate in the same local-generation workflow as the memory-viewers workspaces.
- ref: `.ticket/tickets/3f62f10e-6f7d-4fa1-b205-97fe62babaf2/ticket.toml`

<!-- ticket-index:entry id=c785a6f6-57d3-46d1-9a0e-36e1a4b74a47 slug=open/repo-guidance digest=0105b7a71c60 -->
#### [c785a6f6] [context-stack] Generate first-level child READMEs with parent links
- priority: `high`
- summary: Even once the `context-stack` root README is generated, the internal README tree still fails because its first-level child READMEs do not link back to the parent and are not managed as a coherent rep...
- ref: `.ticket/tickets/c785a6f6-57d3-46d1-9a0e-36e1a4b74a47/ticket.toml`

<!-- ticket-index:entry id=26f570e2-6a2f-4604-9347-a3ac7d0314c3 slug=open/repo-guidance digest=34146d41678d -->
#### [26f570e2] [memory-viewers] Adopt shared README schema and normalize child blocks
- priority: `high`
- summary: `memory-viewers` is the aggregate repo root for the generated family, but its README target still has a bespoke structure and its child-block behavior needs to be normalized after the child repos ado...
- ref: `.ticket/tickets/26f570e2-6a2f-4604-9347-a3ac7d0314c3/ticket.toml`

<!-- ticket-index:entry id=7b8d2e81-6f00-486c-a839-ca5eb77dc109 slug=open/repo-guidance digest=779f30bd11fa -->
#### [7b8d2e81] [readmes][generated-repos] Adopt shared README schema in memory-viewers family
- priority: `high`
- summary: The already-generated README surfaces in `memory-api`, `viewer-api`, and `memory-viewers` still use bespoke target layouts. They need to adopt the shared schema and fill the missing parent or child n...
- ref: `.ticket/tickets/7b8d2e81-6f00-486c-a839-ca5eb77dc109/ticket.toml`

<!-- ticket-index:entry id=95a12f97-dc32-4835-a87a-5e24574be951 slug=open/repo-guidance digest=3bd5f6c3d27c -->
#### [95a12f97] [readmes][manual-repos] Migrate root and context-stack README trees to rule generation
- priority: `high`
- summary: `context-engine` and `context-stack` are still the manual outliers in the repository README family. They need to move onto rule-backed generation and the same parent/child README navigation contract ...
- ref: `.ticket/tickets/95a12f97-dc32-4835-a87a-5e24574be951/ticket.toml`

<!-- ticket-index:entry id=9f14365b-fbe5-4f93-a8da-f7f490dacac0 slug=open/repo-guidance digest=c3bc5d9475f2 -->
#### [9f14365b] [readmes][qa] Add completeness audit and workspace sync checks
- priority: `high`
- summary: Even after the rollout lands, the README tree will drift again unless there is a mechanical check for generated ownership, parent and child navigation blocks, installable-content coverage, and direct...
- ref: `.ticket/tickets/9f14365b-fbe5-4f93-a8da-f7f490dacac0/ticket.toml`

<!-- ticket-index:entry id=ef50db70-90e6-4de4-bcb0-fa364664a6cf slug=open/repo-guidance digest=1656cdac8041 -->
#### [ef50db70] [readmes][rule-api] Roll out shared README schema across workspace trees
- priority: `high`
- summary: Repository README generation is split between manual repo roots and generated nested workspaces. Shared structure is duplicated, parent/child navigation is inconsistent, and there is no single tracke...
- ref: `.ticket/tickets/ef50db70-90e6-4de4-bcb0-fa364664a6cf/ticket.toml`

<!-- ticket-index:entry id=ca30f696-e8a0-4904-9a1d-a507e9ef6147 slug=open/repo-guidance digest=0083f19e9e91 -->
#### [ca30f696] [readmes][rule-api] Track shared schema loader contract and rollout follow-through
- priority: `high`
- summary: The shared README schema rollout now has a concrete loader-contract gap: shared schema fragments can be reached through both explicit imports and ambient fragment discovery, and the rollout depends o...
- ref: `.ticket/tickets/ca30f696-e8a0-4904-9a1d-a507e9ef6147/ticket.toml`


### Component: rule

<!-- ticket-index:entry id=21893f5f-e57f-4cdf-b5de-39f42ae5d89d slug=open/rule digest=d5db5c21b4ad -->
#### [21893f5f] [workflow-tools][per-tool] Extract rule tool as a single `rule` domain crate (api + transport bins)
- priority: `high`
- summary: Phase B. Extract the rule tool into its own `rule` repository (owner mankinskin), built as a single `rule` domain crate per contract `0da6894c`: the crate lib re-exports the internal `rule-api` crate...
- ref: `.ticket/tickets/21893f5f-e57f-4cdf-b5de-39f42ae5d89d/ticket.toml`


### Component: rule-api

<!-- ticket-index:entry id=f15d9e8b-72d2-44d9-965d-9fecbbc02d7f slug=open/rule-api digest=586bc8368b5f -->
#### [f15d9e8b] Build rule-api for generated agent instruction docs
- priority: `high`
- summary: Agent-facing markdown guidance is duplicated across context-engine, memory-viewers, memory-api, and viewer-api. The duplicated files are currently copy-pasted and several are byte-identical. This cre...
- ref: `memory-api/.ticket/tickets/f15d9e8b-72d2-44d9-965d-9fecbbc02d7f/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000000 slug=open/rule-api digest=229978e36b3e -->
#### [9c1de000] [guidance] Multi-client guidance rendering and install-time materialization
- priority: `high`
- summary: Tracker for making the `.rule/` stores the single canonical source of agent guidance, rendering that guidance into client-specific output files via templates, and materializing those files at install...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000000/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000003 slug=open/rule-api digest=782a75d5509d -->
#### [9c1de000] [guidance][p0] Purge machine-specific paths from generated-target state records
- priority: `high`
- summary: `.rule/entities/` holds 109 generated-target state records. Their `config_path` and `output_path` values are absolute Windows UNC paths of the form `//?/C:/Users/linus/git/graph_app/context-engine/.....
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000003/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000004 slug=open/rule-api digest=d02567d62779 -->
#### [9c1de000] [guidance][p0] Re-import current guidance files as canonical rule bodies
- priority: `high`
- summary: The rule stores still hold rule bodies for surfaces whose targets were deleted:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000004/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000002 slug=open/rule-api digest=a3523c1f4c0d -->
#### [9c1de000] [guidance][p0] Re-scope 23e81ad8 into the multi-client track
- priority: `high`
- summary: Ticket `23e81ad8` ("[rule+skill] Rule-store sources for domain-store scaffolding instructions", `priority: high`, `effort: 700`) mandates creating canonical rule entries **and generation targets** fo...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000002/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000001 slug=open/rule-api digest=0f8b78e208a9 -->
#### [9c1de000] [guidance][p0] Reconcile the already-executed decommissioning tickets
- priority: `high`
- summary: Tickets `14c0995c` (epic, isolated node with no edges), `f43cb5cb`, `76d0ace3`, and `16cfd19f` are all state `new`, but the work they describe **already landed in git**:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000001/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000005 slug=open/rule-api digest=524564e9be44 -->
#### [9c1de000] [guidance][p1] Add structured guidance metadata to the rule entry schema
- priority: `high`
- summary: Client format differences are almost entirely *frontmatter* differences. Measured key sets:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000005/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000006 slug=open/rule-api digest=e3ae67139c2b -->
#### [9c1de000] [guidance][p1] Migrate frontmatter out of body.md into structured metadata
- priority: `high`
- summary: `GeneratedMarkdownConfig { file_comment, entry_prefix, skip_provenance_for_yaml_frontmatter }` in `memory-api/crates/memory-api/src/generated_markdown.rs` splits a leading `---` block off the first e...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000006/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000007 slug=open/rule-api digest=c2f24ae1cc97 -->
#### [9c1de000] [guidance][p2] Add minijinja and sandboxed template loading to rule-api
- priority: `high`
- summary: There is no templating engine anywhere in the workspace — `grep` for `handlebars|tera|minijinja|askama|liquid` across every `Cargo.toml` returns zero matches. `memory-api/crates/rule-api/Cargo.toml` ...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000007/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000010 slug=open/rule-api digest=f8a988b9bb03 -->
#### [9c1de000] [guidance][p2] Author the Cline client profile
- priority: `medium`
- summary: Cline is the existing working precedent. `rule-targets/25-cline.yaml` declares `defaults: { repo_scope: context-engine, file_kind: AGENTS }` and a `folders:` tree rooted at `.clinerules` with four `f...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000010/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000009 slug=open/rule-api digest=5d16a6bde3e7 -->
#### [9c1de000] [guidance][p2] Author the GitHub Copilot / VS Code client profile
- priority: `high`
- summary: Copilot discovery is wired through `.vscode/settings.json`:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000009/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000011 slug=open/rule-api digest=d2a71d78fdac -->
#### [9c1de000] [guidance][p2] Author the OpenCode client profile
- priority: `medium`
- summary: OpenCode currently consumes `.agents/` through a single pointer. `opencode.json` at repo root is **hand-maintained** and declares:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000011/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000008 slug=open/rule-api digest=d7edd9d98289 -->
#### [9c1de000] [guidance][p2] Introduce the client-profile model and extend RenderTarget
- priority: `high`
- summary: `RenderTarget` in `memory-api/crates/rule-api/src/targets_model.rs` resolves to `{ name, repo_scope, file_kind, path_scope, section, state, nodes, output_path, source_config_path, source_output_root ...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000008/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000016 slug=open/rule-api digest=dff607534d13 -->
#### [9c1de000] [guidance][p3] Generate client entry configs and hook manifests
- priority: `high`
- summary: Each client has a discovery entry point that is currently hand-maintained:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000016/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000012 slug=open/rule-api digest=02280db6ff5a -->
#### [9c1de000] [guidance][p3] Guidance availability manifest and per-machine selection lockfile
- priority: `high`
- summary: There is currently **no** notion of installing guidance files. `install-tools.sh` (368 lines, 25 Rust binaries), `install-extensions.sh` (350 lines, one VS Code extension), and `install-deps.sh` (241...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000012/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000015 slug=open/rule-api digest=090e7f8aee87 -->
#### [9c1de000] [guidance][p3] Integrate vendored-skill installation into guidance install
- priority: `medium`
- summary: `skills-lock.json` is the existing vendoring mechanism: `{ version: 1, skills: { <name>: { source, sourceType: "github", skillPath, computedHash } } }`. `.agents/skills/` currently holds 116 markdown...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000015/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000014 slug=open/rule-api digest=924066a52825 -->
#### [9c1de000] [guidance][p3] install-guidance.sh top-level wrapper
- priority: `high`
- summary: Add `install-guidance.sh` at repo root, matching the existing install-script family shape: source `tools/install/common.sh`, expose a `<x>_names` array, a `<x>_path()` case dispatch, and a `usage()` ...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000014/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000013 slug=open/rule-api digest=413a784c6869 -->
#### [9c1de000] [guidance][p3] rule install --client subcommand
- priority: `high`
- summary: The `rule` CLI (`memory-api/tools/cli/rule-cli`, binary `rule`) has `generate-file`, `generate-target`, `sync-targets`, `sync-rules`, `scan`, `import-file`, `benchmark-targets`, `missing-rule`, `stor...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000013/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000017 slug=open/rule-api digest=33b25612b1cb -->
#### [9c1de000] [guidance][p4] Implement full sync-rules round-trip over structured metadata
- priority: `high`
- summary: Because all client outputs become gitignored and generated, editing a generated file is the *only* ergonomic authoring path — and the confirmed decision is that editing the generated file, then rever...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000017/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000018 slug=open/rule-api digest=84806a52cf4c -->
#### [9c1de000] [guidance][p4] Overwrite protection in sync-targets
- priority: `high`
- summary: Per the `14c0995c` reconciliation, `sync_targets_payload` (in `memory-api/tools/cli/rule-cli/src/cli/rendering.rs`) now treats a record as an orphaned generated artifact only when the output file exi...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000018/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000019 slug=open/rule-api digest=1b8654fb27ac -->
#### [9c1de000] [guidance][p5] Golden-file fixtures for every client profile
- priority: `high`
- summary: Snapshot the current committed state of every in-scope surface **before** any generation is re-enabled; these snapshots are the correctness baseline.
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000019/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000022 slug=open/rule-api digest=7493b64a2a18 -->
#### [9c1de000] [guidance][p5] Live client smoke test
- priority: `high`
- summary: Golden fixtures prove rendering is stable; they do not prove a client actually *loads* the result. Each client has a different discovery mechanism, so each needs a real load check.
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000022/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000021 slug=open/rule-api digest=4d8e78730d78 -->
#### [9c1de000] [guidance][p5] Re-widen the pre-commit drift gate
- priority: `high`
- summary: `.githooks/pre-commit` was deliberately narrowed away from `.agents/**`. It carries the explicit comment: `.agents/** are hand-owned (not produced by rule-targets.yaml); do NOT trigger the sync-targe...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000021/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000020 slug=open/rule-api digest=82a42db1a902 -->
#### [9c1de000] [guidance][p5] Round-trip idempotence test
- priority: `high`
- summary: Prove `render → sync-rules → render` is a fixed point across the whole store:
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000020/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000024 slug=open/rule-api digest=23d34f092229 -->
#### [9c1de000] [guidance][p6] Big-bang untrack of generated guidance surfaces
- priority: `high`
- summary: Roughly 197 tracked markdown files (41 instructions, 16 agents, 24 prompts, 116 skill files) plus `.clinerules/` and `.github/copilot-instructions.md` become generated artifacts. The confirmed strate...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000024/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000023 slug=open/rule-api digest=1d0234debaf4 -->
#### [9c1de000] [guidance][p6] Bootstrap layer: committed AGENTS.md pointing at install instructions
- priority: `high`
- summary: With all guidance outputs gitignored, a fresh clone would otherwise contain **zero** agent guidance — a bootstrap paradox, since an agent working in the fresh clone has no way to learn that an instal...
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000023/ticket.toml`

<!-- ticket-index:entry id=9c1de000-0000-4000-8000-000000000025 slug=open/rule-api digest=5a767e798e37 -->
#### [9c1de000] [guidance][p6] Documentation cutover
- priority: `medium`
- summary: Update root `README.md` with a setup stage covering client selection and `install-guidance.sh`.
- ref: `.ticket/tickets/9c1de000-0000-4000-8000-000000000025/ticket.toml`

<!-- ticket-index:entry id=0ec7d37a-2357-44c4-bc7c-4839ec01afec slug=open/rule-api digest=142dc8dfafa1 -->
#### [0ec7d37a] [memory-index] Rule catalog: single-line entries, three-segment group keys, collapsible groups, clickable rule links
- priority: `medium`
- summary: Improve the `.rule/README.md` generated index so each entry is a single compact line ordered by section/slug tree rather than UUID, with collapsible slug-prefix groups.
- ref: `.ticket/tickets/0ec7d37a-2357-44c4-bc7c-4839ec01afec/ticket.toml`

<!-- ticket-index:entry id=4e8293da-83cc-445e-9383-aa9be3b53c32 slug=open/rule-api digest=8c72668cacd4 -->
#### [4e8293da] [rule-api][audit-api] rule_overlap trial test fails: RuleStore::init picks up state outside its tempdir, causing DuplicateSlug on first insert
- priority: `high`
- summary: `cargo test -p audit-api` currently reports 13 passed / 1 failed, not the 14/0 recorded on ticket d1b3a6c9's validation evidence. The single failure:
- ref: `memory-api/.ticket/tickets/4e8293da-83cc-445e-9383-aa9be3b53c32/ticket.toml`

<!-- ticket-index:entry id=1fd0c182-f4b4-486b-b757-fe47e3238e43 slug=open/rule-api digest=872a622d3469 -->
#### [1fd0c182] [rule-mcp][rule-http] Workspace-resolution parity — nested-root awareness + pure transport
- priority: `medium`
- summary: Follow-up after the ticket-domain first run. Adopt the shared memory-api resolver + pure-transport pattern for the rule transports.
- ref: `memory-api/.ticket/tickets/1fd0c182-f4b4-486b-b757-fe47e3238e43/ticket.toml`


### Component: search

<!-- ticket-index:entry id=ee43f72e-53ef-4937-8216-92e17f185d85 slug=open/search digest=786f613a4bdb -->
#### [ee43f72e] [bootstrap] implement unified query execution on real indexes
- summary: Support fast query and highlighting across:
- ref: `memory-api/.ticket/tickets/ee43f72e-53ef-4937-8216-92e17f185d85/ticket.toml`


### Component: session

<!-- ticket-index:entry id=db6980d1-38bf-4819-8c07-b6db09229c1c slug=open/session digest=d5a199e4e126 -->
#### [db6980d1] Worktree provisioning and session-worktree lifecycle
- priority: `high`
- summary: Gather all worktree provisioning, worktree tooling, and session-to-worktree assignment work under one tracker.
- ref: `.ticket/tickets/db6980d1-38bf-4819-8c07-b6db09229c1c/ticket.toml`

<!-- ticket-index:entry id=94de0e62-0d8d-4c33-b7b0-1a1e5dcffd25 slug=open/session digest=ffddfb488a98 -->
#### [94de0e62] [workflow-tools][per-tool] Extract session tool as a single `session` domain crate (api + transport bins)
- priority: `high`
- summary: Phase B. Extract the session tool into its own `session` repository (owner mankinskin), built as a single `session` domain crate per contract `0da6894c`: the crate lib re-exports the internal `sessio...
- ref: `.ticket/tickets/94de0e62-0d8d-4c33-b7b0-1a1e5dcffd25/ticket.toml`

<!-- ticket-index:entry id=200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915 slug=open/session digest=50fa523ed54f -->
#### [200e9ecc] session_check_in rejects a single nested worktree unless a persisted assignment exists
- priority: `high`
- summary: `session_check_in` fails for session `b9020ba2-df5d-426a-b1b9-228ef159cad1` even though the session has exactly one nested worktree at `.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learni...
- ref: `.ticket/tickets/200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=cc516b85-e8fd-4184-83d2-ac3b9b120b49 slug=open/session-api digest=2b139f702884 -->
#### [cc516b85] Author session access-lease model specification
- priority: `high`
- summary: Author a specification for ticket-track, worktree, and file access leases.
- ref: `.ticket/tickets/cc516b85-e8fd-4184-83d2-ac3b9b120b49/ticket.toml`

<!-- ticket-index:entry id=1d5f58eb-1ab3-4a9e-8388-f5177a0bac98 slug=open/session-api digest=642c1572a134 -->
#### [1d5f58eb] Backfill session→ticket attribution from transcript tool-call signals
- priority: `high`
- summary: The session store is `.session/sessions/<session-id>/{session.json, transcript.json, events.json, tool-metrics.json}` and contains 238 session directories. The ticket store is `.ticket/tickets/<ticke...
- ref: `.ticket/tickets/1d5f58eb-1ab3-4a9e-8388-f5177a0bac98/ticket.toml`

<!-- ticket-index:entry id=7bb007e9-be8d-4821-a91a-f25a00dcbdbc slug=open/session-api digest=7f586d98c657 -->
#### [7bb007e9] Blocking goal-conformance gate on handoff
- summary: Blocking conformance gate runs in HANDOFF WRITE PATH (session_handoff). Non-conforming handoff REJECTED at write time, fail closed. Not at session start/resume.
- ref: `.ticket/tickets/7bb007e9-be8d-4821-a91a-f25a00dcbdbc/ticket.toml`

<!-- ticket-index:entry id=9cd9440e-ec01-47db-a67f-ec0ee9509ac7 slug=open/session-api digest=df062d2cdea9 -->
#### [9cd9440e] CLI parity for track query and rollup
- summary: CLI commands matching MCP track query and rollup surface.
- ref: `.ticket/tickets/9cd9440e-ec01-47db-a67f-ec0ee9509ac7/ticket.toml`

<!-- ticket-index:entry id=b43363e1-a89e-4c7e-a540-304d04eaa3f9 slug=open/session-api digest=e74f8351411e -->
#### [b43363e1] Docs: track API, routing contract, and migration guide
- summary: Documentation for track API usage, routing contract, and migration guide for existing sessions.
- ref: `.ticket/tickets/b43363e1-a89e-4c7e-a540-304d04eaa3f9/ticket.toml`

<!-- ticket-index:entry id=ec2e1048-4cf1-4129-bdf7-57ac45ebdac6 slug=open/session-api digest=458b08f599ad -->
#### [ec2e1048] Eliminate workspace_session_id identifier across session-api surface
- priority: `medium`
- summary: Mechanical removal of the `workspace_session_id` identifier now that `session_id` is the single identity handle (see spec 709f067a-21b6-41b6-8879-3cacef4bacaf). The string occurs in 28 files, ALL und...
- ref: `memory-api/.ticket/tickets/ec2e1048-4cf1-4129-bdf7-57ac45ebdac6/ticket.toml`

<!-- ticket-index:entry id=be1552ba-99d8-4b9b-ac86-4163f5c5af54 slug=open/session-api digest=35156386fd9b -->
#### [be1552ba] Lazy session artifact creation
- summary: First sub-task**: Audit and enumerate every session-artifact creation site before implementing lazy gating. Deliverable: enumerated list.
- ref: `.ticket/tickets/be1552ba-99d8-4b9b-ac86-4163f5c5af54/ticket.toml`

<!-- ticket-index:entry id=185a00a2-a849-48b1-b4ce-08cc8fd3552d slug=open/session-api digest=dad0d22e250b -->
#### [185a00a2] MCP tool surface for track query and rollup
- summary: MCP tools for querying sessions by track_id and track-scoped rollup.
- ref: `.ticket/tickets/185a00a2-a849-48b1-b4ce-08cc8fd3552d/ticket.toml`

<!-- ticket-index:entry id=c38436d8-08a0-45db-bc49-4aa2906bdf57 slug=open/session-api digest=1fb34504835e -->
#### [c38436d8] Phase 3: Harden capture-hook reliability
- priority: `high`
- summary: Replace synthetic capture-hook assumptions with replay, contention, and failure coverage.
- ref: `.ticket/tickets/c38436d8-08a0-45db-bc49-4aa2906bdf57/ticket.toml`

<!-- ticket-index:entry id=7d685c20-ebd8-4505-8fc0-6a42b0847750 slug=open/session-api digest=2aa26c188764 -->
#### [7d685c20] Record sanitized live capture-hook fixtures
- priority: `high`
- summary: Record and sanitize real `PostToolUse` and `Stop` hook fixtures from live Copilot artifacts.
- ref: `.ticket/tickets/7d685c20-ebd8-4505-8fc0-6a42b0847750/ticket.toml`

<!-- ticket-index:entry id=8858888a-173e-4c48-8059-8a291fc52d16 slug=open/session-api digest=35103374f7e6 -->
#### [8858888a] Replay uncovered capture-hook matrix cases
- priority: `high`
- summary: Implement replay tests for the 31 uncovered rows of the 38-row capture-hook matrix.
- ref: `.ticket/tickets/8858888a-173e-4c48-8059-8a291fc52d16/ticket.toml`

<!-- ticket-index:entry id=490f1cbc-8ae9-434a-9eef-d09433b25798 slug=open/session-api digest=2a6ecf183e78 -->
#### [490f1cbc] Session id traceability injection
- summary: Inject session id into rendered instructions via session_runtime_render_instructions + hook.
- ref: `.ticket/tickets/490f1cbc-8ae9-434a-9eef-d09433b25798/ticket.toml`

<!-- ticket-index:entry id=3a77487e-71c9-41bb-b584-d8b2c076e177 slug=open/session-api digest=d80b5e788535 -->
#### [3a77487e] Spec: formalize track-scoped multi-session execution
- summary: Spec authoring: document track concept, fields, routing contract, completion semantics.
- ref: `.ticket/tickets/3a77487e-71c9-41bb-b584-d8b2c076e177/ticket.toml`

<!-- ticket-index:entry id=648a64a6-5bea-41a1-b31f-2a4b0ed9d6fc slug=open/session-api digest=54023cdeb58b -->
#### [648a64a6] Sub-agent board registration and WIP budget
- summary: Sub-agent sessions register on board. Read-only sub-agents EXEMPT from WIP counting. Raise WIP limit to 20.
- ref: `.ticket/tickets/648a64a6-5bea-41a1-b31f-2a4b0ed9d6fc/ticket.toml`

<!-- ticket-index:entry id=c410ca60-e079-430c-93b0-5db144dd99ad slug=open/session-api digest=7b542933018f -->
#### [c410ca60] Sub-agent session creation and routing
- summary: Every sub-agent invocation gets its own durable isolated session stamped with parent_session_id and inherited track_id.
- ref: `.ticket/tickets/c410ca60-e079-430c-93b0-5db144dd99ad/ticket.toml`

<!-- ticket-index:entry id=8210d2d4-1b8b-4e19-b72d-8665500ad07f slug=open/session-api digest=7c4ad45b0366 -->
#### [8210d2d4] Test capture-hook contention and write failures
- priority: `high`
- summary: Add capture-hook contention and failure-path tests.
- ref: `.ticket/tickets/8210d2d4-1b8b-4e19-b72d-8665500ad07f/ticket.toml`

<!-- ticket-index:entry id=a756fcf5-b1e6-476d-900f-ae2534dcdff3 slug=open/session-api digest=80f4a3f9b943 -->
#### [a756fcf5] Test session transport workflows and lease contention
- priority: `high`
- summary: Test session CLI/MCP workflows, including access-lease contention.
- ref: `.ticket/tickets/a756fcf5-b1e6-476d-900f-ae2534dcdff3/ticket.toml`

<!-- ticket-index:entry id=938a7ae9-570e-40c4-91f5-d32d2fae0b4f slug=open/session-api digest=49a162928646 -->
#### [938a7ae9] Track query and rollup surface
- summary: FULL metrics rollup: session count, status, aggregate tool/cost metrics, duration, token cost, per-agent breakdown.
- ref: `.ticket/tickets/938a7ae9-570e-40c4-91f5-d32d2fae0b4f/ticket.toml`

<!-- ticket-index:entry id=16e4063a-32c6-416c-a6fe-160df9f9edd0 slug=open/session-api digest=3fa25e091f1b -->
#### [16e4063a] Track-scoped multi-session execution
- summary: First implementation batch (one session, one commit at end):
- ref: `.ticket/tickets/16e4063a-32c6-416c-a6fe-160df9f9edd0/ticket.toml`

<!-- ticket-index:entry id=2d48cf8c-c56e-47e1-afbb-ceb5e8035fd4 slug=open/session-api digest=1da0a96f334e -->
#### [2d48cf8c] Verify VS Code delivers hookSpecificOutput.additionalContext to the model on UserPromptSubmit
- priority: `high`
- summary: The repository consistently emits `hookSpecificOutput.additionalContext`, but no repository evidence proves that VS Code injects the field into the model prompt for `UserPromptSubmit`, and no reposit...
- ref: `.ticket/tickets/2d48cf8c-c56e-47e1-afbb-ceb5e8035fd4/ticket.toml`

<!-- ticket-index:entry id=278d22d1-6de1-4426-b022-07262c9657ce slug=open/session-api digest=a4e820562fae -->
#### [278d22d1] [delegation-cost] Reconcile substitutable_shell_count vs classify_shell_command cd-chain divergence
- priority: `medium`
- summary: `memory-api/crates/session-api/src/delegation_cost.rs` has two predicates
- ref: `.ticket/tickets/278d22d1-6de1-4426-b022-07262c9657ce/ticket.toml`

<!-- ticket-index:entry id=5cbae4be-9f62-49ca-827e-44bed8242bc6 slug=open/session-api digest=71a7a128132c -->
#### [5cbae4be] [delegation-cost][verification] Capture a real post-9d527ad1 delegation session and replay it through the 10d21210 harness for epic AC4/AC5/AC6 evidence
- priority: `high`
- summary: Epic [79c4ac3e](.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml)'s
- ref: `.ticket/tickets/5cbae4be-9f62-49ca-827e-44bed8242bc6/ticket.toml`

<!-- ticket-index:entry id=47e4b2e5-d5c0-4f2e-9133-7ea98e08964e slug=open/session-api digest=724689c4ae4b -->
#### [47e4b2e5] [session-api] Advisory convergence detection (shared ticket/spec ids, overlapping files, same track_id)
- summary: Implement the advisory, non-mutating convergence detector per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R8.
- ref: `.ticket/tickets/47e4b2e5-d5c0-4f2e-9133-7ea98e08964e/ticket.toml`

<!-- ticket-index:entry id=baa06c07-5d0a-4802-9936-a91959be09fe slug=open/session-api digest=6920ea1382a4 -->
#### [baa06c07] [session-api] Best-effort backfill migration for populated legacy cross-session links
- summary: Backfill migration per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R9, run once before the legacy-field-removal ticket lands.
- ref: `.ticket/tickets/baa06c07-5d0a-4802-9936-a91959be09fe/ticket.toml`

<!-- ticket-index:entry id=d085cf2b-7683-4aea-8566-99dc883ee491 slug=open/session-api digest=354865369f2d -->
#### [d085cf2b] [session-api] Bidirectional depends_on track edges + derived overlaps
- summary: Implement track-to-track relationships per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R6.
- ref: `.ticket/tickets/d085cf2b-7683-4aea-8566-99dc883ee491/ticket.toml`

<!-- ticket-index:entry id=1d378109-28d2-442b-a2a1-4e18cd716327 slug=open/session-api digest=c7a4a0bc31fc -->
#### [1d378109] [session-api] CLI + MCP exposure of fan_out / merge / pickup
- summary: Expose `fan_out` / `merge` / `pickup` as CLI subcommands and MCP tools per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R4.
- ref: `.ticket/tickets/1d378109-28d2-442b-a2a1-4e18cd716327/ticket.toml`

<!-- ticket-index:entry id=f35f4dd9-1a05-47ee-b334-809bb34e63a7 slug=open/session-api digest=7a7b0cc00e9d -->
#### [f35f4dd9] [session-api] Embed a persistent step graph (nodes + edges snapshot) in the handoff package
- priority: `high`
- summary: `session_handoff` persists `objective`, `target_tickets`, `target_files`, `decisions`, `validation`, `non_goals`, `context_anchors`, `open_escalations`, `risk_notes`, `predecessor_handoff`. None of t...
- ref: `.ticket/tickets/f35f4dd9-1a05-47ee-b334-809bb34e63a7/ticket.toml`

<!-- ticket-index:entry id=0869353b-417c-4ce0-82bb-333e9fd39945 slug=open/session-api digest=95bf502f89ed -->
#### [0869353b] [session-api] Handoff edge model: emitted/picked-up ids, target binding at pickup, unclaimed backlog
- priority: `high`
- summary: Implement the binary handoff provenance edge per spec [c737328d Session merge and pickup](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirements R1, R2, R3.
- ref: `.ticket/tickets/0869353b-417c-4ce0-82bb-333e9fd39945/ticket.toml`

<!-- ticket-index:entry id=fe221c20-bfd1-4673-aac6-2bbcf6169e95 slug=open/session-api digest=85728b338488 -->
#### [fe221c20] [session-api] Orchestrator round-trip fixture and provenance-graph reconstruction unit tests
- summary: Deliver the scripted orchestrator round-trip fixture (spec AC6) and the unit-level graph-reconstruction tests (spec AC1) that validate the whole epic end to end.
- ref: `.ticket/tickets/fe221c20-bfd1-4673-aac6-2bbcf6169e95/ticket.toml`

<!-- ticket-index:entry id=12641ad0-3eea-48e7-927d-20b814b1b7e3 slug=open/session-api digest=515e78ebb9a6 -->
#### [12641ad0] [session-api] Remove parent_session_id / spawned_session_id / predecessor_handoff (keep predecessor_run_id)
- summary: Remove the superseded singular cross-session fields per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R7, after the backfill migration ticket below has...
- ref: `.ticket/tickets/12641ad0-3eea-48e7-927d-20b814b1b7e3/ticket.toml`

<!-- ticket-index:entry id=d28afbc0-9d16-4494-8ca5-4154f3ace9be slug=open/session-api digest=43edb15b8030 -->
#### [d28afbc0] [session-api] Session merge and pickup: handoff-edge provenance graph and first-class tracks
- summary: Make N→1 merge and 1→N split of sessions representable, queryable, and validated, so work that
- ref: `.ticket/tickets/d28afbc0-9d16-4494-8ca5-4154f3ace9be/ticket.toml`

<!-- ticket-index:entry id=857593ec-11d6-4c73-b5d8-7bf3b7eadd37 slug=open/session-api digest=a0dfdff56d4c -->
#### [857593ec] [session-api] Token-efficient transcript range peeking
- priority: `high`
- summary: Provide a token-efficient way to inspect a specific range of turns in a persisted session transcript without reading the entire file. This mirrors the `peek` CLI tool's line-range behavior but operat...
- ref: `.ticket/tickets/857593ec-11d6-4c73-b5d8-7bf3b7eadd37/ticket.toml`

<!-- ticket-index:entry id=a2194b92-d0b2-4eb8-a2a6-975919ab4035 slug=open/session-api digest=5f1f28e326ff -->
#### [a2194b92] [session-api] Track entity and store under .session/tracks/<id>/
- summary: Introduce the first-class `Track` entity per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R5.
- ref: `.ticket/tickets/a2194b92-d0b2-4eb8-a2a6-975919ab4035/ticket.toml`

<!-- ticket-index:entry id=8cfdce69-a8b5-49dd-8ad7-96518bf0b8cc slug=open/session-api digest=806ff2b43f71 -->
#### [8cfdce69] [session-api] Transcript skeleton peeking
- priority: `high`
- summary: Provide a token-efficient way to inspect the structure of a session transcript by returning only the metadata/signatures of turns (sequence, role, captured_at, tool_name, and content length/summary) ...
- ref: `.ticket/tickets/8cfdce69-a8b5-49dd-8ad7-96518bf0b8cc/ticket.toml`

<!-- ticket-index:entry id=618eb6e6-7544-47cc-9ddb-ecdfbfdaf0a1 slug=open/session-api digest=c480c7ad0ad2 -->
#### [618eb6e6] [session-api] fan_out / merge / pickup operations at the API layer
- priority: `high`
- summary: Implement the canonical `fan_out` / `merge` / `pickup` operations at the session-api layer per spec [c737328d](../../.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) requirement R4 (buildi...
- ref: `.ticket/tickets/618eb6e6-7544-47cc-9ddb-ecdfbfdaf0a1/ticket.toml`

<!-- ticket-index:entry id=e3474d5f-07c2-43a9-b296-b85f794f9923 slug=open/session-api digest=ff1c18b3d134 -->
#### [e3474d5f] [session-api] session CLI has no prune/gc subcommand for dangling session records
- priority: `high`
- summary: `session-api`/the `session` CLI has no `prune`/`gc` subcommand for dangling
- ref: `.ticket/tickets/e3474d5f-07c2-43a9-b296-b85f794f9923/ticket.toml`

<!-- ticket-index:entry id=75771e96-3d9b-4ab6-a3f4-821387037aa1 slug=open/session-api digest=be0d702c56da -->
#### [75771e96] [session-api][audit-api] Tracker: workflow diagnostics routing and structural graph validation
- priority: `medium`
- summary: Tracker for work related to routing session workflow diagnostics upward out of rendering paths and validating workflow graph structure (dangling edges, duplicate node ids) before handoff writes and v...
- ref: `memory-api/.ticket/tickets/75771e96-3d9b-4ab6-a3f4-821387037aa1/ticket.toml`

<!-- ticket-index:entry id=a6f17580-f7ea-4c4e-92e7-0c77c68185fe slug=open/session-api digest=bce6d59426e4 -->
#### [a6f17580] [session-api][handoff] Add narrative/blockers/new_entities fields to SessionHandoffRecord model
- priority: `high`
- summary: Extend the `SessionHandoffRecord` schema with the durable knowledge-transfer payload (subticket 1 of `6431985e`). Model + serde only — no population logic.
- ref: `memory-api/.ticket/tickets/a6f17580-f7ea-4c4e-92e7-0c77c68185fe/ticket.toml`

<!-- ticket-index:entry id=96f9ffaa-6514-480a-afbd-b345cc206863 slug=open/session-api digest=cf6fae47a56a -->
#### [96f9ffaa] [session-api][handoff] Delta handoff serialization and workflow snapshot schema cleanup
- priority: `low`
- summary: Reduce handoff record size and remove structural smells so inter-agent records are compact and clean. **This is now a tracker parent decomposed into 2 subtickets** because it bundles one low-risk cle...
- ref: `memory-api/.ticket/tickets/96f9ffaa-6514-480a-afbd-b345cc206863/ticket.toml`

<!-- ticket-index:entry id=89d1f983-5081-4bfa-b22d-9398c5aae58c slug=open/session-api digest=cb05e410dfb9 -->
#### [89d1f983] [session-api][handoff] Delta/baseline handoff serialization with full reconstruct path
- priority: `low`
- summary: Make successive handoffs persist a compact delta instead of re-serializing ~97% identical bytes (subticket 2 of `96f9ffaa`). Depends on the schema cleanup (subticket 1) so the delta is defined over t...
- ref: `memory-api/.ticket/tickets/89d1f983-5081-4bfa-b22d-9398c5aae58c/ticket.toml`

<!-- ticket-index:entry id=68c8a5ef-fc7e-4700-956e-aea832c85011 slug=open/session-api digest=3943ee526914 -->
#### [68c8a5ef] [session-api][handoff] Flatten SessionWorkflowSnapshot and drop denormalized live_ticket_state
- priority: `low`
- summary: Remove the two structural smells in the handoff workflow snapshot (subticket 1 of `96f9ffaa`) — low-risk schema cleanup, independent of the delta work.
- ref: `memory-api/.ticket/tickets/68c8a5ef-fc7e-4700-956e-aea832c85011/ticket.toml`

<!-- ticket-index:entry id=0d3fdba6-45e6-4129-84f7-d98324c9519d slug=open/session-api digest=125f31b4f977 -->
#### [0d3fdba6] [session-api][handoff] Handoff completeness gate for session-created and owned entities
- priority: `medium`
- summary: Prevent a handoff from being "stale at birth" by detecting durable entities the run created or owned that are absent from the workflow graph and pinned entities.
- ref: `memory-api/.ticket/tickets/0d3fdba6-45e6-4129-84f7-d98324c9519d/ticket.toml`

<!-- ticket-index:entry id=0028f61c-e8b9-4fb4-81fd-5d1e37f7a429 slug=open/session-api digest=8b5f6375a069 -->
#### [0028f61c] [session-api][handoff] Make upward context and ticket narrative reproducible in handoff markdown
- ref: `.ticket/tickets/0028f61c-e8b9-4fb4-81fd-5d1e37f7a429/ticket.toml`

<!-- ticket-index:entry id=c77369eb-3fd0-44ca-ba8b-cec32132512e slug=open/session-api digest=e1bee79d1cfc -->
#### [c77369eb] [session-api][handoff] Model and enforce upward context for implementation-ready handoffs
- ref: `.ticket/tickets/c77369eb-3fd0-44ca-ba8b-cec32132512e/ticket.toml`

<!-- ticket-index:entry id=6431985e-e729-426b-9f91-66ad4b1c6fe6 slug=open/session-api digest=aaa1c4c58cf5 -->
#### [6431985e] [session-api][handoff] Persist blockers, resolved validation outcome, and structured narrative in handoff record
- priority: `high`
- summary: Make `SessionHandoffRecord` carry the durable knowledge-transfer payload that spec `8c880efc` AC5 and ticket `0647a212` scope already require, instead of stranding it in the chat transcript. **This i...
- ref: `memory-api/.ticket/tickets/6431985e-e729-426b-9f91-66ad4b1c6fe6/ticket.toml`

<!-- ticket-index:entry id=e8bdb7cf-6605-4879-ab48-2c67da57df65 slug=open/session-api digest=19507420cee3 -->
#### [e8bdb7cf] [session-api][handoff] Populate resolved validation outcome and new_entities at handoff creation
- priority: `high`
- summary: Populate the resolved validation `outcome` and `new_entities` when a handoff record is created (subticket 2 of `6431985e`). Depends on the model fields from subticket 1.
- ref: `memory-api/.ticket/tickets/e8bdb7cf-6605-4879-ab48-2c67da57df65/ticket.toml`

<!-- ticket-index:entry id=4f340766-ad9c-43a4-9ae6-d19518ce24fb slug=open/session-api digest=182eb3a9a70d -->
#### [4f340766] [session-api][handoff] Render resolved ticket narrative and upward context in handoff markdown
- ref: `.ticket/tickets/4f340766-ad9c-43a4-9ae6-d19518ce24fb/ticket.toml`

<!-- ticket-index:entry id=f77e35d8-8d74-4160-9ed0-85bf32c62a97 slug=open/session-api digest=b9c386749f80 -->
#### [f77e35d8] [session-api][handoff] Thread handoff narrative fields through CLI, MCP, and /handoff prompt
- priority: `medium`
- summary: Let agents supply structured handoff narrative through the surfaces instead of pasting prose into chat (subticket 3 of `6431985e`). Depends on model (subticket 1) and population (subticket 2).
- ref: `memory-api/.ticket/tickets/f77e35d8-8d74-4160-9ed0-85bf32c62a97/ticket.toml`

<!-- ticket-index:entry id=e731d333-7c11-4f9d-aec5-277c84be3796 slug=open/session-api digest=77ec1822cb95 -->
#### [e731d333] [session-api][runtime] Resume-time pin resolution and dead-pin garbage collection
- priority: `medium`
- summary: Validate pinned entity URNs at init/resume and surface or prune unresolved pins instead of silently failing instruction rendering and carrying dead pins forward.
- ref: `memory-api/.ticket/tickets/e731d333-7c11-4f9d-aec5-277c84be3796/ticket.toml`

<!-- ticket-index:entry id=ff83caf7-059b-4f2e-a0fb-eaa7757096a8 slug=open/session-api digest=bcbf2e8c3d59 -->
#### [ff83caf7] [session-api][tooling] Managed session-worktree lifecycle: preserve, reuse, rename, and finish
- priority: `high`
- summary: Make the session worktree a MANAGED, long-lived resource with an explicit lifecycle, instead of an ad-hoc directory created per request and abandoned. A session should get one worktree, keep it as th...
- ref: `.ticket/tickets/ff83caf7-059b-4f2e-a0fb-eaa7757096a8/ticket.toml`

<!-- ticket-index:entry id=3d535b2c-7361-4f08-bfb4-63b0b3174afc slug=open/session-api digest=4c7eae812a0d -->
#### [3d535b2c] [session-api][workflow] Add prompt-time worktree bootstrap hook
- priority: `high`
- summary: Add a pre-session bootstrap hook that establishes the session and its authoritative
- ref: `.ticket/tickets/3d535b2c-7361-4f08-bfb4-63b0b3174afc/ticket.toml`


### Component: session-cli

<!-- ticket-index:entry id=06407599-b504-4a2a-a71e-cd33ca69b317 slug=open/session-cli digest=24cf5fba66ec -->
#### [06407599] session-cli handoff hardcodes package: None, blocking handoff packages over the CLI transport
- priority: `low`
- summary: `session-cli handoff` hardcodes the handoff `package` argument to `None`, so handoff packages cannot be supplied over the CLI transport even though the underlying `session-api` (`SessionRuntime::crea...
- ref: `memory-api/.ticket/tickets/06407599-b504-4a2a-a71e-cd33ca69b317/ticket.toml`


### Component: spec

<!-- ticket-index:entry id=3d19a0a1-a845-54c0-acc5-e206b1cab457 slug=open/spec digest=176342f2c323 -->
#### [3d19a0a1] [feedback-followup][spec] Address mixed feedback on ce://default/spec/96bc688a-62ac-4083-923f-e507f2bb19fe
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/spec/96bc688a-62ac-4083-923f-e507f2bb19fe` during session `6f9208a4-c40e-4010-abf8-023505b4bf97` (tool call `call_Dw8Ts8bDIwMgdLvrpN1JqRDp`).
- ref: `.ticket/tickets/3d19a0a1-a845-54c0-acc5-e206b1cab457/ticket.toml`

<!-- ticket-index:entry id=58576ac3-3180-485d-876f-60fb5c84180c slug=open/spec digest=22cc2ec4b789 -->
#### [58576ac3] [workflow-tools][per-tool] Extract spec tool as a single `spec` domain crate (api + transport bins) + viewer frontend
- priority: `high`
- summary: Phase B. Extract the spec tool into its own `spec` repository (owner mankinskin), built as a single `spec` domain crate per contract `0da6894c`: the crate lib re-exports the internal `spec-api` crate...
- ref: `.ticket/tickets/58576ac3-3180-485d-876f-60fb5c84180c/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=c64f2384-0a65-420e-9dfc-e8814f111805 slug=open/spec-api digest=11dc587681a9 -->
#### [c64f2384] [memory-index] Spec index: single-line hierarchical entries, collapsible component groups, clickable spec links
- priority: `high`
- summary: Improve the `.spec/README.md` generated index so each entry is a compact single-line link rendered in true hierarchy order with collapsible component groups.
- ref: `.ticket/tickets/c64f2384-0a65-420e-9dfc-e8814f111805/ticket.toml`

<!-- ticket-index:entry id=29bf9628-1dc5-4bb4-ae00-b7410dd52db5 slug=open/spec-api digest=05327bebf1c4 -->
#### [29bf9628] [spec-api] Add direct feedback on spec entities with integration tests
- priority: `high`
- summary: Agents can now attach ratings and notes to canonical rule entries, but they still cannot attach feedback directly to native `spec-api` entities. Today the only supported workaround is to resolve a ge...
- ref: `memory-api/.ticket/tickets/29bf9628-1dc5-4bb4-ae00-b7410dd52db5/ticket.toml`

<!-- ticket-index:entry id=f4b1bf95-55bc-457f-93e9-1aaa05aa8814 slug=open/spec-api digest=06e296530afb -->
#### [f4b1bf95] [spec-api][refs] spec_refs_validate silently passes dangling .test guard-spec references
- priority: `medium`
- summary: Make `spec refs validate` catch references to non-existent test/validation guard specs instead of reporting a clean pass.
- ref: `memory-api/.ticket/tickets/f4b1bf95-55bc-457f-93e9-1aaa05aa8814/ticket.toml`

<!-- ticket-index:entry id=5318aedd-5188-4bfa-ad7d-a6d76e3243f1 slug=open/spec-api digest=91bd22b3e7b0 -->
#### [5318aedd] [spec-mcp][spec-http] Workspace-resolution parity — nested-root awareness + pure transport
- priority: `medium`
- summary: Follow-up after the ticket-domain first run. Adopt the same shared memory-api resolver + pure-transport pattern for the spec transports.
- ref: `memory-api/.ticket/tickets/5318aedd-5188-4bfa-ad7d-a6d76e3243f1/ticket.toml`

<!-- ticket-index:entry id=f22d5297-3f60-4161-bf90-1eb56f3ced5d slug=open/spec-api digest=b6b5b2e4d76b -->
#### [f22d5297] [spec] spec-api: list canonical component entities
- summary: `SpecManifest` already carries a `component` field, and the spec HTTP layer can filter specs by component, but there is no first-class way to list the canonical component set from `spec-api`.
- ref: `.ticket/tickets/f22d5297-3f60-4161-bf90-1eb56f3ced5d/ticket.toml`

<!-- ticket-index:entry id=0f33944e-fab7-4d8c-b3bb-3c665d51854f slug=open/spec-api digest=ebc5a89adf57 -->
#### [0f33944e] [spec][P3] spec-api: edge management (parent_of, linked, depends_on)
- priority: `high`
- summary: `spec-api`'s schema declares three edge kinds (`parent_of`, `linked`,
- ref: `memory-api/.ticket/tickets/0f33944e-fab7-4d8c-b3bb-3c665d51854f/ticket.toml`


### Component: spec-system

<!-- ticket-index:entry id=ee3864e1-7f9a-4804-bff0-8d861f4549da slug=open/spec-system digest=337fb19a9abb -->
#### [ee3864e1] Epic: Specification System — memory-api extraction, spec-api, tooling, and skill generation
- priority: `critical`
- summary: Build a complete specification and documentation management system that:
- ref: `memory-api/.ticket/tickets/ee3864e1-7f9a-4804-bff0-8d861f4549da/ticket.toml`


### Component: spec-viewer

<!-- ticket-index:entry id=88f87410-e0fa-4196-a461-805050670d08 slug=open/spec-viewer digest=657840df66e6 -->
#### [88f87410] [spec-viewer] Integrate graph improvements (selection, rendering tiers, panel framing, 2D mode)
- priority: `high`
- summary: Integrate the four graph improvements into spec-viewer:
- ref: `.ticket/tickets/88f87410-e0fa-4196-a461-805050670d08/ticket.toml`


### Component: spec-vscode

<!-- ticket-index:entry id=79b0ad85-c57d-4237-bc59-281fa1ad57f8 slug=open/spec-vscode digest=b117d6d32c88 -->
#### [79b0ad85] [spec][vscode] Design and implement spec-vscode VS Code extension
- priority: `high`
- summary: Design and implement `spec-vscode` — a VS Code extension that surfaces the spec store in the sidebar, mirroring the patterns established by `ticket-vscode`. The extension allows developers to browse,...
- ref: `memory-api/.ticket/tickets/79b0ad85-c57d-4237-bc59-281fa1ad57f8/ticket.toml`


### Component: specification

<!-- ticket-index:entry id=22967cfb-f3c7-4c72-8ca3-2edec0726891 slug=open/specification digest=54add7056dde -->
#### [22967cfb] Close specification-to-ticket traceability defect
- priority: `high`
- summary: Close the missing specification-to-ticket traceability defect.
- ref: `.ticket/tickets/22967cfb-f3c7-4c72-8ca3-2edec0726891/ticket.toml`

<!-- ticket-index:entry id=e9109814-45e9-4597-a55a-be736325c18e slug=open/specification digest=5e68214f43a1 -->
#### [e9109814] Phase 1: Refine coverage-critical contracts
- priority: `high`
- summary: Resolve the contract ambiguities that block reliable test design.
- ref: `.ticket/tickets/e9109814-45e9-4597-a55a-be736325c18e/ticket.toml`


### Component: test

<!-- ticket-index:entry id=eab4cd40-e8b3-4d5e-8985-91e3ffce519b slug=open/test digest=c3552aaef90e -->
#### [eab4cd40] [workflow-tools][per-tool] Extract test tool as a single `test` domain crate (api + transport bins)
- priority: `high`
- summary: Phase B. Extract the test tool into its own `test` repository (owner mankinskin), built as a single `test` domain crate per contract `0da6894c`: the crate lib re-exports the internal `test-api` crate...
- ref: `.ticket/tickets/eab4cd40-e8b3-4d5e-8985-91e3ffce519b/ticket.toml`


### Component: test-api

<!-- ticket-index:entry id=01964def-4496-4155-9abe-eb4e9f0520d3 slug=open/test-api digest=31ba2979165d -->
#### [01964def] [bench] Scale-sensitive latency fixtures + meaningful budgets
- priority: `high`
- summary: The parent exists because `ticket get` once took 96–107s. The current benchmark measures ~347ms on a 1-ticket fixture under a 2s budget — it cannot reproduce or flag that regression class. Add a scal...
- ref: `memory-api/.ticket/tickets/01964def-4496-4155-9abe-eb4e9f0520d3/ticket.toml`

<!-- ticket-index:entry id=1bc3982c-e0c2-4b6a-b809-aff4eb78d161 slug=open/test-api digest=e847d1f27ce4 -->
#### [1bc3982c] [memory-api][test] Real end-to-end validation surface — transports, provenance, representative fixtures (sub-tracker)
- priority: `high`
- summary: Sub-tracker under `a0bc8bd8` (Unified validation & benchmark surface in test-api).
- ref: `memory-api/.ticket/tickets/1bc3982c-e0c2-4b6a-b809-aff4eb78d161/ticket.toml`

<!-- ticket-index:entry id=a0bc8bd8-3fe0-4768-a895-b1bacee42759 slug=open/test-api digest=1c07cd1575d0 -->
#### [a0bc8bd8] [memory-api][test] Unified validation & benchmark surface in test-api (tracker)
- priority: `high`
- summary: Build one extensible, executable testing surface anchored in `test-api` that, at minimum, delivers:
- ref: `memory-api/.ticket/tickets/a0bc8bd8-3fe0-4768-a895-b1bacee42759/ticket.toml`

<!-- ticket-index:entry id=a72e3aca-1e95-4fc5-a5b9-701112dcc37e slug=open/test-api digest=e75ac669a059 -->
#### [a72e3aca] [memory-index] Test store catalog generator
- priority: `medium`
- summary: Build a generator that reads the test-api and log-api evidence stores and emits a markdown test catalog at `.test/README.md` and `.test/index.toon`. Gated behind dependent log/test-api bootstrap comp...
- ref: `.ticket/tickets/a72e3aca-1e95-4fc5-a5b9-701112dcc37e/ticket.toml`

<!-- ticket-index:entry id=5a4c2e4d-e7d9-4138-8f25-c699942f739a slug=open/test-api digest=4c69ccb0bf2b -->
#### [5a4c2e4d] [test-api] Add first-class validation spec and result storage
- priority: `high`
- summary: Add a first-class `test-api` for validation specifications and validation results in the memory system.
- ref: `.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a/ticket.toml`

<!-- ticket-index:entry id=274c5119-02cb-4306-b017-b998659ea514 slug=open/test-api digest=00027c6a2e37 -->
#### [274c5119] [test-api] Backfill existing repo test/bench suites into the store
- priority: `medium`
- summary: Ingest the repo's actual `cargo test`/`cargo bench` corpus (and, eventually, TS/browser suites) so the test-api index reflects what actually runs — not just the ~14 hand-authored `vt-*` specs.
- ref: `memory-api/.ticket/tickets/274c5119-02cb-4306-b017-b998659ea514/ticket.toml`

<!-- ticket-index:entry id=8f364a0c-35ab-4faa-b49a-20d98b6f2905 slug=open/test-api digest=4e16fc7084b1 -->
#### [8f364a0c] [test-api][browser] Structured subprocess result adapter and artifact provenance
- priority: `high`
- summary: Implement the versioned structured reporter envelope and repository-native subprocess adapter specified by `test-api/browser-result-ingestion`.
- ref: `memory-api/.ticket/tickets/8f364a0c-35ab-4faa-b49a-20d98b6f2905/ticket.toml`

<!-- ticket-index:entry id=2dada4b7-e5c0-4fe5-be01-df9936659133 slug=open/test-api digest=259ade008a43 -->
#### [2dada4b7] [test-api][ci] Test profiles + CI lanes — fast-on-push vs. large-on-demand
- priority: `medium`
- summary: Provide a **test-profile** mechanism that selects which validation/benchmark cells run, and wire two CI lanes (D6, D10):
- ref: `memory-api/.ticket/tickets/2dada4b7-e5c0-4fe5-be01-df9936659133/ticket.toml`


### Component: testing

<!-- ticket-index:entry id=7c3e47db-90fa-4ad1-8f0d-b52deaeb65b8 slug=open/testing digest=cb3ae4576ebd -->
#### [7c3e47db] Adopt property-based testing foundations
- priority: `high`
- summary: Adopt `proptest` for parser, selector, and path equivalence classes.
- ref: `.ticket/tickets/7c3e47db-90fa-4ad1-8f0d-b52deaeb65b8/ticket.toml`

<!-- ticket-index:entry id=e26373a3-3049-43e0-8b2f-f55867cf39d2 slug=open/testing digest=b131afa57449 -->
#### [e26373a3] Build multi-store sandbox and replay harness
- priority: `high`
- summary: Build reusable multi-store sandbox and replay harness infrastructure.
- ref: `.ticket/tickets/e26373a3-3049-43e0-8b2f-f55867cf39d2/ticket.toml`

<!-- ticket-index:entry id=eea328bf-339e-4d1a-b88b-ee1ec1bc9192 slug=open/testing digest=e92f25eedaa1 -->
#### [eea328bf] Build sanitized fixture convention
- priority: `high`
- summary: Build fixture sanitization and `tests/fixtures/<surface>/<scenario>-v1/` convention.
- ref: `.ticket/tickets/eea328bf-339e-4d1a-b88b-ee1ec1bc9192/ticket.toml`

<!-- ticket-index:entry id=1a296e04-55e6-4492-b93b-6ea7ed58c9ae slug=open/testing digest=4d64d3b34476 -->
#### [1a296e04] Evaluate automated coverage tooling
- priority: `medium`
- summary: Evaluate `cargo-llvm-cov` and `tarpaulin` for CI cost, report hosting, and useful signals.
- ref: `.ticket/tickets/1a296e04-55e6-4492-b93b-6ea7ed58c9ae/ticket.toml`

<!-- ticket-index:entry id=33e1118f-119d-4c03-939e-ff568b054889 slug=open/testing digest=7475b8192a5e -->
#### [33e1118f] Phase 2: Establish test foundations
- priority: `high`
- summary: Build reusable fixture, sandbox, replay, and generative-test foundations.
- ref: `.ticket/tickets/33e1118f-119d-4c03-939e-ff568b054889/ticket.toml`

<!-- ticket-index:entry id=b8ff29d0-613a-416f-93d2-d0befc487d17 slug=open/testing digest=dd76de3326ff -->
#### [b8ff29d0] Phase 4: Verify transport matrices
- priority: `high`
- summary: Test contract parity and negotiated behavior across CLI, MCP, and HTTP transports.
- ref: `.ticket/tickets/b8ff29d0-613a-416f-93d2-d0befc487d17/ticket.toml`

<!-- ticket-index:entry id=69525949-295b-4308-afd5-06dd87f4e25c slug=open/testing digest=a34343bee8e3 -->
#### [69525949] Phase 5: Cover remaining high-risk surfaces
- priority: `high`
- summary: Raise confidence in filesystem, tool-gate, feedback, macro, thin-coverage, viewer, and performance surfaces.
- ref: `.ticket/tickets/69525949-295b-4308-afd5-06dd87f4e25c/ticket.toml`

<!-- ticket-index:entry id=e44c6fc5-a494-483e-a390-0a098e8bf2ff slug=open/testing digest=2c80410986b7 -->
#### [e44c6fc5] Phase 6: Repair weak and superseded tests
- priority: `medium`
- summary: Repair known non-semantic tests and remove superseded coverage.
- ref: `.ticket/tickets/e44c6fc5-a494-483e-a390-0a098e8bf2ff/ticket.toml`

<!-- ticket-index:entry id=fe913b41-4fea-45c2-9d2b-b60964bb9eea slug=open/testing digest=6082ed6ad752 -->
#### [fe913b41] Raise coverage for thinly tested crates
- priority: `medium`
- summary: Improve targeted coverage for thinly tested crates.
- ref: `.ticket/tickets/fe913b41-4fea-45c2-9d2b-b60964bb9eea/ticket.toml`

<!-- ticket-index:entry id=acebde24-0620-48c0-b9b0-723e2e2dd8a4 slug=open/testing digest=051408bdaedf -->
#### [acebde24] Repair weak and superseded tests
- priority: `medium`
- summary: Repair weak tests and remove superseded cases after new coverage lands.
- ref: `.ticket/tickets/acebde24-0620-48c0-b9b0-723e2e2dd8a4/ticket.toml`

<!-- ticket-index:entry id=1b58aaf5-b70e-4202-8cc9-747c3515e5ba slug=open/testing digest=95f674cae4c5 -->
#### [1b58aaf5] Test coverage and contract hardening program
- priority: `high`
- summary: Coordinate the coverage and contract-hardening program across transport, fixture, replay, reliability, and process work.
- ref: `.ticket/tickets/1b58aaf5-b70e-4202-8cc9-747c3515e5ba/ticket.toml`


### Component: ticket

<!-- ticket-index:entry id=ce0c35c2-83e0-4dd2-8ed2-b919bd3866a5 slug=open/ticket digest=391b922031f4 -->
#### [ce0c35c2] Decide file-contention execution-order tie-break policy
- summary: Define the canonical deterministic tie-break for independent tickets that contend on the same normalized target path, including the evidence used to order the pair and the execution-order edge reason...
- ref: `.ticket/tickets/ce0c35c2-83e0-4dd2-8ed2-b919bd3866a5/ticket.toml`

<!-- ticket-index:entry id=62a1663d-7c4b-4676-83a5-9f6707abe099 slug=open/ticket digest=51dec60deb79 -->
#### [62a1663d] Derive and render latent edges: transitive dependencies and execution-order edges
- priority: `medium`
- summary: The ticket store persists only direct `depends_on` edges. Two useful edge
- ref: `.ticket/tickets/62a1663d-7c4b-4676-83a5-9f6707abe099/ticket.toml`

<!-- ticket-index:entry id=ceb3fc56-52f8-42e0-917b-4a2e4dcb7d1e slug=open/ticket digest=b3c4cbc1e64e -->
#### [ceb3fc56] Derive and render latent edges: transitive dependencies and execution-order edges
- priority: `medium`
- summary: The ticket store persists only direct `depends_on` edges. Two useful edge
- ref: `.ticket/tickets/ceb3fc56-52f8-42e0-917b-4a2e4dcb7d1e/ticket.toml`

<!-- ticket-index:entry id=d905b50a-c103-50c1-8552-509a014cc149 slug=open/ticket digest=57b00c40b444 -->
#### [d905b50a] [feedback-followup][ticket] Address mixed feedback on ce://default/ticket/3c6da958-f494-408f-b7dd-cc43997b8ead
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/3c6da958-f494-408f-b7dd-cc43997b8ead` during session `6f9208a4-c40e-4010-abf8-023505b4bf97` (tool call `call_xH9yX3C0tL8uxSpFxIhcG5wA`).
- ref: `.ticket/tickets/d905b50a-c103-50c1-8552-509a014cc149/ticket.toml`

<!-- ticket-index:entry id=eec6049b-5844-595c-8060-84c6a6103252 slug=open/ticket digest=8449d1e7d64e -->
#### [eec6049b] [feedback-followup][ticket] Address not-helpful feedback on ce://default/ticket/6a47ab0f-7e42-463e-afe0-bf51b85249c9
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/6a47ab0f-7e42-463e-afe0-bf51b85249c9` during session `e31bd0e5-ab29-4e76-9284-5f3d2067f40c` (tool call `toolu_01Mx5YQM8t3Fb2CyqEFkGpNW`).
- ref: `.ticket/tickets/eec6049b-5844-595c-8060-84c6a6103252/ticket.toml`

<!-- ticket-index:entry id=7d82c008-bed3-5c8b-b31d-26de96df2bf9 slug=open/ticket digest=4a59f97f380c -->
#### [7d82c008] [feedback-followup][ticket] Address not-helpful feedback on ce://default/ticket/7ef3f8db-d4a9-4135-99eb-3c006070a328
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/7ef3f8db-d4a9-4135-99eb-3c006070a328` during session `82c8b373-b0ef-4e29-b449-6b48d5fbd87e` (tool call `call_VrRUVqDGzTLGoRe0zRkzJc95`).
- ref: `.ticket/tickets/7d82c008-bed3-5c8b-b31d-26de96df2bf9/ticket.toml`

<!-- ticket-index:entry id=b566755e-84ca-5560-b4d3-03a960af9f87 slug=open/ticket digest=8b24beabeb8d -->
#### [b566755e] [feedback-followup][ticket] Address not-helpful feedback on ce://default/ticket/85012858-cbf3-40df-b55e-b82e89f72434
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/85012858-cbf3-40df-b55e-b82e89f72434` during session `82c8b373-b0ef-4e29-b449-6b48d5fbd87e` (tool call `call_GIBRALy3hHNbenu044bp6Wx3`).
- ref: `.ticket/tickets/b566755e-84ca-5560-b4d3-03a960af9f87/ticket.toml`

<!-- ticket-index:entry id=c78de3bd-d665-5a3a-a19e-de497eb28369 slug=open/ticket digest=11dbde43b210 -->
#### [c78de3bd] [feedback-followup][ticket] Address not-helpful feedback on ce://default/ticket/8fdfe135-e3b1-4876-b638-24154edcd78d
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/8fdfe135-e3b1-4876-b638-24154edcd78d` during session `82c8b373-b0ef-4e29-b449-6b48d5fbd87e` (tool call `call_q7SdrA89SLgTphVGMYoTvjRk`).
- ref: `.ticket/tickets/c78de3bd-d665-5a3a-a19e-de497eb28369/ticket.toml`

<!-- ticket-index:entry id=0f6a812d-0fa4-5f06-93bb-b438b530e720 slug=open/ticket digest=3707fe348438 -->
#### [0f6a812d] [feedback-followup][ticket] Address not-helpful feedback on ce://default/ticket/9e450826-60e1-437f-b236-2c8839e4ab9e
- priority: `medium`
- summary: Explicit feedback was recorded against `ce://default/ticket/9e450826-60e1-437f-b236-2c8839e4ab9e` during session `82c8b373-b0ef-4e29-b449-6b48d5fbd87e` (tool call `call_wh3i0rMlXK1KPKA7SepSmivh`).
- ref: `.ticket/tickets/0f6a812d-0fa4-5f06-93bb-b438b530e720/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=1f8e6e6d-c8ea-461d-83c9-c26daf0e3cd3 slug=open/ticket-api digest=76fd19125bb0 -->
#### [1f8e6e6d] Add deterministic dual-format schema loading
- priority: `high`
- summary: Extend schema loading for the contract in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).
- ref: `.ticket/tickets/1f8e6e6d-c8ea-461d-83c9-c26daf0e3cd3/ticket.toml`

<!-- ticket-index:entry id=7e7a5f21-2da6-4d22-bff7-ee2b1296a527 slug=open/ticket-api digest=6aed1b7d3413 -->
#### [7e7a5f21] Complete description->parts migration for tickets stuck in state `planned`
- priority: `medium`
- summary: The store-wide description→parts migration (ticket f65f2b32, memory-api/crates/ticket-api/src/storage/store/migration.rs) deliberately skips tickets currently in state `planned` (`MigrationApplyRepor...
- ref: `memory-api/.ticket/tickets/7e7a5f21-2da6-4d22-bff7-ee2b1296a527/ticket.toml`

<!-- ticket-index:entry id=ccb7b132-10c7-463c-b9d3-12b7c72f07ae slug=open/ticket-api digest=9d33fed2f037 -->
#### [ccb7b132] Define graph traversal semantics for spec 42e8d710
- priority: `high`
- summary: Refine spec `42e8d710` with exact traversal flags and BFS behavior.
- ref: `.ticket/tickets/ccb7b132-10c7-463c-b9d3-12b7c72f07ae/ticket.toml`

<!-- ticket-index:entry id=abd3f280-9bd1-48cf-8503-17dd820afb30 slug=open/ticket-api digest=00faec6420d0 -->
#### [abd3f280] Generate resolved schema catalog and JSON built-ins
- priority: `high`
- summary: Replace shipped ticket-schema TOML sources with generated JSON built-ins backed by the resolved registry manifest in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-...
- ref: `.ticket/tickets/abd3f280-9bd1-48cf-8503-17dd820afb30/ticket.toml`

<!-- ticket-index:entry id=7df984eb-c96c-4501-98fa-4e88dd28ec4e slug=open/ticket-api digest=d1d7fa6a8129 -->
#### [7df984eb] Inventory and migrate legacy ticket schemas
- priority: `high`
- summary: Inventory and migrate legacy `task`, `feature`, `tracker-improvement`, and upgraded `bug`/`epic` records safely.
- ref: `.ticket/tickets/7df984eb-c96c-4501-98fa-4e88dd28ec4e/ticket.toml`

<!-- ticket-index:entry id=f7a0f5b5-c584-46c2-9033-e54fbd2ac670 slug=open/ticket-api digest=4657d07ce217 -->
#### [f7a0f5b5] Remove cross-worktree board claims and reconcile deleted-worktree entries
- priority: `high`
- summary: Ensure board state cannot persist active dependencies on another worktree's absolute path or display deleted worktrees as live claims.
- ref: `.ticket/tickets/f7a0f5b5-c584-46c2-9033-e54fbd2ac670/ticket.toml`

<!-- ticket-index:entry id=c320708d-8dc1-46e2-ab5d-9903460ae27a slug=open/ticket-api digest=d1155e7ebd54 -->
#### [c320708d] Remove the stale duplicate board module in memory-kernel
- priority: `medium`
- summary: Two copies of the same board module exist, confirmed by skeleton inspection:
- ref: `.ticket/tickets/c320708d-8dc1-46e2-ab5d-9903460ae27a/ticket.toml`

<!-- ticket-index:entry id=614fae19-cb51-4f38-96cc-40012d1adaae slug=open/ticket-api digest=04018046d3a8 -->
#### [614fae19] Resolve board/session worktree-claim conflict with orchestrator-provisioned worktrees
- priority: `high`
- summary: The orchestration protocol in `.agents/instructions/commit/branch-worktree.instructions.md` requires each implementation unit to run in its own freshly provisioned worktree on its own branch. Claims ...
- ref: `.ticket/tickets/614fae19-cb51-4f38-96cc-40012d1adaae/ticket.toml`

<!-- ticket-index:entry id=8fdfe135-e3b1-4876-b638-24154edcd78d slug=open/ticket-api digest=21a9d9fdca84 -->
#### [8fdfe135] Schema modernization implementation track
- priority: `high`
- summary: Implement the schema-modernization contract established in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).
- ref: `.ticket/tickets/8fdfe135-e3b1-4876-b638-24154edcd78d/ticket.toml`

<!-- ticket-index:entry id=bbb4bce9-d57c-4f85-8757-8d239f9f7cde slug=open/ticket-api digest=a1124513ff1d -->
#### [bbb4bce9] Structured Ticket Entities (track root)
- priority: `high`
- summary: Turn a ticket entity from a single mutable description blob into a structured, partially-frozen, multi-file mini-plan, so that agents can no longer destroy planned content and can read only the parts...
- ref: `memory-api/.ticket/tickets/bbb4bce9-d57c-4f85-8757-8d239f9f7cde/ticket.toml`

<!-- ticket-index:entry id=fed4fd46-341b-450a-98a8-60c080d24144 slug=open/ticket-api digest=1617be7f4bf2 -->
#### [fed4fd46] Test graph traversal on CLI and MCP
- priority: `high`
- summary: Test `subgraph` and `topgraph` behavior on CLI and MCP.
- ref: `.ticket/tickets/fed4fd46-341b-450a-98a8-60c080d24144/ticket.toml`

<!-- ticket-index:entry id=3bb41fb2-9907-4b54-83b1-c62a0ce96756 slug=open/ticket-api digest=d9c5cfc6a804 -->
#### [3bb41fb2] Validate schema modernization release and repair flow
- priority: `high`
- summary: Provide the release gate and forward-repair protocol for schema modernization.
- ref: `.ticket/tickets/3bb41fb2-9907-4b54-83b1-c62a0ce96756/ticket.toml`

<!-- ticket-index:entry id=3a5df74c-2192-4187-b048-3f6285f20db4 slug=open/ticket-api digest=761632cb9c9a -->
#### [3a5df74c] [memory-index] Ticket index: one-line entries, state-ordered, collapsible state headers, clickable manifest links
- priority: `high`
- summary: Improve the `.ticket/README.md` generated index so each entry is compact, actionable-first, and navigable.
- ref: `.ticket/tickets/3a5df74c-2192-4187-b048-3f6285f20db4/ticket.toml`

<!-- ticket-index:entry id=f2285a55-91d1-48ad-9af7-e8c55ce9bd4d slug=open/ticket-api digest=f3df81797b8c -->
#### [f2285a55] [spec] spec-api <-> ticket-api: link component entities and detect drift
- summary: Tickets still exist with stale component values such as `log-viewer-leptos`, even though that component has been removed. `ticket-api` currently treats `fields.component` as a free-form string, so ou...
- ref: `.ticket/tickets/f2285a55-91d1-48ad-9af7-e8c55ce9bd4d/ticket.toml`

<!-- ticket-index:entry id=eb687360-5765-4efb-b3d6-380a7691fc66 slug=open/ticket-api digest=8cfd765d4d09 -->
#### [eb687360] [ticket-api] Review the added 'feature' delivered ticket type schema
- summary: The graded cost-gate work added a new `feature` ticket type schema to shared ticketing infra without its own tracking ticket:
- ref: `.ticket/tickets/eb687360-5765-4efb-b3d6-380a7691fc66/ticket.toml`

<!-- ticket-index:entry id=7729df51-d4f6-464e-838b-63ce8fc02859 slug=open/ticket-api digest=e92c6660eb80 -->
#### [7729df51] [ticket-api] Tickets with deleted:true still surface in list_tickets
- priority: `medium`
- summary: Ticket `e42d8e0a-c210-4efe-a22c-2565079e67b8` (memory-viewers/.ticket) carried the field `deleted: true` in its manifest yet still appeared in `list_tickets` output and resolved normally through `get...
- ref: `.ticket/tickets/7729df51-d4f6-464e-838b-63ce8fc02859/ticket.toml`

<!-- ticket-index:entry id=9d3d9120-b6d7-4632-ba8e-dbba63857207 slug=open/ticket-api digest=1e29f842b960 -->
#### [9d3d9120] [ticket-api] scan --reindex fails reading a segment file it just wrote
- priority: `high`
- summary: `ticket scan --reindex --force` fails while rebuilding the ticket store search index on Windows. The authoritative on-disk ticket store remains readable, but the derived index cannot be reconciled.
- ref: `.ticket/tickets/9d3d9120-b6d7-4632-ba8e-dbba63857207/ticket.toml`

<!-- ticket-index:entry id=de36ac72-7f86-44af-875b-fd92bd628be9 slug=open/ticket-api digest=4f6b23b8e6a3 -->
#### [de36ac72] [ticket-api][ticket-cli][ticket-mcp][ticket-http][ticket-viewer] Implement blocker trees and recently-unblocked ordering
- priority: `high`
- summary: Implement blocker-tree workflow exploration and recently-unblocked ordering on top of the shared dependency-convergence model.
- ref: `.ticket/tickets/de36ac72-7f86-44af-875b-fd92bd628be9/ticket.toml`

<!-- ticket-index:entry id=ec9498ee-e7a0-4b93-9543-393ac48c08fa slug=open/ticket-api digest=5f160845a688 -->
#### [ec9498ee] [ticket-api][ticket-http][ticket-viewer] Harden workspace linking and search architecture
- priority: `high`
- summary: Harden workspace linking and search beyond the immediate bug fixes by closing the remaining design holes identified in review.
- ref: `.ticket/tickets/ec9498ee-e7a0-4b93-9543-393ac48c08fa/ticket.toml`

<!-- ticket-index:entry id=32e41b08-0177-4572-86b7-f5bbb25edeab slug=open/ticket-api digest=019b540c359d -->
#### [32e41b08] [ticket-api][ticket-mcp] No get_ticket view profile exposes affected/claimed file paths, causing false not-started triage classifications
- priority: `medium`
- summary: No `get_ticket` view profile exposes the ticket's affected/claimed file paths. Verified during a 2026-07-31 bulk triage:
- ref: `.ticket/tickets/32e41b08-0177-4572-86b7-f5bbb25edeab/ticket.toml`

<!-- ticket-index:entry id=5718f77c-fea0-47ca-883c-98361a821fb6 slug=open/ticket-api digest=715cedcdc314 -->
#### [5718f77c] [ticket-api][workflow] Dependency-progress guard blocks parking and demotion transitions
- priority: `high`
- summary: `enforce_dependency_progress` in [memory-api/crates/ticket-api/src/storage/store.rs](memory-api/crates/ticket-api/src/storage/store.rs#L924-L955) applies the dependency-ordering guard to ALL non-`can...
- ref: `.ticket/tickets/5718f77c-fea0-47ca-883c-98361a821fb6/ticket.toml`

<!-- ticket-index:entry id=70019883-b89d-496d-b7d6-a7fea91a0814 slug=open/ticket-api digest=1506940ca841 -->
#### [70019883] next_tickets --root does not traverse containment edges to reach actionable leaves
- priority: `medium`
- summary: Objective
- ref: `.ticket/tickets/70019883-b89d-496d-b7d6-a7fea91a0814/ticket.toml`

<!-- ticket-index:entry id=35a60203-0a2c-4dbc-b33d-b645848871f2 slug=open/ticket-api digest=458ca2c8922d -->
#### [35a60203] ticket-mcp index goes stale against external store writes; reads report on-disk content as absent
- priority: `high`
- summary: Make MCP reads trustworthy after external mutations to the ticket store.
- ref: `.ticket/tickets/35a60203-0a2c-4dbc-b33d-b645848871f2/ticket.toml`


### Component: ticket-api,audit-api

<!-- ticket-index:entry id=5ad5ab28-6c81-4916-9574-d2c470e03a31 slug=open/ticket-api,audit-api digest=8c616bd4846e -->
#### [5ad5ab28] [ticket-api][audit-api] Strengthen canonical ticket health validation
- priority: `high`
- summary: Improve ticket health audit validation so the repository gets more signal from ticket quality checks, especially for newly created tickets.
- ref: `.ticket/tickets/5ad5ab28-6c81-4916-9574-d2c470e03a31/ticket.toml`


### Component: ticket-cli

<!-- ticket-index:entry id=7fa1ee01-ca71-407d-9ba7-9f77b236d84e slug=open/ticket-cli digest=4c81c9955848 -->
#### [7fa1ee01] Define generated TOON documentation examples for spec 6e63979a
- priority: `high`
- summary: Refine spec `6e63979a` so TOON documentation examples are generated and validated.
- ref: `.ticket/tickets/7fa1ee01-ca71-407d-9ba7-9f77b236d84e/ticket.toml`

<!-- ticket-index:entry id=e507818f-2e19-4156-9ec0-f6b6c881cbf2 slug=open/ticket-cli digest=5d2069580c23 -->
#### [e507818f] ticket-cli batch hangs indefinitely during link creation
- priority: `high`
- summary: Fix `ticket batch` so multi-edge link creation completes or fails with a bounded error.
- ref: `.ticket/tickets/e507818f-2e19-4156-9ec0-f6b6c881cbf2/ticket.toml`


### Component: ticket-engine

<!-- ticket-index:entry id=50092f61-f4fb-49a1-ac67-64c8cb7c513e slug=open/ticket-engine digest=5b59b5d89156 -->
#### [50092f61] Investigate workspace/root-resolution divergence behind assumed ticket-index staleness (cf. 1d6a033e)
- priority: `medium`
- summary: This session originally assumed a read-index staleness defect in ticket-api as a premise for spec ticket 54114fc9 (deterministic Rust execution controller). Research could not reproduce a staleness d...
- ref: `.ticket/tickets/50092f61-f4fb-49a1-ac67-64c8cb7c513e/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=56743ad1-b1c4-44a2-b41e-0bcd2eadcc16 slug=open/ticket-http digest=b2e226c26cd2 -->
#### [56743ad1] ticket_ref.workspace echoes a mismatched/unregistered workspace hash, 404ing viewer part fetches
- priority: `high`
- summary: `ticket_ref.workspace` in list/get HTTP responses echoes a workspace id that does not match the requested (and valid) `workspace` query parameter, and does not correspond to any workspace registered ...
- ref: `memory-api/.ticket/tickets/56743ad1-b1c4-44a2-b41e-0bcd2eadcc16/ticket.toml`


### Component: ticket-transports

<!-- ticket-index:entry id=b6b58573-01c6-43c0-b8de-59c9c2afbe3f slug=open/ticket-transports digest=1221e50f68b0 -->
#### [b6b58573] Define default output and transport negotiation for spec 1d62442b
- priority: `high`
- summary: Refine spec `1d62442b` for output defaults and transport negotiation.
- ref: `.ticket/tickets/b6b58573-01c6-43c0-b8de-59c9c2afbe3f/ticket.toml`

<!-- ticket-index:entry id=3824c5da-014b-47fb-b2c1-ab1935a4075a slug=open/ticket-transports digest=85471215da84 -->
#### [3824c5da] Define transport parity matrix for spec 9074b2ef
- priority: `high`
- summary: Refine spec `9074b2ef` with a strict machine-readable transport parity matrix.
- ref: `.ticket/tickets/3824c5da-014b-47fb-b2c1-ab1935a4075a/ticket.toml`

<!-- ticket-index:entry id=e7351b53-bd02-461f-8228-7ffbc59b84b9 slug=open/ticket-transports digest=11b11d1733ad -->
#### [e7351b53] Generate ticket transport parity matrix tests
- priority: `high`
- summary: Generate ticket CLI/MCP/HTTP parity tests from the P1.5 matrix artifact.
- ref: `.ticket/tickets/e7351b53-bd02-461f-8228-7ffbc59b84b9/ticket.toml`

<!-- ticket-index:entry id=b60e1bc2-3067-47fc-b01b-dd0db6f6effa slug=open/ticket-transports digest=7138788b899a -->
#### [b60e1bc2] Test output format negotiation
- priority: `high`
- summary: Test output defaults and explicit selectors across CLI, MCP, and HTTP.
- ref: `.ticket/tickets/b60e1bc2-3067-47fc-b01b-dd0db6f6effa/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=53a6d689-7d31-40ce-b807-4314285b4bfd slug=open/ticket-viewer digest=5b211c0c24c1 -->
#### [53a6d689] [ticket-viewer] Add mixed-workspace endpoint ownership matrix regression tests
- priority: `high`
- summary: Create endpoint-matrix regression tests that validate workspace ownership semantics for mixed-workspace ticket references.
- ref: `.ticket/tickets/53a6d689-7d31-40ce-b807-4314285b4bfd/ticket.toml`

<!-- ticket-index:entry id=0866e27f-ae67-4eb0-9199-00650317e7c3 slug=open/ticket-viewer digest=eadb5382f9a2 -->
#### [0866e27f] [ticket-viewer] Fix asset follow-up file selection and owning-workspace fetch
- priority: `high`
- summary: Clicking an expanded asset row in the ticket tree does not reliably propagate selected_file state and does not trigger the owning-workspace /asset follow-up request.
- ref: `.ticket/tickets/0866e27f-ae67-4eb0-9199-00650317e7c3/ticket.toml`

<!-- ticket-index:entry id=4629b9d9-3bd0-4ef6-82b6-d6e609c16cac slug=open/ticket-viewer digest=ac9fa22ab9db -->
#### [4629b9d9] [ticket-viewer] Migrate list/detail/search flows to workspace-aware ticket references
- priority: `high`
- summary: The Dioxus ticket-viewer currently treats `workspace` as one ambient string that owns everything in the app. That assumption is baked into routing, selection state, caches, SSE, batch selection, and ...
- ref: `memory-api/.ticket/tickets/4629b9d9-3bd0-4ef6-82b6-d6e609c16cac/ticket.toml`

<!-- ticket-index:entry id=178b4091-53c9-45ae-b975-890a23b5f25d slug=open/ticket-viewer digest=fe52bece4a50 -->
#### [178b4091] [ticket-viewer] Normalize release E2E suite to workspace-aware assumptions
- priority: `medium`
- summary: Reduce false confidence from legacy default-workspace assumptions by updating release E2E tests to workspace-aware ticket-reference behavior.
- ref: `.ticket/tickets/178b4091-53c9-45ae-b975-890a23b5f25d/ticket.toml`

<!-- ticket-index:entry id=c33419c2-3fff-4ce2-9b53-8882d6918e53 slug=open/ticket-viewer digest=984789b93c8f -->
#### [c33419c2] [ticket-viewer] Tracker: complete mixed-workspace regression coverage
- priority: `high`
- summary: This tracker captures the regression-test completeness review for the ticket-viewer mixed-workspace rollout.
- ref: `.ticket/tickets/c33419c2-3fff-4ce2-9b53-8882d6918e53/ticket.toml`

<!-- ticket-index:entry id=99ce3f10-ba51-4b03-9c57-fd7a1797f4fd slug=open/ticket-viewer digest=12fbf58641b9 -->
#### [99ce3f10] ticket-viewer release E2E suite fails to load due to a bad relative import path (0 tests run)
- priority: `medium`
- summary: memory-viewers/ticket-viewer/frontend/dioxus/e2e-release/shared/common-viewer-suite.ts imports from an incorrect relative path:
- ref: `memory-api/.ticket/tickets/99ce3f10-ba51-4b03-9c57-fd7a1797f4fd/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=9e7a5f1a-a2ce-43ce-9c8f-bdce7cf712d2 slug=open/ticket-vscode digest=ee96f581d134 -->
#### [9e7a5f1a] Integrate schema catalog into CLI and VS Code
- priority: `medium`
- summary: Replace hard-coded legacy ticket-type choices with catalog-driven client behavior.
- ref: `.ticket/tickets/9e7a5f1a-a2ce-43ce-9c8f-bdce7cf712d2/ticket.toml`


### Component: ticket-workflow

<!-- ticket-index:entry id=7fc7a10d-64a1-4c67-a5a9-5b45d8e03047 slug=open/ticket-workflow digest=427e20b8c313 -->
#### [7fc7a10d] Project tracker: expressive ticket query and ordering interface
- priority: `high`
- summary: The ticket query surface is still fragmented and under-specified.
- ref: `.ticket/tickets/7fc7a10d-64a1-4c67-a5a9-5b45d8e03047/ticket.toml`

<!-- ticket-index:entry id=8f999cfb-4e63-4315-bec7-e95224e41190 slug=open/ticket-workflow digest=1df07935b14d -->
#### [8f999cfb] [ticket-workflow] Board check-in should support claiming ticket-authoring/topic scope, not just implementation files
- priority: `high`
- summary: Board check-in (`mcp_ticket-mcp_board_check_in` / `board check-in`) claims implementation **files only** via `owned_files`. It has no mechanism for claiming "I am authoring tickets/specs under topic ...
- ref: `.ticket/tickets/8f999cfb-4e63-4315-bec7-e95224e41190/ticket.toml`


### Component: tooling

<!-- ticket-index:entry id=69e69b4b-77d8-4519-99cc-f9c95362fce4 slug=open/tooling digest=81e2e70e0c3e -->
#### [69e69b4b] Reject root-only worktree and branch operations invoked from inside a worktree
- priority: `high`
- summary: An agent ran `git checkout main` with its shell current directory inside `.worktrees/45bd0e3f-worktree-config-hijack`. Git returned `fatal: 'main' is already used by worktree at 'C:/Users/linus/git/c...
- ref: `.ticket/tickets/69e69b4b-77d8-4519-99cc-f9c95362fce4/ticket.toml`

<!-- ticket-index:entry id=e068602b-d6d0-425c-9e65-9cf3825bea65 slug=open/tooling digest=0398251ff8e3 -->
#### [e068602b] git worktree prune destroys a live worktree's submodule linked-worktree registration
- priority: `critical`
- summary: Running `git -C <submodule> worktree prune` across the five submodules removed `.git/modules/context-stack/worktrees/context-stack2` while the owning worktree was alive and healthy on disk.
- ref: `.ticket/tickets/e068602b-d6d0-425c-9e65-9cf3825bea65/ticket.toml`

<!-- ticket-index:entry id=f3c2b8a9-1d2e-4c3f-9b8a-0a1b2c3d4e5f slug=open/tooling digest=4e3fbf3a2246 -->
#### [f3c2b8a9] install-ctl: Generalize viewer-ctl into a general install/ctl for tool binaries and extensions
- priority: `high`
- summary: Problem
- ref: `.ticket/tickets/f3c2b8a9-1d2e-4c3f-9b8a-0a1b2c3d4e5f/ticket.toml`

<!-- ticket-index:entry id=503b9711-3f69-4765-88f9-83779b71c8f8 slug=open/tooling digest=2fc2de71f7cf -->
#### [503b9711] worktree.sh: bootstrap agent worktrees from local main instead of origin
- priority: `high`
- summary: `tools/worktree/worktree.sh` bootstrapped agent worktrees by treating the remote `origin` as the source of truth for `main`:
- ref: `.ticket/tickets/503b9711-3f69-4765-88f9-83779b71c8f8/ticket.toml`


### Component: tools/viewer/log-viewer/frontend/dioxus

<!-- ticket-index:entry id=6d0dc335-693a-450e-92ba-9cdaa4087afa slug=open/tools/viewer/log-viewer/frontend/dioxus digest=e38f921be8dd -->
#### [6d0dc335] [LOG-5e] Port log-viewer visualization tabs and overlay-backed tooling to viewer-api-dioxus
- priority: `p2`
- summary: The current log-viewer frontend uses the shared viewer-api frontend for more than the log list. `App.tsx` mounts a shared `WgpuOverlay` and exposes multiple non-trivial surfaces beyond the basic brow...
- ref: `.ticket/tickets/6d0dc335-693a-450e-92ba-9cdaa4087afa/ticket.toml`

<!-- ticket-index:entry id=b22b2a49-1e3a-40f4-b534-0f5e86610da7 slug=open/tools/viewer/log-viewer/frontend/dioxus digest=fa14bacc772d -->
#### [b22b2a49] [LOG-5f] Cut log-viewer over from Preact/Vite to Dioxus/trunk and lock in migration validation
- priority: `p2`
- summary: The current migration ticket set scaffolds and ports features, but it does not include the final cutover step: making the Dioxus frontend the default served build, preventing long-term drift between ...
- ref: `.ticket/tickets/b22b2a49-1e3a-40f4-b534-0f5e86610da7/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=3ca6f36f-fa19-46c6-9415-7d0290371f7f slug=open/unspecified digest=dda6b02c7f37 -->
#### [3ca6f36f] Add minimal terminal command agent
- ref: `.ticket/tickets/3ca6f36f-fa19-46c6-9415-7d0290371f7f/ticket.toml`

<!-- ticket-index:entry id=1157638e-edfe-4d29-a6ac-fc73010d5dd8 slug=open/unspecified digest=472947526cf1 -->
#### [1157638e] Auto-populate active-model marker for price-awareness enforcement (VS Code updater)
- summary: Automatically populate the active-model marker that drives price-awareness enforcement, so no manual entry or prompt is ever needed.
- ref: `.ticket/tickets/1157638e-edfe-4d29-a6ac-fc73010d5dd8/ticket.toml`

<!-- ticket-index:entry id=341f4bf2-6172-40c2-97d6-b1b184350fb1 slug=open/unspecified digest=6c6d671ad8fc -->
#### [341f4bf2] Close the tool-metrics loop: measure real tool output size and grade tool calls from historical data
- summary: For months, `.session/sessions/<id>/tool-metrics.json` was `{"tools":{}}` in **every** session while ten sibling tickets shipped `done`. Root cause (diagnosed 2026-07-29, fixed in memory-api commit `...
- ref: `.ticket/tickets/341f4bf2-6172-40c2-97d6-b1b184350fb1/ticket.toml`

<!-- ticket-index:entry id=1211b2d8-93f9-4b1a-8973-10ee9937ba3d slug=open/unspecified digest=fb735419cbb7 -->
#### [1211b2d8] Combat System: SDF Hit Detection, Damage Model & Voxel Destruction VFX
- priority: `high`
- summary: Combat in this RPG uses real-time SDF collision between weapon swings and player/NPC capsules. There are no hitboxes or animation frames — a weapon's SDF sweep volume is tested against target SDFs on...
- ref: `.ticket/tickets/1211b2d8-93f9-4b1a-8973-10ee9937ba3d/ticket.toml`

<!-- ticket-index:entry id=887f5059-650a-4fd0-a47b-e07b4a1959db slug=open/unspecified digest=fbf7f96b98d2 -->
#### [887f5059] Cross-workspace edge fixture fails with NotFound in serve::routes tests
- summary: `serve::routes::tests::ancestor_graph_ref_from_child_workspace_is_followable` ([memory-api/crates/ticket/src/serve/routes/tests.rs](memory-api/crates/ticket/src/serve/routes/tests.rs#L448))
- ref: `.ticket/tickets/887f5059-650a-4fd0-a47b-e07b4a1959db/ticket.toml`

<!-- ticket-index:entry id=14c0995c-1113-46dc-b924-82f0175628fc slug=open/unspecified digest=e0a718333aad -->
#### [14c0995c] Decouple agent-customization file creation from the rule system
- summary: Guidance across the repo still implied that creating or editing agent-customization files (`.agents/instructions/*.instructions.md`, `.agents/prompts/*.prompt.md`, `.agents/agents/*.agent.md`, `.agen...
- ref: `.ticket/tickets/14c0995c-1113-46dc-b924-82f0175628fc/ticket.toml`

<!-- ticket-index:entry id=03b7ce45-f2cb-4a5f-9b57-ef4c442b3515 slug=open/unspecified digest=6fecea060038 -->
#### [03b7ce45] Dioxus theme settings: Background Smoke section
- priority: `medium`
- summary: The canonical theme settings spec requires a "Background Smoke" section (§2 row 15) with
- ref: `.ticket/tickets/03b7ce45-f2cb-4a5f-9b57-ef4c442b3515/ticket.toml`

<!-- ticket-index:entry id=0ae1faf1-3554-45de-9d79-dc1be97de707 slug=open/unspecified digest=66bb0028292f -->
#### [0ae1faf1] Dioxus theme settings: Glass Panels + CRT Effect controls
- priority: `medium`
- summary: The canonical theme settings spec requires the "Glass Panels" section (§2 row 16,
- ref: `.ticket/tickets/0ae1faf1-3554-45de-9d79-dc1be97de707/ticket.toml`

<!-- ticket-index:entry id=6b51228e-1686-41fc-ac0c-7e19f58d657f slug=open/unspecified digest=863efc7c62cd -->
#### [6b51228e] Dioxus theme settings: Theme Presets grid + header actions
- priority: `medium`
- summary: The canonical theme settings spec (§2 row 1, default open) requires a Theme Presets grid
- ref: `.ticket/tickets/6b51228e-1686-41fc-ac0c-7e19f58d657f/ticket.toml`

<!-- ticket-index:entry id=47438a4f-64f8-4619-93ff-ea4355092606 slug=open/unspecified digest=e9a2091c24dc -->
#### [47438a4f] Dioxus theme settings: full ColorRows for all ThemeColors
- priority: `medium`
- summary: The canonical theme settings spec (§2 rows 2–8) requires color rows for every entry in
- ref: `.ticket/tickets/47438a4f-64f8-4619-93ff-ea4355092606/ticket.toml`

<!-- ticket-index:entry id=fe9b450e-94c2-4d01-9ac9-ff993f45a591 slug=open/unspecified digest=8fcf4767709c -->
#### [fe9b450e] Dioxus theme settings: per-effect controls (Sparks/Embers/Beams/Glitter/Cinder)
- priority: `medium`
- summary: The shared canonical theme settings spec (`viewer-api/theme-settings`) requires per-effect
- ref: `.ticket/tickets/fe9b450e-94c2-4d01-9ac9-ff993f45a591/ticket.toml`

<!-- ticket-index:entry id=c51cfb0f-cbbd-405c-a5c9-791fbd25f122 slug=open/unspecified digest=da216fc8b1b5 -->
#### [c51cfb0f] Enforce write-and-die Worker contract in tests
- summary: Add tests and validation to ensure Worker-tier (write-and-die) sub-agents terminate after one isolated step.
- ref: `.ticket/tickets/c51cfb0f-cbbd-405c-a5c9-791fbd25f122/ticket.toml`

<!-- ticket-index:entry id=b13c5d89-db09-46e7-9061-dbed85c4ab41 slug=open/unspecified digest=110f32a48986 -->
#### [b13c5d89] Epic: Agent Skill Foundation — contract, adopt proven skills, author Dioxus, migrate off rule generator
- summary: Foundation slice of the agent skill infrastructure, unblocking the critical path (ingestion debug tooling -> context/log UI -> UI dev/test/review loop).
- ref: `.ticket/tickets/b13c5d89-db09-46e7-9061-dbed85c4ab41/ticket.toml`

<!-- ticket-index:entry id=def88d4e-8a3c-45bc-82c8-bdacae01a479 slug=open/unspecified digest=e20e192d96f5 -->
#### [def88d4e] Epic: Unified Logging Infrastructure — file sinks, search, Mermaid, table, Dioxus frontend
- summary: Provide every viewer-api tool and context-* crate with consistent, queryable, structured logging.
- ref: `.ticket/tickets/def88d4e-8a3c-45bc-82c8-bdacae01a479/ticket.toml`

<!-- ticket-index:entry id=f90e66e5-6c14-4aff-a2e6-beaca9fb17b8 slug=open/unspecified digest=c1e03d14320f -->
#### [f90e66e5] Fix remaining silent history-append failures in rule-api and spec-api
- summary: Finish fixing the remaining 7 audited discarded-`Result` `append_history` sites that are outside ticket-api. These are the counterparts to the 7 sites fixed in ticket 0c02b304.
- ref: `.ticket/tickets/f90e66e5-6c14-4aff-a2e6-beaca9fb17b8/ticket.toml`

<!-- ticket-index:entry id=612f9dd7-e2d7-48fe-9825-d2283d4bb3fa slug=open/unspecified digest=a0475d686309 -->
#### [612f9dd7] Fix viewer-api sync-targets false drift on 798c9a3c body
- summary: Investigate false-drift failure in aggregated `rule sync-targets --config rule-targets.yaml --check` for viewer-api recurring-principles body target.
- ref: `.ticket/tickets/612f9dd7-e2d7-48fe-9825-d2283d4bb3fa/ticket.toml`

<!-- ticket-index:entry id=3fec54f1-9c8f-4059-a366-7da6e9a1a645 slug=open/unspecified digest=b5281007c2f5 -->
#### [3fec54f1] Force Compute Shader & SVO Collision
- priority: `high`
- summary: Particles must respond to complex physical forces (explosions, attraction, vortices) efficiently on the GPU. Furthermore, they need to physically collide with the Sparse Voxel Octree (SVO), bouncing ...
- ref: `.ticket/tickets/3fec54f1-9c8f-4059-a366-7da6e9a1a645/ticket.toml`

<!-- ticket-index:entry id=aaa1f539-4424-4d4a-a2f4-2871278fead7 slug=open/unspecified digest=07dbc8222e36 -->
#### [aaa1f539] Improve handoff package path validation
- summary: Implement stricter validation for handoff package repo paths.
- ref: `.ticket/tickets/aaa1f539-4424-4d4a-a2f4-2871278fead7/ticket.toml`

<!-- ticket-index:entry id=9e2c1f17-5710-4dff-bd45-ac5cb0b4a02c slug=open/unspecified digest=8b6783cfee8a -->
#### [9e2c1f17] Plan catalog-driven client implementation
- priority: `high`
- summary: Produce the Track 4 implementation design for 9e7a5f1a.
- ref: `.ticket/tickets/9e2c1f17-5710-4dff-bd45-ac5cb0b4a02c/ticket.toml`

<!-- ticket-index:entry id=d4233399-b986-401d-aed1-67c96dec84cf slug=open/unspecified digest=49c9e370dbd9 -->
#### [d4233399] Plan dual-format schema loader implementation
- priority: `high`
- summary: Produce the implementation design for 1f8e6e6d from its research brief.
- ref: `.ticket/tickets/d4233399-b986-401d-aed1-67c96dec84cf/ticket.toml`

<!-- ticket-index:entry id=9242e574-d3b5-4340-8daa-a00d8bfad155 slug=open/unspecified digest=286319d92930 -->
#### [9242e574] Plan legacy migration implementation
- priority: `high`
- summary: Use the completed classifier-research design to plan 7df984eb live migration work.
- ref: `.ticket/tickets/9242e574-d3b5-4340-8daa-a00d8bfad155/ticket.toml`

<!-- ticket-index:entry id=9e450826-60e1-437f-b236-2c8839e4ab9e slug=open/unspecified digest=6977462222af -->
#### [9e450826] Plan lifecycle engine implementation
- priority: `high`
- summary: Turn the Track 1 research brief into an implementation-ready design for 7ef3f8db.
- ref: `.ticket/tickets/9e450826-60e1-437f-b236-2c8839e4ab9e/ticket.toml`

<!-- ticket-index:entry id=9019477b-586d-4f42-ad57-04e0ff6b9b53 slug=open/unspecified digest=f8acded644c3 -->
#### [9019477b] Plan resolved catalog and JSON built-ins implementation
- priority: `high`
- summary: Produce the Track 3 implementation design for abd3f280.
- ref: `.ticket/tickets/9019477b-586d-4f42-ad57-04e0ff6b9b53/ticket.toml`

<!-- ticket-index:entry id=3eae33fb-7289-48fe-8151-7b2077fa810e slug=open/unspecified digest=c6b125e8a1dd -->
#### [3eae33fb] Plan schema modernization release validation
- priority: `high`
- summary: Produce the Track 6 implementation design for 3bb41fb2.
- ref: `.ticket/tickets/3eae33fb-7289-48fe-8151-7b2077fa810e/ticket.toml`

<!-- ticket-index:entry id=84aa1d3e-d98c-4c7c-8352-9ccecb2ca93e slug=open/unspecified digest=701b8c0689fb -->
#### [84aa1d3e] Pre-dispatch gate: Implement delegation checks
- summary: Add the mandated pre-dispatch gate checks for Implement delegations (Explore Agent gate).
- ref: `.ticket/tickets/84aa1d3e-d98c-4c7c-8352-9ccecb2ca93e/ticket.toml`

<!-- ticket-index:entry id=80afa16d-2ea9-4eff-96be-8c4f044ff159 slug=open/unspecified digest=7796c25da603 -->
#### [80afa16d] Probe Ticket
- ref: `.ticket/tickets/80afa16d-2ea9-4eff-96be-8c4f044ff159/ticket.toml`

<!-- ticket-index:entry id=1a1e3953-5275-407f-b690-428bdb90db7b slug=open/unspecified digest=61ccbe9106b8 -->
#### [1a1e3953] Project tracker: Dioxus theme settings backlog
- summary: Group the Dioxus theme settings backlog under a single tracker so those related tickets have a shared parent in the dependency graph.
- ref: `.ticket/tickets/1a1e3953-5275-407f-b690-428bdb90db7b/ticket.toml`

<!-- ticket-index:entry id=53f471a3-8a55-40ca-8f86-5da3b15aa25e slug=open/unspecified digest=41684317816d -->
#### [53f471a3] Project tracker: audit quality backlog
- summary: Project tracker for the audit quality backlog: hardening repository quality audits so ticket/spec/graph health produce more actionable signal.
- ref: `memory-api/.ticket/tickets/53f471a3-8a55-40ca-8f86-5da3b15aa25e/ticket.toml`

<!-- ticket-index:entry id=186caf8a-bbbf-426f-8fc3-2f7882a8a550 slug=open/unspecified digest=00f5fb95cf7d -->
#### [186caf8a] Project tracker: board workflow rollout
- ref: `memory-api/.ticket/tickets/186caf8a-bbbf-426f-8fc3-2f7882a8a550/ticket.toml`

<!-- ticket-index:entry id=72bad0e5-2f5d-4731-8cc6-8d4b167418dd slug=open/unspecified digest=09e3654ec061 -->
#### [72bad0e5] Project tracker: bootstrap executor backlog
- ref: `memory-api/.ticket/tickets/72bad0e5-2f5d-4731-8cc6-8d4b167418dd/ticket.toml`

<!-- ticket-index:entry id=02a3a2a7-1e70-4d25-a86c-17a36e5dd5e1 slug=open/unspecified digest=8ec9a61c9fab -->
#### [02a3a2a7] Project tracker: cargo doc workspace support
- ref: `memory-api/.ticket/tickets/02a3a2a7-1e70-4d25-a86c-17a36e5dd5e1/ticket.toml`

<!-- ticket-index:entry id=be47f545-c72a-43bd-a804-dd9665ce8faa slug=open/unspecified digest=dc74fdf3e515 -->
#### [be47f545] Project tracker: doc validation and install workflow redesign
- summary: Group the documented validation and install-flow redesign tickets under a single parent so the doc-validation backlog is connected into the wider workflow environment.
- ref: `.ticket/tickets/be47f545-c72a-43bd-a804-dd9665ce8faa/ticket.toml`

<!-- ticket-index:entry id=06b4f7d5-eee9-4266-9793-8c18a5bcf745 slug=open/unspecified digest=507197ee5f5c -->
#### [06b4f7d5] Project tracker: log tooling and viewer migration
- summary: Group the log schema, log API, and log viewer migration tickets under a single tracker so the subsystem has a coherent parent in the ticket graph.
- ref: `.ticket/tickets/06b4f7d5-eee9-4266-9793-8c18a5bcf745/ticket.toml`

<!-- ticket-index:entry id=a76c72e6-0bb9-48ef-be43-37c72ad89002 slug=open/unspecified digest=6e982e5cdaac -->
#### [a76c72e6] Project tracker: rule-api hierarchy and documentation pipeline
- ref: `memory-api/.ticket/tickets/a76c72e6-0bb9-48ef-be43-37c72ad89002/ticket.toml`

<!-- ticket-index:entry id=0af903c0-4f97-4773-b277-51dcf278b1f0 slug=open/unspecified digest=bb76ac2e5ab0 -->
#### [0af903c0] Project tracker: ticket API and mutation surfaces
- summary: Group ticket-api, ticket-http, and ticket-mcp mutation and storage-surface work under a single parent so related graph, mutation, and round-trip issues share a coherent backlog home.
- ref: `memory-api/.ticket/tickets/0af903c0-4f97-4773-b277-51dcf278b1f0/ticket.toml`

<!-- ticket-index:entry id=40ba5a15-df3c-42f2-8825-bd43bd66fce7 slug=open/unspecified digest=bb1bab592060 -->
#### [40ba5a15] Project tracker: ticket CLI and next-work backlog
- summary: Group the ticket CLI, board, MCP, and next-work backlog items under one parent so the discovery and workflow contract is represented in the dependency graph.
- ref: `memory-api/.ticket/tickets/40ba5a15-df3c-42f2-8825-bd43bd66fce7/ticket.toml`

<!-- ticket-index:entry id=171eb277-3270-4d52-8283-10cf3dd939b9 slug=open/unspecified digest=eada2dd9241c -->
#### [171eb277] Project tracker: ticket UX surfaces backlog
- ref: `memory-api/.ticket/tickets/171eb277-3270-4d52-8283-10cf3dd939b9/ticket.toml`

<!-- ticket-index:entry id=79efa73e-62d8-4c91-b0b5-b1ad79262efa slug=open/unspecified digest=8a1fee2f7d26 -->
#### [79efa73e] Project tracker: ticket metadata and content pipeline
- ref: `memory-api/.ticket/tickets/79efa73e-62d8-4c91-b0b5-b1ad79262efa/ticket.toml`

<!-- ticket-index:entry id=026401a0-e099-4d8b-840d-2d6b3bb456f3 slug=open/unspecified digest=c0fc054a8634 -->
#### [026401a0] Project tracker: ticket query filter correctness
- ref: `memory-api/.ticket/tickets/026401a0-e099-4d8b-840d-2d6b3bb456f3/ticket.toml`

<!-- ticket-index:entry id=9df4ef26-5168-4bbb-adf4-7f0e4f7ae3cf slug=open/unspecified digest=1028d14d5f9d -->
#### [9df4ef26] Project tracker: ticket/spec/rule operator workflow and discoverability
- summary: Group the unresolved operator-facing workflow and discoverability gaps surfaced while reviewing ticket, spec, and rule tool usage during the May 20-21 session.
- ref: `.ticket/tickets/9df4ef26-5168-4bbb-adf4-7f0e4f7ae3cf/ticket.toml`

<!-- ticket-index:entry id=f93f2266-7f97-4f31-a548-706c7a7e8c4a slug=open/unspecified digest=47a607ab3ac0 -->
#### [f93f2266] Project tracker: viewer logging rollout
- ref: `memory-api/.ticket/tickets/f93f2266-7f97-4f31-a548-706c7a7e8c4a/ticket.toml`

<!-- ticket-index:entry id=afcf2759-9c91-433c-b62c-ae8adcb0cdd5 slug=open/unspecified digest=76503bb1ba29 -->
#### [afcf2759] Project tracker: workflow traceability redesign
- summary: Group the workflow metadata and cross-store traceability redesign tickets under a single parent so the backlog is connected into the broader dependency graph.
- ref: `.ticket/tickets/afcf2759-9c91-433c-b62c-ae8adcb0cdd5/ticket.toml`

<!-- ticket-index:entry id=e4de2cdc-48d0-42b4-92df-900da88e156f slug=open/unspecified digest=39d6502396bf -->
#### [e4de2cdc] Rename ticket dependency labels
- summary: Rename ticket schema/display wording for dependency relationship fields: `dependees` label should become `dependee_count`, and `depends_on` should render as `Dependencies` in ticket schema/display fo...
- ref: `.ticket/tickets/e4de2cdc-48d0-42b4-92df-900da88e156f/ticket.toml`

<!-- ticket-index:entry id=5e1e799e-860c-43f3-ac1a-598d29780b23 slug=open/unspecified digest=b89ef16aeaf0 -->
#### [5e1e799e] Research catalog-driven client integration
- priority: `high`
- summary: Research CLI, MCP, HTTP, VS Code, and affected viewer/search consumers that currently assume legacy type lists or transitions.
- ref: `.ticket/tickets/5e1e799e-860c-43f3-ac1a-598d29780b23/ticket.toml`

<!-- ticket-index:entry id=d8bd4c53-898e-4984-97e5-6ef605569f91 slug=open/unspecified digest=e086af464dbb -->
#### [d8bd4c53] Research deterministic legacy-ticket classifier
- priority: `high`
- summary: Act as the required Track 5 research sub-ticket and define the deterministic rules-first classifier before live migration of legacy `task` and `tracker-improvement` records. This ticket uses the lega...
- ref: `.ticket/tickets/d8bd4c53-898e-4984-97e5-6ef605569f91/ticket.toml`

<!-- ticket-index:entry id=047dffaf-21e6-4bc2-a634-d858d10d214b slug=open/unspecified digest=a2264647181f -->
#### [047dffaf] Research dual-format schema loader design
- priority: `high`
- summary: Research loader paths, parser boundaries, diagnostics, and fixture conventions for Track 2 after the lifecycle engine contract is available.
- ref: `.ticket/tickets/047dffaf-21e6-4bc2-a634-d858d10d214b/ticket.toml`

<!-- ticket-index:entry id=0887b8d6-8c2a-46ee-9550-90b55720a757 slug=open/unspecified digest=eb3536ef7849 -->
#### [0887b8d6] Research resolved catalog and JSON built-ins design
- priority: `high`
- summary: Research manifest generation, built-in schema sources, reproducibility, and catalog-consumer boundaries needed for Track 3.
- ref: `.ticket/tickets/0887b8d6-8c2a-46ee-9550-90b55720a757/ticket.toml`

<!-- ticket-index:entry id=2def6b85-63f6-4e00-a9c0-ae05e3369f48 slug=open/unspecified digest=1a79c5547ed0 -->
#### [2def6b85] Research schema modernization release validation
- priority: `high`
- summary: Research the release-gate evidence surfaces for Track 6 after migration design is available.
- ref: `.ticket/tickets/2def6b85-63f6-4e00-a9c0-ae05e3369f48/ticket.toml`

<!-- ticket-index:entry id=ff2872ad-74be-4e5d-a7ba-416c73506252 slug=open/unspecified digest=ef95928bbb47 -->
#### [ff2872ad] Restore ticket-vscode listing when server is already running
- summary: Improved ticket-vscode error-state surfacing so provider failures now include caller context and the associated request details. The API client throws structured request errors with operation, method...
- ref: `.ticket/tickets/ff2872ad-74be-4e5d-a7ba-416c73506252/ticket.toml`

<!-- ticket-index:entry id=e03eb731-877a-4498-832f-d1e41526423a slug=open/unspecified digest=e7a97c59de9a -->
#### [e03eb731] Review schema modernization completion evidence
- priority: `high`
- summary: Review final Track 6 evidence against the full decision register before the epic closes.
- ref: `.ticket/tickets/e03eb731-877a-4498-832f-d1e41526423a/ticket.toml`

<!-- ticket-index:entry id=042109c0-8e0e-4585-9b58-37fdd345ce12 slug=open/unspecified digest=92f66778fbc2 -->
#### [042109c0] Rust MCP middleware: require active model per request (supersedes file sync)
- summary: Replace the fragile file-based model-identity mechanism with a **Rust MCP middleware** that requires the active model to be supplied **with every `tools/call`**. This removes shared mutable state (th...
- ref: `.ticket/tickets/042109c0-8e0e-4585-9b58-37fdd345ce12/ticket.toml`

<!-- ticket-index:entry id=4bc90289-2e27-458c-9468-f7daef9840f2 slug=open/unspecified digest=ed311968cd47 -->
#### [4bc90289] Shared context bundle: compute shared-prefix optimization
- summary: Optimize orchestrator to compute and inline the shared context bundle prefix for parallel fan-out.
- ref: `.ticket/tickets/4bc90289-2e27-458c-9468-f7daef9840f2/ticket.toml`

<!-- ticket-index:entry id=89c3189b-381d-4020-8757-39a675791c20 slug=open/unspecified digest=cc322a590820 -->
#### [89c3189b] Skill System: Spell SDFs, Procedural Shader Effects & Volumetric Magic
- priority: `high`
- summary: Magic spells in this RPG are not pre-canned animations — they are transient SDF volumes injected into the ray-marching loop, generating real-time volumetric lighting, refraction, and physical effects...
- ref: `.ticket/tickets/89c3189b-381d-4020-8757-39a675791c20/ticket.toml`

<!-- ticket-index:entry id=3c28bc73-cbca-4022-a93b-0e8a65e13ff7 slug=open/unspecified digest=a7e19d589d14 -->
#### [3c28bc73] Tracker: workflow diagnostics routing and structural graph validation
- summary: Tracker ticket to keep d1b3a6c9 (Route workflow diagnostics upward and add structural workflow-graph validation) connected to the depends_on graph, per ticket-store health check graph_participation f...
- ref: `memory-api/.ticket/tickets/3c28bc73-cbca-4022-a93b-0e8a65e13ff7/ticket.toml`

<!-- ticket-index:entry id=5b4330f6-f1d0-4e80-8a3e-296f557c5a99 slug=open/unspecified digest=ea02faee280d -->
#### [5b4330f6] [LOG-2b] Add context-trace JSON format compatibility test against log-viewer parser
- summary: `crates/context-stack/context-trace` uses a `PrettyJsonWriter` to produce structured log output, and `crates/context-stack/context-api/src/log_parser.rs` parses it. There are no automated tests ensur...
- ref: `.ticket/tickets/5b4330f6-f1d0-4e80-8a3e-296f557c5a99/ticket.toml`

<!-- ticket-index:entry id=159a9862-6ea3-4966-a2b1-992e1d03b578 slug=open/unspecified digest=599e0c15a920 -->
#### [159a9862] [LOG-3a] Log schema-field search: add search_fields MCP tool and HTTP endpoint
- summary: `mcp_log-viewer-mc_query_logs` accepts a raw JQ expression, which is powerful but requires users to know the exact field path. There is no dedicated "find all log entries where field X = Y" API that ...
- ref: `.ticket/tickets/159a9862-6ea3-4966-a2b1-992e1d03b578/ticket.toml`

<!-- ticket-index:entry id=b3fc711c-8c8d-4e3f-a76b-f00c551d9d49 slug=open/unspecified digest=0fd0870f1469 -->
#### [b3fc711c] [LOG-3b] Log full-text search: add search_text MCP tool with regex and context-lines support
- summary: `mcp_log-viewer-mc_search_all_logs` accepts a JQ expression, not a plain text or regex query. There is no simple "grep for a string across log files" interface for users who need a quick `contains("p...
- ref: `.ticket/tickets/b3fc711c-8c8d-4e3f-a76b-f00c551d9d49/ticket.toml`

<!-- ticket-index:entry id=40a4bc9e-7ecd-4fa4-b842-633891bd5cba slug=open/unspecified digest=7d3d4b763566 -->
#### [40a4bc9e] [LOG-4a] Log-to-Mermaid: convert filtered log session to sequenceDiagram
- summary: When debugging multi-component interactions (e.g. `ticket-http` handler → `TicketStore` → `tracing` spans), engineers have no automatic way to visualise the call flow as a Mermaid `sequenceDiagram`. ...
- ref: `.ticket/tickets/40a4bc9e-7ecd-4fa4-b842-633891bd5cba/ticket.toml`

<!-- ticket-index:entry id=f37bdd68-b2d7-4c4b-93b9-fa9d4f61c4a6 slug=open/unspecified digest=5f4ab13d8e7c -->
#### [f37bdd68] [LOG-4b] Log-to-table: render filtered log view as ASCII/Markdown table (MCP + HTTP)
- summary: There is no way to get a quick tabular summary of log entries for terminal/CLI use or for pasting into documentation. Engineers must write JQ expressions and post-process JSON manually.
- ref: `.ticket/tickets/f37bdd68-b2d7-4c4b-93b9-fa9d4f61c4a6/ticket.toml`

<!-- ticket-index:entry id=972c239e-e110-49da-9449-8bdcfaea5f18 slug=open/unspecified digest=db184af30ed4 -->
#### [972c239e] [LOG-5b] Port log-viewer browser UI to Dioxus: file tree, entry list, search bar, stats
- summary: After the scaffold ([LOG-5a]), the Dioxus log-viewer is a stub. This ticket ports the core browsing UI from the Preact frontend to Dioxus components with feature parity.
- ref: `.ticket/tickets/972c239e-e110-49da-9449-8bdcfaea5f18/ticket.toml`

<!-- ticket-index:entry id=bfb95499-ac12-4cd4-808f-879795a938e5 slug=open/unspecified digest=5d6dc58ca0a3 -->
#### [bfb95499] [LOG-5c] Add live-tail view to log-viewer-dioxus: SSE endpoint and real-time browser component
- summary: Engineers launching a server with `viewer-ctl start --fg` need to see live log output in the browser rather than switching between the terminal and the viewer. There is no streaming/tail view in eith...
- ref: `.ticket/tickets/bfb95499-ac12-4cd4-808f-879795a938e5/ticket.toml`

<!-- ticket-index:entry id=7f39fae3-e23a-4c8c-bd8c-cd3674334dbe slug=open/unspecified digest=f7ee4299437c -->
#### [7f39fae3] [PDF][Follow-up] pdf-http transport (deferred out of v1)
- summary: Locked decision 10 of the PDF epic put the HTTP transport out of scope for v1.
- ref: `.ticket/tickets/7f39fae3-e23a-4c8c-bd8c-cd3674334dbe/ticket.toml`

<!-- ticket-index:entry id=c72f1fab-7d98-4f1d-983f-682eade2a23a slug=open/unspecified digest=5a67ce68413d -->
#### [c72f1fab] [PDF][T1] Scaffold pdf-api + pdf facade crate with feature-gated cli/mcp bins
- summary: Create the two-crate skeleton mandated by
- ref: `.ticket/tickets/c72f1fab-7d98-4f1d-983f-682eade2a23a/ticket.toml`

<!-- ticket-index:entry id=e9c0e280-859e-4d85-9bcf-3fcc9754d3fb slug=open/unspecified digest=ae18af696d47 -->
#### [e9c0e280] [PDF][T2] pdf-api core: request/response types, PdfError, execute() dispatch, sandbox + write-safety layer
- summary: Define the `pdf-api` contract and implement the **shared safety layer** that
- ref: `.ticket/tickets/e9c0e280-859e-4d85-9bcf-3fcc9754d3fb/ticket.toml`

<!-- ticket-index:entry id=a4d7df73-003b-46da-9e3a-4dac7fad82d8 slug=open/unspecified digest=62ed47f2e91c -->
#### [a4d7df73] [PDF][T3] Text extraction with page ranges, output bounding, and no-text-layer detection
- summary: Implement the `ExtractText` variant: given a PDF, return its text content.
- ref: `.ticket/tickets/a4d7df73-003b-46da-9e3a-4dac7fad82d8/ticket.toml`

<!-- ticket-index:entry id=e135e28c-7878-472b-8dce-0f756482acb8 slug=open/unspecified digest=783df896f60c -->
#### [e135e28c] [PDF][T4] Document operations: merge, split, page delete/reorder/rotate, metadata read/write
- summary: Implement the four document-manipulation variants. These are the write-path
- ref: `.ticket/tickets/e135e28c-7878-472b-8dce-0f756482acb8/ticket.toml`

<!-- ticket-index:entry id=42780b6e-5dd1-491e-99ca-7e736357aaaf slug=open/unspecified digest=28c3483e3330 -->
#### [42780b6e] [PDF][T5] PDF creation: programmatic primitive plus optional sandboxed typst-cli path
- summary: Implement the `Create` variant with two modes, per locked decision 4.
- ref: `.ticket/tickets/42780b6e-5dd1-491e-99ca-7e736357aaaf/ticket.toml`

<!-- ticket-index:entry id=856c69c2-fd6a-4f8c-b93d-12aec415b6f8 slug=open/unspecified digest=63977f27222b -->
#### [856c69c2] [PDF][T6] pdf-cli transport: subcommands for every operation, --root boundary, json/toon output
- summary: Wire the `pdf-cli` binary so every `pdf-api` operation is reachable from the
- ref: `.ticket/tickets/856c69c2-fd6a-4f8c-b93d-12aec415b6f8/ticket.toml`

<!-- ticket-index:entry id=0a602458-8965-41ed-8884-1f72dfdfa203 slug=open/unspecified digest=a2b2dfa1cbe2 -->
#### [0a602458] [PDF][T7] pdf-mcp transport: named tools, server-owned sandbox root, error mapping
- summary: Expose `pdf-api` as MCP named tools. This is the primary agent-facing surface
- ref: `.ticket/tickets/0a602458-8965-41ed-8884-1f72dfdfa203/ticket.toml`

<!-- ticket-index:entry id=9590bdcf-8c3c-4cd1-bd60-df2ec6ca65f1 slug=open/unspecified digest=f338934a8edb -->
#### [9590bdcf] [PDF][T8] Author .agents/skills/pdf/SKILL.md, README index row, and crate docs
- summary: Author the agent-facing skill so agents actually discover and correctly use the
- ref: `.ticket/tickets/9590bdcf-8c3c-4cd1-bd60-df2ec6ca65f1/ticket.toml`

<!-- ticket-index:entry id=a59f35fb-9e37-4d27-8c0d-35b9229f5813 slug=open/unspecified digest=67672c8722ca -->
#### [a59f35fb] [PDF][T9] Embedded image extraction with filter/colorspace handling (cuttable, blocks nothing)
- summary: Implement the `ExtractImages` variant: extract embedded raster images from a PDF
- ref: `.ticket/tickets/a59f35fb-9e37-4d27-8c0d-35b9229f5813/ticket.toml`

<!-- ticket-index:entry id=01b6fe40-5741-41b5-a73c-e4bd51b49a3f slug=open/unspecified digest=6c537ceb58b2 -->
#### [01b6fe40] [architecture][contracts] Binary composition root wiring
- priority: `medium`
- summary: Wire contracts in binary composition roots using static typing and remove ad-hoc direct coupling.
- ref: `.ticket/tickets/01b6fe40-5741-41b5-a73c-e4bd51b49a3f/ticket.toml`

<!-- ticket-index:entry id=65ea4528-af24-4300-8e5f-3b68e54711d0 slug=open/unspecified digest=6ba76b707618 -->
#### [65ea4528] [architecture][contracts] Core shared contract crate
- priority: `high`
- summary: Define the core shared contract crate for cross-store interaction primitives.
- ref: `.ticket/tickets/65ea4528-af24-4300-8e5f-3b68e54711d0/ticket.toml`

<!-- ticket-index:entry id=d86c66a9-ebe1-4b13-a3a4-87f4246b3062 slug=open/unspecified digest=4be40eb2fb24 -->
#### [d86c66a9] [architecture][contracts] Domain extension contracts and first provider
- priority: `high`
- summary: Define domain extension contract crates and implement first provider/consumer pair.
- ref: `.ticket/tickets/d86c66a9-ebe1-4b13-a3a4-87f4246b3062/ticket.toml`

<!-- ticket-index:entry id=0f2be510-378a-40eb-a98c-ab516b0ec647 slug=open/unspecified digest=2e38f8949f5d -->
#### [0f2be510] [architecture][contracts] IoC contract crates for cross-store interactions
- priority: `high`
- summary: Define and adopt a hybrid cross-store contract layer so domain crates interact via inversion of control rather than direct domain coupling.
- ref: `.ticket/tickets/0f2be510-378a-40eb-a98c-ab516b0ec647/ticket.toml`

<!-- ticket-index:entry id=37e07148-c327-4530-8251-599c14dca04e slug=open/unspecified digest=cc5fb1f8d25e -->
#### [37e07148] [architecture][memory-api] Implement neutral shared storage kernel APIs
- priority: `high`
- summary: Implement neutral shared storage/index/search symbols in memory-api with compatibility aliases.
- ref: `.ticket/tickets/37e07148-c327-4530-8251-599c14dca04e/ticket.toml`

<!-- ticket-index:entry id=13912e44-fee8-4aa5-b28f-68bbc22af401 slug=open/unspecified digest=bb1b27cd7bd0 -->
#### [13912e44] [architecture][memory-api] Neutral naming migration map
- priority: `high`
- summary: Create a concrete neutral naming migration map for shared memory-api storage/index/search APIs.
- ref: `.ticket/tickets/13912e44-fee8-4aa5-b28f-68bbc22af401/ticket.toml`

<!-- ticket-index:entry id=2b1279bd-c42f-4b0e-8835-d0d645a733ab slug=open/unspecified digest=fb3a1cf638cc -->
#### [2b1279bd] [architecture][memory-api] Neutral storage kernel and API migration
- priority: `high`
- summary: Refactor memory-api shared storage/index/search APIs to domain-neutral semantics (`entity`, `store`, `workspace`) and isolate ticket-only behavior from shared storage internals.
- ref: `.ticket/tickets/2b1279bd-c42f-4b0e-8835-d0d645a733ab/ticket.toml`

<!-- ticket-index:entry id=671d4e47-b53d-4a04-aa1d-30f2aa8a2bbe slug=open/unspecified digest=da57b20ea64f -->
#### [671d4e47] [architecture][multi-store] Tracker: cross-store interaction model and migration
- priority: `high`
- summary: Goal: deliver a workspace-wide, domain-isolated multi-store architecture where each store owns persistence and workflow behavior while cross-store interactions are defined by shared contract interfac...
- ref: `.ticket/tickets/671d4e47-b53d-4a04-aa1d-30f2aa8a2bbe/ticket.toml`

<!-- ticket-index:entry id=834632eb-7c0f-4e43-b1ca-3793141e25d8 slug=open/unspecified digest=7d9feab796e4 -->
#### [834632eb] [architecture][observability] CLI and MCP extended error envelope adoption
- priority: `high`
- summary: Adopt extended error envelope in CLI and MCP surfaces.
- ref: `.ticket/tickets/834632eb-7c0f-4e43-b1ca-3793141e25d8/ticket.toml`

<!-- ticket-index:entry id=d8b5cfd0-8516-4dbe-84da-be112f6e5a57 slug=open/unspecified digest=8ec1b0f8a79e -->
#### [d8b5cfd0] [architecture][observability] Extended error envelope schema and mapping rules
- priority: `high`
- summary: Define extended cross-channel error envelope schema and mapping rules.
- ref: `.ticket/tickets/d8b5cfd0-8516-4dbe-84da-be112f6e5a57/ticket.toml`

<!-- ticket-index:entry id=726efe80-3dc4-4b2d-9817-fb2b91b74441 slug=open/unspecified digest=5bcf0c29e3d9 -->
#### [726efe80] [architecture][observability] HTTP extended error envelope adoption
- priority: `high`
- summary: Adopt extended error envelope in HTTP surfaces and trace correlation.
- ref: `.ticket/tickets/726efe80-3dc4-4b2d-9817-fb2b91b74441/ticket.toml`

<!-- ticket-index:entry id=d03530c6-52e4-42d3-8d57-e750ce73c8d4 slug=open/unspecified digest=c3bb48b72d79 -->
#### [d03530c6] [architecture][observability] Unified traceable error channels across stores
- priority: `high`
- summary: Standardize error tracing and user-facing diagnostics across store CLIs, MCP servers, and HTTP handlers using one extended envelope contract.
- ref: `.ticket/tickets/d03530c6-52e4-42d3-8d57-e750ce73c8d4/ticket.toml`

<!-- ticket-index:entry id=e2768479-24d6-4f42-bdbe-ac509167dc62 slug=open/unspecified digest=c614a004c4f8 -->
#### [e2768479] [architecture][rule-spec] Adopt neutral shared APIs in rule-api and spec-api
- priority: `high`
- summary: Migrate rule-api and spec-api to consume neutral memory-api shared APIs.
- ref: `.ticket/tickets/e2768479-24d6-4f42-bdbe-ac509167dc62/ticket.toml`

<!-- ticket-index:entry id=999d9316-fc79-4bb1-b629-7cba52eced31 slug=open/unspecified digest=83183b4bd538 -->
#### [999d9316] [architecture][ticket-api] Adopt neutral shared APIs and alias retirement gate
- priority: `high`
- summary: Migrate ticket-api internal usage to neutral shared APIs and define alias retirement gate.
- ref: `.ticket/tickets/999d9316-fc79-4bb1-b629-7cba52eced31/ticket.toml`

<!-- ticket-index:entry id=11fb9bcf-fcd5-4eff-b380-64b80f4a5c9c slug=open/unspecified digest=e7e977156733 -->
#### [11fb9bcf] [audit-api] Cleanup loop UX and automated remediation suggestions
- priority: `high`
- summary: Design user-facing triage loops, inboxes, and remediation hints for stale/outdated/conflicting rule/spec/ticket entries, including recurring audit cadence.
- ref: `.ticket/tickets/11fb9bcf-fcd5-4eff-b380-64b80f4a5c9c/ticket.toml`

<!-- ticket-index:entry id=bd1c7cc0-2850-418d-b701-981b95c587ee slug=open/unspecified digest=61b8afc63929 -->
#### [bd1c7cc0] [audit-api] Continuous store health scoring and cleanup loops
- priority: `high`
- summary: Plan continuous auditing to detect stale, conflicting, or low-value entries across spec/rule/ticket stores using activity, validation, and feedback signals.
- ref: `.ticket/tickets/bd1c7cc0-2850-418d-b701-981b95c587ee/ticket.toml`

<!-- ticket-index:entry id=8f021514-6d53-45f3-a0cf-667fb3865a4d slug=open/unspecified digest=4907b91eb759 -->
#### [8f021514] [audit-api] Explainable remediation queues and reversible cleanup actions
- priority: `high`
- summary: Define queue explainability and reversible cleanup action semantics for continuous store-health workflows.
- ref: `.ticket/tickets/8f021514-6d53-45f3-a0cf-667fb3865a4d/ticket.toml`

<!-- ticket-index:entry id=67b6117b-5978-4c89-9cd4-4c8b043f4fba slug=open/unspecified digest=d04840a0a353 -->
#### [67b6117b] [audit-api] Health metric taxonomy and scoring model for store entries
- priority: `high`
- summary: Define weighted health metrics (relevance, freshness, conflict, validation coverage, feedback sentiment, activity) and scoring thresholds for healthy/unhealthy entries.
- ref: `.ticket/tickets/67b6117b-5978-4c89-9cd4-4c8b043f4fba/ticket.toml`

<!-- ticket-index:entry id=89f21dd2-6307-4f30-b0f8-7b36b3cfce66 slug=open/unspecified digest=df2f929eb67e -->
#### [89f21dd2] [audit-api] Health score recompute and queue refresh SLOs
- priority: `high`
- summary: Define performance budgets for health score recomputation and remediation queue refresh at workspace scale.
- ref: `.ticket/tickets/89f21dd2-6307-4f30-b0f8-7b36b3cfce66/ticket.toml`

<!-- ticket-index:entry id=f6ee97de-c7c9-46d3-878b-c6df5f4a4bc9 slug=open/unspecified digest=1eab48e92241 -->
#### [f6ee97de] [audit-api] Score-model versioning and anti-gaming safeguards
- priority: `high`
- summary: Define governance for score coefficient changes and safeguards against queue manipulation or noisy-signal dominance.
- ref: `.ticket/tickets/f6ee97de-c7c9-46d3-878b-c6df5f4a4bc9/ticket.toml`

<!-- ticket-index:entry id=8dbff37f-699b-4c91-bf65-6516ea6fe609 slug=open/unspecified digest=595c23d45c5a -->
#### [8dbff37f] [audit-api] Workspace graph health and board check-in validation enforcement
- priority: `high`
- summary: Plan the audit and operator-enforcement layer for validation-aware ticket graphs.
- ref: `.ticket/tickets/8dbff37f-699b-4c91-bf65-6516ea6fe609/ticket.toml`

<!-- ticket-index:entry id=edde88d6-ac19-4b89-a136-73602596f546 slug=open/unspecified digest=3389195b318c -->
#### [edde88d6] [audit-roadmap][2026-07-05] Full repository audit remediation tracker
- summary: Drive the full 2026-07-05 audit baseline from 551 findings toward near-zero through ordered category execution and bounded implementation batches.
- ref: `.ticket/tickets/edde88d6-ac19-4b89-a136-73602596f546/ticket.toml`

<!-- ticket-index:entry id=f23c9450-1999-4fa2-9751-c67faa63fe00 slug=open/unspecified digest=ec4f9c782fb3 -->
#### [f23c9450] [audit-roadmap][file_length] Split oversized files
- summary: Reduce file_length findings from 182 to zero using safe module splits and behavior-preserving moves.
- ref: `.ticket/tickets/f23c9450-1999-4fa2-9751-c67faa63fe00/ticket.toml`

<!-- ticket-index:entry id=936cdd9f-5736-468e-a41d-fa3da20b7e3d slug=open/unspecified digest=266671808b42 -->
#### [936cdd9f] [audit-roadmap][file_length][batch-2] context-stack (39)
- summary: Resolve the current file_length batch for context-stack and reduce 39 findings from the baseline.
- ref: `.ticket/tickets/936cdd9f-5736-468e-a41d-fa3da20b7e3d/ticket.toml`

<!-- ticket-index:entry id=1abda9df-bb74-4c84-8b92-2a38d8731949 slug=open/unspecified digest=5336096bbef1 -->
#### [1abda9df] [audit-roadmap][file_length][batch-3] tools (27)
- summary: Resolve the current file_length batch for tools and reduce 27 findings from the baseline.
- ref: `.ticket/tickets/1abda9df-bb74-4c84-8b92-2a38d8731949/ticket.toml`

<!-- ticket-index:entry id=5f7b2051-e577-4655-a9f8-02d0ea8b588c slug=open/unspecified digest=f05a34c0af97 -->
#### [5f7b2051] [audit-roadmap][file_length][batch-4] memory-viewers (19)
- summary: Resolve the current file_length batch for memory-viewers and reduce 19 findings from the baseline.
- ref: `.ticket/tickets/5f7b2051-e577-4655-a9f8-02d0ea8b588c/ticket.toml`

<!-- ticket-index:entry id=7fbe52b5-ab69-4b3c-adaa-21d42cf26de8 slug=open/unspecified digest=79d03f66bdf7 -->
#### [7fbe52b5] [audit-roadmap][file_length][batch-5] viewer-api (7)
- summary: Resolve the current file_length batch for viewer-api and reduce 7 findings from the baseline.
- ref: `.ticket/tickets/7fbe52b5-ab69-4b3c-adaa-21d42cf26de8/ticket.toml`

<!-- ticket-index:entry id=5f9542bf-483a-4da6-9c78-fcbe588af973 slug=open/unspecified digest=920cafbf4269 -->
#### [5f9542bf] [audit-roadmap][static_complexity] Reduce complexity findings
- summary: Reduce static_complexity findings from 108 to zero through staged complexity reduction, not broad rewrites.
- ref: `.ticket/tickets/5f9542bf-483a-4da6-9c78-fcbe588af973/ticket.toml`

<!-- ticket-index:entry id=99bdcf44-58de-480d-ab9d-a1951c053400 slug=open/unspecified digest=17588bd93e35 -->
#### [99bdcf44] [audit-roadmap][static_complexity][batch-4] memory-viewers (7)
- summary: Resolve the current static_complexity batch for memory-viewers and reduce 7 findings from the baseline.
- ref: `.ticket/tickets/99bdcf44-58de-480d-ab9d-a1951c053400/ticket.toml`

<!-- ticket-index:entry id=819f2e97-4cd4-410b-af3b-f196ba80d720 slug=open/unspecified digest=111117da4abc -->
#### [819f2e97] [audit-roadmap][static_complexity][batch-5] viewer-api (6)
- summary: Resolve the current static_complexity batch for viewer-api and reduce 6 findings from the baseline.
- ref: `.ticket/tickets/819f2e97-4cd4-410b-af3b-f196ba80d720/ticket.toml`

<!-- ticket-index:entry id=cb92b0d2-1361-44c6-8c92-fae3036ac97a slug=open/unspecified digest=76459f089e8f -->
#### [cb92b0d2] [audit-roadmap][ticket_graph][retro] Context-engine core work stream
- summary: Groups historical context-engine work (context-read/search/trace/insert, expansion loop, partition merge, ngrams, CLI read UX, Graph3D visualization) that was tracked as standalone plans/designs/bugs.
- ref: `.ticket/tickets/cb92b0d2-1361-44c6-8c92-fae3036ac97a/ticket.toml`

<!-- ticket-index:entry id=02622207-e102-4fae-b705-ca1cb12704ba slug=open/unspecified digest=0b053795bc96 -->
#### [02622207] [audit-roadmap][ticket_graph][retro] Infra, CI, install and viewer-tooling work stream
- summary: Groups historical infra/CI/install and viewer-tooling work (repository QA/audit tool CLI + config, install-tools, CI viewer workflow split, viewer-api visual validation, README sync, log-viewer graph...
- ref: `.ticket/tickets/02622207-e102-4fae-b705-ca1cb12704ba/ticket.toml`

<!-- ticket-index:entry id=ad63d3da-1661-41e0-a0d3-3163bba324f9 slug=open/unspecified digest=e4285bfd3c7d -->
#### [ad63d3da] [audit-roadmap][ticket_graph][retro] Miscellaneous cross-cutting work stream
- summary: Groups remaining historical orphan tickets that do not fit the ticket-system, rules/specs, context-engine-core, or infra/viewer clusters — cross-cutting work, integration-test harness plans, hooks, t...
- ref: `.ticket/tickets/ad63d3da-1661-41e0-a0d3-3163bba324f9/ticket.toml`

<!-- ticket-index:entry id=45ff05c9-7608-43c4-a98a-e1c44e4b7fbd slug=open/unspecified digest=807d97f0f7ce -->
#### [45ff05c9] [audit-roadmap][ticket_graph][retro] Rules, specs & agent-guidance work stream
- summary: Groups historical work on the rule system, spec system, repo-guidance generation, agent targets, and session-api (rule-cli/rule-api, spec-cli, repo-guidance, agent-rules, session-api, instruction DSL...
- ref: `.ticket/tickets/45ff05c9-7608-43c4-a98a-e1c44e4b7fbd/ticket.toml`

<!-- ticket-index:entry id=3be95a71-f6d7-4c83-b877-1d236751a12c slug=open/unspecified digest=1364f67df26b -->
#### [3be95a71] [content-materialization][epic] Fill specs, Rust/code policy, and the feedback ring on top of session construction
- summary: Turn the wired-but-empty entity graph into durable, queryable content. The session-construction machinery (epic effba966) assembles per-session context from rules/specs/tickets; this epic fills those...
- ref: `memory-api/.ticket/tickets/3be95a71-f6d7-4c83-b877-1d236751a12c/ticket.toml`

<!-- ticket-index:entry id=700127a8-f9a5-415d-a433-2d5b888e6292 slug=open/unspecified digest=f02b6cca0234 -->
#### [700127a8] [context-editor] LLM Integration: Text-to-Voxel/Shader, Naga Validation & Hot-Reload
- priority: `high`
- summary: Players can type natural-language descriptions into the UI to procedurally generate voxel structures, custom shader effects, or skill modifiers. An LLM translates the prompt into either: (a) voxel co...
- ref: `.ticket/tickets/700127a8-f9a5-415d-a433-2d5b888e6292/ticket.toml`

<!-- ticket-index:entry id=5a87d7b2-6d58-41ee-bd74-dd0fc6fde5f1 slug=open/unspecified digest=681d641c1eb1 -->
#### [5a87d7b2] [context-editor][SDF-DAG] GPU SDF Collision & Force Kernel — DAG Traversal Physics
- priority: `high`
- summary: The current physics pipeline has two major gaps:
- ref: `.ticket/tickets/5a87d7b2-6d58-41ee-bd74-dd0fc6fde5f1/ticket.toml`

<!-- ticket-index:entry id=22dc5dfc-ac5d-46e0-979e-1f38ac4ce6c7 slug=open/unspecified digest=dba214371658 -->
#### [22dc5dfc] [context-editor][SDF-DAG] Heterogeneous SDF Atom DAG Architecture — Epic
- priority: `high`
- summary: Replace the current flat-material Sparse Voxel Octree (SVO) with a Directed Acyclic Graph (DAG)
- ref: `.ticket/tickets/22dc5dfc-ac5d-46e0-979e-1f38ac4ce6c7/ticket.toml`

<!-- ticket-index:entry id=52ed521c-2774-40f9-95e1-7deca81d2f09 slug=open/unspecified digest=3768e56933d7 -->
#### [52ed521c] [context-editor][SDF-DAG] Phase 1: Per-Voxel SDF Atom Type System with Typed Dispatch
- priority: `high`
- summary: The current `OctreeNode` stores only a flat `color_data: u32` (R8G8B8 + roughness5 + metallic1)
- ref: `.ticket/tickets/52ed521c-2774-40f9-95e1-7deca81d2f09/ticket.toml`

<!-- ticket-index:entry id=8f0ffc7c-b1d6-423d-bb9b-6f0c6b75852b slug=open/unspecified digest=33e5d64e54a6 -->
#### [8f0ffc7c] [context-editor][SDF-DAG] Phase 2: DAG-Persistent Edit Operations with Hash Consing
- priority: `medium`
- summary: The current SVO uses a mutable flat `Vec<OctreeNode>` where every edit directly mutates nodes
- ref: `.ticket/tickets/8f0ffc7c-b1d6-423d-bb9b-6f0c6b75852b/ticket.toml`

<!-- ticket-index:entry id=6f368e20-00a4-4da5-9388-492ba4209915 slug=open/unspecified digest=0f5338d25bb0 -->
#### [6f368e20] [context-editor][SDF-DAG] Phase 3: 4D Spatio-Temporal DAG — Keyframed SDF Animation & Replay
- priority: `medium`
- summary: The current renderer has no concept of time within the voxel structure. Physics and animation
- ref: `.ticket/tickets/6f368e20-00a4-4da5-9388-492ba4209915/ticket.toml`

<!-- ticket-index:entry id=0fc7b189-5c6c-4b79-a78d-5df8ad7dcf0c slug=open/unspecified digest=152cd9431a5e -->
#### [0fc7b189] [interview-api] Actionable answer-sheet synthesis and iteration loop
- priority: `high`
- summary: Design synthesis pipeline that turns interview and survey responses into actionable sheets, supports iterative refinement, and records provenance links back to source responses.
- ref: `.ticket/tickets/0fc7b189-5c6c-4b79-a78d-5df8ad7dcf0c/ticket.toml`

<!-- ticket-index:entry id=913fdd33-77b3-4e40-914a-db6873bf004d slug=open/unspecified digest=76205e53cad7 -->
#### [913fdd33] [interview-api] Interview sessions, survey orchestration, and answer synthesis
- priority: `high`
- summary: Plan an interview domain store that persists editable interview sessions, supports single-user and multi-user survey flows, and produces actionable synthesized answer sheets.
- ref: `.ticket/tickets/913fdd33-77b3-4e40-914a-db6873bf004d/ticket.toml`

<!-- ticket-index:entry id=1d6a7b5e-5f7f-4a8a-b6d8-cd3ab4c7e221 slug=open/unspecified digest=a380e88f79f5 -->
#### [1d6a7b5e] [interview-api] Session closure governance and unresolved-question routing
- priority: `high`
- summary: Define explicit policy for interview session closure and unresolved-question routing so session outcomes are deterministic and auditable.
- ref: `.ticket/tickets/1d6a7b5e-5f7f-4a8a-b6d8-cd3ab4c7e221/ticket.toml`

<!-- ticket-index:entry id=7639449a-22a9-4bea-9fcf-517810bc9ddf slug=open/unspecified digest=b1a5b6a9c549 -->
#### [7639449a] [interview-api] Session file model and collaborative survey state
- priority: `high`
- summary: Define and implement the persistent interview session model (files + indexes), editable prompt/response revisions, and multi-user survey participation state with conflict-safe updates.
- ref: `.ticket/tickets/7639449a-22a9-4bea-9fcf-517810bc9ddf/ticket.toml`

<!-- ticket-index:entry id=73b2cd22-942b-4205-86e5-333df2373211 slug=open/unspecified digest=d8d0d9f4ff21 -->
#### [73b2cd22] [memory-api] Shared tracing and log-api runtime diagnostics
- priority: `high`
- summary: The existing unified logging epic covers viewer/context logging, but `memory-api` domain crates and their CLI/MCP/HTTP transports still have uneven tracing setup and sparse instrumentation. Long-runn...
- ref: `.ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml`

<!-- ticket-index:entry id=9be61624-f8ab-42ca-980b-b1510e4136b1 slug=open/unspecified digest=098be4e3d412 -->
#### [9be61624] [memory-matrix] Close failure-bundle gaps for review readiness
- ref: `.ticket/tickets/9be61624-f8ab-42ca-980b-b1510e4136b1/ticket.toml`

<!-- ticket-index:entry id=16cfd19f-f1bd-4008-a15d-84037511f8fc slug=open/unspecified digest=910a18b945d2 -->
#### [16cfd19f] [migration] Convert 11 agent files off rule generator + delete agents rule-target
- summary: Migrate all `.agents/agents/*.agent.md` files off the rule-store generator to hand-owned by-description files, and retire the agents rule-target. Follow-up to CH-D (f43cb5cb) which established the mi...
- ref: `.ticket/tickets/16cfd19f-f1bd-4008-a15d-84037511f8fc/ticket.toml`

<!-- ticket-index:entry id=f43cb5cb-07dd-44c2-876b-8aceb2d841cb slug=open/unspecified digest=9dd31160ac26 -->
#### [f43cb5cb] [migration] Convert 12 instruction files off rule generator + delete agent-guidance rule-targets
- summary: Migrate all 12 `.agents/instructions/*.md` files off the rule-store generator to hand-owned by-description files, and retire the agent-guidance targets.
- ref: `.ticket/tickets/f43cb5cb-07dd-44c2-876b-8aceb2d841cb/ticket.toml`

<!-- ticket-index:entry id=76d0ace3-6e65-493d-9bef-fdc531081992 slug=open/unspecified digest=7cf8df7d698d -->
#### [76d0ace3] [migration] Convert 21 prompt files off rule generator + delete prompts rule-target
- summary: Migrate all `.agents/prompts/*.prompt.md` files off the rule-store generator to hand-owned by-description files, and retire the prompts rule-target. Follow-up to CH-D (f43cb5cb) which established the...
- ref: `.ticket/tickets/76d0ace3-6e65-493d-9bef-fdc531081992/ticket.toml`

<!-- ticket-index:entry id=a325f7f4-2f6f-4b42-acb0-2a8daa01281f slug=open/unspecified digest=215f1acc92a5 -->
#### [a325f7f4] [missing-rule] Add missing rule for situation: rule coverage gap
- priority: `medium`
- summary: A session situation query returned no matching rule.
- ref: `memory-api/.ticket/tickets/a325f7f4-2f6f-4b42-acb0-2a8daa01281f/ticket.toml`

<!-- ticket-index:entry id=0dde154a-ee4d-4f0a-af83-e0a4864d3bfb slug=open/unspecified digest=ec588135c783 -->
#### [0dde154a] [peek-cli] --grep does not support regex alternation (\\|)
- summary: `--grep` with regex alternation (`\|`) reports no match even when individual alternatives do match:
- ref: `.ticket/tickets/0dde154a-ee4d-4f0a-af83-e0a4864d3bfb/ticket.toml`

<!-- ticket-index:entry id=2ea8ec57-fc71-46d2-8eba-a3de40c5bec2 slug=open/unspecified digest=60c09dedbb19 -->
#### [2ea8ec57] [peek-cli] --grep shows only line numbers, not matching line content
- summary: `peek --grep <pattern>` outputs bare line numbers only, with no preview of the matched line text:
- ref: `.ticket/tickets/2ea8ec57-fc71-46d2-8eba-a3de40c5bec2/ticket.toml`

<!-- ticket-index:entry id=c37ea985-3647-421f-99eb-75860a0728e0 slug=open/unspecified digest=7437d6202da5 -->
#### [c37ea985] [profiling] CLI/HTTP/MCP end-to-end test matrix (ticket + spec surfaces)
- priority: `medium`
- summary: Child of tracker `ef3f4a91`. Build a parity E2E test matrix that exercises the
- ref: `.ticket/tickets/c37ea985-3647-421f-99eb-75860a0728e0/ticket.toml`

<!-- ticket-index:entry id=2d59b99c-0205-4bf6-bad9-ecb69a52830a slug=open/unspecified digest=fbe54c4236d0 -->
#### [2d59b99c] [profiling] CLI/HTTP/MCP throughput/latency benchmarks
- priority: `low`
- summary: Child of tracker `ef3f4a91`. Add transport-level throughput/latency benchmarks
- ref: `.ticket/tickets/2d59b99c-0205-4bf6-bad9-ecb69a52830a/ticket.toml`

<!-- ticket-index:entry id=6a19ae5f-8695-47e1-8b21-1062e0546fda slug=open/unspecified digest=1bee2c9c20ed -->
#### [6a19ae5f] [profiling] Native Criterion benchmark matrix for context-* + ticket/spec APIs
- priority: `medium`
- summary: Child of tracker `ef3f4a91`. Add native Criterion benchmarks covering the
- ref: `.ticket/tickets/6a19ae5f-8695-47e1-8b21-1062e0546fda/ticket.toml`

<!-- ticket-index:entry id=d8d18128-656e-4a13-9983-946d6af33c27 slug=open/unspecified digest=6157f3c86272 -->
#### [d8d18128] [profiling] Testing + benchmark matrix index doc and run commands
- priority: `low`
- summary: Child of tracker `ef3f4a91`. Author the single index document that ties the
- ref: `.ticket/tickets/d8d18128-656e-4a13-9983-946d6af33c27/ticket.toml`

<!-- ticket-index:entry id=8a90a63c-0a07-439f-90e8-9124212b2dc8 slug=open/unspecified digest=6f10defc7955 -->
#### [8a90a63c] [program][multi-store] Store expansion and operational health program
- priority: `high`
- summary: Umbrella program for new store domains and operational quality loops extending the cross-store architecture.
- ref: `.ticket/tickets/8a90a63c-0a07-439f-90e8-9124212b2dc8/ticket.toml`

<!-- ticket-index:entry id=23e81ad8-b67c-49af-97b5-f90f8bb0ae2c slug=open/unspecified digest=bab23bee664b -->
#### [23e81ad8] [rule+skill] Rule-store sources for domain-store scaffolding instructions
- priority: `high`
- summary: Create canonical rule entries and generation targets for instruction files and slash-command prompt assets that encode architecture-decisions.md and tracker 671d4e47 guidelines.
- ref: `.ticket/tickets/23e81ad8-b67c-49af-97b5-f90f8bb0ae2c/ticket.toml`

<!-- ticket-index:entry id=a87dcdf9-0638-4c84-a4ed-c8f4d3518e72 slug=open/unspecified digest=563bbd0544dd -->
#### [a87dcdf9] [scaffold] Rollout guardrails, feature flags, and rollback protocol
- priority: `high`
- summary: Define operational rollout controls for scaffold automation, including feature flags, safe defaults, and rollback handling when regression gates fail.
- ref: `.ticket/tickets/a87dcdf9-0638-4c84-a4ed-c8f4d3518e72/ticket.toml`

<!-- ticket-index:entry id=66fae806-203d-4235-9151-4272eb0bb603 slug=open/unspecified digest=55bc0c41f18f -->
#### [66fae806] [scaffold] Rule-generated store bootstrap instructions and slash command skill
- priority: `high`
- summary: Plan rule-generated instruction and prompt assets plus slash command skill for bootstrapping a minimally functional new domain store from one prompt, aligned with architecture-decisions and cross-sto...
- ref: `.ticket/tickets/66fae806-203d-4235-9151-4272eb0bb603/ticket.toml`

<!-- ticket-index:entry id=effba966-f0a8-4d7d-b289-b7feba826cf8 slug=open/unspecified digest=2ff4eb9a8b95 -->
#### [effba966] [session-bootstrap][epic] Dynamic session bootstrapping & context routing redesign
- summary: Redesign agent startup and continuation around a durable logical session workspace rather than always-on static context or transcript replay.
- ref: `memory-api/.ticket/tickets/effba966-f0a8-4d7d-b289-b7feba826cf8/ticket.toml`

<!-- ticket-index:entry id=fd77db9b-b9e7-4e04-aac2-7c4b68061c6b slug=open/unspecified digest=452e80afd388 -->
#### [fd77db9b] [skill] Author Dioxus skill (signals, server fns, WASM toolchain, viewer-api integration, styling)
- summary: Hand-author the Dioxus skill — the one true ecosystem gap (best skills.sh result ~71 installs).
- ref: `.ticket/tickets/fd77db9b-b9e7-4e04-aac2-7c4b68061c6b/ticket.toml`

<!-- ticket-index:entry id=07d4b1b0-bc20-4ba7-98d4-ed09365f0437 slug=open/unspecified digest=548eca5e5429 -->
#### [07d4b1b0] [skill] One-prompt domain-store scaffold slash command flow
- priority: `high`
- summary: Implement slash command flow that accepts one prompt and scaffolds a minimal domain store (crate layout, manifests, base APIs, tests, and registration hooks) using generated instructions.
- ref: `.ticket/tickets/07d4b1b0-bc20-4ba7-98d4-ed09365f0437/ticket.toml`

<!-- ticket-index:entry id=5f217947-0fde-431c-b1ab-9d5d8153ed17 slug=open/unspecified digest=5790ad681123 -->
#### [5f217947] [skill] Skill directory contract + re-home orphan + master index
- summary: Establish the skill directory contract and clean up the current mess.
- ref: `.ticket/tickets/5f217947-0fde-431c-b1ab-9d5d8153ed17/ticket.toml`

<!-- ticket-index:entry id=f7112b9f-55d7-4c15-993b-83568ffb09fe slug=open/unspecified digest=259b2314a6d0 -->
#### [f7112b9f] [skill] Vendor 8 adopted skills.sh skills (Rust, Playwright, WebGPU, interviewing, authoring)
- summary: Vendor the confirmed proven skills.sh skills into the repo, normalized to the contract.
- ref: `.ticket/tickets/f7112b9f-55d7-4c15-993b-83568ffb09fe/ticket.toml`

<!-- ticket-index:entry id=15bad110-f762-4a2b-b20a-f91fa66cf2ec slug=open/unspecified digest=6f6261562aa5 -->
#### [15bad110] [spec-cleanup] Prune fixture/empty specs + consolidate ultra-granular specs (reviewable)
- summary: Prune worthless specs and consolidate ultra-granular ones. Reviewable, destructive — never run during a planning pass.
- ref: `.ticket/tickets/15bad110-f762-4a2b-b20a-f91fa66cf2ec/ticket.toml`

<!-- ticket-index:entry id=9acf1ef1-a7fb-40af-8a7a-4df89ac9a93f slug=open/unspecified digest=b9f46acc51f2 -->
#### [9acf1ef1] [ticket-api] Allow reverse ticket state transitions through schema
- summary: Ticket state transitions are currently constrained by schema edges that only cover the forward workflow path in practice, which prevents moving a ticket back to an earlier valid state through the sam...
- ref: `.ticket/tickets/9acf1ef1-a7fb-40af-8a7a-4df89ac9a93f/ticket.toml`

<!-- ticket-index:entry id=86cde60c-49db-4820-a3a9-37c472ca1c2f slug=open/unspecified digest=db82406ef43d -->
#### [86cde60c] [ticket-api] Distinguish deferred and meta work from actionable tickets
- summary: Deferred, parent, and roadmap-style tickets look too much like actionable implementation tickets.
- ref: `.ticket/tickets/86cde60c-49db-4820-a3a9-37c472ca1c2f/ticket.toml`

<!-- ticket-index:entry id=acefc2ae-e257-4bc8-a4c7-0ec3137e374d slug=open/unspecified digest=397920d23362 -->
#### [acefc2ae] [ticket-api] Validation-aware dependency requirements and health model
- priority: `high`
- summary: Plan how ticket dependencies can declare required validation items whose satisfaction is resolved through test-api evidence rather than ad hoc ticket text.
- ref: `.ticket/tickets/acefc2ae-e257-4bc8-a4c7-0ec3137e374d/ticket.toml`

<!-- ticket-index:entry id=61cbc31f-c66d-46bf-807e-0d4236e04c9e slug=open/unspecified digest=206f93eb6e00 -->
#### [61cbc31f] [ticket-cli] Explain why tickets are absent from next
- summary: `ticket search` and `ticket next` do not explain their mismatch.
- ref: `.ticket/tickets/61cbc31f-c66d-46bf-807e-0d4236e04c9e/ticket.toml`

<!-- ticket-index:entry id=68e3c713-3c35-4d7e-af0c-b4a55a3253c0 slug=open/unspecified digest=34596b5dffde -->
#### [68e3c713] [ticket-cli] Fix next --filter matching for prefix and substring queries
- summary: `ticket next --filter` behaved inconsistently enough to break targeted discovery.
- ref: `.ticket/tickets/68e3c713-3c35-4d7e-af0c-b4a55a3253c0/ticket.toml`

<!-- ticket-index:entry id=f3305925-7217-4ff3-8c4e-820ebc1e6de5 slug=open/unspecified digest=a91153c9b1fa -->
#### [f3305925] [ticket-cli] Graph rendering and closure-aware dependency display
- priority: `high`
- summary: Plan a reusable graph-rendering primitive for ticket and related CLI surfaces, including ASCII and Mermaid outputs and closure-aware expansion over dependency subgraphs.
- ref: `.ticket/tickets/f3305925-7217-4ff3-8c4e-820ebc1e6de5/ticket.toml`

<!-- ticket-index:entry id=d241a482-6fc7-468e-b0a3-748cb72d07eb slug=open/unspecified digest=3adbb99bfad1 -->
#### [d241a482] [ticket-cli][spec-cli] Normalize sibling CLI grammar and JSON envelopes
- summary: The sibling ticket/spec CLIs make automation harder than necessary because their command grammar and JSON envelopes drift in incompatible ways.
- ref: `.ticket/tickets/d241a482-6fc7-468e-b0a3-748cb72d07eb/ticket.toml`

<!-- ticket-index:entry id=61cb6557-e559-4eae-8e59-ea0d520a3bee slug=open/unspecified digest=a8801a47cf45 -->
#### [61cb6557] [ticket-cli][ticket-mcp] Add consolidated ticket detail/context read surface
- summary: Reviewing a ticket currently requires chaining multiple read surfaces.
- ref: `memory-api/.ticket/tickets/61cb6557-e559-4eae-8e59-ea0d520a3bee/ticket.toml`

<!-- ticket-index:entry id=def7fa82-6f4a-4354-b52d-ae7ea9623648 slug=open/unspecified digest=72daede02cdf -->
#### [def7fa82] [ticket-cli][ticket-mcp] Make stale board entries directly check-outable
- summary: `ticket board show` exposed a stale entry with an `entry_id`, `ticket_id`, `agent_id`, and owned files, but `ticket board check-out <ticket-id>` still failed with `no active board entry found` until ...
- ref: `.ticket/tickets/def7fa82-6f4a-4354-b52d-ae7ea9623648/ticket.toml`

<!-- ticket-index:entry id=43fc22b3-9b36-4a54-b520-f51000330a46 slug=open/unspecified digest=77fef266ff21 -->
#### [43fc22b3] [ticket-graph] Tracker: validation-aware graph tooling and audit enforcement
- priority: `high`
- summary: Coordinate planning and delivery for ticket graph rendering, validation-aware dependency requirements, and audit/board enforcement built on the existing ticket graph and test-api evidence store.
- ref: `.ticket/tickets/43fc22b3-9b36-4a54-b520-f51000330a46/ticket.toml`

<!-- ticket-index:entry id=14df656e-cef2-470e-9530-ef760b6c462c slug=open/unspecified digest=469af9ed2970 -->
#### [14df656e] [ticket-viewer][ticket-vscode] Surface the next-work workflow in frontends
- summary: There is no end-user frontend surface for the "best next ticket to implement" workflow.
- ref: `memory-api/.ticket/tickets/14df656e-cef2-470e-9530-ef760b6c462c/ticket.toml`

<!-- ticket-index:entry id=814f22dc-0f75-4c11-b7da-20b3c5928cea slug=open/unspecified digest=49c1c241c787 -->
#### [814f22dc] [ticket-vscode] Fix VS Code CLI discovery in VSIX installer
- ref: `.ticket/tickets/814f22dc-0f75-4c11-b7da-20b3c5928cea/ticket.toml`

<!-- ticket-index:entry id=51671748-8933-4955-9bf4-7bdea961df40 slug=open/unspecified digest=1dc65a607dd6 -->
#### [51671748] [ticket-workflow] Harden best-next-ticket discovery across spec, CLI, MCP, and frontends
- summary: We now have several issue slices for board / next discovery, but no ticket owns the full contract for finding the best next ticket to implement.
- ref: `memory-api/.ticket/tickets/51671748-8933-4955-9bf4-7bdea961df40/ticket.toml`

<!-- ticket-index:entry id=74b56d66-d94f-4422-bda6-5f583d8f7ec4 slug=open/unspecified digest=d88c3668f9ef -->
#### [74b56d66] [tool-metrics][T2 follow-up] Confirm live-session AC1 evidence for output_char_sizes via hook_payload
- summary: Ticket 44119807 (T2, output-size capture) claims AC1 satisfied via the e2e test `e2e_hook_binary_captures_output_chars_from_hook_stdin_tool_response`, but no genuinely live-captured session has ever ...
- ref: `.ticket/tickets/74b56d66-d94f-4422-bda6-5f583d8f7ec4/ticket.toml`

<!-- ticket-index:entry id=eb984596-c766-402b-942f-0087208a9004 slug=open/unspecified digest=51109e919e4c -->
#### [eb984596] [tool-metrics][T3] Add measurement-coverage metric so unmeasured is distinguishable from healthy
- summary: [unmeasured-tool-policy.md](.spec/specs/29ae5f6e-c202-41f1-ba88-a446aa872993/sections/unmeasured-tool-policy.md) assigns cost **0** to any tool absent from the rollup and always allows it: *"Unproven...
- ref: `.ticket/tickets/eb984596-c766-402b-942f-0087208a9004/ticket.toml`

<!-- ticket-index:entry id=70222986-3325-4d45-892e-31e7f4d09aa6 slug=open/unspecified digest=85455fd3df63 -->
#### [70222986] [validation] E2E regression harness for domain-store scaffold prompts
- priority: `high`
- summary: Implement automated end-to-end regression testing for scaffold prompts, validating generated store correctness, compile/test health, and architecture-conformance checks across representative prompt s...
- ref: `.ticket/tickets/70222986-3325-4d45-892e-31e7f4d09aa6/ticket.toml`

<!-- ticket-index:entry id=2ff2c8e8-eaec-4bd9-9312-ae13cd4b243a slug=open/unspecified digest=e59d4645c61a -->
#### [2ff2c8e8] [validation] Prompt replay matrix for scaffold skill domain coverage
- priority: `high`
- summary: Build representative prompt matrix (simple, medium, complex, edge-case) and replay harness to validate scaffold skill behavior, compile health, and architecture conformance across domain types.
- ref: `.ticket/tickets/2ff2c8e8-eaec-4bd9-9312-ae13cd4b243a/ticket.toml`

<!-- ticket-index:entry id=0c72ecac-abaf-4fba-84e6-40c5ad24a941 slug=open/unspecified digest=633d9fa7fbcb -->
#### [0c72ecac] [validation] Prompt-replay matrix — skill discovery by description across all domains
- summary: Build an automated prompt-replay matrix that proves an agent can locate and load the correct skill by its description for each target domain.
- ref: `.ticket/tickets/0c72ecac-abaf-4fba-84e6-40c5ad24a941/ticket.toml`

<!-- ticket-index:entry id=dedac9f5-0d4d-4ad0-8a7e-4acd361c273e slug=open/unspecified digest=2c7be950b552 -->
#### [dedac9f5] [validation] Rule-target generation drift checks for scaffold guidance assets
- priority: `high`
- summary: Automate checks that rule-source entries and generated instruction/prompt outputs for domain-store scaffolding remain in sync, with stable snapshots and actionable diff reporting.
- ref: `.ticket/tickets/dedac9f5-0d4d-4ad0-8a7e-4acd361c273e/ticket.toml`

<!-- ticket-index:entry id=c878707f-1581-49b7-9b67-39c5b11bb34a slug=open/unspecified digest=d8b8bd6f3145 -->
#### [c878707f] rule_overlap trial test fails: RuleStore::init picks up state outside its tempdir, causing DuplicateSlug on first insert
- summary: Discovered while reviewing d1b3a6c9. `cargo test -p audit-api` intermittently fails on `trials::rule_overlap::...reports_high_overlap_between_near_duplicate_rules` with a `DuplicateSlug` panic on the...
- ref: `memory-api/.ticket/tickets/c878707f-1581-49b7-9b67-39c5b11bb34a/ticket.toml`

<!-- ticket-index:entry id=7ad82895-44d1-42b1-bade-3b426d32a98d slug=open/unspecified digest=c4aaded334f4 -->
#### [7ad82895] ticket-http: 5 registry/routes tests expect bare workspace label, not collision-safe hash-suffixed name
- summary: `cargo test -p ticket-http` fails 5 of 78 tests (verified independently, both parallel and single-threaded — same 5 failures either way, ruling out test-isolation/ordering):
- ref: `.ticket/tickets/7ad82895-44d1-42b1-bade-3b426d32a98d/ticket.toml`

<!-- ticket-index:entry id=1d6a033e-c263-4a93-8b31-f62e69345867 slug=open/unspecified digest=73929dc8dfb9 -->
#### [1d6a033e] ticket-mcp add_edge fails with entity-not-found on entities that get_ticket/update_ticket can resolve
- summary: Reproduced independently during the d1b3a6c9 review/reconciliation: `add_edge` (both directions) fails with `entity not found: d1b3a6c9-5f2e-4f6b-9b3c-8fa1e2d3c4b5` against workspace `memory-api/.tic...
- ref: `.ticket/tickets/1d6a033e-c263-4a93-8b31-f62e69345867/ticket.toml`

<!-- ticket-index:entry id=604ee0a0-d07e-47d0-99e9-c6ee61283dc4 slug=open/unspecified digest=476a32ddc520 -->
#### [604ee0a0] ticket-mcp update_ticket/add_edge/create_ticket report ok but silently drop writes (state, depends_on, component, priority)
- summary: Widening 1d6a033e: this is not limited to add_edge. During this iteration, `update_ticket` reported `{\"status\":\"ok\"}` for two separate writes and neither persisted:\n1. `update_ticket(id=d1b3a6c9...
- ref: `.ticket/tickets/604ee0a0-d07e-47d0-99e9-c6ee61283dc4/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=99e78a95-79c8-4484-8f88-7a91dc5c3860 slug=open/viewer-api digest=61542755a3b7 -->
#### [99e78a95] Build viewer evidence suite
- priority: `high`
- summary: Build stable viewer evidence using API fixtures and Playwright screenshots without data-dependent skips.
- ref: `.ticket/tickets/99e78a95-79c8-4484-8f88-7a91dc5c3860/ticket.toml`

<!-- ticket-index:entry id=254ac30d-26c0-4bfe-8a66-b10ab9e4843a slug=open/viewer-api digest=8ddf49ef990c -->
#### [254ac30d] [viewer-api] Generalize graph improvements to spec-viewer and log-viewer
- priority: `high`
- summary: Generalize the four graph improvements implemented in ticket-viewer to spec-viewer and log-viewer:
- ref: `.ticket/tickets/254ac30d-26c0-4bfe-8a66-b10ab9e4843a/ticket.toml`

<!-- ticket-index:entry id=e8d9bfcd-d729-43a6-8efa-4554af609d0c slug=open/viewer-api digest=94eff41d8570 -->
#### [e8d9bfcd] [viewer-api] Update Graph3D component documentation and examples
- priority: `medium`
- summary: Update Graph3D component documentation and examples to reflect the four graph improvements and provide clear integration guidance:
- ref: `.ticket/tickets/e8d9bfcd-d729-43a6-8efa-4554af609d0c/ticket.toml`


### Component: viewer-api-e2e

<!-- ticket-index:entry id=08c86dbd-72b8-446b-a930-30ef3352d604 slug=open/viewer-api-e2e digest=ee102ea55ec1 -->
#### [08c86dbd] [viewer-api] Create comprehensive E2E test suite for graph improvements
- priority: `medium`
- summary: Create comprehensive Playwright E2E test coverage for the four graph improvements across all memory-viewers:
- ref: `.ticket/tickets/08c86dbd-72b8-446b-a930-30ef3352d604/ticket.toml`


### Component: workflow-policy

<!-- ticket-index:entry id=d883f454-8787-4e61-85a2-16c48802c440 slug=open/workflow-policy digest=f79bb042d438 -->
#### [d883f454] [workflow-policy][tracing][log-api] Research and define tracing instrumentation and log execution policy
- priority: `high`
- summary: Research and codify the repository policy for tracing instrumentation, runtime log capture, and managing generated logs plus executions through `log-api`.
- ref: `.ticket/tickets/d883f454-8787-4e61-85a2-16c48802c440/ticket.toml`


### Component: workflow-skill

<!-- ticket-index:entry id=b9a52b79-2beb-4710-958d-25582ed79dcf slug=open/workflow-skill digest=4547d4d130a8 -->
#### [b9a52b79] [workflow-tools][skill] Author workflow-skill skills.sh package as the installable entry point
- priority: `high`
- summary: Phase D. Author the `workflow-skill` — a skills.sh-native SKILL.md package that is the installable entry point for the whole workflow-tooling system. A single install should be able to discover, retr...
- ref: `.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml`

<!-- ticket-index:entry id=24d47244-fcb4-46cf-9bc8-c78c29fe7ff2 slug=open/workflow-skill digest=3ff1899e8bd2 -->
#### [24d47244] [workflow-tools][skill] Define skill scope/precedence for root and nested installs (single active install, self-uninstall)
- priority: `high`
- summary: Phase D. Define and implement the scope/precedence model for the workflow-skill so it works both at the repository root and at deeply nested tool repos without conflicting or being read twice.
- ref: `.ticket/tickets/24d47244-fcb4-46cf-9bc8-c78c29fe7ff2/ticket.toml`


### Component: workflow-tools

<!-- ticket-index:entry id=47a0bcc3-f42d-475e-b05a-777293c4698e slug=open/workflow-tools digest=0b57b7acce83 -->
#### [47a0bcc3] [workflow-tools][artifacts] Establish repo-level and per-tool self-referential artifact stores
- priority: `high`
- summary: Phase C. Establish repo-level self-referential artifact stores in `workflow-tools` (and confirm per-tool artifact stores in each tool repo) so work on the tools themselves is tracked with the tools t...
- ref: `.ticket/tickets/47a0bcc3-f42d-475e-b05a-777293c4698e/ticket.toml`

<!-- ticket-index:entry id=47f2a664-7803-4074-b40c-f41d3caf0c54 slug=open/workflow-tools digest=7b2ee06ca773 -->
#### [47f2a664] [workflow-tools][artifacts] Migrate tool-scoped artifacts into their owning tool repositories
- priority: `high`
- summary: Phase C. Migrate tool-scoped artifacts (tickets, specs, docs, rules, tests) that currently live in the context-engine default store or the memory-api store into the correct per-tool repo stores, pres...
- ref: `.ticket/tickets/47f2a664-7803-4074-b40c-f41d3caf0c54/ticket.toml`

<!-- ticket-index:entry id=0b527d28-9487-4a6c-8c7a-835b4a5d9582 slug=open/workflow-tools digest=0ed65ee39bc1 -->
#### [0b527d28] [workflow-tools][docs] Migration guide and dependency-install documentation for the general framework
- priority: `medium`
- summary: Phase F. Produce the migration guide and dependency-install documentation so the new structure is reproducible and the general framework is documented: how a target project installs workflow-tools, h...
- ref: `.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml`

<!-- ticket-index:entry id=67e254b5-8eac-4251-b640-ec3649f250dd slug=open/workflow-tools digest=753bce8a446a -->
#### [67e254b5] [workflow-tools][entry-points] Wire AGENTS.md/instructions to the workflow-skill across all three install sites
- priority: `high`
- summary: Phase E. Update the agent entry points and guidance across all three install sites so an agent starting at any level is pointed at the workflow-skill guidance: (1) context-engine (uses the tools), (2...
- ref: `.ticket/tickets/67e254b5-8eac-4251-b640-ec3649f250dd/ticket.toml`

<!-- ticket-index:entry id=866049e7-48c9-4b63-b0d2-dd4f987823dd slug=open/workflow-tools digest=09bf6916a45f -->
#### [866049e7] [workflow-tools][foundations] Break the test/log dependency cycle by relocating validation-execution adapters
- priority: `high`
- summary: The repository dependency graph currently contains a cycle that blocks two
- ref: `.ticket/tickets/866049e7-48c9-4b63-b0d2-dd4f987823dd/ticket.toml`

<!-- ticket-index:entry id=bfc52d17-8a3f-40d6-a1bd-3c618ad67f69 slug=open/workflow-tools digest=c719f995b11a -->
#### [bfc52d17] [workflow-tools][foundations] Make memory-matrix build standalone against externally resolved domain crates (blocked)
- priority: `medium`
- summary: Split out of MATRIX-FOLLOWUP 15e632f1. That ticket carried two criteria: (a) rewire in-tree consumers to the external memory-fixtures git dep, and (b) make memory-matrix build standalone against exte...
- ref: `.ticket/tickets/bfc52d17-8a3f-40d6-a1bd-3c618ad67f69/ticket.toml`

<!-- ticket-index:entry id=4cbe11d3-de47-45f5-8c31-6678f6e4a139 slug=open/workflow-tools digest=837706381c01 -->
#### [4cbe11d3] [workflow-tools][foundations] Promote neutral artifact traits from test-api into memory-kernel
- priority: `high`
- summary: `test-api` currently owns two artifact traits that are entirely domain-neutral:
- ref: `.ticket/tickets/4cbe11d3-de47-45f5-8c31-6678f6e4a139/ticket.toml`

<!-- ticket-index:entry id=69eb4118-19ec-4b5b-bb12-30e314029cc5 slug=open/workflow-tools digest=2d05281b6500 -->
#### [69eb4118] [workflow-tools][migration] Extract workflow tooling into standalone per-tool repositories and reframe context-engine as a consuming example
- priority: `high`
- summary: Tracking epic for the repository restructuring that turns this monorepo into an instantiated example of a general, self-improving agent framework, and extracts the workflow tooling into standalone, i...
- ref: `.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml`

<!-- ticket-index:entry id=858c5286-6c2b-4a05-a0f3-4e8f6b90b75e slug=open/workflow-tools digest=92c7c35a05ef -->
#### [858c5286] [workflow-tools][per-tool] Split each domain tool into its own repository (parent tracker)
- priority: `high`
- summary: Phase B parent tracker. Split each of the 11 domain tools into its own bare-named repository under github.com/mankinskin. Each tool repo is built around a single domain crate (see contract `0da6894c`...
- ref: `.ticket/tickets/858c5286-6c2b-4a05-a0f3-4e8f6b90b75e/ticket.toml`

<!-- ticket-index:entry id=b525a7fa-f59d-4a14-b234-2ec7b8a42e95 slug=open/workflow-tools digest=f82e7d362225 -->
#### [b525a7fa] [workflow-tools][umbrella] Create workflow-tools umbrella aggregating tool and shared repositories
- priority: `high`
- summary: Phase C. Create the `workflow-tools` umbrella repository that aggregates the extracted shared libs (`memory-kernel`, `viewer-api`, `memory-fixtures`) and the 11 per-tool repos as dependencies (submod...
- ref: `.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml`

<!-- ticket-index:entry id=2345ba7f-6d83-449b-bf07-d541c5f8e01e slug=open/workflow-tools digest=bde839b56eac -->
#### [2345ba7f] [workflow-tools][validation] End-to-end validation and cutover across split repositories
- priority: `high`
- summary: Phase F. End-to-end validation and cutover across the split repositories: prove that the extracted tools build, test, and operate correctly both standalone and aggregated, and that context-engine wor...
- ref: `.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml`


## State: planned

### Component: agent-guidance

<!-- ticket-index:entry id=fdd059ed-69e4-4328-9167-ea4986aee788 slug=planned/agent-guidance digest=28ecb2af3494 -->
#### [fdd059ed] [agents] Rework Terminal Command Agent for autonomous command execution
- priority: `high`
- summary: Change Terminal Command Agent planning to select only catalog-registered commands and templates. Keep the fixed client tool inventory and user confirmation gate; commands absent from the catalog are ...
- ref: `.ticket/tickets/fdd059ed-69e4-4328-9167-ea4986aee788/ticket.toml`


### Component: agent-harness

<!-- ticket-index:entry id=0f4b3c5b-c5e9-45c4-968c-a8878f359de8 slug=planned/agent-harness digest=d637cf2ecbc2 -->
#### [0f4b3c5b] [agent-harness] Unified minimal interface for on-demand chat + long-running agent loops (Rust core, TUI + WASM)
- priority: `high`
- summary: One cohesive delivery plan that turns existing research and design direction into
- ref: `.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/ticket.toml`

<!-- ticket-index:entry id=fd93671d-2a86-4996-9d26-efcfce156095 slug=planned/agent-harness digest=2bf2bbabcc99 -->
#### [fd93671d] [agent-harness][CH10] Reliability/recovery: checkpointing + reconnect semantics
- priority: `high`
- summary: WS6. Implements D2 persistence and reconnect continuity.
- ref: `.ticket/tickets/fd93671d-2a86-4996-9d26-efcfce156095/ticket.toml`

<!-- ticket-index:entry id=b01a2fbf-6682-4dee-abce-95cdcf4fd325 slug=planned/agent-harness digest=0be02c00dae3 -->
#### [b01a2fbf] [agent-harness][CH11] E2E + Playwright + manual browser verification evidence
- priority: `high`
- summary: WS7 release gate.
- ref: `.ticket/tickets/b01a2fbf-6682-4dee-abce-95cdcf4fd325/ticket.toml`

<!-- ticket-index:entry id=a4273210-ef12-4372-bd30-6e112c9d708e slug=planned/agent-harness digest=b76e2ef13c96 -->
#### [a4273210] [agent-harness][CH12] Docs/runbooks + rollout checklist
- priority: `medium`
- summary: WS7 release readiness.
- ref: `.ticket/tickets/a4273210-ef12-4372-bd30-6e112c9d708e/ticket.toml`

<!-- ticket-index:entry id=a5f08931-24af-4b96-a156-9107c776f946 slug=planned/agent-harness digest=af7909cc1a95 -->
#### [a5f08931] [agent-harness][CH1] Workspace + crate scaffolding + shared protocol contracts
- priority: `high`
- summary: Foundation slice (WS1). Establishes crate boundaries (D1) and the shared
- ref: `.ticket/tickets/a5f08931-24af-4b96-a156-9107c776f946/ticket.toml`

<!-- ticket-index:entry id=c684b092-7f5a-4ebe-aa6d-494f666f5dc8 slug=planned/agent-harness digest=8158a96475c7 -->
#### [c684b092] [agent-harness][CH2] Core loop state machine + unified session/mode model
- priority: `high`
- summary: WS2 core. The single session model that makes chat and loop two modes of one thing.
- ref: `.ticket/tickets/c684b092-7f5a-4ebe-aa6d-494f666f5dc8/ticket.toml`

<!-- ticket-index:entry id=036c270f-6ca7-4372-96e2-570a26e3fdd0 slug=planned/agent-harness digest=af45ba6ef79e -->
#### [036c270f] [agent-harness][CH3] Provider abstraction + guidance injector + budget/policy hooks
- priority: `high`
- summary: WS2. Implements D4 (budgets) and D5 (guidance precedence).
- ref: `.ticket/tickets/036c270f-6ca7-4372-96e2-570a26e3fdd0/ticket.toml`

<!-- ticket-index:entry id=1c63db9d-afb3-4678-b0f6-14e6a4d5daca slug=planned/agent-harness digest=caedbe04ab8b -->
#### [1c63db9d] [agent-harness][CH4] MCP integration + per-session tool routing envelope
- priority: `high`
- summary: WS3. Answers research Q6 (routing MCP instances to the right session).
- ref: `.ticket/tickets/1c63db9d-afb3-4678-b0f6-14e6a4d5daca/ticket.toml`

<!-- ticket-index:entry id=136af497-869b-4cc5-b059-9041a98e5ad3 slug=planned/agent-harness digest=6ed651542c20 -->
#### [136af497] [agent-harness][CH5] Sandboxed command execution with policy gates
- priority: `high`
- summary: WS3. Implements D3 isolation boundary.
- ref: `.ticket/tickets/136af497-869b-4cc5-b059-9041a98e5ad3/ticket.toml`

<!-- ticket-index:entry id=8ed0edbf-a765-4f4a-b50e-695aa79e9180 slug=planned/agent-harness digest=2296ec3fd175 -->
#### [8ed0edbf] [agent-harness][CH6] Axum session lifecycle + websocket broadcast fanout
- priority: `high`
- summary: WS4. The streaming control plane both UIs consume.
- ref: `.ticket/tickets/8ed0edbf-a765-4f4a-b50e-695aa79e9180/ticket.toml`

<!-- ticket-index:entry id=3c208991-1d98-4a9c-be29-890d15244b8d slug=planned/agent-harness digest=bba75f50a513 -->
#### [3c208991] [agent-harness][CH7] Ratatui minimal operator interface
- priority: `high`
- summary: WS5 native client.
- ref: `.ticket/tickets/3c208991-1d98-4a9c-be29-890d15244b8d/ticket.toml`

<!-- ticket-index:entry id=86f95ad8-8d61-43b6-a463-8719b29007c0 slug=planned/agent-harness digest=581bd8a681ae -->
#### [86f95ad8] [agent-harness][CH8] Dioxus/WASM minimal interface parity
- priority: `high`
- summary: WS5 browser client.
- ref: `.ticket/tickets/86f95ad8-8d61-43b6-a463-8719b29007c0/ticket.toml`

<!-- ticket-index:entry id=a496cad3-cdc5-4237-b432-47a6bb43b9c5 slug=planned/agent-harness digest=16a7b11da131 -->
#### [a496cad3] [agent-harness][CH9] Diff preview integration in both clients
- priority: `medium`
- summary: WS5. Consistent code-change preview before execution confirmation.
- ref: `.ticket/tickets/a496cad3-cdc5-4237-b432-47a6bb43b9c5/ticket.toml`


### Component: agent-tooling

<!-- ticket-index:entry id=e342cc4c-a7a4-42de-81fc-572d0497d12b slug=planned/agent-tooling digest=12f0544415f4 -->
#### [e342cc4c] Token-optimized default agent tool suite: peek + compact-terminal + design call for edit/filesystem/search tools
- summary: Implementation note (all three child tickets)**: These are **net-new implementations**, not extractions. The precedent ticket `bd5e9aee` extracted an already-existing `compact-terminal-mcp` into laye...
- ref: `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b/ticket.toml`

<!-- ticket-index:entry id=685b577e-9e5e-4c96-86de-ce5420db46bc slug=planned/agent-tooling digest=a08dfb85c497 -->
#### [685b577e] [token-efficiency] Add pre-flight write validation gates
- priority: `high`
- summary: Strengthen local pre-flight validation so expensive syntax-debugging loops are rejected before code is saved or finalized.
- ref: `.ticket/tickets/685b577e-9e5e-4c96-86de-ce5420db46bc/ticket.toml`

<!-- ticket-index:entry id=e29e24ba-1f7e-43b3-97bd-c20d53b76df8 slug=planned/agent-tooling digest=08101b0dd2d4 -->
#### [e29e24ba] [token-efficiency] Make MCP update tools accept sparse payloads and return minimal changed fields
- priority: `high`
- summary: Make MCP update tools accept sparse payloads that include only the keys being changed, and return minimal response payloads that include only changed or directly relevant fields.
- ref: `.ticket/tickets/e29e24ba-1f7e-43b3-97bd-c20d53b76df8/ticket.toml`


### Component: cli

<!-- ticket-index:entry id=7bf50e75-018e-4b70-b93f-2bac099f9677 slug=planned/cli digest=03ca56439877 -->
#### [7bf50e75] Plan: Sandboxed integration tests for context-tasks
- summary: tags: `#plan` `#testing` `#integration` `#context-tasks` `#sandbox`
- ref: `.ticket/tickets/7bf50e75-018e-4b70-b93f-2bac099f9677/ticket.toml`

<!-- ticket-index:entry id=b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72 slug=planned/cli digest=c9f83c98d760 -->
#### [b1f3e2a4] [bootstrap][T2] enforce assignment start context branch and cwd checks
- summary: When a worker is dispatched to implement a ticket, the assignment packet includes an explicit branch name and working directory. Before the worker can claim the ticket, the executor must verify the w...
- ref: `.ticket/tickets/b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=8d83f9f6-b36e-42bd-ac42-3a6d073873a7 slug=planned/context-engine digest=1fdf34bc4051 -->
#### [8d83f9f6] [sandbox-v1][impl] Firecracker control plane and repo-local microVM foundation
- priority: `high`
- summary: Tokio multi-thread orchestration core.
- ref: `.ticket/tickets/8d83f9f6-b36e-42bd-ac42-3a6d073873a7/ticket.toml`


### Component: context-read

<!-- ticket-index:entry id=978ce8a5-3936-467b-aca8-822eeecd1eb0 slug=planned/context-read digest=61de3ee79f5c -->
#### [978ce8a5] Plan: Expansion loop redesign — cursor-advancing decomposition
- summary: This session did **not** implement the full `context-read` expansion-loop redesign, but it cleared the public API and test-wrapper layers that were masking the deeper engine issue.
- ref: `context-stack/.ticket/tickets/978ce8a5-3936-467b-aca8-822eeecd1eb0/ticket.toml`

<!-- ticket-index:entry id=f95969ba-c797-42d2-b6bc-9265a5fb4cf0 slug=planned/context-read digest=749c3da7e232 -->
#### [f95969ba] Plan: context-read UX improvement — parent plan (multi-phase)
- summary: tags: `#plan` `#context-read` `#context-api` `#context-cli` `#ux` `#algorithm` `#read` `#insert` `#search` `#multi-phase`
- ref: `.ticket/tickets/f95969ba-c797-42d2-b6bc-9265a5fb4cf0/ticket.toml`

<!-- ticket-index:entry id=bfe43d0d-2870-4146-b651-1464a55ec7aa slug=planned/context-read digest=fea77746b818 -->
#### [bfe43d0d] [Bug] context-read largest-overlap incremental join misses expected decompositions
- summary: `read()` should build larger tokens by repeatedly finding the largest next overlap and joining that overlap into the running root. The current failing repeat and rotating-overlap cases show that this...
- ref: `.ticket/tickets/bfe43d0d-2870-4146-b651-1464a55ec7aa/ticket.toml`

<!-- ticket-index:entry id=6e61bef1-6037-42c8-abc1-d79a3f9367f7 slug=planned/context-read digest=2b3ddcf253a5 -->
#### [6e61bef1] [context-insert] Unify overlap bundling under one structural formula
- priority: `high`
- summary: `context-insert::bundle_overlap` still branches on `self_overlap` and `overlap_is_shared_then_t1`, and it falls back to raw `insert_patterns`. The formula is branchy and hard to reason about.
- ref: `.ticket/tickets/6e61bef1-6037-42c8-abc1-d79a3f9367f7/ticket.toml`

<!-- ticket-index:entry id=9f8d842e-3c7c-4470-b840-dd69b92380b5 slug=planned/context-read digest=a1357ee56410 -->
#### [9f8d842e] [context-read] Replace root surgery with structural block materialization
- priority: `high`
- summary: `RootManager` currently grows semantic roots through `flat_root`, `wrap_root`, `replace_last_child`, and `try_extend_tail_with`. That is mutation-heavy shortcut logic, not the structural block-to-blo...
- ref: `.ticket/tickets/9f8d842e-3c7c-4470-b840-dd69b92380b5/ticket.toml`

<!-- ticket-index:entry id=529feeaa-822c-443b-a6a2-f0ae67edc225 slug=planned/context-read digest=76cfeb519e40 -->
#### [529feeaa] [context-read][tests] Layer read tests after lower-crate primitives
- priority: `high`
- summary: `context-read` tests mostly assert whole decomposition families after long worked traces. Lower crates use smaller fixture-based tests to pin one primitive at a time. The current read suite turns eve...
- ref: `.ticket/tickets/529feeaa-822c-443b-a6a2-f0ae67edc225/ticket.toml`


### Component: context-stack

<!-- ticket-index:entry id=aaa810f0-cc14-4226-b7d0-d81a38f856e7 slug=planned/context-stack digest=e17075b265ac -->
#### [aaa810f0] Decide post-import ownership cleanup for context-stack tools
- priority: `medium`
- summary: After the tool-history import, the original tool source trees still exist in `context-engine`. Until ownership cleanup is decided and executed, it is ambiguous which repository is the source of truth...
- ref: `.ticket/tickets/aaa810f0-cc14-4226-b7d0-d81a38f856e7/ticket.toml`


### Component: documentation-tooling

<!-- ticket-index:entry id=5d320d7e-f974-4d52-9e25-8265bf7a42cf slug=planned/documentation-tooling digest=4b79f3dab91e -->
#### [5d320d7e] Design reproducible Docker validation for install and deinstall docs
- priority: `high`
- summary: User-facing installation documentation is not validated continuously from a clean environment. The current repo has install instructions for the CLI tools in `memory-api/README.md`, but there is no D...
- ref: `.ticket/tickets/5d320d7e-f974-4d52-9e25-8265bf7a42cf/ticket.toml`

<!-- ticket-index:entry id=e0c136dd-8bdf-40f6-a39c-29f9e88167d6 slug=planned/documentation-tooling digest=6b8d379ef1e2 -->
#### [e0c136dd] Gate install and deinstall documentation continuously in CI
- priority: `high`
- summary: A local Docker harness is not sufficient on its own. The user-facing installation documentation needs continuous validation in CI so documentation drift or broken installation steps are caught before...
- ref: `.ticket/tickets/e0c136dd-8bdf-40f6-a39c-29f9e88167d6/ticket.toml`

<!-- ticket-index:entry id=0ffac34a-4e33-426c-8eef-ef6482ab3bde slug=planned/documentation-tooling digest=0c6065416b4b -->
#### [0ffac34a] Implement Docker harness for documented install and deinstall flows
- priority: `high`
- summary: After the Docker validation strategy is defined, the repository still needs a runnable harness that executes the documented installation and deinstallation steps in clean containers and proves that t...
- ref: `.ticket/tickets/0ffac34a-4e33-426c-8eef-ef6482ab3bde/ticket.toml`


### Component: history

<!-- ticket-index:entry id=f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16 slug=planned/history digest=9a9afc8aa6a4 -->
#### [f5d7e9a2] [bootstrap][T6] verify merge and completion linkage with assignment chain
- summary: After validation passes (T4), the ticket advances through release gates toward merge. The merge record must be fully traceable: it must include the worker assignment_id, the validator assignment_id, ...
- ref: `memory-api/.ticket/tickets/f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16/ticket.toml`


### Component: lease

<!-- ticket-index:entry id=a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61 slug=planned/lease digest=aee41483bed9 -->
#### [a8d6c1d2] [bootstrap][T1] startup and auth bootstrap for host executor
- summary: The host executor is a Rust service process (`ticket host-executor`) that workers authenticate against to claim tickets, run inference, and report progress. Per the Phase 1.5 design, the executor can...
- ref: `memory-api/.ticket/tickets/a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61/ticket.toml`

<!-- ticket-index:entry id=c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83 slug=planned/lease digest=21a51755d43e -->
#### [c2a4b6d8] [bootstrap][T3] validate ticket lifecycle happy path under executor
- summary: Once a worker is authenticated (T1) and context-verified (T2), it proceeds through the core ticket mutation lifecycle: claim → implement → attach evidence → unclaim. Every event in this lifecycle mus...
- ref: `memory-api/.ticket/tickets/c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83/ticket.toml`

<!-- ticket-index:entry id=d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94 slug=planned/lease digest=4ba32251c521 -->
#### [d3b5c7e9] [bootstrap][T4] implement validator handoff with separation-of-duties
- summary: After a worker completes implementation (T3), the ticket moves to `validating` state. A second agent — the **validator** — is dispatched by the coordinator with a different identity to independently ...
- ref: `memory-api/.ticket/tickets/d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=b03be2d5-5293-4dc7-ad11-cca2dbf32c8b slug=planned/memory-api digest=2396f6daa8cc -->
#### [b03be2d5] [spec][P5] Cross-entity edges — spec depends_on ticket, ticket implements spec
- priority: `medium`
- summary: Extend memory-api's edge system to support edges between entities of different types (spec ↔ ticket). Currently edges are within a single entity store; this enables cross-store relationships.
- ref: `memory-api/.ticket/tickets/b03be2d5-5293-4dc7-ad11-cca2dbf32c8b/ticket.toml`


### Component: memory-fixtures

<!-- ticket-index:entry id=f5875ff3-7c37-4ac2-8ae6-d68aff240bf4 slug=planned/memory-fixtures digest=faf5af441394 -->
#### [f5875ff3] [memory-fixtures][ticket-api][perf] Expand fixture for slow move/health scenarios
- priority: `high`
- summary: Extend `memory-fixtures` so ticket move and health performance tests can materialize representative slow scenarios without bespoke setup in each test.
- ref: `memory-api/.ticket/tickets/f5875ff3-7c37-4ac2-8ae6-d68aff240bf4/ticket.toml`


### Component: rule-api

<!-- ticket-index:entry id=d0ccdb06-db44-464f-846e-9d58c1320fd0 slug=planned/rule-api digest=2377416c63d1 -->
#### [d0ccdb06] Complete memory-api rule-api specs and test links
- priority: `high`
- summary: Nested rule work and repo-local README generation need a committed spec set in `memory-api/.spec` with maintained code references and validation hooks. Initial planning specs now exist, but the imple...
- ref: `memory-api/.ticket/tickets/d0ccdb06-db44-464f-846e-9d58c1320fd0/ticket.toml`

<!-- ticket-index:entry id=7cffac6b-7dca-4134-8c0f-7dbedcd0cbbd slug=planned/rule-api digest=8b572748f490 -->
#### [7cffac6b] Generate memory-api README from repo-local rules
- priority: `high`
- summary: `memory-api` does not yet have a repo-local `.rule` workspace or a local `rule-targets.yaml`, so its `README.md` remains a manually maintained file instead of a generated target owned by the repo tha...
- ref: `memory-api/.ticket/tickets/7cffac6b-7dca-4134-8c0f-7dbedcd0cbbd/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=00798e96-3d82-436e-963c-af347e76ede0 slug=planned/spec-api digest=300d54714370 -->
#### [00798e96] [spec][P3] Spec creation — planned feature specs with acceptance criteria templates
- priority: `medium`
- summary: Create specification files for features that are planned but not yet implemented. These specs serve as the design document and acceptance criteria definition.
- ref: `memory-api/.ticket/tickets/00798e96-3d82-436e-963c-af347e76ede0/ticket.toml`

<!-- ticket-index:entry id=ffc578f7-8a18-4536-9a8c-023d42b98d3e slug=planned/spec-api digest=b06ddedc8092 -->
#### [ffc578f7] [spec][P3] Spec-to-code sync — detect and update references after file moves
- priority: `medium`
- summary: Detect when implementation files are moved/renamed and automatically update spec code references.
- ref: `memory-api/.ticket/tickets/ffc578f7-8a18-4536-9a8c-023d42b98d3e/ticket.toml`

<!-- ticket-index:entry id=80e25216-7ba9-4fd9-bc80-3311f1d2a604 slug=planned/spec-api digest=2b35f6233188 -->
#### [80e25216] [spec][P3] Spec-to-code sync — update specs after implementation changes
- priority: `high`
- summary: Detect when implementation code changes and update spec code references and feature status accordingly.
- ref: `memory-api/.ticket/tickets/80e25216-7ba9-4fd9-bc80-3311f1d2a604/ticket.toml`

<!-- ticket-index:entry id=c4c9e9d4-8831-4135-98a7-0b64031ffe52 slug=planned/spec-api digest=e3730f9cb8db -->
#### [c4c9e9d4] [spec][P4] Feature tracking — record feature completeness and bug status per spec
- priority: `medium`
- summary: Track per-spec feature completeness: which features are implemented, planned, blocked, or have known bugs.
- ref: `memory-api/.ticket/tickets/c4c9e9d4-8831-4135-98a7-0b64031ffe52/ticket.toml`

<!-- ticket-index:entry id=6c00ef55-1531-4494-9bf2-00184740a3b0 slug=planned/spec-api digest=06e652d0a02b -->
#### [6c00ef55] [spec][P4] Skill generation — master index and cross-references
- priority: `medium`
- summary: Generate a master `docs/skills/INDEX.md` that serves as the entry point for all generated skill files, with coverage statistics and cross-references.
- ref: `memory-api/.ticket/tickets/6c00ef55-1531-4494-9bf2-00184740a3b0/ticket.toml`

<!-- ticket-index:entry id=eddf5d2e-e1b6-4ec9-b88f-d50bd192b194 slug=planned/spec-api digest=d9127ad009d7 -->
#### [eddf5d2e] [spec][P4] Skill generation — per-crate and per-domain SKILL.md files from spec data
- priority: `high`
- summary: Build a skill file generation engine that reads spec data from the SpecStore and produces structured SKILL.md files for AI coding agents.
- ref: `memory-api/.ticket/tickets/eddf5d2e-e1b6-4ec9-b88f-d50bd192b194/ticket.toml`

<!-- ticket-index:entry id=ad5fb72b-548c-4215-88a6-eacde7a42d4d slug=planned/spec-api digest=d47faa01a575 -->
#### [ad5fb72b] [spec][P4] Spec health check — completeness, staleness, broken references, coverage
- priority: `medium`
- summary: Validate spec store integrity including completeness, staleness, broken references, and coverage metrics.
- ref: `memory-api/.ticket/tickets/ad5fb72b-548c-4215-88a6-eacde7a42d4d/ticket.toml`

<!-- ticket-index:entry id=45671e0e-24d6-4f51-b216-07e80f2ff302 slug=planned/spec-api digest=137bcc26d87d -->
#### [45671e0e] [spec][P4] Test generation — Rust test stubs and test matrix from spec acceptance criteria
- priority: `medium`
- summary: Generate Rust test stubs for uncovered spec features and a test matrix checklist linking existing tests to spec acceptance criteria.
- ref: `memory-api/.ticket/tickets/45671e0e-24d6-4f51-b216-07e80f2ff302/ticket.toml`

<!-- ticket-index:entry id=f00291a3-bd61-469e-a737-c44cb3911e3b slug=planned/spec-api digest=effdbf757940 -->
#### [f00291a3] [spec][P5] Ticket integration — link specs to tickets, track refinement/validation/bugfix work
- priority: `medium`
- summary: Link specs to tickets bidirectionally. When a ticket implements a spec feature, or a bug is found against a spec, the relationship is tracked.
- ref: `memory-api/.ticket/tickets/f00291a3-bd61-469e-a737-c44cb3911e3b/ticket.toml`

<!-- ticket-index:entry id=7802faa3-5d79-4ec9-9f26-143bca62149c slug=planned/spec-api digest=88235cd759f6 -->
#### [7802faa3] [spec][P6] Hierarchical DAG — parent-child spec relationships, no duplication
- priority: `medium`
- summary: Implement parent-child spec relationships as a DAG (no duplication of specification content). Each spec declares its parent; the system builds a tree with cross-references via edges.
- ref: `memory-api/.ticket/tickets/7802faa3-5d79-4ec9-9f26-143bca62149c/ticket.toml`

<!-- ticket-index:entry id=d72d5114-2521-4d02-9ca1-7f0bee8d470d slug=planned/spec-api digest=c958dc7f1acf -->
#### [d72d5114] [spec][P6] Spec search — full-text search with field predicates
- priority: `medium`
- summary: Full-text search across all specs using Tantivy, with field predicates matching the ticket search pattern.
- ref: `memory-api/.ticket/tickets/d72d5114-2521-4d02-9ca1-7f0bee8d470d/ticket.toml`

<!-- ticket-index:entry id=a7b2a89c-6562-468c-a129-ad4883e5cf6e slug=planned/spec-api digest=9f5cf5ee0727 -->
#### [a7b2a89c] [spec][P6] Table of contents — auto-generated TOC index of all specs
- priority: `medium`
- summary: Auto-generate a table of contents index showing all specs organized by component and hierarchy.
- ref: `memory-api/.ticket/tickets/a7b2a89c-6562-468c-a129-ad4883e5cf6e/ticket.toml`

<!-- ticket-index:entry id=13a57a83-df99-4031-87e2-844772758ebb slug=planned/spec-api digest=6f19b5c28512 -->
#### [13a57a83] [spec][P8] Bootstrap: write spec files for the spec system itself
- priority: `high`
- summary: Author the canonical specification database covering the spec-system crates
- ref: `memory-api/.ticket/tickets/13a57a83-df99-4031-87e2-844772758ebb/ticket.toml`

<!-- ticket-index:entry id=9242a906-cba9-43a4-b45e-942465379a7b slug=planned/spec-api digest=2a878ea7fcb8 -->
#### [9242a906] [spec][P8] Bootstrap: write spec files for ticket-api interfaces
- priority: `high`
- summary: Write comprehensive spec files documenting the ticket-api crate's full API surface, storage layer, schema system, and edge system.
- ref: `memory-api/.ticket/tickets/9242a906-cba9-43a4-b45e-942465379a7b/ticket.toml`

<!-- ticket-index:entry id=c617cee6-3182-47db-a7cf-15cccbc02b6d slug=planned/spec-api digest=7b55f590b2f2 -->
#### [c617cee6] [spec][P8] Generate initial skill files for all ticket system tools
- priority: `high`
- summary: Use the skill generation engine to produce the first set of SKILL.md files covering all ticket system tools.
- ref: `memory-api/.ticket/tickets/c617cee6-3182-47db-a7cf-15cccbc02b6d/ticket.toml`


### Component: spec-cli

<!-- ticket-index:entry id=f2c1ebc2-aaee-4a93-895b-56284b549840 slug=planned/spec-cli digest=3a1a548acdd7 -->
#### [f2c1ebc2] [spec][P8] Bootstrap: write spec files for ticket-cli interface
- priority: `medium`
- summary: Write specs for the ticket-cli crate documenting the CLI command surface, argument parsing, output formatting, and batch execution. Covers all commands: create, get, update, delete, list, search, lin...
- ref: `memory-api/.ticket/tickets/f2c1ebc2-aaee-4a93-895b-56284b549840/ticket.toml`


### Component: spec-editor

<!-- ticket-index:entry id=618f6240-3f08-466f-857e-1c8c52d032d8 slug=planned/spec-editor digest=756339e99d67 -->
#### [618f6240] [spec-editor] Interactive spec authoring — Dioxus SPA with body/section/coderef editing
- priority: `high`
- summary: A single-process, GPU-accelerated web application for **authoring and editing** specs.
- ref: `.ticket/tickets/618f6240-3f08-466f-857e-1c8c52d032d8/ticket.toml`


### Component: spec-http

<!-- ticket-index:entry id=1b19e979-f2a0-4803-bc97-15ffd8f7ab72 slug=planned/spec-http digest=c4b812db4188 -->
#### [1b19e979] [spec][P8] Bootstrap: write spec files for ticket-http interface
- priority: `medium`
- summary: Write specs for the ticket-http crate documenting all HTTP endpoints, request/response formats, middleware, SSE streaming, auth, and error handling.
- ref: `memory-api/.ticket/tickets/1b19e979-f2a0-4803-bc97-15ffd8f7ab72/ticket.toml`


### Component: spec-mcp

<!-- ticket-index:entry id=10a26c64-402b-45e2-8333-2c471d0c0170 slug=planned/spec-mcp digest=9b3b0af0c852 -->
#### [10a26c64] [spec][P8] Bootstrap: write spec files for ticket-mcp interface
- priority: `medium`
- summary: Write specs for the ticket-mcp crate documenting all MCP tools, their input schemas, output formats, and error handling.
- ref: `memory-api/.ticket/tickets/10a26c64-402b-45e2-8333-2c471d0c0170/ticket.toml`


### Component: spec-vscode

<!-- ticket-index:entry id=7f0a4dac-37b0-44c8-ba72-4ea0aaabb374 slug=planned/spec-vscode digest=0bdc2541a59a -->
#### [7f0a4dac] [spec][P7] spec-vscode — VS Code extension for browsing specs with rich HTML viewer
- priority: `low`
- summary: VS Code extension for browsing specification files with rich HTML rendering, navigation links, and code reference jump-to-source.
- ref: `memory-api/.ticket/tickets/7f0a4dac-37b0-44c8-ba72-4ea0aaabb374/ticket.toml`

<!-- ticket-index:entry id=321f4ec7-03df-4e14-9734-a6af76ace55f slug=planned/spec-vscode digest=15ac4733fed1 -->
#### [321f4ec7] [spec][P8] Bootstrap: write spec files for ticket-vscode interface
- priority: `low`
- summary: Write specs for the ticket-vscode extension documenting the tree view provider, webview panel, API client, and VS Code extension lifecycle.
- ref: `memory-api/.ticket/tickets/321f4ec7-03df-4e14-9734-a6af76ace55f/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=6ddfb633-6e04-49ef-a464-f38bb13f6051 slug=planned/ticket-api digest=a30609f2141d -->
#### [6ddfb633] [ticket-api][perf] Add aggressive e2e timing coverage for move and health
- priority: `high`
- summary: Add aggressive end-to-end tests that time and stress `ticket move` and `ticket health` using representative fixtures.
- ref: `memory-api/.ticket/tickets/6ddfb633-6e04-49ef-a464-f38bb13f6051/ticket.toml`

<!-- ticket-index:entry id=cdae25a0-f7a1-4266-a341-a65a0a9e6325 slug=planned/ticket-api digest=2c87514876da -->
#### [cdae25a0] [ticket-api][perf] Benchmark move and health on representative fixtures
- priority: `high`
- summary: Benchmark representative `ticket move` and `ticket health` operations so slow slices are measurable with Criterion instead of anecdotal CLI timing.
- ref: `memory-api/.ticket/tickets/cdae25a0-f7a1-4266-a341-a65a0a9e6325/ticket.toml`

<!-- ticket-index:entry id=49bbe3ae-a80c-479f-98f2-500643706ce6 slug=planned/ticket-api digest=8a9927f71786 -->
#### [49bbe3ae] [ticket-api][perf] Characterize slow ticket move and health end-to-end
- priority: `high`
- summary: Drive a focused performance-characterization track for `ticket move` and `ticket health` so we can reproduce slow operations from the workspace-cleanup work and measure them before optimization.
- ref: `memory-api/.ticket/tickets/49bbe3ae-a80c-479f-98f2-500643706ce6/ticket.toml`

<!-- ticket-index:entry id=f5dba169-b153-4d6a-ae1b-7620f317309e slug=planned/ticket-api digest=b4fa4127c53a -->
#### [f5dba169] [ticket-api][perf] Provoke slow-path and failure-path behavior in move/health tests
- priority: `high`
- summary: Make the move and health test surface deliberately hostile so slow or pathological behavior shows up early.
- ref: `memory-api/.ticket/tickets/f5dba169-b153-4d6a-ae1b-7620f317309e/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=181ed793-481d-4d46-b059-0eda891365d7 slug=planned/ticket-http digest=cba6b0ff8de5 -->
#### [181ed793] [ticket-http] Add /api/next endpoint for best-next ranking
- priority: `high`
- summary: There is no dedicated `GET /api/next` route in the current ticket HTTP router. HTTP consumers that want ranked best-next results have to reconstruct them manually by combining `GET /api/tickets` with...
- ref: `memory-api/.ticket/tickets/181ed793-481d-4d46-b059-0eda891365d7/ticket.toml`

<!-- ticket-index:entry id=5012f293-e871-4e4a-af40-c27b3bd967fb slug=planned/ticket-http digest=21eb1fc01715 -->
#### [5012f293] [ticket-http][ticket-api][ticket-viewer] Track: child-workspace ticket reference rollout
- priority: `high`
- summary: The child-workspace ticket-reference rollout is now split into three well-scoped tickets, but there is no parent tracker that captures the full implementation sequence, the shared goal, or the cross-...
- ref: `memory-api/.ticket/tickets/5012f293-e871-4e4a-af40-c27b3bd967fb/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=10c94251-1c0c-4542-a282-ea3d75a205b5 slug=planned/ticket-viewer digest=12fc30732a23 -->
#### [10c94251] [ticket-viewer][viewer-api] Graph focus and 2D presentation follow-up
- priority: `high`
- summary: Track the next graph-viewer interaction and presentation upgrade for ticket-viewer: property-based node rendering, stronger selection semantics, panel-aware framing, an optional fixed 2D camera mode,...
- ref: `.ticket/tickets/10c94251-1c0c-4542-a282-ea3d75a205b5/ticket.toml`

<!-- ticket-index:entry id=929bc26a-5296-4d64-b1b2-2ec580c0659c slug=planned/ticket-viewer digest=313dec24fb2b -->
#### [929bc26a] [ticket-viewer][viewer-api] Make graph framing panel-aware and keep nodes behind UI panels
- priority: `high`
- summary: Keep graph content behind sidebar and viewport panels while using those panel bounds to bias graph framing, focus centering, and node placement.
- ref: `.ticket/tickets/929bc26a-5296-4d64-b1b2-2ec580c0659c/ticket.toml`

<!-- ticket-index:entry id=923c866a-fecd-4ddb-8be0-00ca4cb22af9 slug=planned/ticket-viewer digest=005ed68fe6dd -->
#### [923c866a] [ticket-viewer][viewer-api] Refine graph selection focus and outside-click deselection
- priority: `high`
- summary: Refine graph node selection and focus falloff so the selected ticket stays emphasized, linked context remains visible, and clicking outside the graph clears selection.
- ref: `.ticket/tickets/923c866a-fecd-4ddb-8be0-00ca4cb22af9/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=6d07d610-75c1-448a-afd5-6ae15098ca21 slug=planned/ticket-vscode digest=320a4dcd0afa -->
#### [6d07d610] [ticket-vscode] Rust/WASM port track
- priority: `high`
- summary: Port `memory-api/tools/ticket-vscode` from a TypeScript-heavy implementation to a Rust/WASM-backed VS Code extension architecture.
- ref: `memory-api/.ticket/tickets/6d07d610-75c1-448a-afd5-6ae15098ca21/ticket.toml`


### Component: tooling

<!-- ticket-index:entry id=c7becdaa-6939-4ab9-a8a5-29fbf8921584 slug=planned/tooling digest=ff41606b5a32 -->
#### [c7becdaa] [install-ctl] Manage registry tools and services across install lifecycle
- priority: `high`
- summary: Make install-ctl consume the canonical registry for list, install, uninstall, start, restart, and stop actions declared by entries. Preserve dry-run and reject unsupported actions. This work follows ...
- ref: `.ticket/tickets/c7becdaa-6939-4ab9-a8a5-29fbf8921584/ticket.toml`

<!-- ticket-index:entry id=495125df-257d-4a56-84cb-784ea822a1d7 slug=planned/tooling digest=8fb2a522ecab -->
#### [495125df] [tooling] Canonical executable and hook registry for command execution
- priority: `high`
- summary: Create the canonical inventory and generated Markdown catalog for all repository executables and hooks. The registry becomes the authoritative repository command surface, while runtime agent tool sch...
- ref: `.ticket/tickets/495125df-257d-4a56-84cb-784ea822a1d7/ticket.toml`

<!-- ticket-index:entry id=15234799-d540-4e49-9bf2-4514b768cb79 slug=planned/tooling digest=ede9e67565d4 -->
#### [15234799] [tooling] Register repository hooks in command registry
- priority: `medium`
- summary: Add repository hook entries to the canonical registry and generated catalog. Record source, trigger, ownership, safety, and supported install-ctl actions. Third-party hook-manager support is excluded.
- ref: `.ticket/tickets/15234799-d540-4e49-9bf2-4514b768cb79/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=4460cad4-c137-45c4-893e-6e340e16bbe7 slug=planned/unspecified digest=202edf7a1e54 -->
#### [4460cad4] Demo: structured parts render check
- summary: Demo ticket used to browser-validate ticket-viewer's parts/frozen/amendment/refs rendering.
- ref: `memory-api/.ticket/tickets/4460cad4-c137-45c4-893e-6e340e16bbe7/ticket.toml`

<!-- ticket-index:entry id=41593795-34a6-470d-8917-f9c789a37332 slug=planned/unspecified digest=cbf3544858cb -->
#### [41593795] E2E structured-parts fixture
- ref: `.ticket/tickets/41593795-34a6-470d-8917-f9c789a37332/ticket.toml`

<!-- ticket-index:entry id=45253917-2ab2-440b-9c2a-b08c90edd0b3 slug=planned/unspecified digest=967793411785 -->
#### [45253917] E2E structured-parts fixture
- ref: `.ticket/tickets/45253917-2ab2-440b-9c2a-b08c90edd0b3/ticket.toml`

<!-- ticket-index:entry id=5fc9e6df-d030-4d3e-b5a0-84665840a3ab slug=planned/unspecified digest=c04f4687e2d9 -->
#### [5fc9e6df] E2E structured-parts fixture
- ref: `.ticket/tickets/5fc9e6df-d030-4d3e-b5a0-84665840a3ab/ticket.toml`

<!-- ticket-index:entry id=5feb57ed-bbe1-4423-b6f0-2d32d0e95443 slug=planned/unspecified digest=d0370461dbeb -->
#### [5feb57ed] E2E structured-parts fixture
- ref: `.ticket/tickets/5feb57ed-bbe1-4423-b6f0-2d32d0e95443/ticket.toml`

<!-- ticket-index:entry id=65df18bc-2d9c-41ec-aa00-226928bcb6c6 slug=planned/unspecified digest=9772f8ab2841 -->
#### [65df18bc] E2E structured-parts fixture
- ref: `.ticket/tickets/65df18bc-2d9c-41ec-aa00-226928bcb6c6/ticket.toml`

<!-- ticket-index:entry id=90234182-8a84-48ec-8b1b-d0e71e509f19 slug=planned/unspecified digest=3d231f898442 -->
#### [90234182] E2E structured-parts fixture
- ref: `.ticket/tickets/90234182-8a84-48ec-8b1b-d0e71e509f19/ticket.toml`

<!-- ticket-index:entry id=d09329f4-fb1b-497e-abe6-e0ca64dc8db0 slug=planned/unspecified digest=20e6e884cb1f -->
#### [d09329f4] E2E structured-parts fixture
- ref: `.ticket/tickets/d09329f4-fb1b-497e-abe6-e0ca64dc8db0/ticket.toml`

<!-- ticket-index:entry id=dcee2f3b-d20c-4069-bcfa-7701306642a3 slug=planned/unspecified digest=f6c22fb378bb -->
#### [dcee2f3b] E2E structured-parts fixture
- ref: `.ticket/tickets/dcee2f3b-d20c-4069-bcfa-7701306642a3/ticket.toml`

<!-- ticket-index:entry id=edc7bc46-cac2-49c5-9d72-e3f78ea1f52e slug=planned/unspecified digest=06a06707ebce -->
#### [edc7bc46] E2E structured-parts fixture
- ref: `.ticket/tickets/edc7bc46-cac2-49c5-9d72-e3f78ea1f52e/ticket.toml`

<!-- ticket-index:entry id=f5abb5b1-aaac-4eda-99b9-0c9872a346f1 slug=planned/unspecified digest=8290f9a20d0f -->
#### [f5abb5b1] E2E structured-parts fixture
- ref: `.ticket/tickets/f5abb5b1-aaac-4eda-99b9-0c9872a346f1/ticket.toml`

<!-- ticket-index:entry id=9ac0a02b-965f-45f3-b8c9-97a063e3bc55 slug=planned/unspecified digest=87d7f64dee10 -->
#### [9ac0a02b] Epic: Viewer Component Port -- framework migration and API surface (Preact to Dioxus/Leptos)
- ref: `.ticket/tickets/9ac0a02b-965f-45f3-b8c9-97a063e3bc55/ticket.toml`

<!-- ticket-index:entry id=111510f4-c74b-4819-800b-d68ab013a73c slug=planned/unspecified digest=503984b608d6 -->
#### [111510f4] Fix graph reactivity: ticket state changes don't update graph nodes
- summary: When a ticket state is changed in the details panel, the graph nodes don't update their visual representation (color, label, etc.). The graph only listens to `edge.*` SSE events, not `ticket.*` event...
- ref: `.ticket/tickets/111510f4-c74b-4819-800b-d68ab013a73c/ticket.toml`

<!-- ticket-index:entry id=322a4737-9fae-4804-8053-6ea1c85205da slug=planned/unspecified digest=809f9dce5564 -->
#### [322a4737] [Epic] PDF domain crate: extract, edit, create, merge PDFs via MCP + skill
- summary: Add a `pdf` domain to the repository so agents can work with PDF files through
- ref: `.ticket/tickets/322a4737-9fae-4804-8053-6ea1c85205da/ticket.toml`

<!-- ticket-index:entry id=fc94716e-58f2-47fc-8750-3d96efbd612d slug=planned/unspecified digest=142732d69f08 -->
#### [fc94716e] [PDF][T0] Verification spike: confirm pure-Rust PDF crate versions, licenses, and capability coverage
- summary: A dedicated research pass with real network access has **already discharged the
- ref: `.ticket/tickets/fc94716e-58f2-47fc-8750-3d96efbd612d/ticket.toml`

<!-- ticket-index:entry id=7f7ef435-0504-40cf-8d6c-dd96477a0223 slug=planned/unspecified digest=5e4957fe772d -->
#### [7f7ef435] [content-materialization][workspace] G-E: De-submodularize — real dependency imports instead of nested submodules (gated)
- summary: Replace nested git submodule links (context-engine → memory-viewers → memory-api/viewer-api) with real dependency-level imports / install-path deps on remote releases. Treat context-engine as a stand...
- ref: `memory-api/.ticket/tickets/7f7ef435-0504-40cf-8d6c-dd96477a0223/ticket.toml`

<!-- ticket-index:entry id=b1e9e744-aeac-474a-91d9-07e3a362dc76 slug=planned/unspecified digest=1bf6c3815661 -->
#### [b1e9e744] [feedback-api] Feedback inbox, metadata indexing, and deep search
- priority: `high`
- summary: Plan a feedback store that ingests human and privileged-agent feedback events, normalizes metadata, and supports deep search and reconciliation at scale.
- ref: `memory-api/.ticket/tickets/b1e9e744-aeac-474a-91d9-07e3a362dc76/ticket.toml`

<!-- ticket-index:entry id=b7b84c10-8dc5-4087-87ad-6fe27ebbcd45 slug=planned/unspecified digest=55b9e0671b20 -->
#### [b7b84c10] [feedback-api] High-scale search, clustering, and reconciliation workflows
- priority: `high`
- summary: Plan and implement deep query/search capabilities and operator reconciliation flows for large feedback corpora, including dedupe, sentiment facets, and routing.
- ref: `memory-api/.ticket/tickets/b7b84c10-8dc5-4087-87ad-6fe27ebbcd45/ticket.toml`

<!-- ticket-index:entry id=4f86d3d2-2b2a-4c9d-9d46-5f2a437f91b7 slug=planned/unspecified digest=f460216fdf10 -->
#### [4f86d3d2] [feedback-api] Privileged feedback governance and abuse-boundary enforcement
- priority: `high`
- summary: Define policy and enforcement boundaries for privileged-agent feedback so trust, attribution, and abuse controls are explicit.
- ref: `memory-api/.ticket/tickets/4f86d3d2-2b2a-4c9d-9d46-5f2a437f91b7/ticket.toml`

<!-- ticket-index:entry id=c2d6a14a-98b7-4f98-9f62-90a5ccf06d9e slug=planned/unspecified digest=997a577e9b6f -->
#### [c2d6a14a] [feedback-api] Retention, redaction, and privacy incident controls
- priority: `high`
- summary: Define enforceable retention and redaction behavior for feedback and interview-derived signals, including privacy incident handling paths.
- ref: `memory-api/.ticket/tickets/c2d6a14a-98b7-4f98-9f62-90a5ccf06d9e/ticket.toml`

<!-- ticket-index:entry id=3a1ec9f8-15ea-43f2-b6d3-89b88cbdcb17 slug=planned/unspecified digest=1b0f0af3da1c -->
#### [3a1ec9f8] [feedback-api] Search latency and index growth SLOs
- priority: `high`
- summary: Define measurable performance SLOs for feedback deep-search operations and index growth behavior under high event volume.
- ref: `memory-api/.ticket/tickets/3a1ec9f8-15ea-43f2-b6d3-89b88cbdcb17/ticket.toml`

<!-- ticket-index:entry id=60222b57-095d-4c9e-b83a-70c3dd8690ba slug=planned/unspecified digest=d012ae542701 -->
#### [60222b57] [presentation] Custom repo theme pack: design tokens, curated presets, preset descriptors
- summary: Parent epic: `0ee95228`. Spec: `2ccde9ee`. Depends on `89b0c64a` (Phase 1 on a stock theme).
- ref: `.ticket/tickets/60222b57-095d-4c9e-b83a-70c3dd8690ba/ticket.toml`

<!-- ticket-index:entry id=e01dd058-a539-4620-87b2-0a4895114ca2 slug=planned/unspecified digest=9b824df0cef9 -->
#### [e01dd058] [presentation] Embed shared repo graph component as standalone WASM inside slides
- summary: Parent epic: `0ee95228`. Spec: `2ccde9ee`. Depends on `89b0c64a` (toolchain with wasm plugins).
- ref: `.ticket/tickets/e01dd058-a539-4620-87b2-0a4895114ca2/ticket.toml`

<!-- ticket-index:entry id=0ee95228-475d-4706-a108-fd208f7c4098 slug=planned/unspecified digest=1db404df34d8 -->
#### [0ee95228] [presentation] Epic: script-to-deck presentation system (Slidev + presentation domain + viewer)
- summary: Governing spec: `presentation-system` (`2ccde9ee-85ac-4c87-9601-f6099f5be01c`).
- ref: `.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml`

<!-- ticket-index:entry id=969ffba0-6bff-4a58-9d74-18368ac87875 slug=planned/unspecified digest=31751630d36b -->
#### [969ffba0] [presentation] First real deck: workflow-tools suite introduction and overview
- summary: Parent epic: `0ee95228`. Spec: `2ccde9ee`. This is the system's real-use validation.
- ref: `.ticket/tickets/969ffba0-6bff-4a58-9d74-18368ac87875/ticket.toml`

<!-- ticket-index:entry id=134b953b-1294-4d27-9479-1662a2f5e250 slug=planned/unspecified digest=8f44b0c136cc -->
#### [134b953b] [presentation] Install skills, author presentation-workflow skill, agent mode + prompt, retire legacy HTML agent
- summary: Parent epic: `0ee95228`. Spec: `2ccde9ee`.
- ref: `.ticket/tickets/134b953b-1294-4d27-9479-1662a2f5e250/ticket.toml`

<!-- ticket-index:entry id=89b0c64a-b573-4f7b-b692-fa3d383e386c slug=planned/unspecified digest=f0a4a9dccca0 -->
#### [89b0c64a] [presentation] Phase 1: Slidev toolchain on stock theme, sample deck, Playwright verification
- summary: Parent epic: `0ee95228`. Spec: `2ccde9ee`.
- ref: `.ticket/tickets/89b0c64a-b573-4f7b-b692-fa3d383e386c/ticket.toml`

<!-- ticket-index:entry id=3cdcaf3b-d958-44f3-afb2-b17be3484419 slug=planned/unspecified digest=0bf3a4644a75 -->
#### [3cdcaf3b] [presentation] Phase 2: presentation-api + presentation facade crate + .presentation store schema
- summary: Parent epic: `0ee95228` — [presentation] Epic: script-to-deck presentation system.
- ref: `.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml`

<!-- ticket-index:entry id=345528ff-a9ca-4d0d-ba94-a5365a14c54e slug=planned/unspecified digest=9fd89fe8fa12 -->
#### [345528ff] [presentation] presentation-viewer: Rust server + Dioxus deck browser on port 3003, registered in viewer-ctl
- summary: Parent epic: `0ee95228`. Spec: `2ccde9ee`. Depends on `3cdcaf3b` (presentation-api).
- ref: `.ticket/tickets/345528ff-a9ca-4d0d-ba94-a5365a14c54e/ticket.toml`

<!-- ticket-index:entry id=d8f76965-1ff3-4a0a-bb24-773b9637fae4 slug=planned/unspecified digest=010a60ebdde8 -->
#### [d8f76965] [session-api] Cascade context gathering from rules/specs/tickets
- summary: Given a `ticket_id` at `session_init`, proactively gather selective context from rules, specs, and tickets across stores by following **hard ID links only** (D5), resolved as URNs.
- ref: `memory-api/.ticket/tickets/d8f76965-1ff3-4a0a-bb24-773b9637fae4/ticket.toml`

<!-- ticket-index:entry id=7db89f25-9395-45b3-a35d-8c5c219067f8 slug=planned/unspecified digest=6655d9401441 -->
#### [7db89f25] [viewer-api] Eliminate per-frame DOM reflow: analytic node rects + skip unchanged LOD writes
- summary: Node cards still trail GPU-drawn edges during orbit/pan/drag. Root cause confirmed by reading graph3d/render.rs: the per-frame render loop performs read-after-write DOM layout thrashing.
- ref: `.ticket/tickets/7db89f25-9395-45b3-a35d-8c5c219067f8/ticket.toml`

<!-- ticket-index:entry id=f7244064-e547-4ba1-9a5e-90240c642b1d slug=planned/unspecified digest=ff5e21804678 -->
#### [f7244064] mcp-toolmon: anomalously slow shutdown after crash-recovery cycle (~89s observed)
- summary: `tests/crash_inflight_subprocess.rs` (T7, real mcp-toolmon subprocess, real OS-level child kill mid-flight) empirically observed: after the automatic crash-recovery respawn (R7) completes and success...
- ref: `.ticket/tickets/f7244064-e547-4ba1-9a5e-90240c642b1d/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=68eaae1f-b230-4aab-8572-cbf41d1d3b6d slug=planned/viewer-api digest=c794234d7496 -->
#### [68eaae1f] [viewer-api][ticket-viewer] Add optional 2D graph mode and presentation keyframing
- priority: `high`
- summary: Add an optional fixed 2D graph presentation mode with a planar camera, 2D grid styling, and presentation keyframing for temporary selection-driven layouts.
- ref: `.ticket/tickets/68eaae1f-b230-4aab-8572-cbf41d1d3b6d/ticket.toml`

<!-- ticket-index:entry id=f9e9aaae-b1ec-434c-a839-7ec990d1e6c7 slug=planned/viewer-api digest=ca4ab0dc1ce4 -->
#### [f9e9aaae] [viewer-api][ticket-viewer] Introduce property-based graph node rendering tiers
- priority: `high`
- summary: Replace the current rich-card-first graph node presentation with property-based level-of-detail rendering that can collapse to points, spheres, icons, labels, and tool-free compact summaries before u...
- ref: `.ticket/tickets/f9e9aaae-b1ec-434c-a839-7ec990d1e6c7/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=dbd048a0-08b4-458d-b860-29b8ce5119e3 slug=planned/viewer-api-dioxus digest=2cf0f9a0d824 -->
#### [dbd048a0] Feature: WgpuOverlay — full-screen GPU compositor with DOM capture and particle effects
- priority: `high`
- ref: `viewer-api/.ticket/tickets/dbd048a0-08b4-458d-b860-29b8ce5119e3/ticket.toml`


### Component: watcher

<!-- ticket-index:entry id=e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5 slug=planned/watcher digest=23e563187f8a -->
#### [e4c6d8f1] [bootstrap][T5] handle early-stop recovery and reassignment
- summary: Agent sessions can terminate unexpectedly at any point during an assignment: stdio disconnect, heartbeat/liveness timeout, repeated auth failures, or explicit worker abort. The executor must handle a...
- ref: `.ticket/tickets/e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5/ticket.toml`


## State: ready

### Component: tooling

<!-- ticket-index:entry id=2b657154-df78-4bb3-807a-66c9ff811ceb slug=ready/tooling digest=59460740ee5e -->
#### [2b657154] Handle unregistered worktree debris during removal
- priority: `high`
- summary: `tools/worktree/worktree.sh list` correctly labels unregistered directories beneath `.worktrees/` as `UNREGISTERED-DEBRIS`, but `remove <name>` identifies such a directory with `git -C <path> rev-par...
- ref: `.ticket/tickets/2b657154-df78-4bb3-807a-66c9ff811ceb/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=6bbda148-e144-4dff-92de-dd6584c82bd7 slug=ready/viewer-api digest=d9ab101d7a59 -->
#### [6bbda148] [viewer-ctl] Implement uninstall command for managed viewers
- priority: `high`
- summary: The install contract now validates `viewer-ctl install` for all managed viewers, but `viewer-ctl` still has no first-class uninstall/remove command. That leaves `VIEW-04` as a manual gap in the insta...
- ref: `viewer-api/.ticket/tickets/6bbda148-e144-4dff-92de-dd6584c82bd7/ticket.toml`

<!-- ticket-index:entry id=63184300-e5d2-4c9b-9052-64ceabdc7f9e slug=ready/viewer-api digest=4ca4df08362f -->
#### [63184300] demo-viewer probe ticket
- ref: `viewer-api/.ticket/tickets/63184300-e5d2-4c9b-9052-64ceabdc7f9e/ticket.toml`

<!-- ticket-index:entry id=ba0fd25e-c23b-48a4-934c-a30542f6fca9 slug=ready/viewer-api digest=aa438992b20a -->
#### [ba0fd25e] demo-viewer: README + viewer-api docs cross-references
- summary: Add a top-level note in `tools/viewer/viewer-api/README.md` linking to
- ref: `viewer-api/.ticket/tickets/ba0fd25e-c23b-48a4-934c-a30542f6fca9/ticket.toml`

<!-- ticket-index:entry id=b83b1002-41ba-4eb7-9f1a-c20cbf49137b slug=ready/viewer-api digest=f0a1017b0266 -->
#### [b83b1002] demo-viewer: e2e harness + WebGPU launch profile + helpers
- summary: Add the demo-viewer e2e harness so per-feature tickets only need to
- ref: `viewer-api/.ticket/tickets/b83b1002-41ba-4eb7-9f1a-c20cbf49137b/ticket.toml`

<!-- ticket-index:entry id=76378ed1-f50d-43d9-9414-04cfc3232a00 slug=ready/viewer-api digest=277f45275da1 -->
#### [76378ed1] demo-viewer: feature page — auth middleware
- summary: Implement the demo page that showcases the `viewer-api/auth-middleware` feature surface.
- ref: `viewer-api/.ticket/tickets/76378ed1-f50d-43d9-9414-04cfc3232a00/ticket.toml`

<!-- ticket-index:entry id=db1cccef-6712-4702-ae58-fe23dacc029f slug=ready/viewer-api digest=321909dad068 -->
#### [db1cccef] demo-viewer: feature page — client log
- summary: Implement the demo page that showcases the `viewer-api/client-log` feature surface.
- ref: `viewer-api/.ticket/tickets/db1cccef-6712-4702-ae58-fe23dacc029f/ticket.toml`

<!-- ticket-index:entry id=e737092d-6083-4bf8-ba0e-1ac22d7c521b slug=ready/viewer-api digest=68df921e08bc -->
#### [e737092d] demo-viewer: feature page — code viewer
- summary: Implement the demo page that showcases the `viewer-api/components/code-viewer` feature surface.
- ref: `viewer-api/.ticket/tickets/e737092d-6083-4bf8-ba0e-1ac22d7c521b/ticket.toml`

<!-- ticket-index:entry id=b543e4ad-7ac4-4fd0-abb7-59a725affa64 slug=ready/viewer-api digest=d4ab61f8076a -->
#### [b543e4ad] demo-viewer: feature page — dev proxy
- summary: Implement the demo page that showcases the `viewer-api/dev-proxy` feature surface.
- ref: `viewer-api/.ticket/tickets/b543e4ad-7ac4-4fd0-abb7-59a725affa64/ticket.toml`

<!-- ticket-index:entry id=42bd0dc8-cc28-46b3-9535-2d1207b18ae6 slug=ready/viewer-api digest=fc7144a90774 -->
#### [42bd0dc8] demo-viewer: feature page — graph3d (WebGPU)
- summary: Implement the demo page that showcases the `viewer-api/components/graph3d` feature surface.
- ref: `viewer-api/.ticket/tickets/42bd0dc8-cc28-46b3-9535-2d1207b18ae6/ticket.toml`

<!-- ticket-index:entry id=6006ec27-babd-4656-9eca-78bdd5eb5b47 slug=ready/viewer-api digest=62ef7017b679 -->
#### [6006ec27] demo-viewer: feature page — icons spinner
- summary: Implement the demo page that showcases the `viewer-api/components/icons-spinner` feature surface.
- ref: `viewer-api/.ticket/tickets/6006ec27-babd-4656-9eca-78bdd5eb5b47/ticket.toml`

<!-- ticket-index:entry id=fc0282b5-844c-4101-9391-c926ffdaf1d7 slug=ready/viewer-api digest=838decd3c4a6 -->
#### [fc0282b5] demo-viewer: feature page — layout
- summary: Implement the demo page that showcases the `viewer-api/components/layout` feature surface.
- ref: `viewer-api/.ticket/tickets/fc0282b5-844c-4101-9391-c926ffdaf1d7/ticket.toml`

<!-- ticket-index:entry id=02025547-027b-43f7-bcd7-6a212108085f slug=ready/viewer-api digest=d8b4aee66b7c -->
#### [02025547] demo-viewer: feature page — pagination query
- summary: Implement the demo page that showcases the `viewer-api/pagination-query` feature surface.
- ref: `viewer-api/.ticket/tickets/02025547-027b-43f7-bcd7-6a212108085f/ticket.toml`

<!-- ticket-index:entry id=8de2f8e2-43b6-4de1-a9f7-54fc64c2bdab slug=ready/viewer-api digest=fcfa774493c1 -->
#### [8de2f8e2] demo-viewer: feature page — server infra
- summary: Implement the demo page that showcases the `viewer-api/server-infra` feature surface.
- ref: `viewer-api/.ticket/tickets/8de2f8e2-43b6-4de1-a9f7-54fc64c2bdab/ticket.toml`

<!-- ticket-index:entry id=ed8252fc-371d-4134-a981-5af988c4241a slug=ready/viewer-api digest=37c3b3715477 -->
#### [ed8252fc] demo-viewer: feature page — session
- summary: Implement the demo page that showcases the `viewer-api/session` feature surface.
- ref: `viewer-api/.ticket/tickets/ed8252fc-371d-4134-a981-5af988c4241a/ticket.toml`

<!-- ticket-index:entry id=48530193-0637-4709-8239-e8f3e1cc0eba slug=ready/viewer-api digest=08e4b9a3a2e1 -->
#### [48530193] demo-viewer: feature page — source
- summary: Implement the demo page that showcases the `viewer-api/source` feature surface.
- ref: `viewer-api/.ticket/tickets/48530193-0637-4709-8239-e8f3e1cc0eba/ticket.toml`

<!-- ticket-index:entry id=258ed497-b5ca-4622-96a3-6f1ea210e7bb slug=ready/viewer-api digest=2e1fcbdb2ecb -->
#### [258ed497] demo-viewer: feature page — sse
- summary: Implement the demo page that showcases the `viewer-api/sse` feature surface.
- ref: `viewer-api/.ticket/tickets/258ed497-b5ca-4622-96a3-6f1ea210e7bb/ticket.toml`

<!-- ticket-index:entry id=1efec195-f8b4-4571-b073-806cac0b66ce slug=ready/viewer-api digest=9f19a37001c9 -->
#### [1efec195] demo-viewer: feature page — store primitives
- summary: Implement the demo page that showcases the `viewer-api/store-primitives` feature surface.
- ref: `viewer-api/.ticket/tickets/1efec195-f8b4-4571-b073-806cac0b66ce/ticket.toml`

<!-- ticket-index:entry id=0eef1873-0626-4a87-93bc-51d182808e16 slug=ready/viewer-api digest=47349fb4ad54 -->
#### [0eef1873] demo-viewer: feature page — tab bar
- summary: Implement the demo page that showcases the `viewer-api/components/tab-bar` feature surface.
- ref: `viewer-api/.ticket/tickets/0eef1873-0626-4a87-93bc-51d182808e16/ticket.toml`

<!-- ticket-index:entry id=6f924445-ea9c-46e9-b051-b5aab6b798fa slug=ready/viewer-api digest=50913c343141 -->
#### [6f924445] demo-viewer: feature page — theme settings
- summary: Implement the demo page that showcases the `viewer-api/theme-settings` feature surface.
- ref: `viewer-api/.ticket/tickets/6f924445-ea9c-46e9-b051-b5aab6b798fa/ticket.toml`

<!-- ticket-index:entry id=8d0e9879-5e42-449f-90a6-0060dbde112f slug=ready/viewer-api digest=4e9db0dc5f03 -->
#### [8d0e9879] demo-viewer: feature page — tracing
- summary: Implement the demo page that showcases the WASM-tracing pipeline
- ref: `viewer-api/.ticket/tickets/8d0e9879-5e42-449f-90a6-0060dbde112f/ticket.toml`

<!-- ticket-index:entry id=ad056493-716c-4c32-b8f6-9b67a25bc52e slug=ready/viewer-api digest=0f787393bcbd -->
#### [ad056493] demo-viewer: feature page — tracing
- summary: Implement the demo page that showcases the WASM-tracing pipeline
- ref: `viewer-api/.ticket/tickets/ad056493-716c-4c32-b8f6-9b67a25bc52e/ticket.toml`

<!-- ticket-index:entry id=3df77f25-0f1c-4c1c-a2a8-e9c885f275db slug=ready/viewer-api digest=f1e37378c23b -->
#### [3df77f25] demo-viewer: feature page — tree view
- summary: Implement the demo page that showcases the `viewer-api/components/tree-view` feature surface.
- ref: `viewer-api/.ticket/tickets/3df77f25-0f1c-4c1c-a2a8-e9c885f275db/ticket.toml`

<!-- ticket-index:entry id=9d7d97bb-fc65-4374-8de8-f22bd2a05c18 slug=ready/viewer-api digest=a22bea008c4a -->
#### [9d7d97bb] demo-viewer: feature page — wgpu overlay (WebGPU)
- summary: Implement the demo page that showcases the `viewer-api/effects/wgpu-overlay` feature surface.
- ref: `viewer-api/.ticket/tickets/9d7d97bb-fc65-4374-8de8-f22bd2a05c18/ticket.toml`

<!-- ticket-index:entry id=ee2b9e6d-e093-41df-9838-d6ab7dfde0fa slug=ready/viewer-api digest=14caed598f84 -->
#### [ee2b9e6d] demo-viewer: manual validation epic (signs off `verified` on the umbrella spec)
- summary: Final sign-off ticket. Closing this ticket transitions the umbrella spec
- ref: `viewer-api/.ticket/tickets/ee2b9e6d-e093-41df-9838-d6ab7dfde0fa/ticket.toml`

<!-- ticket-index:entry id=b779c650-0775-4e4f-a692-3eaaa939a910 slug=ready/viewer-api digest=77163e951668 -->
#### [b779c650] demo-viewer: scaffold bin crate + Dioxus SPA
- summary: Create the demo-viewer crate skeleton inside the `viewer-api` workspace
- ref: `viewer-api/.ticket/tickets/b779c650-0775-4e4f-a692-3eaaa939a910/ticket.toml`

<!-- ticket-index:entry id=5d9e331b-dc18-444b-af45-90a14d096847 slug=ready/viewer-api digest=a5d0eee23dab -->
#### [5d9e331b] demo-viewer: viewer-ctl integration + nav generator
- summary: Add a `[viewers.demo-viewer]` entry to `viewer-ctl.toml` (port 3099,
- ref: `viewer-api/.ticket/tickets/5d9e331b-dc18-444b-af45-90a14d096847/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=f00204fc-f33f-4cd6-9b5f-395071f4e118 slug=ready/viewer-api-dioxus digest=916edb6e725d -->
#### [f00204fc] Bug: ticket-viewer theme inconsistency — --panel-bg hardcoded dark, breaks light themes
- priority: `high`
- summary: `viewer-api-dioxus` defines two parallel surface palettes in `public/css/variables.css`:
- ref: `viewer-api/.ticket/tickets/f00204fc-f33f-4cd6-9b5f-395071f4e118/ticket.toml`

<!-- ticket-index:entry id=dc83b7b4-4b0f-4732-9163-488ef0c6bcc4 slug=ready/viewer-api-dioxus digest=b6a121d8b290 -->
#### [dc83b7b4] UI: transparent context-adaptive header & sidebar action buttons (IconButton + Chip)
- priority: `medium`
- summary: The action buttons in the ticket-viewer header (`🎨 Theme settings`, `☑ Batch`, `+ New Ticket`) are styled inline in `routes.rs` with three different background tokens (`var(--bg-secondary)`, `var(--a...
- ref: `viewer-api/.ticket/tickets/dc83b7b4-4b0f-4732-9163-488ef0c6bcc4/ticket.toml`


### Component: viewer-api-leptos

<!-- ticket-index:entry id=92d5223b-05a4-4b80-ae6a-f5f5d45db2fc slug=ready/viewer-api-leptos digest=2801a37268dc -->
#### [92d5223b] Feature: Complete theme system — colors, effects, presets, CSS variables
- summary: The Leptos frontend has a minimal theme system: 5 hardcoded presets, a simple button grid in a Settings tab, GPU-only uniforms with no CSS variable injection, and no color editing. The TS version has...
- ref: `viewer-api/.ticket/tickets/92d5223b-05a4-4b80-ae6a-f5f5d45db2fc/ticket.toml`


## State: todo

### Component: unspecified

<!-- ticket-index:entry id=8418fa92-bf46-42d9-a93f-9240032893b7 slug=todo/unspecified digest=548e2b8972e5 -->
#### [8418fa92] Dedicated orchestrator agent: single sub-agent tool, plan + aggregate only
- summary: Add an explicit **orchestrator agent** whose only capability is to call sub-agents. It performs high-level planning, dispatches work to multiple sub-agents, and aggregates their results back into the...
- ref: `.ticket/tickets/8418fa92-bf46-42d9-a93f-9240032893b7/ticket.toml`

<!-- ticket-index:entry id=a5ad2721-2d07-47dd-85f5-f180d4a030fa slug=todo/unspecified digest=87819542d1f0 -->
#### [a5ad2721] Model-aware MCP tool wrapper: block expensive-model calls and require delegation
- summary: Route every MCP tool call through a **model-aware wrapper** that enforces price-awareness policy at the tool boundary. Prompt templates, agent templates, and other guidance files already declare whic...
- ref: `.ticket/tickets/a5ad2721-2d07-47dd-85f5-f180d4a030fa/ticket.toml`

