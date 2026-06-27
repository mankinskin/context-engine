# Design: session bootstrap contract & rendering redesign plan

Planning/design ticket for the [session-bootstrap] epic. Produces the specs and the resolved decisions.

## Status: decisions resolved
D1–D9 are frozen in the parent spec `memory-api/session-api/dynamic-session-bootstrapping` (8c880efc):
- D1 session identity = agent-carried id; runtime context in the capture session dir.
- D2 cross-store references via URNs `ce://<ws>/<store>/<id>` (hard prerequisite).
- D3 client-side rendering.
- D4 flush per pin; create session dir on init.
- D5 cascade auto-pins hard-linked entities only.
- D6 headers-only rendering; bodies fetched explicitly.
- D7 remove always-on instructions; bootstrapper-only + discoverable/pinned rules.
- D8 no `current_mode`; every session bootstraps (general chat = empty pins).
- D9 usage counting + feedback ratings (generic memory-api model).

## Specs authored
- Parent contract: `memory-api/session-api/dynamic-session-bootstrapping` (8c880efc)
- `memory-api/session-api/runtime-session-context` (709f067a)
- `memory-api/session-api/cascade-context-gathering` (fda5c915)
- `memory-api/session-api/minimal-bootstrapper-selective-loading` (a28a88db)
- `memory-api/curation/entity-usage-and-feedback` (71b81a55)

## Remaining design work
- Freeze the exact URN shape for rule entries (rules attach by scope, not id today).
- Confirm the always-on `applyTo` glob removal is acceptable (D7 risk).
- Decide store consolidation vs cross-store edges for tracking (see epic risk).

## Done when
Specs above reviewed; remaining design work resolved; implementation children unblocked by their cross-store + usage prerequisites.