# Why

Ticket 8f94b367 fixed the documented bootstrap path, but every completed check ran on an already-provisioned developer machine. Binaries and domain stores were already present, so those passing checks do not prove a genuinely fresh clone works.

The one real fresh-clone attempt during ticket 8f94b367 could not complete: the superproject records memory-api commit df2164aa, but df2164aa existed only locally and not in the configured remote. Retrying with locally rewritten submodule URLs surfaced three further defects:

1. init.sh initialized ticket, spec, and rule stores relative to the caller directory instead of the repository root.
2. install-deps.sh treated trunk, cargo-llvm-cov, and cargo-make as required failures even though the bootstrap can continue without those optional tools.
3. tools/verify-bootstrap.sh ignored CARGO_INSTALL_ROOT and listed audit twice instead of checking audit-mcp.

# Required work

Run a genuine end-to-end test in an isolated container: start from a clean OS image with no preinstalled Cargo binaries and no populated domain stores, clone from the real remote, then execute the documented Getting Started sequence in order:

```bash
bash setup_git.sh
./install-deps.sh
./install-tools.sh --mcp
bash init.sh
bash tools/verify-bootstrap.sh
```

Every command must complete without errors.

# Acceptance criteria

- The container run starts from an image with no repository tooling preinstalled and every documented bootstrap step exits 0.
- tools/verify-bootstrap.sh is green after the sequence completes.
- The test is runnable on demand through a committed script or CI job, rather than a one-off manual run.
- Every defect found by the container test is fixed or filed as its own ticket.

# Known prerequisite

Every submodule commit required by the superproject must be reachable from the configured remotes. Without reachable submodule commits, a fresh clone cannot check out; this prerequisite blocked the first fresh-clone attempt for ticket 8f94b367.