Tracker for six workflow/tooling defects reported by the user and verified against repo state on 2026-07-28.

1. **Ticket/spec update semantics** — silent destructive description overwrite (bf62e2f9); empty/no-op spec updates accepted (f986e666).
2. **Handoff step graph** — handoff packages carry no persistent next-step list or graph (f35f4dd9). Updates existing spec 5e52039d.
3. **Duplicate entity creation** — agents create instead of applying deltas (2c019bce); plus an incidental storage bug where `deleted: true` tickets still list (7729df51). Two confirmed duplicate pairs were cancelled during triage: e42d8e0a and 2bb8b3e1.
4. **Interruption recovery** — no guidance or prompt for resuming an abruptly interrupted agent (36b04541).
5. **Online-search Research Agent** — Research Agent lacks a web grant (3df54f79).
6. **MCP cost gate** — `caller_model` strings with a trailing client parenthetical are rejected outright (32067e83).

All 29 open design decisions were resolved with the user in the interview phase and are recorded in each child ticket's description.

Suggested order: 32067e83 first (it currently blocks MCP calls from clients that append a client suffix), then the two store-semantics bugs, then f35f4dd9 before 36b04541.