<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=24b3d22b-e235-4c4f-b53c-75fb819ea95b slug=ticket-api/entity/structured-ticket-entities digest=8d30ce46228a -->

# Structured Ticket Entities: multi-file parts, plan freezing, typed refs, and projected reads

- slug: `ticket-api/entity/structured-ticket-entities`
- component: ticket-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/24b3d22b-e235-4c4f-b53c-75fb819ea95b/spec.toml`

## Summary

A ticket entity today is a single mutable blob. Its directory holds `ticket.toml`, one `description.md`, `history.ndjson`, and an unused empty `assets/` scaffold (memory-api/crates/ticket-api/src/mod…

## Acceptance Criteria Excerpt

1. A ticket directory holds `parts/` files indexed by a `[[parts]]` manifest table; each entry carries a stable opaque `id`, `kind`, `path`, `frozen`, `created_at`, and optional `supersedes`; core kinds are schema-validated and free-form kinds round-trip as opaque attachments. 2…

## Navigation

- Parent: _(root)_
- Children: _(none)_
