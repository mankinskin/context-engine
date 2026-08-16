# Versioned git hooks

Activate once per clone:

```bash
bash setup_git.sh
```

The setup also pins `core.autocrlf=false`, `core.eol=lf`,
`core.safecrlf=warn`, and `merge.renormalize=true` for this clone. The
repository's `.gitattributes` then normalizes detected text to LF in the index,
so CRLF written by an editor or generator does not become a content diff after
the one-time index normalization has been committed.

Hooks here run for every commit on this clone afterwards. Bypass an
individual run with `git commit --no-verify` (rarely needed).

## Hooks

| hook         | purpose                                                       |
| ------------ | ------------------------------------------------------------- |
| `pre-commit` | Regenerates `.vscode/tasks.json` from `.vscode/tasks.d/*.jsonc` and aborts the commit if it was out of date. |
