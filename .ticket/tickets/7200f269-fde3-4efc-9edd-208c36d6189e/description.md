## Problem

`explore.agent.md` now serves two roles: the general-purpose exploration
template, and the mandated pre-dispatch quality-gate agent (per
`46d8b25d`'s pre-dispatch-gates workflow). There is no explicit signal in
the dispatch prompt that lets the dispatched agent distinguish:

- "gate mode" — a pre-dispatch quality-gate check subject to the 5-turn /
  10-tool-call ceiling, from
- "ordinary exploration" — an open-ended research/discovery dispatch with
  no such ceiling.

Without an explicit marker, a dispatched agent has no reliable way to know
which behavioral contract applies, and an orchestrator has no cheap way to
audit (from the prompt text alone) whether a given dispatch was intended as
a gate check.

## Objective

Add a small, explicit marker convention to the dispatch-prompt contract so
gate-mode dispatches are unambiguously distinguishable from ordinary
exploration dispatches at prompt-construction time.

## Acceptance Criteria

1. A marker convention is defined (e.g. a required `GATE CHECK:` prefix on
   the dispatch prompt, or an equivalent explicit field/flag) that signals
   gate-mode to the dispatched agent.
2. The convention is documented in the relevant instruction/agent template
   (e.g. `pre-dispatch-gates.instructions.md` and/or `explore.agent.md`)
   so orchestrators reliably emit it for every pre-dispatch gate check.
3. The dispatched agent's behavior (turn/tool-call ceiling awareness) is
   made conditional on the marker being present, so ordinary exploration
   dispatches are not implicitly held to the gate ceiling.

## Traceability

- From: [46d8b25d Move quality gates before dispatch](.ticket/tickets/46d8b25d-e80c-4170-9601-1c26a7a0bcb8/ticket.toml) re-review — identified the missing gate/exploration distinction.
