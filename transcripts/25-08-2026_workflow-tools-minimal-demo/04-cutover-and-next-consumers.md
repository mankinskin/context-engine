# 04 - Apply the Proven Contract

## Outcome

Use the green minimal-consumer contract to complete the remaining artifact migration and reframe context-engine, then integrate pitch-scripts as the next product consumer.

## Existing Owners

[artifact stores](../../.ticket/tickets/47a0bcc3-f42d-475e-b05a-777293c4698e/ticket.toml), [artifact migration](../../.ticket/tickets/47f2a664-7803-4074-b40c-f41d3caf0c54/ticket.toml), [context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml), [end-to-end cutover validation](../../.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml), and [installation documentation](../../.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml) own the corresponding phases.

## Requirements

- Artifact implementation moves retain their current Phase C ownership and use safe cross-store moves.
- Context-engine removes vendored workflow-tool coupling only after the minimal fixture proves the public path.
- context-engine's final consumer configuration no longer needs local path patches for workflow domains.
- pitch-scripts uses the published tutorial contract as a second consumer, with deviations captured as compatibility requirements rather than fixture-specific hacks.

## Non-Goal

Do not migrate every historical artifact or use pitch-scripts to discover the base install contract.

## Validation

Run the minimal fixture unchanged against the published contract, then run the context-engine clean-checkout gate and a pitch-scripts integration scenario. Validate reference integrity after each artifact-migration batch.