---
description: "Use when committing changes that involve Git submodules or nested repositories. Covers commit order, detecting dirty submodules, and updating submodule pointers."
---

## Commit order (deepest-first)

1. Commit inside nested submodules (e.g. `memory-api`) first.
2. Commit in their parent (e.g. `viewer-api` / `memory-viewers`) after staging pointer updates.
3. Update and commit the pointer in the root repo last.

## Detecting dirty submodules

```bash
git status --short
git submodule status
```

Lowercase `m` indicates local changes inside a submodule; uppercase `M` indicates the parent records a different SHA.

## Updating submodule pointers

After committing inside a submodule, stage the submodule directory in the parent and commit the pointer update:

```bash
cd memory-viewers && git add memory-api viewer-api && git commit -m "chore: update submodule pointers"
cd .. && git add memory-viewers && git commit -m "chore: update memory-viewers submodule pointer"
```
