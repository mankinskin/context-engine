## Problem
5 orchestration instruction files alone total 16,898 tokens of unconditionally-relevant guidance. Once C1 (routing contract) and C2 (consolidated templates) land, several of these files' rules become redundant with the new templates and can be condensed.

## Scope
- Wait for C1 and C2 to land (this ticket determines which orchestration rules are still load-bearing only after the new contract exists — "which rules pay" per epic notes).
- Audit the 5 files under `.agents/instructions/orchestration/` for overlap with the new role taxonomy, routing table, and consolidated templates.
- Compress into a single instruction file, preserving every rule that still has no home in the new templates; drop rules now fully covered by template content.

## Affected paths
- .agents/instructions/orchestration/*.instructions.md (5 files, exact list to be confirmed at execution time from current directory contents)

## Acceptance criteria
- [ ] Single compressed orchestration instruction file replaces the 5 originals
- [ ] No rule silently dropped — each removed rule is either now covered by a C2 template or explicitly noted as obsolete
- [ ] Token count of the compressed file measured and reported against the 16,898-token baseline
