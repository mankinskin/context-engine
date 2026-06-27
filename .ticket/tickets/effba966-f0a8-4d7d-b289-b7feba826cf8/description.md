# Umbrella: Dynamic Session Bootstrapping & Context Routing

Tracking ticket for redesigning the agent workflow from always-on static metacognition into just-in-time, session-scoped context curation. Source: `DESIGN_SESSION_BOOTSTRAPPING.md`. Contract: spec `memory-api/session-api/dynamic-session-bootstrapping` (8c880efc).

## Resolved decisions
D1 agent-carried session id + resume · D2 cross-store URN references · D3 client-side rendering · D4 flush per pin + create dir on init · D5 hard-linked auto-pin only · D6 headers-only rendering · D7 remove always-on instructions (bootstrapper-only) · D8 no mode · D9 usage counting + feedback.

## Sequenced roadmap (prerequisites first)
1. **Cross-store references (robust) — BEFORE bootstrapping.**
   - default store: [82d6ada4 URN resolver], [6bd67a7a multi-store discovery] (tracker [671d4e47]).
   - memory-api store (hard-link source, textual dep): b03be2d5 cross-entity edges; f00291a3 ticket↔spec integration.
2. **Curation core (usage + feedback) — BEFORE bootstrapping.**
   - [f8b447b7 generic entity usage + feedback] (this store); rule feedback done; spec feedback 29bf9628 (memory-api store).
3. **Design & contract** — afa00b5c (in progress).
4. **session-api runtime model** — 412964a3 (depends on URN resolver + usage core).
5. **Cascade context gathering** — d8f76965 (depends on URN resolver + discovery).
6. **CLI + MCP surfaces** — 6b2dc497 (depends on runtime + cascade + usage core).
7. **Rule rendering redesign** — b4a8dc5e (depends on CLI/MCP).

## Children
afa00b5c · 412964a3 · d8f76965 · 6b2dc497 · b4a8dc5e (epic closes when all reach done).

## Key risk — ticket-store boundary
The session-bootstrap tickets live in the **default** `.ticket` store; the cross-entity-edge and spec-feedback prerequisites live in the **memory-api** `.ticket` store. `add_edge` resolves both endpoints within one workspace, so those cross-store prerequisites **cannot be graph-edged** and are tracked textually until the URN cross-store reference model (item 1) ships. This redesign is itself a primary consumer of that capability.