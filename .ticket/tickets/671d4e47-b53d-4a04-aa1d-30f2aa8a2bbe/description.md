Goal: deliver a workspace-wide, domain-isolated multi-store architecture where each store owns persistence and workflow behavior while cross-store interactions are defined by shared contract interfaces.

Decisions locked:
- Contract topology: hybrid contract layer (small shared core plus domain extension crates).
- Cross-store reference identity: URN form `ce://<workspace>/<store>/<entity>`.
- Discovery mode: fully automatic recursive discovery across local and nested workspaces.
- Error envelope target: extended schema with `code`, `message`, `request_id`, `details`, `cause_chain`, `hint`, and `remediation_id`.

Scope:
- establish architecture contract and migration phases for memory-api, ticket-api, spec-api, rule-api, audit-api, and peers
- coordinate child vectors for storage neutralization, IoC contracts, discovery/references, and diagnostics
- enforce compatibility gates so migration can ship incrementally without breaking active workflows

Migration phases and tracker gates:
1. Phase A: neutral shared vocabulary and compatibility aliases.
2. Phase B: hybrid contract crates and binary composition wiring.
3. Phase C: recursive discovery and URN-based cross-store references.
4. Phase D: extended diagnostic envelope across CLI, MCP, and HTTP.
5. Phase E: retire legacy ticket-biased shared API names after adoption proof.

Acceptance criteria:
- dependency graph is complete and each child ticket has implementation-ready scope
- architecture spec captures final decisions, phase gates, and practical examples
- each child ticket defines validation evidence expectations
- all tickets pass health checks with no missing required planning metadata
