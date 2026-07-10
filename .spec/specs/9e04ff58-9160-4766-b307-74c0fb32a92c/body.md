<!-- aligned-structure:v1 -->

# Summary

Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session handoffs that help a new session resume a specific implementation track quickly.

## Behavior Story

Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session handoffs that help a new session resume a specific implementation track quickly.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.
- [.ticket/tickets/1400919a-84b9-49ff-8e8a-92a7d9068594/ticket.toml](.ticket/tickets/1400919a-84b9-49ff-8e8a-92a7d9068594/ticket.toml)
- [.ticket/tickets/b6cdc89d-30fc-4303-aaba-f959abfeda4b/ticket.toml](.ticket/tickets/b6cdc89d-30fc-4303-aaba-f959abfeda4b/ticket.toml)
- [.ticket/tickets/7769da57-a8f6-4e72-a860-c8263d5a360e/ticket.toml](.ticket/tickets/7769da57-a8f6-4e72-a860-c8263d5a360e/ticket.toml)
- [.ticket/tickets/c851f3af-433a-496e-a586-28631de142ce/ticket.toml](.ticket/tickets/c851f3af-433a-496e-a586-28631de142ce/ticket.toml)

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Goal
Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session handoffs that help a new session resume a specific implementation track quickly.

# Scope
- add root prompt targets for `/handoff` and `/handoff-tickets`
- create canonical rule entries for both generated prompt files
- generate prompt outputs through the existing rule-target workflow
- make `/handoff` produce a short, paragraph-style, reference-centric jumpstart prompt for a specific implementation track
- make `/handoff` carry high-value current-session context, including findings, decisions, blockers, suggested next steps, entity references, and first validation checks, while avoiding generic workflow noise the next session can retrieve from referenced instructions
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
4. The generated `/handoff` prompt requires current-session findings, decisions, blockers, suggested next steps, entity references, and first validation checks when available, and explicitly suppresses generic workflow noise that the next session can retrieve from referenced instructions.
5. The generated `/handoff-tickets` prompt instructs the agent to produce the same style of handoff and create or match the necessary ticket or tracker ticket follow-up items without duplicating existing tickets.
6. Rule target generation and `--check` validation pass for both outputs.

# Traceability
- Ticket: [46d89aa2 Add handoff workflow prompts](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/46d89aa2-043a-4c94-8213-2f365aa2d517/ticket.toml)
- Parent spec: [96dc0068 workflow guidance generation and session capture scaffolding](C:/Users/linus/git/graph_app/context-engine/.spec/specs/96dc0068-d05d-4e61-b785-144272119fa9/spec.toml)

# Validation
- `cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets/30-agents-prompts.yaml --check` verifies the generated prompt outputs remain synchronized with their canonical rule entries.
