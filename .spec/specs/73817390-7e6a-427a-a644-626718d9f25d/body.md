<!-- aligned-structure:v2 -->

# Document Store Evidence Integration

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) is the in-repository explicit-target-resolution baseline consumed by this adjacent provider contract.

## Naming Conventions

Use Document API-owned typed document target grammar and `document-` criterion
ids. This child owns `document-stable-target`, `document-resolution-result`, and
`document-evidence-nongating`.

## Reading Order

1. [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md) - document-target consumer.
2. [workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) - explicit locator baseline.

## Responsibility

If implemented by the adjacent Document API, evidence consumers can resolve a
stable document identity and review location without using availability as
fulfillment status.

## Interfaces And Dependencies

The Document API owns the document repository/index and typed resolver grammar
before spec evidence resolution ships. A typed target supplies stable identity
and optional locator; resolution returns exactly `Resolved { record }`,
`Missing { id }`, or `Unsupported { target }`. This is a specified-but-not-built
prerequisite: spec-api does not introduce an initial adapter grammar or free-form
path semantics.

## Behavior

- `document-stable-target`: a document has a stable target reference.
- `document-resolution-result`: `Resolved { record }` exposes identity and review location; `Missing { id }` and `Unsupported { target }` are typed non-success outcomes.
- `document-evidence-nongating`: unavailable or unobserved material is not a mandatory-evidence health failure.

## Boundaries And Failure Cases

The document store neither decides criterion success nor manufactures a locator.
Missing documents yield `Missing { id }`, unsupported targets yield
`Unsupported { target }`, and neither outcome establishes success or changes
health policy. Before the Document API repository/index and grammar exist,
evidence-reference behavior cannot claim document resolution.

## Provider/Consumer Contract

Provides `document-stable-target` and `document-resolution-result` to [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md); consumes no spec-api artifact beyond the explicit-target convention.

## Examples

A Document API typed target resolves to its stable identity and heading. A
missing target returns `Missing { id }`, which Evidence Reference can consume
without claiming a criterion was fulfilled.

## Evidence

Position: `not-implemented` in this repository: the Document API repository,
index, and typed resolver grammar do not exist. Validate `Resolved`, `Missing`,
and `Unsupported` in the owning Document API test suite before spec evidence
resolution is implemented.

## Scope

Defines the provider contract only; document storage implementation remains external.
