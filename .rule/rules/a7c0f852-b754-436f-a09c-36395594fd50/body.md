Example:

```toml
max_file_lines = 400
max_cyclomatic_complexity = 12
coverage_warn_below = 80.0

[exclude]
paths = ["target", "node_modules"]
```

## Rule Audit Manual

Use this pass when maintaining prompt or instruction quality in the rule system. Apply the same pattern to any other target context you audit.

1. Resolve the target context to audit.

2. Run the audit on that target context. For a baseline CLI run:

```bash
audit run <target-context>
```

3. For compact structured output in this repository, prefer:

```bash
rtk audit --toon run <target-context>
```

4. Read structured `findings` first, then the deduplicated repair `instructions`.

5. Summarize the run in this canonical format:
- `Findings`
- one bullet per finding with severity, scope or path, and the failing signal
- `Recommendations`
- one bullet per remediation action, deduplicated when several findings share the same fix

6. When overlap is high, treat `rule_overlap` findings as a dedup/refactor signal:
- identify the overlapping rule ids or file scopes
- keep one canonical owner for repeated guidance
- remove duplicated wording from secondary rules and regenerate targets

7. After edits, rerun the audit on the same target context to confirm findings were reduced or resolved.