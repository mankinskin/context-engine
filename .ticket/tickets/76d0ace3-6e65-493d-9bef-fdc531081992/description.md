Migrate all `.agents/prompts/*.prompt.md` files off the rule-store generator to hand-owned by-description files, and retire the prompts rule-target. Follow-up to CH-D (f43cb5cb) which established the migration pattern for the 12 instruction files.

Anchor spec: agents/skill-infrastructure (a9b7ef39) — extends AC5 to the prompt surface. Runs AFTER the instruction migration (CH-D) so the parity-gate + target-deletion procedure is already proven.

Scope:
- 1:1 conversion now (keep the current file boundaries); refine into finer-grained by-description files incrementally later.
- Content-parity gate: seed each hand-owned file from its current generated content, then git-diff to confirm no guidance is lost before deleting the target.
- Delete the prompts target `rule-targets/30-agents-prompts.yaml`.
- Strip `rule-api:file generated=true` and `rule-api:entry` headers from every `.agents/prompts/*.prompt.md` file.
- Remove any stale self-reference notes that claim the file is generated from a .rule entry.

Files (21): audit, build-validate-tools, commit, debug-test, handoff-tickets, handoff, implement, interview, memory-setup, next, research, reviews, rule-target, rule, spec, swarm-worker, tdd, ticket-next, ticket, tickets, user-training.

Acceptance criteria (verifiable):
- AC-1: No file under `.agents/prompts/` contains `rule-api:file generated=true` or `rule-api:entry`.
- AC-2: `rule sync-targets` (or `rule generate`) no longer regenerates any `.agents/prompts/**` file (target removed).
- AC-3: git-diff parity record shows seed-from-current content preserved (no guidance dropped) before target deletion.
- AC-4: Each migrated prompt file has accurate frontmatter for discovery/loading.