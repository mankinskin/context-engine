Migrate all 12 `.agents/instructions/*.md` files off the rule-store generator to hand-owned by-description files, and retire the agent-guidance targets.

Anchor spec: agents/skill-infrastructure (a9b7ef39) — AC5. STRICTLY LAST: runs after skills land (contract + vendor + Dioxus).

Scope:
- 1:1 conversion now (keep the current 12 file boundaries); refine into finer-grained by-description files incrementally later.
- Content-parity gate: seed each hand-owned file from its current generated content, then git-diff to confirm no guidance is lost before deleting targets.
- Delete agent-guidance targets from `rule-targets/` (`50-agents-instructions.yaml`, `35-agents-skills.yaml`, and any AGENTS.md / copilot-instructions.md agent-guidance targets).
- Strip `rule-api:file generated=true` headers from `.agents/**` and root guidance files.

Files (12): audit, commit, context-http, core-crates, frontend, session-bootstrap, session-optimization, spec-system, tests, ticket-system, token-efficiency, viewer-api-tools.

Acceptance criteria (verifiable):
- AC-1: No file under `.agents/instructions/` contains `rule-api:file generated=true`.
- AC-2: `rule sync-targets` no longer regenerates any `.agents/**` or agent-guidance root file (targets removed).
- AC-3: git-diff parity record shows the seed-from-current content preserved (no guidance dropped) before target deletion.
- AC-4: Each migrated file has an accurate `description`/`applyTo` frontmatter for by-description loading.