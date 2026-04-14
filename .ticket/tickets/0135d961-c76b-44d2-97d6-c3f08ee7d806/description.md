# Impl: End-to-End Executor Integration and Fault-Injection Suite

## Purpose

Validate that the full AOH execution stack works correctly when all components are composed: Copilot client → sandbox manager → assignment runner → review coordinator → notifier → TUI event flow. This ticket owns the integration test suite and fault-injection harness that exercises the real lifecycle, including failure modes that unit tests on individual components cannot cover.

This is the final validation gate (Wave 3) before the AOH system is considered implementation-complete.

## Component Boundaries

### In scope
- Integration test harness that composes all AOH components in a single test process
- End-to-end lifecycle tests: ticket ready → provision → run → report → review → merge → archive
- Concurrent session tests: multiple isolated assignments running in parallel without interference
- Review handoff tests: worker → review → validation → close (with transition guard enforcement)
- Notifier delivery tests: verify correct notifications are emitted at each lifecycle stage
- Cleanup/recovery tests: verify idempotent cleanup after normal completion, failure, and hard termination
- Merge-chain metadata tests: verify evidence chain, PR records, and archive artifacts are consistent
- Fault-injection scenarios:
  - Container start failure → session transitions to `ProvisionFailed`
  - Agent process crash mid-session → runner detects, updates ticket, emits failure event
  - Soft budget exceeded → self-assessment window triggered, then hard limit terminates
  - Time limit exceeded → session terminated with timeout reason
  - Network failure during agent execution → transient error handling and retry
  - Concurrent sessions compete for the same ticket → conflict detection fires
  - Reviewer rejects → change-request routing to revival queue works correctly
  - Orphaned container on orchestrator restart → reconciliation cleans up
- Mock/stub infrastructure:
  - Mock `CopilotClient` returning canned responses (no real API calls)
  - Test container runtime (or real Docker with test images if CI supports it)
  - In-memory `ticket-api` store
  - Captured notifier that records all sent notifications for assertion

### Out of scope
- Performance benchmarking (separate concern)
- Load testing with many concurrent sessions (separate concern)
- Real Copilot API integration testing (requires live credentials)
- UI testing of the `ratatui` TUI (TUI has its own rendering tests)

## Key Data Types

```rust
/// Test harness that wires up all components with test doubles.
struct TestHarness {
    copilot: MockCopilotClient,
    sandbox_mgr: SandboxManager<TestContainerRuntime>,
    runner: AssignmentRunner,
    reviewer: ReviewCoordinator,
    notifier: CapturedNotifier,
    ticket_store: InMemoryTicketStore,
    event_rx: broadcast::Receiver<ProgressEvent>,
}

/// A notifier that captures all sent notifications for test assertions.
struct CapturedNotifier {
    sent: Arc<Mutex<Vec<Notification>>>,
}

/// Fault injection control for test scenarios.
enum FaultInjection {
    ContainerStartFailure,
    AgentProcessCrash { after: Duration },
    NetworkPartition { duration: Duration },
    BudgetExceeded { tokens: u32 },
    TimeoutExceeded { duration: Duration },
    MergeConflict,
}

/// Test container runtime that can be configured to inject faults.
struct TestContainerRuntime {
    fault: Option<FaultInjection>,
    containers: Arc<Mutex<HashMap<ContainerId, ContainerState>>>,
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-6 (Coordination protocol) | Integration tests validate that `ticket-api` state and `tokio::mpsc` event routing stay consistent across the full lifecycle |
| ADR-10 (Budget controls) | Fault injection includes budget soft/hard limit scenarios with correct escalation behavior |
| ADR-1 (Container BaaS) | Tests use `TestContainerRuntime` by default; optional real Docker tests gated behind a CI feature flag |
| ADR-9 (Session revival) | Change-request → revival queue routing is tested end-to-end |
| ADR-3 (GitHub remote, local-first) | Merge tests validate local merge without remote push; remote push is a separate explicit action |

## Test Scenarios

### Happy path
1. Single ticket ready → provision → agent runs → reports → reviewer approves → merge → archive
2. Two concurrent tickets → both provision and run in parallel → both complete independently

### Failure and recovery
3. Container start failure → `ProvisionFailed` state → ticket updated → failure notification sent
4. Agent crash mid-session → runner detects → `Failed` state → cleanup triggered
5. Soft budget → self-assessment → hard budget → `HardTerminate` → cleanup
6. Time hard limit → timeout → `HardTerminate` → cleanup
7. Orchestrator restart with orphan containers → reconciliation cleans up

### Review flow
8. Reviewer approves → merge → archive trigger → evidence chain intact
9. Reviewer requests changes → revival queue → session revived with change context
10. Reviewer rejects → session terminated → ticket updated with rejection reason

### Conflict detection
11. Two sessions target overlapping files → conflict detector fires → one session paused with notification

## Acceptance Criteria

- [ ] Integration harness composes all AOH components with test doubles in a single test process
- [ ] Happy-path E2E test passes: ready → provision → run → report → review → merge → archive
- [ ] Concurrent session test passes: two parallel assignments complete without interference
- [ ] Review handoff test validates the full `worker → review → validation → close` state progression
- [ ] Notifier capture test verifies correct notifications at each lifecycle stage
- [ ] Cleanup test verifies idempotent cleanup after normal completion and after failure
- [ ] Merge-chain metadata test verifies evidence refs and PR records are consistent post-merge
- [ ] Fault-injection tests pass for: container failure, agent crash, budget escalation, timeout, network fault, conflict detection, and rejection routing
- [ ] All tests run without real Copilot API credentials or real container runtime (mock/stub infrastructure)
- [ ] Test execution time is under 60 seconds for the full suite (no real network calls)
