# Goal

HTTP query responses must only expose tickets that resolve to authoritative indexed paths and workspace ownership.

# Scope

- remove fallback summary synthesis for unresolved search hits
- prefer authoritative resolved metadata over stale local placeholders when a mixed-workspace ticket is available from the registry
- deleted or unresolved ids are dropped from search results and recorded in diagnostics or logs
- query, list, detail, history, files, and asset flows share one ownership-resolution policy
- integrate authoritative folder-path work so follow-up tooling uses the same resolved owner and path

# Acceptance criteria

- a stale Tantivy document whose id no longer resolves is not returned by `/api/tickets`
- deleted tickets never reappear through the search branch
- mixed-workspace query results preserve authoritative owner workspace and follow-up path information
- transport responses never fabricate epoch-timestamp ghost summaries for unresolved ids
- focused HTTP integration tests lock these behaviors

# Required tests

- integration: unresolved search-only doc is dropped from the ticket list response
- integration: wrong local path plus authoritative child ticket resolves to the child owner or is dropped cleanly
- integration: deleted ticket does not reappear through the query branch
- integration: follow-up detail/history/files requests stay reversible for mixed-workspace search hits

# Rigorous validation requirements

- Explicitly inject each bad state: a stale Tantivy-only document, a deleted ticket that still has residual search state, and a wrong local row that competes with an authoritative mixed-workspace result.
- Assert that no fallback summary using ambient workspace ownership, empty metadata, or epoch timestamps can escape the query branch.
- Use the same fixture ids across list, detail, history, files, and asset follow-ups so ownership reversibility is proven end to end rather than per endpoint in isolation.
- Required command gate: focused `cargo test -p ticket-http search_list_ -- --nocapture` coverage for query/list behavior, plus targeted follow-up endpoint integration tests for reversible mixed-workspace hits.
