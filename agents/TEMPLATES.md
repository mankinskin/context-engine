# Document Templates

Reference templates for structured agent documentation. The MCP server uses these formats when creating new documents.

## Common Frontmatter

All documents include YAML frontmatter for metadata:

```yaml
---
tags: `#tag1` `#tag2`
summary: One-line description for INDEX
status: 📋       # Plans only: 📋 Design | 🚧 In Progress | ✅ Complete | ⚠️ Blocked | ❌ Superseded
---
```

---

## Guide Template (`guides/`)

**Filename:** `YYYYMMDD_<NAME>_GUIDE.md`

```markdown
---
tags: `#topic` `#pattern`
summary: Brief description of what this guide solves
---

# Guide Title

## Problem

<!-- What problem does this guide solve? When would you need this? -->


## Solution

<!-- Step-by-step solution or pattern -->


## Example

\`\`\`rust
// Example code here
\`\`\`

## Common Mistakes

<!-- What NOT to do -->

- 

## Related

<!-- Links to related guides, files, or documentation -->

- 
```

---

## Plan Template (`plans/`)

**Filename:** `YYYYMMDD_PLAN_<NAME>.md`

```markdown
---
tags: `#feature` `#refactoring`
summary: Brief objective statement
status: 📋
---

# Plan: Feature Name

## Objective

<!-- One clear sentence: What are we building/fixing/changing? -->


## Context

### Files Affected

<!-- List all files that will be modified -->

- 

### Dependencies

<!-- What other code/systems does this touch? -->

- 

## Analysis

### Current State

<!-- How does it work now? What's the problem? -->


### Desired State

<!-- How should it work after changes? -->


## Execution Steps

<!-- Atomic, testable steps. Each step <5 min. -->

- [ ] Step 1: 
- [ ] Step 2: 
- [ ] Step 3: 

## Validation

<!-- How to verify the changes work -->

- [ ] Tests pass: `cargo test -p <crate>`
- [ ] 

## Risks

<!-- What could go wrong? -->

- 
```

---

## Implemented Template (`implemented/`)

**Filename:** `YYYYMMDD_<NAME>_COMPLETE.md`

```markdown
---
tags: `#feature` `#api`
summary: Brief description of what was implemented
---

# Implemented: Feature Name

## Summary

<!-- 2-3 sentence summary of what was implemented -->


## Changes

<!-- Key changes made -->

| File | Change |
|------|--------|
| | |

## API

<!-- New or changed APIs (if applicable) -->

\`\`\`rust
// Key types or functions
\`\`\`

## Migration

<!-- How to update code using old APIs (if breaking change) -->


## Testing

<!-- How this was validated -->

- 
```

---

## Bug Report Template (`bug-reports/`)

**Filename:** `YYYYMMDD_BUG_<COMPONENT>_<DESCRIPTION>.md`

```markdown
---
tags: `#bug` `#component`
summary: One-line description of the bug
---

# Bug: Brief Description

## Symptoms

<!-- What goes wrong? Error messages, panics, incorrect behavior -->


## Reproduction

<!-- Minimal steps to reproduce -->

1. 
2. 
3. 

## Root Cause

<!-- Why does this happen? -->


## Location

<!-- Where in the code is the bug? -->

- File: 
- Function: 
- Line: 

## Fix

<!-- How to fix it (or options if unclear) -->

### Option A

<!-- Preferred fix -->


### Option B (if applicable)

<!-- Alternative fix -->


## Verification

<!-- How to verify the fix works -->

- [ ] 
```

---

## Analysis Template (`analysis/`)

**Filename:** `YYYYMMDD_<TOPIC>_ANALYSIS.md`

```markdown
---
tags: `#algorithm` `#architecture`
summary: Brief description of what was analyzed
---

# Analysis: Topic Name

## Overview

<!-- What is being analyzed and why? -->


## Findings

### Key Finding 1

<!-- Description -->


### Key Finding 2

<!-- Description -->


## Comparison (if applicable)

| Aspect | Option A | Option B |
|--------|----------|----------|
| | | |

## Conclusions

<!-- What should be done based on this analysis? -->


## References

<!-- Related files, docs, or external resources -->

- 
```

---

## Simplified INDEX Format

Each category has an INDEX.md that serves only as a table of contents:

```markdown
# Category Index

Brief description.

| Date | File | Summary |
|------|------|---------|
| 2025-12-03 | [EXAMPLE.md](20251203_EXAMPLE.md) | One-line summary |
```

For plans, add a Status column:

```markdown
| Date | Status | File | Summary |
|------|--------|------|---------|
| 2025-12-03 | 📋 | [PLAN_EXAMPLE.md](20251203_PLAN_EXAMPLE.md) | One-line summary |
```
