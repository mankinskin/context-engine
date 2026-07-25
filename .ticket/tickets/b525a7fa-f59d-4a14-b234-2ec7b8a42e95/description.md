Phase C. Create the `workflow-tools` umbrella repository that aggregates the extracted shared libs (`memory-kernel`, `viewer-api`, `memory-fixtures`) and the 11 per-tool repos as dependencies (submodules and/or versioned deps), forming the single installable workflow-tooling bundle that a target project consumes.

Per the domain-crate contract (`0da6894c`), the dependency unit for each tool is its single domain crate (`{domain}`); the umbrella depends on that crate and re-exposes/passes through its transport binaries (`{domain}-cli`, `{domain}-mcp`, `{domain}-http`) so installers get the same interface tools.

## Scope
- Initialize `workflow-tools` with an aggregation manifest referencing every tool domain crate + shared repo.
- Provide a build/install entry that pulls and wires all tools (the dependency any target environment installs), exposing the transport bins.
- Establish the repo-level guidance entry point (AGENTS.md) that points into the workflow-skill.

## Acceptance criteria
- `workflow-tools` resolves and builds all aggregated domain crates and exposes their transport bins.
- A documented install path exists for a target project to depend on workflow-tools.
- Repo-level entry point references the workflow-skill guidance.

## Dependencies
- Blocked by per-tool extraction (tool domain crates must exist) and foundations.