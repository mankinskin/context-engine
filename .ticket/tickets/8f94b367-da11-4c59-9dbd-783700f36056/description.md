# Objective

A first-time user must be able to go from a fresh clone to a fully usable repository in a minimal, documented sequence of steps, with **zero error messages** along the way.

This ticket is the durable anchor for that goal. It was created during the first fresh-eyes read of this repository (session `b3bd7c08-7dcc-4ba6-9718-05bff1365cf7`, 2026-08-12) by actually executing the documented onboarding path and recording every point of friction.

# Method

Executed the documented onboarding path as a first-time user would, top-down from README.md, and recorded each failure or ambiguity.

# Findings

## F1 — README's very first command is broken (`peek` not installed)

README.md's "Repository Map" section instructs:

```
peek repo_map.toon --grep "crates"
```

Actual result:

```
bash: peek: command not found
```

Root cause: `install-tools.sh` has `peek-mcp` in both `mcp_tool_names` and `tool_names`, but **`peek-cli` is in neither**. The package exists at `memory-api/tools/cli/peek-cli/Cargo.toml`. So the very first documented command in the repository fails for every fresh user, and the bounded-inspection workflow mandated by `.agents/instructions/orchestration/file-inspection.instructions.md` is unavailable.

## F2 — `rule` CLI is installed but its store is never initialized

```
$ rule list
rule error: workspace not initialized at .../.rule: run the 'init' command to create a new workspace
```

`.rule/` is absent while `.ticket`, `.spec`, `.audit`, `.session`, `.test`, `.feedback` all exist. `rule` is installed by `install-tools.sh` (`rule-cli`) and referenced by `.github/copilot-instructions.md` as self-bootstrapping, but nothing in the bootstrap path runs `rule init`.

## F3 — Bootstrap scripts are untracked and undocumented

`init.sh` and `setup_git.sh` exist in the repository root but are **untracked** (`??` in `git status`) and are **not mentioned anywhere in README.md**.

`init.sh` contents:

```bash
copilot init
ticket init
spec init
```

It is also incomplete: it omits `rule init` (see F2) and does not cover the `.test`, `.feedback`, `.audit`, or `.doc` stores.

`setup_git.sh` contents:

```bash
git submodule update --init --recursive
```

This duplicates the README's "Working With Submodules" snippet instead of being referenced by it.

## F4 — Several workspace CLIs are never installed

`install-tools.sh` `tool_names` omits these existing CLI packages:

- `peek-cli` (see F1)
- `test-cli`
- `context-cli`
- `fs-cli`
- `compact-terminal-cli`

`install-ctl` and `worktree-ctl` are present on PATH but are also absent from `tool_names`, so their installation route is implicit rather than documented.

## F5 — `cargo-make` is required by instructions but not installed

`.github/copilot-instructions.md` states "prefer `cargo make <task>`" and `Makefile.toml` defines the canonical build/install/start/stop tasks for every viewer. `cargo-make` is not on PATH and neither `install-deps.sh` nor `install-tools.sh` is documented as providing it.

## F6 — README has no ordered "first run" section

README.md presents the repository map, then installable tools, then submodules, then validation. It never states the **order of operations** for a fresh clone. A new user cannot tell whether to init submodules before or after installing tools, or that store initialization is required at all.

# Positive findings (no action needed)

- `git submodule status` shows all five submodules initialized and on `heads/main`.
- `cargo check --workspace` completes clean in ~39s with only 2 pre-existing warnings (`parse_uuid_field` never used in `ticket-cli`; unused imports in `memory-matrix`). No compilation errors.
- `ticket next` works out of the box and returns a correctly ordered work queue.
- Worktree auto-provisioning fired correctly for this session.

# Acceptance Criteria

1. A single documented command sequence takes a fresh clone to a fully working state, and README.md states that sequence in order, at the top.
2. Every command appearing in README.md executes successfully on a fresh clone — specifically `peek repo_map.toon --grep "crates"` succeeds.
3. `install-tools.sh` installs every CLI a documented workflow depends on, including `peek-cli`.
4. `rule list` succeeds on a fresh clone (store initialized by the bootstrap path).
5. `init.sh` and `setup_git.sh` are either committed and referenced from README.md, or removed and their content folded into the documented bootstrap.
6. The prerequisite set (including `cargo-make`) is stated explicitly, and installing it is part of the documented bootstrap.
7. A verification command exists that asserts the bootstrapped state is complete, so the "no errors" property is testable rather than asserted.


## Bootstrap status
- F1 resolved: [install-tools.sh](install-tools.sh) now installs `peek-cli`; `peek repo_map.toon --grep "crates"` succeeds.
- F2 resolved: [init.sh](init.sh) now runs `rule init` and creates `.doc`.
- F3 resolved: [init.sh](init.sh) and [setup_git.sh](setup_git.sh) are now tracked and documented in [README.md](README.md); `setup_git.sh` also runs `tools/checkout-submodule-branches.sh`.
- F4 resolved: [install-tools.sh](install-tools.sh) now includes `peek-cli`, `test-cli`, `fs-cli`, `compact-terminal-cli`, `context-cli`, `install-ctl`, and `worktree-ctl`; `./install-tools.sh --dry-run` reports `requested=29, succeeded=29, failed=0`.
- F5 resolved: [install-deps.sh](install-deps.sh) now installs `cargo-make`.
- F6 resolved: [README.md](README.md) now has an ordered Getting Started sequence and removes the duplicate submodule bootstrap commands from the later section.
- Verification: [tools/verify-bootstrap.sh](tools/verify-bootstrap.sh) now checks all 29 binaries, all 8 domain stores, and workspace compilation.
- Validation: `bash tools/verify-bootstrap.sh --skip-check` passed; a PATH-stripped negative run failed as expected and listed the missing binaries; restoring PATH passed again; `cargo make verify` passed; `cargo check --workspace` reported 0 errors.



# Closing status

- F1-F7 are resolved, including the mcp-toolmon proxy fix that preserves session_id for session-mcp calls and was verified live.
- Performed validation: tools/verify-bootstrap.sh --skip-check passed; a forced-negative PATH-stripped run failed as expected and listed missing binaries; restoring PATH passed again; cargo make verify was green; cargo check --workspace completed with 0 errors; and README.md's first peek command works.
- Full containerized fresh-clone validation was deliberately deferred by the user to ticket d9e1c624-d422-4c1e-b221-daef5a557765, [onboarding] Verify fresh-clone bootstrap end-to-end in an isolated container. The deferral is not a completed fresh-clone proof.