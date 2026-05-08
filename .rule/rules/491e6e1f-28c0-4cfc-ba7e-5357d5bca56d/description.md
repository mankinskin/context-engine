### Top-level fields

- `service`: service identifier. Current value is `audit-mcp`.
- `repo_root`: canonical repository root used for the audit.
- `index_database`: path to the local SQLite index at `.audit/audit.sqlite3`.
- `sync`: current scan statistics.
- `run`: persisted audit run metadata.
- `metrics`: raw collected metric values and trial status.
- `findings`: actionable issue records.
- `instructions`: unique repo-level fix instructions aggregated from findings.