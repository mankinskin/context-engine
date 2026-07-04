# Goal

Track the remaining performance work after the initial hot-path fixes so scan reconciliation and move flows can keep improving without losing traceability.

## Scope

This tracker owns the remaining follow-up slices for:

- incremental workflow-facts recompute
- batched residual SQLite/index writes during reconciliation
- targeted incremental reconcile/scan modes for move and related tooling

## Non-goals

- redoing the already-landed batched Tantivy commit fix
- broad ticket-store performance work outside scan/move reconciliation
- public CLI/HTTP/MCP surface redesign

## Done condition

This tracker closes when the linked child tickets complete and the owning spec is updated with evidence for the remaining slices.

## Context

This tracker builds on the already-implemented hot-path ticket `bf094901-cdb6-4b25-8ccd-3eb7716f9d20` and the related profiling/benchmark context captured in the existing evidence tickets.