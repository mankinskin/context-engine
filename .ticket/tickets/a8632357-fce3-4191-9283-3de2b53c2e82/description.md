# Impl: Assignment Runner and Progress Watcher for Concurrent Sub-Agents

## Purpose

Drive agent sessions from kickoff through completion, streaming progress events and managing the session lifecycle state machine. The assignment runner sits between the sandbox manager (which provisions the environment) and the review coordinator (which handles post-implementation handoff). It starts the agent process inside a provisioned container, monitors progress, enforces budget limits, and handles timeout/failure recovery.

Per ADR-6, coordination uses `ticket-api` for durable state. Intra-process event routing uses `tokio::mpsc` channels (runner → TUI, runner → notifier) as internal plumbing — not a coordination protocol.

## Component Boundaries

### In scope
- `AssignmentRunner` service that manages one or more concurrent agent sessions
- Session lifecycle state machine: `Provisioning → KickingOff → Running → Reporting → PROpen`
- Start agent process inside a sandbox container (via sandbox manager handle)
- Stream progress events from the agent session (stdout/stderr, structured progress, tool invocations)
- Publish progress events via `tokio::mpsc` channels to TUI and notifier (intra-process routing)
- Update ticket state transitions in `ticket-api` as sessions progress
- Budget enforcement integration: receive soft/hard budget signals from cost watchdog
  - Soft limit → trigger agent self-assessment window (ADR-10: 2,000-token window)
  - Hard limit → force terminate session
  - Time limits: soft 30 min, hard 90 min (configurable in `orchestrator.toml`)
- Timeout handling: detect hung sessions and trigger recovery
- Early-stop: operator-initiated session termination via TUI/notifier action
- Error recovery: capture failure state, update ticket, and emit failure event
- Concurrent session tracking: run up to N sessions in parallel (configurable)

### Out of scope
- Sandbox provisioning/cleanup (owned by `51471c3e` sandbox manager)
- Review/validation handoff logic (owned by `d0cc3c8b` review coordinator)
- Notification delivery (owned by `8db8ef2f` notifier adapters)
- Per-session MCP server lifecycle (separate concern, though runner triggers startup)
- Cost watchdog implementation (runner receives budget signals, does not compute them)

## Key Data Types

```rust
/// Represents a running agent session.
struct Assignment {
    session_id: SessionId,
    ticket_id: TicketId,
    agent_id: AgentId,
    sandbox: Sandbox,           // from sandbox manager
    state: SessionState,
    started_at: Instant,
    budget: BudgetLimits,
}

/// Session lifecycle states (from design ticket 34bc4938).
enum SessionState {
    Provisioning,
    KickingOff,
    Running,
    BudgetWarning,
    SelfAssessment,
    Reporting,
    PROpen,
    HardTerminate,
    Failed { reason: String },
}

/// Budget limits per session (from ADR-10).
struct BudgetLimits {
    soft_token_limit: u32,          // default: 80,000
    hard_token_limit: u32,          // default: 200,000
    self_assessment_window: u32,    // default: 2,000 tokens
    time_soft_limit: Duration,      // default: 30 min
    time_hard_limit: Duration,      // default: 90 min
    user_notify_wait: Duration,     // default: 5 min
}

/// Progress event emitted via mpsc channels (one per subscriber).
enum ProgressEvent {
    StateChanged { session_id: SessionId, from: SessionState, to: SessionState },
    Output { session_id: SessionId, stream: OutputStream, data: String },
    ToolInvocation { session_id: SessionId, tool: String, status: ToolStatus },
    BudgetAlert { session_id: SessionId, kind: BudgetAlertKind },
    Completed { session_id: SessionId, result: SessionResult },
    Failed { session_id: SessionId, error: String },
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-6 (Coordination protocol) | `ticket-api` for durable state; `tokio::mpsc` for intra-process event routing (not a coordination protocol) |
| ADR-7 (Per-session MCP) | Runner triggers MCP server start inside the container; each session gets isolated MCP tools |
| ADR-10 (Budget controls) | Runner enforces soft/hard token and time limits with configurable thresholds from `orchestrator.toml` |
| ADR-4 (Rust daemon + TUI) | Progress events flow to the TUI via mpsc channels |
| ADR-9 (Session revival) | On failure or change-request, session state is captured for potential revival |

## Acceptance Criteria

- [ ] Runner starts agent process inside a provisioned sandbox and transitions through the session state machine
- [ ] Progress events are delivered via `tokio::mpsc` channels to TUI and notifier subscribers
- [ ] Ticket state transitions are updated in `ticket-api` at each lifecycle stage
- [ ] Soft budget signal triggers a self-assessment window; hard budget signal force-terminates the session
- [ ] Time limits (soft/hard) are enforced with configurable thresholds
- [ ] Operator-initiated early-stop terminates the session cleanly and updates ticket state
- [ ] Hung session detection triggers recovery after configurable timeout
- [ ] Concurrent sessions are tracked and limited to a configurable maximum
- [ ] Failure state is captured with reason and emitted as a typed event
- [ ] Unit tests cover: normal lifecycle, budget soft→hard escalation, timeout recovery, early-stop, and concurrent session tracking
