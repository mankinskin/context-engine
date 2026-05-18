# Validation and hardening gates

## Scope

- Integration harness for end-to-end Firecracker-backed sandbox execution.
- Concurrent session validation for 5 to 10 sessions on Linux/KVM hosts.
- Selector and admission matrix coverage for `firecracker`, `browser-container`, and `browser-gpu-container` decisions.
- Chaos cases: API socket failure, boot failure, guest network setup failure, agent crash, hard termination, orphan cleanup, and compatibility-path routing failure.
- Load and stability measurements for worktree provisioning, guest asset staging, and archive cleanup.
- Leak-detection procedure and long-run monitoring guidance.
- Explicit recording of the `jailer` hardening state and any blocker created by `firepilot`'s current executor surface.
- Browser-runner validation for deterministic launch profile, required artifact capture, and GPU admission failure behavior.
- No requirement for a public Web UI or live streaming validation.

## Acceptance criteria

- Focused integration suite passes for the functional v1 Firecracker flow on a Linux/KVM acceptance host.
- Concurrency, cleanup, fault recovery, and selector admission outcomes are exercised with repeatable commands.
- Validation covers both admitted and blocked browser-lane decisions, including explicit rejection on non-GPU hosts when GPU is required.
- Validation results record the current `jailer` integration status and the compatibility-path policy for browser or GPU-bound workloads.
- Validation artifacts confirm the required browser metadata and archive outputs for compatibility-lane runs.
- Remaining non-v1 gaps are explicitly blocked or deferred.