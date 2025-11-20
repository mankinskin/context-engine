# Agents Directory - Master Index

Organization system for agent workflows, documentation, and knowledge management.

## Directory Structure

```
agents/
├── guides/           # How-to guides and troubleshooting patterns
├── plans/            # Task plans (before execution)
├── implemented/      # Completed feature documentation
├── bug-reports/      # Known issues and problem analyses
└── tmp/              # Temporary analysis files (never commit)
```

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

**Format:** `<TOPIC>_GUIDE.md`

**Index:** `agents/guides/GUIDES_INDEX.md` (tag-based search)

**Examples:**
- `TOKEN_TEST_LABELING_GUIDE.md` - How to fix token display in tests
- `COMPACT_FORMAT_GUIDE.md` - Log formatting patterns
- `TRACING_GUIDE.md` - Tracing setup and usage

---

### `agents/plans/` 📋
**Purpose:** Task plans before implementation (research phase)

**What goes here:**
- Multi-file refactoring plans
- Large feature implementation strategies
- Architecture change proposals
- Task breakdowns with context

**When to add:**
- >5 files affected
- >100 lines changed
- Unclear scope or dependencies
- Need user review before execution
- Want parallel execution capability

**Workflow:**
1. Create `PLAN_<task_name>.md` using template
2. Gather ALL context before planning
3. Document: Objective, Context, Analysis, Steps, Risks, Validation
4. Get user approval (if needed)
5. Execute in separate session (fresh context)
6. Move to `agents/implemented/` when done

**Format:** `PLAN_<task_name>.md`

**Template:** See `agents/plans/template.md`

**Benefits:**
- Fresh context for execution = more tokens for code
- Recoverable from failures
- Parallel execution possible
- User can review strategy

---

### `agents/implemented/` ✅
**Purpose:** Completed feature documentation and implementation summaries

**What goes here:**
- Feature implementation summaries
- Completed enhancement documentation
- Implementation strategies (after execution)
- API design documentation

**When to add:**
- After completing a significant feature
- When documenting a completed refactor
- After implementing a new API
- When moving completed plan from `agents/plans/`

**Format:** `<FEATURE>_IMPLEMENTATION.md` or `<FEATURE>.md`

**Index:** `agents/implemented/INDEX.md` (tag-based search)

**Examples:**
- `TRACING_IMPLEMENTATION.md` - Test tracing system
- `CACHING_IMPLEMENTATION.md` - String repr caching
- `UNIFIED_API_IMPLEMENTATION_SUMMARY.md` - Refactor tool API

---

### `agents/bug-reports/` 🐛
**Purpose:** Known issues, bug analyses, and problem documentation

**What goes here:**
- Bug reports with root cause analysis
- Architectural problem analyses
- Algorithm deviation documentation
- Design issue investigations

**When to add:**
- After identifying a bug's root cause
- When documenting why something doesn't work
- After analyzing incorrect behavior
- When establishing "correct" vs "incorrect" patterns

**Format:** `BUG_<component>_<description>.md` or `<PROBLEM>_ANALYSIS.md`

**Index:** `agents/bug-reports/INDEX.md` (tag-based search)

**Required sections:**
- Summary (what's wrong)
- Root Cause (why it's wrong)
- Evidence (how we know)
- Fix Options (what to do)
- Related Files (where to look)

**Examples:**
- `BUG_REPORT_CAN_ADVANCE.md` - Search algorithm panic
- `DEBUG_VS_COMPACT_FORMAT.md` - Formatting architecture issue
- `SEARCH_ALGORITHM_ANALYSIS_SUMMARY.md` - Algorithm deviations

---

### `agents/tmp/` 🗑️
**Purpose:** Temporary analysis files during investigation

**What goes here:**
- Quick investigation notes
- Scratch analysis files
- Test output captures
- Temporary documentation during research

**When to add:**
- During active investigation
- When exploring a problem
- For temporary notes
- During research phase

**IMPORTANT:** 
- **Never commit these files**
- Move findings to appropriate directory when done
- Clean up after task completion

**Migration:**
- Patterns → `CHEAT_SHEET.md`
- Concepts → `<crate>/HIGH_LEVEL_GUIDE.md`
- How-tos → `agents/guides/`
- Questions → `QUESTIONS_FOR_AUTHOR.md`

---

## Quick Decision Tree

**"I'm confused by X"**
1. Check `agents/guides/GUIDES_INDEX.md` for existing guide
2. Research 10-15 min
3. Still confused? → Ask user
4. After clarification → Create guide in `agents/guides/`

**"I need to implement large feature Y"**
1. Check if >5 files or >100 lines
2. Yes? → Create plan in `agents/plans/`
3. No? → Implement directly

**"I found a bug Z"**
1. Investigate root cause
2. Document in `agents/bug-reports/`
3. After fix → Update guide in `agents/guides/`

**"I completed feature W"**
1. Write summary in `agents/implemented/`
2. Update relevant guides if patterns changed
3. Update index files

**"I'm investigating something"**
1. Use `agents/tmp/` for scratch work
2. After done → migrate findings
3. Clean up tmp files

---

## Index Files

All major directories have tag-based index files for quick searching:

- `agents/guides/GUIDES_INDEX.md` - All how-to guides
- `agents/implemented/INDEX.md` - All completed features
- `agents/bug-reports/INDEX.md` - All bug reports

**Search by tag:** Each index organizes content by tags like `#testing`, `#api`, `#optimization`, etc.

---

## Documentation Maintenance

**⚠️ CRITICAL: Keep indexes current!**

When adding a new document:
1. Add entry to appropriate INDEX.md
2. Include tags for searchability
3. Write clear summary
4. Link to related documents

When removing/archiving:
1. Remove from index
2. Update related documents
3. Clean up cross-references

---

## Related Documentation

- `AGENTS.md` - Master workflow rules and code requirements
- `CHEAT_SHEET.md` - API patterns and gotchas
- `<crate>/HIGH_LEVEL_GUIDE.md` - Crate concepts and architecture
- `QUESTIONS_FOR_AUTHOR.md` - Unresolved questions
