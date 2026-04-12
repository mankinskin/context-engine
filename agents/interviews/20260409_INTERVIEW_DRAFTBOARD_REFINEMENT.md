---
tags: `#board` `#ticket-system` `#coordination` `#refinement`
summary: Interview guide for resolving the open draftboard refinement topics in manageable batches
status: Draft
---

# Interview: Draftboard Refinement, Cleanup Workflow, and Synchronization Invariants

**Date:** 2026-04-09  
**Feature:** Workspace draftboard for concurrent-agent coordination  
**Tickets:** `84ceb9ce`, `c3143e3c`, `be38e809`  
**Status:** Refinement / Interview

---

## How To Use This Interview

Work through **one batch at a time**. Each batch is small enough to answer in one pass without needing the whole design in your head.

- Batches 1-4 resolve `84ceb9ce`.
- Batches 5-6 resolve `c3143e3c`.
- Batch 7 sets the validation priorities for `be38e809`.

If you want to answer quickly, you can reply with compact forms like `Q1: B`, `Q2: A+C`, or freeform text when needed.

---

## Batch 1: Entry Identity and Reuse

### Q1. Board Entry Identity
What should uniquely identify a board entry?

- [ ] A. **Composite key only** — `(ticket_id, agent_id)` is the identity
- [ ] B. **Immutable entry ID** — every check-in gets a new `entry_id` / `session_id`
- [x] C. **Hybrid** — every entry has an immutable `entry_id`, but there may be uniqueness rules on `(ticket_id, agent_id)` for active entries
- [ ] D. **Other** — Describe: ___

**Answer:** C. Use a hybrid model.

- Every check-in creates a new immutable `entry_id` / `session_id` for auditability, retries, and crash recovery.
- In v1, enforce at most one non-terminal active/stale entry per `(ticket_id, agent_id)`.
- Completed entries remain historical attempts and are never overwritten in place.

---

### Q2. Same-Agent Re-Check-In Before Cleanup
If the same agent checks back into the same ticket while an older completed entry still exists in the audit window, what should happen?

- [ ] A. **Reject** — agent must explicitly resume or clean first
- [ ] B. **Auto-resume old entry** — completed/stale entry becomes active again
- [x] C. **Create new attempt** — keep old entry for audit, create a fresh entry linked to it
- [ ] D. **Prompt choice** — resume old vs create new
- [ ] E. **Other** — Describe: ___

**Answer:** C. Create a new attempt.

- A completed entry should be treated as historical evidence, not silently resumed.
- The new entry should link back to the prior completed entry via a `previous_entry_id` or equivalent relation.
- This answer applies specifically to completed entries still inside the audit window. Active or stale entries should be handled through an explicit resume flow later.

---

### Q3. Multiple Agents on the Same Ticket
Should multiple agents be allowed to check into the same ticket at the same time?

- [x] A. **No** — one active board entry per ticket in v1
- [ ] B. **Yes, if files are disjoint** — allow parallel same-ticket work only with non-overlapping ownership
- [ ] C. **Yes, even with overlaps** — allow with explicit conflict state and human review
- [ ] D. **Other** — Describe: ___

**Answer:** A. Forbid same-ticket multi-agent check-in in v1.

- This keeps review, resume, stale cleanup, and ownership semantics simple for the first implementation.
- Parallel work should happen via sub-tickets in v1 rather than multiple board entries on the same ticket.
- File ownership tracking is still valuable across different tickets that touch the same files.

---

## Batch 2: Resume and Handoff Flow

### Q4. Resume Priority
If an agent starts a session and already has an active or stale entry, what should the system do before suggesting new work?

- [x] A. **Strongly recommend resume** — show existing work first and suppress new suggestions until acknowledged
- [ ] B. **Soft recommendation** — show existing work prominently, but still return new-ticket candidates
- [ ] C. **Ignore existing entry** — `next` only looks for new work
- [ ] D. **Other** — Describe: ___

**Answer:** A. Strongly recommend resume.

- The board is supposed to control short-term WIP, not just report it. If an agent already owns active or stale work, the system should push that work to the front before proposing new tickets.
- In v1, `next` should not silently treat existing work as optional background noise.
- Acknowledged escape hatches can still exist, but the default should be to resume first and only branch to new work intentionally.

---

### Q5. `board_show(agent_id=self)` Treatment
How should the board present the caller's own existing entries?

- [x] A. **Separate section** — `Your Active Work` / `Your Stale Work`
- [ ] B. **Inline only** — show in the global table with no special treatment
- [ ] C. **Resume-first view** — show only caller-owned entries first, then global board
- [ ] D. **Other** — Describe: ___

**Answer:** A. Use a separate caller-owned section.

- `board_show(agent_id=self)` should surface `Your Active Work` / `Your Stale Work` before the global board.
- This gives the agent immediate orientation without hiding the broader workspace situation.
- It also supports the resume-first policy from Q4 while preserving the board's role as a global coordination view.

---

### Q6. Intentional Handoff Between Agents
How should one agent hand work on a ticket to another?

- [x] A. **Check-out + new check-in** — no explicit handoff primitive in v1
- [ ] B. **Transfer action** — explicit ownership transfer preserving audit trail
- [ ] C. **Dual-entry handoff window** — both entries coexist temporarily until transfer completes
- [ ] D. **Other** — Describe: ___

**Answer:** A. Check-out followed by a new check-in.

- This matches the v1 rule of one active board entry per ticket.
- It avoids inventing a transfer state machine before the core board semantics are proven.
- If we need auditability, the check-out path can carry a handoff reason such as `handoff_to=<agent>` without introducing dual ownership.

---

## Batch 3: Source of Truth and Lifecycle Hooks

### Q7. Ownership Source of Truth
Which system is canonical for short-term ownership?

- [x] A. **Board is primary** — leases are compatibility mirrors, ticket state is advisory
- [ ] B. **Lease is primary** — board is a richer view over leases
- [ ] C. **Ticket state is primary** — board and lease derive from lifecycle state
- [ ] D. **Other** — Describe: ___

**Answer:** A. The board is the canonical short-term ownership system.

- The board is the only layer that can represent active ownership, file claims, stale status, human-review-needed cleanup, and WIP budgeting together.
- Leases become a compatibility mirror for lower-level reservation semantics rather than the main coordination model.
- Ticket state remains lifecycle/workflow state, not ownership truth.

---

### Q8. Raw `claim` / `unclaim` Commands
What should happen if someone uses raw lease commands outside the board flow?

- [ ] A. **Allow silently** — leases and board are independent systems
- [ ] B. **Allow but warn** — document that board is the preferred coordination layer
- [ ] C. **Mirror into board** — raw lease commands create/update board-visible state
- [x] D. **Deprecate for agents** — keep only for admin/debug use
- [ ] E. **Other** — Describe: ___

**Answer:** D, with a stronger v1 resolution: remove raw `claim` / `unclaim` as public workflow commands in favor of board commands.

- Agents should use `board check-in` / `board check-out` exclusively.
- Public raw lease mutation is too weak to preserve board invariants because it carries no file ownership, resume intent, stale-review semantics, or cleanup approval semantics.
- If a low-level lease primitive still exists internally, it should be treated as implementation detail or admin repair path, not a normal operator/agent interface.

---

### Q9. Ticket Lifecycle Hooks
Which ticket operations should trigger required board reconciliation?

- [ ] A. **Close / cancel only**
- [ ] B. **Close / cancel / revert**
- [ ] C. **Close / cancel / revert / update-state**
- [x] D. **All mutating ticket lifecycle operations**
- [ ] E. **Other** — Describe: ___

**Answer:** D. All mutating ticket lifecycle operations should trigger board reconciliation.

- This avoids hardcoding only today's lifecycle commands and then drifting when new state-transition paths are added later.
- At minimum, close, cancel, revert, and explicit state transitions must reconcile against board ownership.
- The invariant is broader than individual commands: whenever ticket lifecycle changes in a way that may invalidate active ownership, board state must be checked and updated or surfaced for review.

---

## Batch 4: File Ownership Rules

### Q10. Path Normalization Strategy
How should file paths be normalized before conflict detection?

- [ ] A. **Raw strings** — compare as provided
- [x] B. **Workspace-relative lexical normalization** — normalize separators, `.` / `..`, and canonical workspace-relative spelling
- [ ] C. **Filesystem canonicalization** — resolve real paths on disk when possible
- [ ] D. **Hybrid** — lexical normalization first, filesystem canonicalization only for existing files
- [ ] E. **Other** — Describe: ___

**Answer:** B. Use workspace-relative lexical normalization as the v1 conflict key.

- The board must support planned work on files that do not exist yet, so filesystem canonicalization would make conflict detection depend on incidental on-disk state.
- Lexical normalization gives a stable, portable comparison key: normalize separators, collapse `.` / `..`, and require paths to stay within the workspace root.
- Store and compare the normalized workspace-relative spelling, while optionally preserving the originally submitted spelling only for display or audit context.

---

### Q11. Case Sensitivity
How should case sensitivity be handled across Windows / WSL / Linux?

- [ ] A. **Always case-sensitive**
- [ ] B. **Always case-insensitive**
- [x] C. **Workspace/platform aware** — preserve actual filesystem semantics
- [ ] D. **Other** — Describe: ___

**Answer:** C. Respect workspace/platform case-sensitivity rules instead of forcing one global policy.

- A global case-sensitive rule would create false conflicts on Windows-style workspaces, while a global case-insensitive rule would hide real distinctions on Linux-style workspaces.
- The normalized ownership key should therefore be interpreted using the workspace's actual path semantics.
- If the workspace abstraction later spans multiple roots with different semantics, the rule should stay root-aware rather than collapsing to one repo-wide default.

---

### Q12. Ownership Scope in v1
What ownership model should v1 support?

- [ ] A. **Files only**
- [x] B. **Files + explicit renamed-file transitions**
- [ ] C. **Files + directories**
- [ ] D. **Files + generated output groups**
- [ ] E. **Other** — Describe: ___

**Answer:** B. Support file ownership plus explicit renamed-file transitions in v1.

- Plain file ownership is the simplest safe baseline, but rename flows are common enough that omitting them would push agents into manual workarounds and inconsistent conflict handling.
- Directory ownership and generated-output groups add too much ambiguity for a first release because their conflict surface is broad and hard to explain operationally.
- The v1 model should therefore allow an explicit transition that releases the old normalized file path and claims the new one as one audited ownership change.

---

## Batch 5: Stale Entry Review and Cleanup Approval

### Q13. Cleanup Approval Flow
How should explicit cleanup work for stale/completed entries?

- [ ] A. **Preview then confirm** — show planned removals, then require a second confirmation step
- [ ] B. **Single destructive command** — cleanup runs immediately when requested
- [x] C. **Confirmation token** — preview returns a token; cleanup requires that token
- [ ] D. **Other** — Describe: ___

**Answer:** C. Cleanup should use a preview-generated confirmation token.

- This keeps destructive cleanup explicitly tied to a reviewed candidate set instead of whatever happens to be stale when the apply command runs.
- The token gives us an auditable approval artifact and a natural place to encode snapshot/version checks so stale cleanup cannot silently race with board changes.
- A plain preview-then-confirm UX is still possible in CLI, but the token should be the actual approval primitive underneath it.

---

### Q14. MCP Approval Representation
MCP tools cannot rely on interactive prompts. How should cleanup approval be represented there?

- [ ] A. **Two MCP tools** — `board_clean_preview` and `board_clean_apply`
- [ ] B. **Single tool with mode** — `mode=preview|apply`
- [x] C. **Confirmation token** — preview returns token; apply must include it
- [ ] D. **Other** — Describe: ___

**Answer:** C. MCP should also use a confirmation token rather than a purely interactive approval shape.

- This keeps CLI and MCP aligned on the same safety model instead of creating one cleanup contract for humans and another for agents.
- Whether the transport exposes preview/apply as separate tools or one tool with modes, the apply step should still require the preview token.
- That makes approval explicit, replay-resistant, and easy to trace after the fact.

---

### Q15. One-Hour Stale Threshold Behavior
What should happen when an entry crosses the one-hour threshold?

- [ ] A. **Immediately cleanup-eligible** — still explicit, but no further review stage
- [x] B. **Review-required stale** — surface high-priority warning; cleanup remains blocked until reviewed
- [ ] C. **Two-stage** — stale at one hour, cleanup-eligible only after a second threshold
- [ ] D. **Other** — Describe: ___

**Answer:** B. Crossing the one-hour threshold should mark the entry stale and force explicit review before cleanup.

- The stale entry should remain visible as occupying its claimed files until an operator explicitly chooses to renew or remove it.
- `board show`, `status`, and `next` should surface a strong warning and recommend review, not silently downgrade the entry into ignorable noise.
- A second threshold adds complexity without much extra value in v1, while immediate cleanup eligibility is too aggressive for a coordination primitive that must preserve trust.

---

## Batch 6: Conflict Resolution and Auditability

### Q16. Conflict Resolution Actions in v1
Which actions should be supported when two entries conflict on owned files?

- [x] A. **Renew** an existing entry
- [x] B. **Release specific files** from an entry
- [ ] C. **Transfer ownership** to another agent
- [ ] D. **Force override** with audit record
- [x] E. **Mark abandoned + clean**
- [ ] F. **Other** — Describe: ___

**Answer:** A + B + E. Support renew, release specific files, and mark abandoned + clean in v1.

- Renew is the happy path for stale entries that are actually still active — the agent paused, lost connectivity, or the heartbeat stopped while work was still in progress.
- Release specific files lets an operator narrow an entry's scope without destroying it, which matters when only one file is blocking another agent's check-in.
- Mark abandoned + clean is the decommission path for genuinely dead entries, going through the confirmation-token flow from Batch 5.
- Transfer and force override are deferred: transfer is better modeled as check-out + new check-in, and force override can be approximated by mark-abandoned + clean + new check-in, which keeps an explicit audit gap between destruction and re-acquisition.

---

### Q17. Simplest Safe v1 Policy
Which policy should v1 prefer when there is uncertainty?

- [x] A. **Conservative** — block aggressively, require human review for anything ambiguous
- [ ] B. **Balanced** — allow renew/release/transfer, but not force override by default
- [ ] C. **Operationally aggressive** — allow override to keep work moving fast
- [ ] D. **Other** — Describe: ___

**Answer:** A. Start conservative — block aggressively and require human review for anything ambiguous.

- The cost of a false negative (blocking work unnecessarily) is one operator command to renew or clean.
- The cost of a false positive (silently allowing conflicting ownership) is two agents corrupting each other's work with no indication until after the fact.
- Starting conservative means coordination bugs surface as review prompts rather than data corruption. The policy can be relaxed toward balanced once real operational data shows where false blocks actually occur.

---

### Q18. Audit Record Location
Where should audit traces for renew, clean, transfer, and override operations live?

- [ ] A. **Board state only**
- [ ] B. **Lease history only**
- [ ] C. **Ticket history only**
- [x] D. **Board + ticket history**
- [ ] E. **Board + lease + ticket history**
- [ ] F. **Other** — Describe: ___

**Answer:** D. Write audit records to board state and ticket history.

- Board state captures the operational coordination event: who cleaned/renewed/released, when, with which confirmation token.
- Ticket history captures the lifecycle impact: this ticket's ownership changed because of a board operation.
- Lease history is redundant now that the board is canonical and leases are internal mirrors (per Q7/Q8). Writing to all three would create consistency maintenance burden without adding real traceability value.
- Every destructive or state-changing board operation should append an entry to both the board event log and the affected ticket's history, including operator identity, timestamp, confirmation token if applicable, and a human-readable reason.

---

## Batch 7: Validation Priorities

### Q19. Highest-Risk Race Conditions
Which race conditions must have automated tests before the feature is considered safe?

- [ ] A. **Overlapping-file simultaneous check-in**
- [ ] B. **WIP-limit boundary races**
- [ ] C. **`board_show()` vs `board_heartbeat()`**
- [ ] D. **`board_clean()` vs `board_check_out()` / `board_heartbeat()`**
- [x] E. **All of the above**
- [ ] F. **Other** — Describe: ___

**Answer:** E. All four race conditions must have automated tests before the feature ships.

- Overlapping-file simultaneous check-in (A) is the most dangerous: if both succeed, the entire coordination model is broken. Must prove exactly one wins.
- WIP-limit boundary races (B) matter because the limit is a global constraint checked at write time; two check-ins seeing "1 slot remaining" could both succeed without atomic enforcement.
- `board_show()` vs `board_heartbeat()` (C) must return a consistent snapshot even when heartbeat mutates concurrently.
- `board_clean()` vs `board_heartbeat()`/`board_check_out()` (D) is operationally subtle: if a cleanup token is applied while the agent simultaneously heartbeats, the system must either honor or reject the token consistently.
- Skipping any of these means discovering the failure under exactly the conditions where coordination matters most.

---

### Q20. Restart Recovery Guarantee
After a process restart, what should the system guarantee?

- [ ] A. **Best effort only** — board may need manual reconciliation
- [x] B. **Persisted entries recovered** — board state is restored and stale recomputed
- [ ] C. **Strict reconciliation** — board, leases, and ticket state are compared and drift is surfaced immediately
- [ ] D. **Other** — Describe: ___

**Answer:** B. On restart, recover all persisted board entries and recompute stale status from current time vs last heartbeat.

- Board state lives in redb and already survives restarts. The guarantee should be that all entries are loaded and staleness is recomputed, not that the system also cross-checks lease or ticket state.
- Since leases are internal mirrors (Q7/Q8) and ticket state is advisory (Q7), strict reconciliation against them adds complexity for limited value. Trust the board as source of truth on restart.
- Best-effort (A) is too weak for a coordination primitive — operators should not need to manually reconcile after every restart.

---

### Q21. Cross-Interface Consistency Bar
How strict should CLI/MCP consistency be?

- [ ] A. **Same semantics, different shapes allowed**
- [x] B. **Same semantics and same core fields**
- [ ] C. **Golden parity** — same semantics, same core fields, and shared examples/golden outputs
- [ ] D. **Other** — Describe: ___

**Answer:** B. CLI and MCP must return the same semantics and the same core response fields.

- Core fields (entry IDs, status, file lists, stale flags, confirmation tokens) must match so scripts and agents can switch between interfaces without behavior drift.
- Golden parity (C) sounds ideal but is expensive to maintain — output formatting, ordering, and human-readable text naturally differ between a terminal and a JSON tool response. Locking those down creates maintenance burden without protecting real coordination safety.
- "Same semantics, different shapes" (A) is too loose — it would let fields drift or go missing from one interface without detection until an agent hits a missing field in production.
- A shared integration test harness that exercises the same workflow through both CLI and MCP, then asserts structural field parity, is the recommended testing shape.

---

## Final Summary

When all seven batches are answered, we should be able to:

- finalize `84ceb9ce`
- finalize `c3143e3c`
- fold the decisions back into `8aff39cb`
- start `0db86ac1` without leaving core coordination semantics undefined

### Decision Summary

Q1: C — Hybrid identity: immutable `entry_id` plus uniqueness rules for active `(ticket_id, agent_id)` entries.
Q2: C — Completed entries create a new linked attempt rather than being resumed in place.
Q3: A — One active board entry per ticket in v1.
Q4: A — Strongly recommend resume before suggesting new work.
Q5: A — Show caller-owned active/stale work in a separate section before the global board.
Q6: A — Handoff in v1 is check-out plus new check-in, with optional handoff reason metadata.
Q7: A — The board is the canonical short-term ownership system; leases mirror it and ticket state is advisory.
Q8: D — Remove public raw `claim` / `unclaim` workflows in favor of board commands; any lease primitive is internal/admin-only.
Q9: D — All mutating ticket lifecycle operations must trigger board reconciliation.
Q10: B — Use workspace-relative lexical normalization as the stable conflict key.
Q11: C — Respect workspace/platform case-sensitivity semantics.
Q12: B — Support files plus explicit renamed-file transitions in v1.
Q13: C — Cleanup uses a preview-generated confirmation token.
Q14: C — MCP cleanup approval also uses a confirmation token.
Q15: B — One hour marks entries as review-required stale, not immediately cleanable.
Q16: A+B+E — Support renew, release specific files, and mark abandoned + clean in v1.
Q17: A — Start conservative; block aggressively and require human review for ambiguity.
Q18: D — Audit records go to board state and ticket history.
Q19: E — All four race conditions (overlapping-file check-in, WIP-limit, show/heartbeat, clean/checkout) must have automated tests.
Q20: B — Persisted entries recovered on restart; stale status recomputed from current time.
Q21: B — Same semantics and same core fields across CLI and MCP.