<!-- aligned-structure:v1 -->

# Summary

Define the first functional sandbox orchestration slice for this repository's agentic workflow system while keeping memory-stack traceability intact. The first delivery target must produce isolated agent execution with captured evidence instead of trying to ship every possible operator UI, transport, and review workflow at the same time.

## Behavior Story

Define the first functional sandbox orchestration slice for this repository's agentic workflow system while keeping memory-stack traceability intact. The first delivery target must produce isolated agent execution with captured evidence instead of trying to ship every possible operator UI, transport, and review workflow at the same time.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Functional v1 sandbox orchestration layer

## Problem statement

Define the first functional sandbox orchestration slice for this repository's agentic workflow system while keeping memory-stack traceability intact. The first delivery target must produce isolated agent execution with captured evidence instead of trying to ship every possible operator UI, transport, and review workflow at the same time.

## Proposal synthesis

### Input A: existing AOH architecture and implementation planning

- `.ticket/tickets\34bc4938-fe4a-4ab1-94da-9d8d3697b268`
- `.ticket/tickets\02412b9a-bccd-46f7-bded-0fbd7a067478`
- `.ticket/tickets\ffa5361a-892f-4e9d-9aa7-f79ed8f97638`
- `.ticket/tickets\65d8e6c7-78ea-48ce-a6bd-8bc1eb712c4f`

### Input B: recent sandbox and browser roadmap reviewed in planning

The recent roadmap pushed harder on browser-facing orchestration, runtime streaming, and microVM alternatives.

### Input C: current Firecracker and firepilot research

- Firecracker offers a stable API-driven microVM model with low overhead on Linux/KVM hosts.
- `firepilot` exposes both a high-level `Machine` abstraction and a lower-level `Executor` abstraction for controlling Firecracker lifecycle and configuration from Rust.
- `firepilot` currently exposes `FirecrackerExecutor`, but its `JailerExecutor` is not implemented.
- The existing browser dead-end finding in `.ticket/tickets\65d8e6c7-78ea-48ce-a6bd-8bc1eb712c4f` remains valid: Firecracker still lacks virtio-gpu and is not a good primary runtime for browser-heavy or GPU-bound sessions.

### Common ground

- Isolated per-session execution environments.
- Repo-local git state management.
- Durable artifact capture.
- Explicit lifecycle handling for start, failure, and cleanup.
- Memory-stack-aware planning and evidence linking.

### Divergences

- The older AOH plan pulls ratatui, notifier adapters, local PR management, and review coordination into the initial implementation slice.
- The newer roadmap pushes harder on microVM adoption, but the browser-heavy use case still collides with Firecracker's device model.

### Decision

Revise the earlier blanket Firecracker deferral. The functional v1 is now Firecracker-first for Linux/KVM workloads that fit the microVM model, while preserving a narrow compatibility fallback for browser or GPU-bound sessions and unsupported hosts. This changes the implementation track away from Docker-first provisioning without pretending that the browser constraint disappeared.

## Final v1 decisions

1. Isolation uses Firecracker microVMs via `firepilot` and the Firecracker API on Linux/KVM hosts when the workload fits Firecracker's minimal device model. The high-level `Machine` abstraction is the default lifecycle API; the lower-level `Executor` path is reserved for orchestration code that needs finer control.
2. The current implementation track replaces Docker-first provisioning with Firecracker boot source, drive, and network provisioning. The orchestrator owns versioned guest kernel and rootfs assets plus per-session API socket and network configuration.
3. `firepilot` currently supports direct Firecracker process execution through `FirecrackerExecutor`; `JailerExecutor` is not implemented. Functional bring-up may use direct Firecracker execution, but hardened isolation requires a local `jailer` wrapper or equivalent integration and must be tracked explicitly in validation.
4. Browser or GPU-bound tasks are not routed to Firecracker by default. The prior research ticket `.ticket/tickets\65d8e6c7-78ea-48ce-a6bd-8bc1eb712c4f` remains valid for that workload class. A narrow compatibility runtime fallback is allowed only for browser-heavy sessions or hosts that cannot satisfy Firecracker prerequisites, and that fallback must be modeled as an explicit capability-selected runtime lane instead of an implicit best-effort downgrade.
5. Repository isolation uses `git2` worktrees under `.aoh/worktrees/`. A mandatory shared bare mirror cache is still deferred; it can be added later as an optimization.
6. The orchestrator core uses a Tokio multi-thread runtime, bounded concurrency, and simple in-process routing. That routing now includes a runtime selector that chooses the correct isolation lane from declared session capabilities and host inventory. Durable workflow state belongs in the memory-stack stores, not in ad hoc sidecar metadata.
7. The minimal operator surface for v1 is existing CLI and viewer tooling plus minimal internal HTTP endpoints for health, control, and secret delivery. There is no ratatui TUI, messenger adapter layer, public WebSocket API, gRPC API, or WASM control console in v1.
8. Browser-required work stays outside the Firecracker primary path. The orchestrator may launch and supervise ephemeral browser-capable compatibility runtimes, but it does not own a chromiumoxide CDP layer, page streaming, or screencast transport in v1.
9. Memory-stack authority is split by native ownership:
   - ticket and spec stores own planning and execution metadata
   - doc surfaces own runbook and validation documentation
   - rule surfaces remain guidance inputs
   - test and log evidence are captured as archived artifacts and linked from tickets and specs until native store integrations are added
10. Archive and evidence storage remains under `.aoh/archive/...`, following the completed archive contract from `.ticket/tickets\ffa5361a-892f-4e9d-9aa7-f79ed8f97638`.
11. Crash handling for v1 is cleanup plus reconcile plus requeue. Full mid-flight live resume is still deferred.
12. Review coordination, local PR management, remote push, and merge automation are out of scope for the first functional sandbox orchestration slice.
13. Functional concurrency target is 5 to 10 sessions on a Linux/KVM acceptance host for the Firecracker lane, with separate and smaller browser-capable host pools for compatibility runtimes. Higher concurrency belongs to hardening and benchmark work, not base v1 acceptance.
14. Primary host support target is Linux with KVM, TAP networking, the Firecracker binary, and provisioned guest kernel/rootfs assets for the microVM lane. Browser compatibility hosts may be Linux container hosts with optional GPU support. Windows and WSL2 are not primary Firecracker acceptance environments; they can only participate through the compatibility runtime path if needed.

## Hybrid runtime model

The compatibility runtime is a first-class part of the v1 design, not an undocumented escape hatch. The orchestrator must make routing decisions from explicit session capabilities and host capabilities so the resulting execution path is deterministic, auditable, and testable.

### Session capability contract

Every orchestrated session must declare the minimum capability set needed before provisioning starts. The initial contract for v1 is:

- `needs_browser`: requires a real browser process for validation, scraping, or UI automation
- `needs_gpu`: requires hardware acceleration for acceptable correctness or performance
- `needs_interactive_display`: sensitive to compositor, viewport, or display-backed rendering behavior
- `needs_kvm`: requires kernel-level isolation and a Linux/KVM host
- `requires_network`: needs outbound network access beyond the default local control path
- `host_os`: constrains the acceptable host class when the workload is runtime-specific
- `is_untrusted_code`: requests the strongest available isolation lane compatible with the remaining capabilities

The orchestrator may infer safe defaults, but it must not silently drop a declared requirement. If a session requests GPU or browser capabilities that the selected host pool cannot satisfy, the session must be rejected or explicitly marked blocked rather than downgraded to software rendering without record.

### Runtime lanes

The v1 selector chooses exactly one runtime lane per session:

1. `firecracker`: default lane for non-browser workloads on Linux/KVM hosts when the workload fits Firecracker's device model.
2. `browser-container`: ephemeral browser-capable container lane for browser automation and layout checks that do not require GPU acceleration.
3. `browser-gpu-container`: ephemeral browser-capable container lane on GPU-enabled hosts for WebGL, WebGPU, accelerated canvas, compositor-sensitive rendering, or other performance-sensitive browser sessions.

The selector must emit the chosen lane plus the routing reason into archived session metadata so tickets and specs can trace why a given run used a microVM, a CPU browser container, or a GPU browser container.

### Common session envelope

All runtime lanes share one orchestration contract:

- repo-local `git2` worktree provisioning under `.aoh/worktrees/`
- per-session environment and secret delivery
- bounded runtime, cancellation, and cleanup handling
- stdout/stderr capture plus structured artifact archiving under `.aoh/archive/...`
- session metadata linking back to the active ticket, spec, and validation evidence

Lane-specific provisioning must not fork the metadata model. Firecracker sessions and browser compatibility sessions should differ in runtime implementation, not in how the workflow system records ownership, evidence, or cleanup.

### Browser compatibility lane rules

Browser sessions in v1 must run in ephemeral compatibility runtimes instead of inside Firecracker guests. The browser lanes are responsible for:

- launching a pinned Chromium-family browser build with explicit window size, device scale factor, locale, timezone, and font set
- attaching Playwright or equivalent browser automation from the session runner layer
- isolating each session with per-session users, namespaces, cgroup limits, and minimal required device access
- recording the browser version, viewport, host class, and GPU status in archived metadata

The `browser-gpu-container` lane must treat hardware acceleration as a hard requirement. If the host does not expose the required GPU capability, the session must fail capability admission instead of falling back to SwiftShader or LLVMpipe invisibly.

### Host pools and admission

V1 host inventory must distinguish at least these pools:

- Linux/KVM hosts prepared for Firecracker microVMs
- browser-capable container hosts without GPU requirements
- GPU-enabled browser container hosts with a validated driver and container runtime stack

Admission control must happen before worktree mutation and sandbox boot. A session that requests `needs_gpu=true` or `needs_browser=true` must be routed only to hosts that advertise the corresponding capability. Unsupported combinations should produce a blocked run result with a recorded reason instead of an implicit retry in the wrong lane.

### Selector and admission contract

The runtime selector and admission layer must expose a stable internal contract that can be implemented without guessing across tickets. The minimum v1 contract is:

```text
SelectorInput {
   session_id,
   ticket_ref,
   spec_ref,
   capability_request,
   timeout_budget,
   network_policy,
   candidate_host_pools,
}

AdmissionDecision {
   status: admitted | blocked,
   runtime_lane,
   host_pool,
   host_id?,
   routing_reason,
   rejection_reason?,
   metadata_seed,
}
```

`status=admitted` means the orchestrator found a compatible lane and host. `status=blocked` means the declared requirements could not be satisfied and the session must stop before worktree provisioning or sandbox boot. The selector must never return an admitted decision that omits `runtime_lane`, `host_pool`, or `routing_reason`.

### Archived session metadata shape

Every session archive must carry enough structured metadata to explain the runtime choice and reproduce validation context. The required v1 fields are:

- `session_id`
- `ticket_ref` and `spec_ref`
- `runtime_lane`
- `routing_reason`
- `capability_request`
- `host_pool` and `host_id` when admitted
- `admission_status` and `rejection_reason` when blocked
- `browser_profile` for browser lanes: browser family and version, viewport, device scale factor, locale, timezone, font bundle id, launch flags
- `gpu_status` for browser lanes: required or not, available or not, adapter summary when present, software-renderer fallback forbidden flag
- `artifact_manifest` with archive-relative refs to stdout, stderr, structured results, screenshots, traces, logs, and any run summary

The metadata shape may grow, but these fields form the minimum review and debugging contract for v1.

### Browser runner contract

The browser compatibility lanes need an explicit runner contract so that lane-specific behavior stays deterministic.

The browser runner is responsible for:

- provisioning an ephemeral browser-capable container after admission succeeds
- applying a deterministic browser launch profile with pinned Chromium-family version, viewport, device scale factor, locale, timezone, and fonts
- attaching Playwright from the session runner layer to drive the browser workload
- collecting browser artifacts into `.aoh/archive/...`, including at minimum screenshots, Playwright trace output when enabled, browser console output, and structured browser session metadata
- enforcing the GPU hard requirement on `browser-gpu-container` instead of allowing silent software-renderer fallback
- tearing down the browser runtime and releasing host resources with the same cleanup guarantees expected from the Firecracker lane

The browser runner does not own a general live-streaming transport, remote desktop stack, or long-lived shared browser farm in v1.

## Non-goals and deferrals

- Browser-first execution inside Firecracker microVMs
- GPU-dependent or interactive browser rendering inside Firecracker microVMs
- Silent software-renderer downgrade for sessions that declared GPU requirements
- Direct containerd integration
- Live browser streaming or page screencast transport
- ratatui TUI
- notifier and messenger integration
- local PR and review coordinator flows
- remote Git push or merge automation
- mandatory shared bare mirror cache

## Implementation track

### New v1 track

- `.ticket/tickets\456d9b69-ec43-4746-b47d-20704da01be9`
- `.ticket/tickets\8d83f9f6-b36e-42bd-ac42-3a6d073873a7`
- `.ticket/tickets\0884ab64-e54d-4f9c-abbf-de61990773eb`
- `.ticket/tickets\2a3ad242-8c01-4779-94ec-9e4d5595f538`
- `.ticket/tickets\5ed70069-b080-4a95-8dc5-ddf495007bdd`

### Superseded active planning and implementation tickets

- `.ticket/tickets\4e28bf38-bd3c-466c-9eee-cd618d5f45fe`
- `.ticket/tickets\d5ced7e2-fc67-4a37-a621-96a54a71e51f`
- `.ticket/tickets\51471c3e-a088-47d4-9922-ba49d914af17`
- `.ticket/tickets\6e6b8cf6-3dd8-4b82-939e-a879248271ce`
- `.ticket/tickets\8c185de3-88f9-4565-915e-220d5656d9ac`
- `.ticket/tickets\a8632357-fce3-4191-9283-3de2b53c2e82`
- `.ticket/tickets\a92569e5-3582-4191-9513-80ce6938cda4`
- `.ticket/tickets\d0cc3c8b-efc8-44c4-bbca-5daf4ddcdb8b`
- `.ticket/tickets\8db8ef2f-e33c-4234-a39a-64a481b27984`
- `.ticket/tickets\5af54f6c-6192-49d8-8a35-c8581066a586`
- `.ticket/tickets\0135d961-c76b-44d2-97d6-c3f08ee7d806`

Historical research and architecture tickets remain valid inputs and are not cancelled.

## Validation expectations

- Focused integration coverage for Firecracker provisioning, session execution, archive capture, and cleanup.
- Focused integration coverage for runtime selection, host admission, and compatibility-lane provisioning based on declared session capabilities.
- Focused validation of the selector input or decision contract, including explicit blocked outcomes for unsupported capability and host combinations.
- Fault injection for runtime failure, API socket failure, boot failure, agent crash, orphan cleanup, and restart reconciliation.
- Documentation and runbook coverage for host prerequisites, guest asset management, TAP networking, browser container prerequisites, GPU host prerequisites, and cleanup commands.
- Validation must explicitly record the current `jailer` integration status.
- Browser-facing validation must record the browser lane used, viewport or display resolution, browser version, and whether GPU acceleration was required.
- Browser-facing changes must validate with Playwright coverage plus external fullscreen Chromium-family verification on the affected viewer surface.
- Browser-runner validation must confirm archive completeness for the required browser artifacts and metadata fields.
- Spec and ticket metadata must record passing commands or blocked results for the v1 track, including cases where admission correctly rejects an unsupported host or lane.

## Acceptance additions for the hybrid model

- The spec defines the session capability contract and the exact runtime lanes used by v1 orchestration.
- The spec defines the selector input, admission decision, and archive metadata contracts required to make routing auditable.
- The selector records both chosen runtime lane and routing reason in session artifacts.
- Browser sessions are specified as ephemeral compatibility runtimes instead of Firecracker guests.
- The browser runner contract defines deterministic Chromium launch behavior, Playwright attachment, and minimum artifact capture.
- GPU-requiring browser sessions fail admission on non-GPU hosts instead of silently falling back to software rendering.
- Validation artifacts capture browser version, viewport, host class, and GPU status for browser-facing runs.

## Related workflow specs

- `38e337c2-cdda-4488-9aa7-b47a300563b0`
- `a4f48d84-50ed-4769-a42f-38321ea9600c`
- `cf5e2942-1a47-43cc-a0ee-14e5774680a6`
