Turn `context-engine/session-worktree-default-workflow` (`2860a8db-0c4e-4e94-984a-c10a72a67ffc`) into the concrete planning contract for the default worktree-backed session path.

# Scope
- define the required startup order for implementation sessions: session check-in -> authoritative worktree assignment -> board check-in/file claims -> implementation -> stop or handoff capture
- define session ownership of worktree assignments, including the metadata that downstream slices rely on
- define when worktrees are reused versus rotated for resume, revival, handoff, or invalidated worktree cases
- define responsibility boundaries across `session-api`, the ticket draftboard, and hooks/guidance so later implementation tickets do not guess
- lock the rollout order so `e2189e9d` implements the assignment surfaces before `326bfe38` adopts them in guidance and hooks

# Out of Scope
- implementing the `session-api` surfaces themselves
- landing repository guidance or hook changes beyond what is needed to specify the contract

# Acceptance Criteria
1. The governing spec records the authoritative startup order and names the worktree assignment returned by `session-api` as the only supported working directory for new sessions.
2. The planning slice defines the session/worktree ownership model, including required metadata for path, branch, allocation mode, and predecessor lineage when rotation happens.
3. The planning slice defines a reuse-versus-rotation decision table that covers resume, revival, cross-session handoff, and worktree invariant failures, with rotation as the default handoff path.
4. The planning slice defines the authority boundary: `session-api` owns session identity and worktree lifecycle, the draftboard owns ticket/file coordination, and hooks only read or announce state unless an explicit future adopt flow is implemented.
5. The planning slice leaves `e2189e9d` as the next implementation ticket and `326bfe38` as the follow-on guidance rollout, both linked back to the same spec and prior research.

# Traceability
- Governing spec: `2860a8db-0c4e-4e94-984a-c10a72a67ffc`
- Next implementation: [e2189e9d Implement session check-in and worktree assignment surfaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml)
- Follow-on rollout: [326bfe38 Add worktree-first session guidance and hooks](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/326bfe38-6f5e-4000-9ffc-e5be0839194f/ticket.toml)
- Related research: [09b68366 Multi-agent coordination and cross-agent communication protocols](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/09b68366-486e-4e39-a610-1d14676368aa/ticket.toml)
- Related completed capture slice: [e663f9e9 Wire VS Code Copilot stop-hook session capture](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e663f9e9-ac52-4c0e-8e07-d17c8a15b48d/ticket.toml)
- Superseded implementation context: [51471c3e Sandbox Manager -- per-assignment worktree and branch isolation](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/51471c3e-a088-47d4-9922-ba49d914af17/ticket.toml)