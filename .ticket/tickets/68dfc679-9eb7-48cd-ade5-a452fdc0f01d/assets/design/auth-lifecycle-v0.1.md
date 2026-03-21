# Auth Token Lifecycle v0.1

Status: draft for review

## Source precedence
1. In-memory override (admin reload command)
2. Environment variable
3. Config file token list

## Token model
- Active set includes:
  - `primary_token`
  - optional `grace_tokens[]`
- Grace window default: 15 minutes.

## Reload protocol
1. Load candidate token config.
2. Validate format and minimum entropy/length constraints.
3. Build candidate token set.
4. Atomic swap to new set.
5. Emit `auth.reloaded` diagnostic event with generation id.

## Failure policy
- On validation failure, keep previous token set.
- Emit `auth.reload_failed` with reason and generation id.
- Do not interrupt active HTTP connections.

## Runtime metadata
- Expose `auth_generation` and `auth_last_reload_ts` in health diagnostics.
- Audit log entries:
  - `auth.reload.start`
  - `auth.reload.success`
  - `auth.reload.failed`

## Security constraints
- Never log raw token material.
- Store only hashed token fingerprints in diagnostics.

## Operator runbook (summary)
- Rotate:
  - publish new token in env/config
  - trigger reload
  - verify health generation bump
  - remove old token after grace window
