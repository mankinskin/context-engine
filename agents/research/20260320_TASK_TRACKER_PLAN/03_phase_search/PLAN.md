# Phase 3 — Full-Text Search + Unified Query Language (Tantivy)

**Status:** BLOCKED (requires Phase 1 CRUD stable; **promoted to run alongside Phase 1** per Q9 answer)

## Objective

Index every ticket's textual content (and extractable binary content) in Tantivy and
expose a **unified query language** that combines free-text and metadata predicates
in one expression. Highlighted snippets are required.

## Problem/Solution/Reference Baseline

1. Problem: swarm agents need one query surface for both planning text and operational filters.
Solution: unified query language combining FTS + structured predicates.
Reference: machine-oriented list/ready/search workflow patterns in both beads projects.

2. Problem: search index can drift from filesystem/index source of truth.
Solution: index is strictly derived and fully rebuildable from scan.
Reference: source-of-truth separation discipline seen in `Dicklesworthstone/beads_rust`.

## Deliverables

- [ ] `TantivySearchIndex::upsert(id, doc)` — index new/updated ticket
- [ ] `TantivySearchIndex::remove(id)` — delete from index on ticket deletion
- [ ] `QueryParser::parse(expr: &str) -> Query` — unified AST from query string
- [ ] `TantivySearchIndex::search(query, limit, highlight) -> Vec<SearchResult>` —
      returns ranked results with highlighted snippet text
- [ ] `ticket search "<query>"` CLI command
- [ ] HTTP `POST /api/tickets/search` endpoint
- [ ] HTTP `POST /api/tickets/query` endpoint (same grammar; structured execution mode)
- [ ] `ticket scan --reindex` flag — full Tantivy rebuild from FS content
- [ ] Binary content extractor registry: text → pass-through; PDF → best-effort text
      extract; unknown binary → filename + metadata only

## Unified Query Language

Grammar (informal):

```
query  := expr (WS expr)*
expr   := field_pred | fts_term | quoted_phrase
field_pred := IDENT ":" value
value  := bare_word | quoted_string | range
range  := "[" value "TO" value "]"
```

Examples:
```
"login page"                                          # free-text phrase
status:open                                           # exact field match
assigned:alice "login page"                           # combined
status:open created:[2026-01-01 TO 2026-03-31]        # date range + status filter
```

Field names map to Tantivy `FAST`/`STRING` fields; unrecognised field names surface
as a query parse error with a suggestion.

## Tantivy Field Schema

```rust
// Universal fields (always present)
schema_builder.add_text_field("id",          STRING | STORED);
schema_builder.add_date_field("created_at",  INDEXED | STORED | FAST);
schema_builder.add_date_field("updated_at",  INDEXED | STORED | FAST);
schema_builder.add_text_field("ticket_type", STRING | STORED | FAST);

// Well-known optional fields (populated when present in manifest)
schema_builder.add_text_field("body",        TEXT | STORED);  // description.md content
schema_builder.add_text_field("attachments", TEXT | STORED);  // extracted text from assets

// Dynamic fields from type schema registered when a type schema is loaded
// (e.g. status, assignee, priority, labels)
// Namespace prefix x_ reserved to avoid collision with universal fields
```

## Index Lifecycle

- **Write path**: Phase 1 `create`/`update` calls `TantivySearchIndex::upsert` before
  releasing the per-ticket lock.
- **Crash recovery**: if the process dies after the redb commit but before the Tantivy
  write, `ticket scan --reindex` rebuilds the full index. Tantivy is always derived;
  never source-of-truth.
- **Schema evolution**: index schema version stored in redb `META`; schema change
  triggers automatic full reindex on next startup.

## Key Interview Answers Applied Here

| Answer | Search impact |
|--------|--------------|
| Q8 — Any attachments | Binary content extractor registry; text-only fallback for unknowns |
| Q9 — FTS + metadata + unified query | Unified query AST; both Tantivy and redb predicates in one pass |

## Risks

- Tantivy index directory must be excluded from OS file-sync tools (OneDrive, Dropbox)
  to avoid corruption; document this prominently.
- Index schema changes require full re-index; version the schema in `META`.
- Dynamic type-schema fields (configurable SM from Q3) may collide with universal field
  names; enforce namespace prefix `x_<type>_<field>`.

## TODO

- TODO: Decide index storage path: `.context-engine/ticket-index/search/` alongside redb.
- TODO: Write query language grammar as a formal PEG/pest grammar before implementation.
- TODO: Benchmark index rebuild time for 10 000 tickets with mixed binary attachments.
- TODO: Define field name collision policy for dynamic type-schema fields.
