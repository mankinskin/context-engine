# repo-qa

`repo-qa` is the repository quality audit tool for this workspace. It exposes the same audit pipeline through a human-facing CLI (`repo-qa`) and a single-tool MCP server (`repo-qa-mcp`).

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