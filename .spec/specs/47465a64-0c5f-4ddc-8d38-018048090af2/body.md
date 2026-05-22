# Motivation

Repository guidance is partly rule-generated today, but nested workspaces still carry hand-written agent files and the parent workspace duplicates child target definitions directly in its own `rule-targets.yaml`.

That duplication weakens ownership boundaries and makes guidance drift more likely because child workspaces cannot be the single source of truth for their own generated outputs.

# Intended Behavior

The rule-target system must allow a workspace to import target definitions from child workspaces and compose them into a parent workspace run without copying those target definitions into the parent config.

Each workspace should define only the smallest set of local target definitions and canonical rules that it owns. Parent workspaces should import and reuse child targets wherever the generated outputs belong to those child workspaces.

Nested workspace guidance files under `.github/agents/` should be rendered from canonical rule entries instead of remaining hand-written files.

# Constraints

- Existing flat `targets` configs must remain supported
- Existing `files` and `folders` tree-shaped target configs must remain supported
- Relative import paths must resolve from the importing config file so nested workspace configs stay relocatable
- Imported targets must retain deterministic ordering and duplicate-name validation across the merged config set
- Frontmatter-based `.agent` outputs must continue rendering without provenance markers when that is the established renderer behavior

# Acceptance Criteria

- A parent `rule-targets` config can import child workspace target configs with relative paths
- Loading a config merges local and imported targets deterministically and rejects duplicate target names across the combined config graph
- Root guidance generation reuses imported child targets instead of redefining the same child targets inline
- Child workspaces own the targets for their `.github/agents/*.agent.md` outputs
- Canonical rules exist for the nested workspace `roast` and `Ticket Refinement Agent` files and generate the current agent content correctly
- Regenerating the touched workspaces updates the nested agent files from rule targets without manual edits to those outputs
- Focused tests cover config import parsing and duplicate handling across imports

# Traceability

- [e4f6e712 [repo-guidance][rule-api] Import child rule-target configs and generate nested workspace agent files](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e4f6e712-b3b6-493a-9ca2-d5f0d91f61b9)
