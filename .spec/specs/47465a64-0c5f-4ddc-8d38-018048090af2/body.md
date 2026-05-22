# Motivation

Repository guidance is partly rule-generated today, but nested workspaces still carry hand-written agent files and the parent workspace duplicates child target definitions directly in its own `rule-targets.yaml`.

That duplication weakens ownership boundaries and makes guidance drift more likely because child workspaces cannot be the single source of truth for their own generated outputs.

# Intended Behavior

The rule-target system must allow a workspace to import target definitions from child workspaces and compose them into a parent workspace run without copying those target definitions into the parent config.

Each workspace should define only the smallest set of local target definitions and canonical rules that it owns. Parent workspaces should import and reuse child targets wherever the generated outputs belong to those child workspaces.

Rule-target definitions should be splittable into thematic files under a `rule-targets/` directory so each workspace can organize owned outputs by concern instead of keeping one monolithic config file.

An `imports:` entry should be able to reference either a specific config file or a directory of config fragments. Directory imports must load supported config files in deterministic order so parent workspaces can import a child workspace's themed `rule-targets/` directory directly.

Nested workspace guidance files under `.github/agents/` should be rendered from canonical rule entries instead of remaining hand-written files.

Guidance and prompt surfaces that reference tickets should preserve the exact authoritative ticket folder path returned by `ticket-api` output and append `/ticket.toml` only at markdown-link render time so editors can open a concrete file directly.

# Constraints

- Existing flat `targets` configs must remain supported
- Existing `files` and `folders` tree-shaped target configs must remain supported
- Relative import paths must resolve from the importing config file so nested workspace configs stay relocatable
- Directory imports must ignore non-config files and preserve deterministic ordering across supported config fragments
- Imported targets must retain deterministic ordering and duplicate-name validation across the merged config set
- Frontmatter-based `.agent` outputs must continue rendering without provenance markers when that is the established renderer behavior

# Acceptance Criteria

- A parent `rule-targets` config can import child workspace target configs with relative paths
- A parent `rule-targets` config can import a directory of child config fragments with relative paths
- Loading a config merges local and imported targets deterministically and rejects duplicate target names across the combined config graph
- The root, `memory-viewers`, `memory-api`, and `viewer-api` workspaces all move from single `rule-targets.yaml` files to thematic config fragments under `rule-targets/`
- Root guidance generation reuses imported child targets instead of redefining the same child targets inline
- Child workspaces own the targets for their `.github/agents/*.agent.md` outputs
- Canonical rules exist for the nested workspace `roast` and `Ticket Refinement Agent` files and generate the current agent content correctly
- Regenerating the touched workspaces updates the nested agent files from rule targets without manual edits to those outputs
- Generated guidance and traceability links that reference tickets use targets of the form `<exact authoritative ticket folder path>/ticket.toml` so editor links open a concrete file without rewriting the returned folder path
- Focused tests cover directory import parsing, deterministic fragment ordering, and duplicate handling across imported fragments

# Traceability

- [e4f6e712 [repo-guidance][rule-api] Import child rule-target configs and generate nested workspace agent files](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e4f6e712-b3b6-493a-9ca2-d5f0d91f61b9/ticket.toml)
- [45379405 [repo-guidance][rule-api] Split rule-target configs into thematic folders across nested workspaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/45379405-d7c3-41bf-bd6d-059354c4291b/ticket.toml)
- [5d3cd5da [repo-guidance] Link ticket references to ticket.toml in generated guidance](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/5d3cd5da-99e5-4320-979c-595fedf24a88/ticket.toml)
