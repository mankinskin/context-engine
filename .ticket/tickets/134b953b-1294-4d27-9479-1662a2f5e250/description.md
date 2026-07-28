Parent epic: `0ee95228`. Spec: `2ccde9ee`.

## Scope

### Vendored external skills (into `.agents/skills/`, recorded in `skills-lock.json`)
- `marcoshaber99/slidev-skills` — Slidev syntax correctness, code demos, Mermaid, frontmatter.
- `yoanbernabeu/slidev-skills` — modular: animations, code/Shiki magic-move.
- `neversight/slidev-syntax-guide` — on-demand MDC/layout/notes syntax manual.
- `zarazhangrui/frontend-slides` — plan-first narrative strategy, layout presets, visual QA.

Vendor the content (do not fetch at runtime) so the workflow is offline-safe and reviewable.

### Repo-local skill
Author `.agents/skills/presentation-workflow/SKILL.md` composing the above into this repo's
loop: narrative plan -> preset selection -> Slidev authoring -> `presentation` CLI/MCP deck
materialization -> build -> Playwright verification with per-slide screenshots. Encode the
density rule (one idea per slide) and point at the theme's preset descriptors as the
authoritative layout vocabulary.

### Agent surface
- A Presentation agent mode in `.agents/` owning the script -> deck -> build -> verify loop.
- A workflow prompt in `.agents/prompts/` for the same loop.
- Both must make the loop cheap: a mid-tier model should produce a correct deck using
  templates + presets without reading theme source or Slidev docs.

### Retire the legacy agent
Delete `Presentation.agent 2.md` **only after** ticket `60222b57` (custom repo theme pack)
has encoded its design principles as presets: full-viewport hero, sticky nav with progress
bar, one idea per screen-height section, dark CTA bookend. Verify each principle has a
corresponding preset before deleting.

## Definition of done

- All five skills present and lock-recorded.
- Agent mode + prompt exist and are discoverable.
- A dry-run authoring pass produces a valid deck without human syntax correction.
- `Presentation.agent 2.md` removed, with its four principles traced to theme presets.
