# Session Hook Routing Dossier

The session capture hook and `mcp-toolmon` use eager provisioning and positional worktree discovery, while the requested model uses a main-checkout-first session record plus a local registry. The `ba4aaa9c` main-checkout mutation is concrete evidence but remains unreproduced as a routing regression; the dossier sequences reproduction before diagnosis-dependent changes.

## Binding Decisions

| ID | Decision |
| --- | --- |
| D1 | The main checkout is the authoritative session-to-worktree registry; the registry is local-only, while only the working branch is committed in the main session record. |
| D2 | Provisioning is deferred: capture begins in the main checkout, explicit instantiation creates the worktree, and registration follows instantiation. |
| D3 | Related code-quality findings block and are fixed; unrelated findings become follow-up tickets under [.agents/instructions/orchestration/code-quality.instructions.md](.agents/instructions/orchestration/code-quality.instructions.md). |
| D4 | A post-commit creation failure blocks the session; recovery is a new forward unset commit, never a revert. |
| D5 | Validation is unit tests only; no full E2E suite is planned. |

## Documents

| Document | Purpose |
| --- | --- |
| [input.md](input.md) | Raw source transcript. |
| [input.clean.md](input.clean.md) | Cleaned English request. |
| [REVIEW.md](REVIEW.md) | Stage 2 verdict and findings F1-F8. |
| [01-research-inventory.md](01-research-inventory.md) | Verified ownership, current behavior, and incident evidence. |
| [02-routing-operating-model.md](02-routing-operating-model.md) | Binding lifecycle, transfer, routing, deregistration, and compatibility model. |
| [03-work-packages.md](03-work-packages.md) | Ticketable sequencing, unit-test completion criteria, relationships, and risks. |

**Status:** Stage 2 verdict was "Changes requested"; blocking questions are answered; no tickets have been created yet.

**Separately routed user item:** whether [ba4aaa9c [workflow-tools][per-tool] Extract ticket tool as a single `ticket` domain crate (api + transport bins) + viewer/vscode frontends](.ticket/tickets/ba4aaa9c-d270-4cfc-a1e2-395634608371/ticket.toml) merges before ticket-viewer verification. The user specified unit tests only for now, not the full E2E suite.
