<!-- aligned-structure:v2 -->

# Summary

Define generated `/handoff` and `/handoff-tickets` prompts that persist and render compact, reference-centric handoffs for durable session workspaces.

## Motivation ("why")

Conversation summaries alone do not provide stable session identity or an authoritative restart point. Every handoff must carry the durable workspace, run lineage, roadmap status, and exact resume operation without replaying raw transcripts.

## Dependent expectation

If this spec is implemented, dependents can rely on `/handoff` persisting a structured handoff record before rendering; always emitting `workspace_session_id`, outgoing `run_id`, handoff ID, and exact resume command; and instructing the receiver to reuse the workspace ID with a new linked run ID.

## Guards

- `val-session-handoff-continuity`.
- `val-handoff-prompt-rule-sync`.
- `val-handoff-reference-completeness`.

## Positions

- Existing generated handoff prompt: `implemented` at `.agents/prompts/handoff.prompt.md`; requires durable session identity and persisted handoff input via canonical rule `084fd4e6-660b-4227-a13e-514edf44e393`.
- Existing generated handoff-tickets prompt: `partial` at `.agents/prompts/handoff-tickets.prompt.md`.
- Durable handoff core: `implemented` at `memory-api/crates/session-api/src/store.rs`; validated by `exec-val-session-handoff-continuity-20260714`.
- Prompt contract update: `implemented` in `.rule/rules/084fd4e6-660b-4227-a13e-514edf44e393/body.md` and generated `.agents/prompts/handoff.prompt.md`; validated by `exec-val-handoff-prompt-rule-sync-20260714` and `exec-val-handoff-reference-completeness-20260714`.

## Governing-rule requirement

Generated prompt changes are governed by canonical rule entries and `rule-targets/30-agents-prompts.yaml`; generated files are never edited directly.

# Contract

- Handoff persists a structured record before rendering prompt text.
- Every output includes durable workspace ID, outgoing run ID, handoff-record ID, exact resume command, pinned entities, workflow status, blockers, and required validation state.
- The receiver reuses `workspace_session_id` and creates a distinct new `run_id` linked to the outgoing run.
- Raw transcript content remains a pointer unless a finding cannot be represented durably.
- `/handoff-tickets` follows the same identity/resume contract and creates or matches durable ticket work without duplication.
- Outputs remain compact, reference-centric, and compliant with clickable reference policy.

# Non-goals

- Implementing session persistence in generated prompt text.
- Reusing one capture run ID across multiple agent executions.
- Replaying full transcripts by default.

# Acceptance Criteria

1. Both generated prompts are sourced from canonical rule entries and pass synchronization checks.
2. `/handoff` always contains the durable ID and exact resume command from a persisted handoff record.
3. Receiver instructions create a new linked run rather than reusing the outgoing run ID.
4. Roadmap status, blockers, pins, and validation state are represented compactly.
5. All entity references resolve and no placeholder identity remains undeclared.

# Traceability

- Durable workflow spec: `c677182e-90da-4ac3-8b94-9e2e97c825cf`.
- Core handoff ticket: `0647a212-9d2e-4943-9627-f854ce3f14c4`.
- Prompt update ticket: `9577b114-ec11-431b-8740-c488bef05fc9`.
- Original generated prompt ticket: `46d89aa2-043a-4c94-8213-2f365aa2d517`.
