# Firecracker control plane and repo-local microVM foundation

## Scope

- Tokio multi-thread orchestration core.
- Semaphore-based concurrency guard.
- `Arc<DashMap<SessionId, SandboxState>>` or equivalent shared registry.
- Minimal axum endpoints: health, internal control, and secret delivery.
- git2-based repo-local worktree creation under `.aoh/worktrees/`.
- Firecracker microVM lifecycle via `firepilot` and the Firecracker API.
- Versioned guest kernel and rootfs asset management for the microVM fleet.
- Per-session API sockets plus guest network setup for sandbox access.
- Session capability request model plus host inventory model consumed by the runtime selector.
- Admission decision contract for `admitted` versus `blocked` outcomes, including explicit routing and rejection reasons.
- Archive metadata seed generation for runtime lane, host pool, host id, and routing context before sandbox boot.
- Idempotent cleanup and orphan reconciliation on restart.
- Functional bring-up through `FirecrackerExecutor`; hardened `jailer` integration tracked as a required hardening concern because `firepilot` does not yet implement `JailerExecutor`.
- Explicit compatibility-lane hook for browser or GPU-bound workloads and unsupported hosts via the shared selector contract.
- No Docker-first runtime abstraction in the primary path.

## Acceptance criteria

- The orchestrator provisions isolated worktree, branch, and Firecracker-backed microVM sandboxes.
- The selector and admission layer choose `firecracker` only when declared session capabilities and host inventory allow it, or return a blocked result before provisioning starts.
- Guest kernel, rootfs, and per-session API socket setup are automated and repeatable.
- Secret delivery works through the internal control plane used by the guest session.
- Runtime lane, host assignment, and routing reason are written into the initial session metadata seed before the runner starts.
- Cleanup is idempotent after success, failure, and restart reconciliation.
- Unsupported capability or host combinations are rejected explicitly rather than downgraded implicitly.
- Linux/KVM host setup, Firecracker binary requirements, and guest asset layout are documented well enough to run locally.