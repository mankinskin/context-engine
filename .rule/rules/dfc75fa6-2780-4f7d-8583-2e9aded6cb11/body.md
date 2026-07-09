### Workspace Discovery vs Enumeration (note to self)

Do not assume a nested `.ticket` store is "excluded" just because it is absent from `list_workspaces`.

- **Discovery/reads are aggregated.** When the root `.ticket/workspace-policy.toml` sets `include_descendants = true`, every descendant store (for example `memory-api`, `memory-viewers`, `viewer-api`, `context-stack`) is recursively discovered and folded into the aggregated `default` workspace index. `get`, `list`, `next`, `search`, and graph/health queries against `default` therefore already include descendant tickets, and `get` returns each ticket's real owning-store path.
- **`list_workspaces` only enumerates the aggregated root.** It reports `default` plus the root store path and does **not** list each descendant store as a separately selectable workspace. A store missing from `list_workspaces` is an enumeration/presentation limitation, not a discovery exclusion.
- **Writes target the addressed store.** `create` and `update` land in the store that owns the `workspace` you pass. To co-locate a new ticket with a descendant subtree (for example ticket-viewer / viewer-api work under `memory-viewers`), pass that store's absolute path as `workspace` instead of `default`.
- **Before concluding a store is unreachable:** confirm with `get`/`list` against `default` and inspect `.ticket/index.toon` `source_path` prefixes; only treat it as a real gap if the descendant tickets are genuinely absent from the aggregated index.
