# Summary

Remove the remaining live `workflow-cli` prototype surface from the repository.

The corrected workflow architecture is embedded in the ticket/spec/doc layers and future `test-api` and `log-api` work. Canonical rules, generated instructions, and workspace membership should no longer point at the wrapper prototype.

# Why

The repository already rewrote the target specs away from a dedicated wrapper CLI, but one canonical rule entry still instructs agents to use `workflow-cli`, the generated instructions repeat that guidance, and the prototype crate is still a live workspace member.

# Scope

- update the canonical test-execution rule so it no longer references `workflow-cli` or `workflow docs ...`
- regenerate the affected generated instructions from the canonical rules
- remove `tools/cli/workflow-cli` from the root workspace membership
- delete the `tools/cli/workflow-cli` prototype crate directory
- validate that no remaining live repo surfaces point at the wrapper prototype

# Acceptance criteria

- Canonical rules and generated instructions no longer reference `workflow-cli` or `workflow docs ...`.
- The root workspace no longer includes `tools/cli/workflow-cli`.
- The `tools/cli/workflow-cli` directory is removed from the repository.
- Focused validation confirms no remaining live rule, instruction, or workspace surface points at the wrapper prototype.

# Implementation status

- Updated the canonical test-execution rule to remove wrapper-CLI guidance.
- Regenerated `.agents/instructions/tests.instructions.md` from the canonical rule set.
- Removed `tools/cli/workflow-cli` from the root workspace and deleted the prototype crate directory.

# Validation status

- `cargo run -p rule-cli -- sync-targets --config rule-targets.yaml` passed.
- `cargo run -p rule-cli -- sync-targets --config rule-targets.yaml --check` passed.
- Focused grep checks found no `workflow-cli` or `workflow docs` references in the live canonical rule, generated test instructions, or root workspace manifest.

# Documentation status

- The canonical rule and generated instruction text now point agents at direct validation commands and explicit status recording instead of the removed wrapper prototype.