# Problem

Root guidance generation still duplicates child workspace target definitions, and several nested-workspace guidance files under `.github/agents/` are still hand-written instead of being rendered from canonical rules.

That leaves three problems in place:
- parent workspaces redefine child targets instead of importing them
- child workspaces do not own the minimal target definitions for their own generated guidance files
- nested `.github/agents/*.agent.md` files are not synchronized from canonical rule entries

# Scope

Implement nested rule-target config imports so parent workspaces can reuse child workspace target definitions, then migrate the remaining nested workspace agent files to canonical rule-backed generation.

# Acceptance Criteria

- `rule-api` supports loading a rule-target config that imports one or more child workspace target configs
- imported target configs can be organized in nested file or module structures without breaking existing flat `targets` support
- the root `rule-targets.yaml` imports child workspace target definitions instead of redefining the same child targets inline
- each child workspace keeps only the minimal local target definitions it owns
- the remaining nested workspace guidance files under `.github/agents/` are generated from canonical rule entries and no longer rely on hand-written source content
- the imported canonical rules preserve the existing agent behavior and metadata after regeneration
- focused rule generation validation passes, including `sync-targets` and `sync-targets --check` for the touched workspaces
