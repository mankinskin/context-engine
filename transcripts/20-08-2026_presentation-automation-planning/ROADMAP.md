# Presentation Automation Roadmap

## Relevant Artifact IDs

- [2ccde9ee Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml)
- [0ee95228 presentation epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml)
- [89b0c64a Phase 1](../../.ticket/tickets/89b0c64a-b573-4f7b-b692-fa3d383e386c/ticket.toml)
- [3cdcaf3b Phase 2](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml)
- Ticket `1500a9e6-293f-4803-969d-0dcabeaa470a` — conceptual deck contract (DB-backed; resolve through `mcp_ticket_get_ticket`)
- Ticket `693763fc-e4c1-4c93-b39f-5e0958b57d19` — typed projections (DB-backed; resolve through `mcp_ticket_get_ticket`)
- Ticket `ec1f452d-8eba-488c-bcfe-8dd8728130f1` — conceptual deck validation (DB-backed; resolve through `mcp_ticket_get_ticket`)
- [ARTIFACTS.md](ARTIFACTS.md), [01-conceptual-input-contract.md](01-conceptual-input-contract.md), [02-projection-extractors.md](02-projection-extractors.md), and [03-deck-generation-validation.md](03-deck-generation-validation.md)

## Active Blockers

1. [89b0c64a Phase 1](../../.ticket/tickets/89b0c64a-b573-4f7b-b692-fa3d383e386c/ticket.toml) and [3cdcaf3b Phase 2](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml) are planned. Preflight the Phase 2 output with `test -f memory-viewers/presentation-viewer/crates/presentation-api/Cargo.toml`; until it passes, the new contract and extractor test targets cannot run.
2. Before the extractor task reads `spec-api`, run `git -C workflow-tools submodule update --init spec`, then preflight `test -f workflow-tools/spec/crates/spec-api/src/lib.rs`.

No design decision blocks the roadmap. Source-lock serialization and topology
viewport/density values are package-level decisions with fixture validation.

The three new tickets are authoritative ticket-store entities but their
filesystem manifests are not materialized in this checkout. A cold executing
session must resolve their IDs through `mcp_ticket_get_ticket` before use.

## Validation Gates

1. Run the ticket-specific Rust fixture commands declared in the tickets after `presentation-api` exists:
   - `cargo test -p presentation-api --test conceptual_input_contract`
   - `cargo test -p presentation-api --test managed_output_boundary`
   - `cargo test -p presentation-api --test extraction_adapters`
   - `cargo test -p presentation-api --test typed_projections`
2. Validate full domain behavior with `cargo test -p presentation-api`.
3. Build the static deck with `npm --prefix .presentation run build`.
4. Run `npm --prefix .presentation/e2e run test`; the evolved suite must visit every manifest slide at a fixed viewport, save per-slide screenshots, and fail on console errors or missing assets.
5. Review the generated-source Git patch before accepting explicit replacement output.

## Full Roadmap

1. **Complete existing foundation — ticket-backed.** Complete [89b0c64a Phase 1](../../.ticket/tickets/89b0c64a-b573-4f7b-b692-fa3d383e386c/ticket.toml), then [3cdcaf3b Phase 2](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml), so the Slidev toolchain and `presentation-api` persistence/materialization boundary exist. Acceptance is the tickets' existing definition of done. These are multi-session tracked work, not roadmap sub-tasks.
2. **Define conceptual deck authority — ticket-backed.** Execute ticket `1500a9e6-293f-4803-969d-0dcabeaa470a`: one outcome is a tested source-lock, claim/citation, sidecar, and managed-output contract. Its acceptance is the fixture suite in [01-conceptual-input-contract.md](01-conceptual-input-contract.md). It depends on Phase 2.
3. **Extract citable repository facts — ticket-backed.** Execute ticket `693763fc-e4c1-4c93-b39f-5e0958b57d19`: one outcome is independently typed specification, Git containment, Cargo membership, and Cargo dependency facts. Its acceptance is the fixture suite in [02-projection-extractors.md](02-projection-extractors.md). It depends on item 2 and the nested `spec` submodule preflight.
4. **Generate and prove a conceptual deck — ticket-backed.** Execute ticket `ec1f452d-8eba-488c-bcfe-8dd8728130f1`: one outcome is deterministic managed output with static, per-slide human-facing evidence. Its acceptance is the build and browser suite in [03-deck-generation-validation.md](03-deck-generation-validation.md). It depends on item 3.
5. **Apply the completed track to the first real deck — existing ticket-backed.** Continue [969ffba0 First real deck](../../.ticket/tickets/969ffba0-6bff-4a58-9d74-18368ac87875/ticket.toml) only after its existing theme and graph-embedding dependencies, plus item 4, are fulfilled.

## Heads-Up Notes

- The current root deck is a legacy singleton `.presentation/deck.toml` plus `slides.md`; do not make a new registry canonical without migration/discovery behavior.
- `.gitmodules` containment, Cargo membership, and Cargo dependencies are overlapping facts, never one unlabeled hierarchy.
- `Peek` is a bounded-inspection tool; this roadmap must not reframe it as a complete repository graph engine.
- Telemetry can illustrate a declarative workflow but cannot create normative claims in this track.
- The current presentation E2E only covers title-level behavior; generated conceptual decks require static, per-slide screenshot evidence.
- Do not build flagship topology slides until the deferred theme/preset work fixes legend, role, density, and viewport contracts.