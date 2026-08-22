<!-- aligned-structure:v2 -->

# Document Store Evidence Integration

## Responsibility And Interface

Supply stable, locatable document targets to Evidence Reference through a target
reference and optional locator. It is an adjacent evidence provider, not a
fulfillment service.

## Behavior And Contract

- `document-stable-target`: a document has a stable target reference.
- `document-resolution-result`: resolution exposes identity and review location.
- `document-evidence-nongating`: unavailable or unobserved material is not a
	mandatory-evidence health failure.
- Evidence Reference consumes the first two criteria through the root map.

## Boundaries And Failure Cases

The document store does not decide criterion success or manufacture a locator.
Missing documents yield a resolvable failure result, not a changed health policy.

## Acceptance Evidence And Position

Validate a locatable document and a missing-target result through the reviewed
document API tests. This contract intentionally leaves implementation ownership
to that adjacent subsystem.
