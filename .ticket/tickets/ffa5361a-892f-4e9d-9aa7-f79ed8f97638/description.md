# [AOH][Design] Session Archive, Artifact Retention, and Revival Schema

## Objective

Turn ADR-9 into an implementation-ready contract.

The current architecture says sessions are revived by injecting `session-archive.toml` plus archived context, but there is no dedicated definition of what the archive contains, which artifacts are retained, where they live, how long they persist, or what exactly a revival is allowed to reuse.

## Why This Needs Its Own Ticket

This affects multiple subsystems simultaneously:
- `agent-session` needs a stable archive schema
- `sandbox-manager` needs to know what workspace/container state is reusable
- `pr-manager` and review flow need to preserve evidence and change requests
- notifier/TUI need stable links to archived artifacts
- cost/accounting needs historical run data

Without a dedicated contract, each subsystem will make different assumptions.

## Questions to Resolve

### Archive content
1. What fields must `session-archive.toml` contain?
   - session ID
   - ticket ID
   - agent/persona ID
   - branch/worktree path
   - summary
   - modified files
   - validation results
   - open questions
   - cost metrics
   - notifier correlation IDs
   - review/change-request history
2. Which artifacts are stored separately from the TOML?
   - stdout/stderr logs
   - test logs
   - screenshots
   - patch/diff snapshot
   - structured result JSON

### Retention and lifecycle
3. How long are archives kept locally?
4. What is pruned automatically vs only by explicit user action?
5. What is the cleanup behavior after merge, cancellation, or hard termination?

### Revival semantics
6. What constitutes a revival?
   - same branch/worktree reused
   - same container image reused
   - same named volume reused
   - same persona reused
7. Which state is authoritative on revival: archive summary, current branch diff, local PR record, or ticket fields?
8. Can multiple revivals exist for the same ticket, and how are they versioned?

### Evidence and auditability
9. How do archives reference evidence needed for review or bug triage?
10. How are artifact paths represented so TUI/notifier can deep-link safely?
11. What must remain readable after worktree cleanup?

## Alternatives to Consider

### Storage layout
- **Option A**: `.aoh/archive/{ticket-id}/{session-id}/...` inside repo metadata branch/worktree
- **Option B**: external orchestrator data dir with ticket store references
- **Option C**: hybrid: metadata in repo, large artifacts outside repo

### Revival source of truth
- **Option A**: archive TOML is canonical; branch/worktree is a mutable implementation detail
- **Option B**: branch diff is canonical; archive is summary-only
- **Option C**: local PR record is canonical once reporting completes

## Deliverables

- Archive schema for `session-archive.toml`
- Artifact directory layout and retention policy
- Revival state machine and versioning rules
- Clear contract for what survives cleanup and what is disposable
- Recommendation for archive storage location and source-of-truth hierarchy

## Acceptance Criteria

- [ ] `session-archive.toml` schema is documented with required and optional fields
- [ ] Artifact directory layout is documented with naming conventions
- [ ] Retention policy defined for merged, rejected, failed, and cancelled sessions
- [ ] Revival semantics defined for worktree, branch, container, volume, and persona reuse
- [ ] Versioning rules defined for multiple revivals of the same ticket
- [ ] Evidence/artifact references are specified so TUI and notifier can link safely
- [ ] Architecture ticket updated to reference the approved archive/revival contract