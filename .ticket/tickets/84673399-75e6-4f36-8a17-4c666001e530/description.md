# Goal

Resolve the core architecture decisions for unified logging, operation journaling, and replayable visualization before implementation proceeds.

## Scope

- Decide owning crate boundaries for shared tracing runtime, log-api metadata, and generalized operation journals.
- Decide whether journals live under `.log`, domain stores, or a dedicated `.journal` layout.
- Define schema-versioning strategy for logs, journals, and graph replay events.
- Classify operations as replayable, rollbackable, or manual-recovery only.
- Document dependency-direction constraints so `log-api`, `memory-api`, context-stack, and viewer tooling stay DRY without cycles.

## Acceptance criteria

- Open decisions from spec `aa769a27` are answered or narrowed to explicit follow-up tickets.
- A short architecture decision record lists artifact boundaries: trace log vs operation journal vs visualization event.
- The tracker roadmap can proceed without ambiguous ownership for shared facilities.