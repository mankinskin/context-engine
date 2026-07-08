<!-- aligned-structure:v1 -->

# Summary

Deliver one all-Rust autonomous agent harness where a **single minimal operator interface** can, from the **same session model**: 1. start or resume an ad-hoc conversation immediately, 2. promote that session into a persistent long-running autonomous loop, 3. stream lifecycle events, logs, and tool activity in real time, 4. enforce safety and budget gates, 5. recover cleanly after process or client disconnects.

## Behavior Story

Deliver one all-Rust autonomous agent harness where a **single minimal operator interface** can, from the **same session model**: 1. start or resume an ad-hoc conversation immediately, 2. promote that session into a persistent long-running autonomous loop, 3. stream lifecycle events, logs, and tool activity in real time, 4. enforce safety and budget gates, 5. recover cleanly after process or client disconnects.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Unified Agent Harness — Operator Interface Contract

## Goal
Deliver one all-Rust autonomous agent harness where a **single minimal operator
interface** can, from the **same session model**:
1. start or resume an ad-hoc conversation immediately,
2. promote that session into a persistent long-running autonomous loop,
3. stream lifecycle events, logs, and tool activity in real time,
4. enforce safety and budget gates,
5. recover cleanly after process or client disconnects.

The interactive ("chat") and autonomous ("loop") modes are two operating modes
of **one session type**, not two subsystems.

## Problem / Current-State Gap
Prior research (`.ticket/tickets/cba080b5-3c38-495d-8b67-d690b52de4d6`) confirms
there is **no external GitHub Copilot session lifecycle API**. The harness must
therefore **own loop execution in Rust** and treat VS Code / Copilot extension
surfaces as optional, secondary integrations rather than the control plane. The
architecture blueprint lives in `DESIGN_AGENT_HARNESS.md`.

## Scope
- Shared protocol/types crate with versioned, tagged lifecycle event schemas.
- Core ReAct loop runtime with an explicit state machine and a single session
  model that supports both interactive and autonomous modes.
- Provider abstraction (OpenAI/Anthropic/OpenRouter) with budget/policy preflight hooks.
- MCP client integration with per-session tool routing metadata.
- Sandboxed command execution with policy gates.
- Axum streaming server with session lifecycle control and websocket broadcast fanout.
- One minimal operator interface delivered as a native TUI (Ratatui) and a
  browser/WASM (Dioxus) client sharing the same interaction model and semantics.
- Reliability: checkpoint persistence, reconnect continuity, watchdog for stuck/over-budget loops.

## Non-Goals (v1)
- Multi-tenant OS-level isolation beyond per-session working directory + sandboxed
  command execution. v1 assumes a single operator on a single trusted host.
- Building a GitHub Copilot marketplace extension as the primary control surface.
- Distributed/multi-node orchestration or horizontal scale-out of the server.
- A rich visual IDE; the interface is intentionally minimal (session list, live
  stream, mode controls, diff preview).
- Cloud-hosted persistence; v1 persists locally.

## Acceptance Criteria
1. The same UI entrypoint (TUI and WASM) can start an ad-hoc conversation and
   promote it to a long-running loop without changing session identity.
2. Session state remains authoritative and correct across client disconnect /
   reconnect and across client type changes (TUI <-> browser).
3. Lifecycle events, logs, and tool/command activity stream in real time to all
   connected observers of a session via broadcast fanout.
4. Budget, policy, and sandbox gates are enforced before expensive/irreversible
   actions and are auditable (tool calls, command invocations, exits, artifacts).
5. A loop can be paused, resumed, and stopped; checkpoints allow recovery and
   postmortem replay after a process restart.
6. Browser verification of the WASM client passes in an external Chromium-family
   browser at a documented resolution, with Playwright coverage and screenshots
   for transient UI states (diff preview, mode-switch, live stream).

## Traceability
- Epic / tracker: `.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8`
- Research prerequisite (done): `.ticket/tickets/cba080b5-3c38-495d-8b67-d690b52de4d6`
- Design blueprint: `DESIGN_AGENT_HARNESS.md`
- Child implementation tickets (CH1–CH12) are linked from the epic and each depend
  on this spec's acceptance criteria for their definition of done.
- Validation evidence: per-child validation matrix recorded on each ticket; loop /
  server test logs land under `target/test-logs/`; browser evidence via Playwright
  screenshots.
