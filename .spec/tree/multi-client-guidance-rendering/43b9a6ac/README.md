<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=43b9a6ac-89d6-479a-876b-3d1a5e2d39e6 slug=rule-system/multi-client-guidance-rendering digest=f4087c45ec00 -->

# Multi-Client Guidance Rendering

- slug: `rule-system/multi-client-guidance-rendering`
- component: rule-api
- state: draft
- index_ref: `.spec/specs/43b9a6ac-89d6-479a-876b-3d1a5e2d39e6/spec.toml`

## Summary

The repository maintains agent-facing guidance (instructions, agent definitions, prompts, skills, root guidance) as hand-owned markdown under `.agents/**`, `AGENTS.md`, and `.github/copilot-instructi…

## Acceptance Criteria Excerpt

1. A single canonical rule fragment renders correctly into Copilot, Cline, and OpenCode output without any client-specific text stored in the fragment body. 2. Frontmatter is produced by the client profile from structured rule-entry metadata, not hoisted from a body. 3. `rule in…

## Navigation

- Parent: _(root)_
- Children: _(none)_
