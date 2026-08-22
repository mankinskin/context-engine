# 01 - Case Study and Target

## Outcome

Produce a decision-ready comparison between the Presentation System specification and the intended component-oriented specification model.

## Case-Study Finding

The Presentation System specification is a useful example of the current monolithic shape: one prose body combines confirmed decisions, Phase 1 delivery, later conceptual-deck work, requirements R1-R15, acceptance criteria AC1-AC17, deferred ideas, and non-goals. Its manifest records only basic identity and classification metadata.

The problem is not that the prose is incomplete. The problem is that the document is the primary container for architecture, contracts, acceptance criteria, evidence expectations, and delivery phases. A reader cannot traverse those concepts as independent, linked specification artifacts.

## Target Model

The future model should make the following entities independently addressable:

1. **Component**: a small specification artifact with a concise purpose, usage context, and responsibility.
2. **Acceptance criterion**: a short, measurable artifact that can be validated by one or more tests or validation executions.
3. **External evidence reference**: a typed link to tests, tickets, documentation, and other relevant context.
4. **Directed contract edge**: a relationship from a consuming or reading component to a serving or writing component. It names the interface contract and references the criteria the serving component must satisfy for the consumer to accept the contract.

The model permits cycles: components may consume and serve one another. A collection of component artifacts and their edges should describe the system being built, including interactions manifested in code and storage.

## Non-Goal

This work package does not rewrite the Presentation System specification or prescribe a serialization format for the target model.

## Validation Method

Review the resulting comparison against:

- the current Presentation System manifest and body; and
- the clean specification-architecture guidance.

The comparison is complete when each target entity is either mapped to a verified existing primitive or explicitly recorded as a gap.