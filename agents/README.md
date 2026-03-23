# Agents Directory - Master Index

Organization system for agent workflows, documentation, and knowledge management.

> **📁 PROJECT STRUCTURE:** All crates are in `crates/` directory:
> - `crates/context-trace/` - Foundation layer
> - `crates/context-search/` - Search layer
> - `crates/context-insert/` - Insert layer
> - `crates/context-read/` - Read layer

## Directory Structure

```
agents/
├── guides/           # How-to guides and troubleshooting patterns
├── plans/            # Task plans (before execution)
├── implemented/      # Completed feature documentation
├── bug-reports/      # Known issues and problem analyses
├── analysis/         # Algorithm analysis and comparisons
└── tmp/              # Temporary analysis files (never commit)
```

## File Naming Convention (CRITICAL)

**All agent-generated files MUST include a timestamp prefix for chronological ordering:**

- **Format:** `YYYYMMDD_<FILENAME>.md` (e.g., `20251203_FEATURE_NAME.md`)
- **Benefits:**
  - Files sorted newest-to-oldest alphabetically (descending date order)
  - File age immediately visible without checking git history
  - Easy tracking of document evolution over time
  - Prevents filename collisions across time periods
  
**Examples:**
- ✅ `20251203_SEARCH_ALGORITHM_GUIDE.md`
- ✅ `20251127_PLAN_EFFICIENT_CHECKPOINTED_CURSOR.md`
- ✅ `20250122_TRAIT_CONSOLIDATION_V2_COMPLETE.md`
- ❌ `SEARCH_ALGORITHM_GUIDE.md` (missing timestamp)

**When to use:**
- Always for new files in `guides/`, `plans/`, `implemented/`, `bug-reports/`, `analysis/`
- Not required for `INDEX.md` files (special case)
- Not required for `tmp/` files (temporary, not committed)

---

## When to Use Each Directory

### `agents/guides/` 📚
**Purpose:** Persistent how-to guides and troubleshooting patterns

**What goes here:**
- Pattern guides (how to do X correctly)
- Common mistakes and fixes
- Migration checklists
- API usage examples
- Troubleshooting workflows

**When to add:**
- After solving a confusing problem
- When documenting a pattern that will recur
- After user clarifies unclear behavior
- When establishing best practices

**Format:** `YYYYMMDD_<TOPIC>_GUIDE.md`

**Index:** `agents/guides/INDEX.md` (tag-based search)

**⚠️ REQUIRED:** Add entry to INDEX.md with summary and tags.

---

### `agents/plans/` 📋
**Purpose:** Task plans before implementation (research phase)

**What goes here:**
- Multi-file refactoring plans
- Large feature implementation strategies
- Architecture change proposals

**When to add:** >5 files affected, >100 lines changed, or unclear scope

**Workflow:**
1. Create `YYYYMMDD_PLAN_<task_name>.md` using template
2. Gather ALL context before planning
3. Document: Objective, Context, Analysis, Steps, Risks, Validation
4. Execute in separate session (fresh context)
5. Create summary in `agents/implemented/` + update INDEX.md
6. Archive plan (rename to `YYYYMMDD_PLAN_<task>_DONE.md`) or delete if obsolete

**Format:** `YYYYMMDD_PLAN_<task_name>.md`

**Template:** `agents/plans/20251203_PLAN_TEMPLATE.md`

---

### `agents/implemented/` ✅
**Purpose:** Completed feature documentation and implementation summaries

**What goes here:**
- Feature implementation summaries
- Completed enhancement documentation
- API design documentation
- Completed plans from `agents/plans/`

**When to add:** After completing significant features, refactors, or new APIs

**Format:** `YYYYMMDD_<FEATURE>_IMPLEMENTATION.md` or `YYYYMMDD_<FEATURE>.md`

**Index:** `agents/implemented/INDEX.md` (tag-based search)

**⚠️ REQUIRED:** Add entry to INDEX.md with summary and tags.

---

### `agents/bug-reports/` 🐛
**Purpose:** Known issues, bug analyses, and problem documentation

**What goes here:**
- Bug reports with root cause analysis
- Architectural problem analyses
- Algorithm deviation documentation

**When to add:** After identifying root cause or documenting incorrect behavior

**Format:** `YYYYMMDD_BUG_<component>_<description>.md` or `YYYYMMDD_<PROBLEM>_ANALYSIS.md`

**Index:** `agents/bug-reports/INDEX.md` (tag-based search)

**Required sections:** Summary, Root Cause, Evidence, Fix Options, Related Files

**⚠️ REQUIRED:** Add entry to INDEX.md with summary and tags.

---

### `agents/analysis/` 🔬
**Purpose:** Deep algorithm analysis and comparison documents

**What goes here:**
- Algorithm comparison documents
- Design analysis
- Architectural deep dives
- Theory vs implementation analysis

**When to add:** When documenting algorithmic differences, design decisions, or comparing approaches

**Format:** `YYYYMMDD_<TOPIC>_COMPARISON.md` or `YYYYMMDD_<TOPIC>_ANALYSIS.md`

**Index:** `agents/analysis/INDEX.md` (tag-based search)

**⚠️ REQUIRED:** Add entry to INDEX.md with summary and tags

---

### `agents/tmp/` 🗑️
**Purpose:** Temporary analysis files during investigation

**What goes here:** Investigation notes, scratch files, test outputs

**When to add:** During active investigation or research

**⚠️ NEVER COMMIT** - Move findings when done:
- Patterns → `CHEAT_SHEET.md`
- Concepts → `<crate>/HIGH_LEVEL_GUIDE.md`
- How-tos → `agents/guides/`
- Questions → `QUESTIONS_FOR_AUTHOR.md`

---

## Quick Decision Tree

| Situation | Action |
|-----------|--------|
| Confused by X | Check `agents/guides/INDEX.md` → Research 10-15min → Ask user → Document in `guides/` |
| Large feature (>5 files) | Create plan in `agents/plans/` → Execute later → Move to `implemented/` |
| Found bug | Investigate → Document in `agents/bug-reports/` → After fix, update `guides/` |
| Completed feature | Write summary in `agents/implemented/` → Update indexes |
| Algorithm analysis | Document in `agents/analysis/` → Update INDEX.md |
| Investigating | Use `agents/tmp/` → Migrate findings → Clean up |

---

## Index Files

- `agents/guides/INDEX.md` - How-to guides
- `agents/implemented/INDEX.md` - Completed features
- `agents/bug-reports/INDEX.md` - Bug reports
- `agents/analysis/INDEX.md` - Algorithm analysis

**⚠️ Update indexes:** When adding/removing documents, update appropriate INDEX.md with tags and summary.

---

## Related Documentation

- `AGENTS.md` - Master workflow rules and code requirements
- `CHEAT_SHEET.md` - API patterns and gotchas
- `crates/<crate>/HIGH_LEVEL_GUIDE.md` - Crate concepts and architecture
- `QUESTIONS_FOR_AUTHOR.md` - Unresolved questions
