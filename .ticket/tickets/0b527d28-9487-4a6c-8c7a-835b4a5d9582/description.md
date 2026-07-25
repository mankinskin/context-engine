Phase F. Produce the migration guide and dependency-install documentation so the new structure is reproducible and the general framework is documented: how a target project installs workflow-tools, how the per-tool repos relate, and how the workflow-skill bootstraps everything.

## Scope
- Migration guide covering the before/after structure and the moves performed.
- Install docs: how a target environment consumes workflow-tools as a dependency.
- Document the general framework (target environment vs tooling vs generated artifacts) using context-engine as the worked example.
- Update root READMEs across affected repos.

## Acceptance criteria
- Migration guide + install docs published and validated (doc validation where available).
- context-engine documented as the instantiated example of the general framework.
- Cross-links between workflow-tools, tool repos, and workflow-skill are correct.

## Dependencies
- Blocked by end-to-end validation.