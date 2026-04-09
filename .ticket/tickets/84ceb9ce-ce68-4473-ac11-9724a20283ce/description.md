# [Board][Design] Entry Identity, Resume Flow, and Synchronization Invariants

## Objective

Close the remaining correctness gaps around what a draftboard entry actually represents, how an agent resumes existing work, and how board state stays synchronized with leases and ticket state transitions.

The current design is strong on surface API shape, but it still leaves several foundational invariants underspecified. If these questions are not answered before implementation, the first failures will be subtle synchronization bugs rather than obvious command errors.

## Why This Needs Its Own Ticket

The current design uses a compound key of `"{ticket_id}:{agent_id}"` and retains completed entries through an audit window. That creates immediate ambiguity when the same agent later resumes or re-checks into the same ticket:

- does the old completed entry get overwritten?
- does the new check-in fail because an entry already exists?
- do we need a stable `entry_id` / `session_id` independent of agent and ticket?

There is also no fully defined source-of-truth hierarchy between:
- board entries
- leases (`claim` / `unclaim`)
- ticket state (`new`, `in-implementation`, `in-review`, ...)

Without explicit invariants, the system will drift under normal use.

## Concrete Problems to Resolve

### 1. Entry identity and reuse
- Is a board entry identified by `(ticket_id, agent_id)` or by a separate immutable `entry_id` / `session_id`?
- What happens when the same agent checks back into the same ticket before the old completed entry has been cleaned?
- Can multiple agents be checked into the same ticket at the same time if they claim disjoint files?
- If yes, how are they displayed and filtered in `next`, `status`, and `board show`?

### 2. Resume and handoff flow
- When an agent starts and already has an active or stale board entry, what is the expected resume workflow?
- Should `board_show(agent_id=self)` highlight "your existing work" separately from the global board?
- Should `next` / `next_tickets` return a resume recommendation before proposing new work?
- How does intentional handoff between agents work for the same ticket?

### 3. Synchronization invariants
- What must always be true between board entry state, lease state, and ticket state?
- If a ticket is closed, cancelled, reverted, or moved back from `in-review`, what board updates are required?
- What happens if someone uses `claim` / `unclaim` directly without the board commands?
- Is the board the primary source of truth for short-term ownership, with leases as a compatibility mirror, or are they peers?

### 4. File ownership canonicalization
- How are file paths normalized before conflict detection?
- How do we handle Windows vs WSL path separators and case sensitivity?
- How do we represent renamed files, deleted files, and generated outputs?
- Is directory-level ownership supported, or files only?

## Deliverables

- Canonical identity model for board entries
- Resume / handoff workflow for agents with existing entries
- Synchronization invariants between board, leases, and ticket state
- File ownership canonicalization rules for cross-platform use
- Recommendation for whether same-ticket multi-agent work is supported in v1

## Acceptance Criteria

- [ ] Board entry identity is defined unambiguously (`entry_id` / `session_id` vs composite key)
- [ ] Re-check-in semantics are defined for the same agent and ticket across audit windows
- [ ] Resume workflow is specified for agents that already own active or stale entries
- [ ] Same-ticket multi-agent semantics are either explicitly supported or explicitly forbidden in v1
- [ ] Synchronization invariants between board entries, leases, and ticket states are documented
- [ ] Required hooks are identified for ticket close/cancel/revert/update paths
- [ ] File ownership canonicalization rules are specified for Windows/WSL/Linux
- [ ] `8aff39cb` is updated to reference the approved identity and synchronization contract
