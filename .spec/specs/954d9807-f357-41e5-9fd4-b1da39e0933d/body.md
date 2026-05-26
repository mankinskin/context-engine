<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=cd7b5d36-428b-4810-a559-6d8a53c1beee slug=context-engine/recurring-principles/recurring-cross-cutting-principles/l1 -->
# Recurring cross-cutting principles

This spec captures cross-cutting design principles that appear repeatedly across context-engine workspace specs and agent guidance. Each principle has its own section so it can be referenced individually from generated documents and so a `rule scan` produces one canonical rule entry per principle.

<!-- spec-api:entry id=a4486f09-e4b9-414e-996e-0dac99aa198a slug=context-engine/recurring-principles/recurring-cross-cutting-principles/l5 -->
The four principles owned at the context-engine root apply across every nested workspace (memory-api, viewer-api, doc/log/ticket/spec viewers, audit, context-trace/search/read/insert/api).

<!-- spec-api:entry id=a885218e-c6e7-4d10-8c00-fa5d85733cec slug=context-engine/recurring-principles/recurring-cross-cutting-principles/sections/l7 -->
## Sections

- `browser-validation` — Mandatory external-browser verification for any change to a server interface or frontend feature.
- `generated-file-markers` — `*-api:file` and `*-api:entry` provenance markers and byte-stable regeneration.
- `traceability-link-format` — Canonical `[<short-id> <title>](<canonical folder>/ticket.toml)` link form for ticket references in chat output and docs.
- `validation-evidence` — Specs must link related tickets, updated docs, and passing or blocked validation results before any ticket reaches `in-review`.

<!-- spec-api:entry id=661e017f-01ab-4a5f-8f0d-d6d4a9d21acc slug=context-engine/recurring-principles/recurring-cross-cutting-principles/related-tickets/l14 -->
## Related tickets

- [f147eb0e Migrate recurring spec principles to canonical rule entries via spec sync-generated](.ticket/tickets/f147eb0e-c758-459b-a956-a1162c3e1af6/ticket.toml)
- [a5fe4c58 Adopt rule targets for generated spec artifacts](memory-viewers/memory-api/.ticket/tickets/a5fe4c58-f59c-4d97-8ee6-3447724b5fac/ticket.toml)

<!-- spec-api:entry id=a1e06151-c860-46c1-8139-d0e88c66a598 slug=context-engine/recurring-principles/recurring-cross-cutting-principles/related-specs/l19 -->
## Related specs

- `spec-api/generated-documents` (`1cf68c36-7f64-4d81-b553-1947b978fbe3` in memory-viewers/memory-api)
