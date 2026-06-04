## Commit Workflow

### 1 — Check status before staging

```bash
git status --short                        # see all changes
git submodule foreach --recursive 'git status --short && echo "=== $name ==="'
```

### 2 — Regenerate generated files first

Before staging anything, regenerate all rule-managed outputs that may have drifted:

```bash
cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml
```

Only run if you changed `.rule/` entries, `rule-targets.yaml`, or `rule-targets/*.yaml`.

### 3 — Stage in logical batches

Group changes by concern. Do not stage everything in one shot unless the diff is trivially small.

Suggested batch order:
1. Rule entries and generated outputs (`AGENTS.md`, `.rule/`, `rule-targets/`)
2. New source files (crates, tools, scripts, agent files)
3. Cargo workspace changes (`Cargo.toml`, `Cargo.lock`)
4. Hook changes (`.github/hooks/`, `.clinerules/hooks/`, `tools/agent-hooks/`)
5. Agent workspace artifacts (`.agent/`)
6. Ticket and spec store (`.ticket/`, `.spec/`)
7. Submodule pointer updates (deepest-first up to root)

### 4 — Commit each batch

```bash
git add <files for this batch>
git commit -m "<type>(<scope>): <summary>"
```

The pre-commit hook runs `rule sync-targets --check` automatically when rule-related files are staged. If it fails, run `rule sync-targets` (without `--check`) and re-stage.

### 5 — Update submodule pointers last

```bash
# Commit inside the dirty submodule first
cd memory-viewers/memory-api && git add .spec/ && git commit -m "chore(specs): ..."
cd ../viewer-api && git add .spec/ && git commit -m "chore(specs): ..."
cd .. && git add memory-api viewer-api && git commit -m "chore: update submodule pointers"
cd .. && git add memory-viewers && git commit -m "chore: update memory-viewers submodule pointer"
```

### 6 — Verify clean state

```bash
git status --short
```

The only acceptable untracked entry is the stray `?? '` (empty-name artifact from git bash on Windows — safe to ignore).
