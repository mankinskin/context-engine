## Objective

Relocate the `viewer-api` submodule from `context-engine/viewer-api` to a new top-level `workflow-tools/viewer-api` submodule, following the same cutover pattern `4d6d6301-1001` used for `install-ctl` (canonical source path, no context-engine-specific coupling, context-engine consumes it as an external dependency afterward). Additionally, adapt the existing context-engine Docker install-validation container into a workflow-tools-owned `viewer-validation` container placed next to `install/docker-validation`, and extract the shared Dockerfile/entrypoint/driver-script patterns between the two into one reusable template.

## Requirements

- `workflow-tools/viewer-api` becomes the canonical source location; `context-engine/viewer-api` submodule entry is removed once context-engine is updated to consume it externally (mirroring how `92741a14` removes the nested `workflow-tools` gitlink).
- Every reference to `viewer-api` inside workflow-tools crates/tools (paths, Cargo dependencies, registry entries in `install/artifacts.toml`) uses workflow-tools-relative paths.
- `context-engine/tools/install-validation/Dockerfile.viewer` + `run-docker-viewer-validation.sh` + `run-viewer-in-container.sh` are ported to `workflow-tools/install/viewer-validation/`, updated to build/validate the relocated `workflow-tools/viewer-api` instead of a context-engine-local path.
- Extract the parts common to `install/docker-validation/` and the new `install/viewer-validation/` (base-image selection, build-arg plumbing, build+run driver script shape, diagnostics-on-failure convention) into one shared template (e.g. a common base Dockerfile stage and/or a shared shell library sourced by both `run-docker-*.sh` drivers) so future validation containers reuse it instead of copy-pasting.
- No behavior change to unrelated context-engine viewer functionality; this is a source-location and container-ownership move, not a viewer feature change.

## Non-Goal

Do not redesign the viewer-api crate itself, its Dioxus frontend, or its transport surfaces. Do not merge `install-validation` and `viewer-validation` into a single container — they validate different consumer scenarios and stay as two thin containers built from the shared template.

## Acceptance Criteria

- `workflow-tools/viewer-api` exists as a top-level submodule with full history preserved; `context-engine/.gitmodules` no longer declares `viewer-api`.
- `cargo metadata --format-version 1 --no-deps` from context-engine resolves `viewer-api` externally (Git dependency or installed artifact), not a local path.
- `bash install/viewer-validation/run-docker-viewer-validation.sh` builds and runs a fresh-image validation of the relocated `workflow-tools/viewer-api`, mirroring what `install/docker-validation/run-docker-validation.sh` proves for the minimal ticket/spec consumer.
- The shared template is the single place that defines base-image args and the build+run driver shape; both `run-docker-validation.sh` and `run-docker-viewer-validation.sh` are thin callers of it.
- `install/artifacts.toml` registers `viewer-api`'s relevant binaries/crates with workflow-tools-relative `source_path` entries.

## Validation

Run both Docker validation containers locally after the move and confirm both exit 0. Run `cargo test` for the relocated `viewer-api` crate from its new location. Run `cargo metadata` from context-engine to confirm no local path dependency on `viewer-api` remains.