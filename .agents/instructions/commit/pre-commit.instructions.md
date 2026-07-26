---
description: "Use when troubleshooting or configuring the repository pre-commit hook. Covers what the hook checks, how to resolve failures, and when bypassing is acceptable."
---

## What the pre-commit hook checks

The hook runs checks on staged files (examples):

- `.vscode/tasks.json` / `.vscode/tasks.d/*.jsonc` — regenerate tasks and reject drift
- `rule-targets.yaml` and `.rule/**` — `rule sync-targets --check`
- `memory-viewers/rule-targets.yaml` and `memory-api/rule-targets.yaml` — per-submodule rule checks

## Resolving failures

Regenerate the failing outputs and stage them before re-committing:

```bash
cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml
git add .clinerules/10-core-rules.md
git commit -m "chore(rule): regenerate targets"
```

## Bypass (rare)

Only use `--no-verify` when you can justify the bypass in the commit message and the failure is a confirmed false positive:

```bash
git commit --no-verify
```
