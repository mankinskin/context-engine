## Problem
Static `applyTo`-glob classing (see ticket 9cd886d5) covers the near-term case: a fixed set of always-on general rules plus domain-scoped specialized files. It does not cover selecting guidance dynamically at dispatch time by ticket/context tag (e.g. "this ticket touches frontend + rust" -> inject only the relevant instruction subset, weighted by relevance, without hand-maintaining glob lists).

## Goal
Design and implement a runtime mechanism that selects and injects guidance dynamically by tag/context rather than via static `applyTo` globs. This is the tooling-driven counterpart to 9cd886d5's static convention.

## Explicit dependency
This ticket is blocked on future rule-api deployment — the runtime guidance-selection mechanism does not exist yet. Do not start implementation until rule-api is available; use this ticket to track design/requirements in the meantime.

## Track placement
- General/future track. NOT a child of epic 7e8bc1c3 ([epic] Guidance corpus quick-win track) and does NOT block Epic A.
- Linked (non-blocking `linked`, not `depends_on`) to 9cd886d5 as its static predecessor: the static applyTo-class scheme in 9cd886d5 should land first as the near-term win, but this ticket is not gated on it.

## Acceptance criteria (draft, refine once rule-api lands)
- A design note documents how tag/context is extracted at dispatch time and mapped to a guidance subset.
- The mechanism can select a strict subset of `.agents/instructions/**` at runtime without requiring every file to declare its own `applyTo` glob.
- rule-api integration point is named and the ticket records the blocking dependency explicitly (not just informally).

## Source
Split out of "Scoped/dynamic guidance injection by ticket domain tag" (ticket 9cd886d5) during a scoping interview on 2026-07-31. The interview separated the near-term static `applyTo`-glob convention (kept in 9cd886d5) from this deferred runtime/tooling-driven mechanism.
