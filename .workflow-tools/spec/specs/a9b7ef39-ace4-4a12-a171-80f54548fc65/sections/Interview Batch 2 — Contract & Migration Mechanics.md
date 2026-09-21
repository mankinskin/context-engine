User-confirmed mechanics for the foundation slice:

1. Skill format: standardize on the skills.sh native SKILL.md shape (upstream frontmatter kept), used for both adopted and authored skills. Contract must stay compatible with VS Code Copilot by-description loading (name + description present).
2. Adoption: vendor normalized copies into `.agents/skills/<name>/SKILL.md` and commit them (offline, reviewable, full control). No install-on-setup dependency.
3. Instruction migration: 1:1 conversion now (keep the current 12 boundaries), refine into finer-grained by-description files incrementally later.
4. Generator fate: delete the agent-guidance targets (`50-agents-instructions.yaml`, `35-agents-skills.yaml`, and any AGENTS.md/copilot-instructions.md agent-guidance targets) from `rule-targets/` entirely; drop the `rule-api:file generated=true` headers from `.agents/**` and root guidance files so they become hand-owned.
5. Migration safety: content-parity gate — seed each hand-owned file from its current generated content, then git-diff to prove no guidance is lost before retiring the generator targets.