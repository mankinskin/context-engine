# Recurring cross-cutting principles

This spec captures cross-cutting design principles that appear repeatedly across context-engine workspace specs and agent guidance. Each principle has its own section so it can be referenced individually from generated documents and so a `rule scan` produces one canonical rule entry per principle.

The principles owned at the context-engine root apply across every nested workspace (memory-api, viewer-api, doc/log/ticket/spec viewers, audit, context-trace/search/read/insert/api).

## Sections

- `browser-validation` — Mandatory external-browser verification for any change to a server interface or frontend feature.
- `generated-file-markers` — `*-api:file` and `*-api:entry` provenance markers and byte-stable regeneration.
- `traceability-link-format` — Canonical `[<short-id> <title>](<canonical folder>/ticket.toml)` link form for ticket references in chat output and docs.
- `clickable-references` — Single format-switchable policy for rendering entity references as clickable links (viewer deep-link, manifest path, or description path) with unix path normalization.
- `validation-evidence` — Specs must link related tickets, updated docs, and passing or blocked validation results before any ticket reaches `in-review`.

## Related tickets

- [f147eb0e Migrate recurring spec principles to canonical rule entries via spec sync-generated](.ticket/tickets/f147eb0e-c758-459b-a956-a1162c3e1af6/ticket.toml)
- [a5fe4c58 Adopt rule targets for generated spec artifacts](memory-api/.ticket/tickets/a5fe4c58-f59c-4d97-8ee6-3447724b5fac/ticket.toml)
- [15c39147 Global clickable-reference policy rendered across agent targets](.ticket/tickets/15c39147-4dac-4bdf-9b4f-b8b51e2a6c6e/ticket.toml)

## Related specs

- `spec-api/generated-documents` (`1cf68c36-7f64-4d81-b553-1947b978fbe3` in memory-viewers/memory-api)