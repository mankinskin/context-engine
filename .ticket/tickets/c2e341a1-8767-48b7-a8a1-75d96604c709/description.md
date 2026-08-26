## Objective

Publish `workflow-tools/install.sh`, the public commit-pinned `curl | bash` entry point required by [01-install-contract-and-skill.md]. It installs `install-ctl` into a caller-supplied local workspace directory (never a global executable directory), then hands off to the existing `install-ctl` interactive TUI for tool/instruction/hook selection and installation-home choice.

## Requirements

- Fetched via a single `curl -fsSL <raw-github-url-at-commit> | bash` command; the script pins its own source commit and only installs the same commit's `install-ctl`.
- Installs `install-ctl` into a local workspace directory (e.g. `./.workflow-tools-bootstrap/bin` or a caller-supplied `--root`), never a system-wide bin directory.
- Delegates the ticket/spec CLI bundle installation and consumer workspace initialization to the existing `bootstrap.sh` contract rather than re-implementing it.
- Fails with a clear, non-zero-exit error if `--root`/target directory or `--workspace` is missing, mirroring `bootstrap.sh`'s explicit-selector requirement.
- Documented in `workflow-tools/README.md` alongside the existing `bootstrap.sh` instructions.

## Non-Goal

Do not implement the `install-ctl` TUI itself, tool/instruction/hook selection logic, or skills.sh publication — those are separate subtasks.

## Acceptance Criteria

- A fresh temporary directory can run `curl -fsSL <pinned-commit-url>/install.sh | bash -s -- --root <dir> --workspace <dir>` and end up with an executable `install-ctl` under `<dir>`.
- `bash -n install.sh` passes (syntax check), and a `--dry-run` (or equivalent) flag previews the pinned commands without installing.
- README documents the one-command public entry point next to the existing `bootstrap.sh` section.

## Validation

Run the script against a scratch directory locally (simulating the `curl | bash` flow with a local file read instead of a network fetch) and confirm `install-ctl --help` succeeds from the installed path.