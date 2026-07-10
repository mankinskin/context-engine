# CH12 — Docs/runbooks + rollout checklist

WS7 release readiness.

## Scope
- Architecture notes for the agent-harness crates and event/session contracts.
- Operator runbook: start/observe/pause/resume/stop, budget config, sandbox setup.
- Failure-handling guide (recovery from checkpoint, reconnect, watchdog).
- Rollout checklist tying back to spec acceptance criteria.

## Acceptance criteria
- Docs cover architecture, operator runbook, and failure handling.
- Rollout checklist maps each spec acceptance criterion to its validation evidence.
- Doc validation workflow (if applicable) passes.

## Dependencies
- depends_on CH11 (evidence exists to document). Spec: unified-operator-interface (Definition of Done).

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | Markdown lint / link check on new docs |
| Primary gate | Doc validation workflow (if present) + reviewer confirmation checklist complete |
| Manual/browser | Not applicable |
| Failure logs | `target/test-logs/` where doc tooling emits logs |
