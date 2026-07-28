# Presentation System

## Purpose

A repository-native system that turns a text script into an interactive, web-native
presentation cheaply and repeatably. "Cheaply" means all three of: low token cost
(templates + skills so a mid-tier model can author a deck), low infra cost (static
output, no runtime services required for a built deck), and low authoring effort
(one command from script to deck).

## Decisions (confirmed 2026-07-28)

| Decision | Choice |
|---|---|
| Rendering stack | Slidev (Vue 3 + Vite, markdown-first) |
| System shape | Full domain: `presentation-api` + `presentation` facade (cli/mcp/http bins) + `presentation-viewer` |
| Domain name | `presentation` |
| Crate placement | Under `memory-viewers/presentation-viewer/` (viewer-shaped domain) |
| Deck store root | `.presentation/` at repo root, alongside `.ticket/` and `.spec/` |
| Deck storage model | `deck.toml` manifest + `slides.md` body (mirrors `spec.toml` + `body.md`) |
| Authoring input | Standard Slidev templates written by agents |
| Viewer shape | Rust server + Dioxus shell (deck browser) that iframes built Slidev decks |
| Deck hosting | Multiple decks: index at `/`, deck at `/deck/{id}` |
| Viewer port | 3003 |
| Theme base | **Stock Slidev theme unmodified in Phase 1**; custom theme pack is a separate later ticket |
| Styling | Repo theme pack + optional per-deck override + curated layout presets (delivered by the custom-theme ticket) |
| HTTP transport | **Full HTTP CRUD surface in v1** |
| Live repo viewers in slides | viewer-api's shared graph component extracted and compiled to standalone WASM |
| Skills | Vendor 4 external skills + author one repo-local presentation-workflow skill |
| Agent surface | Presentation agent mode + workflow prompt in `.agents/prompts/` |
| Legacy | `Presentation.agent 2.md` retired once the custom theme pack carries its presets |
| Phasing | Phase 1: working deck end-to-end on a stock theme. Phase 2: formalize the domain crate. |
| Validation | Playwright E2E + per-slide screenshots, per AGENTS.md browser rules |
| First real deck | Workflow-tools suite introduction and overview |

## Requirements

### R1 — Script to deck
An author (human or agent) supplies a text script. The system produces a Slidev deck
whose slides use curated layout presets. Slide boundaries and layout selection follow
standard Slidev templates. Phase 1 uses a stock Slidev theme; the repo theme pack
supersedes it in a later ticket without changing the authoring contract.

### R2 — Deck as a first-class entity
Each deck is stored under `.presentation/` as a `deck.toml` manifest plus a `slides.md`
body. `presentation-api` owns: deck registry CRUD, slide-level CRUD (add/reorder/delete
as addressable entities), materialization of the deck entity into on-disk Slidev
sources (`slides.md` + `components/`), build orchestration (invoking `slidev dev`/
`slidev build` and tracking artifacts), a theme/preset registry resolving theme-pack
plus per-deck override, and traceability links to the specs and tickets a deck presents.

### R3 — Interactivity
V1 slides must support: charts (ECharts/Chart.js) from inline data, Mermaid diagrams,
custom TS/Vue components dropped in by the agent, embedded Rust-compiled WASM modules,
and viewer-api's shared graph component compiled to WASM and embedded standalone with
no dependency on a running viewer.

### R4 — Outputs
Dev server with hot reload for the authoring loop; static SPA build for hosting;
presenter mode with speaker notes; and a managed viewer registered in `viewer-ctl.toml`.
All three transports ship: `presentation-cli`, `presentation-mcp`, `presentation-http`
with a full CRUD HTTP surface.

### R5 — Theming
Phase 1 ships on a stock Slidev theme so the end-to-end loop is provable without theme
work. A dedicated later ticket delivers the repo theme pack: colors, typography, layouts,
and curated per-slide presets shared across all decks, with optional per-deck override.
The retired HTML agent's proven principles — full-viewport hero, sticky nav with progress
bar, one idea per screen-height section, dark CTA section bookending the hero — are
encoded as presets in that theme pack, and only then is the legacy agent deleted.

### R6 — Agent enablement
Vendored skills (`marcoshaber99/slidev-skills`, `yoanbernabeu/slidev-skills`,
`neversight/slidev-syntax-guide`, `zarazhangrui/frontend-slides`) plus a repo-local
presentation-workflow skill, a Presentation agent mode, and a `.agents/prompts/`
workflow prompt covering the script -> deck -> build -> verify loop.

### R7 — Validation
A deck is done when Playwright E2E passes with a screenshot captured per slide, per
the mandatory browser-verification rules in AGENTS.md.

## Non-Goals

- PPTX/PowerPoint generation.
- PDF export (explicitly not selected for v1).
- Real-time multi-user collaborative editing of decks.
- Replacing the doc-viewer or spec-viewer surfaces.
- A bespoke Slidev theme in Phase 1.

## Acceptance Criteria

- AC1: A text script becomes a rendered Slidev deck via a documented single command.
- AC2: `.presentation/` contains at least one deck as `deck.toml` + `slides.md`, readable
  through `presentation-api`.
- AC3: `presentation-viewer` runs on port 3003, lists decks at `/`, and serves a deck at
  `/deck/{id}`, and is registered in `viewer-ctl.toml` as a server + frontend pair.
- AC4: The workflow-tools introduction deck exists, builds, and passes Playwright E2E with
  per-slide screenshots.
- AC5: The four external skills plus the repo-local presentation-workflow skill are present
  under `.agents/skills/` and recorded in `skills-lock.json`.
- AC6: `Presentation.agent 2.md` is removed and its design principles are represented in the
  custom theme pack presets.
- AC7: `presentation-http` exposes full CRUD over decks and slides.

## Open Decisions

None. All decisions resolved 2026-07-28.
