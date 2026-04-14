# Impl: Review Coordinator with Validator Handoff and State Guards

## Purpose

Enforce separation-of-duties between agent implementation and human review. The review coordinator manages the handoff from a completed agent session (`Reporting` / `PROpen` state) through human review, validation of acceptance criteria, and final merge or change-request routing.

Per the session lifecycle in `34bc4938`, the path is: `PROpen → approve → Merging → Merged → Archiving → Archived`, or `PROpen → changes requested → RevivalQueue → Running`. The review coordinator owns the transition guards that prevent skipping review steps and records the evidence chain linking agent output to reviewer decisions.

## Component Boundaries

### In scope
- `ReviewCoordinator` service receiving completed session reports from the assignment runner
- Local PR record creation: structured record of branch, diff summary, agent report, and ticket link
- Transition guards: enforce `worker → review → validating` ordering (no direct close from implementation)
- Reviewer action handling: approve, request-changes, or reject
- Change-request routing: extract structured change feedback and route to revival queue
- Evidence chain: link agent's validation results, test output, and implementation notes to the PR record
- Merge execution: local merge (squash/merge/rebase configurable) on approval
- Optional remote push: push branch to GitHub remote only on explicit operator trigger (ADR-3)
- Archive trigger: after merge, signal the archiving subsystem (per `ffa5361a` schema)
- State guards: validate preconditions at each transition; reject invalid state changes with typed errors

### Out of scope
- Agent session execution (owned by `a8632357` assignment runner)
- GitHub API calls for remote PR creation/merge (owned by research/design under `f3c6ed90`)
- Session revival implementation (owned by session lifecycle; coordinator only routes to revival queue)
- Notification delivery of review events (owned by `8db8ef2f`; coordinator emits events, notifier delivers)

## Key Data Types

```rust
/// Local PR record (not a GitHub PR until explicitly pushed).
struct LocalPR {
    pr_id: PrId,
    session_id: SessionId,
    ticket_id: TicketId,
    agent_id: AgentId,
    branch_name: String,
    base_branch: String,
    title: String,
    body: String,               // agent report + metadata template
    evidence: Vec<EvidenceRef>,
    state: PRState,
    created_at: DateTime<Utc>,
    reviewed_at: Option<DateTime<Utc>>,
    reviewer_id: Option<OperatorId>,
}

/// PR lifecycle states.
enum PRState {
    Open,
    Approved,
    ChangesRequested { feedback: Vec<ChangeFeedback> },
    Merging,
    Merged { merge_commit: String },
    Rejected { reason: String },
}

/// Structured change feedback from reviewer.
struct ChangeFeedback {
    file: Option<String>,
    line_range: Option<(u32, u32)>,
    comment: String,
    severity: FeedbackSeverity,
}

/// Evidence linking agent output to review record.
struct EvidenceRef {
    kind: EvidenceKind,         // TestResult, CargoCheck, Screenshot, DiffSnapshot
    path: PathBuf,
    summary: String,
}

/// Transition precondition violation.
enum ReviewError {
    InvalidTransition { from: PRState, to: PRState },
    MissingEvidence(String),
    UnauthorizedReviewer,
    MergeConflict(String),
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-3 (GitHub remote, local-first) | PR records are local by default; remote push only on explicit trigger |
| ADR-6 (Coordination protocol) | PR state persisted via `ticket-api`; review events routed via `tokio::mpsc` (intra-process) |
| ADR-9 (Session revival) | Change requests route to revival queue; coordinator does not own revival execution |
| `db784443` (Trust boundaries) | Reviewer identity verified before accepting approve/merge actions; merge requires authorized operator |
| `ffa5361a` (Archive schema) | After merge, coordinator triggers archiving with the session's evidence chain |

## Acceptance Criteria

- [ ] Local PR record is created from the assignment runner's session report with full metadata
- [ ] Transition guards enforce `worker → review → validating` ordering; direct close from implementation is rejected
- [ ] Approve action transitions PR to `Merging` and executes local merge (squash/merge/rebase configurable)
- [ ] Change-request action extracts structured feedback and routes session to revival queue
- [ ] Evidence chain links agent's test results, cargo check output, and implementation notes to the PR record
- [ ] Remote push to GitHub is gated behind explicit operator action (not automatic)
- [ ] Archive trigger fires after successful merge
- [ ] Reviewer identity is validated before accepting approve/reject/merge actions
- [ ] Invalid state transitions produce typed `ReviewError` values
- [ ] Unit tests cover: approve→merge flow, change-request→revival routing, transition guard rejection, evidence chain integrity, and unauthorized reviewer rejection
