Build a generator that reads the rule store via rule-api and emits a grouped, tagged rules catalog under `.rule/README.md` (and `.rule/index.toon` for its co-located machine sidecar).

## Scope
- Implement a `rule-catalog` subcommand (or extend `rule-cli`) that reads all rule entries.
- Group rules by slug prefix segments (D4) — e.g. grouping by the domain segments of the rule's slug, such as `shared/agent-rules/`. No new `category` schema field is introduced for now.
- For each rule entry: emit slug, section, title, summary, tags, feedback-rating if available, and a ContextRef to the canonical rule entry.
- Conforms to the ContextNode schema (0dba399a); co-located under `.rule/`.
- Emit an `.agents/` agent-hook node pointing agents at the rules catalog (D1).
- Both index files plus the `.agents/` hook are committed to git (D5) and regenerated during the rule-file pre-commit hook (D2).

## Acceptance criteria
- Catalog output is written to `.rule/README.md` and `.rule/index.toon`.
- Rules are categorized by the segment hierarchy derived from the slug.
- A visual badge/indicator surfaces low-rated rules.
- Re-running with unchanged rule data is digest-stable.

## Non-goals
- No global `.context/` store.
- Does not replace rule-api rule formatting.
- Does not add a `category` field to rule-api (slug-derived grouping only, per D4).

## Resolved design decisions
- D4: group by slug prefix for now. D8: TOON sidecar. D2: pre-commit. D5: committed.