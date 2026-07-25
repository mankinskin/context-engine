Phase C. Create the `workflow-tools` umbrella repository that aggregates the extracted shared libs (`memory-kernel`, `viewer-api`, `memory-fixtures`) and the 11 per-tool repos as dependencies (submodules and/or versioned deps), forming the single installable workflow-tooling bundle that a target project consumes.

## Scope
- Initialize `workflow-tools` with an aggregation manifest referencing every tool + shared repo.
- Provide a build/install entry that pulls and wires all tools (the dependency any target environment installs).
- Establish the repo-level guidance entry point (AGENTS.md) that points into the workflow-skill.

## Acceptance criteria
- `workflow-tools` resolves and builds all aggregated repos.
- A documented install path exists for a target project to depend on workflow-tools.
- Repo-level entry point references the workflow-skill guidance.

## Dependencies
- Blocked by per-tool extraction (tool repos must exist) and foundations.