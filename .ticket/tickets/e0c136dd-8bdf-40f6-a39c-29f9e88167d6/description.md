# Problem

A local Docker harness is not sufficient on its own. The user-facing installation documentation needs continuous validation in CI so documentation drift or broken installation steps are caught before release. That CI coverage also has to protect synchronization between executable install tests, the specification contracts in `.spec`, and the generated README rules in `.rule`.

# Scope

Integrate Docker-based documentation validation into continuous integration.

The CI work should cover:

- a workflow that runs the install/deinstall Docker scenarios on every relevant change
- trigger rules for README updates, generated rule sources, install scripts, Docker harness files, and tool packaging changes
- artifact capture for logs and failure context
- a policy for when the workflow is advisory versus required
- contributor guidance for reproducing CI failures locally
- trigger coverage for spec entry changes under `.spec` and README rule changes under `.rule` or `rule-targets.yaml`
- checks that detect divergence between the executable Docker scenarios, the install/deinstall `.spec` contract, and the generated README targets
- CI artifact output that makes it obvious whether a failure came from executable install behavior, spec contract drift, or README generation drift

# Acceptance Criteria

- A CI workflow runs the Docker install/deinstall validation automatically on relevant changes.
- The workflow surfaces reproducible logs or artifacts for failing scenarios.
- The workflow is wired to the documentation sources that define the user-facing install flow.
- Contributors have a documented local rerun path that matches CI behavior closely.
- The workflow clearly fails when installation docs drift from actual behavior.
- CI covers changes to the install-related `.spec` entries and `.rule` README generation inputs, not just generated README output.
- CI exposes a clear failing check when the executable tests, specification contracts, and generated documentation rules are no longer synchronized.