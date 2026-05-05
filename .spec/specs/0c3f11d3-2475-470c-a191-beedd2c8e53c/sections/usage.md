# Usage

## CLI

Run a default audit:

```bash
cargo run -p repo-qa-mcp --bin repo-qa -- audit .
```

Request structured JSON:

```bash
cargo run -p repo-qa-mcp --bin repo-qa -- --json audit .
```

Override thresholds:

```bash
cargo run -p repo-qa-mcp --bin repo-qa -- audit . \
  --max-file-lines 300 \
  --max-cyclomatic-complexity 10 \
  --coverage-warn-below 85
```

## MCP

Start the stdio server:

```bash
cargo run -p repo-qa-mcp --bin repo-qa-mcp
```

Call `audit_repository` with a payload such as:

```json
{
  "repo_root": ".",
  "max_file_lines": 350,
  "max_cyclomatic_complexity": 10,
  "coverage_warn_below": 85.0
}
```

## Config

`repo-qa` auto-loads `.repo-qa.toml` from the repository root.

```toml
exclude_paths = [
  "crates/deps/",
  "target/",
]
```

Excluded paths are omitted from source indexing and from Cargo-scoped quality trials.