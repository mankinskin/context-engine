Tracker for eight workflow/tooling defects reported by the user and verified against repo state on 2026-07-28.

1. **Ticket/spec update semantics** — silent destructive description overwrite (bf62e2f9); empty/no-op spec updates accepted (f986e666).
2. **Handoff step graph** — handoff packages carry no persistent next-step list or graph (f35f4dd9). Updates existing spec 5e52039d.
3. **Duplicate entity creation** — agents create instead of applying deltas (2c019bce); plus an incidental storage bug where `deleted: true` tickets still list (7729df51). Two confirmed duplicate pairs were cancelled during triage: e42d8e0a and 2bb8b3e1.
4. **Interruption recovery** — no guidance or prompt for resuming an abruptly interrupted agent (36b04541).
5. **Online-search Research Agent** — Research Agent lacks a web grant (3df54f79).
6. **MCP cost gate** — `caller_model` strings with a trailing client parenthetical are rejected outright (32067e83); plus stale instruction-doc drift describing the old zero-cost-budget fallback (4ca4ce83).
7. **Board check-in scope** — board check-in claims implementation files only, not a ticket-authoring/topic scope, so two agents can concurrently author the same ticket/epic track undetected (8f999cfb). Concrete incident: duplicate track 84a9f497 + ~10 children cancelled against epic 322a4737.
8. **Research-before-authoring ordering** — ticket authoring currently runs before research, so an implementation-affecting choice (backend crate `printpdf`) was locked into tickets before research showed the risk it addressed was already solved upstream (b7a3c75e).

All 29 open design decisions were resolved with the user in the interview phase and are recorded in each child ticket's description.

Suggested order: 32067e83 first (it currently blocks MCP calls from clients that append a client suffix), then the two store-semantics bugs, then f35f4dd9 before 36b04541, then 8f999cfb and b7a3c75e (process-ordering fixes, low implementation risk, can run any time).