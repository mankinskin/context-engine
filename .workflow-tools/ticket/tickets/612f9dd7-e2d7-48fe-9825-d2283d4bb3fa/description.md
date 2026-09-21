Investigate false-drift failure in aggregated `rule sync-targets --config rule-targets.yaml --check` for viewer-api recurring-principles body target.

Observed behavior:
- Failing path: `viewer-api/.spec/specs/798c9a3c-404a-4842-874d-484edb4209ef/body.md`
- `generate-target --dry-run` output for `viewer-api-recurring-principles-body` is byte-identical to file content (`cmp` exit 0).
- Despite byte identity, both aggregated and direct `generate-target --check` report `generated output differs`.
- Attempted write path via `generate-target`/`sync-targets` in viewer-api workspace frequently returns `storage error: os error 3`.

Acceptance criteria:
1. Identify root cause of false positive and/or workspace path error.
2. `rule sync-targets --config rule-targets.yaml --check` succeeds from context-engine root.
3. Document fix in ticket notes and confirm no unintended changes to unrelated targets.
