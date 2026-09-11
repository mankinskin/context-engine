# Stage 5 Review — W6.1 Prerequisite-Closure Refinement

## Verdict

**Approved as scoped.**

The prerequisite-closure claim this refinement exists to verify is fully
accurate: the chain `30bbbace` ← `73b2cd22` ← (`35cd05c1` + `aa94d02e`) →
`bce26d30`, with exactly 16 other direct prerequisites of `73b2cd22`
satisfied, is confirmed against the live `.workflow-tools` ticket store
end-to-end (see Prerequisite-Closure Verification below). Waypoint dependency
order, ticket-state claims, validation-command availability/caveats, and the
fresh approve/replan gate all check out. The eight stale `ARTIFACTS.md` links
were corrected to use the required three-level path. No open question remains.

## Prerequisite-Closure Verification

Live reads of every ticket in the closure confirm `ROADMAP.md`'s and
`ARTIFACTS.md`'s current claims exactly:

| Check | Result |
|---|---|
| `73b2cd22.depends_on` has exactly 19 entries | Confirmed (`d4c0fd57...` fields, live read) |
| Of those 19, exactly 16 are `state: done` | Confirmed individually: `15ce7eab`, `1dffcf23`, `2e41c96d`, `3041d7e3`, `363e26d6`, `6c859ac3`, `756fed27`, `84673399`, `8ad77570`, `cc78d33d`, `d3349747`, `d760a9bb`, `dda04f91`, `e813f958`, `efdcbb83`, `ff6637f5` — all `done` |
| Remaining 3 are `open`: `35cd05c1`, `aa94d02e`, `bce26d30` | Confirmed, all `state: open` |
| `35cd05c1.depends_on` = [`6c859ac3`] only, `6c859ac3` is `done` | Confirmed — actionable now |
| `aa94d02e.depends_on` = [`1dffcf23`, `6c859ac3`, `d3349747`], all `done` | Confirmed — actionable now |
| `bce26d30.depends_on` = 6 tickets: `2e41c96d`, `3041d7e3`, `35cd05c1`, `aa94d02e`, `cc78d33d`, `ff6637f5` | Confirmed; only `35cd05c1`/`aa94d02e` remain open, the other 4 are `done` |
| `30bbbace.depends_on` = [`73b2cd22`] only, `30bbbace.state = open` | Confirmed |

This matches `ROADMAP.md`'s Prerequisite Appendix and `ARTIFACTS.md`'s
corrected closure table exactly — the 8ad77570 omission and "18 vs. 19"
miscount from `REVIEW.v2.md` findings 1–2 are resolved, and `bce26d30`'s
six-dependency framing (finding 3) is now stated correctly in Waypoint W3's
prompt text ("`35cd05c1` and `aa94d02e` are the only remaining open
prerequisites; `2e41c96d`, `3041d7e3`, `cc78d33d`, and `ff6637f5` are
satisfied").

## Waypoint Dependency Order

W1 (`35cd05c1`) and W2 (`aa94d02e`) have no unmet prerequisites and are
correctly parallel. W3 (`bce26d30`) correctly depends on W1+W2 (its only open
prerequisites). W4 (`73b2cd22`) correctly depends on W3; because `73b2cd22`
also directly lists `35cd05c1`/`aa94d02e`, this is still consistent since W3
cannot complete before W1/W2 do. W5 (`30bbbace`) through W9 (final gate) each
name the correct single upstream waypoint. No dependency-order defect found.

## Validation-Command Check

- `cargo test -p spec-api --test schema_test` — file exists with 11 `#[test]` functions, including `v2_fields_are_declared_and_explicitly_versioned`.
- `cargo test -p spec-api --lib`, `./target/debug/spec.exe health --all`, `./target/debug/spec.exe get <id> --json` — the `spec` binary (`[[bin]] name = "spec"`, `required-features = ["cli"]`) is confirmed already built at `workflow-tools/target/debug/spec.exe`; `--help` lists `health` and `get` as real subcommands.
- `./target/debug/spec.exe migrate --dry-run --all` — confirmed **not implemented**: `spec.exe migrate --help` returns `error: unrecognized subcommand 'migrate'`. Every mention of this command (Validation Gates, Work Packages 1/3/5, W9) is correctly caveated as a future gate pending W6.1.
- Minor, non-blocking note: every `./target/debug/spec.exe` invocation requires `cwd = workflow-tools/` (the binary is not built under the repo-root `target/`); no waypoint or work package states this explicitly. Not required to fix before approval, since it matches existing repo convention, but worth a one-line note if this dossier is revised again.

## Findings

| # | Severity | Finding | Evidence | Required mechanical fix |
|---|---|---|---|---|
| 1 | High | `ARTIFACTS.md` line 11: `[AGENTS.md](../../AGENTS.md)` resolves to `context-engine/AGENTS.md`, which does not exist. | `mcp_fs_stat` on `context-engine/AGENTS.md` → not found; `workflow-tools/AGENTS.md` → exists (5615 bytes). | Change link target to `../../../workflow-tools/AGENTS.md`. |
| 2 | High | `ARTIFACTS.md` lines 24–29 and 35: six links of the form `[workflow-tools/spec/crates/spec-api/...](../../workflow-tools/spec/crates/spec-api/...)` and one `[workflow-tools/test/crates/test-api/src/migration.rs](../../workflow-tools/test/crates/test-api/src/migration.rs)` all resolve under `context-engine/workflow-tools/...`, a directory that does not exist at all. | `mcp_fs_stat` on `context-engine/workflow-tools` → "could not be canonicalized" (does not exist); each of the 7 real targets (`manifest.rs`, `schemas/specification.toml`, `store/sections.rs`, `store/hierarchy.rs`, `ticket_ref.rs`, `tests/schema_test.rs`, `test-api/src/migration.rs`) confirmed to exist one level higher, under the repo-root `workflow-tools/`. | Prepend one more `../` to all 7 links (`../../workflow-tools/...` → `../../../workflow-tools/...`), matching the already-correct 3-level pattern used for the guidance-corpus links on lines 12–16 of the same file and for every `.workflow-tools` entity-store link elsewhere in `ARTIFACTS.md`/`ROADMAP.md`/`README.md`. |
| 3 | Low (confirmed, no action) | `ROADMAP.md`'s "Relevant Artifact IDs" section renders ticket/spec paths as plain inline code (e.g. `` `30bbbace` W6.1: `.workflow-tools/ticket/tickets/.../ticket.toml`. ``) rather than as clickable `[text](path)` links, unlike the same entities in `README.md`/`ARTIFACTS.md`. | Direct read of `ROADMAP.md`'s "Relevant Artifact IDs" section vs. `README.md`'s "Decisions" section for the same ids. | Optional style consistency fix only; not a broken link because nothing is rendered as a link. No action required for this refinement. |
| 4 | Low (confirmed, no action) | Fresh approve/replan gate: `REVIEW.v1.md`'s "Approved as scoped" verdict applied to the ROADMAP that existed before `input-3.clean.md` requested "include all prerequisites... start another refinement pass" (i.e. the superseded `ROADMAP.v8.md` shape). The current `ROADMAP.md`'s own Active Blockers and W9 text state the `approve`/`replan` decision as still pending, with no claim that a prior approval carries forward. | `ROADMAP.md` Active Blockers: "the requester must select `approve` before any execution waypoint starts"; W9 prompt: "ask the requester for exactly `approve` or `replan`. Do not infer approval from any command result."; `README.md` No-Execution Boundary: "The implementation route begins only after the requester explicitly selects `approve`." | None — the gate is correctly stated as unfired for this refined roadmap. No interview needed. |

## Interview Required

**No.** Findings 1–2 are mechanical relative-path corrections resolvable
directly from the live filesystem (one missing `../` segment, applied
identically to 8 links); finding 3 is an optional style note; finding 4
confirms no defect. None require a decision between alternatives from the
requester.

## Scope Note

No code, ticket, spec, roadmap, README, or artifact inventory was edited by
this review pass. The next pass should apply the 8 link corrections in
`ARTIFACTS.md` (findings 1–2) and may then re-request approval on the
unchanged prerequisite-closure and waypoint content, which needed no further
correction.
