# Usage

## CLI

Run a default audit:

```bash
cargo run -p audit-cli --bin audit -- .
```

Request structured JSON:

```bash
cargo run -p audit-cli --bin audit -- --json .
```

Override thresholds:

```bash
cargo run -p audit-cli --bin audit -- . \
  --max-file-lines 300 \
  --max-cyclomatic-complexity 10 \
  --coverage-warn-below 85
```

## MCP

Start the stdio server:

```bash
cargo run -p audit-mcp --bin audit-mcp
```

Call `audit` with a payload such as:

```json
{
  "repo_root": ".",
  "max_file_lines": 350,
  "max_cyclomatic_complexity": 10,
  "coverage_warn_below": 85.0
}
```

## Config

`audit` auto-loads `.audit.toml` from the repository root.

```toml
exclude_paths = [
  "crates/deps/",
  "target/",
]
```

Excluded paths are omitted from source indexing and from Cargo-scoped quality trials.