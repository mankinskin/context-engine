# Bulk Ticket Closure Audit

On 2026-08-10, a bulk lifecycle operation closed tickets based on `in-review` state and dependency progression. The operation did **not** independently verify ticket acceptance criteria or deferrals before closure.

The command output reported 100 tickets closed. The list below contains the 50 ticket closures directly recoverable from `history.ndjson` events authored by `github-copilot` during the operation. The remaining reported closures require a separate audit before any acceptance-criteria conclusion is drawn.

## Directly Attributable Closures

- `0afe45b5` — [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store
- `161454bd` — [ticket-api] Reject ticket creation when `type` has no registered schema
- `16d8aed9` — [ticket-api][ticket-cli] Auto-walk allowed transition paths by default; make strict single-hop an opt-out flag
- `1dffcf23` — [context-stack] Define replayable graph-operation journal format for log-viewer
- `203248cb` — [session-api][session-mcp] Separate behavioral vs descriptive workflow node kinds; add Spec kind
- `20b6a09a` — [token-efficiency] Omit default workspace and schema from ticket outputs
- `244c3113` — filesystem operations: bounded list/stat/move tool suite (api + cli + mcp)
- `25b5f3e7` — [session-api][handoff] Make upward context and ticket narrative reproducible in handoff markdown
- `2b75bac2` — [session-api] sessions_for_ticket is inert: capture ticket linkage at check-in and decide a structured-data backfill
- `2e41c96d` — [memory-api] Create domain instrumentation and journaling coverage map
- `32067e83` — [mcp-cost-gate] Tolerate common caller_model naming deviations instead of rejecting the call
- `321f6a3a` — [repo-guidance] Model cost-awareness and tiered model-routing guidance
- `33463861` — [context-enrichment] Dogfood sessions_for_ticket against ticket 06cfe998 and record findings
- `37b5026f` — Migrate .agents/instructions into nested workflow folders
- `416ebd52` — [ticket-http] Return only authoritative resolved hits in workspace-aware search
- `44119807` — [tool-metrics][T2] Capture tool output size at capture time with per-call source attribution
- `4a9b49fd` — [ticket-viewer] Keep filtered explorer state authoritative under live refresh
- `4f066c96` — [token-efficiency] Add compact terminal MCP tool
- `528af270` — [workspace-policy] 7/6 Cleanup retro fixture references + audit boundary rule
- `565ae4b1` — Make copilot-capture-hook provisioning outcome observable
- `5ad77aba` — [ticket-mcp][spec-mcp][rule-api][session-mcp] Add self-describing capability catalog and help surfaces
- `6ded2540` — [workspace] Relocate viewer and peek crates into owning repositories
- `72c1e92d` — [token-efficiency] Generate static `.agent/repo_map.toon`
- `742dbc65` — [session-api][handoff] Model and enforce upward context for implementation-ready handoffs
- `7c74f2fe` — [agents] Add dedicated context-enrichment.agent.md template for enrich-review-close workflow
- `7d857543` — [ticket-mcp][spec-mcp][rule-api] Self-describing capability catalog for ticket/spec/rule surfaces (CLI + MCP parity)
- `7f1ed44f` — [session-mcp][schema] Enum-constrain and document workflow mutation parameters
- `84c7757d` — Session store: populate tool-metrics, capture tool error text, and count timeouts/hangs as non-success
- `85012858` — Research lifecycle engine design surfaces
- `8a3ad90a` — [ticket-cli][ticket-api] Safe dangling-edge remediation workflow
- `8bb97b73` — [ticket-cli][ticket-mcp][session-mcp] Explain invalid state/enum transitions with allowed values
- `90279c46` — [hooks][rule] Make pre-commit validate only repo-local rule targets
- `a2c469c4` — [workflow-policy][benchmarks] Research and define end-to-end benchmark and execution-evidence policy
- `a71c2da8` — [workflow-policy][tracing][log-api] Research and define tracing instrumentation and log execution policy
- `b4f444ee` — Move context-stack to repo root and remove deprecated folders
- `ba8f5528` — [session-api][handoff] Render resolved ticket narrative and upward context in handoff markdown
- `bba9b313` — [session-api][session-cli][session-mcp] Add sessions_for_ticket query with selectable relation-strength tiers
- `c96f325f` — [audit instructions] Focus audit guidance on executing target-context audits and canonical findings summaries
- `db9bad13` — [log-api][test-api][journal] Normalize artifact routing for executions, runtime sessions, and journals
- `e179f11a` — [audit-roadmap][static_complexity][batch-3] memory-api (28)
- `e2f25c12` — Retire the rule system: delete rule stores and rule-targets, freeze generated docs
- `e4d4c667` — [session-api] sessions_for_ticket aborts the whole scan on a malformed/corrupt session store entry
- `e70471d4` — Generalize mcp-toolmon path rewriting beyond the workspace argument
- `f147eb0e` — Migrate recurring spec principles to canonical rule entries via spec sync-generated
- `f52cc8e5` — [tooling] Define executable and hook registry schema with Markdown catalog generation
- `f93e5db5` — [token-efficiency] Replace repo_map Python generation with peek-api folder skeleton tree output
- `f97d7086` — [planning][workspace-policy] unify recovery hints across memory-api stores
- `f9f46954` — [context-engine] Fix VS Code Copilot hook file path
- `fd374421` — compact-terminal-mcp hangs: spawned shell inherits the server's MCP stdin
- `fdf53556` — [ticket-api][ticket-cli][ticket-mcp] Ticket state-transition recovery contract: report current + allowed next states, intermediate states, HTTP parity, inspection command

## Follow-up

Audit every reported closure against its acceptance criteria and recorded validation evidence. Reopen tickets whose acceptance criteria were neither met nor explicitly deferred.
