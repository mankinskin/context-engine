# Functional v1 sandbox orchestration layer

## Goal

Deliver the first functional sandbox orchestration slice for agent execution in this repository.

This track supersedes the earlier AOH execution and UX plan set for the initial delivery target. The new v1 slice keeps only the shared core between the older AOH proposal and the newer sandbox/browser roadmap: repo-local worktree provisioning, Firecracker-backed isolation, session execution, artifact capture, and memory-store traceability.

## Final decisions

- Firecracker microVMs via `firepilot` and the Firecracker API are the primary isolation path on Linux/KVM hosts.
- The implementation track replaces Docker-first sandbox provisioning with Firecracker boot, drive, and network provisioning.
- Container fallback is no longer the default runtime. It exists only as a narrow compatibility path for browser or GPU-bound workloads and unsupported hosts.
- git2 repo-local worktrees remain under `.aoh/worktrees/`.
- Tokio orchestration core, semaphore-limited concurrency, and minimal axum endpoints remain in scope.
- ticket, spec, and doc stores are authoritative workflow metadata. test and log stay artifact-linked until native stores exist.
- No ratatui TUI, messenger adapters, PR manager, review coordinator, WebSocket or WASM console, or CDP screencast pipeline in this first functional slice.
- Functional concurrency target remains 5 to 10 sessions on a Linux/KVM acceptance host.

## Child track

1. Firecracker control plane and repo-local microVM foundation.
2. Session execution, per-session MCP, and artifact capture.
3. Memory-stack traceability, archive linking, and runbook/docs.
4. Validation and hardening gates.

## Superseded tickets

This track replaces the earlier AOH implementation and epic tickets plus the older execution-layer plan parent. Historical research and design tickets remain as inputs and are not cancelled.

## Acceptance criteria

- Final v1 orchestration spec exists in the spec store and reflects the Firecracker-first decision set.
- The sequential child track exists and reflects the Firecracker-first scope plus the narrow compatibility fallback rule.
- Superseded active planning and implementation tickets are cancelled.