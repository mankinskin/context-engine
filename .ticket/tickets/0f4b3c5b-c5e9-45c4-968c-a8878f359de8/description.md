# Unified Agent Harness Tracker

## Why this ticket
One cohesive delivery plan that turns existing research and design direction into
an executable implementation roadmap for an all-Rust autonomous agent harness with
a single minimal operator interface supporting both on-demand chat and long-running
supervised loops from the same session model.

## Refinement status (2026-07-08)
Refinement pass complete. Requirements clarified, decisions locked, child tickets
sliced with acceptance criteria + validation matrices, dependency DAG wired, and the
owning spec created. Ready for implementation handoff.

## Owning spec
- `agent-harness/unified-operator-interface` — id `6f286eee-115b-4476-a6fc-9bd6796b5f3f`
  (`.spec/specs/6f286eee-115b-4476-a6fc-9bd6796b5f3f`).
- Spec carries: goal, scope, non-goals, acceptance criteria, resolved decisions
  (D1–D5), validation design, and the refinement interview questionnaire.

## Prior context (inputs)
- Research baseline (done): `cba080b5-3c38-495d-8b67-d690b52de4d6` — confirms **no
  external Copilot session lifecycle API**; the harness must own loop execution in
  Rust and treat VS Code / Copilot extension surfaces as optional/secondary.
- Design blueprint: `DESIGN_AGENT_HARNESS.md`.

## Outcome statement
The same UI entrypoint (TUI + WASM) can: (1) start/resume an ad-hoc conversation,
(2) promote it into a persistent long-running loop, (3) stream lifecycle events/logs/
tool activity in real time, (4) enforce safety/budget gates, (5) recover cleanly after
process or client disconnects.

## Interview outcomes (assumptions of record)
No live stakeholder was available; the following were assumed from repo conventions +
blueprint and are captured in the spec `refinement-interview` and
`resolved-decisions-and-assumptions` sections. Operator may overturn; changes must
update the spec and affected children.
- Mode transitions: promotion chat->loop keeps the same session id + history; a loop
  can be paused for interactive turns and resumed from checkpoint.
- Safety/budget: 200k token default per loop (hard stop + resumable checkpoint); all
  host commands sandboxed (Docker/bollard), deny-by-default outside the session dir;
  max 4 concurrent loops (configurable).
- Observability/recovery: append-only NDJSON events + periodic checkpoints survive
  crashes; audit covers tool calls, command invocations, exits, artifacts, budget/policy
  decisions, correlated by session + tool-call id.
- UX: TUI and WASM share the same control set (start / toggle-loop / pause / resume /
  stop / inspect / diff) and session semantics; browser verification in external
  Chromium at a documented resolution with Playwright screenshots for transient UI.

## Resolved open questions (was: open questions)
1. **Crate paths/naming** (D1): top-level `agent-harness/` with `agent-shared`,
   `agent-core`, `agent-server`, `agent-tui`, `agent-web/frontend/dioxus`,
   `tools/cli/agent-cli`; registered in root `Cargo.toml`.
2. **Checkpoint/event persistence** (D2): append-only NDJSON per-session log +
   materialized index + periodic checkpoints; SQLite deferred.
3. **Isolation** (D3): per-session working dir + Docker sandbox (bollard); multi-tenant
   OS-level isolation is a v1 non-goal; backend is pluggable.
4. **Concurrency/budgets** (D4): max 4 concurrent loops; 200k token default; per-iteration
   caps; all configurable.
5. **`.agentguidance` precedence** (D5): deterministic layering repo-root < path-scoped <
   session/task; most-specific wins; equal specificity -> newer/explicit wins; overrides audited.

## Scope and Workstreams
- **WS1** Workspace + contract foundation (agent-shared, versioned tagged serde events).
- **WS2** Core loop runtime (state machine, unified session/mode model, provider abstraction, hooks, guidance).
- **WS3** Tooling + sandbox safety (rmcp integration, per-session routing, sandboxed exec, audit).
- **WS4** Streaming server (Axum lifecycle, websocket broadcast fanout, resumability, CORS).
- **WS5** Unified minimal interface (Ratatui + Dioxus/WASM parity, diff preview via `similar`).
- **WS6** Reliability/observability/persistence (checkpoints, reconnect, watchdog, correlated tracing).
- **WS7** Validation + release readiness (unit/integration/e2e, Playwright, browser evidence, docs).

## Child tickets (refined, with dependency DAG)
| Child | Ticket ID | Depends on |
|---|---|---|
| CH1 Workspace + shared contracts | `a5f08931-24af-4b96-a156-9107c776f946` | — |
| CH2 Core loop + session/mode model | `c684b092-7f5a-4ebe-aa6d-494f666f5dc8` | CH1 |
| CH3 Provider + guidance + budget hooks | `036c270f-6ca7-4372-96e2-570a26e3fdd0` | CH2 |
| CH4 MCP integration + tool routing | `1c63db9d-afb3-4678-b0f6-14e6a4d5daca` | CH2 |
| CH5 Sandboxed exec + policy gates | `136af497-869b-4cc5-b059-9041a98e5ad3` | CH4 |
| CH6 Axum lifecycle + broadcast | `8ed0edbf-a765-4f4a-b50e-695aa79e9180` | CH1, CH2 |
| CH7 Ratatui interface | `3c208991-1d98-4a9c-be29-890d15244b8d` | CH6 |
| CH8 Dioxus/WASM parity | `86f95ad8-8d61-43b6-a463-8719b29007c0` | CH6 |
| CH9 Diff preview both clients | `a496cad3-cdc5-4237-b432-47a6bb43b9c5` | CH7, CH8 |
| CH10 Reliability/recovery | `fd93671d-2a86-4996-9d26-efcfce156095` | CH6 |
| CH11 E2E + Playwright + browser | `b01a2fbf-6682-4dee-abce-95cdcf4fd325` | CH7, CH8, CH9, CH10 |
| CH12 Docs/runbooks + rollout | `a4273210-ef12-4372-bd30-6e112c9d708e` | CH11 |

Each child carries a four-part validation matrix (fast check, primary automated gate,
manual/browser evidence, failure log path) in its description and `validation_plan` field.

## Sequencing
- CH1 first. CH2 then unblocks CH3/CH4/CH6 in parallel. CH5 after CH4.
- UI (CH7/CH8) after CH6; CH9 after both; CH10 after CH6.
- CH11 is the release gate over CH7–CH10; CH12 documents after CH11.
- This tracker depends_on all twelve children and converges only when they complete.

## Definition of Done
- Single minimal UI flow supports immediate chat and long-running loop management for
  the same session; state correct across reconnects and client-type changes.
- Policy/budget/sandbox controls enforced and auditable.
- Required tests pass; validation evidence + commands recorded on child updates.
- Spec acceptance criteria satisfied; docs/runbooks updated (CH12).