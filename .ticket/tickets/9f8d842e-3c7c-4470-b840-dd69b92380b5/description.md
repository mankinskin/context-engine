# Problem

`RootManager` currently grows semantic roots through `flat_root`, `wrap_root`, `replace_last_child`, and `try_extend_tail_with`. That is mutation-heavy shortcut logic, not the structural block-to-block materialization described by the algorithm.

## Deviation from the algorithm

The algorithm materializes `t_block_{n+1}` from `t_block_n`, the chosen postfix, and the two complements. Current code rewrites the last child or whole root in place and keeps a repeated-atom fast path for semantic growth.

## Design decisions

- Unknown-atom buffering is allowed only before semantic structure exists; once a known-block step starts, root updates must be expressed as block materialization.
- `t_block_{n+1}` must be built from the previous block, the chosen postfix, and the two complements; semantic growth must not depend on last-child replacement shortcuts.
- `try_extend_tail_with` is not part of the intended algorithm. If a repetition case still matters, the general block rule must produce it.
- Dirty-cut replacement remains a lower-layer concern. `RootManager` should
	consume the root-level tokenization decision supplied by `context-insert`:
	either the requested token directly or a beneficial wrapper token, without
	reinterpreting dirty boundaries locally.

## 2026-05-26 dependency boundary

This ticket still owns the removal of mutation-heavy root growth shortcuts in
`context-read`, but it does not by itself solve the remaining dirty-cut
decomposition bug.

- The remaining `abcabababcaba`-style failure depends on an explicit
	requested-range versus witness-and-wrapper replacement plan in the split or
	merge layer.
- The root-materialization slice should therefore stay aligned with that plan:
	once a step is known structurally, `RootManager` should splice whichever token
	`context-insert` chose for the root update and leave requested-range exposure
	and inner materialization to the lower merge layer.

## Specification touchpoints

- Keep the `context-read pipeline` spec authoritative for the block transition formula.
- Keep the `induced graph structure` spec authoritative for the peer decompositions that must survive the transition.
- Keep the reviewed dirty-cut merge requirements and examples in:
	- [context-read pipeline](../../../.spec/specs/e0913182-7a5e-4c8f-a750-799afd58baae/body.md)
	- [graph induction](../../../.spec/specs/16c3ad95-451d-4c09-a118-ca90bcefed9a/body.md)

## Manual validation guidelines

When a spec-backed implementation lands for this ticket:

1. Run focused checks around local block growth:
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib sync_read_text2 -- --nocapture`
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib read_repeating_known1 -- --nocapture`
	- `cargo test --manifest-path context-stack/context-read/Cargo.toml --lib repetition_aabbaabb -- --nocapture`
2. Manually inspect the resulting root decompositions and confirm:
	- semantic growth happened by materializing the next block, not by swapping the last child in place;
	- repeated-atom cases still arise from the same general rule;
	- unknown-only buffering stops once the semantic step begins.
3. If the broad suite is still red, note which remaining failures are outside root materialization.

## Scope

- express known-block commits in terms of `t_block_n`, `t_postfix`, `t_overlap`, `t_complement_block`, and `t_complement_overlap`
- remove `try_extend_tail_with` from semantic overlap growth
- restrict flat concatenation to pure unknown-atom buffering only

## Acceptance criteria

- the known-block commit path no longer depends on `try_extend_tail_with`
- semantic root updates are structural materializations of the next block, not last-child surgery
- repeat and rotating-overlap scenarios stay green without atom-tail special cases
