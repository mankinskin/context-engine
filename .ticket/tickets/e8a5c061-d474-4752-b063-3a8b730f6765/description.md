# Repository-level dependency-cycle check for crate extraction

Document a mandatory pre-extraction dependency-cycle check that inspects cycles across repository boundaries, not only a single Cargo crate graph. The guidance must record two concrete discoveries:

- The resolved repository-level cycle in which `ticket-api` depended on the legacy base while memory-api leaf crates depended on `ticket-api`.
- The still-open `test-cli` -> `log-api` -> `test-api` cycle, which currently blocks extracting the test and log tools.

The check is needed because both cycles surfaced only during extraction work. A crate-local graph cannot expose the repository-level `ticket-api`/legacy-base cycle, and the second cycle remains an active architectural blocker. Before test or log extraction is attempted, the guidance must require an explicit remediation approach.

Target file: create `.agents/instructions/engine/workflow-tool-extraction.instructions.md` with the pre-extraction check and blocker-recording procedure. Reference epic `69eb4118` and ticket `858c5286`.

Placement options and consequences: adding the rule to `.agents/instructions/commit/branch-worktree.instructions.md` would place the requirement near integration procedure but would bury architecture screening in a 268-line worktree lifecycle document. Adding the rule to `.agents/instructions/commit/submodule.instructions.md` incorrectly suggests that the check applies only to submodules. A dedicated workflow-tool extraction instruction makes the check discoverable before a tool split and preserves a clear boundary between extraction architecture and branch mechanics.