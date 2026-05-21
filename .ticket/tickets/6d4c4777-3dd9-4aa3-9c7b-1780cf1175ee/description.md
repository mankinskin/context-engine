# Problem

The repository workflow guidance spec says the guidance under `.agents/` and `.github/` is regenerated from canonical rule content, but three key files are still effectively being treated as directly edited markdown instead of obvious generated targets:

- `.agents/instructions/ticket-system.instructions.md`
- `.github/prompts/ticket.prompt.md`
- `.github/prompts/tickets.prompt.md`

That creates two problems:

1. the rendered files do not clearly advertise that they are generated outputs
2. workflow updates can still be applied by editing the rendered files first instead of treating the rule store as the source of truth

## Scope

Make those three guidance files fully rule-managed generated targets.

This includes:

- ensuring the canonical rule entries are the source of truth for the file content
- regenerating the files through the rule target pipeline instead of manual edits
- confirming the rendered files follow the generated-output shape used elsewhere in the repository
- validating `rule sync-targets` and `rule sync-targets --check`

## Acceptance criteria

1. `.agents/instructions/ticket-system.instructions.md`, `.github/prompts/ticket.prompt.md`, and `.github/prompts/tickets.prompt.md` are rendered by the rule target pipeline from canonical rule entries.
2. The generated files clearly present themselves as generated outputs, consistent with the repository's generated-guidance conventions.
3. The latest ticket-link formatting rule lives in canonical rule content and survives regeneration.
4. `rule sync-targets --config rule-targets.yaml` passes.
5. `rule sync-targets --config rule-targets.yaml --check` passes.

## Related context

This is a follow-up to the completed workflow-guidance work tracked in `.ticket/tickets/762d9ac9-e0e0-4f02-b60f-21c79e3c26f6` and the spec `.spec/specs/47465a64-0c5f-4ddc-8d38-018048090af2`.
