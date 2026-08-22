# 02 - Existing Capability and Contract Decision

## Outcome

Establish which target-model elements can reuse the current spec system and isolate the semantic decision that must be made before a schema or migration proposal is written.

## Verified Capability

The current spec system already provides useful building blocks:

- structured manifest fields for acceptance criteria, evidence requirements, expected properties, fulfillment summaries, and related tickets;
- named sections and hierarchy traversal;
- typed schema edge rules, including direction and cycle constraints; and
- structured `TicketRef` support for cross-store references.

The Presentation System spec does not currently use those structured contract fields. Its prose acceptance criteria and evidence expectations therefore cannot participate in the manifest's automated structural health checks.

## Boundary of Existing Capability

Existing typed edges and hierarchy are not sufficient evidence that the desired contract model already exists. The desired edge needs explicit consumer/provider roles and criterion references whose fulfillment is owned by the provider and claimed by the consumer. That semantic mapping has not been established by the reviewed source.

## Required Decision

Decide contract ownership before proposing storage changes:

> Does each component declare only the expectations it has of another component, or does it also declare the contract it offers to other components?

The selected answer must state how a contract edge is stored, which side owns its criterion references, and how duplicate or conflicting declarations are prevented.

## Non-Goal

This work package does not add new edge kinds, alter the schema, or migrate any legacy spec.

## Validation Method

Create a small set of representative component relationships, including a cycle, and verify on paper that the selected ownership rule assigns every contract, provider obligation, consumer claim, and acceptance criterion exactly once.

Use these repository checks as evidence when a later implementation proposal is prepared:

```bash
cargo test -p spec-api --test schema_test
./target/debug/spec.exe get 2ccde9ee-85ac-4c87-9601-f6099f5be01c --json
./target/debug/spec.exe health --all
```