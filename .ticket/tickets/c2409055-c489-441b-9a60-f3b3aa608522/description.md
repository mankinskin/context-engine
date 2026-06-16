Build a workspace summary capability locally inside each tool/domain (e.g. ticket-cli, spec-cli, rule-cli). Under this contract, each store folder (like `.ticket/` or `.spec/`) acts as the root anchor for its tool execution and contains its own workspace index, config folder, and child/parent workspace lookup.

## Scope
- Implement workspace configuration folders locally inside each tool's workspace root (e.g., `.ticket/.config/`, `.spec/.config/`).
- Each workspace is a node in a DAG with **multiple parents and multiple children** (D9), indexing each parent and child workspace's name and file path — not assuming a single global workspace list.
- Each store workspace serves as the root anchor for tool executions, enabling referencing or even importing of other workspace stores using relative paths.
- The local summary indexes its own domain metadata (e.g. freshness, counts, health) and outputs a localized workspace `IndexEntry` with `ContentKind::workspace_summary` inside its workspace folder.
- Across the file tree (D1): folder-level READMEs act as index entries, the store workspace folder holds the workspace-level index, and an `.agents/` agent-hook entry exposes the workspace summary to agent clients.

## Acceptance criteria
- Tool execution resolves the store root as the execution anchor.
- The store workspace folder contains a config folder (e.g. `.ticket/.config/`) holding multiple parent and child workspace references (DAG edges with names + relative paths).
- Running the workspace overview from ticket-cli produces an isolated report under `.ticket/README.md` containing only ticket-domain summaries and workspace configuration links, without coupling to global state.
- Cross-workspace referencing/importing resolves via the relative paths recorded in the config folder.
- An `.agents/` agent-hook entry is emitted for the workspace summary.

## Non-goals
- Does not build a single global index store.
- Does not cross-compile different store engines into a single binary.

## Resolved design decisions
- D9: workspaces are DAG nodes with multiple parents/children, each with a tool config folder indexing parent/child names + locations; each workspace is the root anchor for tool execution and can reference/import other workspace stores.
- D1: folder READMEs + workspace-folder index + `.agents/` hook. D5: committed to git.