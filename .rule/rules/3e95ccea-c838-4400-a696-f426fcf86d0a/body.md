## Commit Message Conventions

Use conventional commits format: `<type>(<scope>): <imperative summary>`.

### Types

| Type | Use for |
|---|---|
| `feat` | New features or capabilities |
| `fix` | Bug fixes |
| `chore` | Maintenance, submodule pointer updates, housekeeping |
| `refactor` | Code changes that neither fix a bug nor add a feature |
| `docs` | Documentation-only changes |
| `test` | Tests only |
| `perf` | Performance improvements |

### Scope

Use the crate name, subsystem, or area affected. Examples:
- `feat(token-efficiency): add peek-cli`
- `feat(ticket-api): add board health endpoint`
- `chore(tickets): update tracker state`
- `chore(specs): spec store history from rule sync`
- `chore: update memory-viewers submodule pointer`

### Multi-commit batches

When multiple logical groups of files are staged, split into separate commits per concern:

1. Source / feature files
2. Generated outputs (regenerated after source changes)
3. Ticket / spec store updates
4. Submodule pointer updates (one per level)

### Body format

For non-trivial commits, add a body after a blank line:

```
feat(token-efficiency): add peek-cli — token-bounded file inspection utility

- tools/cli/peek-cli/: new CLI crate with --start/--end/--window/--head/--tail/--grep/--count/--skeleton/--all
- --skeleton mode strips function bodies, returns signatures only (Rust, Python, generic)
- Wire into Cargo.toml workspace members
```

### Ticket checkpoint suggestions

Suggest a `git commit` checkpoint when:
- A ticket transitions to `done`
- A batch of related tickets all reach `done` or `in-review`
- A dependency graph changes materially
- A tracked bug is fixed

Suggested message format: `feat(<component>): <imperative summary of ticket work>`
