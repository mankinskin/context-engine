# repo-qa

`repo-qa` is the repository quality audit tool for this workspace. Its code is split across three layers:

- `repo-qa-api` for audit logic, models, config loading, indexing, and quality trials
- `repo-qa-cli` for the `repo-qa` command and output rendering
- `repo-qa-mcp` for the thin MCP transport exposing `audit_repository`

Each run canonicalizes the repository root, loads `.repo-qa.toml`, synchronizes source files into `.repo-qa/repo-qa.sqlite3`, prunes stale index rows, collects repository quality metrics, stores an audit run record, and returns actionable findings plus aggregated repair instructions.

The current quality trials cover:

- file length
- compiler warnings
- unit test success
- line coverage
- Rust static complexity metrics

Output is designed for both agents and humans:

- JSON returns the full `AuditReport` contract for downstream automation.
- Text output renders the same report as a compact, readable summary.

Paths in both surfaces are normalized to Unix format, including on Windows.