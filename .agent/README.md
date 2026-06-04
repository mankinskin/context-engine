# .agent/ — Agent Workspace Artifacts

This directory contains generated artifacts to help AI agents orient quickly
without expensive exploratory scans.

## repo_map.toon

**Read this file first** when starting a new session or entering an unfamiliar
part of the codebase. It provides:

- Workspace root and top-level directory layout
- All Rust crate names and their paths
- Agent guidance file index (instructions, prompts, skills)
- Key tool locations
- Bounded file inspection pattern

### Usage

```bash
cat .agent/repo_map.toon      # full map (compact, ~100 lines)
peek .agent/repo_map.toon --grep "crates"  # jump to crate section
```

### Refresh

Regenerate after adding/removing crates or agent files:

```bash
python3 .agent/gen_repo_map.py
```

Or add a call to the pre-commit hook so it auto-refreshes on commit:

```bash
# In .githooks/pre-commit, add:
python3 .agent/gen_repo_map.py && git add .agent/repo_map.toon
```

### When to Use

1. **Start of every session** — read `repo_map.toon` before exploring source files.
2. **Crate discovery** — look up a crate path instead of running `find` or `ls -R`.
3. **Instruction lookup** — find the right `.agents/instructions/*.instructions.md`
   for a task before reading it.
4. **Tool orientation** — check `key-tools` section for CLI tool locations.

Only fall back to `semantic_search` or directory listing when the map lacks
the detail needed.
