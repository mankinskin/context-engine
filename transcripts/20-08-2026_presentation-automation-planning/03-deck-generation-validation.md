# 03. Deterministic Generation and Static Validation

## Outcome

Materialize managed conceptual decks from locked conceptual inputs and typed
projections, preserving presenter-note coverage and sidecars. Build static
Slidev output and prove each resulting slide in a fixed-viewport browser run.

## Inputs

- [Conceptual input contract](01-conceptual-input-contract.md).
- [Projection extractors](02-projection-extractors.md).
- [Phase 1 toolchain](../../.ticket/tickets/89b0c64a-b573-4f7b-b692-fa3d383e386c/ticket.toml), [Phase 2 API plan](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml), and [existing presentation E2E harness](../../.presentation/e2e/playwright.config.ts).

## Required Behavior

- Identical locks and inputs create byte-identical managed output.
- Changed locks explicitly fail or mark output stale; generation changes only
  declared paths after explicit replacement.
- Every conceptual slide has presenter notes or declares `no notes required`.
- Static E2E derives expected slide count from the manifest, visits every
  slide, captures a screenshot, asserts required citations and legends, and
  fails on console errors or missing assets.

## Non-Goals

- Implement the custom theme pack or flagship topology visual.
- Add live data, viewer behavior, cross-language extraction, or normative
  telemetry.

## Validation

Run `cargo test -p presentation-api`, then `npm run build` from
`.presentation/`, followed by the presentation Playwright command from
`.presentation/e2e/package.json`. The suite must run against static output and
store a screenshot for every manifest slide at the declared viewport.

## Tracking

Ticket `ec1f452d-8eba-488c-bcfe-8dd8728130f1` depends on DB-backed ticket `693763fc-e4c1-4c93-b39f-5e0958b57d19`; resolve both through `mcp_ticket_get_ticket`. Density limits and visual baselines stay with the deferred theme/preset work, so this package must not ship a flagship topology slide.
