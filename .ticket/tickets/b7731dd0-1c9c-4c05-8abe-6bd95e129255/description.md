## Gap

`.agents/agents/live-validation.agent.md` has Constraints at line 29 but has no proof contract for a cross-repository dependency resolving from its remote source.

## Session Evidence

A green `cargo build --workspace` was treated as proof that a remote git dependency resolved. A root Cargo `[patch]` override masked the dependency source; the actual proof was the `Cargo.lock` source line.

## Required Corrected State

Add a Constraints rule: for remote dependency validation, disable the root Cargo `[patch]`, run the workspace build, and read back the `Cargo.lock` source line as the artifact of record. A green build with the patch active is explicitly insufficient. This work depends on the cross-repo dependency policy being written down first.