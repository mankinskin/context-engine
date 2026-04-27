# Versioned git hooks

Activate once per clone:

```bash
git config core.hooksPath .githooks
```

Hooks here run for every commit on this clone afterwards. Bypass an
individual run with `git commit --no-verify` (rarely needed).

## Hooks

| hook         | purpose                                                       |
| ------------ | ------------------------------------------------------------- |
| `pre-commit` | Regenerates `.vscode/tasks.json` from `.vscode/tasks.d/*.jsonc` and aborts the commit if it was out of date. |
