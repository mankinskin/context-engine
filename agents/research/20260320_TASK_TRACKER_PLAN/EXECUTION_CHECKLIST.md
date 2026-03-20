# Task Tracker Plan — Global Execution Checklist

Status: ACTIVE
Owner: Coordinator
Last updated: 2026-03-20

## Goal

Drive the Task Tracker plan to completion with strict work-in-progress limits.
Do not run too many topics in parallel. Finish current topics before starting new ones.

## WIP Policy (mandatory)

- Global major-topic WIP limit: 2
- Per agent WIP limit: 1 claimed ticket
- Validation queue target: <= 5 tickets waiting
- New major topic can start only if one active topic is marked COMPLETE

If WIP limit is exceeded:
1. stop creating new tickets
2. pause non-critical work
3. push active tickets to validation and closure

## Active Topics (Now)

### Topic A — Phase 1 Core Backend + Search + Agent Exec
Reference:
- 01_phase_minimal_backend/PLAN.md
- PROTOCOL_LAYER.md

Scope:
- real storage wiring for create/get/update/list/delete/scan/search
- ticket exec single-command JSON path
- ticket exec --batch transactional behavior
- explicit index_root + full UUID enforcement
- fields projection support for agent responses

Exit gates:
- [ ] command handlers are real (no stubs)
- [ ] crash/reconcile smoke path passes
- [ ] search returns stable mixed metadata + text results
- [ ] exec path parity with CLI command behavior is verified
- [ ] transactional batch rollback boundary is tested

### Topic B — Phase 1.5 Lease + Serve Stdio + Validation Assignment Rules
Reference:
- 015_phase_lease_protocol/PLAN.md
- VALIDATION_RELEASE_GOVERNANCE.md
- HOST_EXECUTOR_AUTH_PROVIDER.md

Scope:
- claim/unclaim/heartbeat/leases
- ticket serve --stdio request/response session
- session-liveness auto-renewal
- validator/worker separation-of-duties check on validating assignments
- host executor auth skeleton (ephemeral worker tokens, assignment-scoped authorization)

Exit gates:
- [ ] claim collision and stale reclaim tests pass
- [ ] serve session can process multi-command workflow with request IDs
- [ ] session drop stops auto-renewal and TTL expiry works
- [ ] validator cannot equal worker for validating assignment

## Queued Topics (Do Not Start Until a Slot Opens)

- Topic C — Phase 2 History + Rollback
- Topic D — Phase 3 Graph + Merge Queue
- Topic E — Phase 4 Dogfooding rollout and gates
- Topic F — Phase 5 Integrations (HTTP/MCP dashboards + messenger)

## Ticket Flow Checklist (Coordinator)

For every ticket moved to in-progress:
- [ ] acceptance criteria is explicit
- [ ] risk_level is set
- [ ] validation_plan is set
- [ ] release_target is set
- [ ] owner/worker/validator assignment is valid

Before moving to validated:
- [ ] worker and validator identities differ
- [ ] evidence_refs include tests and command outputs
- [ ] linked bugs created for discovered defects

Before moving to release-candidate:
- [ ] validation_status = passed
- [ ] no open sev0/sev1 linked bugs
- [ ] dependency blockers are cleared

## Bug and Release Fast-Path

When validation fails:
1. open linked bug ticket immediately
2. set source ticket back to review
3. assign fix owner with high priority if sev0/sev1

For stable release candidate:
- [ ] all included tickets are validated
- [ ] release smoke suite passes
- [ ] rollback path is verified
- [ ] post-release monitoring window is planned

## Daily Execution Rhythm

1. WIP audit (enforce max 2 major topics)
2. close oldest in-progress tickets first
3. drain validation queue
4. promote validated tickets to release-candidate only when gates pass
5. publish short status digest: now, blocked, next

## Completion Rule

A topic is COMPLETE only when:
- all topic exit gates are checked
- no blocking sev0/sev1 bug remains linked to that topic
- coordinator publishes completion note and opens at most one new topic from queue
