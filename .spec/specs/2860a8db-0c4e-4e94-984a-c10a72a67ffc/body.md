# Goal
Make dedicated git worktrees the default workflow for new agent sessions in this repository so parallel implementation tracks do not share one staging area.

# Scope
- require new sessions to check into the session tool before implementation begins
- assign each checked-in session an authoritative worktree working directory
- expose that working directory through targeted `session-api` CLI, MCP, hook, or startup surfaces
- define how ticket-board ownership, session state, and worktree lifecycle interact
- define revival and handoff behavior for reusing or rotating session-owned worktrees
- update workflow guidance and hooks so the worktree-first path becomes the repository default

# Workflow Contract
1. **Session check-in comes first.** A new implementation session must obtain a session record before code changes, board file claims, or startup guidance treats the session as active work. The session record carries the owner identity, ticket context, and worktree assignment status.
2. **Worktree assignment is authoritative in `session-api`.** The assigned working directory returned by session check-in or resume is the source of truth for CLI startup, MCP startup, hooks, and guidance. Other tools may reference the path, but they must not derive or replace it independently.
3. **Board coordination begins after worktree assignment.** The draftboard remains authoritative for ticket activity and file ownership. `session-api` remains authoritative for session identity, worktree metadata, and revival history. Hooks may read both stores for reminders or evidence capture, but they must not silently allocate worktrees or transfer board ownership on their own.
4. **Default rollout order is fixed.** Planning defines the contract first, `session-api` implements the worktree assignment surfaces second, then repository guidance and hooks adopt the new default startup path. The tracker closes only after all three slices are linked back to this spec.

# Ownership and Lifecycle
- One active session owns exactly one active worktree assignment at a time.
- Worktree metadata must record at least the assigned path, branch, allocation mode (`new`, `reused`, or `rotated`), and predecessor reference when rotation occurs.
- Multiple sessions may target the same ticket over time, but they must not silently share one active worktree assignment.
- Session stop hooks capture evidence and transcript state, but they do not reassign worktree ownership by themselves.

# Reuse vs Rotation Contract
- **Reuse** the existing worktree when the same session, or an explicit revival of that same owner/session lineage, resumes and the recorded path and branch invariants still hold.
- **Rotate** to a new worktree when a fresh session starts after handoff, when the previous worktree is missing or fails invariants, or when another active session still owns the prior assignment.
- **Default handoff behavior is rotation.** Cross-session or cross-owner reuse requires an explicit adopt flow with validation; it is not the silent startup default.

# Non-goals
- reviving the cancelled AOH sandbox-manager implementation as-is
- changing the existing append-only `session-api` transcript persistence contract
- designing full merge automation or PR orchestration in this slice

# Acceptance Criteria
1. This spec defines the mandatory startup order: session check-in, authoritative worktree assignment, board check-in or file claims, implementation, then stop or handoff capture.
2. This spec defines the ownership boundary between `session-api`, the ticket draftboard, and repository hooks or guidance so each surface has one clear authority.
3. This spec defines reuse vs rotation rules for resume, revival, handoff, and invalid worktree recovery, including rotation as the default handoff path.
4. The `session-api` implementation slice is required to return the authoritative working directory plus allocation metadata and to validate the reuse or rotation rules with focused tests.
5. The guidance or hook slice is required to consume the assigned worktree path, explain the board interaction order, and document stop or handoff expectations without reassigning ownership implicitly.
6. The planning slice links the existing related research and completed session capture work so implementation can reuse them instead of re-deciding the model.

# Traceability
- Tracker: [b6af9f40 Default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/b6af9f40-e1f7-4f68-92e7-0a063a4ac020/ticket.toml)
- Planning: [68a49ca7 Plan default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/68a49ca7-a6f6-42a8-b820-0a86e6a4de2e/ticket.toml)
- Infrastructure: [e2189e9d Implement session check-in and worktree assignment surfaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml)
- Guidance: [326bfe38 Add worktree-first session guidance and hooks](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/326bfe38-6f5e-4000-9ffc-e5be0839194f/ticket.toml)
- Related completed work: [e663f9e9 Wire VS Code Copilot stop-hook session capture](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e663f9e9-ac52-4c0e-8e07-d17c8a15b48d/ticket.toml)
- Related research: [09b68366 Multi-agent coordination and cross-agent communication protocols](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/09b68366-486e-4e39-a610-1d14676368aa/ticket.toml)
- Superseded implementation context: [51471c3e Sandbox Manager -- per-assignment worktree and branch isolation](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/51471c3e-a088-47d4-9922-ba49d914af17/ticket.toml)