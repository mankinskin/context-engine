# Problem

After the tool-history import, the original tool source trees still exist in `context-engine`. Until ownership cleanup is decided and executed, it is ambiguous which repository is the source of truth for those tools and which references should be updated.

# Scope

Plan and execute the post-import cleanup for context-stack-related tools in the source repository.

The work should cover:

- whether the original tool trees remain temporarily, move behind a submodule boundary, or are removed outright
- the order of operations between standalone manifest retargeting and source-repo cleanup
- updates to workspace members, docs, prompts, scripts, and references that still assume the original source location
- any temporary compatibility bridges needed while the ownership move is being completed

# Acceptance Criteria

- The source-of-truth location for each moved tool is explicit.
- The cleanup plan names the sequencing constraints between destination integration and source removal.
- Repository references are updated to the chosen ownership model.
- The cleanup does not silently break the root workspace without an explicit transition plan.
