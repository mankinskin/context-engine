# Cross-repository dependency and patch override policy

Create `.agents/instructions/commit/cross-repo-dependencies.instructions.md` as the repository-wide policy for dependencies that move into another repository. The instruction must state all of the following:

1. Committed cross-repository Cargo dependencies use a `git = "https://..."` URL with `branch = "main"`.
2. The root `[patch]` section is a local-development override only. A local patch must never be the condition that makes a build pass.
3. Patch-free verification disables the root `[patch]`, runs `cargo build --workspace`, then asserts that the dependency source in `Cargo.lock` is `git+https://...#<commit>`.
4. The documented procedure must include the known pitfall: after the patch is disabled, `cargo update -p <pkg>` may fail to select the package and is not a valid repair for the proof.
5. A migration remains incomplete until the dependency's remote `main` branch is pushed and the patch-free `Cargo.lock` proof exists.

The policy is needed because an active `[patch]` masked remote dependency resolution: a green `cargo build --workspace` provided no evidence that the git source worked. The first verification attempt also failed when `cargo update -p memory-kernel` could not select the package after the patch was removed.

Do not add the policy to `.agents/instructions/engine/core-crates.instructions.md`. That 38-line instruction explicitly scopes its frontmatter to context-engine core crates (`trace/search/insert/read/api`), so adding cross-repository Cargo policy there would hide a repo-wide rule behind an incorrect scope. A new commit-scoped instruction is discoverable beside `.agents/instructions/commit/submodule.instructions.md` while avoiding the false implication that git dependencies are submodules.