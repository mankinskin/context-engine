# Deferred: Executor and Sandbox Integration

**Status:** PARKED — revisit after dogfooding (Phase 4) proves the core workflow.

## What Was Deferred

The following deliverables were part of the original plan but are not required for
the core ticket tracker to be useful:

1. **Pluggable executor model** — abstraction for local-process vs COW-sandbox backends.
2. **Zeroboot COW sandbox integration** — fast VM sandbox spawning for isolated agent execution.
3. **Executor assignment metadata** on lease — `executor_backend`, `sandbox_id` fields.
4. **Capability-based scheduler** — matching ticket requirements to host capabilities.
5. **Sandbox lifecycle management** — spawn/teardown tied to lease lifecycle.

## Why Deferred

- The ticket tracker's primary value is coordination (CRUD, history, search, graph),
  not execution infrastructure.
- Executor integration adds substantial complexity (C dependencies, platform-specific
  code, Linux/KVM requirements) without improving the core tracking workflow.
- Dogfooding can proceed with manual or simple process-based execution; the tracker
  does not need to own the execution surface.
- Zeroboot is Linux/KVM only; the tracker must work on all platforms from day one.

## Reactivation Criteria

Revisit this work when ALL of the following are true:

- [ ] Core tracker (Phases 0–3) is stable and dogfooded for ≥2 weeks.
- [ ] Parallel swarm execution is a proven bottleneck that cannot be solved by
      lease protocol alone.
- [ ] A clear, scoped proposal exists for executor abstraction that does not
      compromise cross-platform compatibility.
- [ ] Zeroboot (or equivalent) has a stable Rust API and is available as a crate.

## Related Documents

- [COW Sandboxed Swarm Execution Use Case](05_use_cases/20260320_USE_CASE_COW_SANDBOXED_SWARM_EXECUTION.md)
- [Phase 1.5 Lease Protocol](015_phase_lease_protocol/PLAN.md) — leases work without executor integration
