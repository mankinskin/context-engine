# Epic: Presentation System

Governing spec: `presentation-system` (`2ccde9ee-85ac-4c87-9601-f6099f5be01c`).

## Goal

Turn a text script into an interactive, web-native presentation cheaply and repeatably.
"Cheaply" = low token cost (templates + skills so a mid-tier model can author), low infra
cost (static output, no runtime service required for a built deck), low authoring effort
(one command from script to deck).

## Confirmed architecture

- Rendering stack: **Slidev** (Vue 3 + Vite, markdown-first).
- Domain name: `presentation`. Crates live under `memory-viewers/presentation-viewer/`.
- Deck store: `.presentation/` at repo root; each deck = `deck.toml` + `slides.md`.
- Viewer: Rust server + Dioxus shell (deck browser) iframing built Slidev decks;
  index at `/`, deck at `/deck/{id}`; port **3003**; registered in `viewer-ctl.toml`.
- Interactivity v1: ECharts/Chart.js, Mermaid, agent-authored TS/Vue components,
  Rust-compiled WASM modules, shared repo graph component compiled to WASM standalone.
- Outputs: dev server w/ hot reload, static SPA build, presenter mode + speaker notes,
  managed viewer. (No PDF export in v1.)
- Theming: repo theme pack + optional per-deck override + curated layout presets.
- Skills: vendor `marcoshaber99/slidev-skills`, `yoanbernabeu/slidev-skills`,
  `neversight/slidev-syntax-guide`, `zarazhangrui/frontend-slides`; author one
  repo-local presentation-workflow skill on top.
- Agent surface: a Presentation agent mode + a `.agents/prompts/` workflow prompt.
- `Presentation.agent 2.md` is retired; its hero/sticky-nav/CTA-bookend principles
  migrate into the theme pack presets.
- Validation: Playwright E2E + per-slide screenshots (AGENTS.md browser rules).

## Phasing

**Phase 1 — working deck end-to-end.** Filesystem + npm scripts only, no MCP/CLI surface.
Slidev toolchain, theme pack + presets, sample deck, Playwright verification.

**Phase 2 — formalize the domain.** `presentation-api` + `presentation` facade crate
(cli/mcp/http bins per the workflow-tools domain crate contract) + `.presentation/`
store schema, then the viewer, then skills/agent/first real deck.

## First real deck

Workflow-tools suite introduction and overview.

## Acceptance criteria

- AC1: Documented single command turns a text script into a rendered Slidev deck.
- AC2: `.presentation/` holds decks as `deck.toml` + `slides.md`, readable via `presentation-api`.
- AC3: `presentation-viewer` on port 3003, deck index at `/`, deck at `/deck/{id}`,
  registered in `viewer-ctl.toml` as server + frontend.
- AC4: Workflow-tools intro deck builds and passes Playwright E2E with per-slide screenshots.
- AC5: Four vendored skills + repo-local presentation-workflow skill under `.agents/skills/`,
  recorded in `skills-lock.json`.
- AC6: `Presentation.agent 2.md` removed, principles represented as theme-pack presets.

## Open decisions

- Slidev theme base: custom from scratch vs fork an existing theme.
- Whether `presentation-http` is needed in v1 or only cli + mcp.
- Which shared graph component crate is extracted for standalone WASM slide embedding.
