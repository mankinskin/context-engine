# T8: PDF Skill + Documentation

## Objective

Author the agent-facing skill so agents actually discover and correctly use the
new tools, plus the crate documentation.

## Files To Create

- `.agents/skills/pdf/SKILL.md`
- `memory-api/crates/pdf-api/README.md`
- `memory-api/crates/pdf/README.md`

## Files To Modify

- `.agents/skills/README.md` — add a row to the Master Index table.

## Design

### Skill authoring rules (locked decision 9)

- Plain hand-authored markdown. **Not** rule-mcp generated.
- Must **not** carry a `<!-- rule-api:file generated=true -->` header — that
  header belongs to the rule-generation system and would mark this file as
  machine-owned.
- Folder name matches the `name` frontmatter field.
- Required frontmatter: `name` and `description`.

### The `description` field is the whole ballgame

Skills load by description. If the description does not let an agent decide
applicability without opening the file, the skill will never fire. Write it to
trigger on: PDF, extract text from PDF, merge/split PDF, PDF metadata, create a
PDF, PDF pages, typst. Follow the style of the existing skill descriptions in
`.agents/skills/` — they are concrete about trigger conditions.

### Skill body

- Prefer the `pdf-mcp` named tools; document `pdf-cli` as the fallback for
  non-MCP contexts. This mirrors how `peek` is documented.
- A capability → tool mapping table so an agent picks the right tool first time.
- The safety model stated plainly: writes need an explicit output path,
  clobbering needs `overwrite: true`, paths are confined to a sandbox root.
- The typst caveat: `pdf_create` in typst mode needs `typst-cli` on PATH and
  errors cleanly without it.
- The scanned-PDF caveat: no text layer means no text; OCR is out of scope.
- Worked examples for the common flows: extract text, merge, split, set
  metadata, create.

### Do not

- Do not vendor the upstream Anthropic/community PDF skill. It targets a Python
  toolchain and would send agents to the wrong tools.
- Do not touch `skills-lock.json` — that tracks vendored skills only, and this
  one is hand-owned.

### Crate READMEs

Follow the existing `memory-api/tools/mcp/peek-mcp/README.md` style: short,
naming the tools and pointing at the API crate for behavior.

## Acceptance Criteria

- [ ] `.agents/skills/pdf/SKILL.md` exists with valid `name` + `description`
      frontmatter, and the folder name matches `name`.
- [ ] No generated-file header is present.
- [ ] The description names concrete trigger conditions, not a vague summary.
- [ ] Every implemented tool appears in the capability → tool mapping.
- [ ] Write-safety, sandbox, typst, and scanned-PDF caveats are all documented.
- [ ] Worked examples are accurate against the shipped tool schemas — verify by
      running them, not by assuming.
- [ ] `.agents/skills/README.md` Master Index has the new row.
- [ ] `skills-lock.json` is unchanged.
- [ ] Both crate READMEs exist.
- [ ] A fresh agent session, given only the skill, can successfully complete a
      merge and a text extraction. This is the real test — run it.

## Validation

Manual: start a fresh agent session, ask for a PDF merge without naming any
tool, and confirm the skill triggers and the task succeeds. Record the outcome.

## Depends On

T7.
