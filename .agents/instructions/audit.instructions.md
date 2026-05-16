---
description: "Use when editing or operating the audit tool. Covers CLI and MCP usage, repo config, and how to interpret audit output."
applyTo: "crates/audit-api/**,tools/cli/audit-cli/**,tools/mcp/audit-mcp/**,.audit.toml"
---

## Purpose

`audit` is the repository quality audit tool for this workspace.

- Core library crate: `audit-api`
- CLI package: `audit-cli` with the `audit` binary
- MCP package: `audit-mcp`
- MCP tool: `audit`

Keep the layering thin and explicit:

1. `audit-api` owns audit logic, models, config loading, indexing, and trials.
2. `audit-cli` owns argument parsing and human/json rendering.
3. `audit-mcp` only translates MCP inputs into `audit-api` calls and serializes the result.

One audit run:

1. resolves the repo root
2. loads `.audit.toml`
3. syncs source files into `.audit/audit.sqlite3`
4. prunes stale index rows not seen in the latest scan
5. collects file length, compiler warning, test success, coverage, and static complexity metrics
6. returns raw metrics plus actionable findings and deduplicated fix instructions

Prefer JSON output for automation and agent workflows. Prefer text output for local inspection.

## CLI Usage

Basic audit:

```bash
cargo run -p audit-cli --bin audit -- run .
```

Machine-readable output:

```bash
cargo run -p audit-cli --bin audit -- --json run .
```

Override thresholds for a stricter audit:

```bash
cargo run -p audit-cli --bin audit -- run . \
  --max-file-lines 300 \
  --max-cyclomatic-complexity 10 \
  --coverage-warn-below 85
```

The default thresholds are:

- `max_file_lines = 400`
- `max_cyclomatic_complexity = 12`
- `coverage_warn_below = 80.0`

## MCP Usage

Run the server on stdio:

```bash
cargo run -p audit-mcp --bin audit-mcp
```

Tool input example:

```json
{
  "repo_root": ".",
  "max_file_lines": 350,
  "max_cyclomatic_complexity": 10,
  "coverage_warn_below": 85.0
}
```

The MCP tool always returns the full structured `AuditReport` payload. Use it as the single synchronized read for repository quality state.

## Repo Config

`audit` auto-loads a repo-root `.audit.toml` file.

Example:
