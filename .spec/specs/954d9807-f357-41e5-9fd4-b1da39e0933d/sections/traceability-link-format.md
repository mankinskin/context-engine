<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=24dc1ff2-2a3b-4f51-8b72-24e93a3e7ff4 slug=context-engine/recurring-principles/traceability-link-format/ticket-traceability-link-format/l1 -->
# Ticket traceability link format

When mentioning a ticket in chat output, spec prose, or generated documentation, render the reference as a markdown link whose target is the canonical ticket folder path returned by ticket-api, with `/ticket.toml` appended so editors can open the ticket file directly.

<!-- spec-api:entry id=cc4936df-940c-480f-b49b-7950c0d23fa7 slug=context-engine/recurring-principles/traceability-link-format/ticket-traceability-link-format/required-form/l5 -->
## Required form

`[<short-id> <title>](<canonical ticket folder path>/ticket.toml)`

<!-- spec-api:entry id=08ade94d-1202-4f78-bfa7-3135418c2170 slug=context-engine/recurring-principles/traceability-link-format/ticket-traceability-link-format/required-form/l9 -->
- `<short-id>` is the first 8 characters of the authoritative ticket id.
- `<title>` is the authoritative ticket title.
- `<canonical ticket folder path>` is the exact folder path returned by ticket-api for that ticket. Never synthesize a path from a UUID, from the current store root, or from an example path.

<!-- spec-api:entry id=e502f969-0d1e-4484-9c98-efcd74ef4509 slug=context-engine/recurring-principles/traceability-link-format/ticket-traceability-link-format/resolving-the-canonical-path/l13 -->
## Resolving the canonical path

If the first ticket-api response (for example `ticket create`) omits the folder path, run a follow-up call such as `ticket get <id> --json` and read `.payload.ticket.path` before composing the reference. Nested workspaces, alternate scan roots, and ancestor checkouts all produce different canonical paths, so the path must come from ticket-api rather than from a template.
