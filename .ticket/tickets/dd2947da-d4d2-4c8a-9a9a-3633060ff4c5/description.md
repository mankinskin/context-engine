# Goal

Make aggregate scan the single source of truth for both index visibility and search visibility.

# Scope

- normal scan removes or tombstones search documents when tickets disappear, are deleted on disk, move, or leave a scan root
- normal scan and force scan share the same visible semantics even if force scan still performs a heavier rebuild internally
- aggregate path repair updates both indexed metadata and search documents together
- reconciliation emits actionable diagnostics for unresolved or unrecoverable rows instead of silently leaving drift behind

# Acceptance criteria

- `scan(false)` removes a nested ticket from search results when the manifest is marked deleted on disk
- `scan(false)` prunes search and list visibility for tickets that leave a configured scan root
- moved or repaired nested tickets remain readable and searchable after a normal scan
- search, list, and get agree on visibility after add, update, delete, and move cases in a parent aggregate store
- focused storage regression coverage covers deleted-on-disk, removed-root, moved-path, and stale-search-doc cases

# Required tests

- unit: deleted nested ticket disappears from Tantivy results after `scan(false)`
- unit: removed scan root prunes previously indexed child tickets from search/list visibility
- unit: moved child ticket path repair keeps title, state, and searchability aligned
- regression: parent aggregate store returns the same visibility set through list, search, and get after reconciliation

# Rigorous validation requirements

- Reuse the same fixture graph across `scan(false)` and `scan(true)` so the tests prove semantic equivalence instead of two unrelated happy paths.
- Exercise each fixture through both repository-root and direct `.ticket` entry points when the entry point can change ownership resolution.
- Every scenario must assert agreement across `search_tickets`, `list`, `get`, and indexed metadata; if one surface diverges, the test should fail loudly.
- Include at least one negative regression where stale search state or stale indexed metadata exists before reconciliation and is repaired or pruned afterward.
- Required command gate: focused `cargo test -p ticket-api scan_ -- --nocapture` coverage for the new fixtures, plus any additional targeted storage slice needed for moved-path or removed-root coverage.
