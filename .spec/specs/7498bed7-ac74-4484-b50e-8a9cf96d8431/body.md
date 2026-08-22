<!-- aligned-structure:v2 -->

# Evidence Reference

## Responsibility And Interface

Link external material without embedding fulfillment. An evidence reference
requires `id`, `spec_id`, `target_kind`, `target_ref`, and `relation`; `locator`
is optional. Store-backed targets follow `src/ticket_ref.rs` explicit workspace
and store-root identity.

## Behavior And Contract

- `evidence-required-fields`: require the five target fields.
- `evidence-optional-locator`: preserve an exact target location when supplied.
- `evidence-explicit-store-target`: forbid implicit store resolution.
- `evidence-external-state`: link material without mandatory fulfillment state.
- Consume root membership and Document Store locatability.

## Boundaries And Failure Cases

Evidence is neither an observation nor a success claim. Missing identity,
unresolvable declared store metadata, or a cross-root reference is invalid;
an unavailable document is not a health gate.

## Acceptance Evidence And Position

Add TicketRef-style serialization tests and manifest/store reference checks.
`src/ticket_ref.rs` is the implemented explicit-resolution baseline.
