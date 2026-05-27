# Problem

The current pipeline materializes overlap products locally but does not reliably revisit already-known roots when new subparts appear later. That is why `bcdea` still misses `[bc, dea]` after later reads materialize `bc` and `dea`.

## Deviation from the algorithm

Multiple first-class decompositions are accumulating graph facts. When a later read materializes tighter subparts for an existing span, prior roots for that same span must gain the compatible decomposition.

## Design decisions

- Revisitation is keyed by equal-span token identity, not by projection order or caller-facing tree selection.
- Recomposition adds peer `child_patterns`; it must not delete or replace the earlier valid decomposition.
- Public projection remains deterministic and secondary. The graph fact comes first.

## Specification touchpoints

- Keep the revisitation rule explicit in the `induced graph structure` spec for the rotating-overlap family.
- Worked traces should name the newly-added peer decomposition on the already-known root, not only on the latest root.

## Manual validation guidelines

When a spec-backed implementation lands for this ticket:

1. Run the rotating-overlap regression directly:
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_multiple_overlaps1 -- --nocapture`
2. Manually confirm from the resulting graph state that:
	- `bcdea` gains `[bc, dea]` once `bc` and `dea` exist;
	- earlier roots keep their prior valid decompositions;
	- no public-tree ordering hack is being used to fake the graph fact.
3. If broader reread cases are still blocked, record which equal-span recompositions are still missing.

## Scope

- define when a newly materialized token should trigger recomposition of existing equal-span parents
- repair rotating-overlap corpus behavior without relying on projection-order hacks
- document the recomposition rule in the induced-graph spec and worked traces

## Acceptance criteria

- `bcdea` gains `[bc, dea]` once `bc` and `dea` exist
- the rotating-overlap family matches the spec'd decomposition set
- recomposition behavior has focused tests separate from public projection ordering
