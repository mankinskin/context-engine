## Clickable Reference Policy

Render every reference to a workspace entity (ticket, spec, doc, log, or one of their files) as a clickable markdown link in **all** agent and prompt responses. This entry is the single canonical owner of reference formatting for the repository: the "Formatting conflict policy" note in the Instruction Precedence section defers here, and switching the reference format is done here once.

**Scope.** These rules govern the reference token you emit in a response — the markdown link and the path inside it. They do not govern ordinary prose that merely names a file, nor backticked shell commands. The anti-backtick rule below applies to the emitted reference, not to illustrative prose in this policy.

Emit a reference in exactly one of three forms, selected by the active reference mode (default: manifest):

1. viewer — a deep link to the domain viewer server that opens the entity. Routes that exist today:
   - ticket-viewer: http://localhost:3002/workspace/{workspace}/ticket/{id}
   - spec-viewer: http://localhost:4002/specs/{id}
   - log-viewer: http://localhost:3000/#/file/{url-encoded-log-name} (append /stats or /hypergraph for those tabs)
   - doc-viewer (port 3001) has no stable per-entity deep-link route yet — its artifacts are keyed by package::target, not a URL. Use manifest or description mode for docs until a route exists.
2. manifest — a relative link to the entity's manifest file: a ticket's ticket.toml, a spec's spec.toml, or the equivalent manifest.
3. description — a relative link to the entity's top description/body file: a spec's body.md, or a ticket's rendered description.md.

Link text is always "{short-id} {title}", where {short-id} is the first 8 characters of the authoritative entity id and {title} is the authoritative entity title.

Path normalization for every emitted reference:
- Use forward-slash (unix) paths only; convert any Windows backslashes.
- Use repo-root-relative paths; never emit a drive-letter absolute path (no C:/… form).
- Assume a mingw (Git Bash) or WSL shell, so a repo-root-relative unix path resolves for both file links and terminal use.
- Do not wrap the emitted reference — its link text or its path — in backticks.

Resolve manifest and description paths from the owning API (ticket-api, spec-api, and so on), not from a template. If the first response omits the folder path, run a follow-up call (for example ticket get {id} --json and read the payload's ticket path) before composing the reference.
