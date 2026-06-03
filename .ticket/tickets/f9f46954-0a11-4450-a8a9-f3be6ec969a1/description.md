Restore the VS Code Copilot hook configuration after the hook file was renamed from `.github/hooks/docs-validation.json` to `.github/hooks/hooks.json` in this checkout.

Scope:
- update the folder workspace settings to point `chat.hookFilesLocations` at `.github/hooks/hooks.json`
- update the `.code-workspace` file to use the same hook path so both entry modes load the repo hooks
- validate that the configured hook file exists and the settings JSON remains valid

Acceptance criteria:
1. Opening the repo as a folder loads `.github/hooks/hooks.json` via `.vscode/settings.json`.
2. Opening the repo via `context-engine.code-workspace` loads `.github/hooks/hooks.json` as well.
3. No runtime settings surface in this repo still points at the deleted `.github/hooks/docs-validation.json` path.
