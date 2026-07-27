<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=34d5f1cd-e6ad-41db-955c-672b22fc9bb5 slug=context-engine/session-api/session-audit-unified-interface digest=ea0643fd2dd9 -->

# Session audit in unified audit interface with schema-versioned persisted sessions

- slug: `context-engine/session-api/session-audit-unified-interface`
- component: session-api
- state: draft
- index_ref: `.spec/specs/34d5f1cd-e6ad-41db-955c-672b22fc9bb5/spec.toml`

## Summary

Define a stable contract for session-level auditing from persisted session artifacts and expose it through the unified audit CLI surface.

## Acceptance Criteria Excerpt

1. Session-api exposes a `session_audit` operation that accepts either explicit session id or latest-session selector and returns a structured report. 2. Persisted session records include a schema version field managed by session-api. 3. Session load/audit behavior handles schem…

## Navigation

- Parent: _(root)_
- Children: _(none)_
