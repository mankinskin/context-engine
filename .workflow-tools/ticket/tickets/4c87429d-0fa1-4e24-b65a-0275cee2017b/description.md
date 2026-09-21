Install a hard-blocking Git pre-commit hook in every repo that has a .agents/ tree (meta-workspace root, context-engine/, workflow-tools/, and nested sub-crates: workflow-tools/audit, workflow-tools/session, workflow-tools/spec, workflow-tools/ticket, workflow-tools/test) that runs the new audit link-coherence rule (ticket a880d8ad) against staged Markdown files and rejects the commit when a broken guidance link is introduced.

Context / origin: dossier transcripts/09-09-2026_guidance-link-audit-hook/ROADMAP.md (waypoint W2). Requester-confirmed decision (see REVIEW.md finding #3/#4 in that dossier): hard-block, full repo-wide rollout in v1. `--no-verify` remains the existing documented escape hatch per workflow-tools/.agents/instructions/repository/pre-commit.instructions.md.

Acceptance criteria:
- Each target repo's pre-commit hook invokes the audit link rule scoped to staged Markdown files under that repo's own .agents/ tree (and AGENTS.md/SKILL.md files) before allowing the commit.
- A commit introducing a new broken link is rejected with a clear message naming the file and broken link.
- A commit that does not touch guidance Markdown files is unaffected (no added hook latency of consequence for unrelated commits).
- `pre-commit.instructions.md` is updated to document the new check alongside the existing checks it already lists.
- Existing context-engine/.githooks/pre-commit is extended or a matching hook is added consistently for the other repos, rather than diverging patterns per repo.

Depends on: a880d8ad (link-coherence rule must exist before it can be wired into a hook).