Build a generator that reads the ticket store and emits a generated markdown index file listing all tickets as ContextNode references with short semantic summaries, co-located directly in `.ticket/README.md` along with its machine sidecar under `.ticket/index.toon`.

## Scope
- Implement a `context-index-cli` subcommand (or extend `ticket-cli`) that reads all tickets from the active workspace, groups them by state and component, and emits a markdown index file to `.ticket/README.md`.
- Each entry in the index should include: ticket id (short), title, state, priority, summary extracted from the ticket description, and a ContextRef pointing to the canonical ticket.toml path.
- The output must conform to the ContextNode schema defined in the schema ticket (0dba399a).
- Emit an agent-client consumable instruction hook under `.agents/` (ContentKind `agent_hook`) that points agents at `.ticket/README.md` / `.ticket/index.toon` (D1 third surface).
- Wire as a git pre-commit hook (D2) that regenerates `.ticket/README.md`, `.ticket/index.toon`, and the `.agents/` hook when ticket store files under `.ticket/tickets/` are staged. Profile the command and keep commit latency low; if the profiled pre-commit budget is exceeded, fall back to a post-commit regeneration path.
- All generated index files are committed to git (D5).

## Acceptance criteria
- Running the generator produces `.ticket/README.md`, `.ticket/index.toon`, and the `.agents/` agent hook.
- Each ticket entry is a ContextRef with stable id, source path, and semantic summary.
- The index is grouped by state and by component.
- Re-running the generator with unchanged input produces files with the same digest.
- The pre-commit hook runs on staged ticket.toml files and completes within the profiled latency threshold (target under 100ms); the profiling result is recorded.

## Non-goals
- No central `.context/` store folder.
- Does not emit full ticket bodies, only summaries and references.

## Resolved design decisions
- D1: workspace-folder index in `.ticket/`, plus an `.agents/` agent-hook node.
- D2: git pre-commit hook, profiled for low commit latency (post-commit fallback only if the budget is exceeded).
- D5: outputs committed to git.