# Goal

Resolve the core architecture decisions for unified logging, operation journaling, and replayable visualization before implementation proceeds.

## Scope

- Decide owning crate boundaries for shared tracing runtime, log-api metadata, and generalized operation journals.
- Decide whether journals live under `.log`, domain stores, or a dedicated `.journal` layout.
- Define schema-versioning strategy for logs, journals, and graph replay events.
- Classify operations as replayable, rollbackable, or manual-recovery only.
- Document dependency-direction constraints so `log-api`, `memory-api`, context-stack, and viewer tooling stay DRY without cycles.

## Placement decision

Keep this ticket in the root `context-engine` workspace as the lowest common ancestor. Its output governs specialized follow-on work in lower crates; do not move it into `memory-api`, `log-api`, or `context-stack`.

## Specialized follow-on tickets to link from here

- memory-api: `756fed27`, `6c859ac3`, `2e41c96d`, `35cd05c1`, `3041d7e3`
- log-api: `d3349747`, `aa94d02e`
- context-stack: `1dffcf23`

## Acceptance criteria

- Open decisions from spec `aa769a27` are answered or narrowed to explicit follow-up tickets.
- A short architecture decision record lists artifact boundaries: trace log vs operation journal vs visualization event.
- The tracker roadmap can proceed without ambiguous ownership for shared facilities.