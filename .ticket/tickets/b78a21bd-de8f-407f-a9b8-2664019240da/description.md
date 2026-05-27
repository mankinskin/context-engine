# Problem

`ExpansionCtx` reimplements postfix search with `collect_postfix_candidates`, `collect_postfix_candidates_inner`, and `find_postfix_path`. That search ignores the graph invariant that each node already links directly to its largest contained postfix tokens, so the code walks more structure than the algorithm requires.

## Deviation from the algorithm

The read algorithm should follow each longest postfix path from the current block and choose the first postfix on those paths that can expand rightward. Current code instead collects every postfix candidate at every node, sorts them again, and reconstructs paths manually.

## Design decisions

- `context-trace` is the sole owner of longest postfix path discovery and largest-direct-postfix descent.
- `context-read` may adapt trace output into overlap-search inputs, but it must not rebuild the postfix tree or rescan every postfix of every node.
- Path output must carry the full root-to-postfix route through the intermediate largest postfix tokens so complement construction can reuse it directly.

## Specification touchpoints

- Keep the owner boundary explicit in the `context-read pipeline` spec: trace owns longest postfix paths, read selects along those paths and advances the block algorithm.
- If spec code references change, move postfix-path refs toward trace-owned traversal surfaces and leave only orchestration refs in `context-read`.

## Manual validation guidelines

When a spec-backed implementation lands for this ticket:

1. Run focused read checks:
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_repeating_known1 -- --nocapture`
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_multiple_overlaps1 -- --nocapture`
2. If unrelated read failures still block the broader suite, manually inspect the emitted graph snapshot or test log and confirm:
	- each search step follows only the largest direct postfix branch(es) of the current token;
	- postfix-path ordering comes from trace traversal rather than a read-local collector;
	- the overlap path used for complement construction matches the actual graph embedding.
3. Record whether the spec change only clarified ownership or also required code-ref relocation.

## Scope

- replace custom postfix candidate collection with trace-owned longest postfix paths
- use the largest-direct-postfix invariant instead of rescanning every postfix at every node
- keep largest-first postfix-path selection deterministic and testable

## Acceptance criteria

- `context-read/src/expansion/mod.rs` no longer owns custom postfix candidate collection or postfix-path reconstruction
- focused tests prove largest-direct-postfix descent and exact path recovery for repeat and rotating-overlap fixtures
- remaining blockers, if any, are moved into separate tickets instead of hiding inside the overlap probe path
