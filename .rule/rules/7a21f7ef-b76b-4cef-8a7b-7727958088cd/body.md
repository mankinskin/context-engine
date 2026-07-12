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
- File, path, and entity references in responses are owned by the Clickable Reference Policy section below — it is the single definition of reference formatting (forward-slash markdown links, repo-root-relative, no backslashes, no backticks around the reference). Do not restate those rules here; follow that section.
- If another instruction requests backtick-wrapped file references, the Clickable Reference Policy takes precedence for file/path citations.
