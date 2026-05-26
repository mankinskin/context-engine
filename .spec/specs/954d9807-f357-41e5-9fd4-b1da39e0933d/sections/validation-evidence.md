<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=151d6e46-84ae-4c52-86d7-f8ef96b1e922 slug=context-engine/recurring-principles/validation-evidence/spec-linked-validation-evidence/l1 -->
# Spec-linked validation evidence

Before a ticket transitions to `in-review`, the spec that drives the work must already link the evidence that supports closing it. Reviewers should be able to start from the spec and reach the related tickets, the updated documentation, and the validation result without searching the workspace.

<!-- spec-api:entry id=3c02d587-9f5d-458c-adeb-7d22fa0936f9 slug=context-engine/recurring-principles/validation-evidence/spec-linked-validation-evidence/required-links-on-the-spec/l5 -->
## Required links on the spec

- The exact ticket folder paths of every related ticket, rendered in the canonical traceability link format.
- The updated documentation that accompanies the change (READMEs, instruction files, generated docs).
- The passing validation commands (test invocations, `spec sync-generated --check`, browser verification steps) or a clearly documented blocker if validation could not pass.

<!-- spec-api:entry id=1ae9aeb7-4156-4d5f-9164-1c4c2d48c747 slug=context-engine/recurring-principles/validation-evidence/spec-linked-validation-evidence/status-summary/l11 -->
## Status summary

Each ticket's status summary must explicitly report implementation status, validation status, and documentation status. When a required validation repeatedly fails, the failing command or manual verification, the observed result, and the blocker must be recorded — do not silently skip the gate.

<!-- spec-api:entry id=bf653268-549c-4da2-af24-9147a8ae41a8 slug=context-engine/recurring-principles/validation-evidence/spec-linked-validation-evidence/substitutes/l15 -->
## Substitutes

When dedicated test, documentation, or cross-store-link tooling is missing or only partial, use the strongest available substitute and call out the gap in both the ticket status summary and the spec traceability links. The spec is the source of truth for "what evidence exists today", so omissions must be visible there.
