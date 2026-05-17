### next_tickets board-aware signals

`next_tickets` is board-aware, but it does not return a full board snapshot. Use
`board_show` when a client needs board load, stale counts, or the complete board state.
