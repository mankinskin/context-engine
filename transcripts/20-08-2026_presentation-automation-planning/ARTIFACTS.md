# Artifact Inventory

This inventory freezes the repository evidence used by the refined roadmap.
Later dossier stages cite these entries rather than rediscovering the same
surfaces.

| Artifact | Kind | Current state | Relevance |
| --- | --- | --- | --- |
| [input.md](input.md) | Raw transcript | Present | Verbatim source for the requested work. |
| [input.clean.md](input.clean.md) | Clean transcript | Present | Denoised requirements and earlier refinement findings. |
| [REVIEW.md](REVIEW.md) | Review gate | Approved for roadmap work | Defines the bounded initial track and exclusions. |
| [2ccde9ee Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) | Specification | Reviewed | Authoritative presentation-system requirements, including conceptual-deck additions. |
| [0ee95228 presentation epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml) | Epic ticket | Planned | Parent delivery track and existing phased plan. |
| [89b0c64a Phase 1](../../.ticket/tickets/89b0c64a-b573-4f7b-b692-fa3d383e386c/ticket.toml) | Tracker ticket | Planned | Slidev toolchain, sample deck, and initial Playwright baseline. |
| [3cdcaf3b Phase 2](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml) | Tracker ticket | Planned | `presentation-api`, deck store, materialization, and facade surface. |
| [969ffba0 First real deck](../../.ticket/tickets/969ffba0-6bff-4a58-9d74-18368ac87875/ticket.toml) | Tracker ticket | Planned | Workflow-tools overview deck that will consume the new track. |
| [e01dd058 graph embedding](../../.ticket/tickets/e01dd058-a539-4620-87b2-0a4895114ca2/ticket.toml) | Tracker ticket | Planned | Standalone WASM graph embedding; later than the typed projection contract. |
| [60222b57 theme pack](../../.ticket/tickets/60222b57-095d-4c9e-b83a-70c3dd8690ba/ticket.toml) | Tracker ticket | Planned | Future visual presets and descriptors; not a blocker for contract/extractor work. |
| [06cfe998 Peek API](../../.ticket/tickets/06cfe998-c2e1-48a4-83e9-11e85e7c40f4/ticket.toml) | Tracker ticket | Done | Bounded inspection and skeletonization baseline; not yet a repository graph model. |
| `1500a9e6-293f-4803-969d-0dcabeaa470a` conceptual deck contract | Tracker ticket | Planned, DB-backed | New cross-cutting contract for source locks, claims, sidecars, and generated-path ownership. Resolve with `mcp_ticket_get_ticket`; no filesystem manifest is materialized yet. |
| `693763fc-e4c1-4c93-b39f-5e0958b57d19` typed projections | Tracker ticket | Planned, DB-backed | New extractor track for authoritative specification facts and distinct Git/Cargo projections. Resolve with `mcp_ticket_get_ticket`; no filesystem manifest is materialized yet. |
| `ec1f452d-8eba-488c-bcfe-8dd8728130f1` conceptual deck validation | Tracker ticket | Planned, DB-backed | New deterministic materialization and static per-slide validation track. Resolve with `mcp_ticket_get_ticket`; no filesystem manifest is materialized yet. |
| [root presentation README](../../.presentation/README.md) | Documentation | Present | Existing Slidev deck layout and cross-repository composition behavior. |
| [root deck manifest](../../.presentation/deck.toml) | Deck source | Present | Legacy singleton deck that needs explicit migration/discovery policy. |
| [root deck slides](../../.presentation/slides.md) | Deck source | Present | Existing root compositional presentation source. |
| [workflow-tools deck](../../workflow-tools/.presentation/slides.md) | Deck source | Present | Existing repository-local composable slide input. |
| [presentation E2E config](../../.presentation/e2e/playwright.config.ts) | Validation config | Present | Existing Playwright harness to extend to static, per-slide checks. |
| [.gitmodules](../../.gitmodules) | Repository metadata | Present | Authoritative submodule containment input. |
| [root Cargo manifest](../../Cargo.toml) | Workspace metadata | Present | Root Rust workspace membership input. |
| [context-stack Cargo manifest](../../context-stack/Cargo.toml) | Workspace metadata | Present | Independent nested workspace input. |
| [repo map](../../repo_map.toon) | Generated structure map | Present | Compact structural orientation input; not a replacement for typed extraction. |
| [spec-api source](../../workflow-tools/spec/crates/spec-api/src/) | Implementation reference | Present after nested-submodule initialization | Closest file-backed manifest, section, and store conventions used by the current workflow-tools checkout. |
| [browser validation rules](../../AGENTS.md) | Repository instruction | Present | External-browser and Playwright screenshot requirements. |
| [prompt-ingestion pipeline](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) | Repository instruction | Present | Governs this dossier and roadmap handoff boundary. |

## Ordering Constraints

1. The Phase 1 toolchain ticket precedes the Phase 2 API/store ticket.
2. The first-real-deck ticket depends on theme and graph-embedding work; it is
   not the first validation target for the contract/extractor track.
3. Source locks, typed claims, sidecars, and overwrite boundaries must be
   specified before generated output is allowed.
4. Git/submodule and Cargo structures must remain separate typed projections.
5. Static, per-slide Playwright verification is the acceptance gate for any
   generated human-facing deck.