<!-- aligned-structure:v2 -->

# Document Store Evidence Integration

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) is the in-repository explicit-target-resolution baseline consumed by this adjacent provider contract.

## Naming Conventions

Use `document-<repository>-<name>` target refs and `document-` criterion ids.
This child owns `document-stable-target`, `document-resolution-result`, and `document-evidence-nongating`.

## Reading Order

1. [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md) - document-target consumer.
2. [workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) - explicit locator baseline.

## Responsibility

If implemented by the adjacent document subsystem, evidence consumers can resolve
a stable document identity and review location without using availability as fulfillment status.

## Interfaces And Dependencies

A document target supplies a stable reference and optional locator; resolution
returns identity plus review location or a resolvable failure result.

## Behavior

- `document-stable-target`: a document has a stable target reference.
- `document-resolution-result`: resolution exposes identity and review location.
- `document-evidence-nongating`: unavailable or unobserved material is not a mandatory-evidence health failure.

## Boundaries And Failure Cases

The document store neither decides criterion success nor manufactures a locator.
Missing documents yield a resolvable failure result, not changed health policy.

## Provider/Consumer Contract

Provides `document-stable-target` and `document-resolution-result` to [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md); consumes no spec-api artifact beyond the explicit-target convention.

## Examples

A document reference resolves `transcripts/spec-template.md#health` to its stable
identity and heading. A missing file returns a missing-target result that Evidence
Reference can record without claiming a criterion was fulfilled.

## Evidence

Position: `not-implemented` in this repository; validate a locatable document
and missing-target result in the owning document API test suite when it is linked.

## Scope

Defines the provider contract only; document storage implementation remains external.
