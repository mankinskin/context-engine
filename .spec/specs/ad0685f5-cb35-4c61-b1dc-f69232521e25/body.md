<!-- aligned-structure:v2 -->

# Directed Contract Edge

## Responsibility And Interface

Record one consumer dependency on provider-owned criteria. The edge requires
`id`, `spec_id`, consumer/provider component IDs, nonempty
`provider_criterion_ids[]`, and `name`.

## Behavior And Contract

- `edge-required-fields`, `edge-nonempty-provider-criteria`, and
	`edge-provider-ownership` validate shape and provider ownership.
- `edge-distinct-endpoints` rejects self dependencies; `edge-cycles-allowed`
	permits multi-component cycles.
- `edge-unique-claim` rejects duplicate `(consumer, provider, criterion)` claims.
- `edge-consumer-does-not-copy` preserves the root ownership invariant.
- Consume Component membership/ownership and Criterion single-owner/uniqueness.

## Boundaries And Failure Cases

An edge is not a copied criterion or a hierarchy edge. Self-edge, empty list,
foreign criterion, missing endpoint, and duplicate claim are invalid.

## Acceptance Evidence And Position

Add a two-component cycle and each rejected case to `src/store/tests.rs`.
Current schema only declares generic `depends_on`, `linked`, and `parent_of` rules.
