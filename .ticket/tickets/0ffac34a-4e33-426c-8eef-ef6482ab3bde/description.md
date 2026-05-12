# Problem

After the Docker validation strategy is defined, the repository still needs a runnable harness that executes the documented installation and deinstallation steps in clean containers and proves that the installed tools behave as documented. That harness also needs to stay synchronized with the specification contracts and the generated README rules rather than becoming a fourth, drifting copy of the install process.

# Scope

Implement a reproducible Docker-based scenario runner for the user-facing install docs.

The implementation should cover:

- container build definitions and runner entrypoints for clean install/deinstall scenarios
- execution of the documented `cargo install` flows for `rule`, `spec`, `ticket`, and `audit`
- smoke verification that installed binaries run and basic documented commands succeed
- deinstallation or cleanup verification that matches the documented user flow
- stable logs and machine-readable failure output that point back to the failing documented step
- a structure that can be extended to viewer installation scenarios later if the design recommends it
- creation or update of `.spec` entries that define the install and deinstall contract for the supported CLI workflows
- wiring the executable Docker scenarios to the same canonical install steps or fixtures referenced by those spec entries
- updating the `.rule` README generation path so the install section is rendered from, validated against, or otherwise mechanically synchronized with the same contract used by the executable tests
- validation checks that fail when the executable scenarios, spec contract text, and generated README install rules diverge

# Acceptance Criteria

- A single documented command runs the install/deinstall validation scenarios inside Docker.
- The harness executes the same user-facing commands or generated command fixtures that the docs advertise.
- The harness verifies installed binaries are usable after installation.
- The harness verifies the documented deinstallation or cleanup path leaves the expected state behind.
- Failures identify the scenario, command step, and relevant doc source clearly enough to fix drift quickly.
- Install and deinstall behavior is captured in canonical `.spec` entries that are updated as part of this work.
- The executable Docker scenarios and the generated README install section are synchronized against the same install contract instead of maintaining separate hand-written command lists.
- A focused validation path exists to prove that `.spec`, executable install tests, and `.rule` README generation remain aligned.