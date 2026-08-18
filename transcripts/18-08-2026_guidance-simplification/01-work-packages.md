# Work Packages — Guidance and Specification Simplification

Each package is independently actionable: a single ticket (or small ticket set)
can implement it without waiting on the others, except where a dependency is
stated. Sequencing: WP-A and WP-B can run in parallel; WP-C blocks WP-D and
WP-E; WP-F runs last (and partially at the start, for baseline).

## WP-A — Guidance corpus inventory and condensation

**Outcome**: `.agents/instructions/**/*.md` (currently 59 files / 4,450 lines)
is reduced by at least 50% in total lines while every remaining file stays
fully correct, and no remaining instruction file links into a `.ticket`,
`.spec`, `.test`, or `.session` store entry — only to other guidance files.

**Steps**:
1. List all 59 files with title, `applyTo`/description, and line count.
2. Classify each as keep-as-is / keep-and-shrink / merge-into-sibling / delete.
3. For each file that currently links into a store entry (ticket/spec path),
   inline the load-bearing content of that link into the guidance file itself,
   then remove the link.
4. Shrink kept files to their essential rule + one short example each.

**Non-goals**: Rewriting agent templates (WP-B) or touching the spec store (WP-C/D/E).

**Validation**: Line-count table before/after per file; grep for
`\.ticket/tickets/|\.spec/specs/|\.test/|\.session/sessions/` inside
`.agents/instructions/**` returns zero matches on completion.

## WP-B — Agent template and prompt rewrite

**Outcome**: Each `.agents/agents/*.agent.md` template (currently 36 files /
2,472 lines) and each `.agents/prompts/*.md` file leads with an explicit
numbered workflow (step → allowed action), states allowed tools and execution
order, and drops standalone "scope limits / prohibitions" sections in favor of
stating what to do. Prompts and templates link only to other guidance files.

**Non-goals**: Changing the Clickable Reference Policy or how a running agent
cites entities in its own chat responses (see [REVIEW.md](./REVIEW.md) Finding 2)
— that policy governs live output, not template prose, and is out of scope.

**Validation**: Each rewritten template has a "## Workflow" section with
numbered steps before any "must not" list; `model:` frontmatter and tool
wildcards remain valid per [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md).

## WP-C — Spec schema redesign (design only, no migration yet)

**Outcome**: A new `spec.toml`/companion-file schema is designed and documented
where (a) `acceptance_criteria` is a structured list (id + statement, not
prose), (b) all links to tickets/specs/docs live in one dedicated fields block
separate from descriptive prose, and (c) `body.md` free text is reserved for
the informal description only. A derived full-Markdown rendering (existing
`body.md`-style view) is generated from the structured data, not hand-authored.

**Grounded in current state**: today's `spec.toml` (sampled: `03d93adb`) has no
`acceptance_criteria` field; the entire contract lives in `body.md` prose. This
is a genuine schema addition, not a reformat of existing structured data.

**Non-goals**: Writing the migration script (WP-D) or deciding which of the 139
existing specs to delete or shrink (WP-E).

**Validation**: Schema documented with example before/after for the sampled
spec `03d93adb`; `spec-api`/`spec-mcp` contract reviewed against the new shape
before any code is written.

## WP-D — Migration tooling and spec-tool adaptation (depends on WP-C)

**Outcome**: A one-shot migration tool converts every existing spec from the
current schema to the WP-C schema (extracting acceptance criteria and links
out of prose where mechanically possible, flagging specs where extraction is
not safe for manual follow-up). `spec-api`, `spec-mcp`, and `spec` CLI are
updated to read/write the new schema.

**Non-goals**: Running the migration against the live `.spec/specs` store
without a human-reviewed dry-run report first.

**Validation**: Dry-run report listing every spec, old line count, new
structured-field count, and any spec flagged for manual review; `spec_health`
passes against the migrated store in a scratch copy before applying to the
real store.

## WP-E — Spec deletion and shortening pass (depends on WP-C/D)

**Outcome**: Specs no longer needed are deleted; remaining specs are massively
shortened under the new schema.

**Non-goals**: Bulk, unreviewed deletion. Per operational-safety rules
(destructive, hard-to-reverse action against a shared store), this package
produces a **candidate list** (spec id, title, last-touched date, referencing
ticket ids) for explicit user sign-off before any `spec_delete` call executes.

**Validation**: User-approved deletion candidate list exists before any delete;
post-pass `.spec/specs/*/body.md` total line count reported against the
13,692-line baseline.

## WP-F — Measurement and second pass

**Outcome**: A single measurement report tracks, per corpus (instructions,
agents/prompts, specs): file count, total lines, and distribution shift toward
fewer large files, baseline vs. final. After WP-A–E land, a second condensation
pass runs specifically for productivity/focus (not correctness fixes).

**Baseline** (already captured in [REVIEW.md](./REVIEW.md)):

| Corpus | Files | Lines |
|---|---:|---:|
| `.agents/instructions` | 59 | 4,450 |
| `.agents/agents` | 36 | 2,472 |
| `.agents/prompts` | — | 1,510 |
| `.spec/specs/*/body.md` | 139 | 13,692 |

**Non-goals**: Defining the reduction target for specs beyond "massively
shorten" — recommend the same ≥50% total-line target used for instructions,
stated explicitly when WP-C/D/E tickets are written, since the source request
does not give specs a numeric target.

**Validation**: Final measurement table compared line-by-line against this
baseline table; second-pass diff reviewed for scope creep (must only shrink,
not add new rules).
