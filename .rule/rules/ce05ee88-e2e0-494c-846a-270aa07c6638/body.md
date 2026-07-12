## Clickable Reference Policy

Render every reference to a workspace entity (ticket, spec, doc, log, or one of their files) as a clickable markdown link in **all** responses. This policy is the single global switch: change the reference format here and it applies everywhere agents and prompts emit references. Canonical source: the `clickable-references` section of the recurring cross-cutting principles spec.

Emit a reference in exactly one of three forms, selected by the active reference mode (`manifest` is the default):

1. `viewer` — a deep link to the domain viewer server that opens the entity:
   - ticket-viewer: `http://localhost:3002/workspace/<workspace>/ticket/<id>`
   - spec-viewer: `http://localhost:4002/specs/<id>`
   - doc-viewer: `http://localhost:3001/<doc-route>`
   - log-viewer: `http://localhost:3000/<log-route>`
2. `manifest` — a relative link to the entity's manifest file (for example a ticket's `ticket.toml` or a spec's `spec.toml`).
3. `description` — a relative link to the entity's top description/body file (for example a spec's `body.md`).

Link text is always `<short-id> <title>`, where `<short-id>` is the first 8 characters of the authoritative entity id and `<title>` is the authoritative entity title.

Path normalization:
- Always emit forward-slash (unix) paths; convert any Windows backslashes.
- Emit repo-root-relative paths; never emit drive-letter absolute paths such as `C:/...`.
- Assume a mingw (Git Bash) or WSL shell, so a repo-relative unix path resolves for both file links and terminal use.
- Never wrap a clickable file or path reference in backticks.

Resolve manifest and description paths from the owning API (ticket-api, spec-api, …), not from a template. If the first response omits the folder path, run a follow-up call (for example `ticket get <id> --json` and read `.payload.ticket.path`) before composing the reference.
