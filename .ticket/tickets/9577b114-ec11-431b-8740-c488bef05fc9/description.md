# Goal

Update the generated `/handoff` prompt contract so every handoff persists and carries durable session identity into the receiving run.

## Scope

- Update the canonical rule entry that generates `.agents/prompts/handoff.prompt.md`; do not edit generated output directly.
- Require every handoff to include `workspace_session_id`, outgoing `run_id`, structured handoff-record ID, and an exact resume command.
- Require the receiving session to reuse the durable `workspace_session_id` while creating a distinct new `run_id` linked to the outgoing run.
- Include pinned entities, workflow graph status, blockers, required validation state, and finish readiness from the structured handoff record.
- Persist the handoff record before rendering the prompt.
- Update the handoff workflow spec to aligned-structure v2 and link the durable session workflow contract.
- Regenerate rule-managed prompt outputs and validate rule target synchronization.

## Acceptance Criteria

1. `/handoff` always emits the durable workspace session ID and exact resume command.
2. Guidance distinguishes durable workspace identity from per-run capture identity.
3. Prompt rendering consumes a persisted handoff record rather than relying only on conversation summarization.
4. The generated prompt remains compact and reference-centric.
5. Rule synchronization check passes.

## Depends on

- Session core handoff/resume ticket `0647a212-9d2e-4943-9627-f854ce3f14c4`.

## Specs

- `c677182e-90da-4ac3-8b94-9e2e97c825cf`.
- `9e04ff58-9160-4766-b307-74c0fb32a92c`.