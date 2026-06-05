# Goal
Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session handoffs that help a new session resume a specific implementation track quickly.

# Scope
- add root prompt targets for `/handoff` and `/handoff-tickets`
- create canonical rule entries for both generated prompt files
- generate prompt outputs through the existing rule-target workflow
- make `/handoff` produce a short, paragraph-style, reference-centric jumpstart prompt for a specific implementation track
- make `/handoff-tickets` follow the same handoff format and additionally create or match the tickets or tracker tickets needed to formalize the handoff track
- align the prompt guidance with ticket workflow, board awareness, and existing `session-api` session capture behavior

# Non-goals
- changing the existing `session-api` storage model or Stop-hook persistence path
- introducing a new first-class session coordination primitive in this slice
- implementing board automation beyond the current ticket workflow tools

# Acceptance Criteria
1. `rule-targets/30-agents-prompts.yaml` defines targets for `.agents/prompts/handoff.prompt.md` and `.agents/prompts/handoff-tickets.prompt.md`.
2. Canonical rule entries exist for both prompts with `.prompt` metadata and matching path scopes.
3. The generated `/handoff` prompt instructs the agent to return a short, paragraph-style, reference-centric handoff for a specific implementation track.
4. The generated `/handoff-tickets` prompt instructs the agent to produce the same style of handoff and create or match the necessary ticket or tracker ticket follow-up items without duplicating existing tickets.
5. Rule target generation and `--check` validation pass for both outputs.

# Traceability
- Ticket: [46d89aa2 Add handoff workflow prompts](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/46d89aa2-043a-4c94-8213-2f365aa2d517/ticket.toml)
- Parent spec: [96dc0068 workflow guidance generation and session capture scaffolding](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/96dc0068-d05d-4e61-b785-144272119fa9/spec.toml)