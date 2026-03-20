# Phase 4 — Transition to Dogfooding the Task Tracker

Status: PLANNED

## Objective

Define a safe transition from planning-only usage to using the task tracker itself to manage ongoing tracker improvements.

## Why this phase exists

Problem:
- Switching too early creates a bootstrap loop where immature tooling blocks its own development.

Solution:
- Introduce maturity gates and an incremental adoption rollout with rollback paths.

Reference:
- Existing phase plans in this folder and current Phase 0 execution checklist.

## Remaining Work Before Dogfooding

The following major work remains from current plans:

1. Phase 0 closure — DONE
- Contract version, schema version, grammar version, guaranteed command list published.

2. Phase 1 core backend + search
- Wire create/get/update/list/delete to real storage implementations.
- Complete watcher + reconcile integration.
- Wire unified query parser to Tantivy execution path.
- Implement Tantivy indexing lifecycle and full reindex path.

3. Phase 1.5 lease protocol
- Implement claim/unclaim/heartbeat commands.
- Complete lease semantics and lock behavior in full workflow paths.
- Stale lease recovery proven by tests.

4. Phase 2 history
- Implement history log/diff/revert commands against git-backed store.
- Finalize and enforce branch-boundary lifecycle behavior.

5. Phase 3 graph workflows
- Implement dependency traversal/validation commands.
- Add merge queue and board views with lease/blocker overlays.

6. Operational readiness
- Define migration and backup/restore procedures.
- Add smoke/integration tests for end-to-end command workflows.
- Add observability and diagnostics standards for worker failures.

7. Integration readiness (Phase 5)
- Implement visualization endpoints and automated artifact generation.
- Define messenger event routing and digest policies for long-running swarms.

## Maturity Gates (must be green)

Gate A: Core command reliability
- create/get/update/list/delete/scan/claim/unclaim are implemented (not draft stubs).
- Command JSON envelopes are stable and versioned.

Gate B: Crash and reconcile safety
- Simulated crash during write recovers via scan/reconcile.
- Stale lease recovery and lock cleanup are proven by tests.

Gate C: Auditability
- history/diff/revert work end-to-end and preserve forward-only history semantics.

Gate D: Search and retrieval
- Unified query supports mixed metadata + free text on real indexed content.

Gate E: Graph correctness
- blocked-by/blocking/validate-graph pass known scenario tests.

Gate F: Lease stability
- claim/unclaim/heartbeat cycle proven under parallel swarm load.
- Stale lease recovery and conflict domain collision tested.

Gate G: Operator trust
- One week of internal trial with no critical data-loss or deadlock incidents.

## Transition Rollout

### Stage 1 — Mirror mode
- Continue using current planning docs as source of truth.
- Mirror a subset of tracker-improvement work into task tracker tickets.
- Compare outcomes and identify workflow gaps.

Exit criteria:
- 90% parity between doc-tracked and tracker-tracked status for mirrored items.

### Stage 2 — Hybrid mode
- New improvement work starts in task tracker.
- Planning docs remain authoritative for major architecture decisions.
- Weekly reconciliation report between docs and tracker state.

Exit criteria:
- Two consecutive weeks with no blocker caused by tracker workflow limitations.

### Stage 3 — Tracker-first mode
- Task tracker becomes primary source of truth for implementation tickets.
- Planning docs used for roadmap/design only.
- Add periodic exports/snapshots for archival and review.
- Begin Phase 5 integrations: visualization endpoints first, messenger delivery second.

Exit criteria:
- Full team/agent adoption and successful release cycle completed using tracker-first process.

## Governance Rules During Transition

- Every tracker ticket must include acceptance criteria and explicit owner/worker fields.
- State transitions must be machine-validated; no manual state drift.
- Any critical tracker defect that blocks workflow creates a high-priority bootstrap ticket.
- Bootstrap tickets may temporarily be managed in docs until blocker is resolved.

## Bootstrap Ticket Template

Use this template for tracker-improving-tracker work:

- title: [bootstrap] <short task>
- type: tracker-improvement
- state: open
- fields:
  - component: cli | storage | history | search | graph | watcher | lease
  - risk_level: low | medium | high
  - acceptance_criteria: explicit checklist
  - bootstrap_blocker: true | false
  - rollout_stage: mirror | hybrid | tracker-first

## Immediate Next Actions

1. ~~Finalize Phase 0 exit decision~~ DONE — scoped to context-tasks crate.
2. Replace command stubs with backend wiring for create/get/update/list/delete (Phase 1).
3. Implement one end-to-end bootstrap path:
   create ticket → claim → update → history → unclaim → close
4. Start Stage 1 mirror mode with 5-10 tracker-improvement tickets.

## Seed Backlog Artifacts

- `BOOTSTRAP_TICKETS.jsonl`:
  machine-friendly seed set of 10 bootstrap tickets aligned with the transition stages.
- `BOOTSTRAP_TICKETS_COMMANDS.sh`:
  runnable `ticket create` commands for seeding the same backlog through CLI.
- `BOOTSTRAP_TICKET_DEPENDENCIES.jsonl`:
  machine-friendly edge set describing ordering constraints between bootstrap tickets.
- `BOOTSTRAP_DEPENDENCY_COMMANDS.sh`:
  runnable `ticket update --field blocked_by=...` commands to apply dependency hints
  until dedicated edge commands are fully wired.
