## Submodule Structure

The workspace contains three levels of Git submodules:

```
context-engine/          ← root repo
├── context-stack/       ← submodule (heads/main)
└── memory-viewers/      ← submodule (heads/main)
    ├── memory-api/      ← nested submodule (heads/main)
    └── viewer-api/      ← nested submodule (heads/main)
```

### Commit order

Always commit deepest-first:

1. Commit in `memory-api/` when it has local changes.
2. Commit in `viewer-api/` when it has local changes.
3. Update the submodule pointer in `memory-viewers/` and commit there.
4. Update the submodule pointer in the root repo and commit there.
5. Commit in `context-stack/` when it has local changes (independent of memory-viewers).
6. Update the `context-stack` pointer in the root repo when context-stack changed.

### Detecting dirty submodules

```bash
git status --short           # lowercase 'm' means dirty submodule (local changes)
git submodule status         # shows commit hash + branch per submodule
```

Lowercase `m` next to a submodule name means it has local uncommitted changes.
Uppercase `M` means the parent has a different recorded SHA than the submodule HEAD.

### Updating submodule pointers

After committing inside a submodule, return to the parent repo and stage the submodule directory:

```bash
cd memory-viewers && git add memory-api viewer-api && git commit -m "chore: update submodule pointers"
cd .. && git add memory-viewers && git commit -m "chore: update memory-viewers submodule pointer"
```
