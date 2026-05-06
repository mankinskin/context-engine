---
description: "Use when editing or operating the repo-qa audit tool. Covers CLI and MCP usage, repo config, and how to interpret audit output."
applyTo: "crates/repo-qa-api/**,tools/cli/repo-qa-cli/**,tools/mcp/repo-qa-mcp/**,.repo-qa.toml"
---

# Repo QA Guidance

## Purpose

`repo-qa` is the repository quality audit tool for this workspace.

- Core library crate: `repo-qa-api`
- CLI package: `repo-qa-cli` with the `repo-qa` binary
- MCP package: `repo-qa-mcp`
- MCP tool: `audit_repository`

Keep the layering thin and explicit:

1. `repo-qa-api` owns audit logic, models, config loading, indexing, and trials.
2. `repo-qa-cli` owns argument parsing and human/json rendering.
3. `repo-qa-mcp` only translates MCP inputs into `repo-qa-api` calls and serializes the result.

One audit run:

1. resolves the repo root
2. loads `.repo-qa.toml`
3. syncs source files into `.repo-qa/repo-qa.sqlite3`
4. prunes stale index rows not seen in the latest scan
5. collects file length, compiler warning, test success, coverage, and static complexity metrics
6. returns raw metrics plus actionable findings and deduplicated fix instructions

Prefer JSON output for automation and agent workflows. Prefer text output for local inspection.

## CLI Usage

Basic audit:

```bash
cargo run -p repo-qa-cli --bin repo-qa -- audit .
```

Machine-readable output:

```bash
cargo run -p repo-qa-cli --bin repo-qa -- --json audit .
```

Override thresholds for a stricter audit:

```bash
cargo run -p repo-qa-cli --bin repo-qa -- audit . \
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
cargo run -p repo-qa-mcp --bin repo-qa-mcp
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

`repo-qa` auto-loads a repo-root `.repo-qa.toml` file.

Example:

```toml
# Paths are relative to the repository root.
# Entries exclude matching directories and files from repo-qa audits.
exclude_paths = [
  "crates/deps/",
  "target/",
]
```

Exclusions affect both:

- source-file indexing
- Cargo-scoped metrics such as compiler warnings, tests, and coverage

## Output Contract

All printed and serialized paths use Unix separators (`/`), including on Windows.

### Top-level fields

- `service`: service identifier. Current value is `repo-qa-mcp`.
- `repo_root`: canonical repository root used for the audit.
- `index_database`: path to the local SQLite index at `.repo-qa/repo-qa.sqlite3`.
- `sync`: current scan statistics.
- `run`: persisted audit run metadata.
- `metrics`: raw collected metric values and trial status.
- `findings`: actionable issue records.
- `instructions`: unique repo-level fix instructions aggregated from findings.

### Sync

`sync` explains how the file index changed during the current run:

- `scanned_files`: files seen in the current walk
- `updated_files`: files re-read because content or metadata changed
- `reused_files`: unchanged files reused from the existing index
- `pruned_files`: stale index rows deleted because the file was no longer seen

`pruned_files` is the stale-entry signal. A non-zero value means the run removed outdated index rows.

### Metrics

`metrics` contains both summary values and per-trial status.

- `file_length`: always summarizes indexed source files
- `compiler_warnings`: count-style metric with `status`, `count`, and optional `details`
- `test_results`: pass/fail totals plus success rate
- `coverage`: line coverage summary
- `static_metrics`: complexity summary over analyzed Rust functions

Trial `status` values mean:

- `collected`: metric ran successfully
- `unavailable`: required tool is missing or the environment cannot provide the metric
- `failed`: the metric execution itself failed
- `not_applicable`: the metric does not apply to the current repo slice

Example: if `cargo llvm-cov` is missing, `coverage.status` will be `unavailable` and `details` will explain why.

### Findings

Each finding is an issue record with enough detail for an agent or user to act immediately.

- `category`: high-level issue group such as file length or coverage
- `severity`: `low`, `medium`, or `high`
- `summary`: short human-readable diagnosis
- `path` and `line`: optional source location
- `metric_name`, `metric_value`, `threshold`: raw measurement data
- `instructions`: concrete fix steps for that finding
- `evidence`: structured supporting details

Use `findings` when you need to drive follow-up remediation work. Use `instructions` when you need a deduplicated repair checklist across the whole run.

### Human Output

Text mode renders the same report as a compact summary:

- repo and index paths
- sync counters
- metric summaries
- one line per finding
- indented `fix:` lines for each finding-specific instruction

Treat text output as a readable projection of the JSON report, not as a separate contract.

## Operating Notes

- The audit database is local runtime state and should not be committed.
- Coverage degrades gracefully when `cargo llvm-cov` is unavailable; it should produce a structured unavailable result rather than aborting the full audit.
- If you change the audit contract, update tests and this instruction file together.