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

## Cross-store prerequisite — consolidation via move tooling (recorded textually; edges cannot cross stores)
The hard-link cross-store reference work has a prerequisite that lives in the **memory-api** store and therefore cannot be graph-edged from this `default`-store tracker:
- move tooling: memory-api `505b2cd4` "Deliver safe cross-workspace ticket move for git-backed stores" (+ children) — delivers safe, journaled, ref-relinking moves.
- cleanup migration: memory-api `7599ed31` "Migrate misplaced context-engine-workspace tickets into the memory-api store" — depends_on `505b2cd4`; consolidates the misplaced `default`-store entities (session-bootstrap, URN, feedback-api tickets) into memory-api so intra-store hard edges become possible.
- the memory-api hard-link tickets `b03be2d5` / `f00291a3` now depend_on `7599ed31`.

OPEN DECISION (see Phase C): once entities are consolidated into one store via the move tooling, evaluate whether full cross-store URN references are still required, or whether intra-store edges + a thin URN facade suffice. Resolve before committing Phase C scope.