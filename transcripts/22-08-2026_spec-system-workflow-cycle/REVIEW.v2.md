# Stage 3 Continuation Review — W6.1 Dependency-Closure Expansion

## Verdict

**Changes requested.**

Expanding `ROADMAP.md`'s Active Blockers to state the complete W6.1
dependency closure — not just "blocked by [73b2cd22](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml)" but the full chain through
[35cd05c1](../../../.workflow-tools/ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml), [aa94d02e](../../../.workflow-tools/ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml), and [bce26d30](../../../.workflow-tools/ticket/tickets/bce26d30-0a79-40b4-812a-c14b4a246de5/ticket.toml) — is the right
direction and the core relationship the request describes is verified
accurate against the live `.workflow-tools` ticket store. It cannot be
approved as scoped yet: the source closure table in `ARTIFACTS.md`, which
`ROADMAP.md` would draw from, is itself incomplete. A live re-read of
`73b2cd22`'s `depends_on` field found a 19th direct prerequisite,
[8ad77570](../../../.workflow-tools/ticket/tickets/8ad77570-6dbe-4647-9d58-bf324a82b4fc/ticket.toml), that does not appear anywhere in `ARTIFACTS.md`'s
prerequisite-closure table (neither in its "done" rows nor its "frontier open
nodes" rows), and `ARTIFACTS.md`'s own prose states "18 direct prerequisites"
where the live ticket has 19. Copying that table into `ROADMAP.md` under the
label "complete dependency closure" would carry the gap forward into the
document meant to be authoritative.

## Findings

| # | Severity | Finding | Evidence | Required improvement |
|---|---|---|---|---|
| 1 | High | `ARTIFACTS.md`'s prerequisite-closure table for [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) omits ticket [8ad77570](../../../.workflow-tools/ticket/tickets/8ad77570-6dbe-4647-9d58-bf324a82b4fc/ticket.toml), a direct prerequisite of `73b2cd22` that is `state: done` with recorded validation evidence (`validation_execution_id: exec-8ad77570-log-session-linkage-20260705`). The ticket does not appear in either the "done" rows or the "Frontier (open) nodes directly blocking 73b2cd22" rows of the table. | Live `73b2cd22.depends_on` (fetched from `.workflow-tools`) lists 19 ids: `15ce7eab, 1dffcf23, 2e41c96d, 3041d7e3, 35cd05c1, 363e26d6, 6c859ac3, 756fed27, 84673399, 8ad77570, aa94d02e, bce26d30, cc78d33d, d3349747, d760a9bb, dda04f91, e813f958, efdcbb83, ff6637f5`. `ARTIFACTS.md`'s table lists only 18 of these 19 (15 "done" direct rows + the 3 open rows); `8ad77570` is absent. A file-wide `grep` for `8ad77570` in `ARTIFACTS.md` returns no match. `8ad77570` itself resolves `state: done`. | Add `8ad77570` to `ARTIFACTS.md`'s prerequisite-closure table as a `done` direct prerequisite before that table (or any summary drawn from it) is copied into `ROADMAP.md` as "the complete dependency closure." |
| 2 | High | `ARTIFACTS.md`'s own "Current facts" prose states "Of the **18** direct prerequisites of `73b2cd22`..." — this undercounts the live ticket's 19 direct `depends_on` entries by exactly one, consistent with finding 1's missing row. The requester's stated closure ("19 direct prerequisites, 16 done, three open") matches the live totals but not `ARTIFACTS.md`'s own written count. | Direct field comparison: `73b2cd22.depends_on` has 19 elements (confirmed above); `ARTIFACTS.md` prose says "18." With `8ad77570` restored as `done`, the correct split is 16 done direct (15 currently listed + `8ad77570`) and 3 open (`35cd05c1`, `aa94d02e`, `bce26d30`), totaling 19 — matching the requester's figures, not `ARTIFACTS.md`'s current "18." | Correct the "18 direct prerequisites" sentence in `ARTIFACTS.md` to "19," in the same edit that restores the missing `8ad77570` row, so the prose and table agree with each other and with the live store. |
| 3 | Medium | The requester's summary "`bce26d30` depends on both [`35cd05c1` and `aa94d02e`]" is true but incomplete: `bce26d30`'s live `depends_on` field lists **six** tickets — `2e41c96d, 3041d7e3, 35cd05c1, aa94d02e, cc78d33d, ff6637f5` — of which four (`2e41c96d`, `3041d7e3`, `cc78d33d`, `ff6637f5`) are already `done`. Only `35cd05c1` and `aa94d02e` remain open, which is why the two-item framing is directionally correct, but a roadmap sentence that says only "`bce26d30` depends on both" without noting the other four satisfied dependencies could read as `bce26d30` having exactly two prerequisites rather than six (two open, four done). | Live `bce26d30.depends_on` field (fetched from `.workflow-tools`): `["2e41c96d-fe9f-4cf2-b941-6f0d452f237c","3041d7e3-2b34-4597-b354-e0aa6ffb0459","35cd05c1-45f7-4d65-b943-7c000570928f","aa94d02e-9620-4db6-9974-36699cd56537","cc78d33d-1744-4945-bb77-f0fd1142568e","ff6637f5-01f6-46c3-b727-e1a19ee0f202"]`; all four non-`35cd05c1`/`aa94d02e` entries are `state: done` per `ARTIFACTS.md`'s existing table. | If `ROADMAP.md` is expanded to name `bce26d30`'s dependency on `35cd05c1` and `aa94d02e`, phrase it as "`bce26d30`'s only remaining open prerequisites are `35cd05c1` and `aa94d02e`; its other four declared dependencies are already done" so the closure reads as complete rather than as `bce26d30` having only two prerequisites. |
| 4 | Low (confirmed, no action) | The requester's core relationship claim — `35cd05c1` and `aa94d02e` are independently actionable now (each has only already-`done` prerequisites), `bce26d30` depends on both of them and is blocked until they complete, and completing `bce26d30` clears the last open direct prerequisite of `73b2cd22` — is verified accurate against the live store once `8ad77570` (finding 1) is accounted for as already-done. | Live reads of `35cd05c1` (single prerequisite `6c859ac3`, `state: done`), `aa94d02e` (all declared prerequisites `done` per `ARTIFACTS.md`), and `bce26d30` (six prerequisites, four done, two open — `35cd05c1` and `aa94d02e`) all corroborate `ARTIFACTS.md`'s "Minimal actionable frontier" and "Short recommended next step" sections. | None — this is the sound part of the request and needs no correction. |
| 5 | Low (confirmed, no action) | `ROADMAP.md`'s current "Active Blockers" entry names `35cd05c1` and `aa94d02e` as the unresolved frontier tickets but omits `bce26d30` entirely, even though `bce26d30` is the ticket that actually gates `73b2cd22` and is itself blocked on the two named tickets. This is the gap the requester's continuation intends to close, and the intended fix (naming `bce26d30` explicitly, with its relationship to `35cd05c1`/`aa94d02e`) is directionally correct. | Read of `ROADMAP.md`'s "Active Blockers" section: text names only `35cd05c1` and `aa94d02e`, with no mention of `bce26d30`. | None beyond findings 1-3 — confirms the requested expansion targets a real, currently-undocumented gap in `ROADMAP.md`; the expansion should proceed only after `ARTIFACTS.md`'s source table is corrected. |

## Scope Decision

The requested continuation — expand `ROADMAP.md`'s dependency-closure
statement for W6.1 to include the full chain through `bce26d30` down to
`35cd05c1`/`aa94d02e` — is the correct scope and is not, on its own, an
over-reach: `ROADMAP.md` currently under-documents its own blocker chain
(finding 5), and the request's core dependency relationship is accurate
(finding 4). The scope is **not yet unambiguous and evidence-supported as
drafted**, because the artifact the expansion would draw from —
`ARTIFACTS.md`'s prerequisite-closure table — has a verified gap (findings
1-2: a missing done ticket, `8ad77570`, and a resulting off-by-one count in
prose) and an imprecision that would understate `bce26d30`'s true dependency
count if copied verbatim (finding 3). Expanding `ROADMAP.md` from the current
`ARTIFACTS.md` table, unmodified, would propagate an inaccurate closure into
the document the roadmap positions as authoritative. Correct `ARTIFACTS.md`
first (findings 1-3), then expand `ROADMAP.md`'s Active Blockers from the
corrected table; no code, ticket, spec, `ROADMAP.md`, `README.md`, or
`ARTIFACTS.md` was edited by this review pass itself.

## Interview Required

**No.** Every finding above is a factual correction resolvable from the live
`.workflow-tools` ticket store already read during this review (a missing
table row, an off-by-one prose count, and an incomplete-but-not-wrong summary
sentence) — none of them is a judgment call the requester must decide between
alternatives for. The next pass can apply findings 1-3 to `ARTIFACTS.md` and
then expand `ROADMAP.md`'s Active Blockers section from the corrected table,
without further requester input.
