---
description: "Use when editing files. Covers surgical replacement patterns and avoiding full-file rewrites."
---

## Differential Patching

When editing files, always use the narrowest applicable edit operation:

- `replace_string_in_file` with 3–5 lines of context — preferred for surgical changes.
- `multi_replace_string_in_file` — batch multiple independent replacements in one call.
- Only use `create_file` when creating a new file from scratch.
- **Never** read a full file and rewrite it wholesale to make a small change.
