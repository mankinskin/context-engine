## Problem

`AGENTS.md` lists `CHEAT_SHEET.md` under `Canonical Sources` and cites the file in the Discovery Protocol for type-level gotchas and common patterns. `.github/copilot-instructions.md` also references `CHEAT_SHEET.md`. No `CHEAT_SHEET.md` exists at the repository root.

Every agent following the documented discovery workflow reaches a dead reference, so the guidance is not executable.

## Required State

Choose and record one of two approaches before implementation:

1. Create root `CHEAT_SHEET.md` with real, maintained content that satisfies the documented type-level-gotcha and common-pattern purpose. This preserves the guidance but creates a documentation maintenance obligation.
2. Remove or redirect every `CHEAT_SHEET.md` reference to an existing authoritative source. This removes the dead reference but changes the discovery workflow.

After the decision, apply the selected approach consistently across `AGENTS.md` and `.github/copilot-instructions.md`, and record the decision in the ticket implementation notes.

Related guidance-cleanup context: ticket `0b527d28`.
