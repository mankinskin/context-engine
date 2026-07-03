# Goal

Define a replayable graph-operation format for context-stack operations that fits log-viewer visualizations and links to trace logs and journals.

## Scope

- Review `GraphOpEvent`, `GraphMutation`, `Transition`, `path_id`, snapshots, and existing log-viewer replay expectations.
- Define grouping by operation id/path id/run id and deterministic step ordering.
- Distinguish observation-only search/read replay from mutation-capable insert/update journals.
- Specify snapshot-vs-delta rules, schema versioning, and benchmark timing links.
- Decide whether the canonical graph replay artifact lives in `context-api`, `log-api`, or both via metadata references.

## Acceptance criteria

- Log-viewer can replay graph operation streams from the proposed format.
- The format can link each replay stream to a trace log session and optional operation journal.
- Timing/profiling metadata does not pollute deterministic replay state.