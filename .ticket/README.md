<!-- ticket-index:file generated=true -->

# Ticket Catalog

Generated ticket index grouped by state and component. Use this before scanning raw `.ticket/tickets/` folders.

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

<!-- ticket-index:entry id=2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2 slug=cancelled/lease digest=f5856a1550eb -->
#### [2a1fa2f2] [bootstrap] implement lease lifecycle with stale recovery
- summary: Status:** BLOCKED (requires Phase 1 CRUD stable)
- ref: `memory-viewers/memory-api/.ticket/tickets/2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2/ticket.toml`


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


### Component: ticket-http

<!-- ticket-index:entry id=b458cba7-54b1-45d8-8c86-17b920416b8b slug=cancelled/ticket-http digest=6e13164d19a5 -->
#### [b458cba7] API: Batch mutation endpoint for transactional multi-command execution
- priority: `high`
- ref: `memory-viewers/memory-api/.ticket/tickets/b458cba7-54b1-45d8-8c86-17b920416b8b/ticket.toml`

<!-- ticket-index:entry id=3fd32109-7122-4fdf-80f2-b741db5d3b30 slug=cancelled/ticket-http digest=218c34e62944 -->
#### [3fd32109] [ticket-http][ticket-viewer] Expose workspace graph payload for focused full-graph navigation
- priority: `high`
- summary: Provide infrastructure for a ticket-viewer graph mode that can keep the whole workspace graph visible while focusing the selected ticket.
- ref: `memory-viewers/memory-api/.ticket/tickets/3fd32109-7122-4fdf-80f2-b741db5d3b30/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=fea28293-5494-49e1-bdb4-8165457b59ca slug=cancelled/ticket-viewer digest=48b4e37b89a4 -->
#### [fea28293] Feature: Batch operations — multi-select, queue, bulk apply, filter-based updates
- priority: `high`
- ref: `memory-viewers/.ticket/tickets/fea28293-5494-49e1-bdb4-8165457b59ca/ticket.toml`

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

<!-- ticket-index:entry id=2dcc66b5-c061-45fa-a55f-63b731727bb6 slug=cancelled/ticket-viewer digest=8629cb7212d4 -->
#### [2dcc66b5] [ticket-viewer] Build integrated ticket document panel
- priority: `high`
- summary: Replace the split metadata/content treatment with a single compact ticket document area in the main layout.
- ref: `memory-viewers/memory-api/.ticket/tickets/2dcc66b5-c061-45fa-a55f-63b731727bb6/ticket.toml`

<!-- ticket-index:entry id=379ff931-7e0a-4069-a1d7-86cc3ef73e9e slug=cancelled/ticket-viewer digest=972d8c4f8b42 -->
#### [379ff931] [ticket-viewer] Fix graph layout defaults and isometric settings
- priority: `high`
- summary: Fix graph layout defaults and settings so dependency hierarchy reads cleanly from top to bottom on a 2D plane optimized for isometric viewing.
- ref: `memory-viewers/memory-api/.ticket/tickets/379ff931-7e0a-4069-a1d7-86cc3ef73e9e/ticket.toml`

<!-- ticket-index:entry id=3526cce3-c934-4c37-b7a8-c7c0979f308d slug=cancelled/ticket-viewer digest=70bed52c970c -->
#### [3526cce3] [ticket-viewer] Keep full workspace graph visible with focused navigation
- priority: `high`
- summary: Change the ticket-viewer graph mode so the full graph stays visible while the selected ticket becomes the active focus anchor.
- ref: `memory-viewers/memory-api/.ticket/tickets/3526cce3-c934-4c37-b7a8-c7c0979f308d/ticket.toml`

<!-- ticket-index:entry id=0d2e5a7d-f76b-474a-8991-b3a56ea73ac5 slug=cancelled/ticket-viewer digest=434ff2e93095 -->
#### [0d2e5a7d] [ticket-viewer][ticket-http][viewer-api] Improve main layout ticket documents and focused full-graph navigation
- priority: `high`
- summary: Upgrade the ticket-viewer main layout so ticket details render as a compact integrated document and the graph view becomes a focused full-workspace navigation surface with better layout, settings, an...
- ref: `memory-viewers/memory-api/.ticket/tickets/0d2e5a7d-f76b-474a-8991-b3a56ea73ac5/ticket.toml`


### Component: unspecified

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


### Component: viewer-api

<!-- ticket-index:entry id=ca0f6ccc-545d-45df-bbbb-74a3daf0d18c slug=cancelled/viewer-api digest=352df8dc96ee -->
#### [ca0f6ccc] Arch: Extract viewer-api-leptos shared crate for all Leptos viewers
- summary: The log-viewer Leptos frontend currently lives as a monolith in `tools/viewer/log-viewer/frontend-leptos/`. Shared UI primitives (ResizeHandle, TreeView, TabBar, CodeViewer, ThemeSettings, WgpuOverla...
- ref: `memory-viewers/viewer-api/.ticket/tickets/ca0f6ccc-545d-45df-bbbb-74a3daf0d18c/ticket.toml`

<!-- ticket-index:entry id=7f41940d-617a-495d-aad8-5a19111bdab9 slug=cancelled/viewer-api digest=346e11c59702 -->
#### [7f41940d] Epic: Leptos Viewer Platform — port viewer-api + all viewers to Leptos/Rust
- ref: `memory-viewers/viewer-api/.ticket/tickets/7f41940d-617a-495d-aad8-5a19111bdab9/ticket.toml`

<!-- ticket-index:entry id=7b33f98e-9572-4ceb-8379-189621e4ae74 slug=cancelled/viewer-api digest=c584693164b3 -->
#### [7b33f98e] [viewer-api] Extract a reusable interactive chip button for Dioxus explorer filters
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/7b33f98e-9572-4ceb-8379-189621e4ae74/ticket.toml`

<!-- ticket-index:entry id=d3fb343c-1fea-47b1-8137-5ac7a37a95e1 slug=cancelled/viewer-api digest=7555b7c3583d -->
#### [d3fb343c] [viewer-api][ticket-viewer] Add multi-level graph node detail rendering
- priority: `high`
- summary: Introduce multiple graph node detail levels so zoomed-out views stay legible and zoomed-in views can show rich ticket content.
- ref: `memory-viewers/memory-api/.ticket/tickets/d3fb343c-1fea-47b1-8137-5ac7a37a95e1/ticket.toml`


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

### Component: audit-api

<!-- ticket-index:entry id=95d4f986-b81c-4951-bae5-4227f2d72a6d slug=done/audit-api digest=1d183fd45113 -->
#### [95d4f986] [audit-api] Include dependency convergence findings in default repo audit
- priority: `high`
- summary: The default repo audit currently validates only a narrow slice of ticket dependency topology. The shipped audit spec covers orphan tickets, and this ticket started as a follow-up to surface raw depen...
- ref: `memory-viewers/.ticket/tickets/95d4f986-b81c-4951-bae5-4227f2d72a6d/ticket.toml`

<!-- ticket-index:entry id=a762448e-464c-43da-95b8-e49eb07814ed slug=done/audit-api digest=af2e904f104f -->
#### [a762448e] [audit-api] Require every ticket to participate in dependency graph
- priority: `high`
- summary: Add an audit validation rule that flags tickets with neither outgoing depends_on edges nor incoming dependees so every ticket participates in the ticket graph. For legitimately standalone work, creat...
- ref: `memory-viewers/memory-api/.ticket/tickets/a762448e-464c-43da-95b8-e49eb07814ed/ticket.toml`


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

<!-- ticket-index:entry id=1b65d658-07d0-4d31-881b-6111321b5752 slug=done/context-editor digest=d7a14655082d -->
#### [1b65d658] SDF Item Cutting: CSG Shader Subtraction, Cut Particles & Liquid Glass Impact Feedback
- priority: `high`
- ref: `.ticket/tickets/1b65d658-07d0-4d31-881b-6111321b5752/ticket.toml`

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

<!-- ticket-index:entry id=609099ac-c5b5-4fe2-8072-a7b19ff8d75c slug=done/doc-api digest=e9b578557929 -->
#### [609099ac] [doc-api] Support cargo metadata outputs as docs workspace inputs
- priority: `high`
- summary: Add `cargo metadata` output support to the new docs surface so `doc-api` can use Cargo's workspace and package graph as a docs workspace input.
- ref: `memory-viewers/memory-api/.ticket/tickets/609099ac-c5b5-4fe2-8072-a7b19ff8d75c/ticket.toml`


### Component: doc-http

<!-- ticket-index:entry id=4e99c7dd-6e1b-4bce-a8c9-67e5182a4dc3 slug=done/doc-http digest=5d7d7e34bd76 -->
#### [4e99c7dd] [doc-http] Support cargo doc generated HTML and JSON outputs
- priority: `high`
- summary: Add support for generated `cargo doc` outputs so the docs family can register, describe, and serve Rust documentation HTML and rustdoc JSON artifacts.
- ref: `memory-viewers/memory-api/.ticket/tickets/4e99c7dd-6e1b-4bce-a8c9-67e5182a4dc3/ticket.toml`


### Component: doc-viewer

<!-- ticket-index:entry id=391fcd15-0da6-4b39-86f3-19afca688377 slug=done/doc-viewer digest=cca8ea2a1df2 -->
#### [391fcd15] [doc-viewer] Rewrite doc-viewer as a Dioxus viewer over doc-http
- priority: `high`
- summary: Implement a concrete doc-viewer migration path that replaces the current Preact-first shell with a Dioxus frontend built on `viewer-api-dioxus` and backed by `doc-http` for its server-facing document...
- ref: `memory-viewers/viewer-api/.ticket/tickets/391fcd15-0da6-4b39-86f3-19afca688377/ticket.toml`


### Component: documentation

<!-- ticket-index:entry id=2fb3adb0-fa3a-41a6-8fd6-38096635a38b slug=done/documentation digest=527c343c6dbd -->
#### [2fb3adb0] [readmes] Smooth repository README surfaces
- priority: `high`
- ref: `.ticket/tickets/2fb3adb0-fa3a-41a6-8fd6-38096635a38b/ticket.toml`


### Component: history

<!-- ticket-index:entry id=77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4 slug=done/history digest=13136a32287a -->
#### [77f1eb5c] [bootstrap] wire history, diff, and revert end-to-end
- ref: `memory-viewers/memory-api/.ticket/tickets/77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4/ticket.toml`


### Component: instructions

<!-- ticket-index:entry id=33565741-c3ce-4697-91d3-092a803aaac0 slug=done/instructions digest=ec0c2f6acb0b -->
#### [33565741] [ticket-system] Instruction updates: mandatory review gate and diligent state progression
- priority: `high`
- ref: `.ticket/tickets/33565741-c3ce-4697-91d3-092a803aaac0/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=8affb65d-605b-4225-819a-af951e0bd318 slug=done/memory-api digest=ac138f6baff6 -->
#### [8affb65d] [memory-api] Add shared store bootstrap open_or_init helpers
- priority: `high`
- summary: `ticket-viewer` needed a viewer-local workaround to start from a checkout where
- ref: `memory-viewers/memory-api/.ticket/tickets/8affb65d-605b-4225-819a-af951e0bd318/ticket.toml`

<!-- ticket-index:entry id=6124971a-0775-455f-a7b8-840766a43ce3 slug=done/memory-api digest=e633e0d27df8 -->
#### [6124971a] [memory-api] Canonicalize local store root resolution
- priority: `high`
- summary: Implemented shared workspace/store root normalization for ticket/spec/rule stores, validated ticket create target roots so repo/store paths resolve into .ticket/tickets, removed the ticket workspace ...
- ref: `memory-viewers/memory-api/.ticket/tickets/6124971a-0775-455f-a7b8-840766a43ce3/ticket.toml`

<!-- ticket-index:entry id=7f7fe4a8-a1d6-44b4-baf9-9500f6db40a5 slug=done/memory-api digest=67637f4a5e0f -->
#### [7f7fe4a8] [memory-index] Define domain digest input contract for generated index entries
- priority: `high`
- summary: `IndexEntry` and `IndexSidecar` already define a generic digest algorithm, but the generator tickets do not define how each domain produces the stable input fields that feed that digest. In particula...
- ref: `.ticket/tickets/7f7fe4a8-a1d6-44b4-baf9-9500f6db40a5/ticket.toml`

<!-- ticket-index:entry id=9491f6b7-c11b-4d94-aed6-f5c6ea004e8a slug=done/memory-api digest=61636de9f0ef -->
#### [9491f6b7] [session-api] Plan and scaffold Copilot chat-session capture in memory-api
- priority: `high`
- summary: Plan and scaffold a bounded first `session-api` slice under `memory-viewers/memory-api` for saving Copilot chat sessions into a memory-api-backed store.
- ref: `.ticket/tickets/9491f6b7-c11b-4d94-aed6-f5c6ea004e8a/ticket.toml`

<!-- ticket-index:entry id=d5722e8e-4932-4ccc-9cee-480ada710202 slug=done/memory-api digest=cc8634cbda3d -->
#### [d5722e8e] [spec][P0.5] memory-api — EntityStore convenience facade
- priority: `high`
- summary: Add an `EntityStore` struct to `memory-api` that composes `RedbIndexStore`, `EntityFs`, and `TantivySearchIndex` into a single convenient type. This gives downstream crates (spec-api, ticket-api) a u...
- ref: `memory-viewers/memory-api/.ticket/tickets/d5722e8e-4932-4ccc-9cee-480ada710202/ticket.toml`

<!-- ticket-index:entry id=e0b3e9a8-bd43-472a-8222-f8c5e3321dbd slug=done/memory-api digest=83f9155aa2ce -->
#### [e0b3e9a8] [spec][P0] Extract memory-api crate — generic entity storage, index, search, schema engine
- priority: `critical`
- summary: Extract ~75% of ticket-api into a generic `memory-api` crate that provides filesystem-backed entity storage with schema validation, indexing, search, and graph edges. Both ticket-api and the new spec...
- ref: `memory-viewers/memory-api/.ticket/tickets/e0b3e9a8-bd43-472a-8222-f8c5e3321dbd/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=762d9ac9-e0e0-4f02-b60f-21c79e3c26f6 slug=done/repo-guidance digest=fe3bfd7a0020 -->
#### [762d9ac9] Enforce ticket-spec-validation-doc-review workflow across generated guidance
- priority: `high`
- summary: The repository guidance does not consistently enforce one workflow for normal engineering work.
- ref: `.ticket/tickets/762d9ac9-e0e0-4f02-b60f-21c79e3c26f6/ticket.toml`

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

<!-- ticket-index:entry id=d6f5f59e-3955-443f-9381-afc486d0b8ad slug=done/repo-guidance digest=0dd20e1124e7 -->
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

<!-- ticket-index:entry id=af7ee01c-d649-4ed2-898c-d4f2e148f00f slug=done/rule-api digest=296ef7080591 -->
#### [af7ee01c] Add explain and preview tooling for rule target composition
- priority: `high`
- summary: The current generation flow does not explain why a file contains the entries it does. That makes target construction difficult to review, debug, and evolve.
- ref: `memory-viewers/memory-api/.ticket/tickets/af7ee01c-d649-4ed2-898c-d4f2e148f00f/ticket.toml`

<!-- ticket-index:entry id=48b1cefb-dcc5-4cd4-ac41-568e57c97aca slug=done/rule-api digest=100f85d51eed -->
#### [48b1cefb] Add rule-api tools to edit and generate agent markdown
- priority: `high`
- summary: A canonical `rule-api` store is not enough by itself. The team needs tools to import duplicated markdown, edit canonical rule entries, and generate repo-local files so manual file editing is no longe...
- ref: `memory-viewers/memory-api/.ticket/tickets/48b1cefb-dcc5-4cd4-ac41-568e57c97aca/ticket.toml`

<!-- ticket-index:entry id=88800b2e-74f5-4d65-958a-1423d18072e3 slug=done/rule-api digest=a77578378b60 -->
#### [88800b2e] Attach ratings and feedback to rule entries
- priority: `high`
- summary: The current markdown files do not provide a structured, uniform way for agents to record whether a rule entry was helpful, outdated, conflicting, or in need of revision.
- ref: `memory-viewers/memory-api/.ticket/tickets/88800b2e-74f5-4d65-958a-1423d18072e3/ticket.toml`

<!-- ticket-index:entry id=dee7de7a-4af0-468e-b779-309192e2e4db slug=done/rule-api digest=a44f8bb2964d -->
#### [dee7de7a] Create rule-api storage model and stable rule IDs
- priority: `high`
- summary: We need a concrete `rule-api` domain on top of `memory-api` storage primitives so canonical rule entries can be stored, indexed, searched, versioned, rendered into markdown, and annotated with rating...
- ref: `memory-viewers/memory-api/.ticket/tickets/dee7de7a-4af0-468e-b779-309192e2e4db/ticket.toml`

<!-- ticket-index:entry id=18eb59ee-05f6-4a03-b522-438b67556141 slug=done/rule-api digest=70e5eababa74 -->
#### [18eb59ee] Design hierarchical rule target schema
- priority: `high`
- summary: `RenderTarget` currently acts as a flat filter plus output path. That is not expressive enough to describe a document outline, per-section composition, or explicit ordering within a file.
- ref: `memory-viewers/memory-api/.ticket/tickets/18eb59ee-05f6-4a03-b522-438b67556141/ticket.toml`

<!-- ticket-index:entry id=84ee1a9b-e0e8-4990-a9c7-af0e7b336d0e slug=done/rule-api digest=883647334a63 -->
#### [84ee1a9b] Implement deterministic hierarchical rule target evaluation
- priority: `high`
- summary: Even with a better schema, the current generation path still assumes one file-wide filter and one flat ordered list of rule entries. The evaluator needs to understand a hierarchical target tree and r...
- ref: `memory-viewers/memory-api/.ticket/tickets/84ee1a9b-e0e8-4990-a9c7-af0e7b336d0e/ticket.toml`

<!-- ticket-index:entry id=050c5441-1d3a-46bc-9748-cfb7030a93bd slug=done/rule-api digest=ae0a11086d92 -->
#### [050c5441] Implement nested rule workspaces across submodule repositories
- priority: `high`
- summary: `rule-api` currently operates on one workspace root and one target config per invocation. That is enough for the top-level `context-engine` workflow, but it does not let `memory-viewers/`, `memory-ap...
- ref: `memory-viewers/memory-api/.ticket/tickets/050c5441-1d3a-46bc-9748-cfb7030a93bd/ticket.toml`

<!-- ticket-index:entry id=c809ae33-a4fa-4e5f-b920-5d269466a11c slug=done/rule-api digest=97207c82b933 -->
#### [c809ae33] Improve rule target construction for hierarchical document outlines
- priority: `high`
- summary: The current `rule-targets.toml` model builds each output file from one flat filter. That keeps the implementation small, but it makes file composition hard to reason about, encourages repeating rule ...
- ref: `memory-viewers/memory-api/.ticket/tickets/c809ae33-a4fa-4e5f-b920-5d269466a11c/ticket.toml`

<!-- ticket-index:entry id=454405a2-a37e-4be6-b7a7-b96008afa974 slug=done/rule-api digest=10f11f16ef05 -->
#### [454405a2] Migrate duplicated agent docs into generated rule-api outputs
- priority: `high`
- summary: The existing `AGENTS.md` and `.github` markdown files are duplicated across context-engine, memory-viewers, memory-api, and viewer-api. Shared text is currently owned by copy-paste instead of by a ca...
- ref: `memory-viewers/memory-api/.ticket/tickets/454405a2-a37e-4be6-b7a7-b96008afa974/ticket.toml`

<!-- ticket-index:entry id=f76169f7-239d-4993-a0a2-0709414acb7f slug=done/rule-api digest=ea0faf9640f5 -->
#### [f76169f7] Preserve existing line endings in generated outputs
- priority: `medium`
- summary: Implemented shared generated-output newline preparation in rule-api so rewrites adapt to the existing file's newline sequence while new files stay canonical LF. Wired the behavior into rule-cli and r...
- ref: `memory-viewers/memory-api/.ticket/tickets/f76169f7-239d-4993-a0a2-0709414acb7f/ticket.toml`

<!-- ticket-index:entry id=e057932b-aaa8-43f5-be33-91dbf7399057 slug=done/rule-api digest=4d8e4007878d -->
#### [e057932b] [rule-api] Backfill existing rule workspaces to body.md storage
- priority: `high`
- summary: Once `rule-api` can understand the new storage contract, the repository still has 543 existing rule folders spread across four workspaces that need to be brought into the new layout. Leaving them mix...
- ref: `memory-viewers/memory-api/.ticket/tickets/e057932b-aaa8-43f5-be33-91dbf7399057/ticket.toml`

<!-- ticket-index:entry id=d8581db8-ab3b-4445-8f1b-1b5dbf801b5e slug=done/rule-api digest=80e0f62008b6 -->
#### [d8581db8] [rule-api] Define body.md rule storage contract and migration plan
- priority: `high`
- summary: `rule-api` currently stores canonical rule body text in two places:
- ref: `memory-viewers/memory-api/.ticket/tickets/d8581db8-ab3b-4445-8f1b-1b5dbf801b5e/ticket.toml`

<!-- ticket-index:entry id=e395bad6-c70c-4957-80da-412491304c84 slug=done/rule-api digest=de0d8cde130d -->
#### [e395bad6] [rule-api] Implement canonical body.md storage with legacy rule compatibility
- priority: `high`
- summary: Even after the desired `body.md` contract is clear, `rule-api` still depends on shared storage helpers that hardcode `description.md`, and the rule schema still requires a manifest-level `body` field...
- ref: `memory-viewers/memory-api/.ticket/tickets/e395bad6-c70c-4957-80da-412491304c84/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=959c94bd-4a42-47d6-bee4-a12332a23b52 slug=done/session-api digest=ce26229619a4 -->
#### [959c94bd] [session-api] Add hook ingestion and read/query support
- priority: `high`
- summary: Implement the next `session-api` batch in the nested `memory-api` workspace by making transcript persistence append only and adding the first read/query plus hook-facing capture APIs.
- ref: `memory-viewers/memory-api/.ticket/tickets/959c94bd-4a42-47d6-bee4-a12332a23b52/ticket.toml`

<!-- ticket-index:entry id=f76b0fa9-d880-45da-b039-b483e904ee2f slug=done/session-api digest=d330ca154756 -->
#### [f76b0fa9] [session-api] Add session-cli and session-mcp for session subcommands
- priority: `high`
- summary: Expose the `session-api` capabilities (check-in, lookup, query, range peeking, and skeleton peeking) through dedicated CLI and MCP surfaces so that agents and users can interact with sessions cleanly.
- ref: `.ticket/tickets/f76b0fa9-d880-45da-b039-b483e904ee2f/ticket.toml`

<!-- ticket-index:entry id=c8f79641-6f99-4401-9b08-ad960a8d785c slug=done/session-api digest=6fd8a9a876fe -->
#### [c8f79641] [session-api] Persist session captures to filesystem store
- priority: `high`
- summary: Implement the next `session-api` batch in the nested `memory-api` workspace by turning the current store plan into a real filesystem write path.
- ref: `memory-viewers/memory-api/.ticket/tickets/c8f79641-6f99-4401-9b08-ad960a8d785c/ticket.toml`

<!-- ticket-index:entry id=e663f9e9-ac52-4c0e-8e07-d17c8a15b48d slug=done/session-api digest=e47192ef4337 -->
#### [e663f9e9] [session-api] Wire VS Code Copilot stop-hook session capture
- priority: `high`
- summary: Implement the first external integration slice for session capture by wiring VS Code GitHub Copilot chat hooks to the existing `session-api` persistence path.
- ref: `.ticket/tickets/e663f9e9-ac52-4c0e-8e07-d17c8a15b48d/ticket.toml`

<!-- ticket-index:entry id=e2189e9d-8ea7-4747-bda9-51e573ba51ca slug=done/session-api digest=40d7654b3f3c -->
#### [e2189e9d] [session-api][worktree] Implement session check-in and worktree assignment surfaces
- priority: `high`
- summary: Implement the first executable slice of the default worktree-backed session workflow after `68a49ca7` locks the contract.
- ref: `.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=09641443-a8f2-479d-85cb-ea44a963595b slug=done/spec-api digest=1c18dab3cc3b -->
#### [09641443] Add spec-local target mapping for generated spec artifacts
- priority: `high`
- summary: `spec-api` has generated artifact write paths, but a spec folder has no explicit way to declare which `rule-api` target should produce `body.md` or any named section file. If that mapping is left imp...
- ref: `memory-viewers/memory-api/.ticket/tickets/09641443-a8f2-479d-85cb-ea44a963595b/ticket.toml`

<!-- ticket-index:entry id=a5fe4c58-f59c-4d97-8ee6-3447724b5fac slug=done/spec-api digest=f20478c8d60e -->
#### [a5fe4c58] Adopt rule targets for generated spec artifacts
- priority: `high`
- summary: The shared generated-markdown builder and `spec-api` generated body/section update paths now exist, but there is still no end-to-end workflow for a spec to declare that `body.md` or `sections/*.md` s...
- ref: `memory-viewers/memory-api/.ticket/tickets/a5fe4c58-f59c-4d97-8ee6-3447724b5fac/ticket.toml`

<!-- ticket-index:entry id=f4b0be64-a2f5-4cb5-a476-b2b921d6ff02 slug=done/spec-api digest=dee64b2d72f0 -->
#### [f4b0be64] Generate spec documents from canonical snippets via shared builder
- priority: `high`
- summary: `rule-api` already knows how to collect ordered snippet content from a database-backed store, render generated markdown outputs, and rewrite files without losing the current newline convention. `spec...
- ref: `memory-viewers/memory-api/.ticket/tickets/f4b0be64-a2f5-4cb5-a476-b2b921d6ff02/ticket.toml`

<!-- ticket-index:entry id=7f869c33-15ff-4959-8161-731844eef21b slug=done/spec-api digest=6fcb1dcfc3a2 -->
#### [7f869c33] Pilot migration for rule-target-backed spec artifacts
- priority: `high`
- summary: The proposed rule-target-backed spec workflow is still theoretical until at least one real spec stops duplicating canonical prose and proves the migration path end to end.
- ref: `memory-viewers/memory-api/.ticket/tickets/7f869c33-15ff-4959-8161-731844eef21b/ticket.toml`

<!-- ticket-index:entry id=87a35ccb-d91c-4ce8-93b3-e150bb5afe1d slug=done/spec-api digest=78ae0c9182cc -->
#### [87a35ccb] [rule-cli][rule-mcp] Route spec-doc targets through spec-owned generation
- priority: `high`
- ref: `.ticket/tickets/87a35ccb-d91c-4ce8-93b3-e150bb5afe1d/ticket.toml`

<!-- ticket-index:entry id=55a1b302-9b33-4389-8962-65362b9b3eb0 slug=done/spec-api digest=fbf273e1552b -->
#### [55a1b302] [spec][P1] spec-api code references — symbol-level links to implementation files with line ranges
- priority: `high`
- summary: Implement the `CodeRef` system that links spec features to exact symbols in the implementation code with file paths and line ranges.
- ref: `memory-viewers/memory-api/.ticket/tickets/55a1b302-9b33-4389-8962-65362b9b3eb0/ticket.toml`

<!-- ticket-index:entry id=dc0df24e-075c-4147-b96c-3b26b428b0a2 slug=done/spec-api digest=ca08fd254f96 -->
#### [dc0df24e] [spec][P1] spec-api crate — umbrella ticket (manifest, slugs, folders, schema, code refs, storage)
- priority: `high`
- summary: This is the parent ticket for the spec-api crate. It tracks the execution order and dependencies of all P1 sub-tickets.
- ref: `memory-viewers/memory-api/.ticket/tickets/dc0df24e-075c-4147-b96c-3b26b428b0a2/ticket.toml`

<!-- ticket-index:entry id=4b6dc9d5-4932-4573-8635-d477804538ac slug=done/spec-api digest=da3d65d353e2 -->
#### [4b6dc9d5] [spec][P1] spec-api schema — draft/reviewed/approved/implemented/verified state machine
- priority: `high`
- summary: Define the specification type schema with a full lifecycle from draft to verified implementation.
- ref: `memory-viewers/memory-api/.ticket/tickets/4b6dc9d5-4932-4573-8635-d477804538ac/ticket.toml`

<!-- ticket-index:entry id=ab47648c-d1f8-4ad5-a652-08ef97f76ccd slug=done/spec-api digest=0592bc22f010 -->
#### [ab47648c] [spec][P1] spec-api storage — SpecStore on memory-api EntityStore with parent-child hierarchy
- priority: `high`
- summary: Build `SpecStore` on top of `memory_api::EntityStore` adding spec-specific features: parent-child hierarchy, slug uniqueness, multi-file folder support, and section management.
- ref: `memory-viewers/memory-api/.ticket/tickets/ab47648c-d1f8-4ad5-a652-08ef97f76ccd/ticket.toml`

<!-- ticket-index:entry id=ad531f63-124b-4edd-b2b3-1f8a35173649 slug=done/spec-api digest=cf667d8eb6d1 -->
#### [ad531f63] [spec][P1a] spec-api crate scaffold + SpecManifest model
- priority: `high`
- summary: Create the `crates/spec-api/` crate with its Cargo.toml and define the SpecManifest model using the same `extra: BTreeMap<String, Value>` pattern as EntityManifest/TicketManifest.
- ref: `memory-viewers/memory-api/.ticket/tickets/ad531f63-124b-4edd-b2b3-1f8a35173649/ticket.toml`

<!-- ticket-index:entry id=90c88ead-86cc-4aba-ac36-85e7355bdcce slug=done/spec-api digest=f8bb03417ce9 -->
#### [90c88ead] [spec][P1b] spec-api slug system — validation, uniqueness, resolution
- priority: `high`
- summary: Implement the slug validation and resolution system for specs. Slugs are hierarchical, human-readable identifiers (e.g. `ticket-api/storage/store`) that provide a user-friendly alternative to UUIDs.
- ref: `memory-viewers/memory-api/.ticket/tickets/90c88ead-86cc-4aba-ac36-85e7355bdcce/ticket.toml`

<!-- ticket-index:entry id=614f5f2a-3e86-412a-b4f0-d36f73935907 slug=done/spec-api digest=89fef4dba2a2 -->
#### [614f5f2a] [spec][P1c] spec-api multi-file folder structure
- priority: `high`
- summary: Extend the EntityFs pattern to support the multi-file spec folder layout. Each spec lives in a `<scan_root>/<uuid>/` directory with a defined set of files.
- ref: `memory-viewers/memory-api/.ticket/tickets/614f5f2a-3e86-412a-b4f0-d36f73935907/ticket.toml`

<!-- ticket-index:entry id=d12e6ca5-a83f-41c6-b612-219d4c2e82e3 slug=done/spec-api digest=c5e339841a32 -->
#### [d12e6ca5] [spec][P3] Spec creation — bootstrap specs for existing interfaces from code analysis
- priority: `high`
- summary: Build tooling to analyze existing Rust crate source code and generate initial spec files documenting the current implementation. This is for interfaces that are already implemented but not yet docume...
- ref: `memory-viewers/memory-api/.ticket/tickets/d12e6ca5-a83f-41c6-b612-219d4c2e82e3/ticket.toml`


### Component: spec-cli

<!-- ticket-index:entry id=b2ef1de1-5801-47c6-97c6-e3c5cd8d7dae slug=done/spec-cli digest=4ebbb85573bd -->
#### [b2ef1de1] Add spec sync-generated orchestration for rule-target-backed artifacts
- priority: `high`
- summary: Even with generated body/section APIs, there is no supported command that evaluates the declared rule targets for a spec and updates the spec store consistently. Running `rule-api` generation directl...
- ref: `memory-viewers/memory-api/.ticket/tickets/b2ef1de1-5801-47c6-97c6-e3c5cd8d7dae/ticket.toml`

<!-- ticket-index:entry id=090b6db9-f88e-418b-888e-94641d347432 slug=done/spec-cli digest=66ff556a5e94 -->
#### [090b6db9] [spec][P2] spec-cli — CRUD, search, hierarchy, health commands
- priority: `high`
- summary: Create a `spec` CLI binary with CRUD, search, hierarchy navigation, and health check commands.
- ref: `memory-viewers/memory-api/.ticket/tickets/090b6db9-f88e-418b-888e-94641d347432/ticket.toml`


### Component: spec-http

<!-- ticket-index:entry id=fc18c607-2147-4481-8a44-19bdc754f366 slug=done/spec-http digest=ecb9e72be0dc -->
#### [fc18c607] [spec][P2] spec-http — HTTP endpoints for spec-api (alongside ticket-http)
- priority: `medium`
- summary: Add HTTP endpoints for spec-api, either as part of ticket-http or as a separate spec-http crate. Routes follow the same pattern as ticket-http.
- ref: `memory-viewers/memory-api/.ticket/tickets/fc18c607-2147-4481-8a44-19bdc754f366/ticket.toml`


### Component: spec-mcp

<!-- ticket-index:entry id=fbb5a87d-44b5-4a92-8c6c-79f8302dcba5 slug=done/spec-mcp digest=bd320cf6c64a -->
#### [fbb5a87d] [spec][P2] spec-mcp — MCP tool surface for spec-api
- priority: `high`
- summary: Create MCP tools for spec-api, following the same pattern as ticket-mcp. Tools for creating, reading, updating, searching, and generating skills from specs.
- ref: `memory-viewers/memory-api/.ticket/tickets/fbb5a87d-44b5-4a92-8c6c-79f8302dcba5/ticket.toml`


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

<!-- ticket-index:entry id=995c5394-e892-4b33-870b-f53c2cff9e05 slug=done/storage digest=906b5b738ee0 -->
#### [995c5394] Impl: FsWatcher background daemon loop — auto-reconcile on FS events
- ref: `memory-viewers/memory-api/.ticket/tickets/995c5394-e892-4b33-870b-f53c2cff9e05/ticket.toml`

<!-- ticket-index:entry id=02a79934-3782-4840-bfb6-caec08ee00c7 slug=done/storage digest=dbd4f570b624 -->
#### [02a79934] Impl: enforce explicit index_root for exec/exec-batch agent protocol
- ref: `memory-viewers/memory-api/.ticket/tickets/02a79934-3782-4840-bfb6-caec08ee00c7/ticket.toml`

<!-- ticket-index:entry id=62c04e04-e976-4464-b7c9-f31bda78c0d5 slug=done/storage digest=5b84d08e7f4e -->
#### [62c04e04] Impl: expose integrate_orphan(path) as public API + wire to reconciler
- ref: `memory-viewers/memory-api/.ticket/tickets/62c04e04-e976-4464-b7c9-f31bda78c0d5/ticket.toml`

<!-- ticket-index:entry id=834fc3eb-589e-4f83-b5cd-501187aa4d5f slug=done/storage digest=f64aa569af73 -->
#### [834fc3eb] Impl: fix exec --batch — true rollback semantics on first failure
- ref: `memory-viewers/memory-api/.ticket/tickets/834fc3eb-589e-4f83-b5cd-501187aa4d5f/ticket.toml`

<!-- ticket-index:entry id=ec355bad-85cc-4ceb-8d3f-57222934e871 slug=done/storage digest=ec7401d6717a -->
#### [ec355bad] Impl: fix scan --reindex — clear stale Tantivy entries before rebuild
- ref: `memory-viewers/memory-api/.ticket/tickets/ec355bad-85cc-4ceb-8d3f-57222934e871/ticket.toml`

<!-- ticket-index:entry id=9d0258de-bf87-4b7e-b8f0-e78f4fdf0b58 slug=done/storage digest=adfb1ffc9422 -->
#### [9d0258de] [bootstrap] define backup and restore procedure for index plus history
- summary: The `.ticket/` directory has two distinct layers:
- ref: `memory-viewers/memory-api/.ticket/tickets/9d0258de-bf87-4b7e-b8f0-e78f4fdf0b58/ticket.toml`

<!-- ticket-index:entry id=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1 slug=done/storage digest=fa5c79e9a466 -->
#### [4f2d2a5e] [bootstrap] wire create/get/update/list/delete to storage backend
- summary: Status:** READY (Phase 0 formally closed)
- ref: `memory-viewers/memory-api/.ticket/tickets/4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=261e7567-e234-43d5-881b-c481e34131f8 slug=done/ticket-api digest=98684c471f3e -->
#### [261e7567] API: Add author field to ticket history revisions in ticket-api
- priority: `medium`
- ref: `memory-viewers/memory-api/.ticket/tickets/261e7567-e234-43d5-881b-c481e34131f8/ticket.toml`

<!-- ticket-index:entry id=09c5e822-740c-453e-91ae-07d01d897e15 slug=done/ticket-api digest=790d54048393 -->
#### [09c5e822] Bug: scan --force does not prune orphan entries from redb index
- priority: `medium`
- ref: `memory-viewers/memory-api/.ticket/tickets/09c5e822-740c-453e-91ae-07d01d897e15/ticket.toml`

<!-- ticket-index:entry id=b88b1fc0-eabe-444e-8511-e3467a699849 slug=done/ticket-api digest=83abade9a662 -->
#### [b88b1fc0] Phase 1: Add schema fields (doc_category, tags, workflow_stage, priority, source_agent_files, bug_validity, phase)
- summary: Add 7 new fields to `crates/ticket-api/schemas/tracker-improvement.toml` to replace 30+ ad-hoc fields with structured, filterable schema fields.
- ref: `memory-viewers/memory-api/.ticket/tickets/b88b1fc0-eabe-444e-8511-e3467a699849/ticket.toml`

<!-- ticket-index:entry id=1600e55e-1def-4e84-9f09-7b866b8ac99a slug=done/ticket-api digest=f842a30874e4 -->
#### [1600e55e] Phase 2: Copy plan descriptions into open plan tickets
- summary: For each open ticket that has a matching agent plan file but no `description.md`, copy the primary plan file as the ticket's description and set structured metadata fields.
- ref: `memory-viewers/memory-api/.ticket/tickets/1600e55e-1def-4e84-9f09-7b866b8ac99a/ticket.toml`

<!-- ticket-index:entry id=a2ebab34-3001-4fec-8454-1f74421c3049 slug=done/ticket-api digest=a1d847203f75 -->
#### [a2ebab34] Phase 3: Attach interview files as ticket assets
- summary: Copy interview files into `assets/interviews/` for their parent plan tickets. Interviews are supplementary to plans and should not be standalone tickets.
- ref: `memory-viewers/memory-api/.ticket/tickets/a2ebab34-3001-4fec-8454-1f74421c3049/ticket.toml`

<!-- ticket-index:entry id=56d080d3-011b-4eea-86a2-bb528b2d683f slug=done/ticket-api digest=6d6fb4b54160 -->
#### [56d080d3] Phase 4: Copy descriptions for bootstrap tickets
- summary: Copy research phase docs as descriptions for the 13 bootstrap tickets that lack them. Set `doc_category=research`, `workflow_stage=plan`.
- ref: `memory-viewers/memory-api/.ticket/tickets/56d080d3-011b-4eea-86a2-bb528b2d683f/ticket.toml`

<!-- ticket-index:entry id=b682a57c-8c5c-4763-af50-1c70cff2df46 slug=done/ticket-api digest=ba684c39d710 -->
#### [b682a57c] Phase 5: Enrich bug tickets with bug_validity and reproduction tracking
- summary: Enrich bug tickets with structured validity tracking and reproduction status, replacing the informal confidence emoji system.
- ref: `memory-viewers/memory-api/.ticket/tickets/b682a57c-8c5c-4763-af50-1c70cff2df46/ticket.toml`

<!-- ticket-index:entry id=5afd39bf-276c-4b3a-a1e4-f9b3b6643483 slug=done/ticket-api digest=7fcb7a40c9a2 -->
#### [5afd39bf] Phase 6: Cleanup stale tickets and deduplicate
- summary: Clean up the ticket store by cancelling stale tickets whose agent files were deleted, merging duplicates, and ensuring all tickets with descriptions have `doc_category` set.
- ref: `memory-viewers/memory-api/.ticket/tickets/5afd39bf-276c-4b3a-a1e4-f9b3b6643483/ticket.toml`

<!-- ticket-index:entry id=6bb1e3fd-646d-424c-a216-826cf5f06867 slug=done/ticket-api digest=fdbfd2fc42f2 -->
#### [6bb1e3fd] Plan: Migrate agent files into ticket system — schema improvements + content migration
- summary: Migrate the 201 agent documentation files from `agents/` into the ticket system as structured, filterable ticket content. Add schema fields so all tickets can be queried by category, tags, workflow s...
- ref: `memory-viewers/memory-api/.ticket/tickets/6bb1e3fd-646d-424c-a216-826cf5f06867/ticket.toml`

<!-- ticket-index:entry id=82652305-ab94-4270-847c-a5209c2dcd44 slug=done/ticket-api digest=dd45d2359c2b -->
#### [82652305] [spec][P0] Refactor ticket-api to depend on memory-api
- priority: `critical`
- summary: After memory-api is extracted, refactor ticket-api to be a thin domain layer on top of memory-api, keeping only ticket-specific logic.
- ref: `memory-viewers/memory-api/.ticket/tickets/82652305-ab94-4270-847c-a5209c2dcd44/ticket.toml`

<!-- ticket-index:entry id=429f6f1d-6429-4601-bfac-b572fdb4dbff slug=done/ticket-api digest=d763aa35bb34 -->
#### [429f6f1d] [ticket-api] Child workspaces surface parent dependency entries
- priority: `high`
- summary: >Backend ancestor dependency/graph visibility and workspace-aware refs validated; ready for review.
- ref: `memory-viewers/memory-api/.ticket/tickets/429f6f1d-6429-4601-bfac-b572fdb4dbff/ticket.toml`

<!-- ticket-index:entry id=3b6a2a26-bd4e-44ce-ba15-41594b809b9a slug=done/ticket-api digest=c618c5822a2f -->
#### [3b6a2a26] [ticket-api] Derive blocker and unlock trees with frontier leaf metrics
- priority: `high`
- summary: Extend the shared workflow layer with explicit blocker and unlock tree derivation.
- ref: `.ticket/tickets/3b6a2a26-bd4e-44ce-ba15-41594b809b9a/ticket.toml`

<!-- ticket-index:entry id=a4c31280-66d3-44a3-9a5d-13d4fbde1bfe slug=done/ticket-api digest=e0a6cf803f70 -->
#### [a4c31280] [ticket-api] Fix health false positives for description and resolved dependencies
- priority: `high`
- summary: Fix ticket health false positives across CLI, HTTP, and MCP surfaces.
- ref: `.ticket/tickets/a4c31280-66d3-44a3-9a5d-13d4fbde1bfe/ticket.toml`

<!-- ticket-index:entry id=3d72029b-cf2d-49bb-9dde-00587304b857 slug=done/ticket-api digest=b722d9631b9a -->
#### [3d72029b] [ticket-api] Materialize recent-unblock and blocker-progress facts
- priority: `high`
- summary: Materialize recent-unblock and blocker-progress workflow facts for scalable ordering.
- ref: `.ticket/tickets/3d72029b-cf2d-49bb-9dde-00587304b857/ticket.toml`

<!-- ticket-index:entry id=deeeb26d-cb73-46c5-bf2a-1778caa7f82a slug=done/ticket-api digest=84b7675fad41 -->
#### [deeeb26d] [ticket-api] Persist dependency edges in tracked ticket files
- priority: `high`
- summary: `ticket link` and `ticket unlink` currently mutate dependency edges only in the ignored `.ticket/tickets.db` SQLite index.
- ref: `memory-viewers/memory-api/.ticket/tickets/deeeb26d-cb73-46c5-bf2a-1778caa7f82a/ticket.toml`

<!-- ticket-index:entry id=dd2947da-d4d2-4c8a-9a9a-3633060ff4c5 slug=done/ticket-api digest=f4b8498bffc3 -->
#### [dd2947da] [ticket-api] Reconcile aggregate scan, prune, and search visibility
- priority: `high`
- summary: Make aggregate scan the single source of truth for both index visibility and search visibility.
- ref: `.ticket/tickets/dd2947da-d4d2-4c8a-9a9a-3633060ff4c5/ticket.toml`

<!-- ticket-index:entry id=d74412e4-1d0e-4679-8725-e5da6f266fe9 slug=done/ticket-api digest=a508ffe796b4 -->
#### [d74412e4] [ticket-api][ticket-cli] Blueprint blocker trees and recently-unblocked workflow ordering
- priority: `high`
- summary: Create the full implementation blueprint for blocker-tree workflow exploration and recently-unblocked ordering.
- ref: `.ticket/tickets/d74412e4-1d0e-4679-8725-e5da6f266fe9/ticket.toml`

<!-- ticket-index:entry id=d1f9f390-dda0-4762-a14c-9ce339abc393 slug=done/ticket-api digest=ec1f947872a0 -->
#### [d1f9f390] [ticket-api][ticket-cli][ticket-mcp] Redesign best-next ranking around dependency convergence
- priority: `high`
- summary: `ticket next`, `ticket board show`, and `ticket-mcp next_tickets` currently use the dependees-first contract documented in the current best-next spec: candidate workflow progress, then priority, then...
- ref: `memory-viewers/memory-api/.ticket/tickets/d1f9f390-dda0-4762-a14c-9ce339abc393/ticket.toml`

<!-- ticket-index:entry id=c031aeb0-f374-4d57-9d46-2463dfa8571d slug=done/ticket-api digest=cf1e688e9cff -->
#### [c031aeb0] [ticket-api][ticket-cli][ticket-mcp][ticket-http] Define minimal workflow and health core plus adapter responsibilities
- priority: `high`
- summary: The same ticket store can still produce different workflow and health answers because parity-critical domain behavior is split across the interface crates.
- ref: `.ticket/tickets/c031aeb0-f374-4d57-9d46-2463dfa8571d/ticket.toml`

<!-- ticket-index:entry id=0e375356-b74e-48c4-8f1d-77cd28e055bc slug=done/ticket-api digest=c43ce9651b18 -->
#### [0e375356] [ticket-api][ticket-cli][ticket-mcp][ticket-http] Implement scoped selectors for board and next
- priority: `high`
- summary: The workflow discovery surfaces currently build scope locally and inconsistently.
- ref: `.ticket/tickets/0e375356-b74e-48c4-8f1d-77cd28e055bc/ticket.toml`

<!-- ticket-index:entry id=cf4246c3-6539-4f1c-a876-6d34073db7b3 slug=done/ticket-api digest=a669d9ac623c -->
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

<!-- ticket-index:entry id=11450369-0d45-4922-988f-49bc88fd4079 slug=done/ticket-api digest=e9c5485bd74f -->
#### [11450369] [ticket-cli] Render board show recommendations as pretty cards
- priority: `high`
- summary: Render recommendation lists with the same compact pretty-card layout anywhere the CLI shows human-readable next-work candidates.
- ref: `memory-viewers/memory-api/.ticket/tickets/11450369-0d45-4922-988f-49bc88fd4079/ticket.toml`

<!-- ticket-index:entry id=6484d4b7-e24b-4c13-999c-d0b00928d97c slug=done/ticket-api digest=5d0d8e5daefc -->
#### [6484d4b7] [ticket-cli][ticket-http][ticket-mcp] Build larger-integration parity routine for workflow and health surfaces
- priority: `high`
- summary: The current validation approach mostly proves ticket-cli, ticket-http, and ticket-mcp in isolation.
- ref: `.ticket/tickets/6484d4b7-e24b-4c13-999c-d0b00928d97c/ticket.toml`

<!-- ticket-index:entry id=2d85467b-23a3-4a70-a376-70ef5370d9f8 slug=done/ticket-api digest=aa2ba39d0d8b -->
#### [2d85467b] [ticket-system] Add dependees to next-ticket ordering
- priority: `high`
- summary: Add an incoming-dependees ranking key to best-next ticket selection. Compute dependees as the count of depends_on edges whose target is the candidate ticket. Keep workflow progress first and priority...
- ref: `memory-viewers/memory-api/.ticket/tickets/2d85467b-23a3-4a70-a376-70ef5370d9f8/ticket.toml`

<!-- ticket-index:entry id=77629631-8076-4fca-9640-316583ff290c slug=done/ticket-api digest=2d32d655fe1b -->
#### [77629631] [ticket-system] Expose ordering keys on priority-sorted outputs
- priority: `high`
- summary: Expose the full best-next ordering metadata anywhere the CLI surfaces the priority-sorted board recommendation list. Preserve dependees and created_at when board show rewraps next candidates, and ren...
- ref: `memory-viewers/memory-api/.ticket/tickets/77629631-8076-4fca-9640-316583ff290c/ticket.toml`

<!-- ticket-index:entry id=a62f28bd-8600-473f-a831-de4736ffc219 slug=done/ticket-api digest=d2091bd4545f -->
#### [a62f28bd] [ticket-system] Per-ticket workflow paths: required_states field
- priority: `high`
- ref: `memory-viewers/memory-api/.ticket/tickets/a62f28bd-8600-473f-a831-de4736ffc219/ticket.toml`


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

<!-- ticket-index:entry id=6f8bcf0a-c5e3-423c-b3cd-190a5bb0b18f slug=done/ticket-cli digest=82a6c4ba8e52 -->
#### [6f8bcf0a] Plan: ticket attach — asset file management for tickets
- priority: `medium`
- summary: Tickets can have an `assets/` directory for supplementary files (interview transcripts,
- ref: `memory-viewers/memory-api/.ticket/tickets/6f8bcf0a-c5e3-423c-b3cd-190a5bb0b18f/ticket.toml`

<!-- ticket-index:entry id=b0056fa6-bdb3-40a6-acfb-9c96dd1ca82f slug=done/ticket-cli digest=92dbfb9cd739 -->
#### [b0056fa6] Plan: ticket audit — store health check and statistics
- priority: `medium`
- summary: After the migration, a 90-line Python script was needed to audit the ticket store:
- ref: `memory-viewers/memory-api/.ticket/tickets/b0056fa6-bdb3-40a6-acfb-9c96dd1ca82f/ticket.toml`

<!-- ticket-index:entry id=b11cde49-100f-4443-95d9-6d9c30d21622 slug=done/ticket-cli digest=b85d523ac653 -->
#### [b11cde49] Plan: ticket batch-update — bulk field updates with filter
- priority: `medium`
- summary: The existing `batch` command takes NDJSON `TaskCommand` objects, which is powerful
- ref: `memory-viewers/memory-api/.ticket/tickets/b11cde49-100f-4443-95d9-6d9c30d21622/ticket.toml`

<!-- ticket-index:entry id=f77ff07e-c250-4740-8da8-8cf065564f8a slug=done/ticket-cli digest=4b2459dd52bb -->
#### [f77ff07e] Plan: ticket close / state fast-forward — skip intermediate states
- priority: `high`
- summary: The tracker-improvement schema has 11 states with a strict transition chain:
- ref: `memory-viewers/memory-api/.ticket/tickets/f77ff07e-c250-4740-8da8-8cf065564f8a/ticket.toml`

<!-- ticket-index:entry id=0c660fde-39ed-4d59-9ddf-f9d98c2b7740 slug=done/ticket-cli digest=52ecfd208c28 -->
#### [0c660fde] Plan: ticket list --include-deleted — visibility into soft-deleted tickets
- priority: `high`
- summary: Soft-deleted tickets (`deleted = true`) are completely invisible to the CLI:
- ref: `memory-viewers/memory-api/.ticket/tickets/0c660fde-39ed-4d59-9ddf-f9d98c2b7740/ticket.toml`

<!-- ticket-index:entry id=a48475e3-42fc-44f6-88b2-0f4a86930a31 slug=done/ticket-cli digest=140368917393 -->
#### [a48475e3] Plan: ticket list --where — structured field-value filtering
- priority: `high`
- summary: `ticket list` currently supports `--state` and `--type` filters only.
- ref: `memory-viewers/memory-api/.ticket/tickets/a48475e3-42fc-44f6-88b2-0f4a86930a31/ticket.toml`

<!-- ticket-index:entry id=d39e9e08-5104-461b-83ff-bd4361e967d9 slug=done/ticket-cli digest=5fc71c9f4ab9 -->
#### [d39e9e08] [ticket-cli] Add blockers command and nested tree rendering
- priority: `high`
- summary: Add an upstream `ticket blockers <id>` command and upgrade `ticket unblocked-by <id>` to nested tree output.
- ref: `.ticket/tickets/d39e9e08-5104-461b-83ff-bd4361e967d9/ticket.toml`

<!-- ticket-index:entry id=40282486-bd98-4f3b-8bb5-96cfe853e247 slug=done/ticket-cli digest=d8565dd48879 -->
#### [40282486] [ticket-cli] Add reverse-dependency follow-up queries for next and unblocked-by
- priority: `high`
- summary: Users can ask which tickets were unblocked by finishing a dependency, but the current CLI requires a manual topgraph plus per-ticket health fan-out to answer that question.
- ref: `memory-viewers/memory-api/.ticket/tickets/40282486-bd98-4f3b-8bb5-96cfe853e247/ticket.toml`

<!-- ticket-index:entry id=8de93812-3a8c-4937-9f09-05a9a9b86309 slug=done/ticket-cli digest=5ae851523a53 -->
#### [8de93812] [ticket-cli] Canonicalize board subcommand option naming
- priority: `medium`
- ref: `.ticket/tickets/8de93812-3a8c-4937-9f09-05a9a9b86309/ticket.toml`

<!-- ticket-index:entry id=15837e16-8755-4eb1-8b36-6c4453899e46 slug=done/ticket-cli digest=c40a48b4a4f8 -->
#### [15837e16] [ticket-cli][ticket-mcp] Integrate recent-unblock ordering into workflow surfaces
- priority: `high`
- summary: Integrate recent-unblock ordering and tree metadata into prioritized workflow surfaces.
- ref: `.ticket/tickets/15837e16-8755-4eb1-8b36-6c4453899e46/ticket.toml`

<!-- ticket-index:entry id=129d4f4e-7db8-4c3b-87d5-de8ed12c0b09 slug=done/ticket-cli digest=2b4eabc69131 -->
#### [129d4f4e] [ticket-system] Next command: sort by workflow progress
- priority: `medium`
- ref: `memory-viewers/memory-api/.ticket/tickets/129d4f4e-7db8-4c3b-87d5-de8ed12c0b09/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=8034efd8-e165-4798-afe1-3445026345d9 slug=done/ticket-http digest=19f42e4bd345 -->
#### [8034efd8] API: Batch mutation endpoint for transactional multi-command execution
- priority: `high`
- ref: `memory-viewers/memory-api/.ticket/tickets/8034efd8-e165-4798-afe1-3445026345d9/ticket.toml`

<!-- ticket-index:entry id=15871ee6-8e6b-40a0-8293-46d31deae3e8 slug=done/ticket-http digest=ce93bc6bb96f -->
#### [15871ee6] API: Edge mutation endpoints — add and remove edges
- priority: `high`
- ref: `memory-viewers/memory-api/.ticket/tickets/15871ee6-8e6b-40a0-8293-46d31deae3e8/ticket.toml`

<!-- ticket-index:entry id=3fda11c3-978b-4f7c-9ee1-934a97debb12 slug=done/ticket-http digest=09607044e609 -->
#### [3fda11c3] API: History and revert endpoints
- priority: `medium`
- ref: `memory-viewers/memory-api/.ticket/tickets/3fda11c3-978b-4f7c-9ee1-934a97debb12/ticket.toml`

<!-- ticket-index:entry id=189a6068-7ccc-4daf-808e-6b0b82e97ef5 slug=done/ticket-http digest=64a23dd7d198 -->
#### [189a6068] API: Schema endpoint — types, states, transitions, fields
- priority: `high`
- ref: `memory-viewers/memory-api/.ticket/tickets/189a6068-7ccc-4daf-808e-6b0b82e97ef5/ticket.toml`

<!-- ticket-index:entry id=69abd1c7-15a9-4d56-8156-0f09ff90783f slug=done/ticket-http digest=e38881392ee4 -->
#### [69abd1c7] API: Ticket mutation endpoints — create, update, close, cancel, delete
- priority: `critical`
- ref: `memory-viewers/memory-api/.ticket/tickets/69abd1c7-15a9-4d56-8156-0f09ff90783f/ticket.toml`

<!-- ticket-index:entry id=d3a8b66a-8efc-493e-9993-3b5a68b0a7f7 slug=done/ticket-http digest=e5e030c917bc -->
#### [d3a8b66a] Impl: Add created_at to TicketSummary HTTP response
- summary: Add `created_at` field to the `TicketSummary` struct in the ticket-http handler so the frontend can sort tickets by creation date.
- ref: `memory-viewers/memory-api/.ticket/tickets/d3a8b66a-8efc-493e-9993-3b5a68b0a7f7/ticket.toml`

<!-- ticket-index:entry id=700b9763-17f8-436e-ace0-45b88bedd1d7 slug=done/ticket-http digest=f9deffa9f41d -->
#### [700b9763] [ticket-http] Design: workspace-aware ticket references for child-workspace frontend endpoints
- priority: `high`
- summary: >Validated workspace-aware HTTP contract and ancestor-owned ref behavior; ready for review.
- ref: `memory-viewers/memory-api/.ticket/tickets/700b9763-17f8-436e-ace0-45b88bedd1d7/ticket.toml`

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

<!-- ticket-index:entry id=58fe9f39-50c2-4e1c-8bdc-336ed5d6da6e slug=done/ticket-mcp digest=df530d414107 -->
#### [58fe9f39] Plan: ticket-mcp write tools — update, close, and batch operations via MCP
- priority: `high`
- summary: The ticket-mcp server currently exposes only read-only tools:
- ref: `memory-viewers/memory-api/.ticket/tickets/58fe9f39-50c2-4e1c-8bdc-336ed5d6da6e/ticket.toml`


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

<!-- ticket-index:entry id=b00b945b-045f-4124-9c69-ea15346b144f slug=done/ticket-viewer digest=cb4ef052625a -->
#### [b00b945b] [ticket-viewer] Fix list-driven content panel selection sync
- priority: `high`
- summary: Opening a different ticket from the sidebar could leave the content panel showing the previous ticket body because the content component kept its state across list-driven selection changes.
- ref: `memory-viewers/memory-api/.ticket/tickets/b00b945b-045f-4124-9c69-ea15346b144f/ticket.toml`

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

<!-- ticket-index:entry id=eeda4039-d82d-4573-9d79-0bc89e152a76 slug=done/ticket-viewer digest=cea5fccaca77 -->
#### [eeda4039] [ticket-viewer][viewer-api] Add kanban table graph layout mode
- priority: `high`
- summary: The current default graph layouts are tuned for planar hierarchy views, but they do not present ticket dependency trees in a way that matches ticket workflow review. Dense ticket sets with mixed stat...
- ref: `memory-viewers/memory-api/.ticket/tickets/eeda4039-d82d-4573-9d79-0bc89e152a76/ticket.toml`

<!-- ticket-index:entry id=d1d38010-08b8-4a06-ad2b-0bbed453c941 slug=done/ticket-viewer digest=73953bc68ff9 -->
#### [d1d38010] [ticket-viewer][viewer-api] Preserve graph layout and camera across same-graph refreshes
- priority: `high`
- summary: The shared Graph3D surface preserves dragged node positions only until the next same-graph re-render. Once the ticket-viewer reuses the same workspace graph payload or changes node focus, the shared ...
- ref: `memory-viewers/memory-api/.ticket/tickets/d1d38010-08b8-4a06-ad2b-0bbed453c941/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=0b231549-d029-4465-997e-0ba4d5e0529e slug=done/ticket-vscode digest=e8ab4917f711 -->
#### [0b231549] [spec][vscode] Write specification for ticket-vscode extension
- priority: `high`
- summary: Produce a complete design specification for the existing `ticket-vscode` VS Code extension (`tools/ticket-vscode/`). The spec must be detailed enough to serve as the architectural reference model whe...
- ref: `memory-viewers/memory-api/.ticket/tickets/0b231549-d029-4465-997e-0ba4d5e0529e/ticket.toml`

<!-- ticket-index:entry id=362448d4-ccf1-4b9d-90f3-d4577da83a65 slug=done/ticket-vscode digest=fe5fc8400049 -->
#### [362448d4] [ticket-vscode] Add dual-host packaging, bundling, and extension test harnesses
- priority: `high`
- summary: Package the ported extension so it activates in both the desktop/remote Node host and the web extension host, and add the harnesses that validate both.
- ref: `memory-viewers/memory-api/.ticket/tickets/362448d4-ccf1-4b9d-90f3-d4577da83a65/ticket.toml`

<!-- ticket-index:entry id=011563c2-59e7-48f1-a61f-d8fdc80d2f6e slug=done/ticket-vscode digest=b6cc51af61a7 -->
#### [011563c2] [ticket-vscode] Extract portable Rust core for ticket/domain logic
- priority: `high`
- summary: Move deterministic, serializable logic out of the current TypeScript extension into a new Rust core that is compiled to WASM and driven by host-provided data.
- ref: `memory-viewers/memory-api/.ticket/tickets/011563c2-59e7-48f1-a61f-d8fdc80d2f6e/ticket.toml`

<!-- ticket-index:entry id=93f7e422-1e41-4145-b8ba-0dcf7fc730ac slug=done/ticket-vscode digest=0b09b8a8c1be -->
#### [93f7e422] [ticket-vscode] Freeze Rust/WASM architecture spec and feature matrix
- priority: `high`
- summary: Use the new planning spec `ticket-vscode/rust-wasm-port` as the canonical design surface for the migration.
- ref: `memory-viewers/memory-api/.ticket/tickets/93f7e422-1e41-4145-b8ba-0dcf7fc730ac/ticket.toml`

<!-- ticket-index:entry id=8735fa5d-0550-40f1-9ee8-7b83a44a7fd1 slug=done/ticket-vscode digest=20334d246d75 -->
#### [8735fa5d] [ticket-vscode] Prefer PATH ticket-viewer before debug fallback
- priority: `medium`
- summary: Auto-start in `tools/ticket-vscode` should prefer the `ticket-viewer` executable on `PATH` before falling back to a workspace-local `target/debug/ticket-viewer(.exe)` binary. This keeps the extension...
- ref: `.ticket/tickets/8735fa5d-0550-40f1-9ee8-7b83a44a7fd1/ticket.toml`

<!-- ticket-index:entry id=14047b99-41d6-4899-bec6-4a919bffcc2d slug=done/ticket-vscode digest=224dc18b0187 -->
#### [14047b99] [ticket-vscode] Prove dual-host WASM activation
- priority: `high`
- summary: Build a narrow architecture spike that proves a Rust/WASM module can be loaded by both VS Code extension hosts used by this port.
- ref: `memory-viewers/memory-api/.ticket/tickets/14047b99-41d6-4899-bec6-4a919bffcc2d/ticket.toml`

<!-- ticket-index:entry id=bfafde19-ddf7-47ef-966e-a1135be4efd6 slug=done/ticket-vscode digest=8dea2e68be92 -->
#### [bfafde19] [ticket-vscode] Replace Node-bound behaviors with host capability adapters
- priority: `high`
- summary: Refactor the extension host layer so runtime-specific behavior is isolated behind explicit capabilities instead of being embedded throughout `extensionSupport.ts`, `extensionCommands.ts`, and `ticket...
- ref: `memory-viewers/memory-api/.ticket/tickets/bfafde19-ddf7-47ef-966e-a1135be4efd6/ticket.toml`

<!-- ticket-index:entry id=4842a2bd-e94d-4066-801e-8883cbc18cab slug=done/ticket-vscode digest=60a1d1be1d7b -->
#### [4842a2bd] ticket-vscode: Auto-detect .ticket workspace from open VS Code folders
- priority: `high`
- summary: The ticket-vscode extension currently resolves the ticket workspace through a hardcoded chain:
- ref: `memory-viewers/memory-api/.ticket/tickets/4842a2bd-e94d-4066-801e-8883cbc18cab/ticket.toml`

<!-- ticket-index:entry id=dbca2bab-77bb-4f58-8460-3714f3d07004 slug=done/ticket-vscode digest=934ed00dda6b -->
#### [dbca2bab] ticket-vscode: Auto-start ticket-viewer server on extension activation
- priority: `medium`
- summary: Currently, users must manually click the ▶ button in the Tickets sidebar or run the "Start Ticket Viewer Server" command before the tree view can display tickets. If the server is not running, the tr...
- ref: `memory-viewers/memory-api/.ticket/tickets/dbca2bab-77bb-4f58-8460-3714f3d07004/ticket.toml`

<!-- ticket-index:entry id=576c5f77-b261-42aa-a3f0-fd2f9597520e slug=done/ticket-vscode digest=39228223e142 -->
#### [576c5f77] ticket-vscode: Navigate to ticket URL in Simple Browser on click
- priority: `medium`
- summary: When a user clicks a ticket in the tree view, the current behavior opens the ticket-viewer root URL and copies the ticket ID to the clipboard. The user must then manually paste and search for the tic...
- ref: `memory-viewers/memory-api/.ticket/tickets/576c5f77-b261-42aa-a3f0-fd2f9597520e/ticket.toml`

<!-- ticket-index:entry id=5b330dd5-2dcc-4460-b468-43ff4c35bfba slug=done/ticket-vscode digest=965dafd88e96 -->
#### [5b330dd5] ticket-vscode: Open clicked tickets in viewer and expose Copy ID in context menu
- priority: `medium`
- summary: The current `ticket-vscode` tree click behavior still routes through `ticket-viewer.openTicket`, which prefers opening `description.md` when the local ticket folder exists. That means clicking a tick...
- ref: `memory-viewers/memory-api/.ticket/tickets/5b330dd5-2dcc-4460-b468-43ff4c35bfba/ticket.toml`

<!-- ticket-index:entry id=a7d6ba2d-ea15-498f-9195-6ee775ea69a4 slug=done/ticket-vscode digest=4eccde7ea8ab -->
#### [a7d6ba2d] ticket-vscode: Replace native tooltip with debounced webview panel beside sidebar
- priority: `high`
- summary: The current hover tooltip on ticket tree items appears too quickly (VS Code's default ~500ms) and is positioned by VS Code at the cursor, which can obscure the tree view. Users want a more relaxed ho...
- ref: `memory-viewers/memory-api/.ticket/tickets/a7d6ba2d-ea15-498f-9195-6ee775ea69a4/ticket.toml`

<!-- ticket-index:entry id=207b70d9-de61-4de4-af80-69732ff5b892 slug=done/ticket-vscode digest=0bcd8fef97ca -->
#### [207b70d9] ticket-vscode: Show ticket description in hover tooltip
- priority: `medium`
- summary: Currently, hovering over a ticket in the tree view shows only basic metadata: title, ID, state, and type. The ticket description (which contains context, acceptance criteria, and implementation detai...
- ref: `memory-viewers/memory-api/.ticket/tickets/207b70d9-de61-4de4-af80-69732ff5b892/ticket.toml`


### Component: ticket-workflow

<!-- ticket-index:entry id=790df512-d8a9-42bd-b3d6-6e2b4d5eda9c slug=done/ticket-workflow digest=b8af2ffd0348 -->
#### [790df512] [spec][ticket-workflow] Specify scoped selector contract for board and next
- priority: `high`
- summary: `ticket board show`, `ticket next`, MCP `board_show` / `next_tickets`, and HTTP `/api/workflow/next` expose only narrow, inconsistent scoping knobs.
- ref: `.ticket/tickets/790df512-d8a9-42bd-b3d6-6e2b4d5eda9c/ticket.toml`


### Component: tooling

<!-- ticket-index:entry id=d30e13e1-3304-4128-9653-be7c47679f9f slug=done/tooling digest=45f4455dada8 -->
#### [d30e13e1] [install-tools] Install all viewer binaries
- priority: `high`
- summary: Update install-tools.sh so the root installer refreshes doc-viewer, log-viewer, spec-viewer, and ticket-viewer PATH binaries. Keep ticket-vscode PATH-first launch behavior unchanged; solve the stale-...
- ref: `.ticket/tickets/d30e13e1-3304-4128-9653-be7c47679f9f/ticket.toml`


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

<!-- ticket-index:entry id=b2392ff7-6b7f-4b48-99b7-adf1142a3fc0 slug=done/unspecified digest=44aef20e99dd -->
#### [b2392ff7] Add repository QA MCP audit tool
- summary: Add a single-endpoint Rust tool for agents to audit repository quality and track findings in a synchronized local database.
- ref: `memory-viewers/memory-api/.ticket/tickets/b2392ff7-6b7f-4b48-99b7-adf1142a3fc0/ticket.toml`

<!-- ticket-index:entry id=4c937720-4e35-4db6-bce7-608fdad5b6c5 slug=done/unspecified digest=f213adaa6c71 -->
#### [4c937720] Add reviews prompt for ranked in-review ticket review
- priority: `medium`
- summary: Create a reusable rule-generated prompt in `.agents/prompts/reviews.prompt.md` that reviews the highest-ranked `in-review` tickets using the ticket system.
- ref: `.ticket/tickets/4c937720-4e35-4db6-bce7-608fdad5b6c5/ticket.toml`

<!-- ticket-index:entry id=dcc6ad3f-6266-4940-96e1-2f26656cae57 slug=done/unspecified digest=f98c32730e0f -->
#### [dcc6ad3f] Advanced Voxel Tools: Fill, Smooth, Extrude, Clone
- priority: `high`
- summary: Beyond basic paint/carve, the world editor needs advanced manipulation tools: flood-fill enclosed regions, smooth surfaces (averaging neighbors), extrude faces outward, and clone/stamp regions.
- ref: `.ticket/tickets/dcc6ad3f-6266-4940-96e1-2f26656cae57/ticket.toml`

<!-- ticket-index:entry id=1211b2d8-93f9-4b1a-8973-10ee9937ba3d slug=done/unspecified digest=8688a662109a -->
#### [1211b2d8] Combat System: SDF Hit Detection, Damage Model & Voxel Destruction VFX
- priority: `high`
- summary: Combat in this RPG uses real-time SDF collision between weapon swings and player/NPC capsules. There are no hitboxes or animation frames — a weapon's SDF sweep volume is tested against target SDFs on...
- ref: `.ticket/tickets/1211b2d8-93f9-4b1a-8973-10ee9937ba3d/ticket.toml`

<!-- ticket-index:entry id=86b0a60e-9e9b-41c5-ba1e-a0f372587dbe slug=done/unspecified digest=8a1b6e687c3f -->
#### [86b0a60e] Core Voxel Editor: Paint, Carve, and Ray-Octree Intersection
- priority: `high`
- summary: The minimum viable world editor: paint voxels onto surfaces, carve voxels away, and ray-cast to find which voxel the cursor is pointing at. Edits flow through VoxelWorld → double buffer → splat regen...
- ref: `.ticket/tickets/86b0a60e-9e9b-41c5-ba1e-a0f372587dbe/ticket.toml`

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

<!-- ticket-index:entry id=3fec54f1-9c8f-4059-a366-7da6e9a1a645 slug=done/unspecified digest=62c75245dbef -->
#### [3fec54f1] Force Compute Shader & SVO Collision
- priority: `high`
- summary: Particles must respond to complex physical forces (explosions, attraction, vortices) efficiently on the GPU. Furthermore, they need to physically collide with the Sparse Voxel Octree (SVO), bouncing ...
- ref: `.ticket/tickets/3fec54f1-9c8f-4059-a366-7da6e9a1a645/ticket.toml`

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

<!-- ticket-index:entry id=bce0d2fb-8ac3-4be3-af42-47f5b6928caa slug=done/unspecified digest=652d61cc6ed8 -->
#### [bce0d2fb] Impl: Extract HypergraphViewCore to viewer-api as primary graph component
- ref: `memory-viewers/viewer-api/.ticket/tickets/bce0d2fb-8ac3-4be3-af42-47f5b6928caa/ticket.toml`

<!-- ticket-index:entry id=367335ee-22bd-4cde-aa6a-312e80702c19 slug=done/unspecified digest=a9c73e500cf6 -->
#### [367335ee] Impl: URL hash routing for ticket-viewer (workspace + ticket ID)
- ref: `memory-viewers/.ticket/tickets/367335ee-22bd-4cde-aa6a-312e80702c19/ticket.toml`

<!-- ticket-index:entry id=f4f5da07-8889-42ed-b32d-8638e811be76 slug=done/unspecified digest=f83232b4f22c -->
#### [f4f5da07] Improve board immediate action wording
- summary: Adjust `ticket board show` immediate action text so the suggested ticket includes its current state immediately after `Start`, wraps the title in quotes, and escapes inner quotes correctly.
- ref: `.ticket/tickets/f4f5da07-8889-42ed-b32d-8638e811be76/ticket.toml`

<!-- ticket-index:entry id=75b12c5a-a0cf-4810-b5b6-a65319dc95a7 slug=done/unspecified digest=73fa6f9a04d0 -->
#### [75b12c5a] Motion-Blurred Particle Splatting
- priority: `high`
- summary: Rendering hundreds of thousands of particles conventionally as point sprites creates hard edges that don't mix gracefully within the soft, liquid-glass/Voxel SDF aesthetic. Particles need volumetric ...
- ref: `.ticket/tickets/75b12c5a-a0cf-4810-b5b6-a65319dc95a7/ticket.toml`

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

<!-- ticket-index:entry id=c7659e24-3687-4581-bc4c-54bfc7e19267 slug=done/unspecified digest=b1739a933d95 -->
#### [c7659e24] Render Graph + Pipeline: Custom Nodes, Canvas Setup, and Mipmap Generation
- priority: `high`
- summary: Bevy's render graph must host 7 custom nodes executing in sequence, plus canvas/WebGPU initialization and mipmap generation for frosted glass. This ticket wires the render graph — it does NOT impleme...
- ref: `.ticket/tickets/c7659e24-3687-4581-bc4c-54bfc7e19267/ticket.toml`

<!-- ticket-index:entry id=d54f034c-b6ab-4c8d-bb81-a287d05834a1 slug=done/unspecified digest=c33be0af621c -->
#### [d54f034c] Simplify ticket state machine: drop in-refinement and in-validation, enforce e2e testing and explicit user review
- priority: `high`
- summary: The current `tracker-improvement` state machine has **8 states** with 6
- ref: `.ticket/tickets/d54f034c-b6ab-4c8d-bb81-a287d05834a1/ticket.toml`

<!-- ticket-index:entry id=89c3189b-381d-4020-8757-39a675791c20 slug=done/unspecified digest=367f6603306e -->
#### [89c3189b] Skill System: Spell SDFs, Procedural Shader Effects & Volumetric Magic
- priority: `high`
- summary: Magic spells in this RPG are not pre-canned animations — they are transient SDF volumes injected into the ray-marching loop, generating real-time volumetric lighting, refraction, and physical effects...
- ref: `.ticket/tickets/89c3189b-381d-4020-8757-39a675791c20/ticket.toml`

<!-- ticket-index:entry id=5070c6b3-a37a-47fa-8dcf-69f805c1a2d2 slug=done/unspecified digest=f2a0d9db1b30 -->
#### [5070c6b3] Sort Key Construction & Tiled Depth Ordering for Voxel Splats
- priority: `high`
- summary: The second rendering stage: project each `VoxelSplat`'s bounding box to screen-space, compute tile membership, and construct composite sort keys `(tile_id << 12) | depth` for the GPU radix sort (T6c)...
- ref: `.ticket/tickets/5070c6b3-a37a-47fa-8dcf-69f805c1a2d2/ticket.toml`

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

<!-- ticket-index:entry id=854f0e8f-c881-48a5-a8bc-a6f7ac3092a9 slug=done/unspecified digest=a51461a27eb0 -->
#### [854f0e8f] [Board] Draftboard — workspace WIP coordination for concurrent agents
- priority: `high`
- summary: Provide a workspace-global, short-term "daily planning board" that tracks the current state of all active work across concurrent agent sessions. The draftboard fills the gap between ephemeral user pr...
- ref: `memory-viewers/memory-api/.ticket/tickets/854f0e8f-c881-48a5-a8bc-a6f7ac3092a9/ticket.toml`

<!-- ticket-index:entry id=74160bb8-ac9c-4fd6-82e4-2e392d96e48b slug=done/unspecified digest=ad6276ecd576 -->
#### [74160bb8] [Board] Integrate draftboard state into next and status commands
- priority: `medium`
- summary: Make the existing `ticket next` and `ticket status` commands draftboard-aware so that agents receive board context automatically, without needing to call `board show` separately. This is the integrat...
- ref: `memory-viewers/memory-api/.ticket/tickets/74160bb8-ac9c-4fd6-82e4-2e392d96e48b/ticket.toml`

<!-- ticket-index:entry id=b72b0a40-496e-43d0-a5b3-ec358d85802b slug=done/unspecified digest=1501118b669d -->
#### [b72b0a40] [Board] ticket-api: Cleanup, file ops, reconciliation, claim deprecation
- priority: `high`
- summary: Add the operational maintenance layer to the draftboard in `crates/ticket-api/`. This builds on the core board storage (types, tables, check-in/out/heartbeat/show/configure) established by `0db86ac1`...
- ref: `memory-viewers/memory-api/.ticket/tickets/b72b0a40-496e-43d0-a5b3-ec358d85802b/ticket.toml`

<!-- ticket-index:entry id=0db86ac1-45ca-49a6-abc7-dd30b5adbee7 slug=done/unspecified digest=25f1ffc64c36 -->
#### [0db86ac1] [Board] ticket-api: Core board storage — types, tables, CRUD
- priority: `high`
- summary: Implement the foundational draftboard data layer in `crates/ticket-api/`. This ticket covers core types, redb tables, and the primary board operations (check-in, check-out, heartbeat, show, configure...
- ref: `memory-viewers/memory-api/.ticket/tickets/0db86ac1-45ca-49a6-abc7-dd30b5adbee7/ticket.toml`

<!-- ticket-index:entry id=bcc111c6-5034-4259-b8cd-3a4dacf3113a slug=done/unspecified digest=b12d9793947e -->
#### [bcc111c6] [Board] ticket-cli: board subcommand family (show, check-in, check-out, heartbeat, clean)
- priority: `medium`
- summary: Expose all draftboard operations as `ticket board <subcommand>` in the CLI. This is the primary human and agent interface for draftboard coordination. All subcommands follow the existing CLI conventi...
- ref: `memory-viewers/memory-api/.ticket/tickets/bcc111c6-5034-4259-b8cd-3a4dacf3113a/ticket.toml`

<!-- ticket-index:entry id=ec52f7cb-7c5e-4854-84d3-80618167762d slug=done/unspecified digest=2ef79e204bf0 -->
#### [ec52f7cb] [Board] ticket-mcp: Board tool endpoints for agent coordination
- priority: `medium`
- summary: Expose the draftboard as MCP tools so that agent sessions can coordinate through the MCP protocol without shelling out to the CLI. This is the primary machine interface for agent-to-board interaction...
- ref: `memory-viewers/memory-api/.ticket/tickets/ec52f7cb-7c5e-4854-84d3-80618167762d/ticket.toml`

<!-- ticket-index:entry id=8aff39cb-2480-4610-9593-2e4e6d96d265 slug=done/unspecified digest=34dcabed96a4 -->
#### [8aff39cb] [Board][Design] Draftboard data model, API contract, and CLI/MCP surface
- priority: `high`
- summary: Produce the implementation-ready contract for the draftboard system: data model, store API, CLI subcommand surface, and MCP tool definitions. This design must be approved before any implementation be...
- ref: `memory-viewers/memory-api/.ticket/tickets/8aff39cb-2480-4610-9593-2e4e6d96d265/ticket.toml`

<!-- ticket-index:entry id=84ceb9ce-ce68-4473-ac11-9724a20283ce slug=done/unspecified digest=0841446d83b1 -->
#### [84ceb9ce] [Board][Design] Entry identity, resume flow, and synchronization invariants
- priority: `high`
- summary: Close the remaining correctness gaps around what a draftboard entry actually represents, how an agent resumes existing work, and how board state stays synchronized with leases and ticket state transi...
- ref: `memory-viewers/memory-api/.ticket/tickets/84ceb9ce-ce68-4473-ac11-9724a20283ce/ticket.toml`

<!-- ticket-index:entry id=c3143e3c-2d16-447a-9062-14305a31b786 slug=done/unspecified digest=244e1970a43d -->
#### [c3143e3c] [Board][Design] Stale-entry review, cleanup approval, and conflict resolution workflow
- priority: `high`
- summary: Define the human-in-the-loop workflow for stale entries, explicit cleanup, and file ownership conflicts.
- ref: `memory-viewers/memory-api/.ticket/tickets/c3143e3c-2d16-447a-9062-14305a31b786/ticket.toml`

<!-- ticket-index:entry id=4c29acf5-df06-44b5-9f1a-890d574b7e75 slug=done/unspecified digest=d0e631180e29 -->
#### [4c29acf5] [Board][Docs] Add board workflow guidance to .github agent files
- priority: `medium`
- summary: The Draftboard feature (epic 854f0e8f) is now fully implemented across all
- ref: `memory-viewers/memory-api/.ticket/tickets/4c29acf5-df06-44b5-9f1a-890d574b7e75/ticket.toml`

<!-- ticket-index:entry id=be38e809-781f-498c-915e-afaca1d1d3e0 slug=done/unspecified digest=af90d6e62bae -->
#### [be38e809] [Board][Validation] Concurrent check-in, crash recovery, and cross-interface consistency
- priority: `medium`
- summary: Validate that the draftboard behaves correctly under the failure modes and concurrency patterns it is explicitly meant to manage.
- ref: `memory-viewers/memory-api/.ticket/tickets/be38e809-781f-498c-915e-afaca1d1d3e0/ticket.toml`

<!-- ticket-index:entry id=c179ef57-6866-451d-ba7f-f7923ad1374b slug=done/unspecified digest=707c689fe677 -->
#### [c179ef57] [LOG-5a] Scaffold log-viewer-dioxus crate with trunk build and API client
- summary: The current log-viewer frontend is a Preact/Vite application. Per the Dioxus Viewer Platform epic (`35a6d14b`), all viewer frontends should be ported to Rust/Dioxus 0.7 compiled to WASM via `trunk`. ...
- ref: `.ticket/tickets/c179ef57-6866-451d-ba7f-f7923ad1374b/ticket.toml`

<!-- ticket-index:entry id=fe5232d9-537a-4217-b8c0-b8e3ca81d95b slug=done/unspecified digest=a67eaff97975 -->
#### [fe5232d9] [agent-rules] Prefer MCP Playwright tools in browser frontend testing guidance
- priority: `medium`
- summary: Browser-hosted frontend guidance requires Playwright coverage, but it does not explicitly tell agents to try the MCP Playwright/browser tools first before falling back to repo-local wrappers or manua...
- ref: `.ticket/tickets/fe5232d9-537a-4217-b8c0-b8e3ca81d95b/ticket.toml`

<!-- ticket-index:entry id=cf77062c-f663-4ed4-beca-9303795cf973 slug=done/unspecified digest=bbfca086bff4 -->
#### [cf77062c] [ci] Split viewer workflows to memory-viewers
- ref: `.ticket/tickets/cf77062c-f663-4ed4-beca-9303795cf973/ticket.toml`

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

<!-- ticket-index:entry id=2e52bd26-1a93-4c62-b712-024a567a934a slug=done/unspecified digest=24cba0cc7e55 -->
#### [2e52bd26] [handoff][ticket-workflow] Work package: regression-resistant best-next-ticket workflow
- summary: Bundle the "best next ticket to implement" hardening work into a single handoff package that another engineer or agent can pick up without needing to reconstruct the backlog from chat history.
- ref: `.ticket/tickets/2e52bd26-1a93-4c62-b712-024a567a934a/ticket.toml`

<!-- ticket-index:entry id=0e1dca8b-2869-4c43-b62b-79eb5f6f3a17 slug=done/unspecified digest=0bca153a6e6e -->
#### [0e1dca8b] [repo] Keep submodules on main instead of detached HEADs
- priority: `high`
- summary: Top-level submodules `memory-viewers` and `context-stack` are checked out in detached HEAD state, and local commits in those submodules are not naturally advancing `origin/main`. Update repository-ow...
- ref: `.ticket/tickets/0e1dca8b-2869-4c43-b62b-79eb5f6f3a17/ticket.toml`

<!-- ticket-index:entry id=830de529-2818-49fd-a792-3b59dd99a748 slug=done/unspecified digest=23343284bbe3 -->
#### [830de529] [spec-api] Normalize spec create target roots into .spec/specs
- summary: Fix spec creation target-root handling so spec entities are always created inside the canonical .spec/specs store when the caller passes a workspace root, the .spec store root, or a path inside that ...
- ref: `memory-viewers/memory-api/.ticket/tickets/830de529-2818-49fd-a792-3b59dd99a748/ticket.toml`

<!-- ticket-index:entry id=0b6e1bf3-2478-40a5-a619-085d8691835a slug=done/unspecified digest=3f8096cb7cbe -->
#### [0b6e1bf3] [spec-api][rules] Define blackbox contract and authoring guidance for expectation-oriented specs
- summary: Add failing blackbox tests and update the concrete authoring guidance surfaces so expectation-oriented specs are defined by intended properties, acceptance criteria, and evidence requirements rather ...
- ref: `.ticket/tickets/0b6e1bf3-2478-40a5-a619-085d8691835a/ticket.toml`

<!-- ticket-index:entry id=eaeaf157-85c3-4caf-a538-4f6ebb2a5ec7 slug=done/unspecified digest=4e8c7a10ca2d -->
#### [eaeaf157] [spec-cli] Normalize refs validate workspace_root JSON paths on Windows
- summary: Fix `spec refs <id> validate` so its JSON `workspace_root` output uses slash-normalized paths on Windows, matching existing expectations in the command tests and the output contract already used by `...
- ref: `.ticket/tickets/eaeaf157-85c3-4caf-a538-4f6ebb2a5ec7/ticket.toml`

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

<!-- ticket-index:entry id=6f3dcdfc-bf2f-45d7-9776-0f0a360ac199 slug=done/unspecified digest=cc436ae1dcc9 -->
#### [6f3dcdfc] [test-cli] Add test-result store and `test` CLI for validation evidence
- summary: `test-api` currently only defines validation identities (`ValidationSpec`, `ValidationExecution`) with no persistence and no CLI. Validation results are being written verbatim into ticket description...
- ref: `.ticket/tickets/6f3dcdfc-bf2f-45d7-9776-0f0a360ac199/ticket.toml`

<!-- ticket-index:entry id=68a08b34-000b-4585-8354-4b1a26a15f4b slug=done/unspecified digest=acad30e56d98 -->
#### [68a08b34] [ticket-cli] Scope-aware board and next for multi-root workspaces
- summary: `ticket board show` and `ticket next` are not scope-aware enough for multi-root repositories.
- ref: `.ticket/tickets/68a08b34-000b-4585-8354-4b1a26a15f4b/ticket.toml`

<!-- ticket-index:entry id=02723a9b-23ff-47b1-8306-0480be087ddd slug=done/unspecified digest=c2a97ee793ad -->
#### [02723a9b] [ticket-cli][ticket-viewer] Fix nested-workspace discovery and stale list races
- summary: Two ticket discovery paths are still unreliable for nested child workspaces.
- ref: `.ticket/tickets/02723a9b-23ff-47b1-8306-0480be087ddd/ticket.toml`

<!-- ticket-index:entry id=d1770bd5-dc7e-42ca-a5d0-2bc0cbc91110 slug=done/unspecified digest=5a022195e637 -->
#### [d1770bd5] [ticket-store] Relocate misplaced ticket and spec directories
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

<!-- ticket-index:entry id=6dc44fbb-4480-4bad-853c-79b8171dd73b slug=done/unspecified digest=f4112e42b4ea -->
#### [6dc44fbb] [viewer-api] Anchor SVG edge endpoints to world_to_screen instead of getBoundingClientRect
- summary: Decouple SVG edge-overlay geometry from the DOM layout pass so edges and node cards are positioned from the same projection source.
- ref: `.ticket/tickets/6dc44fbb-4480-4bad-853c-79b8171dd73b/ticket.toml`

<!-- ticket-index:entry id=c79e2630-3d49-454b-998f-fb52c24303f4 slug=done/unspecified digest=877e2e9b1fa4 -->
#### [c79e2630] [viewer-api] Default visual validation to external fullscreen Chromium
- ref: `.ticket/tickets/c79e2630-3d49-454b-998f-fb52c24303f4/ticket.toml`

<!-- ticket-index:entry id=f685dca9-1a67-4b0b-bc14-a88d6ef1226d slug=done/unspecified digest=15ea483de03a -->
#### [f685dca9] [viewer-api] Guard sync_render_state against resetting layout during active interaction
- summary: Prevent reactive re-renders (e.g. SSE `ticket.upsert`, hover changes) from resetting an in-progress drag or camera interaction.
- ref: `.ticket/tickets/f685dca9-1a67-4b0b-bc14-a88d6ef1226d/ticket.toml`

<!-- ticket-index:entry id=97a9ed0b-4442-4514-8c67-09e3393f79a7 slug=done/unspecified digest=94325040d7cf -->
#### [97a9ed0b] [viewer-api] render_frame: compute VP once and collect node rects once per frame
- summary: Remove the redundant per-frame work in `render_frame` that thrashes layout and inflates the rAF callback, causing DOM node cards to lag behind GPU edges.
- ref: `.ticket/tickets/97a9ed0b-4442-4514-8c67-09e3393f79a7/ticket.toml`

<!-- ticket-index:entry id=b005a8fe-9971-4e59-a3df-171f81b6d3f7 slug=done/unspecified digest=228a7948bde5 -->
#### [b005a8fe] [viewer-api][P1] Visual primitives: Breadcrumbs, Overlay/Modal, MetaHeader+Chip, Card/CardGrid
- summary: Add foundational visual primitives to `viewer-api-dioxus` that doc-viewer relies on but spec-viewer currently lacks.
- ref: `memory-viewers/viewer-api/.ticket/tickets/b005a8fe-9971-4e59-a3df-171f81b6d3f7/ticket.toml`

<!-- ticket-index:entry id=da16dada-e245-4fdd-868a-c3691e6c351a slug=done/unspecified digest=930c7250b2de -->
#### [da16dada] [viewer-api][P2] State containers: TabsStore, PathCodec/url_path, Prefetcher
- summary: Add reusable state containers to `viewer-api-dioxus` for cross-cutting viewer concerns.
- ref: `memory-viewers/viewer-api/.ticket/tickets/da16dada-e245-4fdd-868a-c3691e6c351a/ticket.toml`

<!-- ticket-index:entry id=8bf5edd2-4fe6-4580-ac87-73843f0206f0 slug=done/unspecified digest=109de115c12b -->
#### [8bf5edd2] [viewer-api][P3] Widget extensions: TreeNode rich tooltip, mobile sidebar audit, HeaderActions
- summary: Extend the existing shared widgets with capabilities doc-viewer uses but Dioxus viewers cannot express today.
- ref: `memory-viewers/viewer-api/.ticket/tickets/8bf5edd2-4fe6-4580-ac87-73843f0206f0/ticket.toml`

<!-- ticket-index:entry id=b4127011-4e08-47bc-ac73-3d3761f29587 slug=done/unspecified digest=61709ed4ec76 -->
#### [b4127011] [viewer-api][P4] FilterPanel shell with JQ presets and results list
- summary: Port doc-viewer's filter panel (basic dropdown filters + JQ presets + custom JQ input + results list) to a generic Dioxus shell.
- ref: `memory-viewers/viewer-api/.ticket/tickets/b4127011-4e08-47bc-ac73-3d3761f29587/ticket.toml`

<!-- ticket-index:entry id=5bf1951a-dce4-4efb-80d6-89fe4fa01573 slug=done/unspecified digest=808bd96eb904 -->
#### [5bf1951a] ticket-vscode: Fix tree view state grouping — show only same-state tickets per folder with dependency hierarchy
- priority: `high`
- summary: The current `buildStateGroups()` in `ticketProvider.ts` extends each state folder
- ref: `memory-viewers/memory-api/.ticket/tickets/5bf1951a-dce4-4efb-80d6-89fe4fa01573/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=1789cdfa-cd7e-45c4-a683-815b80c39970 slug=done/viewer-api digest=09ccda339cc7 -->
#### [1789cdfa] Feature: Extract WgpuOverlay/effects to viewer-api and GPU dependency graph in ticket-viewer
- summary: Extract the entire GPU rendering pipeline from log-viewer into viewer-api as shared infrastructure, then use it to build a GPU-rendered dependency graph in ticket-viewer.
- ref: `memory-viewers/viewer-api/.ticket/tickets/1789cdfa-cd7e-45c4-a683-815b80c39970/ticket.toml`

<!-- ticket-index:entry id=d7971816-6f84-419e-abd8-0f84d5f7b82f slug=done/viewer-api digest=725c14813b87 -->
#### [d7971816] Feature: Sortable FileTree with generic sorting header
- summary: Add a generic sorting header to the shared `FileTree` component in `viewer-api/frontend` and integrate it in ticket-viewer.
- ref: `memory-viewers/viewer-api/.ticket/tickets/d7971816-6f84-419e-abd8-0f84d5f7b82f/ticket.toml`

<!-- ticket-index:entry id=c826869a-d40c-425d-ba5c-4003c222cfde slug=done/viewer-api digest=ac308384f5c0 -->
#### [c826869a] Impl: Extract generic Graph3DView component to viewer-api
- summary: Extract a fully self-contained `Graph3DView` component into `viewer-api/frontend`. This is not a thin wrapper — it owns camera, layout, interaction, animation, and GPU rendering. Log-viewer only adds...
- ref: `memory-viewers/viewer-api/.ticket/tickets/c826869a-d40c-425d-ba5c-4003c222cfde/ticket.toml`

<!-- ticket-index:entry id=b3d250d5-dd28-44e6-aaf4-47bee9dea56e slug=done/viewer-api digest=54e661ee0e3b -->
#### [b3d250d5] Impl: Move WgpuOverlay + shaders + effects to viewer-api
- summary: Move the WgpuOverlay component, WGSL shaders, effects, and 3D math utilities from `log-viewer/frontend/src/` to `viewer-api/frontend/src/` so they become shared infrastructure. Also moves the SVG Gra...
- ref: `memory-viewers/viewer-api/.ticket/tickets/b3d250d5-dd28-44e6-aaf4-47bee9dea56e/ticket.toml`

<!-- ticket-index:entry id=a1259318-f992-44e3-9cdf-0ea4c224f6f3 slug=done/viewer-api digest=83562e4d534b -->
#### [a1259318] Impl: viewer-api extraction for shared tree/file/graph server primitives
- summary: Wave 1 / Track E** | Component: `viewer-api`
- ref: `memory-viewers/viewer-api/.ticket/tickets/a1259318-f992-44e3-9cdf-0ea4c224f6f3/ticket.toml`

<!-- ticket-index:entry id=a39f7805-c6bd-47d3-9b1b-fa29215bdf9e slug=done/viewer-api digest=4aa96438b973 -->
#### [a39f7805] Plan: Graph edge visual polish -- lighting, focus colors, and particles
- priority: `medium`
- summary: tags: `#plan` `#viewer-api` `#graph` `#ux` `#rendering` `#webgpu` `#lighting` `#particles`
- ref: `memory-viewers/viewer-api/.ticket/tickets/a39f7805-c6bd-47d3-9b1b-fa29215bdf9e/ticket.toml`

<!-- ticket-index:entry id=21a2e8f4-4bd8-4436-be52-c2c4a07bb692 slug=done/viewer-api digest=615853f38eab -->
#### [21a2e8f4] [viewer-api] Adopt rich tree tooltips in Dioxus spec-viewer and doc-viewer
- priority: `high`
- summary: Adopt the existing shared `TreeNode::tooltip_render` capability in current Dioxus tree consumers so the shared tree surface actually exposes richer doc-viewer-style metadata.
- ref: `memory-viewers/viewer-api/.ticket/tickets/21a2e8f4-4bd8-4436-be52-c2c4a07bb692/ticket.toml`

<!-- ticket-index:entry id=4d9293ab-b7a8-4113-b80a-bfe39297bad2 slug=done/viewer-api digest=7115085e78e4 -->
#### [4d9293ab] [viewer-api] Adopt shared TabsStore in Dioxus doc-viewer
- priority: `high`
- summary: Replace the Dioxus doc-viewer's ad-hoc tab-state signals with the existing shared `viewer_api_dioxus::TabsStore<OpenArtifactTab>` so the frontend actually consumes the tab-state primitive that alread...
- ref: `memory-viewers/viewer-api/.ticket/tickets/4d9293ab-b7a8-4113-b80a-bfe39297bad2/ticket.toml`

<!-- ticket-index:entry id=c6bf5b7a-f822-44bb-8d2b-86c966031ca6 slug=done/viewer-api digest=7ba75bb2324a -->
#### [c6bf5b7a] [viewer-api] Enlarge Graph3D directed edge arrow tips
- priority: `medium`
- summary: The shared Graph3D edge overlay in `memory-viewers/viewer-api/viewer-api/frontend/dioxus/src/graph3d/mod.rs` renders directed-edge arrow markers that are too small to read comfortably in ticket-viewe...
- ref: `.ticket/tickets/c6bf5b7a-f822-44bb-8d2b-86c966031ca6/ticket.toml`

<!-- ticket-index:entry id=763f8c13-a4bd-47af-8894-3e95a63fde8d slug=done/viewer-api digest=dec71667dd6a -->
#### [763f8c13] [viewer-api] Extract a reusable Dioxus explorer shell around FileTree
- priority: `high`
- summary: Extract a reusable Dioxus explorer shell around FileTree so viewers stop duplicating sidebar search and tree-control chrome.
- ref: `memory-viewers/viewer-api/.ticket/tickets/763f8c13-a4bd-47af-8894-3e95a63fde8d/ticket.toml`

<!-- ticket-index:entry id=735502cd-3aec-4772-b2a8-2184aaaf3c21 slug=done/viewer-api digest=850d5effc166 -->
#### [735502cd] [viewer-api] Extract a reusable interactive chip button for Dioxus explorer filters
- priority: `high`
- summary: Extract a shared clickable chip button in viewer-api so explorer filter/state toggles stop duplicating button markup and state wiring across FileTree and ticket-viewer.
- ref: `memory-viewers/viewer-api/.ticket/tickets/735502cd-3aec-4772-b2a8-2184aaaf3c21/ticket.toml`

<!-- ticket-index:entry id=fecbd4d8-b863-4821-bd7d-d6bd16f9356c slug=done/viewer-api digest=6dc7591dce81 -->
#### [fecbd4d8] [viewer-api] Preserve frontend build diagnostics in viewer-ctl failures
- priority: `medium`
- summary: `viewer-ctl prepare <viewer>` shells out to `trunk` and other frontend build
- ref: `memory-viewers/viewer-api/.ticket/tickets/fecbd4d8-b863-4821-bd7d-d6bd16f9356c/ticket.toml`

<!-- ticket-index:entry id=9a81d3e5-82ca-4fd0-84bf-c0a54f6716e5 slug=done/viewer-api digest=ba761cd3b2b9 -->
#### [9a81d3e5] [viewer-api] Reuse the shared toggle contract for Dioxus explorer sort controls
- priority: `high`
- summary: Reuse the shared toggle button contract for FileTree sort controls so explorer sort rows stop duplicating active/inactive button markup.
- ref: `memory-viewers/viewer-api/.ticket/tickets/9a81d3e5-82ca-4fd0-84bf-c0a54f6716e5/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=7346feae-045f-4da9-bf1c-47535132ffa1 slug=done/viewer-api-dioxus digest=2bafdee963f4 -->
#### [7346feae] Arch: viewer-api-dioxus crate scaffold and build system
- priority: `critical`
- ref: `memory-viewers/viewer-api/.ticket/tickets/7346feae-045f-4da9-bf1c-47535132ffa1/ticket.toml`

<!-- ticket-index:entry id=512986e0-9f0e-483c-8201-5c316bffdeb2 slug=done/viewer-api-dioxus digest=fcd6c5b07112 -->
#### [512986e0] Feature: Theme settings UI panel with live preview
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/512986e0-9f0e-483c-8201-5c316bffdeb2/ticket.toml`

<!-- ticket-index:entry id=2405a83e-e3b5-47ad-8d88-8c12f507d252 slug=done/viewer-api-dioxus digest=3455ce23a295 -->
#### [2405a83e] Port: CSS stylesheets — base, layout, buttons, tabs, tree, code-viewer
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/2405a83e-e3b5-47ad-8d88-8c12f507d252/ticket.toml`

<!-- ticket-index:entry id=7330aa36-102d-452c-b61d-6f4c8651b422 slug=done/viewer-api-dioxus digest=f15910faea05 -->
#### [7330aa36] Port: CodeViewer and FileContentViewer
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/7330aa36-102d-452c-b61d-6f4c8651b422/ticket.toml`

<!-- ticket-index:entry id=b3f9878d-5839-4a87-989c-aa3101ee38aa slug=done/viewer-api-dioxus digest=1d42f156d670 -->
#### [b3f9878d] Port: Layout components — Header, Layout, Sidebar, Panel, GlassPanel
- priority: `critical`
- ref: `memory-viewers/viewer-api/.ticket/tickets/b3f9878d-5839-4a87-989c-aa3101ee38aa/ticket.toml`

<!-- ticket-index:entry id=9dec4f23-4e92-4c14-b085-b9f625589228 slug=done/viewer-api-dioxus digest=fbb30e5d077b -->
#### [9dec4f23] Port: ResizeHandle with rAF-batched drag
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/9dec4f23-4e92-4c14-b085-b9f625589228/ticket.toml`

<!-- ticket-index:entry id=11f77899-6def-4140-b6bf-e84035a9264e slug=done/viewer-api-dioxus digest=e42b60b07755 -->
#### [11f77899] Port: TabBar, Spinner, Icons
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/11f77899-6def-4140-b6bf-e84035a9264e/ticket.toml`

<!-- ticket-index:entry id=46864375-0923-420c-b9db-67ce23056e52 slug=done/viewer-api-dioxus digest=df183c88080e -->
#### [46864375] Port: Theme system — ThemeStore, CSS variables, presets, save/load
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/46864375-0923-420c-b9db-67ce23056e52/ticket.toml`

<!-- ticket-index:entry id=31739fc3-bb79-4b56-8dd6-ea789340ac8a slug=done/viewer-api-dioxus digest=cc31d32fb94c -->
#### [31739fc3] Port: TreeView and FileTree with sort/filter
- priority: `critical`
- ref: `memory-viewers/viewer-api/.ticket/tickets/31739fc3-bb79-4b56-8dd6-ea789340ac8a/ticket.toml`

<!-- ticket-index:entry id=503eecc9-c8d6-4932-93df-e40018805818 slug=done/viewer-api-dioxus digest=b0d193b7106f -->
#### [503eecc9] Port: URL state management and session utilities
- priority: `medium`
- ref: `memory-viewers/viewer-api/.ticket/tickets/503eecc9-c8d6-4932-93df-e40018805818/ticket.toml`

<!-- ticket-index:entry id=5f668df8-82e8-4d3c-b3a7-95052a04d688 slug=done/viewer-api-dioxus digest=866bac583f74 -->
#### [5f668df8] [ticket-viewer][spec-viewer] Bug: theme settings action does not open the modal
- priority: `high`
- summary: Root cause: the Dioxus Trunk entrypoints for ticket-viewer, spec-viewer, and viewer-api omitted the shared `modal.css` bundle, so the Theme Settings overlay mounted without the fixed backdrop and sta...
- ref: `.ticket/tickets/5f668df8-82e8-4d3c-b3a7-95052a04d688/ticket.toml`


### Component: viewer-api-leptos

<!-- ticket-index:entry id=29897f92-59bf-45f9-b963-caa7bfad71c8 slug=done/viewer-api-leptos digest=f1d7875ea6c6 -->
#### [29897f92] Feature: UI polish — tab bar, sidebar, and resizable panels
- summary: The Leptos frontend has a minimal tab bar (20px, uppercase, no icons) and a flat sidebar (220px, no tree indentation, no resize). The TS version has a polished tab bar (32px, icons, active accents), ...
- ref: `memory-viewers/viewer-api/.ticket/tickets/29897f92-59bf-45f9-b963-caa7bfad71c8/ticket.toml`


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


## State: in-implementation

### Component: agent-tooling

<!-- ticket-index:entry id=9b9df133-d809-4900-b56a-afae4efcdd08 slug=in-implementation/agent-tooling digest=e63b5a2cec7b -->
#### [9b9df133] [token-efficiency] Track token-efficient agent tooling rollout
- priority: `high`
- summary: Goal: coordinate the workspace-wide token-efficiency rollout for agent-facing tooling and guidance.
- ref: `.ticket/tickets/9b9df133-d809-4900-b56a-afae4efcdd08/ticket.toml`


### Component: context-editor

<!-- ticket-index:entry id=febe05b2-ab03-4309-9d84-39aae471e27a slug=in-implementation/context-editor digest=cf97574e0a0c -->
#### [febe05b2] [context-editor][SVO-RM] Phase 1a: World-to-SVO Transform and Layout Validation
- summary: The ray march shader needs to transform world-space rays into the SVO's normalized $[0,1]^3$ coordinate space. Currently, `compute_node_positions()` outputs world-space centers and half-extents, but ...
- ref: `context-stack/tools/context-editor/.ticket/tickets/febe05b2-ab03-4309-9d84-39aae471e27a/ticket.toml`

<!-- ticket-index:entry id=9ef831d0-0f1d-46db-88bb-e537a37b9606 slug=in-implementation/context-editor digest=b7866261e8c7 -->
#### [9ef831d0] [context-editor][SVO-RM] Phase 1b: Core SVO Ray March Compute Shader
- summary: This is the centrepiece of the rendering rewrite. We need a compute shader that, for each pixel, casts a ray through the world-space SVO and finds the nearest voxel intersection by hierarchical trave...
- ref: `context-stack/tools/context-editor/.ticket/tickets/9ef831d0-0f1d-46db-88bb-e537a37b9606/ticket.toml`

<!-- ticket-index:entry id=22801e4f-36bd-43fc-b765-6e456b2bc63a slug=in-implementation/context-editor digest=070ae9b05a68 -->
#### [22801e4f] [context-editor][SVO-RM] Phase 2a: SDF Blending and Front-to-Back Alpha Compositing
- summary: Phase 1b establishes basic ray-AABB leaf hits with box SDF. This ticket refines the SDF evaluation to support:
- ref: `context-stack/tools/context-editor/.ticket/tickets/22801e4f-36bd-43fc-b765-6e456b2bc63a/ticket.toml`

<!-- ticket-index:entry id=8c2f1575-e704-44cb-bfc0-1f908bfc4855 slug=in-implementation/context-editor digest=0a9063401061 -->
#### [8c2f1575] [context-editor][SVO-RM] Phase 2b: Secondary Rays -- Reflections, Refractions, Shadows
- summary: The old pipeline required special-case code for glass panels (refraction, chromatic aberration, caustics) as a pre-pass before the splat loop. With SVO ray marching, secondary rays (reflections, refr...
- ref: `context-stack/tools/context-editor/.ticket/tickets/8c2f1575-e704-44cb-bfc0-1f908bfc4855/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=f9f46954-0a11-4450-a8a9-f3be6ec969a1 slug=in-implementation/context-engine digest=04792ca07392 -->
#### [f9f46954] [context-engine] Fix VS Code Copilot hook file path
- priority: `medium`
- summary: Restore the VS Code Copilot hook configuration after the hook file was renamed from `.github/hooks/docs-validation.json` to `.github/hooks/hooks.json` in this checkout.
- ref: `.ticket/tickets/f9f46954-0a11-4450-a8a9-f3be6ec969a1/ticket.toml`


### Component: context-read

<!-- ticket-index:entry id=978ce8a5-3936-467b-aca8-822eeecd1eb0 slug=in-implementation/context-read digest=2f7fe007bade -->
#### [978ce8a5] Plan: Expansion loop redesign — cursor-advancing decomposition
- summary: tags: `#plan` `#context-read` `#context-insert` `#algorithm` `#expansion` `#overlap` `#refactoring`
- ref: `context-stack/.ticket/tickets/978ce8a5-3936-467b-aca8-822eeecd1eb0/ticket.toml`

<!-- ticket-index:entry id=bfe43d0d-2870-4146-b651-1464a55ec7aa slug=in-implementation/context-read digest=cedeecdc8f4c -->
#### [bfe43d0d] [Bug] context-read largest-overlap incremental join misses expected decompositions
- summary: `read()` should build larger tokens by repeatedly finding the largest next overlap and joining that overlap into the running root. The current failing repeat and rotating-overlap cases show that this...
- ref: `.ticket/tickets/bfe43d0d-2870-4146-b651-1464a55ec7aa/ticket.toml`

<!-- ticket-index:entry id=9f8d842e-3c7c-4470-b840-dd69b92380b5 slug=in-implementation/context-read digest=c78400ff8c38 -->
#### [9f8d842e] [context-read] Replace root surgery with structural block materialization
- priority: `high`
- summary: `RootManager` currently grows semantic roots through `flat_root`, `wrap_root`, `replace_last_child`, and `try_extend_tail_with`. That is mutation-heavy shortcut logic, not the structural block-to-blo...
- ref: `.ticket/tickets/9f8d842e-3c7c-4470-b840-dd69b92380b5/ticket.toml`


### Component: documentation-tooling

<!-- ticket-index:entry id=0ffac34a-4e33-426c-8eef-ef6482ab3bde slug=in-implementation/documentation-tooling digest=35b048885974 -->
#### [0ffac34a] Implement Docker harness for documented install and deinstall flows
- priority: `high`
- summary: After the Docker validation strategy is defined, the repository still needs a runnable harness that executes the documented installation and deinstallation steps in clean containers and proves that t...
- ref: `.ticket/tickets/0ffac34a-4e33-426c-8eef-ef6482ab3bde/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=8ab31960-f3fa-4a2b-b2ac-f807e1a15fdc slug=in-implementation/memory-api digest=07af18c9dcb7 -->
#### [8ab31960] [memory-api][ticket-api][ticket-cli][ticket-mcp][ticket-http] Implement expressive ticket query and ordering
- priority: `high`
- summary: The repository has the pieces of a query engine, but not the complete interface needed for focused ticket discovery.
- ref: `.ticket/tickets/8ab31960-f3fa-4a2b-b2ac-f807e1a15fdc/ticket.toml`

<!-- ticket-index:entry id=e6e09d6f-a41c-49f7-bc6a-c6d8e822598b slug=in-implementation/memory-api digest=d6d00fb81275 -->
#### [e6e09d6f] [memory-api][ticket-cli][spec-cli][rule-cli] Normalize nested workspace option semantics
- priority: `high`
- summary: Shared nested-workspace resolution exists in progress, but review found that spec now overloads workspace-root for store selection, refs validation, and bootstrap source relativization while rule sti...
- ref: `memory-viewers/memory-api/.ticket/tickets/e6e09d6f-a41c-49f7-bc6a-c6d8e822598b/ticket.toml`

<!-- ticket-index:entry id=ef0ebf38-7f55-4bd7-bf0c-0b416650ee0b slug=in-implementation/memory-api digest=569430b4fcbc -->
#### [ef0ebf38] [memory-api][ticket-cli][spec-cli][rule-cli] Unify child-workspace resolution across CLI tools
- priority: `high`
- summary: Current child-workspace resolution across the CLI tools is inconsistent even though store selection already uses a shared workspace-root/index-root resolver.
- ref: `.ticket/tickets/ef0ebf38-7f55-4bd7-bf0c-0b416650ee0b/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=7b8d2e81-6f00-486c-a839-ca5eb77dc109 slug=in-implementation/repo-guidance digest=f245232f1255 -->
#### [7b8d2e81] [readmes][generated-repos] Adopt shared README schema in memory-viewers family
- priority: `high`
- summary: The already-generated README surfaces in `memory-api`, `viewer-api`, and `memory-viewers` still use bespoke target layouts. They need to adopt the shared schema and fill the missing parent or child n...
- ref: `.ticket/tickets/7b8d2e81-6f00-486c-a839-ca5eb77dc109/ticket.toml`

<!-- ticket-index:entry id=37dfe6cc-0d8d-4b85-b1cb-e9c262a9de5f slug=in-implementation/repo-guidance digest=4ab9fbdda666 -->
#### [37dfe6cc] [repo-guidance][cline] Integrate Cline agent client with client-agnostic .agents/ standard
- priority: `medium`
- ref: `.ticket/tickets/37dfe6cc-0d8d-4b85-b1cb-e9c262a9de5f/ticket.toml`


### Component: rule-api

<!-- ticket-index:entry id=f15d9e8b-72d2-44d9-965d-9fecbbc02d7f slug=in-implementation/rule-api digest=a3c1ebb9f30b -->
#### [f15d9e8b] Build rule-api for generated agent instruction docs
- priority: `high`
- summary: Agent-facing markdown guidance is duplicated across context-engine, memory-viewers, memory-api, and viewer-api. The duplicated files are currently copy-pasted and several are byte-identical. This cre...
- ref: `memory-viewers/memory-api/.ticket/tickets/f15d9e8b-72d2-44d9-965d-9fecbbc02d7f/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=f147eb0e-c758-459b-a956-a1162c3e1af6 slug=in-implementation/spec-api digest=32fabf4d3f4c -->
#### [f147eb0e] Migrate recurring spec principles to canonical rule entries via spec sync-generated
- priority: `high`
- summary: Cross-cutting design principles that recur across many specs (workspace identifiers, typed errors, JSON contracts, browser validation, ticket traceability link format, generated-file markers, `<x>-ap...
- ref: `.ticket/tickets/f147eb0e-c758-459b-a956-a1162c3e1af6/ticket.toml`

<!-- ticket-index:entry id=13a57a83-df99-4031-87e2-844772758ebb slug=in-implementation/spec-api digest=59753c4ac82d -->
#### [13a57a83] [spec][P8] Bootstrap: write spec files for the spec system itself
- priority: `high`
- summary: Author the canonical specification database covering the spec-system crates
- ref: `memory-viewers/memory-api/.ticket/tickets/13a57a83-df99-4031-87e2-844772758ebb/ticket.toml`


### Component: ticket-cli

<!-- ticket-index:entry id=91011568-ae0b-4b23-b060-b0c018e1e912 slug=in-implementation/ticket-cli digest=303882450462 -->
#### [91011568] [ticket-cli][ticket-mcp] Expose authoritative ticket folder paths in query output
- priority: `high`
- ref: `memory-viewers/memory-api/.ticket/tickets/91011568-ae0b-4b23-b060-b0c018e1e912/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=416ebd52-447d-44e4-a4ad-23162d37e0b1 slug=in-implementation/ticket-http digest=8e59d9cf44c6 -->
#### [416ebd52] [ticket-http] Return only authoritative resolved hits in workspace-aware search
- priority: `high`
- summary: HTTP query responses must only expose tickets that resolve to authoritative indexed paths and workspace ownership.
- ref: `.ticket/tickets/416ebd52-447d-44e4-a4ad-23162d37e0b1/ticket.toml`

<!-- ticket-index:entry id=cccf5d99-d7e9-43e6-8aea-90480ad3cf0d slug=in-implementation/ticket-http digest=7064abfa39a6 -->
#### [cccf5d99] [ticket-http][ticket-viewer] Bug: query results ignore active state filter
- priority: `high`
- summary: The ticket explorer currently fails to honor the active state filter once the user types a search query.
- ref: `memory-viewers/memory-api/.ticket/tickets/cccf5d99-d7e9-43e6-8aea-90480ad3cf0d/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=4a228c24-d466-4782-9160-c492f727007a slug=in-implementation/ticket-viewer digest=d12944b91292 -->
#### [4a228c24] Feature: Full-text search UI with field predicates
- priority: `medium`
- summary: The ticket-viewer sidebar works well for browsing known tickets, but it is too slow when users only know part of a title, body phrase, or ticket id. The viewer needs an in-app search surface that can...
- ref: `memory-viewers/.ticket/tickets/4a228c24-d466-4782-9160-c492f727007a/ticket.toml`

<!-- ticket-index:entry id=1f39ba8f-650b-417d-b664-1878f08af669 slug=in-implementation/ticket-viewer digest=627e4f966793 -->
#### [1f39ba8f] [ticket-viewer] Add graph review E2E coverage
- priority: `high`
- summary: Add release Playwright coverage for the graph review checklist items around layout restoration and zoom-driven node detail interactions.
- ref: `memory-viewers/.ticket/tickets/1f39ba8f-650b-417d-b664-1878f08af669/ticket.toml`

<!-- ticket-index:entry id=f121b24b-61b0-41b4-9567-8ffc2417d7cb slug=in-implementation/ticket-viewer digest=6b70b1c97f9f -->
#### [f121b24b] [ticket-viewer] Feature: keyboard navigation in explorer + quick-search
- priority: `high`
- summary: Ticket selection in the current ticket-viewer is still predominantly mouse-driven.
- ref: `memory-viewers/.ticket/tickets/f121b24b-61b0-41b4-9567-8ffc2417d7cb/ticket.toml`

<!-- ticket-index:entry id=60092819-f725-48ec-93f0-aba195ef81eb slug=in-implementation/ticket-viewer digest=c201c0eba317 -->
#### [60092819] [ticket-viewer] Fix graph layout defaults and isometric settings
- priority: `high`
- summary: Record graph layout/defaults implementation and move the ticket to review.
- ref: `memory-viewers/.ticket/tickets/60092819-f725-48ec-93f0-aba195ef81eb/ticket.toml`

<!-- ticket-index:entry id=4a9b49fd-58e0-404c-a120-47ef277dcf9f slug=in-implementation/ticket-viewer digest=0b9c39cf930a -->
#### [4a9b49fd] [ticket-viewer] Keep filtered explorer state authoritative under live refresh
- priority: `high`
- summary: Keep the filtered explorer authoritative under overlapping requests, SSE updates, snapshot refreshes, and workspace switches, and lock the full redesign with focused tests.
- ref: `.ticket/tickets/4a9b49fd-58e0-404c-a120-47ef277dcf9f/ticket.toml`

<!-- ticket-index:entry id=6e7a15c9-d8e6-4bbe-bb34-b83bd651896b slug=in-implementation/ticket-viewer digest=677b27329eb1 -->
#### [6e7a15c9] [ticket-viewer] Keep full workspace graph visible with focused navigation
- priority: `high`
- summary: Change the ticket-viewer graph mode so the full graph stays visible while the selected ticket becomes the active focus anchor.
- ref: `memory-viewers/.ticket/tickets/6e7a15c9-d8e6-4bbe-bb34-b83bd651896b/ticket.toml`

<!-- ticket-index:entry id=4629b9d9-3bd0-4ef6-82b6-d6e609c16cac slug=in-implementation/ticket-viewer digest=0a4241eead41 -->
#### [4629b9d9] [ticket-viewer] Migrate list/detail/search flows to workspace-aware ticket references
- priority: `high`
- summary: >Frontend migration is in progress; root-route mixed-workspace history/files flow is validated and asset follow-up investigation continues.
- ref: `memory-viewers/memory-api/.ticket/tickets/4629b9d9-3bd0-4ef6-82b6-d6e609c16cac/ticket.toml`

<!-- ticket-index:entry id=929bc26a-5296-4d64-b1b2-2ec580c0659c slug=in-implementation/ticket-viewer digest=8c6ea691dc92 -->
#### [929bc26a] [ticket-viewer][viewer-api] Make graph framing panel-aware and keep nodes behind UI panels
- priority: `high`
- summary: Keep graph content behind sidebar and viewport panels while using those panel bounds to bias graph framing, focus centering, and node placement.
- ref: `.ticket/tickets/929bc26a-5296-4d64-b1b2-2ec580c0659c/ticket.toml`

<!-- ticket-index:entry id=923c866a-fecd-4ddb-8be0-00ca4cb22af9 slug=in-implementation/ticket-viewer digest=6cf52e49605c -->
#### [923c866a] [ticket-viewer][viewer-api] Refine graph selection focus and outside-click deselection
- priority: `high`
- summary: Refine graph node selection and focus falloff so the selected ticket stays emphasized, linked context remains visible, and clicking outside the graph clears selection.
- ref: `.ticket/tickets/923c866a-fecd-4ddb-8be0-00ca4cb22af9/ticket.toml`

<!-- ticket-index:entry id=800f09ed-beb0-4a12-be93-1392e45eadb8 slug=in-implementation/ticket-viewer digest=06e8e83d8de1 -->
#### [800f09ed] [ticket-viewer][viewer-api] Tighten graph layout and enlarge rich nodes
- priority: `high`
- summary: Make the default ticket graph easier to read by tightening the layout, enlarging visible node tiers, and giving rich ticket nodes a more cubic high-LOD presentation.
- ref: `memory-viewers/.ticket/tickets/800f09ed-beb0-4a12-be93-1392e45eadb8/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=6de424b0-68ec-43c7-9d70-eb8d17305ab3 slug=in-implementation/ticket-vscode digest=4205b9b51585 -->
#### [6de424b0] [ticket-vscode] Validate Rust/WASM parity across desktop, web, and remote hosts
- priority: `high`
- summary: This ticket closes the track by validating the implemented port against the spec and the current user-visible workflows.
- ref: `memory-viewers/memory-api/.ticket/tickets/6de424b0-68ec-43c7-9d70-eb8d17305ab3/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=111510f4-c74b-4819-800b-d68ab013a73c slug=in-implementation/unspecified digest=757e1971635d -->
#### [111510f4] Fix graph reactivity: ticket state changes don't update graph nodes
- summary: When a ticket state is changed in the details panel, the graph nodes don't update their visual representation (color, label, etc.). The graph only listens to `edge.*` SSE events, not `ticket.*` event...
- ref: `.ticket/tickets/111510f4-c74b-4819-800b-d68ab013a73c/ticket.toml`

<!-- ticket-index:entry id=b4f444ee-4858-4d13-8cdb-690a33115611 slug=in-implementation/unspecified digest=07f56e89cac7 -->
#### [b4f444ee] Move context-stack to repo root and remove deprecated folders
- summary: Move the context-stack submodule from crates/context-stack to context-stack at the repository root, remove the deprecated humans/, agents/, scripts/, and tools/http/ directories, and update the works...
- ref: `.ticket/tickets/b4f444ee-4858-4d13-8cdb-690a33115611/ticket.toml`

<!-- ticket-index:entry id=90279c46-6c9b-42a5-a60e-3ac8bfad346a slug=in-implementation/unspecified digest=8b410b22c1e5 -->
#### [90279c46] [hooks][rule] Make pre-commit validate only repo-local rule targets
- ref: `.ticket/tickets/90279c46-6c9b-42a5-a60e-3ac8bfad346a/ticket.toml`

<!-- ticket-index:entry id=e14f893f-042d-45cc-b748-f48860a640c5 slug=in-implementation/unspecified digest=7ce247f8876a -->
#### [e14f893f] ticket-viewer WgpuOverlay panics with `unreachable` when overlay enabled (default-ON regression)
- priority: `high`
- summary: Surfaced by the e2e test
- ref: `memory-viewers/.ticket/tickets/e14f893f-042d-45cc-b748-f48860a640c5/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=35a6d14b-25b0-4b24-b59f-d0d733cacd20 slug=in-implementation/viewer-api digest=3f11bb049a69 -->
#### [35a6d14b] Epic: Dioxus Viewer Platform — viewer-api-dioxus + ticket-viewer Dioxus frontend
- priority: `critical`
- summary: Port the viewer-api frontend library and ticket-viewer SPA from TypeScript/Preact to Rust/Dioxus 0.7, compiled to WASM via `trunk` (Trunk WASM bundler). Adds full ticket mutation capabilities powered...
- ref: `memory-viewers/viewer-api/.ticket/tickets/35a6d14b-25b0-4b24-b59f-d0d733cacd20/ticket.toml`

<!-- ticket-index:entry id=bb1c32f5-5275-4e4f-85ae-a0fba09c522a slug=in-implementation/viewer-api digest=b0e4b04918e6 -->
#### [bb1c32f5] [viewer-api] Extract a reusable Dioxus page header shell
- priority: `high`
- summary: Extract a reusable Dioxus page-header shell in viewer-api-dioxus so viewer routes stop composing ad-hoc header behavior inline.
- ref: `memory-viewers/viewer-api/.ticket/tickets/bb1c32f5-5275-4e4f-85ae-a0fba09c522a/ticket.toml`

<!-- ticket-index:entry id=322ba030-160c-41d3-8a12-42936ae92858 slug=in-implementation/viewer-api digest=2543f1e386ae -->
#### [322ba030] [viewer-api][ticket-viewer] Add multi-level graph node detail rendering
- priority: `high`
- summary: Introduce multiple graph node detail levels so zoomed-out views stay legible and zoomed-in views can show rich ticket content.
- ref: `memory-viewers/.ticket/tickets/322ba030-160c-41d3-8a12-42936ae92858/ticket.toml`

<!-- ticket-index:entry id=68eaae1f-b230-4aab-8572-cbf41d1d3b6d slug=in-implementation/viewer-api digest=ca3a639a979e -->
#### [68eaae1f] [viewer-api][ticket-viewer] Add optional 2D graph mode and presentation keyframing
- priority: `high`
- summary: Add an optional fixed 2D graph presentation mode with a planar camera, 2D grid styling, and presentation keyframing for temporary selection-driven layouts.
- ref: `.ticket/tickets/68eaae1f-b230-4aab-8572-cbf41d1d3b6d/ticket.toml`

<!-- ticket-index:entry id=f9e9aaae-b1ec-434c-a839-7ec990d1e6c7 slug=in-implementation/viewer-api digest=b5fb99f76232 -->
#### [f9e9aaae] [viewer-api][ticket-viewer] Introduce property-based graph node rendering tiers
- priority: `high`
- summary: Replace the current rich-card-first graph node presentation with property-based level-of-detail rendering that can collapse to points, spheres, icons, labels, and tool-free compact summaries before u...
- ref: `.ticket/tickets/f9e9aaae-b1ec-434c-a839-7ec990d1e6c7/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=dbd048a0-08b4-458d-b860-29b8ce5119e3 slug=in-implementation/viewer-api-dioxus digest=677e7cde4c08 -->
#### [dbd048a0] Feature: WgpuOverlay — full-screen GPU compositor with DOM capture and particle effects
- priority: `high`
- ref: `memory-viewers/viewer-api/.ticket/tickets/dbd048a0-08b4-458d-b860-29b8ce5119e3/ticket.toml`


## State: in-review

### Component: agent-tooling

<!-- ticket-index:entry id=4f066c96-b398-4aba-93f1-7d0fd4da39ba slug=in-review/agent-tooling digest=d234e53e0f67 -->
#### [4f066c96] [token-efficiency] Add compact terminal MCP tool
- priority: `high`
- summary: Implement a compact terminal MCP tool that returns short outputs inline and truncates long outputs automatically.
- ref: `.ticket/tickets/4f066c96-b398-4aba-93f1-7d0fd4da39ba/ticket.toml`

<!-- ticket-index:entry id=65819900-1d16-4c53-8b5d-7548c64a75ef slug=in-review/agent-tooling digest=27f07c9c4954 -->
#### [65819900] [token-efficiency] Add interface skeletonization utility
- priority: `medium`
- summary: Create an interface skeletonization utility that strips implementation bodies and returns only structural information.
- ref: `.ticket/tickets/65819900-1d16-4c53-8b5d-7548c64a75ef/ticket.toml`

<!-- ticket-index:entry id=685b577e-9e5e-4c96-86de-ce5420db46bc slug=in-review/agent-tooling digest=f95c376ffea2 -->
#### [685b577e] [token-efficiency] Add pre-flight write validation gates
- priority: `high`
- summary: Strengthen local pre-flight validation so expensive syntax-debugging loops are rejected before code is saved or finalized.
- ref: `.ticket/tickets/685b577e-9e5e-4c96-86de-ce5420db46bc/ticket.toml`

<!-- ticket-index:entry id=d4605cc0-5901-4b68-94d5-e7e3e6cac06f slug=in-review/agent-tooling digest=2793299c4ce7 -->
#### [d4605cc0] [token-efficiency] Add token-bounded file inspection utility
- priority: `medium`
- summary: Create a token-bounded file inspection utility that defaults to narrow line windows instead of whole-file reads.
- ref: `.ticket/tickets/d4605cc0-5901-4b68-94d5-e7e3e6cac06f/ticket.toml`

<!-- ticket-index:entry id=72c1e92d-65e1-445b-9365-e3384d9da088 slug=in-review/agent-tooling digest=670c0736143a -->
#### [72c1e92d] [token-efficiency] Generate static `repo_map.toon`
- priority: `high`
- summary: Add a generated root-level `repo_map.toon` workspace map for low-token structural awareness.
- ref: `.ticket/tickets/72c1e92d-65e1-445b-9365-e3384d9da088/ticket.toml`

<!-- ticket-index:entry id=06cfe998-c2e1-48a4-83e9-11e85e7c40f4 slug=in-review/agent-tooling digest=43d86087a135 -->
#### [06cfe998] [token-efficiency] Introduce peek-api with peek-cli and peek-mcp transport layers
- priority: `high`
- summary: Introduce a proper `peek-api` library crate and move the current `peek-cli` logic behind the repository’s standard `*-api` layering so `peek-cli` and a new `peek-mcp` become thin transport adapters.
- ref: `.ticket/tickets/06cfe998-c2e1-48a4-83e9-11e85e7c40f4/ticket.toml`

<!-- ticket-index:entry id=e29e24ba-1f7e-43b3-97bd-c20d53b76df8 slug=in-review/agent-tooling digest=395736a411f2 -->
#### [e29e24ba] [token-efficiency] Make MCP update tools accept sparse payloads and return minimal changed fields
- priority: `high`
- summary: Make MCP update tools accept sparse payloads that include only the keys being changed, and return minimal response payloads that include only changed or directly relevant fields.
- ref: `.ticket/tickets/e29e24ba-1f7e-43b3-97bd-c20d53b76df8/ticket.toml`

<!-- ticket-index:entry id=f93e5db5-4f20-4e23-8832-498c4591938f slug=in-review/agent-tooling digest=f1aea6618e6b -->
#### [f93e5db5] [token-efficiency] Replace repo_map Python generation with peek-api folder skeleton tree output
- priority: `high`
- summary: Replace the current Python-based repo-map generation flow with a repo-aware `peek-api` skeleton/tree renderer that can accept a folder path, apply compaction/filtering rules, and emit a tree-shaped s...
- ref: `.ticket/tickets/f93e5db5-4f20-4e23-8832-498c4591938f/ticket.toml`


### Component: agent-workflow

<!-- ticket-index:entry id=f227c217-6fda-452e-ae35-4208eb3974f5 slug=in-review/agent-workflow digest=3e92b5221e4a -->
#### [f227c217] [token-efficiency] Update guidance for compact agent workflows
- priority: `medium`
- summary: Update the repository guidance so agent workflows consistently prefer compact/default outputs and verbose-on-demand expansion.
- ref: `.ticket/tickets/f227c217-6fda-452e-ae35-4208eb3974f5/ticket.toml`


### Component: audit-api

<!-- ticket-index:entry id=855a1e5d-d998-4caf-b60c-d75a13ca3264 slug=in-review/audit-api digest=9fe9d9d33711 -->
#### [855a1e5d] [memory-index] Audit store status summary generator
- priority: `medium`
- summary: Build a generator that reads the audit-api and emits a compact markdown summary of the current audit status at `.audit/README.md` along with its TOON sidecar at `.audit/index.toon`. The purpose is to...
- ref: `.ticket/tickets/855a1e5d-d998-4caf-b60c-d75a13ca3264/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=46d89aa2-043a-4c94-8213-2f365aa2d517 slug=in-review/context-engine digest=3f0f66297c72 -->
#### [46d89aa2] Add handoff workflow prompts
- priority: `medium`
- summary: Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session jumpstart handoffs. Scope includes rule-target config, canonical prompt rule entries, generated pr...
- ref: `.ticket/tickets/46d89aa2-043a-4c94-8213-2f365aa2d517/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=d187d817-d3f5-49ca-8925-8d06b5824912 slug=in-review/memory-api digest=513b2e2b73e1 -->
#### [d187d817] [ticket-cli][spec-cli][rule-cli][audit-cli] Add TOON input and output support
- priority: `medium`
- summary: Implemented TOON machine-readable output across the memory-api CLI suite and extended spec-cli structured field decoding to accept TOON next to JSON.
- ref: `.ticket/tickets/d187d817-d3f5-49ca-8925-8d06b5824912/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=14ff41fa-818e-4a4e-8747-f79a33d174c2 slug=in-review/repo-guidance digest=65180f499bff -->
#### [14ff41fa] [agents][rule] Add token-optimized agentic engineering skill target
- priority: `medium`
- summary: Implemented generated rule target for `.agents/skills/token-optimized-agentic-engineering.SKILL.md`, created canonical `.skill` rule entry, translated source guidance to English, and verified target ...
- ref: `.ticket/tickets/14ff41fa-818e-4a4e-8747-f79a33d174c2/ticket.toml`


### Component: rule-api

<!-- ticket-index:entry id=9336a096-4399-467e-a7d8-fac30080d71f slug=in-review/rule-api digest=99b5f6f9a3ff -->
#### [9336a096] [memory-index] Rule store catalog generator
- priority: `high`
- summary: Build a generator that reads the rule store (rule-api) and emits a grouped catalog at `.rule/README.md` with its TOON sidecar at `.rule/index.toon`. The purpose is to give agents a compact, browsable...
- ref: `.ticket/tickets/9336a096-4399-467e-a7d8-fac30080d71f/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=cf4d1e1a-5315-4aa8-b836-5a90996e63c4 slug=in-review/session-api digest=05ed813e6e82 -->
#### [cf4d1e1a] [session-api] Fix: Resolve session workspace relative to tool execution
- priority: `high`
- summary: Fix a bug where the `.memory-api` folder is created inside the nested `memory-viewers/memory-api` folder even when the ticket tool is run from the `context-engine` root. The session workspace should ...
- ref: `.ticket/tickets/cf4d1e1a-5315-4aa8-b836-5a90996e63c4/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=b9757ba7-3b2c-4f92-919d-f3c443ceb69c slug=in-review/spec-api digest=6c63c35fcc71 -->
#### [b9757ba7] [memory-index] Spec store hierarchy generator
- priority: `high`
- summary: Build a generator that reads the spec store (spec-api) and emits a hierarchical markdown folder tree under `.spec/`, with `.spec/index.toon` as the machine-readable TOON sidecar. The purpose is to gi...
- ref: `.ticket/tickets/b9757ba7-3b2c-4f92-919d-f3c443ceb69c/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=c5e9bb39-d784-4d0c-8de1-3885013cddce slug=in-review/ticket-api digest=02b1bbcf5b99 -->
#### [c5e9bb39] [memory-index] Ticket store index generator with git hook integration
- priority: `high`
- summary: Build a generator that reads the ticket store (ticket-api) and emits a committed markdown index co-located in `.ticket/README.md` along with its TOON sidecar at `.ticket/index.toon`. The purpose is t...
- ref: `.ticket/tickets/c5e9bb39-d784-4d0c-8de1-3885013cddce/ticket.toml`

<!-- ticket-index:entry id=385f2521-b318-403b-a4ea-195a47e5c453 slug=in-review/ticket-api digest=8360e1b3c93b -->
#### [385f2521] [ticket-api] Unify multi-step state transitions across update and close flows
- priority: `high`
- summary: `ticket update` currently enforces a single-step state transition and optionally accepts `from_state`, which duplicates the current store state and rejects legitimate fast-forward workflows such as `...
- ref: `.ticket/tickets/385f2521-b318-403b-a4ea-195a47e5c453/ticket.toml`


### Component: ticket-cli

<!-- ticket-index:entry id=74fd59ca-8253-4e18-99bd-0b1fa204c6d6 slug=in-review/ticket-cli digest=82227f01ef41 -->
#### [74fd59ca] [ticket-cli] Remove constant blocker-progress field from board show JSON recommendations
- priority: `medium`
- summary: `ticket board show --json` currently includes `recommended_next[].last_blocker_progress_at`, but recommended-next items are sourced from the actionable queue where that field is always null by contra...
- ref: `.ticket/tickets/74fd59ca-8253-4e18-99bd-0b1fa204c6d6/ticket.toml`


### Component: ticket-query

<!-- ticket-index:entry id=f6aa9048-c300-4f64-bf20-157d439dd7ca slug=in-review/ticket-query digest=194892b6a356 -->
#### [f6aa9048] [spec][ticket-query] Specify expressive query and ordering contract
- priority: `high`
- summary: The current ticket query contract is not expressive enough for focused discovery.
- ref: `.ticket/tickets/f6aa9048-c300-4f64-bf20-157d439dd7ca/ticket.toml`


### Component: ticket-system

<!-- ticket-index:entry id=6848ffa2-4e31-4985-beff-cba01af9b7ca slug=in-review/ticket-system digest=a9b1c3310e45 -->
#### [6848ffa2] [ticket-system] Add effort field for token-budget estimates
- priority: `medium`
- summary: Extend ticket ordering so `board`, `next`, `list`, and similar listing surfaces account for the new `effort` field.
- ref: `.ticket/tickets/6848ffa2-4e31-4985-beff-cba01af9b7ca/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=10c94251-1c0c-4542-a282-ea3d75a205b5 slug=in-review/ticket-viewer digest=10e636801c24 -->
#### [10c94251] [ticket-viewer][viewer-api] Graph focus and 2D presentation follow-up
- priority: `high`
- summary: Track the next graph-viewer interaction and presentation upgrade for ticket-viewer: property-based node rendering, stronger selection semantics, panel-aware framing, an optional fixed 2D camera mode,...
- ref: `.ticket/tickets/10c94251-1c0c-4542-a282-ea3d75a205b5/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=694d74b4-028b-4602-8090-d6200d577d4a slug=in-review/ticket-vscode digest=844df011a0dd -->
#### [694d74b4] [ticket-vscode] Integrate Rust/WASM core into TS hosts and remove replaced legacy logic
- priority: `high`
- summary: The Rust/WASM core (`ticket-vscode-core`) is built, tested, and packaged, but it is **not yet wired into the live extension code path**. Today only `core_version()` is called as a smoke check in `src...
- ref: `.ticket/tickets/694d74b4-028b-4602-8090-d6200d577d4a/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=635b7e37-8bed-4622-a38d-ef87bb08f46c slug=in-review/unspecified digest=4ee6597083a3 -->
#### [635b7e37] [audit-api] Derive spec fulfillment rollups from store-owned evidence
- summary: Teach `audit-api` to report derived spec fulfillment status by reading store-owned expectation and evidence metadata.
- ref: `.ticket/tickets/635b7e37-8bed-4622-a38d-ef87bb08f46c/ticket.toml`

<!-- ticket-index:entry id=37dc83ab-af5c-4746-9c02-b27ffb8215a9 slug=in-review/unspecified digest=32d4702b1512 -->
#### [37dc83ab] [bug] Tantivy 0.22.1 fastfield panic breaks spec/store full-text search
- summary: Full-text search across the spec store (and any store backed by the same Tantivy index path) is non-functional. `spec scan --force` panics inside Tantivy and incremental scans silently fail to popula...
- ref: `.ticket/tickets/37dc83ab-af5c-4746-9c02-b27ffb8215a9/ticket.toml`

<!-- ticket-index:entry id=87001cb8-46c4-4921-a336-dc0cf0c1f66a slug=in-review/unspecified digest=597c1ab86eee -->
#### [87001cb8] [doc-api] Add documentation-validation evidence identities for spec fulfillment
- summary: Extend `doc-api` with documentation-validation identities and coverage metadata that can satisfy or block spec acceptance clauses.
- ref: `.ticket/tickets/87001cb8-46c4-4921-a336-dc0cf0c1f66a/ticket.toml`

<!-- ticket-index:entry id=1f7a7d60-e7ea-49c4-80c4-dee78e8862be slug=in-review/unspecified digest=799170adc7b7 -->
#### [1f7a7d60] [hooks][rule] Show sync-targets failure output in pre-commit
- ref: `.ticket/tickets/1f7a7d60-e7ea-49c4-80c4-dee78e8862be/ticket.toml`

<!-- ticket-index:entry id=0805fb76-f99b-45a5-87c6-5a8e65bdb2da slug=in-review/unspecified digest=16d199926c1b -->
#### [0805fb76] [log-api] Bootstrap validation-log identities for spec fulfillment
- summary: Bootstrap the first `log-api` entities for validation-log capture and retrieval linked from `test-api` executions.
- ref: `.ticket/tickets/0805fb76-f99b-45a5-87c6-5a8e65bdb2da/ticket.toml`

<!-- ticket-index:entry id=64a7cb3a-b35f-4953-9368-0d7afc89fb53 slug=in-review/unspecified digest=4190f4ed48ec -->
#### [64a7cb3a] [memory-api] Sync install contract README section with canonical rule entry
- ref: `.ticket/tickets/64a7cb3a-b35f-4953-9368-0d7afc89fb53/ticket.toml`

<!-- ticket-index:entry id=099ac71e-bffa-4a5b-89f3-2ca3bc875bac slug=in-review/unspecified digest=f58bc5794efc -->
#### [099ac71e] [profiling] Validate browser profiling pipeline (trace capture + wasm benches)
- priority: `high`
- summary: Phase-1 implementation is landed and compiles; this ticket covers the remaining
- ref: `.ticket/tickets/099ac71e-bffa-4a5b-89f3-2ca3bc875bac/ticket.toml`

<!-- ticket-index:entry id=bf8ef22e-ea06-45de-9f90-a2fee0e4cc6e slug=in-review/unspecified digest=acae63716a74 -->
#### [bf8ef22e] [repo-guidance][rule-api] Add implement agent target from canonical rules
- summary: The repository generates custom agents for research, testing, interview, and audit, but it does not provide a dedicated implement agent that is optimized for surgical execution once the scope is clea...
- ref: `.ticket/tickets/bf8ef22e-ea06-45de-9f90-a2fee0e4cc6e/ticket.toml`

<!-- ticket-index:entry id=68c61b92-af6b-4331-99fd-5a77dd3512e1 slug=in-review/unspecified digest=a5a3cff1e534 -->
#### [68c61b92] [rule-cli] Improve generate-target errors for config directories and output-path targets
- ref: `.ticket/tickets/68c61b92-af6b-4331-99fd-5a77dd3512e1/ticket.toml`

<!-- ticket-index:entry id=f4f955b0-a827-4fce-882d-4df2f5903891 slug=in-review/unspecified digest=fc9c0fd3a76f -->
#### [f4f955b0] [rule-cli] Make scan output explain diagnostics and counters
- ref: `.ticket/tickets/f4f955b0-a827-4fce-882d-4df2f5903891/ticket.toml`

<!-- ticket-index:entry id=665b727c-09b1-43e0-8795-eb67e2758aea slug=in-review/unspecified digest=6ff8bab034f1 -->
#### [665b727c] [rule-cli][rule-api] Allow rule-targets directories for generate-target configs
- ref: `.ticket/tickets/665b727c-09b1-43e0-8795-eb67e2758aea/ticket.toml`

<!-- ticket-index:entry id=e7a31e70-e8f8-4369-aae4-98cc7f35db7c slug=in-review/unspecified digest=66dedb4b0ea3 -->
#### [e7a31e70] [rule-cli][rule-api] Require explicit child-workspace scans for render commands
- ref: `.ticket/tickets/e7a31e70-e8f8-4369-aae4-98cc7f35db7c/ticket.toml`

<!-- ticket-index:entry id=0da01943-4bab-44eb-bc4b-c803f6526b26 slug=in-review/unspecified digest=769aad40c0be -->
#### [0da01943] [rules][copilot] Integrate RTK section into generated copilot instructions
- ref: `.ticket/tickets/0da01943-4bab-44eb-bc4b-c803f6526b26/ticket.toml`

<!-- ticket-index:entry id=c73d4a6b-2610-4e69-9fc3-bfedcf2ec53d slug=in-review/unspecified digest=1bdea200ef89 -->
#### [c73d4a6b] [spec-api] Add native expectation, acceptance, and evidence fields
- summary: Extend `spec-api` with native fields and validation for expected properties, acceptance clauses, and evidence requirements.
- ref: `.ticket/tickets/c73d4a6b-2610-4e69-9fc3-bfedcf2ec53d/ticket.toml`

<!-- ticket-index:entry id=b744bcf5-05a5-4601-bbe1-caae9d42ea5f slug=in-review/unspecified digest=1c3a03212961 -->
#### [b744bcf5] [spec-api] Expectation-oriented spec contract and model
- summary: Redefine the specification contract and the native `spec-api` model so the repository can represent expected properties, acceptance clauses, and evidence requirements without relying on free-form pro...
- ref: `.ticket/tickets/b744bcf5-05a5-4601-bbe1-caae9d42ea5f/ticket.toml`

<!-- ticket-index:entry id=c666f0b3-f1e6-4073-852f-e494bf5c1272 slug=in-review/unspecified digest=341fd1418e20 -->
#### [c666f0b3] [spec-cli][spec-mcp][spec-http] Expose expectation and evidence parity across transports
- summary: Expose the expectation and evidence model consistently through `spec-cli`, `spec-mcp`, and `spec-http`, with one shared parity contract.
- ref: `.ticket/tickets/c666f0b3-f1e6-4073-852f-e494bf5c1272/ticket.toml`

<!-- ticket-index:entry id=86bf3da2-b6cc-4fc7-898d-044403283550 slug=in-review/unspecified digest=de383cbd8f56 -->
#### [86bf3da2] [test-api] Bootstrap validation specification and execution identities for spec fulfillment
- summary: Bootstrap the first `test-api` entities for validation specifications, executions, and outcomes used by expectation-oriented spec fulfillment.
- ref: `.ticket/tickets/86bf3da2-b6cc-4fc7-898d-044403283550/ticket.toml`

<!-- ticket-index:entry id=185419e0-bea4-4c7b-abda-1e92193f32e7 slug=in-review/unspecified digest=2990745a96c8 -->
#### [185419e0] [ticket-api] Allow bidirectional ticket state transitions by default
- summary: Ticket state transitions should work in both directions by default using the same state transition interface. We should not require every schema to spell out reverse edges when the validator can trea...
- ref: `.ticket/tickets/185419e0-bea4-4c7b-abda-1e92193f32e7/ticket.toml`

<!-- ticket-index:entry id=46d16755-309b-479f-aab2-624c3fa7ce9b slug=in-review/unspecified digest=7a7aade2434c -->
#### [46d16755] [ticket-vscode] Fix canonical workspace selection when server exposes path-based or shared workspace ids
- summary: Implemented and validated canonical workspace resolution for ticket-vscode. The extension now maps detected local .ticket roots to canonical server workspace ids by label/path and otherwise prefers a...
- ref: `.ticket/tickets/46d16755-309b-479f-aab2-624c3fa7ce9b/ticket.toml`

<!-- ticket-index:entry id=20b6a09a-080a-480b-8f09-79cbf7bc20bd slug=in-review/unspecified digest=3409fb50cda4 -->
#### [20b6a09a] [token-efficiency] Omit default workspace and schema from ticket outputs
- priority: `high`
- summary: Implemented shared ticket output normalization to omit default workspace/schema metadata across CLI and MCP responses; validated with focused ticket-api, ticket-cli, and ticket-mcp tests.
- ref: `.ticket/tickets/20b6a09a-080a-480b-8f09-79cbf7bc20bd/ticket.toml`

<!-- ticket-index:entry id=7db89f25-9395-45b3-a35d-8c5c219067f8 slug=in-review/unspecified digest=a2abd57f4569 -->
#### [7db89f25] [viewer-api] Eliminate per-frame DOM reflow: analytic node rects + skip unchanged LOD writes
- summary: Node cards still trail GPU-drawn edges during orbit/pan/drag. Root cause confirmed by reading graph3d/render.rs: the per-frame render loop performs read-after-write DOM layout thrashing.
- ref: `.ticket/tickets/7db89f25-9395-45b3-a35d-8c5c219067f8/ticket.toml`

<!-- ticket-index:entry id=8d6895a5-dce8-47c1-98ce-212fd0ae2e08 slug=in-review/unspecified digest=89dbc4f41cc8 -->
#### [8d6895a5] [viewer-api][audit] Fix viewer-api-dioxus compile failure and restore llvm-cov coverage collection
- ref: `.ticket/tickets/8d6895a5-dce8-47c1-98ce-212fd0ae2e08/ticket.toml`

<!-- ticket-index:entry id=618f6ce4-e7b3-48f2-9c9e-840247a119da slug=in-review/unspecified digest=b730b656e01b -->
#### [618f6ce4] [workflow] Bootstrap doc-api, test-api, and log-api evidence stores for spec fulfillment
- summary: Coordinate the bounded evidence-store bootstrap across `doc-api`, `test-api`, and `log-api` so spec acceptance clauses can resolve authoritative documentation, validation, and log evidence without wr...
- ref: `.ticket/tickets/618f6ce4-e7b3-48f2-9c9e-840247a119da/ticket.toml`

<!-- ticket-index:entry id=bc19467f-b4d4-48c3-be92-b551d4fe6679 slug=in-review/unspecified digest=1689e19e05c6 -->
#### [bc19467f] [workflow] Expectation-oriented spec contract rollout
- summary: Coordinate the rollout that redefines specifications around intended properties, acceptance criteria, and store-owned evidence while migrating the affected specs and tickets homogeneously.
- ref: `.ticket/tickets/bc19467f-b4d4-48c3-be92-b551d4fe6679/ticket.toml`

<!-- ticket-index:entry id=577df498-d468-448f-afc1-3e35e48e5f12 slug=in-review/unspecified digest=8541dde0bdd5 -->
#### [577df498] [workflow] Homogeneously migrate remaining expectation-oriented specs and tickets
- summary: Homogeneously migrate the remaining affected expectation-oriented specs and tickets after the pilot proves the contract.
- ref: `.ticket/tickets/577df498-d468-448f-afc1-3e35e48e5f12/ticket.toml`

<!-- ticket-index:entry id=6e5306fb-c1b3-4aec-991d-fabaf3096e23 slug=in-review/unspecified digest=aa9392cdd870 -->
#### [6e5306fb] [workflow] Pilot expectation-oriented spec contract on one workflow spec and one README-rollout spec
- summary: Pilot the expectation-oriented spec contract on one workflow spec and one README-rollout spec.
- ref: `.ticket/tickets/6e5306fb-c1b3-4aec-991d-fabaf3096e23/ticket.toml`

<!-- ticket-index:entry id=aaa90ee6-1358-41ad-b19e-61abdc3f1dc2 slug=in-review/unspecified digest=76bd55da6465 -->
#### [aaa90ee6] [workflow] Store-owned spec evidence integration
- summary: Integrate store-owned acceptance evidence so specs can be satisfied or blocked by native documentation, validation, and log records rather than by wrapper-owned artifact payloads.
- ref: `.ticket/tickets/aaa90ee6-1358-41ad-b19e-61abdc3f1dc2/ticket.toml`


## State: new

### Component: agent-tooling

<!-- ticket-index:entry id=0dd23fe6-6892-4d21-9927-4a81584dc77a slug=new/agent-tooling digest=31f7ea3cf674 -->
#### [0dd23fe6] [token-efficiency] Audit execute MCP tools for terminal reuse and input continuation features
- priority: `medium`
- summary: Audit the currently active execute-style MCP surfaces and adjacent terminal-execution tooling to determine whether they already support terminal reuse, follow-up input, resumable execution, or persis...
- ref: `.ticket/tickets/0dd23fe6-6892-4d21-9927-4a81584dc77a/ticket.toml`


### Component: agent-workflow

<!-- ticket-index:entry id=f18e6885-c193-4159-82c5-d164e470437b slug=new/agent-workflow digest=4df326f07a0e -->
#### [f18e6885] Add spec-system guidance and Spec Agent rule targets
- priority: `medium`
- summary: Add generated spec-system guidance instructions and a Spec Agent workflow so spec creation/update work consistently links tests, tickets, and related specs. Update rule targets and canonical rule ent...
- ref: `.ticket/tickets/f18e6885-c193-4159-82c5-d164e470437b/ticket.toml`


### Component: cli

<!-- ticket-index:entry id=c01ace60-4794-48fd-a22c-f4745ad2ca3c slug=new/cli digest=a5be7fe787d0 -->
#### [c01ace60] Plan: end-to-end test registry
- summary: tags: `#plan` `#context-trace` `#context-search` `#context-insert` `#algorithm` `#debugging` `#testing` `#api` `#performance`
- ref: `.ticket/tickets/c01ace60-4794-48fd-a22c-f4745ad2ca3c/ticket.toml`


### Component: context-api

<!-- ticket-index:entry id=974e6e37-f414-4ac3-8f5c-e867c709b775 slug=new/context-api digest=68acdb5d8aba -->
#### [974e6e37] Design: Instruction Language DSL for graph operations
- summary: tags: `#context-api` `#design` `#instruction-language` `#dsl` `#future`
- ref: `.ticket/tickets/974e6e37-f414-4ac3-8f5c-e867c709b775/ticket.toml`

<!-- ticket-index:entry id=b786f1f5-8d04-4586-8e30-a532069bbd81 slug=new/context-api digest=497c1f11bd5a -->
#### [b786f1f5] Plan: CLI read UX improvement — ReadSequence, ReadFile, REPL parsing
- summary: tags: `#plan` `#context-api` `#context-cli` `#ux` `#read` `#cli` `#repl`
- ref: `.ticket/tickets/b786f1f5-8d04-4586-8e30-a532069bbd81/ticket.toml`


### Component: context-editor

<!-- ticket-index:entry id=e7da478e-b18e-4551-a385-d39e81d09a41 slug=new/context-editor digest=75113d7d83e6 -->
#### [e7da478e] Plan: context-editor — unified GPU-accelerated 3D world editor tool
- priority: `critical`
- summary: A single-binary, GPU-accelerated tool that merges the log-viewer, doc-viewer,
- ref: `.ticket/tickets/e7da478e-b18e-4551-a385-d39e81d09a41/ticket.toml`

<!-- ticket-index:entry id=8922e00c-98ac-4604-ae01-29acca066b61 slug=new/context-editor digest=2ea01c22f08b -->
#### [8922e00c] [context-editor] Epic: Direct SVO Ray Marching — Replace Tiled Forward+ Pipeline
- priority: `critical`
- summary: The current rendering pipeline uses a multi-stage GPU-driven splatting approach:
- ref: `.ticket/tickets/8922e00c-98ac-4604-ae01-29acca066b61/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=c2409055-c489-441b-9a60-f3b3aa608522 slug=new/context-engine digest=ca9504a709f7 -->
#### [c2409055] [memory-index] Memory workspace DAG indexing
- priority: `medium`
- summary: Build a workspace summary capability locally inside each tool/domain (e.g. ticket-cli, spec-cli, rule-cli). Under this contract, each store folder (like `.ticket/` or `.spec/`) acts as the root ancho...
- ref: `.ticket/tickets/c2409055-c489-441b-9a60-f3b3aa608522/ticket.toml`

<!-- ticket-index:entry id=fe098673-f7fa-43ba-af66-047578861596 slug=new/context-engine digest=1e1649463598 -->
#### [fe098673] [memory-index] Roadmap: sequential implementation of domain-owned store indexes
- priority: `high`
- summary: Provide one canonical roadmap tracker for the memory-index work so implementation proceeds in a single explicit order instead of a loose set of related tickets.
- ref: `.ticket/tickets/fe098673-f7fa-43ba-af66-047578861596/ticket.toml`

<!-- ticket-index:entry id=2a3ad242-8c01-4779-94ec-9e4d5595f538 slug=new/context-engine digest=205900f3c974 -->
#### [2a3ad242] [sandbox-v1][impl] memory-stack traceability, archive linking, and runbook docs
- priority: `high`
- summary: Record workflow metadata in ticket, spec, and doc owned surfaces.
- ref: `.ticket/tickets/2a3ad242-8c01-4779-94ec-9e4d5595f538/ticket.toml`

<!-- ticket-index:entry id=0884ab64-e54d-4f9c-abbf-de61990773eb slug=new/context-engine digest=25bd8ef9ac8b -->
#### [0884ab64] [sandbox-v1][impl] session execution, per-session MCP, and artifact capture
- priority: `high`
- summary: Copilot completions client and session runner.
- ref: `.ticket/tickets/0884ab64-e54d-4f9c-abbf-de61990773eb/ticket.toml`

<!-- ticket-index:entry id=5ed70069-b080-4a95-8dc5-ddf495007bdd slug=new/context-engine digest=ad3391ba85d1 -->
#### [5ed70069] [sandbox-v1][impl] validation and hardening gates
- priority: `high`
- summary: Integration harness for end-to-end Firecracker-backed sandbox execution.
- ref: `.ticket/tickets/5ed70069-b080-4a95-8dc5-ddf495007bdd/ticket.toml`

<!-- ticket-index:entry id=6bebc161-63e6-4177-9958-0e36ffcd92bc slug=new/context-engine digest=e52d5e7a3abc -->
#### [6bebc161] [sandbox-v1][track] functional sandbox orchestration implementation
- priority: `high`
- summary: Track completion of the v1 sandbox orchestration implementation after planning and design are complete.
- ref: `.ticket/tickets/6bebc161-63e6-4177-9958-0e36ffcd92bc/ticket.toml`


### Component: context-insert

<!-- ticket-index:entry id=a4210ebf-208c-48b7-814f-da0d3269e236 slug=new/context-insert digest=ed81c606ec27 -->
#### [a4210ebf] Plan: integration test remediation — RC-1, RC-2, RC-3 fix rounds
- summary: tags: `#plan` `#testing` `#integration` `#context-api` `#context-read` `#context-insert` `#bug-fix` `#refactoring`
- ref: `.ticket/tickets/a4210ebf-208c-48b7-814f-da0d3269e236/ticket.toml`


### Component: context-mcp

<!-- ticket-index:entry id=61f78a57-6896-4ad7-9daa-0e9e805aa397 slug=new/context-mcp digest=7e181e9b096d -->
#### [61f78a57] Plan: Context API phase 3.1 — per-command tracing log capture + log query tools
- summary: tags: `#context-api` `#phase3.1` `#tracing` `#logs` `#cli` `#mcp` `#jq`
- ref: `.ticket/tickets/61f78a57-6896-4ad7-9daa-0e9e805aa397/ticket.toml`


### Component: context-read

<!-- ticket-index:entry id=3125d4c5-5eb1-48a0-a935-a5d686408a72 slug=new/context-read digest=fee89bcc0c96 -->
#### [3125d4c5] Bug: context-read crate 28 compilation errors — API mismatch with context-trace
- summary: tags: `#bug-report` `#context-trace` `#context-read` `#debugging` `#refactoring` `#api`
- ref: `.ticket/tickets/3125d4c5-5eb1-48a0-a935-a5d686408a72/ticket.toml`

<!-- ticket-index:entry id=6432858e-0e7c-4a0c-bc59-96b04f932391 slug=new/context-read digest=5bcda499282c -->
#### [6432858e] Plan: context-read completion — text indexing crate
- summary: tags: `#plan` `#context-read` `#algorithm` `#cursor` `#expansion` `#overlap`
- ref: `.ticket/tickets/6432858e-0e7c-4a0c-bc59-96b04f932391/ticket.toml`

<!-- ticket-index:entry id=668743ea-497b-46a2-b7f7-f136684acc8c slug=new/context-read digest=3636867bc98e -->
#### [668743ea] Plan: context-read restructure — migrate bands/, delete dead code, rename pipeline/
- summary: tags: `#plan` `#context-read` `#architecture` `#restructuring` `#api` `#refactoring`
- ref: `.ticket/tickets/668743ea-497b-46a2-b7f7-f136684acc8c/ticket.toml`

<!-- ticket-index:entry id=fe81b165-113f-43fc-87c2-dc7f44170152 slug=new/context-read digest=565fcadcd2b2 -->
#### [fe81b165] Tracker: context-read final test remediation for context-stack integration
- summary: Remediate the remaining `context-read` crate test failures that block full `context-stack` integration, while aligning the ticket set and spec language with the clarified read algorithm.
- ref: `.ticket/tickets/fe81b165-113f-43fc-87c2-dc7f44170152/ticket.toml`

<!-- ticket-index:entry id=c6cc7d5a-dcfb-4ae0-bfc7-d8682462503b slug=new/context-read digest=73d94c4fe585 -->
#### [c6cc7d5a] [Bug] context-read normalization of embedded paths is inconsistent across API layers
- summary: The current failing assertions around infix and overlap matches assume that lower-level path results must always normalize to an `EntireRoot` materialized token.
- ref: `.ticket/tickets/c6cc7d5a-dcfb-4ae0-bfc7-d8682462503b/ticket.toml`

<!-- ticket-index:entry id=05925875-000c-4af3-913e-e4121ab223ca slug=new/context-read digest=4c1710b23834 -->
#### [05925875] [Bug] context-read overlap-step materialization breaks retention policy or graph invariants
- summary: `context-read` should materialize graph state after each overlap expansion step to keep progression safe, but that materialized state must still obey retention policy and structural invariants.
- ref: `.ticket/tickets/05925875-000c-4af3-913e-e4121ab223ca/ticket.toml`

<!-- ticket-index:entry id=f8dfcd09-0e29-4ee6-a61f-de64aed1098f slug=new/context-read digest=a54d33c41c82 -->
#### [f8dfcd09] [context-read] Revisit prior roots when new overlap subparts materialize
- priority: `high`
- summary: The current pipeline materializes overlap products locally but does not reliably revisit already-known roots when new subparts appear later. That is why `bcdea` still misses `[bc, dea]` after later r...
- ref: `.ticket/tickets/f8dfcd09-0e29-4ee6-a61f-de64aed1098f/ticket.toml`


### Component: context-search

<!-- ticket-index:entry id=346573c1-2711-407d-a50f-a2cbce53b965 slug=new/context-search digest=db231524e470 -->
#### [346573c1] Bug: TraceCache root token mismatch causes insert panics
- summary: tags: `#context-search` `#context-insert` `#TraceCache` `#InitInterval` `#panic` `#critical`
- ref: `.ticket/tickets/346573c1-2711-407d-a50f-a2cbce53b965/ticket.toml`

<!-- ticket-index:entry id=d265e603-feac-4cec-86e0-a323acd990b1 slug=new/context-search digest=49f310358fc8 -->
#### [d265e603] Plan: search event refactoring — PathNode, IntoTransition, tentative root
- summary: tags: `#plan` `#refactoring` `#visualization` `#events` `#search`
- ref: `.ticket/tickets/d265e603-feac-4cec-86e0-a323acd990b1/ticket.toml`


### Component: context-tasks

<!-- ticket-index:entry id=4470de7b-8c04-4c06-ae29-af411ade5db5 slug=new/context-tasks digest=16a7f91d91fe -->
#### [4470de7b] Design backlog: stable dependency semantics with state-derived readiness
- priority: `backlog`
- ref: `.ticket/tickets/4470de7b-8c04-4c06-ae29-af411ade5db5/ticket.toml`


### Component: context-trace

<!-- ticket-index:entry id=619e49fc-951a-4e14-bc33-e831525c3002 slug=new/context-trace digest=4ccbc5629dba -->
#### [619e49fc] Plan: fine-grained locking design for context-trace
- summary: tags: `#plan` `#context-trace` `#context-search` `#context-insert` `#context-read` `#debugging` `#testing` `#refactoring` `#api`
- ref: `.ticket/tickets/619e49fc-951a-4e14-bc33-e831525c3002/ticket.toml`

<!-- ticket-index:entry id=19990e37-b5c2-41bc-af39-d649559a8885 slug=new/context-trace digest=c61d670c2998 -->
#### [19990e37] Plan: graph diff command — diff two graph states
- summary: tags: `#plan` `#cli` `#api` `#context-api` `#context-cli` `#graph-diff` `#comparison`
- ref: `.ticket/tickets/19990e37-b5c2-41bc-af39-d649559a8885/ticket.toml`

<!-- ticket-index:entry id=164549c4-1050-4fb6-9bc0-57077cbf2667 slug=new/context-trace digest=6a3537f878ec -->
#### [164549c4] Plan: position-annotated paths — path structures with position metadata
- summary: tags: `#plan` `#context-trace` `#context-search` `#debugging` `#testing` `#performance`
- ref: `.ticket/tickets/164549c4-1050-4fb6-9bc0-57077cbf2667/ticket.toml`

<!-- ticket-index:entry id=0d61b9df-544d-453c-9a8f-68078ec5163f slug=new/context-trace digest=5522a0e243ea -->
#### [0d61b9df] Plan: selective partition merge — avoid full-graph merge
- summary: tags: `#plan` `#context-insert` `#algorithm` `#testing` `#api`
- ref: `.ticket/tickets/0d61b9df-544d-453c-9a8f-68078ec5163f/ticket.toml`

<!-- ticket-index:entry id=f8afe331-41e2-4563-ad6a-456837afb1f8 slug=new/context-trace digest=a649990d9965 -->
#### [f8afe331] [Bug] dedup_atoms_not_duplicated: regression panic in vertex/data/children.rs
- summary: Regression** — this test was previously passing. It now panics.
- ref: `.ticket/tickets/f8afe331-41e2-4563-ad6a-456837afb1f8/ticket.toml`

<!-- ticket-index:entry id=f41f08a8-fad9-4a20-b3a4-58bc1cc4d6ef slug=new/context-trace digest=134db28c7ba8 -->
#### [f41f08a8] [Bug] edge_repeated_single_char: panic — pattern width mismatch in T2w4 token (RC-3)
- summary: cargo test -p context-cli --test cli_integration -- edge_repeated_single_char --nocapture
- ref: `.ticket/tickets/f41f08a8-fad9-4a20-b3a4-58bc1cc4d6ef/ticket.toml`


### Component: doc-cli

<!-- ticket-index:entry id=ad9f6e52-2147-4b25-be2c-9e59dd58a876 slug=new/doc-cli digest=79b3db846d8b -->
#### [ad9f6e52] [doc-cli] Add CLI surface for doc-api
- priority: `high`
- summary: Create `doc-cli` as the CLI interface for `doc-api`.
- ref: `.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876/ticket.toml`


### Component: doc-viewer

<!-- ticket-index:entry id=0515479f-a5c2-47c6-b8c2-3961dfa6dcf7 slug=new/doc-viewer digest=5b4bc1ffa612 -->
#### [0515479f] Plan: MCP crate docs — extend MCP server for crate API documentation
- summary: tags: `#plan` `#context-trace` `#debugging` `#api`
- ref: `.ticket/tickets/0515479f-a5c2-47c6-b8c2-3961dfa6dcf7/ticket.toml`


### Component: docs

<!-- ticket-index:entry id=88cd4cb8-8b31-48b6-9c13-28522d939b0b slug=new/docs digest=95d0b2b706f5 -->
#### [88cd4cb8] Plan: dungeon crawler skill docs (4 skill documents in docs/skills/)
- summary: tags: `#plan` `#documentation` `#skills` `#dungeon-crawler` `#hypergraph` `#educational` `#external-facing`
- ref: `.ticket/tickets/88cd4cb8-8b31-48b6-9c13-28522d939b0b/ticket.toml`


### Component: log-api

<!-- ticket-index:entry id=501d4932-a48e-4c8a-a4f3-8c31be0bdd23 slug=new/log-api digest=39b6d0bfb7af -->
#### [501d4932] [log-api] Add first-class validation log capture and retrieval
- priority: `high`
- summary: Add a first-class `log-api` for workflow validation log capture, indexing, and retrieval in the memory system.
- ref: `.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23/ticket.toml`


### Component: log-viewer

<!-- ticket-index:entry id=06e00e0b-42ce-4a74-aea2-392302dd68f7 slug=new/log-viewer digest=37842b14538e -->
#### [06e00e0b] [log-viewer] Integrate GraphOpEvent replay with 3D graph visualization
- priority: `medium`
- summary: Integrate GraphOpEvent replay system with the enhanced 3D graph visualization in log-viewer:
- ref: `.ticket/tickets/06e00e0b-42ce-4a74-aea2-392302dd68f7/ticket.toml`

<!-- ticket-index:entry id=bf295665-a075-4cfb-9a86-f54e96918695 slug=new/log-viewer digest=158efcf2b427 -->
#### [bf295665] [log-viewer] Integrate graph improvements (selection, rendering tiers, panel framing, 2D mode)
- priority: `high`
- summary: Integrate the four graph improvements into log-viewer:
- ref: `.ticket/tickets/bf295665-a075-4cfb-9a86-f54e96918695/ticket.toml`


### Component: ngrams

<!-- ticket-index:entry id=a25e3cad-5bf2-4432-9c89-3a6dd67ee774 slug=new/ngrams digest=8066bae7330c -->
#### [a25e3cad] Plan: ngrams oracle validation — compare ngrams against oracle
- summary: tags: `#plan` `#testing` `#validation` `#ngrams` `#context-read` `#context-api` `#integration`
- ref: `.ticket/tickets/a25e3cad-5bf2-4432-9c89-3a6dd67ee774/ticket.toml`


### Component: repo-guidance

<!-- ticket-index:entry id=ce0beb35-fc60-45ae-b26b-3cd06a282476 slug=new/repo-guidance digest=751199e93287 -->
#### [ce0beb35] [context-engine] Generate root README and root-owned child READMEs from rules
- priority: `high`
- summary: The root workspace already has a `.rule` store, but its `README.md` and the root-owned child READMEs are still manual. That leaves the most visible repository entry points outside the generation pipe...
- ref: `.ticket/tickets/ce0beb35-fc60-45ae-b26b-3cd06a282476/ticket.toml`

<!-- ticket-index:entry id=3f62f10e-6f7d-4fa1-b205-97fe62babaf2 slug=new/repo-guidance digest=c4818e1a6649 -->
#### [3f62f10e] [context-stack] Add local .rule store and generate root README
- priority: `high`
- summary: `context-stack` still lacks a repo-local `.rule` store and local README targets, so its root README cannot participate in the same local-generation workflow as the memory-viewers workspaces.
- ref: `.ticket/tickets/3f62f10e-6f7d-4fa1-b205-97fe62babaf2/ticket.toml`

<!-- ticket-index:entry id=c785a6f6-57d3-46d1-9a0e-36e1a4b74a47 slug=new/repo-guidance digest=fcba839dde46 -->
#### [c785a6f6] [context-stack] Generate first-level child READMEs with parent links
- priority: `high`
- summary: Even once the `context-stack` root README is generated, the internal README tree still fails because its first-level child READMEs do not link back to the parent and are not managed as a coherent rep...
- ref: `.ticket/tickets/c785a6f6-57d3-46d1-9a0e-36e1a4b74a47/ticket.toml`

<!-- ticket-index:entry id=26f570e2-6a2f-4604-9347-a3ac7d0314c3 slug=new/repo-guidance digest=f6167092c98b -->
#### [26f570e2] [memory-viewers] Adopt shared README schema and normalize child blocks
- priority: `high`
- summary: `memory-viewers` is the aggregate repo root for the generated family, but its README target still has a bespoke structure and its child-block behavior needs to be normalized after the child repos ado...
- ref: `.ticket/tickets/26f570e2-6a2f-4604-9347-a3ac7d0314c3/ticket.toml`

<!-- ticket-index:entry id=95a12f97-dc32-4835-a87a-5e24574be951 slug=new/repo-guidance digest=f962f16ef670 -->
#### [95a12f97] [readmes][manual-repos] Migrate root and context-stack README trees to rule generation
- priority: `high`
- summary: `context-engine` and `context-stack` are still the manual outliers in the repository README family. They need to move onto rule-backed generation and the same parent/child README navigation contract ...
- ref: `.ticket/tickets/95a12f97-dc32-4835-a87a-5e24574be951/ticket.toml`

<!-- ticket-index:entry id=9f14365b-fbe5-4f93-a8da-f7f490dacac0 slug=new/repo-guidance digest=11836e54ba88 -->
#### [9f14365b] [readmes][qa] Add completeness audit and workspace sync checks
- priority: `high`
- summary: Even after the rollout lands, the README tree will drift again unless there is a mechanical check for generated ownership, parent and child navigation blocks, installable-content coverage, and direct...
- ref: `.ticket/tickets/9f14365b-fbe5-4f93-a8da-f7f490dacac0/ticket.toml`

<!-- ticket-index:entry id=ef50db70-90e6-4de4-bcb0-fa364664a6cf slug=new/repo-guidance digest=08ce9d330c7a -->
#### [ef50db70] [readmes][rule-api] Roll out shared README schema across workspace trees
- priority: `high`
- summary: Repository README generation is split between manual repo roots and generated nested workspaces. Shared structure is duplicated, parent/child navigation is inconsistent, and there is no single tracke...
- ref: `.ticket/tickets/ef50db70-90e6-4de4-bcb0-fa364664a6cf/ticket.toml`

<!-- ticket-index:entry id=ca30f696-e8a0-4904-9a1d-a507e9ef6147 slug=new/repo-guidance digest=48e6559e6cc3 -->
#### [ca30f696] [readmes][rule-api] Track shared schema loader contract and rollout follow-through
- priority: `high`
- summary: The shared README schema rollout now has a concrete loader-contract gap: shared schema fragments can be reached through both explicit imports and ambient fragment discovery, and the rollout depends o...
- ref: `.ticket/tickets/ca30f696-e8a0-4904-9a1d-a507e9ef6147/ticket.toml`


### Component: search

<!-- ticket-index:entry id=ee43f72e-53ef-4937-8216-92e17f185d85 slug=new/search digest=07cae98ebcc6 -->
#### [ee43f72e] [bootstrap] implement unified query execution on real indexes
- summary: Support fast query and highlighting across:
- ref: `memory-viewers/memory-api/.ticket/tickets/ee43f72e-53ef-4937-8216-92e17f185d85/ticket.toml`


### Component: session-api

<!-- ticket-index:entry id=857593ec-11d6-4c73-b5d8-7bf3b7eadd37 slug=new/session-api digest=b648a650e37f -->
#### [857593ec] [session-api] Token-efficient transcript range peeking
- priority: `high`
- summary: Provide a token-efficient way to inspect a specific range of turns in a persisted session transcript without reading the entire file. This mirrors the `peek` CLI tool's line-range behavior but operat...
- ref: `.ticket/tickets/857593ec-11d6-4c73-b5d8-7bf3b7eadd37/ticket.toml`

<!-- ticket-index:entry id=8cfdce69-a8b5-49dd-8ad7-96518bf0b8cc slug=new/session-api digest=4a7ce18c72b9 -->
#### [8cfdce69] [session-api] Transcript skeleton peeking
- priority: `high`
- summary: Provide a token-efficient way to inspect the structure of a session transcript by returning only the metadata/signatures of turns (sequence, role, captured_at, tool_name, and content length/summary) ...
- ref: `.ticket/tickets/8cfdce69-a8b5-49dd-8ad7-96518bf0b8cc/ticket.toml`

<!-- ticket-index:entry id=3d535b2c-7361-4f08-bfb4-63b0b3174afc slug=new/session-api digest=f05ec5bb1491 -->
#### [3d535b2c] [session-api][workflow] Add prompt-time worktree bootstrap hook
- priority: `high`
- summary: Add a pre-session bootstrap hook that establishes the session and its authoritative
- ref: `.ticket/tickets/3d535b2c-7361-4f08-bfb4-63b0b3174afc/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=29bf9628-1dc5-4bb4-ae00-b7410dd52db5 slug=new/spec-api digest=8bb3530d1164 -->
#### [29bf9628] [spec-api] Add direct feedback on spec entities with integration tests
- priority: `high`
- summary: Agents can now attach ratings and notes to canonical rule entries, but they still cannot attach feedback directly to native `spec-api` entities. Today the only supported workaround is to resolve a ge...
- ref: `memory-viewers/memory-api/.ticket/tickets/29bf9628-1dc5-4bb4-ae00-b7410dd52db5/ticket.toml`

<!-- ticket-index:entry id=f22d5297-3f60-4161-bf90-1eb56f3ced5d slug=new/spec-api digest=bfa52529ae73 -->
#### [f22d5297] [spec] spec-api: list canonical component entities
- summary: `SpecManifest` already carries a `component` field, and the spec HTTP layer can filter specs by component, but there is no first-class way to list the canonical component set from `spec-api`.
- ref: `.ticket/tickets/f22d5297-3f60-4161-bf90-1eb56f3ced5d/ticket.toml`

<!-- ticket-index:entry id=0f33944e-fab7-4d8c-b3bb-3c665d51854f slug=new/spec-api digest=108d107e0e25 -->
#### [0f33944e] [spec][P3] spec-api: edge management (parent_of, linked, depends_on)
- priority: `high`
- summary: `spec-api`'s schema declares three edge kinds (`parent_of`, `linked`,
- ref: `memory-viewers/memory-api/.ticket/tickets/0f33944e-fab7-4d8c-b3bb-3c665d51854f/ticket.toml`


### Component: spec-system

<!-- ticket-index:entry id=ee3864e1-7f9a-4804-bff0-8d861f4549da slug=new/spec-system digest=326136c37e93 -->
#### [ee3864e1] Epic: Specification System — memory-api extraction, spec-api, tooling, and skill generation
- priority: `critical`
- summary: Build a complete specification and documentation management system that:
- ref: `memory-viewers/memory-api/.ticket/tickets/ee3864e1-7f9a-4804-bff0-8d861f4549da/ticket.toml`


### Component: spec-viewer

<!-- ticket-index:entry id=88f87410-e0fa-4196-a461-805050670d08 slug=new/spec-viewer digest=90797ddcbcda -->
#### [88f87410] [spec-viewer] Integrate graph improvements (selection, rendering tiers, panel framing, 2D mode)
- priority: `high`
- summary: Integrate the four graph improvements into spec-viewer:
- ref: `.ticket/tickets/88f87410-e0fa-4196-a461-805050670d08/ticket.toml`


### Component: spec-vscode

<!-- ticket-index:entry id=79b0ad85-c57d-4237-bc59-281fa1ad57f8 slug=new/spec-vscode digest=c9a0a18bcc49 -->
#### [79b0ad85] [spec][vscode] Design and implement spec-vscode VS Code extension
- priority: `high`
- summary: Design and implement `spec-vscode` — a VS Code extension that surfaces the spec store in the sidebar, mirroring the patterns established by `ticket-vscode`. The extension allows developers to browse,...
- ref: `memory-viewers/memory-api/.ticket/tickets/79b0ad85-c57d-4237-bc59-281fa1ad57f8/ticket.toml`


### Component: test-api

<!-- ticket-index:entry id=a72e3aca-1e95-4fc5-a5b9-701112dcc37e slug=new/test-api digest=dfa7c3964be3 -->
#### [a72e3aca] [memory-index] Test store catalog generator
- priority: `medium`
- summary: Build a generator that reads the test-api and log-api evidence stores and emits a markdown test catalog at `.test/README.md` and `.test/index.toon`. Gated behind dependent log/test-api bootstrap comp...
- ref: `.ticket/tickets/a72e3aca-1e95-4fc5-a5b9-701112dcc37e/ticket.toml`

<!-- ticket-index:entry id=5a4c2e4d-e7d9-4138-8f25-c699942f739a slug=new/test-api digest=a93210272092 -->
#### [5a4c2e4d] [test-api] Add first-class validation spec and result storage
- priority: `high`
- summary: Add a first-class `test-api` for validation specifications and validation results in the memory system.
- ref: `.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a/ticket.toml`


### Component: ticket-api

<!-- ticket-index:entry id=f2285a55-91d1-48ad-9af7-e8c55ce9bd4d slug=new/ticket-api digest=6d457424bb06 -->
#### [f2285a55] [spec] spec-api <-> ticket-api: link component entities and detect drift
- summary: Tickets still exist with stale component values such as `log-viewer-leptos`, even though that component has been removed. `ticket-api` currently treats `fields.component` as a free-form string, so ou...
- ref: `.ticket/tickets/f2285a55-91d1-48ad-9af7-e8c55ce9bd4d/ticket.toml`

<!-- ticket-index:entry id=de36ac72-7f86-44af-875b-fd92bd628be9 slug=new/ticket-api digest=3e28265e37f5 -->
#### [de36ac72] [ticket-api][ticket-cli][ticket-mcp][ticket-http][ticket-viewer] Implement blocker trees and recently-unblocked ordering
- priority: `high`
- summary: Implement blocker-tree workflow exploration and recently-unblocked ordering on top of the shared dependency-convergence model.
- ref: `.ticket/tickets/de36ac72-7f86-44af-875b-fd92bd628be9/ticket.toml`

<!-- ticket-index:entry id=ec9498ee-e7a0-4b93-9543-393ac48c08fa slug=new/ticket-api digest=f904945196ed -->
#### [ec9498ee] [ticket-api][ticket-http][ticket-viewer] Harden workspace linking and search architecture
- priority: `high`
- summary: Harden workspace linking and search beyond the immediate bug fixes by closing the remaining design holes identified in review.
- ref: `.ticket/tickets/ec9498ee-e7a0-4b93-9543-393ac48c08fa/ticket.toml`


### Component: ticket-api,audit-api

<!-- ticket-index:entry id=5ad5ab28-6c81-4916-9574-d2c470e03a31 slug=new/ticket-api,audit-api digest=c0a0c46afff2 -->
#### [5ad5ab28] [ticket-api][audit-api] Strengthen canonical ticket health validation
- priority: `high`
- summary: Improve ticket health audit validation so the repository gets more signal from ticket quality checks, especially for newly created tickets.
- ref: `.ticket/tickets/5ad5ab28-6c81-4916-9574-d2c470e03a31/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=4e2b2b0b-9f56-4786-991c-8f10e653f4c3 slug=new/ticket-viewer digest=173119b6cbd7 -->
#### [4e2b2b0b] Epic: ticket-viewer UI polish — theme consistency, transparent buttons, shared crates, tiling panels
- priority: `high`
- summary: Visual inspection of the running ticket-viewer (http://localhost:3002) revealed four classes of UI defects, ranging from a CSS variable bug to missing platform-level infrastructure. This epic groups ...
- ref: `memory-viewers/.ticket/tickets/4e2b2b0b-9f56-4786-991c-8f10e653f4c3/ticket.toml`

<!-- ticket-index:entry id=dd80a182-8d0d-4439-bb59-668b9e6a5672 slug=new/ticket-viewer digest=862da9c67f61 -->
#### [dd80a182] Feature: History timeline — revision viewer with field diffs
- priority: `medium`
- ref: `memory-viewers/.ticket/tickets/dd80a182-8d0d-4439-bb59-668b9e6a5672/ticket.toml`

<!-- ticket-index:entry id=53a6d689-7d31-40ce-b807-4314285b4bfd slug=new/ticket-viewer digest=9cbe456a0ec4 -->
#### [53a6d689] [ticket-viewer] Add mixed-workspace endpoint ownership matrix regression tests
- priority: `high`
- summary: Create endpoint-matrix regression tests that validate workspace ownership semantics for mixed-workspace ticket references.
- ref: `.ticket/tickets/53a6d689-7d31-40ce-b807-4314285b4bfd/ticket.toml`

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

<!-- ticket-index:entry id=0866e27f-ae67-4eb0-9199-00650317e7c3 slug=new/ticket-viewer digest=8234fcc482b8 -->
#### [0866e27f] [ticket-viewer] Fix asset follow-up file selection and owning-workspace fetch
- priority: `high`
- summary: Clicking an expanded asset row in the ticket tree does not reliably propagate selected_file state and does not trigger the owning-workspace /asset follow-up request.
- ref: `.ticket/tickets/0866e27f-ae67-4eb0-9199-00650317e7c3/ticket.toml`

<!-- ticket-index:entry id=178b4091-53c9-45ae-b975-890a23b5f25d slug=new/ticket-viewer digest=a9205eba95a4 -->
#### [178b4091] [ticket-viewer] Normalize release E2E suite to workspace-aware assumptions
- priority: `medium`
- summary: Reduce false confidence from legacy default-workspace assumptions by updating release E2E tests to workspace-aware ticket-reference behavior.
- ref: `.ticket/tickets/178b4091-53c9-45ae-b975-890a23b5f25d/ticket.toml`

<!-- ticket-index:entry id=c33419c2-3fff-4ce2-9b53-8882d6918e53 slug=new/ticket-viewer digest=90db49954c19 -->
#### [c33419c2] [ticket-viewer] Tracker: complete mixed-workspace regression coverage
- priority: `high`
- summary: This tracker captures the regression-test completeness review for the ticket-viewer mixed-workspace rollout.
- ref: `.ticket/tickets/c33419c2-3fff-4ce2-9b53-8882d6918e53/ticket.toml`

<!-- ticket-index:entry id=05dae5fd-1a1d-4a64-be62-f29ca0771a4d slug=new/ticket-viewer digest=e7858d7e6e8c -->
#### [05dae5fd] [ticket-viewer][ticket-http][viewer-api] Improve main layout ticket documents and focused full-graph navigation
- priority: `high`
- summary: Upgrade the ticket-viewer main layout so ticket details render as a compact integrated document and the graph view becomes a focused full-workspace navigation surface with better layout, settings, an...
- ref: `memory-viewers/.ticket/tickets/05dae5fd-1a1d-4a64-be62-f29ca0771a4d/ticket.toml`


### Component: ticket-workflow

<!-- ticket-index:entry id=7fc7a10d-64a1-4c67-a5a9-5b45d8e03047 slug=new/ticket-workflow digest=53317e4963de -->
#### [7fc7a10d] Project tracker: expressive ticket query and ordering interface
- priority: `high`
- summary: The ticket query surface is still fragmented and under-specified.
- ref: `.ticket/tickets/7fc7a10d-64a1-4c67-a5a9-5b45d8e03047/ticket.toml`


### Component: tools/viewer/log-viewer/frontend/dioxus

<!-- ticket-index:entry id=6d0dc335-693a-450e-92ba-9cdaa4087afa slug=new/tools/viewer/log-viewer/frontend/dioxus digest=456b73e11705 -->
#### [6d0dc335] [LOG-5e] Port log-viewer visualization tabs and overlay-backed tooling to viewer-api-dioxus
- priority: `p2`
- summary: The current log-viewer frontend uses the shared viewer-api frontend for more than the log list. `App.tsx` mounts a shared `WgpuOverlay` and exposes multiple non-trivial surfaces beyond the basic brow...
- ref: `.ticket/tickets/6d0dc335-693a-450e-92ba-9cdaa4087afa/ticket.toml`

<!-- ticket-index:entry id=b22b2a49-1e3a-40f4-b534-0f5e86610da7 slug=new/tools/viewer/log-viewer/frontend/dioxus digest=5e972b855719 -->
#### [b22b2a49] [LOG-5f] Cut log-viewer over from Preact/Vite to Dioxus/trunk and lock in migration validation
- priority: `p2`
- summary: The current migration ticket set scaffolds and ports features, but it does not include the final cutover step: making the Dioxus frontend the default served build, preventing long-term drift between ...
- ref: `.ticket/tickets/b22b2a49-1e3a-40f4-b534-0f5e86610da7/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=03b7ce45-f2cb-4a5f-9b57-ef4c442b3515 slug=new/unspecified digest=9b00cde2f2f7 -->
#### [03b7ce45] Dioxus theme settings: Background Smoke section
- priority: `medium`
- summary: The canonical theme settings spec requires a "Background Smoke" section (§2 row 15) with
- ref: `.ticket/tickets/03b7ce45-f2cb-4a5f-9b57-ef4c442b3515/ticket.toml`

<!-- ticket-index:entry id=0ae1faf1-3554-45de-9d79-dc1be97de707 slug=new/unspecified digest=44b4cc7c0df0 -->
#### [0ae1faf1] Dioxus theme settings: Glass Panels + CRT Effect controls
- priority: `medium`
- summary: The canonical theme settings spec requires the "Glass Panels" section (§2 row 16,
- ref: `.ticket/tickets/0ae1faf1-3554-45de-9d79-dc1be97de707/ticket.toml`

<!-- ticket-index:entry id=6b51228e-1686-41fc-ac0c-7e19f58d657f slug=new/unspecified digest=69e84133185c -->
#### [6b51228e] Dioxus theme settings: Theme Presets grid + header actions
- priority: `medium`
- summary: The canonical theme settings spec (§2 row 1, default open) requires a Theme Presets grid
- ref: `.ticket/tickets/6b51228e-1686-41fc-ac0c-7e19f58d657f/ticket.toml`

<!-- ticket-index:entry id=47438a4f-64f8-4619-93ff-ea4355092606 slug=new/unspecified digest=e5f0c52f4704 -->
#### [47438a4f] Dioxus theme settings: full ColorRows for all ThemeColors
- priority: `medium`
- summary: The canonical theme settings spec (§2 rows 2–8) requires color rows for every entry in
- ref: `.ticket/tickets/47438a4f-64f8-4619-93ff-ea4355092606/ticket.toml`

<!-- ticket-index:entry id=fe9b450e-94c2-4d01-9ac9-ff993f45a591 slug=new/unspecified digest=86a92ff3878f -->
#### [fe9b450e] Dioxus theme settings: per-effect controls (Sparks/Embers/Beams/Glitter/Cinder)
- priority: `medium`
- summary: The shared canonical theme settings spec (`viewer-api/theme-settings`) requires per-effect
- ref: `.ticket/tickets/fe9b450e-94c2-4d01-9ac9-ff993f45a591/ticket.toml`

<!-- ticket-index:entry id=def88d4e-8a3c-45bc-82c8-bdacae01a479 slug=new/unspecified digest=7e5166367259 -->
#### [def88d4e] Epic: Unified Logging Infrastructure — file sinks, search, Mermaid, table, Dioxus frontend
- summary: Provide every viewer-api tool and context-* crate with consistent, queryable, structured logging.
- ref: `.ticket/tickets/def88d4e-8a3c-45bc-82c8-bdacae01a479/ticket.toml`

<!-- ticket-index:entry id=80afa16d-2ea9-4eff-96be-8c4f044ff159 slug=new/unspecified digest=4fc3f60926fd -->
#### [80afa16d] Probe Ticket
- ref: `.ticket/tickets/80afa16d-2ea9-4eff-96be-8c4f044ff159/ticket.toml`

<!-- ticket-index:entry id=1a1e3953-5275-407f-b690-428bdb90db7b slug=new/unspecified digest=860967a6d0be -->
#### [1a1e3953] Project tracker: Dioxus theme settings backlog
- summary: Group the Dioxus theme settings backlog under a single tracker so those related tickets have a shared parent in the dependency graph.
- ref: `.ticket/tickets/1a1e3953-5275-407f-b690-428bdb90db7b/ticket.toml`

<!-- ticket-index:entry id=53f471a3-8a55-40ca-8f86-5da3b15aa25e slug=new/unspecified digest=fad4636fdc6e -->
#### [53f471a3] Project tracker: audit quality backlog
- summary: Project tracker for the audit quality backlog: hardening repository quality audits so ticket/spec/graph health produce more actionable signal.
- ref: `memory-viewers/memory-api/.ticket/tickets/53f471a3-8a55-40ca-8f86-5da3b15aa25e/ticket.toml`

<!-- ticket-index:entry id=186caf8a-bbbf-426f-8fc3-2f7882a8a550 slug=new/unspecified digest=6efb87704be0 -->
#### [186caf8a] Project tracker: board workflow rollout
- ref: `memory-viewers/memory-api/.ticket/tickets/186caf8a-bbbf-426f-8fc3-2f7882a8a550/ticket.toml`

<!-- ticket-index:entry id=72bad0e5-2f5d-4731-8cc6-8d4b167418dd slug=new/unspecified digest=4cef4e39a627 -->
#### [72bad0e5] Project tracker: bootstrap executor backlog
- ref: `memory-viewers/memory-api/.ticket/tickets/72bad0e5-2f5d-4731-8cc6-8d4b167418dd/ticket.toml`

<!-- ticket-index:entry id=02a3a2a7-1e70-4d25-a86c-17a36e5dd5e1 slug=new/unspecified digest=51c5a735c000 -->
#### [02a3a2a7] Project tracker: cargo doc workspace support
- ref: `memory-viewers/memory-api/.ticket/tickets/02a3a2a7-1e70-4d25-a86c-17a36e5dd5e1/ticket.toml`

<!-- ticket-index:entry id=be47f545-c72a-43bd-a804-dd9665ce8faa slug=new/unspecified digest=13d593549e7c -->
#### [be47f545] Project tracker: doc validation and install workflow redesign
- summary: Group the documented validation and install-flow redesign tickets under a single parent so the doc-validation backlog is connected into the wider workflow environment.
- ref: `.ticket/tickets/be47f545-c72a-43bd-a804-dd9665ce8faa/ticket.toml`

<!-- ticket-index:entry id=06b4f7d5-eee9-4266-9793-8c18a5bcf745 slug=new/unspecified digest=34b344722e1a -->
#### [06b4f7d5] Project tracker: log tooling and viewer migration
- summary: Group the log schema, log API, and log viewer migration tickets under a single tracker so the subsystem has a coherent parent in the ticket graph.
- ref: `.ticket/tickets/06b4f7d5-eee9-4266-9793-8c18a5bcf745/ticket.toml`

<!-- ticket-index:entry id=a76c72e6-0bb9-48ef-be43-37c72ad89002 slug=new/unspecified digest=a6c323a6206c -->
#### [a76c72e6] Project tracker: rule-api hierarchy and documentation pipeline
- ref: `memory-viewers/memory-api/.ticket/tickets/a76c72e6-0bb9-48ef-be43-37c72ad89002/ticket.toml`

<!-- ticket-index:entry id=0af903c0-4f97-4773-b277-51dcf278b1f0 slug=new/unspecified digest=8556760602b7 -->
#### [0af903c0] Project tracker: ticket API and mutation surfaces
- summary: Group ticket-api, ticket-http, and ticket-mcp mutation and storage-surface work under a single parent so related graph, mutation, and round-trip issues share a coherent backlog home.
- ref: `memory-viewers/memory-api/.ticket/tickets/0af903c0-4f97-4773-b277-51dcf278b1f0/ticket.toml`

<!-- ticket-index:entry id=40ba5a15-df3c-42f2-8825-bd43bd66fce7 slug=new/unspecified digest=c0b0fb4c2e34 -->
#### [40ba5a15] Project tracker: ticket CLI and next-work backlog
- summary: Group the ticket CLI, board, MCP, and next-work backlog items under one parent so the discovery and workflow contract is represented in the dependency graph.
- ref: `.ticket/tickets/40ba5a15-df3c-42f2-8825-bd43bd66fce7/ticket.toml`

<!-- ticket-index:entry id=171eb277-3270-4d52-8283-10cf3dd939b9 slug=new/unspecified digest=e354a93833fc -->
#### [171eb277] Project tracker: ticket UX surfaces backlog
- ref: `memory-viewers/memory-api/.ticket/tickets/171eb277-3270-4d52-8283-10cf3dd939b9/ticket.toml`

<!-- ticket-index:entry id=79efa73e-62d8-4c91-b0b5-b1ad79262efa slug=new/unspecified digest=215c8ac7d31d -->
#### [79efa73e] Project tracker: ticket metadata and content pipeline
- ref: `memory-viewers/memory-api/.ticket/tickets/79efa73e-62d8-4c91-b0b5-b1ad79262efa/ticket.toml`

<!-- ticket-index:entry id=026401a0-e099-4d8b-840d-2d6b3bb456f3 slug=new/unspecified digest=aa542759db3c -->
#### [026401a0] Project tracker: ticket query filter correctness
- ref: `memory-viewers/memory-api/.ticket/tickets/026401a0-e099-4d8b-840d-2d6b3bb456f3/ticket.toml`

<!-- ticket-index:entry id=9df4ef26-5168-4bbb-adf4-7f0e4f7ae3cf slug=new/unspecified digest=3cb04c50d0fc -->
#### [9df4ef26] Project tracker: ticket/spec/rule operator workflow and discoverability
- summary: Group the unresolved operator-facing workflow and discoverability gaps surfaced while reviewing ticket, spec, and rule tool usage during the May 20-21 session.
- ref: `.ticket/tickets/9df4ef26-5168-4bbb-adf4-7f0e4f7ae3cf/ticket.toml`

<!-- ticket-index:entry id=f93f2266-7f97-4f31-a548-706c7a7e8c4a slug=new/unspecified digest=39f73e451041 -->
#### [f93f2266] Project tracker: viewer logging rollout
- ref: `memory-viewers/memory-api/.ticket/tickets/f93f2266-7f97-4f31-a548-706c7a7e8c4a/ticket.toml`

<!-- ticket-index:entry id=afcf2759-9c91-433c-b62c-ae8adcb0cdd5 slug=new/unspecified digest=a3e69a61064d -->
#### [afcf2759] Project tracker: workflow traceability redesign
- summary: Group the workflow metadata and cross-store traceability redesign tickets under a single parent so the backlog is connected into the broader dependency graph.
- ref: `.ticket/tickets/afcf2759-9c91-433c-b62c-ae8adcb0cdd5/ticket.toml`

<!-- ticket-index:entry id=e4de2cdc-48d0-42b4-92df-900da88e156f slug=new/unspecified digest=16858c916c2b -->
#### [e4de2cdc] Rename ticket dependency labels
- summary: Rename ticket schema/display wording for dependency relationship fields: `dependees` label should become `dependee_count`, and `depends_on` should render as `Dependencies` in ticket schema/display fo...
- ref: `.ticket/tickets/e4de2cdc-48d0-42b4-92df-900da88e156f/ticket.toml`

<!-- ticket-index:entry id=ff2872ad-74be-4e5d-a7ba-416c73506252 slug=new/unspecified digest=8855274e2633 -->
#### [ff2872ad] Restore ticket-vscode listing when server is already running
- summary: Improved ticket-vscode error-state surfacing so provider failures now include caller context and the associated request details. The API client throws structured request errors with operation, method...
- ref: `.ticket/tickets/ff2872ad-74be-4e5d-a7ba-416c73506252/ticket.toml`

<!-- ticket-index:entry id=1d8d82b5-8e40-463f-adaf-30d2f5625844 slug=new/unspecified digest=c742de2de502 -->
#### [1d8d82b5] [LOG-1a] ticket-viewer: wire init_tracing_full with file logging to target/logs/"
- summary: `tools/viewer/ticket-viewer/src/main.rs` calls `init_tracing("info,ticket_http::serve::handlers=debug")`. All structured log output goes to stderr only. When started in detached mode (default `viewer...
- ref: `memory-viewers/.ticket/tickets/1d8d82b5-8e40-463f-adaf-30d2f5625844/ticket.toml`

<!-- ticket-index:entry id=60a2a388-c8b6-4e25-a80a-0ba686f11bf9 slug=new/unspecified digest=5169234ff296 -->
#### [60a2a388] [LOG-1b] doc-viewer + spec-viewer: wire init_tracing_full with file logging
- summary: `doc-viewer` and `spec-viewer` (if they have `main.rs` entry points) currently use `init_tracing()` (console-only) or have no explicit tracing setup. Logs are lost in detached mode.
- ref: `memory-viewers/.ticket/tickets/60a2a388-c8b6-4e25-a80a-0ba686f11bf9/ticket.toml`

<!-- ticket-index:entry id=3b1345eb-9983-4567-a3ab-c2e00c7cec1e slug=new/unspecified digest=f5b157ef322e -->
#### [3b1345eb] [LOG-1c] viewer-ctl: add --log-dir and --log-level flags to start/restart
- summary: There is no way to control the log directory or level for a viewer server launched via `viewer-ctl start` without modifying the binary's default env vars by hand. Operators cannot redirect logs to a ...
- ref: `memory-viewers/viewer-api/.ticket/tickets/3b1345eb-9983-4567-a3ab-c2e00c7cec1e/ticket.toml`

<!-- ticket-index:entry id=12197242-b7b4-4212-83a8-4b0b65a4bd7b slug=new/unspecified digest=21baf71a4cb7 -->
#### [12197242] [LOG-2a] Audit and normalise context-* tracing field names for log-viewer compatibility
- summary: The `crates/context-{insert,read,search,trace}` crates emit `tracing` spans and events but field names, targets, and event shapes are not uniform. The log-viewer parser (`crates/context-api/src/log_p...
- ref: `memory-viewers/memory-api/.ticket/tickets/12197242-b7b4-4212-83a8-4b0b65a4bd7b/ticket.toml`

<!-- ticket-index:entry id=5b4330f6-f1d0-4e80-8a3e-296f557c5a99 slug=new/unspecified digest=160b76a4906e -->
#### [5b4330f6] [LOG-2b] Add context-trace JSON format compatibility test against log-viewer parser
- summary: `crates/context-stack/context-trace` uses a `PrettyJsonWriter` to produce structured log output, and `crates/context-stack/context-api/src/log_parser.rs` parses it. There are no automated tests ensur...
- ref: `.ticket/tickets/5b4330f6-f1d0-4e80-8a3e-296f557c5a99/ticket.toml`

<!-- ticket-index:entry id=159a9862-6ea3-4966-a2b1-992e1d03b578 slug=new/unspecified digest=57a6092ca437 -->
#### [159a9862] [LOG-3a] Log schema-field search: add search_fields MCP tool and HTTP endpoint
- summary: `mcp_log-viewer-mc_query_logs` accepts a raw JQ expression, which is powerful but requires users to know the exact field path. There is no dedicated "find all log entries where field X = Y" API that ...
- ref: `.ticket/tickets/159a9862-6ea3-4966-a2b1-992e1d03b578/ticket.toml`

<!-- ticket-index:entry id=b3fc711c-8c8d-4e3f-a76b-f00c551d9d49 slug=new/unspecified digest=4ba07a36771a -->
#### [b3fc711c] [LOG-3b] Log full-text search: add search_text MCP tool with regex and context-lines support
- summary: `mcp_log-viewer-mc_search_all_logs` accepts a JQ expression, not a plain text or regex query. There is no simple "grep for a string across log files" interface for users who need a quick `contains("p...
- ref: `.ticket/tickets/b3fc711c-8c8d-4e3f-a76b-f00c551d9d49/ticket.toml`

<!-- ticket-index:entry id=40a4bc9e-7ecd-4fa4-b842-633891bd5cba slug=new/unspecified digest=242ebbda23df -->
#### [40a4bc9e] [LOG-4a] Log-to-Mermaid: convert filtered log session to sequenceDiagram
- summary: When debugging multi-component interactions (e.g. `ticket-http` handler → `TicketStore` → `tracing` spans), engineers have no automatic way to visualise the call flow as a Mermaid `sequenceDiagram`. ...
- ref: `.ticket/tickets/40a4bc9e-7ecd-4fa4-b842-633891bd5cba/ticket.toml`

<!-- ticket-index:entry id=f37bdd68-b2d7-4c4b-93b9-fa9d4f61c4a6 slug=new/unspecified digest=88680649a3b7 -->
#### [f37bdd68] [LOG-4b] Log-to-table: render filtered log view as ASCII/Markdown table (MCP + HTTP)
- summary: There is no way to get a quick tabular summary of log entries for terminal/CLI use or for pasting into documentation. Engineers must write JQ expressions and post-process JSON manually.
- ref: `.ticket/tickets/f37bdd68-b2d7-4c4b-93b9-fa9d4f61c4a6/ticket.toml`

<!-- ticket-index:entry id=972c239e-e110-49da-9449-8bdcfaea5f18 slug=new/unspecified digest=2d58228b0530 -->
#### [972c239e] [LOG-5b] Port log-viewer browser UI to Dioxus: file tree, entry list, search bar, stats
- summary: After the scaffold ([LOG-5a]), the Dioxus log-viewer is a stub. This ticket ports the core browsing UI from the Preact frontend to Dioxus components with feature parity.
- ref: `.ticket/tickets/972c239e-e110-49da-9449-8bdcfaea5f18/ticket.toml`

<!-- ticket-index:entry id=bfb95499-ac12-4cd4-808f-879795a938e5 slug=new/unspecified digest=1f2a30cba428 -->
#### [bfb95499] [LOG-5c] Add live-tail view to log-viewer-dioxus: SSE endpoint and real-time browser component
- summary: Engineers launching a server with `viewer-ctl start --fg` need to see live log output in the browser rather than switching between the terminal and the viewer. There is no streaming/tail view in eith...
- ref: `.ticket/tickets/bfb95499-ac12-4cd4-808f-879795a938e5/ticket.toml`

<!-- ticket-index:entry id=01b6fe40-5741-41b5-a73c-e4bd51b49a3f slug=new/unspecified digest=28cf740a305e -->
#### [01b6fe40] [architecture][contracts] Binary composition root wiring
- priority: `medium`
- summary: Wire contracts in binary composition roots using static typing and remove ad-hoc direct coupling.
- ref: `.ticket/tickets/01b6fe40-5741-41b5-a73c-e4bd51b49a3f/ticket.toml`

<!-- ticket-index:entry id=65ea4528-af24-4300-8e5f-3b68e54711d0 slug=new/unspecified digest=7112dae1b710 -->
#### [65ea4528] [architecture][contracts] Core shared contract crate
- priority: `high`
- summary: Define the core shared contract crate for cross-store interaction primitives.
- ref: `.ticket/tickets/65ea4528-af24-4300-8e5f-3b68e54711d0/ticket.toml`

<!-- ticket-index:entry id=d86c66a9-ebe1-4b13-a3a4-87f4246b3062 slug=new/unspecified digest=65dd5c44f4a2 -->
#### [d86c66a9] [architecture][contracts] Domain extension contracts and first provider
- priority: `high`
- summary: Define domain extension contract crates and implement first provider/consumer pair.
- ref: `.ticket/tickets/d86c66a9-ebe1-4b13-a3a4-87f4246b3062/ticket.toml`

<!-- ticket-index:entry id=0f2be510-378a-40eb-a98c-ab516b0ec647 slug=new/unspecified digest=df2c5290fadd -->
#### [0f2be510] [architecture][contracts] IoC contract crates for cross-store interactions
- priority: `high`
- summary: Define and adopt a hybrid cross-store contract layer so domain crates interact via inversion of control rather than direct domain coupling.
- ref: `.ticket/tickets/0f2be510-378a-40eb-a98c-ab516b0ec647/ticket.toml`

<!-- ticket-index:entry id=37e07148-c327-4530-8251-599c14dca04e slug=new/unspecified digest=6f814b0e87ed -->
#### [37e07148] [architecture][memory-api] Implement neutral shared storage kernel APIs
- priority: `high`
- summary: Implement neutral shared storage/index/search symbols in memory-api with compatibility aliases.
- ref: `.ticket/tickets/37e07148-c327-4530-8251-599c14dca04e/ticket.toml`

<!-- ticket-index:entry id=13912e44-fee8-4aa5-b28f-68bbc22af401 slug=new/unspecified digest=d70218d92686 -->
#### [13912e44] [architecture][memory-api] Neutral naming migration map
- priority: `high`
- summary: Create a concrete neutral naming migration map for shared memory-api storage/index/search APIs.
- ref: `.ticket/tickets/13912e44-fee8-4aa5-b28f-68bbc22af401/ticket.toml`

<!-- ticket-index:entry id=2b1279bd-c42f-4b0e-8835-d0d645a733ab slug=new/unspecified digest=6bbf89da96a2 -->
#### [2b1279bd] [architecture][memory-api] Neutral storage kernel and API migration
- priority: `high`
- summary: Refactor memory-api shared storage/index/search APIs to domain-neutral semantics (`entity`, `store`, `workspace`) and isolate ticket-only behavior from shared storage internals.
- ref: `.ticket/tickets/2b1279bd-c42f-4b0e-8835-d0d645a733ab/ticket.toml`

<!-- ticket-index:entry id=671d4e47-b53d-4a04-aa1d-30f2aa8a2bbe slug=new/unspecified digest=1cf5e66398f4 -->
#### [671d4e47] [architecture][multi-store] Tracker: cross-store interaction model and migration
- priority: `high`
- summary: Goal: deliver a workspace-wide, domain-isolated multi-store architecture where each store owns persistence and workflow behavior while cross-store interactions are defined by shared contract interfac...
- ref: `.ticket/tickets/671d4e47-b53d-4a04-aa1d-30f2aa8a2bbe/ticket.toml`

<!-- ticket-index:entry id=834632eb-7c0f-4e43-b1ca-3793141e25d8 slug=new/unspecified digest=a9ae9706bd49 -->
#### [834632eb] [architecture][observability] CLI and MCP extended error envelope adoption
- priority: `high`
- summary: Adopt extended error envelope in CLI and MCP surfaces.
- ref: `.ticket/tickets/834632eb-7c0f-4e43-b1ca-3793141e25d8/ticket.toml`

<!-- ticket-index:entry id=d8b5cfd0-8516-4dbe-84da-be112f6e5a57 slug=new/unspecified digest=63a6a39c6f0f -->
#### [d8b5cfd0] [architecture][observability] Extended error envelope schema and mapping rules
- priority: `high`
- summary: Define extended cross-channel error envelope schema and mapping rules.
- ref: `.ticket/tickets/d8b5cfd0-8516-4dbe-84da-be112f6e5a57/ticket.toml`

<!-- ticket-index:entry id=726efe80-3dc4-4b2d-9817-fb2b91b74441 slug=new/unspecified digest=8f3c10d30bcc -->
#### [726efe80] [architecture][observability] HTTP extended error envelope adoption
- priority: `high`
- summary: Adopt extended error envelope in HTTP surfaces and trace correlation.
- ref: `.ticket/tickets/726efe80-3dc4-4b2d-9817-fb2b91b74441/ticket.toml`

<!-- ticket-index:entry id=d03530c6-52e4-42d3-8d57-e750ce73c8d4 slug=new/unspecified digest=af60ad1289ec -->
#### [d03530c6] [architecture][observability] Unified traceable error channels across stores
- priority: `high`
- summary: Standardize error tracing and user-facing diagnostics across store CLIs, MCP servers, and HTTP handlers using one extended envelope contract.
- ref: `.ticket/tickets/d03530c6-52e4-42d3-8d57-e750ce73c8d4/ticket.toml`

<!-- ticket-index:entry id=e2768479-24d6-4f42-bdbe-ac509167dc62 slug=new/unspecified digest=955917d85bdb -->
#### [e2768479] [architecture][rule-spec] Adopt neutral shared APIs in rule-api and spec-api
- priority: `high`
- summary: Migrate rule-api and spec-api to consume neutral memory-api shared APIs.
- ref: `.ticket/tickets/e2768479-24d6-4f42-bdbe-ac509167dc62/ticket.toml`

<!-- ticket-index:entry id=999d9316-fc79-4bb1-b629-7cba52eced31 slug=new/unspecified digest=472454244347 -->
#### [999d9316] [architecture][ticket-api] Adopt neutral shared APIs and alias retirement gate
- priority: `high`
- summary: Migrate ticket-api internal usage to neutral shared APIs and define alias retirement gate.
- ref: `.ticket/tickets/999d9316-fc79-4bb1-b629-7cba52eced31/ticket.toml`

<!-- ticket-index:entry id=6bd67a7a-2a76-4dd7-a897-b4d325476621 slug=new/unspecified digest=c458c3e8bbfd -->
#### [6bd67a7a] [architecture][workspace] Dynamic multi-store discovery and cross-store references
- priority: `high`
- summary: Implement recursive multi-store workspace discovery and cross-store reference integration with URN-based identities across local and nested workspaces.
- ref: `.ticket/tickets/6bd67a7a-2a76-4dd7-a897-b4d325476621/ticket.toml`

<!-- ticket-index:entry id=7e318b2a-a381-49a1-aee9-18758a4b80fd slug=new/unspecified digest=39a876862f0e -->
#### [7e318b2a] [architecture][workspace] Late store onboarding reconciliation
- priority: `high`
- summary: Support absent-then-present store integration and late onboarding reconciliation.
- ref: `.ticket/tickets/7e318b2a-a381-49a1-aee9-18758a4b80fd/ticket.toml`

<!-- ticket-index:entry id=fa3e0a51-0caa-4a33-bfe2-1b173feaa979 slug=new/unspecified digest=c1967dc4fe99 -->
#### [fa3e0a51] [architecture][workspace] Recursive automatic store discovery
- priority: `high`
- summary: Implement fully automatic recursive store discovery across local and nested workspaces.
- ref: `.ticket/tickets/fa3e0a51-0caa-4a33-bfe2-1b173feaa979/ticket.toml`

<!-- ticket-index:entry id=82d6ada4-ac35-45a7-9df6-7b7501d58e70 slug=new/unspecified digest=82adbd88d326 -->
#### [82d6ada4] [architecture][workspace] URN cross-store reference model and resolver
- priority: `high`
- summary: Implement URN-based cross-store reference model and resolver APIs.
- ref: `.ticket/tickets/82d6ada4-ac35-45a7-9df6-7b7501d58e70/ticket.toml`

<!-- ticket-index:entry id=11fb9bcf-fcd5-4eff-b380-64b80f4a5c9c slug=new/unspecified digest=2df32043fb86 -->
#### [11fb9bcf] [audit-api] Cleanup loop UX and automated remediation suggestions
- priority: `high`
- summary: Design user-facing triage loops, inboxes, and remediation hints for stale/outdated/conflicting rule/spec/ticket entries, including recurring audit cadence.
- ref: `.ticket/tickets/11fb9bcf-fcd5-4eff-b380-64b80f4a5c9c/ticket.toml`

<!-- ticket-index:entry id=bd1c7cc0-2850-418d-b701-981b95c587ee slug=new/unspecified digest=e5ab30755276 -->
#### [bd1c7cc0] [audit-api] Continuous store health scoring and cleanup loops
- priority: `high`
- summary: Plan continuous auditing to detect stale, conflicting, or low-value entries across spec/rule/ticket stores using activity, validation, and feedback signals.
- ref: `.ticket/tickets/bd1c7cc0-2850-418d-b701-981b95c587ee/ticket.toml`

<!-- ticket-index:entry id=67b6117b-5978-4c89-9cd4-4c8b043f4fba slug=new/unspecified digest=4b049c0a0669 -->
#### [67b6117b] [audit-api] Health metric taxonomy and scoring model for store entries
- priority: `high`
- summary: Define weighted health metrics (relevance, freshness, conflict, validation coverage, feedback sentiment, activity) and scoring thresholds for healthy/unhealthy entries.
- ref: `.ticket/tickets/67b6117b-5978-4c89-9cd4-4c8b043f4fba/ticket.toml`

<!-- ticket-index:entry id=8dbff37f-699b-4c91-bf65-6516ea6fe609 slug=new/unspecified digest=f0ebf425a19f -->
#### [8dbff37f] [audit-api] Workspace graph health and board check-in validation enforcement
- priority: `high`
- summary: Plan the audit and operator-enforcement layer for validation-aware ticket graphs.
- ref: `.ticket/tickets/8dbff37f-699b-4c91-bf65-6516ea6fe609/ticket.toml`

<!-- ticket-index:entry id=700127a8-f9a5-415d-a433-2d5b888e6292 slug=new/unspecified digest=f583e706ee3c -->
#### [700127a8] [context-editor] LLM Integration: Text-to-Voxel/Shader, Naga Validation & Hot-Reload
- priority: `high`
- summary: Players can type natural-language descriptions into the UI to procedurally generate voxel structures, custom shader effects, or skill modifiers. An LLM translates the prompt into either: (a) voxel co...
- ref: `.ticket/tickets/700127a8-f9a5-415d-a433-2d5b888e6292/ticket.toml`

<!-- ticket-index:entry id=5a87d7b2-6d58-41ee-bd74-dd0fc6fde5f1 slug=new/unspecified digest=f1253b9625da -->
#### [5a87d7b2] [context-editor][SDF-DAG] GPU SDF Collision & Force Kernel — DAG Traversal Physics
- priority: `high`
- summary: The current physics pipeline has two major gaps:
- ref: `.ticket/tickets/5a87d7b2-6d58-41ee-bd74-dd0fc6fde5f1/ticket.toml`

<!-- ticket-index:entry id=22dc5dfc-ac5d-46e0-979e-1f38ac4ce6c7 slug=new/unspecified digest=f3eec8618a1c -->
#### [22dc5dfc] [context-editor][SDF-DAG] Heterogeneous SDF Atom DAG Architecture — Epic
- priority: `high`
- summary: Replace the current flat-material Sparse Voxel Octree (SVO) with a Directed Acyclic Graph (DAG)
- ref: `.ticket/tickets/22dc5dfc-ac5d-46e0-979e-1f38ac4ce6c7/ticket.toml`

<!-- ticket-index:entry id=52ed521c-2774-40f9-95e1-7deca81d2f09 slug=new/unspecified digest=9dbaeae8434a -->
#### [52ed521c] [context-editor][SDF-DAG] Phase 1: Per-Voxel SDF Atom Type System with Typed Dispatch
- priority: `high`
- summary: The current `OctreeNode` stores only a flat `color_data: u32` (R8G8B8 + roughness5 + metallic1)
- ref: `.ticket/tickets/52ed521c-2774-40f9-95e1-7deca81d2f09/ticket.toml`

<!-- ticket-index:entry id=8f0ffc7c-b1d6-423d-bb9b-6f0c6b75852b slug=new/unspecified digest=b5ffbcc01a87 -->
#### [8f0ffc7c] [context-editor][SDF-DAG] Phase 2: DAG-Persistent Edit Operations with Hash Consing
- priority: `medium`
- summary: The current SVO uses a mutable flat `Vec<OctreeNode>` where every edit directly mutates nodes
- ref: `.ticket/tickets/8f0ffc7c-b1d6-423d-bb9b-6f0c6b75852b/ticket.toml`

<!-- ticket-index:entry id=6f368e20-00a4-4da5-9388-492ba4209915 slug=new/unspecified digest=bde8e73caca3 -->
#### [6f368e20] [context-editor][SDF-DAG] Phase 3: 4D Spatio-Temporal DAG — Keyframed SDF Animation & Replay
- priority: `medium`
- summary: The current renderer has no concept of time within the voxel structure. Physics and animation
- ref: `.ticket/tickets/6f368e20-00a4-4da5-9388-492ba4209915/ticket.toml`

<!-- ticket-index:entry id=a6fd15f6-f9c3-407a-af99-3febee5b2557 slug=new/unspecified digest=8f10bf6adc8d -->
#### [a6fd15f6] [doc-viewer][P6] (Deferred) Migrate doc-viewer onto shared viewer-api
- summary: Optional / deferred.** Once the Dioxus shared crate is stable and proven in spec-viewer, expose its components to doc-viewer (Preact/TS) via thin TypeScript bindings or a wasm-bindgen surface, elimin...
- ref: `memory-viewers/viewer-api/.ticket/tickets/a6fd15f6-f9c3-407a-af99-3febee5b2557/ticket.toml`

<!-- ticket-index:entry id=9c95c1e4-3cdb-428e-b9de-800684651226 slug=new/unspecified digest=161d78728551 -->
#### [9c95c1e4] [feedback-api] Event ingestion, metadata normalization, and retention policy
- priority: `high`
- summary: Define feedback event ingestion for human and privileged-agent authors, normalize metadata, and establish retention/privacy boundaries.
- ref: `.ticket/tickets/9c95c1e4-3cdb-428e-b9de-800684651226/ticket.toml`

<!-- ticket-index:entry id=b1e9e744-aeac-474a-91d9-07e3a362dc76 slug=new/unspecified digest=cfa95951b9c6 -->
#### [b1e9e744] [feedback-api] Feedback inbox, metadata indexing, and deep search
- priority: `high`
- summary: Plan a feedback store that ingests human and privileged-agent feedback events, normalizes metadata, and supports deep search and reconciliation at scale.
- ref: `.ticket/tickets/b1e9e744-aeac-474a-91d9-07e3a362dc76/ticket.toml`

<!-- ticket-index:entry id=b7b84c10-8dc5-4087-87ad-6fe27ebbcd45 slug=new/unspecified digest=254bc01e9701 -->
#### [b7b84c10] [feedback-api] High-scale search, clustering, and reconciliation workflows
- priority: `high`
- summary: Plan and implement deep query/search capabilities and operator reconciliation flows for large feedback corpora, including dedupe, sentiment facets, and routing.
- ref: `.ticket/tickets/b7b84c10-8dc5-4087-87ad-6fe27ebbcd45/ticket.toml`

<!-- ticket-index:entry id=0fc7b189-5c6c-4b79-a78d-5df8ad7dcf0c slug=new/unspecified digest=82d049ea603d -->
#### [0fc7b189] [interview-api] Actionable answer-sheet synthesis and iteration loop
- priority: `high`
- summary: Design synthesis pipeline that turns interview and survey responses into actionable sheets, supports iterative refinement, and records provenance links back to source responses.
- ref: `.ticket/tickets/0fc7b189-5c6c-4b79-a78d-5df8ad7dcf0c/ticket.toml`

<!-- ticket-index:entry id=913fdd33-77b3-4e40-914a-db6873bf004d slug=new/unspecified digest=6110681b4b9e -->
#### [913fdd33] [interview-api] Interview sessions, survey orchestration, and answer synthesis
- priority: `high`
- summary: Plan an interview domain store that persists editable interview sessions, supports single-user and multi-user survey flows, and produces actionable synthesized answer sheets.
- ref: `.ticket/tickets/913fdd33-77b3-4e40-914a-db6873bf004d/ticket.toml`

<!-- ticket-index:entry id=7639449a-22a9-4bea-9fcf-517810bc9ddf slug=new/unspecified digest=4ee6fef14244 -->
#### [7639449a] [interview-api] Session file model and collaborative survey state
- priority: `high`
- summary: Define and implement the persistent interview session model (files + indexes), editable prompt/response revisions, and multi-user survey participation state with conflict-safe updates.
- ref: `.ticket/tickets/7639449a-22a9-4bea-9fcf-517810bc9ddf/ticket.toml`

<!-- ticket-index:entry id=0dde154a-ee4d-4f0a-af83-e0a4864d3bfb slug=new/unspecified digest=279338b147a4 -->
#### [0dde154a] [peek-cli] --grep does not support regex alternation (\\|)
- summary: `--grep` with regex alternation (`\|`) reports no match even when individual alternatives do match:
- ref: `.ticket/tickets/0dde154a-ee4d-4f0a-af83-e0a4864d3bfb/ticket.toml`

<!-- ticket-index:entry id=2ea8ec57-fc71-46d2-8eba-a3de40c5bec2 slug=new/unspecified digest=90450143ecc6 -->
#### [2ea8ec57] [peek-cli] --grep shows only line numbers, not matching line content
- summary: `peek --grep <pattern>` outputs bare line numbers only, with no preview of the matched line text:
- ref: `.ticket/tickets/2ea8ec57-fc71-46d2-8eba-a3de40c5bec2/ticket.toml`

<!-- ticket-index:entry id=c37ea985-3647-421f-99eb-75860a0728e0 slug=new/unspecified digest=dafe7a9e997d -->
#### [c37ea985] [profiling] CLI/HTTP/MCP end-to-end test matrix (ticket + spec surfaces)
- priority: `medium`
- summary: Child of tracker `ef3f4a91`. Build a parity E2E test matrix that exercises the
- ref: `.ticket/tickets/c37ea985-3647-421f-99eb-75860a0728e0/ticket.toml`

<!-- ticket-index:entry id=2d59b99c-0205-4bf6-bad9-ecb69a52830a slug=new/unspecified digest=48429f9c0722 -->
#### [2d59b99c] [profiling] CLI/HTTP/MCP throughput/latency benchmarks
- priority: `low`
- summary: Child of tracker `ef3f4a91`. Add transport-level throughput/latency benchmarks
- ref: `.ticket/tickets/2d59b99c-0205-4bf6-bad9-ecb69a52830a/ticket.toml`

<!-- ticket-index:entry id=6a19ae5f-8695-47e1-8b21-1062e0546fda slug=new/unspecified digest=4a68c1d15d6f -->
#### [6a19ae5f] [profiling] Native Criterion benchmark matrix for context-* + ticket/spec APIs
- priority: `medium`
- summary: Child of tracker `ef3f4a91`. Add native Criterion benchmarks covering the
- ref: `.ticket/tickets/6a19ae5f-8695-47e1-8b21-1062e0546fda/ticket.toml`

<!-- ticket-index:entry id=ef3f4a91-734f-47aa-a9cf-fdfdb60ac2db slug=new/unspecified digest=a072066cea21 -->
#### [ef3f4a91] [profiling] Performance profiling & benchmark matrix (tracker)
- priority: `high`
- summary: Parent tracker for adding performance profiling and benchmarking across the
- ref: `.ticket/tickets/ef3f4a91-734f-47aa-a9cf-fdfdb60ac2db/ticket.toml`

<!-- ticket-index:entry id=d8d18128-656e-4a13-9983-946d6af33c27 slug=new/unspecified digest=1ae3d1c0ae58 -->
#### [d8d18128] [profiling] Testing + benchmark matrix index doc and run commands
- priority: `low`
- summary: Child of tracker `ef3f4a91`. Author the single index document that ties the
- ref: `.ticket/tickets/d8d18128-656e-4a13-9983-946d6af33c27/ticket.toml`

<!-- ticket-index:entry id=8a90a63c-0a07-439f-90e8-9124212b2dc8 slug=new/unspecified digest=f3207d00d1a8 -->
#### [8a90a63c] [program][multi-store] Store expansion and operational health program
- priority: `high`
- summary: Umbrella program for new store domains and operational quality loops extending the cross-store architecture.
- ref: `.ticket/tickets/8a90a63c-0a07-439f-90e8-9124212b2dc8/ticket.toml`

<!-- ticket-index:entry id=23e81ad8-b67c-49af-97b5-f90f8bb0ae2c slug=new/unspecified digest=f759a96796ac -->
#### [23e81ad8] [rule+skill] Rule-store sources for domain-store scaffolding instructions
- priority: `high`
- summary: Create canonical rule entries and generation targets for instruction files and slash-command prompt assets that encode architecture-decisions.md and tracker 671d4e47 guidelines.
- ref: `.ticket/tickets/23e81ad8-b67c-49af-97b5-f90f8bb0ae2c/ticket.toml`

<!-- ticket-index:entry id=66fae806-203d-4235-9151-4272eb0bb603 slug=new/unspecified digest=fc8dd38aa25e -->
#### [66fae806] [scaffold] Rule-generated store bootstrap instructions and slash command skill
- priority: `high`
- summary: Plan rule-generated instruction and prompt assets plus slash command skill for bootstrapping a minimally functional new domain store from one prompt, aligned with architecture-decisions and cross-sto...
- ref: `.ticket/tickets/66fae806-203d-4235-9151-4272eb0bb603/ticket.toml`

<!-- ticket-index:entry id=07d4b1b0-bc20-4ba7-98d4-ed09365f0437 slug=new/unspecified digest=7766e3b9b45a -->
#### [07d4b1b0] [skill] One-prompt domain-store scaffold slash command flow
- priority: `high`
- summary: Implement slash command flow that accepts one prompt and scaffolds a minimal domain store (crate layout, manifests, base APIs, tests, and registration hooks) using generated instructions.
- ref: `.ticket/tickets/07d4b1b0-bc20-4ba7-98d4-ed09365f0437/ticket.toml`

<!-- ticket-index:entry id=59d96577-09a8-44a7-b0ea-3d51b3a6fb05 slug=new/unspecified digest=3d394663c104 -->
#### [59d96577] [spec-cli][spec-mcp] Make spec workflows root-aware across nested .spec stores
- summary: Spec workflows are not root-aware enough across nested `.spec` stores.
- ref: `.ticket/tickets/59d96577-09a8-44a7-b0ea-3d51b3a6fb05/ticket.toml`

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

<!-- ticket-index:entry id=9acf1ef1-a7fb-40af-8a7a-4df89ac9a93f slug=new/unspecified digest=27dbe58d0099 -->
#### [9acf1ef1] [ticket-api] Allow reverse ticket state transitions through schema
- summary: Ticket state transitions are currently constrained by schema edges that only cover the forward workflow path in practice, which prevents moving a ticket back to an earlier valid state through the sam...
- ref: `.ticket/tickets/9acf1ef1-a7fb-40af-8a7a-4df89ac9a93f/ticket.toml`

<!-- ticket-index:entry id=86cde60c-49db-4820-a3a9-37c472ca1c2f slug=new/unspecified digest=d9a50d0ceb1e -->
#### [86cde60c] [ticket-api] Distinguish deferred and meta work from actionable tickets
- summary: Deferred, parent, and roadmap-style tickets look too much like actionable implementation tickets.
- ref: `.ticket/tickets/86cde60c-49db-4820-a3a9-37c472ca1c2f/ticket.toml`

<!-- ticket-index:entry id=acefc2ae-e257-4bc8-a4c7-0ec3137e374d slug=new/unspecified digest=75a34502c8ab -->
#### [acefc2ae] [ticket-api] Validation-aware dependency requirements and health model
- priority: `high`
- summary: Plan how ticket dependencies can declare required validation items whose satisfaction is resolved through test-api evidence rather than ad hoc ticket text.
- ref: `.ticket/tickets/acefc2ae-e257-4bc8-a4c7-0ec3137e374d/ticket.toml`

<!-- ticket-index:entry id=61cbc31f-c66d-46bf-807e-0d4236e04c9e slug=new/unspecified digest=9ea24eb33513 -->
#### [61cbc31f] [ticket-cli] Explain why tickets are absent from next
- summary: `ticket search` and `ticket next` do not explain their mismatch.
- ref: `.ticket/tickets/61cbc31f-c66d-46bf-807e-0d4236e04c9e/ticket.toml`

<!-- ticket-index:entry id=68e3c713-3c35-4d7e-af0c-b4a55a3253c0 slug=new/unspecified digest=6c5533643c79 -->
#### [68e3c713] [ticket-cli] Fix next --filter matching for prefix and substring queries
- summary: `ticket next --filter` behaved inconsistently enough to break targeted discovery.
- ref: `.ticket/tickets/68e3c713-3c35-4d7e-af0c-b4a55a3253c0/ticket.toml`

<!-- ticket-index:entry id=f3305925-7217-4ff3-8c4e-820ebc1e6de5 slug=new/unspecified digest=a034e16aea90 -->
#### [f3305925] [ticket-cli] Graph rendering and closure-aware dependency display
- priority: `high`
- summary: Plan a reusable graph-rendering primitive for ticket and related CLI surfaces, including ASCII and Mermaid outputs and closure-aware expansion over dependency subgraphs.
- ref: `.ticket/tickets/f3305925-7217-4ff3-8c4e-820ebc1e6de5/ticket.toml`

<!-- ticket-index:entry id=07836f41-7fa5-4e41-8411-1c7cf8aeee75 slug=new/unspecified digest=c4c479528238 -->
#### [07836f41] [ticket-cli] Make get/search/list workspace-aware across nested roots
- summary: `ticket get <id>` was not workspace-aware and failed with a raw path error when the ticket lived under a different ticket root.
- ref: `.ticket/tickets/07836f41-7fa5-4e41-8411-1c7cf8aeee75/ticket.toml`

<!-- ticket-index:entry id=d241a482-6fc7-468e-b0a3-748cb72d07eb slug=new/unspecified digest=58bef92aa7fd -->
#### [d241a482] [ticket-cli][spec-cli] Normalize sibling CLI grammar and JSON envelopes
- summary: The sibling ticket/spec CLIs make automation harder than necessary because their command grammar and JSON envelopes drift in incompatible ways.
- ref: `.ticket/tickets/d241a482-6fc7-468e-b0a3-748cb72d07eb/ticket.toml`

<!-- ticket-index:entry id=61cb6557-e559-4eae-8e59-ea0d520a3bee slug=new/unspecified digest=29d8b3416f32 -->
#### [61cb6557] [ticket-cli][ticket-mcp] Add consolidated ticket detail/context read surface
- summary: Reviewing a ticket currently requires chaining multiple read surfaces.
- ref: `.ticket/tickets/61cb6557-e559-4eae-8e59-ea0d520a3bee/ticket.toml`

<!-- ticket-index:entry id=8bb97b73-9dbc-43ee-9939-46b3ddf2612f slug=new/unspecified digest=fa00162c740b -->
#### [8bb97b73] [ticket-cli][ticket-mcp] Explain invalid state transitions with allowed next states
- summary: Invalid ticket state transitions are enforced, but they are not explained well enough at the moment they fail.
- ref: `.ticket/tickets/8bb97b73-9dbc-43ee-9939-46b3ddf2612f/ticket.toml`

<!-- ticket-index:entry id=def7fa82-6f4a-4354-b52d-ae7ea9623648 slug=new/unspecified digest=a58b4f9a2a70 -->
#### [def7fa82] [ticket-cli][ticket-mcp] Make stale board entries directly check-outable
- summary: `ticket board show` exposed a stale entry with an `entry_id`, `ticket_id`, `agent_id`, and owned files, but `ticket board check-out <ticket-id>` still failed with `no active board entry found` until ...
- ref: `.ticket/tickets/def7fa82-6f4a-4354-b52d-ae7ea9623648/ticket.toml`

<!-- ticket-index:entry id=43fc22b3-9b36-4a54-b520-f51000330a46 slug=new/unspecified digest=d6169b637cd9 -->
#### [43fc22b3] [ticket-graph] Tracker: validation-aware graph tooling and audit enforcement
- priority: `high`
- summary: Coordinate planning and delivery for ticket graph rendering, validation-aware dependency requirements, and audit/board enforcement built on the existing ticket graph and test-api evidence store.
- ref: `.ticket/tickets/43fc22b3-9b36-4a54-b520-f51000330a46/ticket.toml`

<!-- ticket-index:entry id=5ad77aba-c7f7-4058-854e-dd0412746c7c slug=new/unspecified digest=bebff821ba10 -->
#### [5ad77aba] [ticket-mcp][spec-mcp][rule-api] Add self-describing capability catalog and help surfaces
- summary: The ticket/spec/rule tool surfaces are not self-describing enough for operators or agents.
- ref: `.ticket/tickets/5ad77aba-c7f7-4058-854e-dd0412746c7c/ticket.toml`

<!-- ticket-index:entry id=14df656e-cef2-470e-9530-ef760b6c462c slug=new/unspecified digest=7252700c3e99 -->
#### [14df656e] [ticket-viewer][ticket-vscode] Surface the next-work workflow in frontends
- summary: There is no end-user frontend surface for the "best next ticket to implement" workflow.
- ref: `.ticket/tickets/14df656e-cef2-470e-9530-ef760b6c462c/ticket.toml`

<!-- ticket-index:entry id=814f22dc-0f75-4c11-b7da-20b3c5928cea slug=new/unspecified digest=f876721b89a6 -->
#### [814f22dc] [ticket-vscode] Fix VS Code CLI discovery in VSIX installer
- ref: `.ticket/tickets/814f22dc-0f75-4c11-b7da-20b3c5928cea/ticket.toml`

<!-- ticket-index:entry id=51671748-8933-4955-9bf4-7bdea961df40 slug=new/unspecified digest=a749e08a2489 -->
#### [51671748] [ticket-workflow] Harden best-next-ticket discovery across spec, CLI, MCP, and frontends
- summary: We now have several issue slices for board / next discovery, but no ticket owns the full contract for finding the best next ticket to implement.
- ref: `.ticket/tickets/51671748-8933-4955-9bf4-7bdea961df40/ticket.toml`

<!-- ticket-index:entry id=70222986-3325-4d45-892e-31e7f4d09aa6 slug=new/unspecified digest=bbcb0b7c492b -->
#### [70222986] [validation] E2E regression harness for domain-store scaffold prompts
- priority: `high`
- summary: Implement automated end-to-end regression testing for scaffold prompts, validating generated store correctness, compile/test health, and architecture-conformance checks across representative prompt s...
- ref: `.ticket/tickets/70222986-3325-4d45-892e-31e7f4d09aa6/ticket.toml`

<!-- ticket-index:entry id=2ff2c8e8-eaec-4bd9-9312-ae13cd4b243a slug=new/unspecified digest=9f6866e5c14e -->
#### [2ff2c8e8] [validation] Prompt replay matrix for scaffold skill domain coverage
- priority: `high`
- summary: Build representative prompt matrix (simple, medium, complex, edge-case) and replay harness to validate scaffold skill behavior, compile health, and architecture conformance across domain types.
- ref: `.ticket/tickets/2ff2c8e8-eaec-4bd9-9312-ae13cd4b243a/ticket.toml`

<!-- ticket-index:entry id=dedac9f5-0d4d-4ad0-8a7e-4acd361c273e slug=new/unspecified digest=b362b19531e9 -->
#### [dedac9f5] [validation] Rule-target generation drift checks for scaffold guidance assets
- priority: `high`
- summary: Automate checks that rule-source entries and generated instruction/prompt outputs for domain-store scaffolding remain in sync, with stable snapshots and actionable diff reporting.
- ref: `.ticket/tickets/dedac9f5-0d4d-4ad0-8a7e-4acd361c273e/ticket.toml`

<!-- ticket-index:entry id=936d38d6-a238-4cb9-b00a-1b2a4b65dc04 slug=new/unspecified digest=b70f4bba3bf2 -->
#### [936d38d6] [viewer-api] Port doc-viewer features to shared Dioxus viewer-api
- summary: Track the remaining Dioxus adoption work needed after the shared doc-viewer-inspired primitives landed in `viewer-api-dioxus`, so current viewers reuse the shared shells and stores instead of carryin...
- ref: `memory-viewers/viewer-api/.ticket/tickets/936d38d6-a238-4cb9-b00a-1b2a4b65dc04/ticket.toml`

<!-- ticket-index:entry id=8f349d96-a307-400b-a90e-3aceb2250166 slug=new/unspecified digest=c6c79abab73b -->
#### [8f349d96] viewer-api-dioxus: ship WASM tracing logs to a server file sink
- priority: `low`
- summary: Build on the structured-tracing parent ticket: add a tracing `Layer` that batches log records and POSTs them to a server endpoint (e.g. `POST /api/client-log`) which appends them to a per-session JSO...
- ref: `memory-viewers/viewer-api/.ticket/tickets/8f349d96-a307-400b-a90e-3aceb2250166/ticket.toml`

<!-- ticket-index:entry id=b480632a-8605-4b5b-a4e8-f2988b2565a0 slug=new/unspecified digest=d1d023c24644 -->
#### [b480632a] viewer-api-dioxus: structured tracing for WASM frontend
- priority: `medium`
- summary: Replace ad-hoc `web_sys::console::log_1!()` calls in the Dioxus WASM frontend (viewer-api, spec-viewer, ticket-viewer) with structured tracing.
- ref: `memory-viewers/viewer-api/.ticket/tickets/b480632a-8605-4b5b-a4e8-f2988b2565a0/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=81a6a595-7426-478d-9487-17142dcfa8a0 slug=new/viewer-api digest=9b56efaa5dfc -->
#### [81a6a595] Plan: Context API phase 4.1 — viewer-api + log-viewer as thin frontend layers
- summary: tags: `#context-api` `#phase4.1` `#refactor` `#viewer-api` `#log-viewer` `#context-http` `#frontend`
- ref: `memory-viewers/viewer-api/.ticket/tickets/81a6a595-7426-478d-9487-17142dcfa8a0/ticket.toml`

<!-- ticket-index:entry id=301dc3ce-38b6-4b28-bd84-266e33b46c90 slug=new/viewer-api digest=bdfcf7fec586 -->
#### [301dc3ce] Plan: DOM 3D integration for graph viewer
- summary: tags: `#plan` `#rendering` `#3d` `#webgpu` `#dom`
- ref: `memory-viewers/viewer-api/.ticket/tickets/301dc3ce-38b6-4b28-bd84-266e33b46c90/ticket.toml`

<!-- ticket-index:entry id=ee7aa0cd-04ae-423e-83be-6edf58eeaf41 slug=new/viewer-api digest=fca0e5270461 -->
#### [ee7aa0cd] Plan: nesting view mode for graph viewer
- summary: Date:** 2026-03-07
- ref: `memory-viewers/viewer-api/.ticket/tickets/ee7aa0cd-04ae-423e-83be-6edf58eeaf41/ticket.toml`

<!-- ticket-index:entry id=608bb106-f22d-4bb4-bbde-d87ec33fd6e6 slug=new/viewer-api digest=6e5d5c6bf3c8 -->
#### [608bb106] Plan: search visualisation in graph viewer
- summary: tags: `#plan` `#visualization` `#search` `#logging` `#frontend`
- ref: `memory-viewers/viewer-api/.ticket/tickets/608bb106-f22d-4bb4-bbde-d87ec33fd6e6/ticket.toml`

<!-- ticket-index:entry id=20c4d807-042f-4c4b-a683-3d84658094c3 slug=new/viewer-api digest=d7fc22156370 -->
#### [20c4d807] Plan: viewer refactoring and mobile support — HypergraphView extraction
- summary: Date:** 2026-03-04
- ref: `memory-viewers/viewer-api/.ticket/tickets/20c4d807-042f-4c4b-a683-3d84658094c3/ticket.toml`

<!-- ticket-index:entry id=68912b00-e189-4dc1-8124-ca41d9aab953 slug=new/viewer-api digest=d37bc395eb44 -->
#### [68912b00] Plan: viewer tools features (2026-03-01 batch)
- summary: Date:** 2026-03-01
- ref: `memory-viewers/viewer-api/.ticket/tickets/68912b00-e189-4dc1-8124-ca41d9aab953/ticket.toml`

<!-- ticket-index:entry id=97c757b1-3c58-4b54-ab4b-35b7d0ea9ece slug=new/viewer-api digest=ac8ce70e994c -->
#### [97c757b1] Plan: viewer-api refactoring — extract shared server infrastructure
- summary: tags: `#plan` `#refactoring` `#tools` `#viewer-api`
- ref: `memory-viewers/viewer-api/.ticket/tickets/97c757b1-3c58-4b54-ab4b-35b7d0ea9ece/ticket.toml`

<!-- ticket-index:entry id=d1e4ab96-52e1-4b80-ad7c-bfff459d3fac slug=new/viewer-api digest=14bfcc5d28af -->
#### [d1e4ab96] [viewer-api] Converge shared Dioxus viewer shells across frontends
- priority: `high`
- summary: Converge the duplicated Dioxus viewer shell patterns across the current frontend implementations so each viewer stays thin and generic behavior lives in viewer-api.
- ref: `memory-viewers/viewer-api/.ticket/tickets/d1e4ab96-52e1-4b80-ad7c-bfff459d3fac/ticket.toml`

<!-- ticket-index:entry id=08c86dbd-72b8-446b-a930-30ef3352d604 slug=new/viewer-api digest=61799d8e7845 -->
#### [08c86dbd] [viewer-api] Create comprehensive E2E test suite for graph improvements
- priority: `high`
- summary: Create comprehensive Playwright E2E test coverage for the four graph improvements across all memory-viewers:
- ref: `.ticket/tickets/08c86dbd-72b8-446b-a930-30ef3352d604/ticket.toml`

<!-- ticket-index:entry id=254ac30d-26c0-4bfe-8a66-b10ab9e4843a slug=new/viewer-api digest=61421a2a8635 -->
#### [254ac30d] [viewer-api] Generalize graph improvements to spec-viewer and log-viewer
- priority: `high`
- summary: Generalize the four graph improvements implemented in ticket-viewer to spec-viewer and log-viewer:
- ref: `.ticket/tickets/254ac30d-26c0-4bfe-8a66-b10ab9e4843a/ticket.toml`

<!-- ticket-index:entry id=e8d9bfcd-d729-43a6-8efa-4554af609d0c slug=new/viewer-api digest=5f26c65d6693 -->
#### [e8d9bfcd] [viewer-api] Update Graph3D component documentation and examples
- priority: `medium`
- summary: Update Graph3D component documentation and examples to reflect the four graph improvements and provide clear integration guidance:
- ref: `.ticket/tickets/e8d9bfcd-d729-43a6-8efa-4554af609d0c/ticket.toml`

<!-- ticket-index:entry id=4e0dc8fb-18fa-4be1-a43d-37008d0453e3 slug=new/viewer-api digest=caf3a4721053 -->
#### [4e0dc8fb] [viewer-api][ticket-viewer][design] viewer-wide keyboard support model
- priority: `medium`
- summary: Design a viewer-wide keyboard interaction model for the Dioxus viewer stack without bundling it into the immediate explorer fixes.
- ref: `memory-viewers/viewer-api/.ticket/tickets/4e0dc8fb-18fa-4be1-a43d-37008d0453e3/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=01932eb7-54e5-441b-87bc-db3013a0882c slug=new/viewer-api-dioxus digest=e77d32da134e -->
#### [01932eb7] Feature: tiling + tabbed panel system replacing flat Sidebar/Panel primitives
- priority: `medium`
- summary: The current `viewer-api-dioxus::components::layout::{Sidebar, Panel}` primitives are flat: each viewer hardcodes a single left `Sidebar` + one optional right `Panel`. There is no support for:
- ref: `memory-viewers/viewer-api/.ticket/tickets/01932eb7-54e5-441b-87bc-db3013a0882c/ticket.toml`

<!-- ticket-index:entry id=92964ada-4ab5-4fe1-ab29-5bfd55583ad2 slug=new/viewer-api-dioxus digest=0d35689bd70a -->
#### [92964ada] Refactor: extract viewer-theme and viewer-widgets crates from viewer-api-dioxus
- priority: `medium`
- summary: `viewer-api-dioxus` currently bundles three logically distinct concerns:
- ref: `memory-viewers/viewer-api/.ticket/tickets/92964ada-4ab5-4fe1-ab29-5bfd55583ad2/ticket.toml`


## State: on-hold

### Component: unspecified

<!-- ticket-index:entry id=47914c71-bb3c-4b95-9120-6121dd42ae2b slug=on-hold/unspecified digest=230209341bb1 -->
#### [47914c71] Multiplayer Backend: SpacetimeDB Server Module — Tables, Reducers, Auth & Tick Loop
- priority: `high`
- summary: The context-editor is currently a single-player application. To enable multiplayer open-world gameplay, we need an authoritative server that stores world state, validates player actions, manages auth...
- ref: `context-stack/tools/context-editor/.ticket/tickets/47914c71-bb3c-4b95-9120-6121dd42ae2b/ticket.toml`


## State: ready

### Component: cli

<!-- ticket-index:entry id=7bf50e75-018e-4b70-b93f-2bac099f9677 slug=ready/cli digest=4d493e9b6ba0 -->
#### [7bf50e75] Plan: Sandboxed integration tests for context-tasks
- summary: tags: `#plan` `#testing` `#integration` `#context-tasks` `#sandbox`
- ref: `.ticket/tickets/7bf50e75-018e-4b70-b93f-2bac099f9677/ticket.toml`

<!-- ticket-index:entry id=b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72 slug=ready/cli digest=26ba71799ee7 -->
#### [b1f3e2a4] [bootstrap][T2] enforce assignment start context branch and cwd checks
- summary: When a worker is dispatched to implement a ticket, the assignment packet includes an explicit branch name and working directory. Before the worker can claim the ticket, the executor must verify the w...
- ref: `.ticket/tickets/b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72/ticket.toml`


### Component: context-api

<!-- ticket-index:entry id=0727b7dd-b90b-4edb-8c16-2d6220506585 slug=ready/context-api digest=45172ea26745 -->
#### [0727b7dd] Plan: Context API — master multi-phase architecture plan
- summary: tags: `#context-api` `#architecture` `#multi-phase` `#api-design` `#plan`
- ref: `.ticket/tickets/0727b7dd-b90b-4edb-8c16-2d6220506585/ticket.toml`


### Component: context-engine

<!-- ticket-index:entry id=8d83f9f6-b36e-42bd-ac42-3a6d073873a7 slug=ready/context-engine digest=8e7bf2cb6755 -->
#### [8d83f9f6] [sandbox-v1][impl] Firecracker control plane and repo-local microVM foundation
- priority: `high`
- summary: Tokio multi-thread orchestration core.
- ref: `.ticket/tickets/8d83f9f6-b36e-42bd-ac42-3a6d073873a7/ticket.toml`


### Component: context-read

<!-- ticket-index:entry id=f95969ba-c797-42d2-b6bc-9265a5fb4cf0 slug=ready/context-read digest=84d61424764a -->
#### [f95969ba] Plan: context-read UX improvement — parent plan (multi-phase)
- summary: tags: `#plan` `#context-read` `#context-api` `#context-cli` `#ux` `#algorithm` `#read` `#insert` `#search` `#multi-phase`
- ref: `.ticket/tickets/f95969ba-c797-42d2-b6bc-9265a5fb4cf0/ticket.toml`

<!-- ticket-index:entry id=6e61bef1-6037-42c8-abc1-d79a3f9367f7 slug=ready/context-read digest=e16cca381a8f -->
#### [6e61bef1] [context-insert] Unify overlap bundling under one structural formula
- priority: `high`
- summary: `context-insert::bundle_overlap` still branches on `self_overlap` and `overlap_is_shared_then_t1`, and it falls back to raw `insert_patterns`. The formula is branchy and hard to reason about.
- ref: `.ticket/tickets/6e61bef1-6037-42c8-abc1-d79a3f9367f7/ticket.toml`

<!-- ticket-index:entry id=529feeaa-822c-443b-a6a2-f0ae67edc225 slug=ready/context-read digest=f585222e646d -->
#### [529feeaa] [context-read][tests] Layer read tests after lower-crate primitives
- priority: `high`
- summary: `context-read` tests mostly assert whole decomposition families after long worked traces. Lower crates use smaller fixture-based tests to pin one primitive at a time. The current read suite turns eve...
- ref: `.ticket/tickets/529feeaa-822c-443b-a6a2-f0ae67edc225/ticket.toml`


### Component: context-stack

<!-- ticket-index:entry id=aaa810f0-cc14-4226-b7d0-d81a38f856e7 slug=ready/context-stack digest=b544cb089762 -->
#### [aaa810f0] Decide post-import ownership cleanup for context-stack tools
- priority: `medium`
- summary: After the tool-history import, the original tool source trees still exist in `context-engine`. Until ownership cleanup is decided and executed, it is ambiguous which repository is the source of truth...
- ref: `.ticket/tickets/aaa810f0-cc14-4226-b7d0-d81a38f856e7/ticket.toml`


### Component: documentation-tooling

<!-- ticket-index:entry id=5d320d7e-f974-4d52-9e25-8265bf7a42cf slug=ready/documentation-tooling digest=1fad466d66a9 -->
#### [5d320d7e] Design reproducible Docker validation for install and deinstall docs
- priority: `high`
- summary: User-facing installation documentation is not validated continuously from a clean environment. The current repo has install instructions for the CLI tools in `memory-viewers/memory-api/README.md`, bu...
- ref: `.ticket/tickets/5d320d7e-f974-4d52-9e25-8265bf7a42cf/ticket.toml`

<!-- ticket-index:entry id=e0c136dd-8bdf-40f6-a39c-29f9e88167d6 slug=ready/documentation-tooling digest=022ed87d25b3 -->
#### [e0c136dd] Gate install and deinstall documentation continuously in CI
- priority: `high`
- summary: A local Docker harness is not sufficient on its own. The user-facing installation documentation needs continuous validation in CI so documentation drift or broken installation steps are caught before...
- ref: `.ticket/tickets/e0c136dd-8bdf-40f6-a39c-29f9e88167d6/ticket.toml`


### Component: history

<!-- ticket-index:entry id=f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16 slug=ready/history digest=d51d9f1efaeb -->
#### [f5d7e9a2] [bootstrap][T6] verify merge and completion linkage with assignment chain
- summary: After validation passes (T4), the ticket advances through release gates toward merge. The merge record must be fully traceable: it must include the worker assignment_id, the validator assignment_id, ...
- ref: `memory-viewers/memory-api/.ticket/tickets/f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16/ticket.toml`


### Component: lease

<!-- ticket-index:entry id=a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61 slug=ready/lease digest=3f9bf47e5711 -->
#### [a8d6c1d2] [bootstrap][T1] startup and auth bootstrap for host executor
- summary: The host executor is a Rust service process (`ticket host-executor`) that workers authenticate against to claim tickets, run inference, and report progress. Per the Phase 1.5 design, the executor can...
- ref: `memory-viewers/memory-api/.ticket/tickets/a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61/ticket.toml`

<!-- ticket-index:entry id=c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83 slug=ready/lease digest=5172e265bdc5 -->
#### [c2a4b6d8] [bootstrap][T3] validate ticket lifecycle happy path under executor
- summary: Once a worker is authenticated (T1) and context-verified (T2), it proceeds through the core ticket mutation lifecycle: claim → implement → attach evidence → unclaim. Every event in this lifecycle mus...
- ref: `memory-viewers/memory-api/.ticket/tickets/c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83/ticket.toml`

<!-- ticket-index:entry id=d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94 slug=ready/lease digest=66b2a4b588df -->
#### [d3b5c7e9] [bootstrap][T4] implement validator handoff with separation-of-duties
- summary: After a worker completes implementation (T3), the ticket moves to `validating` state. A second agent — the **validator** — is dispatched by the coordinator with a different identity to independently ...
- ref: `memory-viewers/memory-api/.ticket/tickets/d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94/ticket.toml`


### Component: memory-api

<!-- ticket-index:entry id=b03be2d5-5293-4dc7-ad11-cca2dbf32c8b slug=ready/memory-api digest=73868ab855f9 -->
#### [b03be2d5] [spec][P5] Cross-entity edges — spec depends_on ticket, ticket implements spec
- priority: `medium`
- summary: Extend memory-api's edge system to support edges between entities of different types (spec ↔ ticket). Currently edges are within a single entity store; this enables cross-store relationships.
- ref: `memory-viewers/memory-api/.ticket/tickets/b03be2d5-5293-4dc7-ad11-cca2dbf32c8b/ticket.toml`


### Component: rule-api

<!-- ticket-index:entry id=d0ccdb06-db44-464f-846e-9d58c1320fd0 slug=ready/rule-api digest=464976e6472f -->
#### [d0ccdb06] Complete memory-api rule-api specs and test links
- priority: `high`
- summary: Nested rule work and repo-local README generation need a committed spec set in `memory-viewers/memory-api/.spec` with maintained code references and validation hooks. Initial planning specs now exist...
- ref: `memory-viewers/memory-api/.ticket/tickets/d0ccdb06-db44-464f-846e-9d58c1320fd0/ticket.toml`

<!-- ticket-index:entry id=7cffac6b-7dca-4134-8c0f-7dbedcd0cbbd slug=ready/rule-api digest=d6ded4413d74 -->
#### [7cffac6b] Generate memory-api README from repo-local rules
- priority: `high`
- summary: `memory-api` does not yet have a repo-local `.rule` workspace or a local `rule-targets.yaml`, so its `README.md` remains a manually maintained file instead of a generated target owned by the repo tha...
- ref: `memory-viewers/memory-api/.ticket/tickets/7cffac6b-7dca-4134-8c0f-7dbedcd0cbbd/ticket.toml`


### Component: spec-api

<!-- ticket-index:entry id=00798e96-3d82-436e-963c-af347e76ede0 slug=ready/spec-api digest=190b8105d518 -->
#### [00798e96] [spec][P3] Spec creation — planned feature specs with acceptance criteria templates
- priority: `medium`
- summary: Create specification files for features that are planned but not yet implemented. These specs serve as the design document and acceptance criteria definition.
- ref: `memory-viewers/memory-api/.ticket/tickets/00798e96-3d82-436e-963c-af347e76ede0/ticket.toml`

<!-- ticket-index:entry id=ffc578f7-8a18-4536-9a8c-023d42b98d3e slug=ready/spec-api digest=3937860b82b1 -->
#### [ffc578f7] [spec][P3] Spec-to-code sync — detect and update references after file moves
- priority: `medium`
- summary: Detect when implementation files are moved/renamed and automatically update spec code references.
- ref: `memory-viewers/memory-api/.ticket/tickets/ffc578f7-8a18-4536-9a8c-023d42b98d3e/ticket.toml`

<!-- ticket-index:entry id=80e25216-7ba9-4fd9-bc80-3311f1d2a604 slug=ready/spec-api digest=8448bb26a061 -->
#### [80e25216] [spec][P3] Spec-to-code sync — update specs after implementation changes
- priority: `high`
- summary: Detect when implementation code changes and update spec code references and feature status accordingly.
- ref: `memory-viewers/memory-api/.ticket/tickets/80e25216-7ba9-4fd9-bc80-3311f1d2a604/ticket.toml`

<!-- ticket-index:entry id=c4c9e9d4-8831-4135-98a7-0b64031ffe52 slug=ready/spec-api digest=78903c83f3b7 -->
#### [c4c9e9d4] [spec][P4] Feature tracking — record feature completeness and bug status per spec
- priority: `medium`
- summary: Track per-spec feature completeness: which features are implemented, planned, blocked, or have known bugs.
- ref: `memory-viewers/memory-api/.ticket/tickets/c4c9e9d4-8831-4135-98a7-0b64031ffe52/ticket.toml`

<!-- ticket-index:entry id=6c00ef55-1531-4494-9bf2-00184740a3b0 slug=ready/spec-api digest=6401f65626f3 -->
#### [6c00ef55] [spec][P4] Skill generation — master index and cross-references
- priority: `medium`
- summary: Generate a master `docs/skills/INDEX.md` that serves as the entry point for all generated skill files, with coverage statistics and cross-references.
- ref: `memory-viewers/memory-api/.ticket/tickets/6c00ef55-1531-4494-9bf2-00184740a3b0/ticket.toml`

<!-- ticket-index:entry id=eddf5d2e-e1b6-4ec9-b88f-d50bd192b194 slug=ready/spec-api digest=f461052b6fd0 -->
#### [eddf5d2e] [spec][P4] Skill generation — per-crate and per-domain SKILL.md files from spec data
- priority: `high`
- summary: Build a skill file generation engine that reads spec data from the SpecStore and produces structured SKILL.md files for AI coding agents.
- ref: `memory-viewers/memory-api/.ticket/tickets/eddf5d2e-e1b6-4ec9-b88f-d50bd192b194/ticket.toml`

<!-- ticket-index:entry id=ad5fb72b-548c-4215-88a6-eacde7a42d4d slug=ready/spec-api digest=e3ccbe2fdff5 -->
#### [ad5fb72b] [spec][P4] Spec health check — completeness, staleness, broken references, coverage
- priority: `medium`
- summary: Validate spec store integrity including completeness, staleness, broken references, and coverage metrics.
- ref: `memory-viewers/memory-api/.ticket/tickets/ad5fb72b-548c-4215-88a6-eacde7a42d4d/ticket.toml`

<!-- ticket-index:entry id=45671e0e-24d6-4f51-b216-07e80f2ff302 slug=ready/spec-api digest=336311df73c1 -->
#### [45671e0e] [spec][P4] Test generation — Rust test stubs and test matrix from spec acceptance criteria
- priority: `medium`
- summary: Generate Rust test stubs for uncovered spec features and a test matrix checklist linking existing tests to spec acceptance criteria.
- ref: `memory-viewers/memory-api/.ticket/tickets/45671e0e-24d6-4f51-b216-07e80f2ff302/ticket.toml`

<!-- ticket-index:entry id=f00291a3-bd61-469e-a737-c44cb3911e3b slug=ready/spec-api digest=d2e7b1235309 -->
#### [f00291a3] [spec][P5] Ticket integration — link specs to tickets, track refinement/validation/bugfix work
- priority: `medium`
- summary: Link specs to tickets bidirectionally. When a ticket implements a spec feature, or a bug is found against a spec, the relationship is tracked.
- ref: `memory-viewers/memory-api/.ticket/tickets/f00291a3-bd61-469e-a737-c44cb3911e3b/ticket.toml`

<!-- ticket-index:entry id=7802faa3-5d79-4ec9-9f26-143bca62149c slug=ready/spec-api digest=99c5d23daec9 -->
#### [7802faa3] [spec][P6] Hierarchical DAG — parent-child spec relationships, no duplication
- priority: `medium`
- summary: Implement parent-child spec relationships as a DAG (no duplication of specification content). Each spec declares its parent; the system builds a tree with cross-references via edges.
- ref: `memory-viewers/memory-api/.ticket/tickets/7802faa3-5d79-4ec9-9f26-143bca62149c/ticket.toml`

<!-- ticket-index:entry id=d72d5114-2521-4d02-9ca1-7f0bee8d470d slug=ready/spec-api digest=1326ccfda4c3 -->
#### [d72d5114] [spec][P6] Spec search — full-text search with field predicates
- priority: `medium`
- summary: Full-text search across all specs using Tantivy, with field predicates matching the ticket search pattern.
- ref: `memory-viewers/memory-api/.ticket/tickets/d72d5114-2521-4d02-9ca1-7f0bee8d470d/ticket.toml`

<!-- ticket-index:entry id=a7b2a89c-6562-468c-a129-ad4883e5cf6e slug=ready/spec-api digest=9df4849eef09 -->
#### [a7b2a89c] [spec][P6] Table of contents — auto-generated TOC index of all specs
- priority: `medium`
- summary: Auto-generate a table of contents index showing all specs organized by component and hierarchy.
- ref: `memory-viewers/memory-api/.ticket/tickets/a7b2a89c-6562-468c-a129-ad4883e5cf6e/ticket.toml`

<!-- ticket-index:entry id=9242a906-cba9-43a4-b45e-942465379a7b slug=ready/spec-api digest=9146478d7e2c -->
#### [9242a906] [spec][P8] Bootstrap: write spec files for ticket-api interfaces
- priority: `high`
- summary: Write comprehensive spec files documenting the ticket-api crate's full API surface, storage layer, schema system, and edge system.
- ref: `memory-viewers/memory-api/.ticket/tickets/9242a906-cba9-43a4-b45e-942465379a7b/ticket.toml`

<!-- ticket-index:entry id=c617cee6-3182-47db-a7cf-15cccbc02b6d slug=ready/spec-api digest=36942d1dd4b8 -->
#### [c617cee6] [spec][P8] Generate initial skill files for all ticket system tools
- priority: `high`
- summary: Use the skill generation engine to produce the first set of SKILL.md files covering all ticket system tools.
- ref: `memory-viewers/memory-api/.ticket/tickets/c617cee6-3182-47db-a7cf-15cccbc02b6d/ticket.toml`


### Component: spec-cli

<!-- ticket-index:entry id=f2c1ebc2-aaee-4a93-895b-56284b549840 slug=ready/spec-cli digest=cda28fb82b50 -->
#### [f2c1ebc2] [spec][P8] Bootstrap: write spec files for ticket-cli interface
- priority: `medium`
- summary: Write specs for the ticket-cli crate documenting the CLI command surface, argument parsing, output formatting, and batch execution. Covers all commands: create, get, update, delete, list, search, lin...
- ref: `memory-viewers/memory-api/.ticket/tickets/f2c1ebc2-aaee-4a93-895b-56284b549840/ticket.toml`


### Component: spec-editor

<!-- ticket-index:entry id=618f6240-3f08-466f-857e-1c8c52d032d8 slug=ready/spec-editor digest=467afb01f6c7 -->
#### [618f6240] [spec-editor] Interactive spec authoring — Dioxus SPA with body/section/coderef editing
- priority: `high`
- summary: A single-process, GPU-accelerated web application for **authoring and editing** specs.
- ref: `.ticket/tickets/618f6240-3f08-466f-857e-1c8c52d032d8/ticket.toml`


### Component: spec-http

<!-- ticket-index:entry id=1b19e979-f2a0-4803-bc97-15ffd8f7ab72 slug=ready/spec-http digest=b25fbbc96906 -->
#### [1b19e979] [spec][P8] Bootstrap: write spec files for ticket-http interface
- priority: `medium`
- summary: Write specs for the ticket-http crate documenting all HTTP endpoints, request/response formats, middleware, SSE streaming, auth, and error handling.
- ref: `memory-viewers/memory-api/.ticket/tickets/1b19e979-f2a0-4803-bc97-15ffd8f7ab72/ticket.toml`


### Component: spec-mcp

<!-- ticket-index:entry id=10a26c64-402b-45e2-8333-2c471d0c0170 slug=ready/spec-mcp digest=377b0e3f3b4e -->
#### [10a26c64] [spec][P8] Bootstrap: write spec files for ticket-mcp interface
- priority: `medium`
- summary: Write specs for the ticket-mcp crate documenting all MCP tools, their input schemas, output formats, and error handling.
- ref: `memory-viewers/memory-api/.ticket/tickets/10a26c64-402b-45e2-8333-2c471d0c0170/ticket.toml`


### Component: spec-vscode

<!-- ticket-index:entry id=7f0a4dac-37b0-44c8-ba72-4ea0aaabb374 slug=ready/spec-vscode digest=c16e8cd7517c -->
#### [7f0a4dac] [spec][P7] spec-vscode — VS Code extension for browsing specs with rich HTML viewer
- priority: `low`
- summary: VS Code extension for browsing specification files with rich HTML rendering, navigation links, and code reference jump-to-source.
- ref: `memory-viewers/memory-api/.ticket/tickets/7f0a4dac-37b0-44c8-ba72-4ea0aaabb374/ticket.toml`

<!-- ticket-index:entry id=321f4ec7-03df-4e14-9734-a6af76ace55f slug=ready/spec-vscode digest=8d2889c07c13 -->
#### [321f4ec7] [spec][P8] Bootstrap: write spec files for ticket-vscode interface
- priority: `low`
- summary: Write specs for the ticket-vscode extension documenting the tree view provider, webview panel, API client, and VS Code extension lifecycle.
- ref: `memory-viewers/memory-api/.ticket/tickets/321f4ec7-03df-4e14-9734-a6af76ace55f/ticket.toml`


### Component: ticket-http

<!-- ticket-index:entry id=181ed793-481d-4d46-b059-0eda891365d7 slug=ready/ticket-http digest=a943902b0f6a -->
#### [181ed793] [ticket-http] Add /api/next endpoint for best-next ranking
- priority: `high`
- summary: There is no dedicated `GET /api/next` route in the current ticket HTTP router. HTTP consumers that want ranked best-next results have to reconstruct them manually by combining `GET /api/tickets` with...
- ref: `memory-viewers/memory-api/.ticket/tickets/181ed793-481d-4d46-b059-0eda891365d7/ticket.toml`

<!-- ticket-index:entry id=5012f293-e871-4e4a-af40-c27b3bd967fb slug=ready/ticket-http digest=c74aede6aa6a -->
#### [5012f293] [ticket-http][ticket-api][ticket-viewer] Track: child-workspace ticket reference rollout
- priority: `high`
- summary: The child-workspace ticket-reference rollout is now split into three well-scoped tickets, but there is no parent tracker that captures the full implementation sequence, the shared goal, or the cross-...
- ref: `memory-viewers/memory-api/.ticket/tickets/5012f293-e871-4e4a-af40-c27b3bd967fb/ticket.toml`


### Component: ticket-viewer

<!-- ticket-index:entry id=2b3a6e2e-4911-4b33-a3a9-9ace11f26637 slug=ready/ticket-viewer digest=9346967768ed -->
#### [2b3a6e2e] Bug: TicketDetail right panel hardcoded colors — port to theme variables
- priority: `high`
- summary: `tools/viewer/ticket-viewer/frontend/dioxus/src/components/ticket_detail.rs` builds the right-side ticket detail panel from inline hardcoded hex colors. When the theme is set to PAPER (light) the pan...
- ref: `memory-viewers/.ticket/tickets/2b3a6e2e-4911-4b33-a3a9-9ace11f26637/ticket.toml`


### Component: ticket-vscode

<!-- ticket-index:entry id=6d07d610-75c1-448a-afd5-6ae15098ca21 slug=ready/ticket-vscode digest=faa27559f457 -->
#### [6d07d610] [ticket-vscode] Rust/WASM port track
- priority: `high`
- summary: Port `memory-viewers/memory-api/tools/ticket-vscode` from a TypeScript-heavy implementation to a Rust/WASM-backed VS Code extension architecture.
- ref: `memory-viewers/memory-api/.ticket/tickets/6d07d610-75c1-448a-afd5-6ae15098ca21/ticket.toml`


### Component: unspecified

<!-- ticket-index:entry id=9ac0a02b-965f-45f3-b8c9-97a063e3bc55 slug=ready/unspecified digest=f9d1c32f045b -->
#### [9ac0a02b] Epic: Viewer Component Port -- framework migration and API surface (Preact to Dioxus/Leptos)
- ref: `.ticket/tickets/9ac0a02b-965f-45f3-b8c9-97a063e3bc55/ticket.toml`


### Component: viewer-api

<!-- ticket-index:entry id=6bbda148-e144-4dff-92de-dd6584c82bd7 slug=ready/viewer-api digest=7e6b9009e60d -->
#### [6bbda148] [viewer-ctl] Implement uninstall command for managed viewers
- priority: `high`
- summary: The install contract now validates `viewer-ctl install` for all managed viewers, but `viewer-ctl` still has no first-class uninstall/remove command. That leaves `VIEW-04` as a manual gap in the insta...
- ref: `memory-viewers/viewer-api/.ticket/tickets/6bbda148-e144-4dff-92de-dd6584c82bd7/ticket.toml`

<!-- ticket-index:entry id=ba0fd25e-c23b-48a4-934c-a30542f6fca9 slug=ready/viewer-api digest=2868618f0693 -->
#### [ba0fd25e] demo-viewer: README + viewer-api docs cross-references
- summary: Add a top-level note in `tools/viewer/viewer-api/README.md` linking to
- ref: `memory-viewers/viewer-api/.ticket/tickets/ba0fd25e-c23b-48a4-934c-a30542f6fca9/ticket.toml`

<!-- ticket-index:entry id=b83b1002-41ba-4eb7-9f1a-c20cbf49137b slug=ready/viewer-api digest=52fe5f3c72c0 -->
#### [b83b1002] demo-viewer: e2e harness + WebGPU launch profile + helpers
- summary: Add the demo-viewer e2e harness so per-feature tickets only need to
- ref: `memory-viewers/viewer-api/.ticket/tickets/b83b1002-41ba-4eb7-9f1a-c20cbf49137b/ticket.toml`

<!-- ticket-index:entry id=76378ed1-f50d-43d9-9414-04cfc3232a00 slug=ready/viewer-api digest=c54b88e2cbe6 -->
#### [76378ed1] demo-viewer: feature page — auth middleware
- summary: Implement the demo page that showcases the `viewer-api/auth-middleware` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/76378ed1-f50d-43d9-9414-04cfc3232a00/ticket.toml`

<!-- ticket-index:entry id=db1cccef-6712-4702-ae58-fe23dacc029f slug=ready/viewer-api digest=5adab3327b7c -->
#### [db1cccef] demo-viewer: feature page — client log
- summary: Implement the demo page that showcases the `viewer-api/client-log` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/db1cccef-6712-4702-ae58-fe23dacc029f/ticket.toml`

<!-- ticket-index:entry id=e737092d-6083-4bf8-ba0e-1ac22d7c521b slug=ready/viewer-api digest=9321703add12 -->
#### [e737092d] demo-viewer: feature page — code viewer
- summary: Implement the demo page that showcases the `viewer-api/components/code-viewer` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/e737092d-6083-4bf8-ba0e-1ac22d7c521b/ticket.toml`

<!-- ticket-index:entry id=b543e4ad-7ac4-4fd0-abb7-59a725affa64 slug=ready/viewer-api digest=0651ad6f2f35 -->
#### [b543e4ad] demo-viewer: feature page — dev proxy
- summary: Implement the demo page that showcases the `viewer-api/dev-proxy` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/b543e4ad-7ac4-4fd0-abb7-59a725affa64/ticket.toml`

<!-- ticket-index:entry id=42bd0dc8-cc28-46b3-9535-2d1207b18ae6 slug=ready/viewer-api digest=87dc6f5c2b52 -->
#### [42bd0dc8] demo-viewer: feature page — graph3d (WebGPU)
- summary: Implement the demo page that showcases the `viewer-api/components/graph3d` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/42bd0dc8-cc28-46b3-9535-2d1207b18ae6/ticket.toml`

<!-- ticket-index:entry id=6006ec27-babd-4656-9eca-78bdd5eb5b47 slug=ready/viewer-api digest=6ae424245474 -->
#### [6006ec27] demo-viewer: feature page — icons spinner
- summary: Implement the demo page that showcases the `viewer-api/components/icons-spinner` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/6006ec27-babd-4656-9eca-78bdd5eb5b47/ticket.toml`

<!-- ticket-index:entry id=fc0282b5-844c-4101-9391-c926ffdaf1d7 slug=ready/viewer-api digest=71baba43d177 -->
#### [fc0282b5] demo-viewer: feature page — layout
- summary: Implement the demo page that showcases the `viewer-api/components/layout` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/fc0282b5-844c-4101-9391-c926ffdaf1d7/ticket.toml`

<!-- ticket-index:entry id=02025547-027b-43f7-bcd7-6a212108085f slug=ready/viewer-api digest=29f840f3e38e -->
#### [02025547] demo-viewer: feature page — pagination query
- summary: Implement the demo page that showcases the `viewer-api/pagination-query` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/02025547-027b-43f7-bcd7-6a212108085f/ticket.toml`

<!-- ticket-index:entry id=8de2f8e2-43b6-4de1-a9f7-54fc64c2bdab slug=ready/viewer-api digest=0f2f2b48ea77 -->
#### [8de2f8e2] demo-viewer: feature page — server infra
- summary: Implement the demo page that showcases the `viewer-api/server-infra` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/8de2f8e2-43b6-4de1-a9f7-54fc64c2bdab/ticket.toml`

<!-- ticket-index:entry id=ed8252fc-371d-4134-a981-5af988c4241a slug=ready/viewer-api digest=1ef83a9b4ba5 -->
#### [ed8252fc] demo-viewer: feature page — session
- summary: Implement the demo page that showcases the `viewer-api/session` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/ed8252fc-371d-4134-a981-5af988c4241a/ticket.toml`

<!-- ticket-index:entry id=48530193-0637-4709-8239-e8f3e1cc0eba slug=ready/viewer-api digest=108032f1d0fc -->
#### [48530193] demo-viewer: feature page — source
- summary: Implement the demo page that showcases the `viewer-api/source` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/48530193-0637-4709-8239-e8f3e1cc0eba/ticket.toml`

<!-- ticket-index:entry id=258ed497-b5ca-4622-96a3-6f1ea210e7bb slug=ready/viewer-api digest=553008fec1ed -->
#### [258ed497] demo-viewer: feature page — sse
- summary: Implement the demo page that showcases the `viewer-api/sse` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/258ed497-b5ca-4622-96a3-6f1ea210e7bb/ticket.toml`

<!-- ticket-index:entry id=1efec195-f8b4-4571-b073-806cac0b66ce slug=ready/viewer-api digest=a4f69b7a7eac -->
#### [1efec195] demo-viewer: feature page — store primitives
- summary: Implement the demo page that showcases the `viewer-api/store-primitives` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/1efec195-f8b4-4571-b073-806cac0b66ce/ticket.toml`

<!-- ticket-index:entry id=0eef1873-0626-4a87-93bc-51d182808e16 slug=ready/viewer-api digest=aca449a96ee8 -->
#### [0eef1873] demo-viewer: feature page — tab bar
- summary: Implement the demo page that showcases the `viewer-api/components/tab-bar` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/0eef1873-0626-4a87-93bc-51d182808e16/ticket.toml`

<!-- ticket-index:entry id=6f924445-ea9c-46e9-b051-b5aab6b798fa slug=ready/viewer-api digest=11e782fdeb06 -->
#### [6f924445] demo-viewer: feature page — theme settings
- summary: Implement the demo page that showcases the `viewer-api/theme-settings` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/6f924445-ea9c-46e9-b051-b5aab6b798fa/ticket.toml`

<!-- ticket-index:entry id=8d0e9879-5e42-449f-90a6-0060dbde112f slug=ready/viewer-api digest=53983fd56ca8 -->
#### [8d0e9879] demo-viewer: feature page — tracing
- summary: Implement the demo page that showcases the WASM-tracing pipeline
- ref: `memory-viewers/viewer-api/.ticket/tickets/8d0e9879-5e42-449f-90a6-0060dbde112f/ticket.toml`

<!-- ticket-index:entry id=ad056493-716c-4c32-b8f6-9b67a25bc52e slug=ready/viewer-api digest=d167d924949d -->
#### [ad056493] demo-viewer: feature page — tracing
- summary: Implement the demo page that showcases the WASM-tracing pipeline
- ref: `memory-viewers/viewer-api/.ticket/tickets/ad056493-716c-4c32-b8f6-9b67a25bc52e/ticket.toml`

<!-- ticket-index:entry id=3df77f25-0f1c-4c1c-a2a8-e9c885f275db slug=ready/viewer-api digest=e83cd3cd7358 -->
#### [3df77f25] demo-viewer: feature page — tree view
- summary: Implement the demo page that showcases the `viewer-api/components/tree-view` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/3df77f25-0f1c-4c1c-a2a8-e9c885f275db/ticket.toml`

<!-- ticket-index:entry id=9d7d97bb-fc65-4374-8de8-f22bd2a05c18 slug=ready/viewer-api digest=85ad804e0517 -->
#### [9d7d97bb] demo-viewer: feature page — wgpu overlay (WebGPU)
- summary: Implement the demo page that showcases the `viewer-api/effects/wgpu-overlay` feature surface.
- ref: `memory-viewers/viewer-api/.ticket/tickets/9d7d97bb-fc65-4374-8de8-f22bd2a05c18/ticket.toml`

<!-- ticket-index:entry id=ee2b9e6d-e093-41df-9838-d6ab7dfde0fa slug=ready/viewer-api digest=7e979df9fd0c -->
#### [ee2b9e6d] demo-viewer: manual validation epic (signs off `verified` on the umbrella spec)
- summary: Final sign-off ticket. Closing this ticket transitions the umbrella spec
- ref: `memory-viewers/viewer-api/.ticket/tickets/ee2b9e6d-e093-41df-9838-d6ab7dfde0fa/ticket.toml`

<!-- ticket-index:entry id=b779c650-0775-4e4f-a692-3eaaa939a910 slug=ready/viewer-api digest=1ca6d42965e0 -->
#### [b779c650] demo-viewer: scaffold bin crate + Dioxus SPA
- summary: Create the demo-viewer crate skeleton inside the `viewer-api` workspace
- ref: `memory-viewers/viewer-api/.ticket/tickets/b779c650-0775-4e4f-a692-3eaaa939a910/ticket.toml`

<!-- ticket-index:entry id=5d9e331b-dc18-444b-af45-90a14d096847 slug=ready/viewer-api digest=dbacd5704860 -->
#### [5d9e331b] demo-viewer: viewer-ctl integration + nav generator
- summary: Add a `[viewers.demo-viewer]` entry to `viewer-ctl.toml` (port 3099,
- ref: `memory-viewers/viewer-api/.ticket/tickets/5d9e331b-dc18-444b-af45-90a14d096847/ticket.toml`


### Component: viewer-api-dioxus

<!-- ticket-index:entry id=f00204fc-f33f-4cd6-9b5f-395071f4e118 slug=ready/viewer-api-dioxus digest=6d9fa0e1ec49 -->
#### [f00204fc] Bug: ticket-viewer theme inconsistency — --panel-bg hardcoded dark, breaks light themes
- priority: `high`
- summary: `viewer-api-dioxus` defines two parallel surface palettes in `public/css/variables.css`:
- ref: `memory-viewers/viewer-api/.ticket/tickets/f00204fc-f33f-4cd6-9b5f-395071f4e118/ticket.toml`

<!-- ticket-index:entry id=dc83b7b4-4b0f-4732-9163-488ef0c6bcc4 slug=ready/viewer-api-dioxus digest=74032b2ec4f0 -->
#### [dc83b7b4] UI: transparent context-adaptive header & sidebar action buttons (IconButton + Chip)
- priority: `medium`
- summary: The action buttons in the ticket-viewer header (`🎨 Theme settings`, `☑ Batch`, `+ New Ticket`) are styled inline in `routes.rs` with three different background tokens (`var(--bg-secondary)`, `var(--a...
- ref: `memory-viewers/viewer-api/.ticket/tickets/dc83b7b4-4b0f-4732-9163-488ef0c6bcc4/ticket.toml`


### Component: viewer-api-leptos

<!-- ticket-index:entry id=92d5223b-05a4-4b80-ae6a-f5f5d45db2fc slug=ready/viewer-api-leptos digest=4fa835d23d8f -->
#### [92d5223b] Feature: Complete theme system — colors, effects, presets, CSS variables
- summary: The Leptos frontend has a minimal theme system: 5 hardcoded presets, a simple button grid in a Settings tab, GPU-only uniforms with no CSS variable injection, and no color editing. The TS version has...
- ref: `memory-viewers/viewer-api/.ticket/tickets/92d5223b-05a4-4b80-ae6a-f5f5d45db2fc/ticket.toml`


### Component: watcher

<!-- ticket-index:entry id=e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5 slug=ready/watcher digest=340107995c7d -->
#### [e4c6d8f1] [bootstrap][T5] handle early-stop recovery and reassignment
- summary: Agent sessions can terminate unexpectedly at any point during an assignment: stdio disconnect, heartbeat/liveness timeout, repeated auth failures, or explicit worker abort. The executor must handle a...
- ref: `.ticket/tickets/e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5/ticket.toml`

