# Context-enrichment workflow for review tickets: session-ticket relation query

## Objective

39 tickets sit in `in-review` today, on unrelated topics, with no reliable way to reconstruct
"what was actually done and by which sessions" before closing each one. This spec locks the
design for the first step of a reusable context-enrichment workflow: querying the Session API
for sessions related to a given ticket, so that step can seed context reconstruction, review,
and eventual closure of in-review tickets.

Source interview: `transcripts/04-08-2026_context-enrichment-workflow/input.clean.md`.

## Background / current state

- `SessionQuery` (`memory-api/crates/session-api/src/store.rs#L92-L104`) filters only on
  `session_id_prefix`, `conversation_id`, `agent_id`, `text`, `limit`. There is no ticket filter.
- `session_matches_query` (`memory-api/crates/session-api/src/store/helpers/storage.rs#L438-L480`)
  does not consult `SessionMetadata.ticket_id` or `SessionLinks.ticket_ids` at all.
- The data model already carries the relation:
  - `SessionMetadata.ticket_id: Option<String>` (`memory-api/crates/session-api/src/model.rs#L292-L306`)
    — the ticket a session formally checked in against.
  - `SessionLinks.ticket_ids: Vec<String>` (`memory-api/crates/session-api/src/model/links.rs#L7-L15`)
    — tickets explicitly linked to a session.
- This is a query-surface gap, not a data-model gap.
- No existing ticket or spec covered "context enrichment" or "review closure" before this spec.
- `.agents/agents/review.agent.md` explicitly drops the `session-mcp` tool grant and so cannot
  reconstruct context from session history today.
- `.agents/agents/iteration.agent.md` has `session-mcp` but drives the current session's own
  workflow, not arbitrary-ticket history reconstruction.
- Only `.ticket`, `.spec`, `.session`, `.test`, `.feedback`, `.benchmark` exist as dot-directory
  stores today. There is no `.rule`, `.doc`, or `.log` store, so this spec does not assume they
  exist.

## Locked decisions

1. **Session scope for this workflow slice**: query capability + one dogfood run + a new agent
   template (all three; see linked tickets).
2. **Relation-strength semantics**: three selectable tiers, caller picks a minimum tier:
   - `strict` — `SessionMetadata.ticket_id == ticket_id` only.
   - `linked` — `strict` OR ticket id present in `SessionLinks.ticket_ids`.
   - `mentioned` — `linked` OR ticket id present in the session's handoff package
     `target_tickets`.
   - Bare substring scanning of `transcript.json` is explicitly excluded at every tier — too
     noisy.
3. **API shape**: a separate dedicated entry point, `sessions_for_ticket(ticket_id, strength)`,
   not a new field on `SessionQuery`. Keeps the existing query struct unchanged and lets the
   result type carry richer per-session relation metadata.
4. **Result payload**: each row carries session id, agent id, start/end timestamps, git branch,
   worktree path, and which relation-strength signal matched. The full handoff package body is
   not inlined — callers fetch it separately if needed.
5. **Inverse direction** (Ticket API querying "which sessions worked on me"): explicitly out of
   scope for this spec's tickets, deferred to a follow-up ticket
   (`.ticket/tickets/1ff57502-ad4e-4c40-a852-18752c18f44c/ticket.toml`).
6. **Agent template**: a new, dedicated `.agents/agents/context-enrichment.agent.md`, not an
   extension of `review.agent.md`. Keeps "guide a human reviewer" and "autonomously reconstruct
   and close" as separate contracts.
7. **Dogfood target**: ticket `06cfe998` ([token-efficiency] Introduce peek-api with peek-cli and
   peek-mcp transport layers), currently `in-review`.
8. **Closure authority**: autonomous. When the workflow concludes a ticket's acceptance criteria
   are met (with recorded evidence), it may transition the ticket to `done`/`accepted` without a
   separate human-confirmation step.

## Non-goals

- The Ticket API inverse query is not implemented by this spec's tickets (see decision 5).
- Querying `.rule`, `.doc`, or `.log` stores is out of scope — those stores do not exist on disk
  today.
- Bare transcript-text matching as a relation signal is explicitly rejected, not deferred.

## Related tickets

- `.ticket/tickets/bba9b313-ff13-4fd1-91d4-6485a6c2f4de/ticket.toml` — Session API
  `sessions_for_ticket` query capability (api + cli + mcp).
- `.ticket/tickets/33463861-ffba-4ead-905e-5d867b707936/ticket.toml` — dogfood run against
  ticket `06cfe998`.
- `.ticket/tickets/7c74f2fe-2bfd-477c-847e-bc02200a4819/ticket.toml` — new
  `context-enrichment.agent.md` template.
- `.ticket/tickets/1ff57502-ad4e-4c40-a852-18752c18f44c/ticket.toml` — deferred Ticket API
  inverse query (backlog).
