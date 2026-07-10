## Instruction Precedence and Exceptions

When guidance overlaps, apply instructions in this order and document any explicit exception used for the current task.

| Priority | Source | How to apply |
|---|---|---|
| 1 | System + safety policy | Always mandatory; cannot be overridden by repository guidance. |
| 2 | Developer/session instructions | Treat as global execution contract for this session. |
| 3 | AGENTS.md global rules | Baseline repository behavior. |
| 4 | Path-scoped instruction files | Apply only when `applyTo` matches touched files. |
| 5 | Prompt/task-specific directives | Use for ticket-local implementation details. |

Exception handling rules:
- Prefer the most specific matching guidance over broader guidance.
- If two rules conflict at the same specificity, follow the newer or explicitly scoped one and record the conflict in ticket notes.
- If a path-scoped rule conflicts with AGENTS.md global guidance, follow the path-scoped rule for that file scope and keep AGENTS.md as default elsewhere.
- If an instruction conflicts with platform/tooling constraints, apply the safest feasible behavior and note the limitation in the ticket/spec summary.
- Never resolve conflicts by silently ignoring one side; capture the chosen precedence and rationale in the active ticket description.

Formatting conflict policy (canonical):
- When referencing workspace files, paths, or line citations in responses, use markdown links with forward slashes only.
- For files in the current directory, use `./`-prefixed links (for example `[./AGENTS.md](./AGENTS.md#L1)`).
- Do not use Windows-style backslashes in markdown links.
- Do not wrap file references in backticks when the linkified-file policy is active.
- If another instruction requests backtick-wrapped file references, the linkified-file policy takes precedence for file/path citations.
