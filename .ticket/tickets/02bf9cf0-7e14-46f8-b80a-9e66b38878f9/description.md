# Problem

The repository workflow requires reusable validation capture, but the corrected architecture is embedded workflow metadata in the ticket/spec/doc layers plus future first-class test/log stores.

An initial wrapper-oriented prototype proved some capture mechanics, but that shape is not the product goal.

# Scope

Document the prototype validation slice as migration input for the corrected design.

This ticket should:

- record what the prototype proved about focused validation capture
- point the target architecture at [.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c](.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c)
- link the first-class follow-up ownership to [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a) and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23)
- make clear that wrapper-oriented validation capture is migration context, not the long-term public interface

# Acceptance criteria

- Prototype validation capture exists as migration input for the rewritten architecture.
- This ticket points to [.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c](.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c), [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a), and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23) as the target design path.
- Wrapper-oriented validation capture is explicitly documented as transitional rather than target architecture.

# Implementation status

- A wrapper-oriented validation prototype exists and produced reusable validation artifacts.
- The corrected target architecture now lives in [.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c](.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c), [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a), and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23).

# Validation status

- The prototype validation slice was exercised with focused smoke checks.
- A reusable validation artifact is recorded at `target/tmp/workflow-smoke/20260517T130242Z-03695a48-7dfb-4eb8-b914-6cbf76362f28.json`.

# Documentation status

- The corrected design path is documented in [.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c](.ticket/tickets/75e9fef3-b624-4e12-9709-5d800222908c), [.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a](.ticket/tickets/5a4c2e4d-e7d9-4138-8f25-c699942f739a), and [.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23](.ticket/tickets/501d4932-a48e-4c8a-a4f3-8c31be0bdd23).
