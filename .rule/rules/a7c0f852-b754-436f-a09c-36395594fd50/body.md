Example:

```toml
max_file_lines = 400
max_cyclomatic_complexity = 12
coverage_warn_below = 80.0

[exclude]
paths = ["target", "node_modules"]
```

## Rule Audit Manual

Use this pass when maintaining prompt or instruction quality in the rule system.

1. Run a baseline audit:

```bash
audit run .
```

2. For compact structured output in this repository, prefer:

```bash
rtk audit --toon run .
```

3. Check the human summary line `Rule overlap: ...` or inspect structured `rule_overlap` findings.

4. When overlap is high, treat it as a dedup/refactor signal:
- identify the overlapping rule ids or file scopes
- keep one canonical owner for repeated guidance
- remove duplicated wording from secondary rules and regenerate targets

5. After edits, rerun `audit run .` to confirm overlap findings were reduced or resolved.