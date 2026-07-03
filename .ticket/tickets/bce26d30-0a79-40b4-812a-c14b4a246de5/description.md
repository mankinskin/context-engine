# Goal

Create the documentation and validation matrix for the unified logging and journaling design.

## Scope

- Document configuration, field naming, privacy/redaction rules, retention, and examples for tests, benchmarks, servers, MCP, CLI, and agent sessions.
- Add validation cases for shared tracing initialization, runtime log metadata, journal metadata, move-journal compatibility, graph replay linkage, active-file indexing, and transport correlation.
- Record which checks are automated vs manual/browser/log-viewer validation.

## Acceptance criteria

- The tracker has a clear evidence checklist before review.
- Documentation explains how to enable logs/journals and how to search them.
- Validation covers both completed and active log files.