# Memory-stack traceability, archive linking, and runbook docs

## Scope

- Record workflow metadata in ticket, spec, and doc owned surfaces.
- Link archive artifacts from tickets and specs as relative evidence refs.
- Align with the workflow traceability and validation specs already in the spec store.
- Write the host provisioning and operations runbook for the first functional version.
- Document the session capability contract, selector and admission decision fields, and archived metadata fields required for review.
- Document Firecracker prerequisites: Linux/KVM host support, Firecracker binary, guest kernel/rootfs assets, TAP or equivalent guest networking setup, and compatibility runtime routing.
- Document browser compatibility runtime prerequisites: container host preparation, deterministic browser profile expectations, and GPU host prerequisites for the GPU lane.
- Document deferred items: native test-api and log-api migration, review and merge automation, richer operator UIs.
- No new dedicated wrapper store for orchestration metadata.

## Acceptance criteria

- New spec and ticket records point to the archive and evidence paths needed for review and debugging.
- The runbook covers host setup, microVM prerequisites, guest asset layout, cache and worktree layout, compatibility routing rules, browser runtime prerequisites, and cleanup commands.
- The documentation explains the selector and admission contract plus the minimum archived metadata and browser evidence fields expected from each runtime lane.
- Deferred items are documented as follow-on scope, not silent gaps.