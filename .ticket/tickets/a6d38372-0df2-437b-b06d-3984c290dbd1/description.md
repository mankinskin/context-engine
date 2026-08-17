## Problem

`.agents/instructions/engine/workflow-tool-extraction.instructions.md` requires that before any `{domain}` crate is extracted into its own repository, the repository-level dependency graph across source and consuming repos is mapped, and any cross-repo cycle gets an explicit remediation decision before extraction proceeds. That instruction file calls out a specific unresolved cycle: `test-cli -> log-api -> test-api`.

## Confirmed dependency edges (verified 2026-08-17)

- [memory-api/tools/cli/test-cli/Cargo.toml](memory-api/tools/cli/test-cli/Cargo.toml): `test-cli` depends on both `test-api = { path = "../../../crates/test-api" }` and `log-api = { path = "../../../crates/log-api" }`.
- [memory-api/crates/log-api/Cargo.toml](memory-api/crates/log-api/Cargo.toml): `log-api` depends on `test-api = { path = "../test-api" }`.
- [memory-api/crates/test-api/Cargo.toml](memory-api/crates/test-api/Cargo.toml): `test-api` has no dependency on `log-api` or `test-cli` — it is a leaf in this subgraph today.

At the Cargo crate-graph level there is **no cycle** (test-api is a leaf). The cycle exists only at the **repository level** once the planned per-domain extraction happens:

- The `log` domain repo would contain `log-api` (+ `log`, `log-mcp`, etc.), and `log-api` depends on `test-api`, so `log-repo -> test-repo`.
- The `test` domain repo would contain `test-api` and `test-cli` (+ `test`, etc.), and `test-cli` depends on `log-api`, so `test-repo -> log-repo`.

Two separately-extracted repositories would depend on each other in both directions — a cross-repo cycle that Cargo path/git dependencies cannot express once the crates live in different remote repositories, and that blocks ticket [2736c3dc [workflow-tools][per-tool] Extract log tool as a single `log` domain crate (api + transport bins) + viewer frontend](.ticket/tickets/2736c3dc-ac19-4095-8a4a-e0a61340c58b/ticket.toml) from proceeding per the instruction file's gate.

## Why this blocks the log extraction

Ticket 2736c3dc plans to extract `log-api`/`log`/`log-mcp` into a standalone `log` domain repository, following the pattern defined by epic [0da6894c Single domain crate per tool](.ticket/tickets/0da6894c-dcbb-4196-8ac7-b6fae7c40ec9/ticket.toml) (done) and spec [5ee7f36a Workflow-tools domain crate contract](.spec/specs/5ee7f36a-2aea-4373-8c67-e6b26ae174bf/spec.toml). That contract assumes a domain repo can depend on lower layers without a reverse dependency looping back in. `test-cli`'s dependency on `log-api` breaks that assumption for the `test`/`log` repo pair specifically, so extracting `log` first — without resolving this cycle — would either strand `test-cli` on an unpublished/inaccessible in-tree `log-api`, or force the new `log` repo to depend back on the not-yet-extracted `test` repo, or vice versa, whichever extraction happens first.

**Caveat on remediation confidence**: a separate verification pass just found the FIRST domain extraction (ticket ba4aaa9c, `ticket` domain) is itself incomplete — legacy in-tree `memory-api/crates/ticket-api` and `memory-api/crates/ticket` are still active root workspace members, 8 in-tree consumers still depend on the in-tree relative path rather than the extracted repo, and the in-tree copy has diverged (~26 files) from the submodule copy. The "extract cleanly and cut over consumers" pattern is therefore not yet proven end-to-end. Acceptance criteria below deliberately do not assume an easy repo split is fully validated — remediation must be demonstrated with real builds/tests, not asserted by analogy to ticket ba4aaa9c.

## Candidate remediation strategies (pick one, document the choice)

1. **Extract the shared capability into a smaller shared crate.** Identify exactly what `test-cli` needs from `log-api` (likely: writing/reading validation log entries during a `test` command's log-capture flow) and move that narrow capability into a new crate that both `test-api` and `log-api` can depend on without either depending on the other's domain crate. `test-cli` would then depend on the shared crate directly instead of on `log-api`.
2. **Thin trait boundary.** Define a trait (in `test-api` or a new tiny crate) that captures the log-write/log-read operations `test-cli` needs, have `log-api` implement it, and have `test-cli` depend only on the trait definition (in `test-api` or the tiny crate) plus a runtime-wired implementation, avoiding a compile-time path dependency from `test-cli` onto `log-api`.
3. **Remove the `log-api -> test-api` dependency instead.** Determine whether `log-api`'s dependency on `test-api` is structurally necessary (e.g. shared identity/ID types, or an optional cross-link feature) and, if not, extract just the shared types `log-api` needs into a crate `test-api` also depends on, removing the `log-api -> test-api` edge entirely. This breaks the cycle from the other side and may be simpler if the `test-api` dependency in `log-api` is narrow.

Do not implement remediation in this ticket — record the investigation and the chosen strategy, then apply it in a follow-up pass gated by this ticket's acceptance criteria.