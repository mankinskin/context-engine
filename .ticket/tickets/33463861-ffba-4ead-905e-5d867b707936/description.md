## Objective

Run a manual dogfood pass of `sessions_for_ticket` against ticket `06cfe998` ([token-efficiency] Introduce peek-api with peek-cli and peek-mcp transport layers, currently `in-review`), then record findings and any workflow improvements needed.

## Depends on

Ticket bba9b313 ([session-api][session-cli][session-mcp] Add sessions_for_ticket query with selectable relation-strength tiers) must ship first — this ticket exercises that capability, it does not implement it.

## Locked design decisions (from interview 04-08-2026)

- Dogfood target ticket: `06cfe998`.
- Run the query at all three relation-strength tiers (`strict`/`linked`/`mentioned`) against `06cfe998` and compare the resulting session sets.
- Closure authority: this dogfood run MAY conclude and record that `06cfe998`'s acceptance criteria are met and transition it toward `done`/`accepted` autonomously if the reconstructed context supports it — per the interview's "autonomous closure allowed" decision. It is not required to stop for human confirmation before the state transition.

## Acceptance Criteria

1. `sessions_for_ticket("06cfe998", tier)` is run for all three tiers and the resulting session id lists (with matched signal) for each tier are recorded in this ticket's description or a linked artifact.
2. The recorded session results are used to reconstruct 06cfe998's implementation history (what was done, what remains) well enough to state, with evidence pointers, whether its acceptance criteria are satisfied.
3. Based on that reconstruction, 06cfe998 is transitioned to `done`/`accepted` (if criteria are met) or back to `open`/`ready` (if not), consistent with the "autonomous closure allowed" decision — no human confirmation gate is required before this transition.
4. Any gap found in the dogfood run (missing signal, noisy result, missing session data) is recorded as a follow-up finding in this ticket, distinguishing "workflow needs improvement" from "06cfe998 itself needs more work".