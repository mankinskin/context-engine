## Workflow Expectations

- Start implementation work by searching for existing tickets and creating or updating the required ticket set before code changes.
- Update or create the relevant spec before implementation when requirements, goals, or behavior are new or changing.
- For each ticket, implement the scoped change, run the required validation until it passes or repeatedly fails, update docs, verify the spec links the related tickets with openable `ticket.toml` targets plus the updated docs and validation results, then move the ticket to `in-review`.
- When capturing validation or documentation evidence in this repository, prefer the repo-local `workflow` CLI so the resulting artifact can be linked from specs and tickets.
- If validation repeatedly fails, do not silently skip it. Record the failing command or manual verification result and the blocker in the ticket/spec status summary.
- Summaries and handoffs must report implementation, validation, and documentation status.
- When dedicated test, doc, or cross-store-link tooling is missing or partial, use the strongest available substitute and note the gap explicitly.
- When mentioning tickets in chat output, use the exact canonical ticket folder path returned by ticket-api output as the base path for the markdown link target.
- Never synthesize a ticket folder path from a UUID, the current store root, or an example path; if the first ticket-api response omits the path, run a follow-up ticket-api command that returns the authoritative path before responding.
- Render ticket references in chat markdown as `[<short-id> <title>](<canonical ticket folder path>/ticket.toml)`, where `<short-id>` is the first 8 characters of the authoritative ticket id, `<title>` is the authoritative ticket title, and the link target appends `/ticket.toml` to the exact returned folder path so editors can open the ticket file directly.
