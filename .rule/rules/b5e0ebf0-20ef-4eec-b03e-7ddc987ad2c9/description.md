## Workflow Expectations

- Start implementation work by searching for existing tickets and creating or updating the required ticket set before code changes.
- Update or create the relevant spec before implementation when requirements, goals, or behavior are new or changing.
- For each ticket, implement the scoped change, run the required validation until it passes or repeatedly fails, update docs, verify the spec links the ticket folder path(s), docs, and validation results, then move the ticket to `in-review`.
- If validation repeatedly fails, do not silently skip it. Record the failing command or manual verification result and the blocker in the ticket/spec status summary.
- Summaries and handoffs must report implementation, validation, and documentation status.
- When dedicated test, doc, or cross-store-link tooling is missing or partial, use the strongest available substitute and note the gap explicitly.
- When mentioning tickets in chat output, reference each ticket by the exact ticket folder path returned by ticket-api output.
- Never synthesize a ticket folder path from a UUID, the current store root, or an example path; if the first ticket-api response omits the path, run a follow-up ticket-api command that returns the authoritative path before responding.
- Render ticket references in chat markdown as links whose text and target are both that exact returned folder path, preserving nested workspace segments exactly as emitted by the tool output.