## Pre-commit Hook

The pre-commit hook at `.githooks/pre-commit` runs automatically when `git config core.hooksPath .github/hooks` is set.

### What it checks

| Staged file pattern | Check triggered |
|---|---|
| `.vscode/tasks.json`, `.vscode/tasks.d/*.jsonc` | Regenerate tasks from part-files; reject drift |
| `rule-targets.yaml`, `.rule/**`, `AGENTS.md`, `.github/copilot-instructions.md`, `.agents/instructions/*.instructions.md` | `rule sync-targets --config rule-targets.yaml --check` |
| `memory-viewers/rule-targets.yaml`, `memory-viewers/.rule/**`, `memory-viewers/README.md` | `rule sync-targets --config memory-viewers/rule-targets.yaml --check` |
| `memory-api/rule-targets.yaml`, `memory-api/.rule/**`, `memory-api/README.md` | `rule sync-targets --config memory-api/rule-targets.yaml --check` |
| `viewer-api/rule-targets.yaml`, `viewer-api/.rule/**`, `viewer-api/README.md` | `rule sync-targets --config viewer-api/rule-targets.yaml --check` |

### Resolving pre-commit failures

When the rule check fails, the generated output differs from the file on disk. Fix with:

```bash
# Run the failing sync to regenerate outputs
cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml

# Stage the regenerated files
git add AGENTS.md .github/copilot-instructions.md .clinerules/10-core-rules.md

# Re-commit
git commit -m "..."
```

**Never commit a rule-managed file by hand.** Always regenerate via `rule sync-targets` and commit the output.

### Bypass (rare)

```bash
git commit --no-verify
```

Only use `--no-verify` when the check is a confirmed false positive. Document why in the commit message.
