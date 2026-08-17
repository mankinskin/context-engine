# Review: Session Hook Routing and Refactoring Policy

## Verdict

**Changes requested.** The cleaned request identifies credible maintenance and
routing risks, but it combines a repository-wide automatic-refactoring policy,
a capture-hook decomposition, and a new session lifecycle in one unbounded
proposal. The current implementation instead eagerly provisions a worktree on
`SessionStart` and resolves MCP targets by positional `.worktrees` discovery;
the proposed main-checkout-first transfer model is therefore an architectural
change, not a local repair. The reported ticket write in the main checkout
cannot be classified as a regression without a reproducible session id, hook
payload, proxy configuration, and resulting store paths.

## Research Findings

| Request claim | Repository evidence | Verdict |
| --- | --- | --- |
| The capture hook has mixed responsibilities and needs decomposition. | [memory-api/crates/session-capture-hook/src/main.rs](memory-api/crates/session-capture-hook/src/main.rs) combines hook capture, provisioning, resolver use, spill statistics, feedback reporting, ticket synthesis, and metrics rollup; `src/args.rs` is the only other source module. | confirmed |
| Agents lack guidance to start a refactor when encountering an oversized or unstructured artifact. | [.agents/instructions/](.agents/instructions/) has no such trigger. [engine/core-crates.instructions.md](.agents/instructions/engine/core-crates.instructions.md) instead directs engine-crate edits toward minimal local changes over broad refactors. | confirmed |
| The “MCP proxy” fails to resolve the active worktree. | The concrete proxy is [memory-api/tools/mcp/mcp-toolmon/src/proxy.rs](memory-api/tools/mcp/mcp-toolmon/src/proxy.rs), which delegates workspace selection to [memory-api/crates/session-workspace-resolver/src/lib.rs](memory-api/crates/session-workspace-resolver/src/lib.rs). The implementation can reject missing or ambiguous worktrees and blocks main-checkout targets, but no supplied reproduction proves the reported `ticket-extraction-finish` misroute. | partially confirmed |
| Capture should start in the main checkout, then move the session to a later worktree. | [memory-api/crates/session-capture-hook/src/main.rs](memory-api/crates/session-capture-hook/src/main.rs) provisions eagerly only for `SessionStart`; [memory-api/crates/session-workspace-resolver/src/lib.rs](memory-api/crates/session-workspace-resolver/src/lib.rs) discovers a worktree positionally and has no main-checkout session registry or session-transfer operation. | contradicted |
| Session transfer must be atomic and preserve main-checkout history before the move. | [memory-api/crates/session-worktree-provision/src/lib.rs](memory-api/crates/session-worktree-provision/src/lib.rs) exposes worktree Git operations and [memory-api/crates/session-worktree-provision/src/policy.rs](memory-api/crates/session-worktree-provision/src/policy.rs) provisions, reuses, and reclaims worktrees, but neither surface establishes an atomic cross-store session transfer contract. | partially confirmed |
| Deregistration after merge/removal must restore main-checkout routing. | [memory-api/crates/session-worktree-provision/src/policy.rs](memory-api/crates/session-worktree-provision/src/policy.rs) models reclaim ownership, while [2b657154 Handle unregistered worktree debris during removal](.ticket/tickets/2b657154-df78-4bb3-807a-66c9ff811ceb/ticket.toml) concerns debris removal. Neither establishes the requested session deregistration and routing-reset lifecycle. | partially confirmed |
| The reported failure is already fully covered by an existing ticket. | [200e9ecc session_check_in rejects a single nested worktree unless a persisted assignment exists](.ticket/tickets/200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915/ticket.toml) covers the positional-discovery versus persisted-assignment contradiction, whereas the request additionally proposes a different routing lifecycle and refactoring policy. [0afe45b5 [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store](.ticket/tickets/0afe45b5-9ec8-4f4a-af74-f46f06cc7516/ticket.toml) fixed a related prior store-anchoring defect but does not prove the new report is a regression. | partially confirmed |
| The `ticket-extraction-finish` MCP write proves capture-hook registration failed. | The supplied account names a symptom but supplies no captured hook event, `mcp-toolmon` resolution result, session record, or before/after store paths. | unverifiable without reproduction |

## Findings and Required Improvements

| ID | Finding | Severity | Required improvement |
| --- | --- | --- | --- |
| F1 | “Automatically trigger a refactor” has no threshold, authority, or exception rule and conflicts with the existing minimal-local-change guidance for engine crates. | blocker | Define measurable trigger signals, a decision owner, an opt-out rule, and whether the policy creates a recommendation, a review finding, or mandatory follow-up work. |
| F2 | Capture-hook decomposition is conflated with the session-routing redesign. The two concerns have different owners, risks, and validation methods. | major | Separate the policy/decomposition concern from routing semantics before ticketing; the routing package must not presume a particular module layout. |
| F3 | The requested main-checkout-first lifecycle contradicts current eager provisioning and positional resolver discovery without identifying the migration authority or compatibility behavior. | blocker | Select a lifecycle authority and state how existing nested and legacy worktrees, in-flight sessions, and failed moves are handled. |
| F4 | “Move commits the session entry before worktree creation” is ambiguous about transaction boundaries, rollback, and the authoritative store after a partial failure. | blocker | Specify one atomic operation with durable preconditions, rollback/recovery semantics, and read-back evidence for both stores. |
| F5 | Deregistration is under-specified: “merged and deleted” does not define who verifies merge equivalence, what happens to dirty or ahead branches, or how hooks are suppressed. | major | Define legal deregistration preconditions, the actor, idempotence, hook-suppression scope, and the observable postcondition. |
| F6 | The reported misrouting is a symptom report, not a deterministic reproduction, so classifying it as a regression of [0afe45b5 [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store](.ticket/tickets/0afe45b5-9ec8-4f4a-af74-f46f06cc7516/ticket.toml) would overclaim. | major | Require a minimal reproduction that records session id, process cwd, `MCP_MAIN_CHECKOUT`, resolver result, hook diagnostic, and actual mutated store path. |
| F7 | [200e9ecc session_check_in rejects a single nested worktree unless a persisted assignment exists](.ticket/tickets/200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915/ticket.toml) is related but not a duplicate: the open ticket repairs the current positional model, while the request proposes replacing that model. | major | Decide whether the later routing work preserves and fixes positional discovery first, or explicitly supersedes the current contract with a migration plan. |
| F8 | The ticket-viewer browser-E2E/merge decision is an unrelated review of [ba4aaa9c [workflow-tools][per-tool] Extract ticket tool as a single `ticket` domain crate (api + transport bins) + viewer/vscode frontends](.ticket/tickets/ba4aaa9c-d270-4cfc-a1e2-395634608371/ticket.toml), not an acceptance decision for session routing. | minor | Route the merge decision back to the requester as a separate review question; do not include it in the routing dossier. |

## Scope Decision

A subsequent Stage 3 dossier will cover only:

- a bounded evidence inventory for capture-hook, `mcp-toolmon`, resolver, provision, and session-store ownership surfaces;
- a reproducible routing-failure protocol that distinguishes hook capture, resolver selection, proxy forwarding, and store mutation;
- alternative session-lifecycle designs, including compatibility and recovery constraints for the current positional model;
- the atomic transfer and deregistration contracts, including authority, rollback, idempotence, and read-back validation;
- a separately bounded refactoring-policy proposal with trigger thresholds and explicit interaction with existing minimal-change guidance;
- the relationship of the request to [200e9ecc session_check_in rejects a single nested worktree unless a persisted assignment exists](.ticket/tickets/200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915/ticket.toml), [0afe45b5 [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store](.ticket/tickets/0afe45b5-9ec8-4f4a-af74-f46f06cc7516/ticket.toml), and [2b657154 Handle unregistered worktree debris during removal](.ticket/tickets/2b657154-df78-4bb3-807a-66c9ff811ceb/ticket.toml).

A subsequent Stage 3 dossier will not cover:

- implementation, source-code edits, ticket creation, specification changes, or workflow-state transitions;
- the prior ticket-domain extraction background or any decision to merge [ba4aaa9c [workflow-tools][per-tool] Extract ticket tool as a single `ticket` domain crate (api + transport bins) + viewer/vscode frontends](.ticket/tickets/ba4aaa9c-d270-4cfc-a1e2-395634608371/ticket.toml);
- dispatching ticket-viewer browser or Playwright E2E;
- automatic refactoring of the capture hook or any other crate;
- changing the current eager-provisioning or positional-discovery behavior before a chosen lifecycle contract exists;
- treating the reported worktree misroute as a confirmed regression without reproduction evidence.

The ticket-viewer E2E/merge question must be routed back to the requester separately.

## Open Questions

1. Should the session-routing design retain the current positional `.worktrees/<session-id>/<slug>` discovery after a worktree exists, or replace it with a main-checkout registry that becomes authoritative after transfer? Retaining positional discovery minimizes migration work but does not provide the requested transfer record; a registry enables transfer/deregistration state but requires migration and recovery rules.
2. When a session has no worktree at `SessionStart`, should the capture hook continue current eager provisioning or keep capture in the main checkout until an agent explicitly requests a worktree? Eager provisioning preserves current isolation behavior; deferred provisioning enables the requested lifecycle but permits main-checkout capture before transfer.
3. Should an automatic-refactoring rule create only a review finding, require a tracked follow-up, or block feature work once thresholds are exceeded? A review finding is lowest disruption, a follow-up makes debt durable, and a blocking rule enforces consistency but can delay urgent fixes.
4. Should a completed merge of [ba4aaa9c [workflow-tools][per-tool] Extract ticket tool as a single `ticket` domain crate (api + transport bins) + viewer/vscode frontends](.ticket/tickets/ba4aaa9c-d270-4cfc-a1e2-395634608371/ticket.toml) wait for ticket-viewer browser E2E, or may manual review authorize merge while E2E remains deferred? Waiting requires executable browser evidence before merge; manual authorization accepts the documented validation gap.