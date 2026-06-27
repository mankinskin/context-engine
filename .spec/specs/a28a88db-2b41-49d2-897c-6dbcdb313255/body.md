# Goal
Remove the always-on generated instruction load and replace it with a minimal **bootstrapper** plus **discoverable, pinnable** rule entries that each agent renders into its own per-session instruction set.

# Problem
Four `applyTo: "**"` instruction files plus `AGENTS.md` and copilot-instructions inject ~1,190 lines (~8-10k tokens) on every turn, regardless of task. The rendering pipeline (`rule sync-targets`) is all-or-nothing. This is the root cause of the context dilution the session-bootstrap epic exists to fix.

# Decision (D7)
Only a minimal bootstrapper instruction stays always-on. All other guidance becomes discoverable rule entries that an agent gathers (via `rule_search`), pins, and renders itself. Rule *filters/scopes* may be pinned; individual rule bodies are fetched on demand (headers-only by default, D6).

# Scope
- Author a minimal **bootstrapper instruction** (<500 tokens) that knows only the search + session tools and drives `session_init` → cascade → pin → agent-side render.
- Stop generating the always-on `applyTo: "**"` instruction bodies as per-turn content. Convert their content into canonical rule entries that remain **discoverable** (indexed/searchable) but are **not** force-loaded. Target files:
  - `ticket-system.instructions.md` (447 lines)
  - `commit.instructions.md` (239 lines)
  - `spec-system.instructions.md` (180 lines, currently duplicated)
  - `token-efficiency.instructions.md` (132 lines)
- Narrow remaining `applyTo` globs so only the bootstrapper is universal; path-scoped instructions may keep narrow globs.
- **Fix the `spec-system.instructions.md` duplication** (rendered body repeats — `## Scope` at lines 6 and 94); trace it to the rule entry / `rule-targets/*.yaml` node that double-includes content.
- Provide the agent-side render path: given pinned rule entries/filters, produce a focused session instruction set.
- Keep `rule sync-targets` deterministic for whatever remains generated.

# Non-goals
- Changing the canonical rule store format beyond discoverability metadata.
- Implementing the session pin mechanism (owned by the runtime + CLI/MCP specs); this spec consumes it.

# Dependencies
- Consumes the pin/view surfaces: [6b2dc497 init/pin/unpin/view](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/6b2dc497-188c-44f5-9106-bf35deecb7a1/ticket.toml).

# Acceptance Criteria (test-validatable)
1. After the change, the always-on instruction surface is the bootstrapper only; a test asserts the generated always-on set excludes the four converted files' bodies. *(generation/output test)*
2. The four converted instruction bodies are discoverable via `rule_search` by representative queries (e.g. "ticket state machine", "commit hooks"). *(search test)*
3. `spec-system` content appears exactly once in any generated output (no `## Scope` duplication). *(output assertion)*
4. `rule sync-targets --check` passes deterministically on the reduced target set. *(CI gate)*
5. Given a set of pinned rule entries, the agent-side render produces a focused instruction set containing only those entries. *(unit test of the render function)*

# Traceability
- Parent: `memory-api/session-api/dynamic-session-bootstrapping`
- Ticket: [b4a8dc5e minimal bootstrapper + selective loading](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/b4a8dc5e-9d80-4fea-bb42-0c30aba0ecd6/ticket.toml)

# Validation
- ValidationSpec: rule-generation output tests + `rule_search` discoverability tests + agent-side render unit test.
- ValidationExecution (planned): `cargo test -p rule-cli` / `rule sync-targets --check`.