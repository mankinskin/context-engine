# Design: auth token lifecycle and rotation/reload behavior

## Objective
Define how ticket serve token auth can rotate safely without process restarts.

## Proposed model
- Sources (highest priority first):
  - in-memory override (admin signal)
  - env var
  - config file
- Active token set supports one primary and one grace token during rotation.

## Reload strategy
- Trigger options:
  - SIGHUP/manual command hook
  - periodic config poll (optional)
- Reload behavior:
  - validate new token set before swap
  - atomic swap active token map
  - emit `auth.reloaded` event

## Failure handling
- Invalid token config keeps previous valid token set.
- Emit `auth.reload_failed` diagnostic event with reason.
- Rate-limit reload attempts on persistent failures.

## Operational notes
- Audit log for token reload actions.
- Health endpoint includes auth config generation/version metadata.

## Draft artifacts
- Lifecycle and runbook draft: `assets/design/auth-lifecycle-v0.1.md`

## Checklist
- [x] Source precedence finalized
- [x] Reload trigger mechanism finalized
- [x] Grace-window semantics finalized
- [x] Failure rollback behavior finalized
- [x] Runbook written
