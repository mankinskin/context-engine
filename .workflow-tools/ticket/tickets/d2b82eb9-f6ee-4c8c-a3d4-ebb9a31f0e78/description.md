Run the new audit link-coherence rule (ticket a880d8ad) across the full guidance corpus in every repo with a .agents/ tree, and fix every reported broken link by pointing it at its correct relocated target (or removing the link with a one-line note if the target was genuinely deleted, not moved).

Context / origin: dossier transcripts/09-09-2026_guidance-link-audit-hook/ROADMAP.md (waypoint W3). Two concrete known examples to fix as part of this ticket (see ARTIFACTS.md in that dossier):
- workflow-tools/.agents/instructions/repository/generated-files.instructions.md: `[AGENTS.md](../../../AGENTS.md)` -> should point at the correct existing AGENTS.md (resolve which one is intended; context-engine/AGENTS.md is the most likely target given repository-wide-principles framing).
- Multiple files (loop-closure.instructions.md, question-quality.instructions.md, escalation-gate.instructions.md, interview.agent.md, and any others the rule finds) reference `.agents/instructions/orchestration/...` which no longer exists after a rename to `.agents/instructions/workflow/...` — update every such reference to the workflow/ path.

Acceptance criteria:
- `audit run .`'s link-coherence rule reports zero broken links across every repo with a .agents/ tree after this ticket's changes.
- No link's destination content was rewritten, only its path corrected (or the link removed with a note, when the target is genuinely gone).
- A follow-up run of the new pre-commit hook (ticket 4c87429d) passes cleanly on the fixed corpus.

Depends on: a880d8ad (needs the rule to enumerate broken links).