# [AOH][Design] Operator Authorization, Secret Lifecycle, and Trust Boundaries

## Objective

Define the security and trust model for AOH before implementation starts.

Current tickets describe messenger approvals, container secret injection, remote GitHub actions, and operator control, but there is no dedicated ticket that answers who is allowed to do what, how secrets move through the system, or how the orchestrator proves that an approval/termination command came from an authorized human.

## Why This Is Missing but Critical

The current plan already assumes all of the following:
- users can approve/reject work from messenger adapters
- containers can fetch one-time secrets from the orchestrator
- local and remote git operations can happen under agent identities
- logs and reports are safe to surface in TUI and messenger notifications

Without a security contract, these assumptions are unsafe and ambiguous.

## Questions to Resolve

### Operator identity and authorization
1. Which identities are trusted to control AOH: local OS user only, configured allow-list, GitHub identity mapping, or per-messenger user IDs?
2. Can messenger users approve merges in v1, or are messenger adapters notify-only until identity mapping exists?
3. Which actions require stronger trust than others?
   - approve/reject review
   - extend budget
   - terminate session
   - push remote branch
   - merge to main
4. How is operator identity represented in audit logs?

### Secret lifecycle
5. Which secrets exist in v1?
   - Copilot token
   - GitHub token/app credentials
   - messenger bot tokens
   - per-agent SSH keys, if used
6. Where are secrets stored at rest?
7. How are secrets scoped per session and expired?
8. How are secrets rotated and revoked?

### Container trust boundary
9. How does a container reach the secret server under Docker Desktop/WSL2 and Linux CI?
10. How do we prevent the secret-fetch path from being replayed by another process/container?
11. What host resources are intentionally exposed to containers, and which are forbidden?

### Log/output safety
12. Which fields must always be redacted from:
   - TUI logs
   - messenger notifications
   - session archives
   - PR metadata
13. What is the failure behavior if redaction cannot be proven?

## Alternatives to Consider

### Approval model alternatives
- **Option A**: TUI-only approval/merge in v1; messenger is notify-only
- **Option B**: Messenger approval allowed for low-risk actions only; merge still TUI-only
- **Option C**: Full messenger control with explicit operator allow-list and signed action tokens

### Secret delivery alternatives
- **Option A**: One-time HTTP/Unix-socket secret fetch with expiry and nonce
- **Option B**: tmpfs-mounted secret files created by orchestrator and removed on cleanup
- **Option C**: short-lived env vars accepted only for local dev mode, forbidden in CI/prod

## Deliverables

- Threat model and trust-boundary diagram
- Operator authorization model for TUI and messenger actions
- Secret inventory with storage, scope, lifetime, and rotation rules
- Redaction policy for logs, notifications, archives, and PR metadata
- Recommendation for v1 approval mode and secret transport

## Acceptance Criteria

- [ ] Trusted operator identities and allowed control actions are defined
- [ ] v1 decision made: messenger notify-only vs limited control vs full control
- [ ] Secret inventory and lifecycle matrix documented for all v1 credentials
- [ ] Secret delivery path specified for Docker Desktop/WSL2 and Linux CI
- [ ] Replay protection / nonce / expiry rules defined for secret fetch and operator actions
- [ ] Redaction rules documented for logs, notifications, archives, and PR metadata
- [ ] Failure mode defined when auth or redaction checks fail
- [ ] Architecture ticket updated to reference the approved trust model