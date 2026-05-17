## `.vscode/tasks.d/` — modular VS Code tasks

VS Code only reads a single `tasks.json`, so this directory is a build-time
source layout: each `*.jsonc` file owns one logical group of tasks (or
inputs), and `scripts/build-vscode-tasks.py` merges them into the canonical
`.vscode/tasks.json`.

### Layout

| file                          | scope                                                |
| ----------------------------- | ---------------------------------------------------- |
| `00-inputs.jsonc`             | reserved merge slot; shared inputs currently unused  |
| `ticket-viewer.jsonc`         | direct cargo-run ticket-viewer tasks                 |
| `ticket-vscode.jsonc`         | ticket-vscode extension compile/watch                |
| `context-editor.jsonc`        | context-editor sandbox-app (trunk serve)             |
| `viewer-ctl-managed.jsonc`    | viewer-ctl `start` + external-browser compounds (4 viewers) |
| `viewer-ctl-prepare.jsonc`    | viewer-ctl `prepare` preLaunchTasks for lldb         |

Each file contains a JSON object with optional `tasks` and/or `inputs`
arrays. Comments (`//` and `/* */`) and trailing commas are allowed
(JSONC). Numeric prefixes order the input merge but don't affect runtime.

### Regenerating `tasks.json`

```bash
python scripts/build-vscode-tasks.py
```

The script:
- reads every `.vscode/tasks.d/*.jsonc` in lexicographic order,
- strips comments, parses JSON,
- concatenates `tasks` and `inputs` arrays,
- validates label uniqueness across `tasks` and id uniqueness across `inputs`,
- writes `.vscode/tasks.json` with a generated-file header.

Viewer-facing open tasks should prefer `scripts/open-external-browser.mjs`
over VS Code's integrated browser so visual validation runs in an external
Chromium-family window by default.

`.vscode/tasks.json` is committed (VS Code needs to read it directly) but
**must not be hand-edited** — the header reminds you. Edit the part-files
under `.vscode/tasks.d/` and regenerate.
