# Problem

The repository workflow requires documentation validation for authored docs and generated guidance surfaces, but the corrected architecture is native workflow metadata owned by `doc-api` and surfaced by a future `doc-cli`.

An initial wrapper-oriented prototype proved some capture mechanics, but that shape is not the product goal.

# Scope

Document the prototype documentation-validation slice as migration input for the corrected design.

This ticket should:

- record what the prototype proved about documentation-validation capture
- point the target architecture at [.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5](.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5)
- link the primary implementation ownership to [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876)
- make clear that wrapper-oriented documentation capture is migration context, not the long-term public interface

# Acceptance criteria

- Prototype documentation-validation capture exists as migration input for the rewritten architecture.
- This ticket points to [.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5](.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5) and [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876) as the target design path.
- Wrapper-oriented documentation capture is explicitly documented as transitional rather than target architecture.

# Implementation status

- A wrapper-oriented documentation-validation prototype exists and produced reusable documentation-validation artifacts.
- The corrected target architecture now lives in [.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5](.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5) and [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876).

# Validation status

- The prototype documentation-validation slice was exercised with focused smoke checks.
- A generated-guidance validation artifact is recorded at `target/tmp/workflow-smoke/20260517T140113Z-540e4674-6224-45d5-8b30-1cfdebf8321b.json`.

# Documentation status

- The corrected design path is documented in [.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5](.ticket/tickets/06778dd8-a894-4759-b8fc-f00f6dd21fa5) and [.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876](.ticket/tickets/ad9f6e52-2147-4b25-be2c-9e59dd58a876).
