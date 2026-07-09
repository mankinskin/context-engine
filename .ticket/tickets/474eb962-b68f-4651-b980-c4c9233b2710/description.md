# Goal
Plan and execute policy-file and generated guidance updates after the session_audit + schema-version feature checkpoint.

## Scope
- Identify policy/instruction surfaces that should reflect session_audit usage and schema-version expectations.
- Update canonical rule/prompt sources instead of hand-editing generated outputs.
- Regenerate rule-managed outputs and validate no drift.

## Candidate policy surfaces
- .agents/prompts/handoff.prompt.md source rule
- AGENTS.md guidance references for session schema/versioned artifacts
- audit-cli README rule source sections

## Done when
- Required policy/rule sources are updated.
- Regenerated outputs are in sync.
- Follow-up validation captures command evidence and no pre-commit drift remains.
