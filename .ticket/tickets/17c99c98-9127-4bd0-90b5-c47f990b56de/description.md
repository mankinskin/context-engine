# Problem

`crane-cli` now exists and has shown the intended migration shape, but future migrations still need a repeatable verification workflow before production use. Right now the proof is split across crate tests, one real dry run, and one successful live import.

# Scope

Define and harden the standard verification flow for future `crane-cli` migrations.

The work should cover:

- focused crate-level validation for the streaming import path
- at least one production-like temp-repo fixture that exercises combined path filtering and remapping
- a dry-run review checklist for real source/target repositories
- the operator metadata that must be reviewed before a live import: mappings, anchor commit, computed range, target branch, import branch
- the post-import inspection checklist for target history, target cleanliness, and imported path coverage
- where this workflow is documented and how it stays in sync with the tool behavior

# Acceptance Criteria

- A single documented verification workflow exists for future `crane-cli` migrations.
- The workflow includes focused test coverage plus a real-source dry-run review step.
- The documented dry-run review names the exact metadata to inspect before a live import.
- The post-import inspection checklist includes target repo history and cleanliness checks.
- The verification workflow is reflected in the relevant `.spec` entry or linked documentation.
