# Problem

User-facing installation documentation is not validated continuously from a clean environment. The current repo has install instructions for the CLI tools in `memory-api/README.md`, but there is no Docker-based clean-room test strategy for installation, deinstallation, or documentation drift. There is also no canonical install contract in `.spec` that keeps executable tests, specification language, and generated README rule content synchronized.

# Scope

Design a reproducible Docker-based validation strategy for user-facing installation and deinstallation flows.

The design should cover:

- the scenario matrix to validate, including `cargo install` flows for `rule`, `spec`, `ticket`, and `audit`
- whether viewer installation and removal flows should be part of the same matrix or a follow-up track
- how tests will execute documented commands without duplicating the docs by hand
- reproducibility requirements for container images, toolchain pinning, caches, environment variables, and workspace layout
- deinstallation and cleanup assertions, including binaries, local state folders, and generated indices
- failure reporting, artifacts, and contributor ergonomics for local reruns
- the canonical representation of the install contract in `.spec`, including which spec entries define the supported install and deinstall workflow
- how the README generation rules under `.rule` consume or align with that contract so the generated install section stays synchronized with the executable scenarios
- the ownership boundary between executable fixtures, specification contracts, and generated documentation rules when the install flow changes

# Exact Scenario Matrix

## CLI coverage

| ID | Surface | Status | Environment | Install flow under test | Deinstall / cleanup flow under test | Required assertions |
| --- | --- | --- | --- | --- | --- | --- |
| CLI-01 | `rule`, `spec`, `ticket`, `audit` binaries | Required in first implementation | Clean Linux Docker image with pinned Rust toolchain | `cargo install --path tools/cli/<tool>-cli --bin <tool>` from `memory-viewers/memory-api` | `cargo uninstall <tool>` for each installed binary | Binary is on `PATH`; `<tool> --help` exits successfully before uninstall; binary is absent after uninstall |
| CLI-02 | Repo-local root auto-init | Required in first implementation | Fresh workspace checkout with no pre-existing `.rule`, `.spec`, `.ticket`, `.audit` | Run documented repo-root commands: `rule list`, `spec list`, `ticket board show`, `audit run --repo .` | Remove local tool roots created by the test workspace after assertions | Commands succeed without `--index-root`; local roots and folder-local `.gitignore` files are created as documented |
| CLI-03 | Nested-directory discovery | Required in first implementation | Fresh workspace checkout, command invoked from nested subdirectory | Run `rule list`, `spec list`, `ticket board show`, and `audit run --repo ..` or equivalent from a nested package/subdir after root init | Remove test workspace state after run | Tools discover the nearest parent `.rule`, `.spec`, and `.ticket` roots by walking upward |
| CLI-04 | Canonical folder materialization | Required in first implementation | Fresh workspace after local root init | Create the first rule/spec/ticket entry through documented commands | Remove created entries and local state as part of test cleanup | `rules/`, `specs/`, and `tickets/` appear automatically only when first content is added |
| CLI-05 | README workflow smoke path | Required in first implementation | Repo-local workspace with generated install contract | Run `rule sync-targets --config rule-targets.yaml`, `spec refs <spec-id> validate`, `ticket board show`, `audit run --repo .` | Clean generated artifacts in the disposable workspace | The documented common-task commands still work after install and root init |

## Viewer coverage

| ID | Surface | Status | Environment | Install flow under test | Deinstall / cleanup flow under test | Required assertions |
| --- | --- | --- | --- | --- | --- | --- |
| VIEW-01 | `viewer-ctl` binary | Required design decision; implementation may be deferred behind CLI completion | Clean Linux Docker image with pinned Rust toolchain plus frontend prerequisites | `cargo install --path viewer-api/viewer-ctl --bin viewer-ctl` or the documented equivalent | `cargo uninstall viewer-ctl` | `viewer-ctl --help` succeeds; binary can drive viewer lifecycle commands |
| VIEW-02 | Viewer server install lifecycle | Follow-up after CLI matrix is stable | Clean Linux Docker image with frontend build deps | `viewer-ctl install doc-viewer`, `log-viewer`, `ticket-viewer`, `spec-viewer` | No first-class uninstall command exists today; cleanup is manual removal of installed artifacts and stopped processes | Installed viewer artifacts exist where `viewer-ctl` expects them; install command is repeatable |
| VIEW-03 | Viewer start/prepare lifecycle | Follow-up after install lifecycle is covered | Clean Linux Docker image with frontend build deps and available ports | `viewer-ctl prepare <viewer>` and `viewer-ctl start <viewer>` for one representative viewer first, then matrix expansion | `viewer-ctl stop <viewer>` and workspace cleanup | Viewer starts from a clean environment and advertises a reachable port or installed static dir |
| VIEW-04 | Viewer deinstall ergonomics | Explicit non-goal for first implementation; design follow-up required | N/A until uninstall surface exists | None in first implementation | Manual artifact removal only until `viewer-ctl` exposes uninstall/remove support | Design documents the current gap and whether to add an uninstall command or keep deinstall checks out of scope |

## Synchronization rules

1. The CLI matrix is mandatory for the first implementation and is the gating path for CI.
2. Viewer install coverage is split from CLI coverage because `viewer-ctl` has install/start/stop/prepare commands but no explicit uninstall command today.
3. The canonical install contract must live in install-focused `.spec` entries under `memory-api/.spec/specs/**`.
4. The generated install section in `memory-api/README.md` must remain sourced from `.rule` entries and synchronized with the same install contract used by the Docker scenarios.
5. The executable Docker scenarios must validate the documented commands directly or consume generated fixtures derived from the same `.spec` contract rather than maintaining a separate handwritten matrix.

# Acceptance Criteria

- A documented scenario matrix exists for install and deinstall validation of the current user-facing docs.
- The design names the Docker image strategy and reproducibility controls needed for stable CI results.
- The design explains how README command blocks or generated documentation sources will be kept executable and in sync.
- The design identifies the recommended boundary between CLI install validation and viewer install validation.
- The design includes a rollout plan for local developer execution and CI gating.
- The design defines where the canonical install contract lives in `.spec` and how executable tests and `.rule` README generation derive from or validate against it.
- The design names the rule entries and generated README targets that must stay synchronized with the install contract.
- The design records which viewer scenarios are in scope now, which are follow-up work, and why.