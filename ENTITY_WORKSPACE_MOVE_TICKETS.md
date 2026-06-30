Found the migration ticket set. The core cross-workspace move track is already centered in the memory-api workspace store and is a strong prep set for effba966 session-bootstrap epic.

Most relevant tickets for moving entities/tickets between workspaces:

- [13e9ce28 [ticket-api] Cross-workspace move + automatic reference re-linking for store entries](memory-api/.ticket/tickets/13e9ce28-ff35-4898-8dda-6d333dc1f222/ticket.toml) (in-review)
- [eb6033a8 [ticket-api] Add move preflight planner and destination-visibility validation](memory-api/.ticket/tickets/eb6033a8-f15b-4024-952e-5c86dc108939/ticket.toml) (new)
- [bc691249 [ticket-api] Add journaled storage-layer execution for cross-workspace ticket moves](memory-api/.ticket/tickets/bc691249-5a2d-409e-8e7b-2602d80cf61e/ticket.toml) (new)
- [3a26572a [ticket-api] Rewrite repo path references that cite the moved ticket folder](memory-api/.ticket/tickets/3a26572a-5e1a-4a57-aefa-9b342886a5ca/ticket.toml) (new)
- [22cd3001 [ticket-api] Enforce board safety and migrate historical board rows during ticket moves](memory-api/.ticket/tickets/22cd3001-0127-4a27-8834-721250ff39ff/ticket.toml) (new)
- [373a3317 [ticket-http] Add ticket move endpoint for workspace relocation](memory-api/.ticket/tickets/373a3317-4dfd-456a-a86e-523f4e7692f0/ticket.toml) (new)
- [53176121 [ticket-cli] Add ticket move CLI with dry-run and recovery guidance](memory-api/.ticket/tickets/53176121-eb55-4aa9-a1d6-5075db1c163b/ticket.toml) (new)
- [84d19fab [ticket-mcp] Expose ticket move planning and execution over MCP](memory-api/.ticket/tickets/84d19fab-9086-4eb2-9d1b-f6bbbae62ceb/ticket.toml) (new)
- [da27c074 [ticket-api] Validate cross-workspace ticket move flows end to end](memory-api/.ticket/tickets/da27c074-8c9e-4613-b8b9-bf02c72b50f7/ticket.toml) (new)
- [dc70628a [ticket-api][spec] Extend workspace-ownership specs with move/relink contract](memory-api/.ticket/tickets/dc70628a-3a84-4441-ad46-f59b7757e1f7/ticket.toml) (in-review)
- [505b2cd4 [ticket-api] Deliver safe cross-workspace ticket move for git-backed stores](memory-api/.ticket/tickets/505b2cd4-f21d-4e8d-8e6a-ae06a5b69854/ticket.toml) (new)
- [44abe1d4 [ticket-vscode] Move ticket 694d74b4 into the memory-api workspace store](memory-api/.ticket/tickets/44abe1d4-5727-45f8-be3b-d1ca5bf4c1ae/ticket.toml) (new)

Also relevant cleanup history in root workspace:

- [d1770bd5 [ticket-store] Relocate misplaced ticket and spec directories](.ticket/tickets/d1770bd5-dc7e-42ca-a5d0-2bc0cbc91110/ticket.toml) (done)

Quick takeaway:
- The implementation track for cross-workspace move is already captured under memory-api/.ticket.
- The explicit “move from context-engine to memory-api” operational ticket is 44abe1d4, which directly matches your cleanup goal.