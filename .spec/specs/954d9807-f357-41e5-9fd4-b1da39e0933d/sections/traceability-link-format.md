# Ticket traceability link format

When mentioning a ticket in chat output, spec prose, or generated documentation, render the reference as a markdown link whose target is the canonical ticket folder path returned by ticket-api, with `/ticket.toml` appended so editors can open the ticket file directly.

## Required form

Render references per the Clickable Reference Policy in `AGENTS.md`.

- `<short-id>` is the first 8 characters of the authoritative ticket id.
- `<title>` is the authoritative ticket title.
- `<canonical ticket folder path>` is the exact folder path returned by ticket-api for that ticket. Never synthesize a path from a UUID, from the current store root, or from an example path.

## Resolving the canonical path

If the first ticket-api response (for example `ticket create`) omits the folder path, run a follow-up call such as `ticket get <id> --json` and read `.payload.ticket.path` before composing the reference. Nested workspaces, alternate scan roots, and ancestor checkouts all produce different canonical paths, so the path must come from ticket-api rather than from a template.
