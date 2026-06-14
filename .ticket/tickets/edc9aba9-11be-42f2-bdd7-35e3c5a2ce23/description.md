The ticket-vscode extension can list tickets from the wrong workspace when the ticket-viewer server returns canonical workspace ids that do not match VS Code folder names. In the current workspace it falls back to the first server-reported workspace and shows the five tickets from context-stack/tools/context-editor/.ticket instead of the root context-engine ticket store.

Acceptance criteria:
- When the open VS Code folder contains a local .ticket root, the extension resolves the matching server workspace instead of defaulting to the first workspace name.
- When the server exposes canonical ids such as path-based names or shared-- ids, the extension uses server labels and/or local .ticket paths to preserve the intended selection.
- Unit coverage exercises the mismatch case so the extension does not regress to listing tickets from a sibling or descendant workspace.