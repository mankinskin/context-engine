Adopt the new worktree-backed session path in repository guidance and hooks after the planning contract and `session-api` assignment surfaces are in place.

# Scope
- update instructions, prompts, and startup guidance so new implementation sessions must obtain a session-backed worktree assignment before board check-in or code changes
- update hook or startup helper behavior to consume the authoritative working directory returned by `session-api` instead of assuming the root checkout
- document how revival and handoff behave under the new contract, including when a worktree is reused versus rotated
- ensure stop/handoff capture guidance reports session and worktree context without implicitly reassigning ownership

# Out of Scope
- redefining the session/worktree contract from the planning slice
- changing the allocator behavior owned by `session-api`

# Acceptance Criteria
1. Repository guidance tells users and agents to resolve a session-backed working directory before implementation work, then perform draftboard check-in/file claims from that assigned worktree.
2. Startup helpers or hooks consume the authoritative worktree path from `session-api` rather than deriving it from root-checkout assumptions.
3. Guidance distinguishes same-session reuse from rotated follow-up sessions and documents rotation as the default cross-session handoff path.
4. Stop or handoff hooks capture or announce session/worktree context without mutating worktree ownership or draftboard ownership.
5. Focused validation proves the guidance or hook path points new sessions at the assigned worktree and stays aligned with the governing spec.

# Traceability
- Governing spec: [default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/context-engine/session-worktree-default-workflow/spec.toml)
- Planning prerequisite: [68a49ca7 Plan default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/68a49ca7-a6f6-42a8-b820-0a86e6a4de2e/ticket.toml)
- Infrastructure prerequisite: [e2189e9d Implement session check-in and worktree assignment surfaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml)