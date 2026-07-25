Migrate all `.agents/agents/*.agent.md` files off the rule-store generator to hand-owned by-description files, and retire the agents rule-target. Follow-up to CH-D (f43cb5cb) which established the migration pattern for the 12 instruction files.

Anchor spec: agents/skill-infrastructure (a9b7ef39) — extends AC5 to the agent-mode surface. Runs AFTER the instruction migration (CH-D) so the parity-gate + target-deletion procedure is already proven.

Scope:
- 1:1 conversion now (keep the current file boundaries); refine into finer-grained by-description files incrementally later.
- Content-parity gate: seed each hand-owned file from its current generated content, then git-diff to confirm no guidance is lost before deleting the target.
- Delete the agents target `rule-targets/45-agents-agents.yaml`.
- Strip `rule-api:file generated=true` and `rule-api:entry` headers from every `.agents/agents/*.agent.md` file.
- Remove any stale self-reference notes that claim the file is generated from a .rule entry.

Files (11): audit, commit, default, implement, interview, research, review, roast, spec, testing, ticket-refinement.

Acceptance criteria (verifiable):
- AC-1: No file under `.agents/agents/` contains `rule-api:file generated=true` or `rule-api:entry`.
- AC-2: `rule sync-targets` (or `rule generate`) no longer regenerates any `.agents/agents/**` file (target removed).
- AC-3: git-diff parity record shows seed-from-current content preserved (no guidance dropped) before target deletion.
- AC-4: Each migrated agent file has accurate frontmatter for agent-mode selection.