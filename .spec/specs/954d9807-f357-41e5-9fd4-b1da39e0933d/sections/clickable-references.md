# Clickable reference policy

Every agent and prompt should render references to workspace entities (tickets,
specs, docs, logs, and their files) as clickable markdown links in **all**
responses. This policy is the single source of truth: switching the reference
format is done here once, and the change propagates to every generated agent
surface that renders this principle.

## Reference modes

A reference to an entity is emitted in exactly one of three forms, selected by
the active reference mode. `manifest` is the default.

1. `viewer` — a link to the domain viewer server that opens the entity:
   - ticket-viewer: `http://localhost:3002/workspace/<workspace>/ticket/<id>`
   - spec-viewer: `http://localhost:4002/specs/<id>`
   - doc-viewer: `http://localhost:3001/<doc-route>`
   - log-viewer: `http://localhost:3000/<log-route>`
2. `manifest` — a relative link to the entity's manifest file, for example a
   ticket's `ticket.toml`, a spec's `spec.toml`, or an equivalent manifest.
3. `description` — a relative link to the entity's top description/body file,
   for example a spec's `body.md` or a ticket's rendered description file.

Regardless of mode, the link text is `<short-id> <title>`, where `<short-id>`
is the first 8 characters of the authoritative entity id and `<title>` is the
authoritative entity title.

## Path normalization

- Always emit forward-slash (unix) paths. Convert any Windows-style backslashes.
- Emit relative paths from the repository root; never emit drive-letter
  absolute paths such as `C:/...`.
- Assume the reader runs in a mingw (Git Bash) or WSL shell, so a repo-relative
  unix path resolves correctly for both file links and terminal use.
- Never wrap a clickable file or path reference in backticks.

## Resolving the canonical path

The manifest and description paths must come from the owning API
(ticket-api, spec-api, …), not from a template. If the first response omits the
folder path, run a follow-up call (for example `ticket get <id> --json` and read
`.payload.ticket.path`) before composing the reference. Nested workspaces,
alternate scan roots, and ancestor checkouts all produce different canonical
paths.