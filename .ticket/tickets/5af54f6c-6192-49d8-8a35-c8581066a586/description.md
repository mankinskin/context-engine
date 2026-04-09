# Impl: Terminal UI for Queue, Assignment Status, and Review Workflow

## Purpose

Provide the primary human interface for the AOH orchestrator in v1. Per ADR-4, the entrypoint is a **Rust daemon with a `ratatui` TUI** — no VS Code extension in v1. The TUI is the operator's console for monitoring session queues, viewing per-assignment progress, triggering review actions (approve/reject/request-changes), and managing budget/lifecycle controls.

The TUI consumes real-time events from the `tokio::broadcast` channels populated by the assignment runner and review coordinator. It does not own business logic — it renders state and dispatches operator commands.

## Component Boundaries

### In scope
- `ratatui`-based terminal UI application
- **Queue view**: tickets in ready/provisioning/running states, sorted by priority
- **Assignment view**: per-session detail with live progress, stdout/stderr tail, budget meters, and state
- **Review view**: local PR list with approve/reject/request-changes actions
- **Log view**: per-assignment scrollable log output
- Operator actions: approve review, request changes, reassign, retry failed session, stop session, extend budget
- Latency indicators: time-in-state, budget consumption bars (token + time)
- Error indicators: failed sessions highlighted with reason summary
- Keyboard-driven navigation with help overlay
- Configurable refresh rate for live data
- Responsive layout: adapts to terminal width (minimum viable at 80 columns)

### Out of scope
- VS Code extension UI (deferred to Phase 2 per ADR-4)
- Web-based dashboard
- Notification delivery (owned by `8db8ef2f` notifier adapters)
- Business logic for review transitions (owned by `d0cc3c8b` review coordinator; TUI dispatches commands)
- Session lifecycle management (owned by `a8632357` assignment runner; TUI renders state)

## Key Data Types

```rust
/// Top-level TUI application state.
struct App {
    mode: ViewMode,
    queue: QueueState,
    assignments: Vec<AssignmentView>,
    reviews: Vec<ReviewView>,
    selected: usize,
    event_rx: broadcast::Receiver<ProgressEvent>,
    command_tx: mpsc::Sender<OperatorCommand>,
}

enum ViewMode {
    Queue,
    AssignmentDetail(SessionId),
    ReviewList,
    ReviewDetail(PrId),
    LogView(SessionId),
    Help,
}

/// Rendered assignment summary for the queue view.
struct AssignmentView {
    session_id: SessionId,
    ticket_id: TicketId,
    ticket_title: String,
    agent_name: String,         // persona display name
    state: SessionState,
    time_in_state: Duration,
    token_usage: (u32, u32),    // (used, limit)
    time_usage: (Duration, Duration),  // (elapsed, limit)
    last_event: String,
}

/// Rendered review item.
struct ReviewView {
    pr_id: PrId,
    ticket_title: String,
    agent_name: String,
    branch: String,
    files_changed: u32,
    evidence_count: u32,
    state: PRState,
}

/// Commands the operator can issue from the TUI.
enum OperatorCommand {
    ApproveReview(PrId),
    RequestChanges(PrId, String),   // feedback text
    RejectReview(PrId, String),     // reason
    RetrySession(SessionId),
    StopSession(SessionId),
    ExtendBudget(SessionId, BudgetExtension),
    Reassign(SessionId),
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-4 (Rust daemon + TUI) | `ratatui` is the primary operator UI in v1; VS Code deferred |
| ADR-6 (Coordination protocol) | TUI receives events via `tokio::broadcast`; sends commands via `mpsc` channel to orchestrator |
| ADR-10 (Budget controls) | TUI displays token/time budget meters and supports `ExtendBudget` operator action |
| ADR-8 (Agent identity) | TUI displays persona names alongside session IDs for readability |
| `db784443` (Trust boundaries) | TUI is a trusted local operator surface; all review actions go through transition guards in the review coordinator |

## Acceptance Criteria

- [ ] TUI displays a live queue of sessions in all non-terminal states, sorted by priority
- [ ] Assignment detail view shows per-session progress, stdout/stderr tail, and budget meters
- [ ] Review view lists local PRs with approve/reject/request-changes actions
- [ ] Per-assignment log view supports scrolling through session output
- [ ] Operator actions (approve, reject, retry, stop, extend budget, reassign) dispatch to the orchestrator
- [ ] Latency indicators show time-in-state and budget consumption (token + time) as visual bars
- [ ] Failed sessions are highlighted with error reason summary
- [ ] Keyboard-driven navigation works with a help overlay listing all keybindings
- [ ] Layout adapts to terminal width (functional at 80 columns)
- [ ] Unit tests cover: view rendering with mock data, operator command dispatch, and event-driven state updates
