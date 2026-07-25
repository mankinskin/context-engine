Establish the skill directory contract and clean up the current mess.

Anchor spec: agents/skill-infrastructure (a9b7ef39) — AC2.

Scope:
- Adopt skills.sh-native SKILL.md shape as canonical (frontmatter with name + description; description drives by-description loading; compatible with VS Code Copilot skill loading).
- Enforce folder-per-skill: `.agents/skills/<skill-name>/SKILL.md`; retire the root `NAME.SKILL.md` scheme.
- Re-home the orphan: move `.agents/skills/token-optimized-agentic-engineering.SKILL.md` -> `.agents/skills/token-optimized-agentic-engineering/SKILL.md`; strip the `rule-api:file generated=true` header so it is hand-owned.
- Add a master skills index listing each skill's name + description trigger.

Acceptance criteria (verifiable):
- AC-1: A written contract doc/section defines the SKILL.md schema and folder rule.
- AC-2: No file matching `.agents/skills/*.SKILL.md` remains at the skills root (all inside folders).
- AC-3: The re-homed skill has no `rule-api:file generated=true` header.
- AC-4: A master index enumerates all skills with descriptions and resolves to existing folders.