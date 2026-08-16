<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=1f652236-d39e-44a2-97c4-50264de6a574 slug=session-api/session-ticket-attribution digest=5f1f404c632e -->

# Session-to-ticket attribution evidence tiers and transcript backfill

- slug: `session-api/session-ticket-attribution`
- component: session-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/1f652236-d39e-44a2-97c4-50264de6a574/spec.toml`

## Summary

<!-- aligned-structure:v2 -->

## Acceptance Criteria Excerpt

1. After a dry-run, reading every affected `session.json` confirms `metadata.ticket_id` and `links.ticket_ids` are unchanged from the pre-run artifacts. 2. After a write run, reading each affected `session.json` confirms every new `links.ticket_ids` entry resolves to an existing…

## Navigation

- Parent: _(root)_
- Children: _(none)_
