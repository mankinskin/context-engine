# Use Case: Conflict Avoidance and Active Work Leases

## Goal

Prevent agents from working on conflicting items concurrently and provide clear visibility into active ownership.

## Preconditions

- Per-ticket locks are implemented.
- Ticket schema supports lease fields (`working_by`, `lease_expires_at`, `work_intent`).
- Conflict rules exist (for example same module ownership, mutually exclusive edge sets).

## Scenario

1. Agent requests a work lease for a candidate ticket.
2. System checks hard conflicts:
   - ticket already leased,
   - ticket blocked by unresolved dependencies,
   - conflict domain overlap with active tickets.
3. If clear, lease is granted with TTL and heartbeat requirements.
4. Agent heartbeats lease while actively working.
5. If heartbeat stops, lease expires and ticket returns to open queue.
6. Conflict detector continuously scans active leases and warns about emerging overlaps.

## Data Flows

- Index: active lease table keyed by ticket UUID.
- Filesystem: optional lock sidecar `.ticket-lock` and lease metadata in `ticket.toml`.
- Query: `state:in-progress lease_active:true conflict_domain:<x>`.

## Concurrency Rules

- Lock and lease are distinct: lock is short-lived for writes, lease is long-lived for ownership.
- Lease mutation requires lock to ensure monotonic lease transitions.
- Lease renewals are idempotent.

## Failure Modes

- Ghost leases after crash: TTL expiry and watchdog cleanup.
- Clock skew affecting TTL: use monotonic service clock where possible.
- Manual filesystem edits bypassing lease rules: watcher reconciles and flags violations.

## Success Metrics

- Collision rate (duplicate work on same concern).
- Lease timeout recovery time.
- Number of prevented conflicts per sprint.
