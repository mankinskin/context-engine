# Path-Specific Custom Instructions

This directory can contain `.instructions.md` files for path-specific Copilot guidance.

## Format

Each file should:
1. End with `.instructions.md` (e.g., `trace.instructions.md`)
2. Start with YAML frontmatter specifying paths:

```markdown
---
applyTo: "context-trace/**/*.rs"
excludeAgent: "code-review"  # Optional: exclude from code-review or coding-agent
---

Your instructions here...
```

## Examples

- `trace.instructions.md` - Instructions for context-trace crate
- `search.instructions.md` - Instructions for context-search crate
- `tests.instructions.md` - Testing-specific guidance

## Glob Patterns

- `"context-trace/**/*.rs"` - All Rust files in context-trace
- `"**/*.rs,**/*.toml"` - All Rust and TOML files
- `"**/tests/**"` - All test files
- `"**"` - All files

## See Also

- `.github/copilot-instructions.md` - Repository-wide instructions (applied to all files)
- GitHub docs: https://docs.github.com/en/copilot/customizing-copilot/adding-custom-instructions-for-github-copilot
