# Problem

The repository workflow expects first-class traceability across tickets, specs, docs, validation records, and logs, but the corrected architecture is store-owned metadata rather than wrapper-owned link payloads.

An initial wrapper-oriented prototype proved some lookup mechanics, but that shape is not the product goal.

# Scope

Document the prototype traceability slice as migration input for the corrected design.

This ticket should:

- record what the prototype proved about cross-store traceability capture
- point the target architecture at [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274)
- link the first-class follow-up ownership to [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a) and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)
- make clear that wrapper-owned link capture is migration context, not the long-term source of truth

# Acceptance criteria

- Prototype cross-store traceability capture exists as migration input for the rewritten architecture.
- This ticket points to [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274), [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a), and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23) as the target design path.
- Wrapper-owned link payloads are explicitly documented as transitional rather than authoritative architecture.

# Implementation status

- A wrapper-oriented traceability prototype exists and produced reusable cross-store lookup artifacts.
- The corrected target architecture now lives in [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274), [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a), and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23).

# Validation status

- The prototype traceability slice was exercised with focused smoke checks.
- A traceability lookup artifact is recorded at `target/tmp/workflow-smoke/20260517T130326Z-c6b8b9f6-cdf6-40b7-afb1-ea83013900eb.json`.

# Documentation status

- The corrected design path is documented in [.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274](.ticket/tickets/0fb5a2e5-af2b-4b52-81a5-c3a49ffc3274), [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a), and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23).
