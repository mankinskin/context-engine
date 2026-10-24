# context-engine guidance router

This Cargo superproject contains independently buildable engine, storage and
viewer repositories. Before engine/build/frontend work, read the relevant
section of the [repository reference](../.agents/copilot-instructions-reference.md);
do not preload the whole reference or every nested guide at session start.
The reference preserves the original architecture, commands and conventions;
its old workflow/discovery links are historical. Use the current owners below
for behavioral contracts.

- Workflow operations and branch isolation: read
  [Key Conventions](../.agents/copilot-instructions-reference.md#key-conventions).
  Use the current owning repository's domain instructions when installed;
  missing tools or guidance are explicit blockers, not permission to invent
  lifecycle operations.
- Kernel architecture: read
  [Repository Shape](../.agents/copilot-instructions-reference.md#repository-shape)
  and resolve the kernel checkout before following repository-specific links.
- Guidance changes: preserve canonical sources, regenerate only through a
  verified owning tool, and check the resulting links before publication.
- Shell/build commands: prefix commands with `rtk`; scope Cargo checks with
  `-p` before considering workspace-wide execution.
- Viewer changes: run the affected frontend's checks and browser path; consult
  the reference for the build system before choosing commands.

Check file ownership before edits; isolate overlapping work. Preserve unrelated
changes. Use domain tools rather than hand-edit entity records. Stage explicit
paths, commit nested repositories first, and do not hand-edit generated
projections. Record actual validation and blockers, not inferred success.
