# Output

`repo-qa` returns an `AuditReport` with these top-level fields:

- `service`
- `repo_root`
- `index_database`
- `sync`
- `run`
- `metrics`
- `findings`
- `instructions`

## Sync semantics

`sync.pruned_files` is the indicator that stale index rows were removed during this run. `updated_files` and `reused_files` distinguish newly refreshed content from unchanged index entries.

## Metric semantics

Each trial returns a status:

- `collected`
- `unavailable`
- `failed`
- `not_applicable`

Unavailable metrics remain part of the report. For example, missing `cargo llvm-cov` should produce an unavailable coverage summary rather than aborting the audit.

## Findings

Each finding carries:

- severity
- summary
- optional path and line
- raw metric value and threshold
- concrete repair instructions
- structured evidence for downstream tooling

`instructions` at the report root is the deduplicated union of all finding-level fix guidance.

## Human-readable projection

Text output shows:

- repo path and index path
- sync summary
- metric summaries
- one finding line per issue
- `fix:` lines under each finding

The text view is a condensed rendering of the same report contract used in JSON mode.