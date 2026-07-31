## Problem
`main.rs` spawns the child directly from its resolved PATH location, which holds a Windows file lock (`os error 5`) on `~/.cargo/bin/<server>.exe` for the process lifetime, blocking `cargo install --force`. Part of epic 25780944; depends on T1 (rename), T2 (policy split, so this slots into the lifecycle-neutral core).

## Approach
On startup, resolve the child binary's canonical path P (from the `--` args, PATH lookup). Copy P to a private shadow path S under a temp dir keyed by name+pid+hash (e.g. `%TEMP%/mcp-toolmon/<name>-<pid>-<hash>/<exe>`). Spawn S instead of P. Clean up shadow copies on normal exit. This alone fixes the Windows lock problem even without reload (T4-T6).

## Acceptance criteria
- [ ] Canonical child path P resolved once at startup (PATH lookup honoring existing arg-parsing in `main.rs`)
- [ ] Shadow copy created at a private path keyed by name+pid+hash before spawn
- [ ] Child process spawned from the shadow path S, not P
- [ ] P is never open/locked by mcp-toolmon at any point after the copy completes (verified: `cargo install --force` targeting P succeeds while mcp-toolmon runs)
- [ ] Shadow directory removed on graceful process exit; stale shadow dirs from crashed processes do not accumulate unbounded (best-effort cleanup or documented TTL)
- [ ] `TOOLMON_SHADOW_DIR` env var overrides the default shadow root (used later by T6 too, defined here since this ticket owns shadow-copy mechanics)
- [ ] Existing spawn behavior (piped stdin/stdout, inherited stderr) preserved
- [ ] Unit test: shadow copy is created and spawned binary's actual path differs from P
- [ ] Integration/manual note: Windows lock scenario documented as verified (can be deferred to T7 for full automated coverage)

## Files touched
- memory-api/tools/mcp/mcp-toolmon/src/main.rs
- memory-api/tools/mcp/mcp-toolmon/src/shadow.rs (new)
- memory-api/tools/mcp/mcp-toolmon/tests/ (new shadow-copy unit tests)