Track the migration to a default worktree-backed session workflow so parallel agent sessions no longer share one staging area.

# Ordered Rollout
1. [68a49ca7 Plan default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/68a49ca7-a6f6-42a8-b820-0a86e6a4de2e/ticket.toml) locks the contract for session ownership, worktree reuse versus rotation, and board/session/hook responsibilities.
2. [e2189e9d Implement session check-in and worktree assignment surfaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml) makes `session-api` return the authoritative worktree path and lifecycle metadata.
3. [326bfe38 Add worktree-first session guidance and hooks](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/326bfe38-6f5e-4000-9ffc-e5be0839194f/ticket.toml) rolls the new path into default guidance and hook behavior.

# Done Condition
This tracker closes only after all three slices are complete, each links back to spec `2860a8db-0c4e-4e94-984a-c10a72a67ffc`, and the repository has one coherent default path from session check-in through worktree assignment to guidance rollout.