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

Presets also encode a small semantic visual grammar: a fixed vocabulary of roles — goal,
input, evidence, decision, tool, verification, output, risk, unresolved-question — each
mapped consistently to color, edge style, and typography across every preset, with
non-color/text distinctions preserved so the mapping stays accessible and machine-readable
for authoring agents.

### R6 — Agent enablement
Vendored skills (`marcoshaber99/slidev-skills`, `yoanbernabeu/slidev-skills`,
`neversight/slidev-syntax-guide`, `zarazhangrui/frontend-slides`) plus a repo-local
presentation-workflow skill, a Presentation agent mode, and a `.agents/prompts/`
workflow prompt covering the script -> deck -> build -> verify loop.

### R7 — Validation
A deck is done when Playwright E2E passes with a screenshot captured per slide, per
the mandatory browser-verification rules in AGENTS.md.

### R8 — Specification-derived conceptual inputs
The next track produces conceptual decks for a live human audience from specifications.
Specifications are authoritative; implementation and documentation evidence may only be
recorded as structured disagreement sidecars, not used to replace normative claims. Each
claim declares whether it is quoted or synthesized and carries claim-level source selectors
and citations. Sidecars record category, severity, owner, resolution state, and source
locations. An unresolved material contradiction blocks publication or visibly qualifies the
affected slide.

Each conceptual visual is backed by an evidence card: a structured record with a source path
or entity URN, a stable claim identifier, a content digest, and a capture timestamp, so every
generated statement remains traceable to the exact repository fact it renders.

### R9 — Locked and bounded generation
Managed generation records a source lock containing specification paths and sections with
content hashes, transform and theme/preset versions, and the Git base. If a lock no longer
matches, the deck is marked stale or regeneration fails explicitly. Generation may write only
declared generated paths, rejects path traversal and symlink escapes, separates generated
sources from a human-owned overlay, and preflights unexpected generated-output modifications
before an explicit replace. Git patch review is the review mechanism for generated output.

### R10 — Typed repository projections
Conceptual structural slides initially cover Git repository/submodule topology and Cargo
workspace/Rust crate topology. Git containment, Cargo workspace/crate membership, and Cargo
dependency relationships are distinct named projections. Every node and edge has a type and
source; the system must not imply an unlabeled single tree. Extraction adapters normalize
these facts separately from `presentation-api`, which continues to own persistence,
materialization, builds, and traceability.

The combined Git-repository-plus-Rust-crate view required by this track is one addressable
graph over a shared node set keyed by path; Git containment, Cargo workspace/crate membership,
and Cargo dependency remain separately labeled, distinctly-typed edge collections layered on
that shared graph. It is never flattened into a single untyped parent-child tree.

### R11 — Declarative workflows and bounded provenance
Workflow content derives from formal declarations. Durable session telemetry may be shown only
as illustrative examples, is labeled as such, and cannot alter normative claims. Visual
provenance is either `synthetic` or a pinned `snapshot`; live visual sources are excluded from
this track. Every generated conceptual slide includes presenter notes or explicitly declares
`no notes required`.

### R12 — Deterministic discovery and materialization
Before a multi-deck registry becomes canonical, the system defines deterministic discovery and
migration for legacy singleton `.presentation/deck.toml` sources and deterministic
cross-repository imports. Materialization creates a managed deck, its presenter notes, source
lock, and disagreement sidecar deterministically from the selected inputs.

### R13 — Static per-slide evidence
Validation materializes and builds static output. It derives the expected slide count from the
deck manifest, visits every slide at a fixed viewport, and captures one screenshot per slide.
The suite asserts required citations and legends and fails on browser console errors or missing
assets; title-page-only checks are insufficient.

### R14 — Deferred topology visual preset
Before flagship structural slides are introduced, the later theme work defines a topology visual
preset contract with a required legend, named node and edge roles, density limits, and baseline
screenshots. Cross-language parsing and telemetry-derived normativity are out of scope for this
track.

### R15 — Deck information architecture
Every deck and slide declares a position on a shared information-architecture ladder
(audience-problem, workflow, tool-role, domain-contract, component, implementation-detail),
its prerequisite levels, and any drill-down target to a more detailed nested deck. Composed
decks share this vocabulary so a composing deck and an imported sub-repository deck remain
coherent. Levels are author-declared, not inferred.

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
- AC8: A conceptual deck can be materialized from locked specification inputs with every claim
  classified as quoted or synthesized and cited to a source selector, and every generated
  visual carries an evidence card (source path/URN, claim id, content digest, capture timestamp).
- AC9: A changed source lock produces explicit stale or failed-regeneration behavior, and writes
  outside declared generated paths, through symlinks, or over unexpected modifications are
  rejected until explicitly replaced.
- AC10: Fixture validation distinguishes typed Git containment, Cargo membership, and Cargo
  dependency projections and preserves a source for every node and edge.
- AC11: Formal workflow slides and any telemetry examples visibly distinguish normative claims
  from illustrative data; all visual provenance is synthetic or a pinned snapshot.
- AC12: Discovery/migration covers a legacy singleton deck and deterministic cross-repository
  imports, and generated conceptual slides include presenter notes or `no notes required`.
- AC13: Static Playwright E2E derives slide count from the manifest, visits and screenshots every
  slide at a fixed viewport, verifies required citations and legends, and fails on console errors
  or missing assets.
- AC14: A topology preset contract defines its legend, node/edge roles, density limits, and
  baseline screenshots before flagship structural slides ship.
- AC15: Fixture/test validation verifies disagreement sidecars record category, enum severity,
  owner, resolution state, and source locations; unresolved material contradictions either fail
  publication or visibly qualify affected slides.
- AC16: Deck/slide manifests carry a declared information-architecture level, prerequisites, and
  optional drill-down target; composing decks with an incompatible or missing level produce an
  explicit diagnostic.
- AC17: Every theme preset maps the semantic visual grammar's roles (goal, input, evidence,
  decision, tool, verification, output, risk, unresolved-question) consistently to color, edge
  style, and typography, and preserves a non-color/text distinction for each role.

## Deferred Ideas (Later Tracks)

- **Progressive structural lens**: extend the typed topology extractor (`693763fc`) beyond
  Git/Cargo into a stable ladder of levels of detail — repository/submodule graph,
  workspace/package graph, crate/module graph, file skeleton, and symbol signatures — each
  level reusing the same shared node set and adding finer-grained typed edges, with an
  incremental path to additional languages (TypeScript, JavaScript, Python). Explicitly out
  of scope for this track (see R10, R14); recorded here so the idea is not lost, not as an
  active ticket.
- **Executable workflow storyboards**: once end-to-end test sessions exercising all tools as
  intended exist as a data source (per R11), a storyboard extractor can consume a completed
  session's durable workflow graph and produce stages — request, research, planning,
  implementation, validation, review, handoff — as a static Mermaid storyboard, compared
  against the authored "typical session" explanation. Durable session telemetry remains
  illustrative-only per R11 until that data source exists.

## Open Decisions

None. All decisions resolved 2026-07-28.
