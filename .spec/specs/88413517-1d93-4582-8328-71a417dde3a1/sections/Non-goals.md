Explicitly out of scope for this spec and its implementing tickets:

- Rebuilding ticket-DAG traversal tooling. `topgraph`, `next_tickets`, `subgraph`, and `list_edges` already exist in ticket-mcp/ticket-api and are reused as-is; no new DAG query surface is introduced here.
- The external workflow-skill epic (skill packaging/distribution for third-party skill authors) — tracked separately and not touched by C1-C6.
- Cost-telemetry instrumentation beyond the Telemetry template's role definition (R8's purpose/trigger/tooling). Actual metering, dashboards, and cost-gate enforcement are a separate track covered by a4d61b8c-df1c-454d-ab56-4bce5706eb15 and 39983ddf-1f7e-4081-a060-6b8258eb4c41; this spec only assigns R8 its owning template.