<!-- aligned-structure:v1 -->

# Summary

Capture the recurring cross-cutting principles that apply across the context-engine root workspace and its nested stores and viewers.

## Behavior Story

The root workspace keeps one canonical home for recurring principles so generated guidance and downstream specs can reference stable cross-cutting rules instead of duplicating them in each subsystem.

## Provided Surface Contracts

- The root recurring-principles spec is the canonical authority for shared cross-cutting principles across nested workspaces.
- Each principle is maintained as its own section so generated guidance can reference it independently.
- The root principles currently cover browser validation, generated file provenance markers, ticket link format, and validation evidence expectations.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- [f147eb0e Migrate recurring spec principles to canonical rule entries via spec sync-generated](.ticket/tickets/f147eb0e-c758-459b-a956-a1162c3e1af6/ticket.toml)
- [a5fe4c58 Adopt rule targets for generated spec artifacts](memory-api/.ticket/tickets/a5fe4c58-f59c-4d97-8ee6-3447724b5fac/ticket.toml)

## Background Knowledge References

- `spec-api/generated-documents` (`1cf68c36-7f64-4d81-b553-1947b978fbe3` in memory-viewers/memory-api)

## Legacy Content (Preserved)

# Recurring cross-cutting principles

This spec captures cross-cutting design principles that appear repeatedly across context-engine workspace specs and agent guidance. Each principle has its own section so it can be referenced individually from generated documents and so a `rule scan` produces one canonical rule entry per principle.

The four principles owned at the context-engine root apply across every nested workspace (memory-api, viewer-api, doc/log/ticket/spec viewers, audit, context-trace/search/read/insert/api).

## Sections

- `browser-validation` — Mandatory external-browser verification for any change to a server interface or frontend feature.
- `generated-file-markers` — `*-api:file` and `*-api:entry` provenance markers and byte-stable regeneration.
- `traceability-link-format` — Canonical `[<short-id> <title>](<canonical folder>/ticket.toml)` link form for ticket references in chat output and docs.
- `validation-evidence` — Specs must link related tickets, updated docs, and passing or blocked validation results before any ticket reaches `in-review`.

## Related tickets

- [f147eb0e Migrate recurring spec principles to canonical rule entries via spec sync-generated](.ticket/tickets/f147eb0e-c758-459b-a956-a1162c3e1af6/ticket.toml)
- [a5fe4c58 Adopt rule targets for generated spec artifacts](memory-api/.ticket/tickets/a5fe4c58-f59c-4d97-8ee6-3447724b5fac/ticket.toml)

## Related specs

- `spec-api/generated-documents` (`1cf68c36-7f64-4d81-b553-1947b978fbe3` in memory-viewers/memory-api)
