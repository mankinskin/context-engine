<!-- aligned-structure:v2 -->

## Motivation

Workflow-tool extraction needs one discoverable policy layer for the
repository decisions that span independent repositories: how Cargo resolves
the checked-out `memory-kernel`, what a development-only patch may replace,
where neutral shared behavior ends, and how each domain exposes transports.
Without one contract, the instruction changes tracked by
[9a1bffce Document the {domain}-api plus public {domain} crate architecture](.ticket/tickets/9a1bffce-b825-4f58-a078-2351d9bdaa16/ticket.toml),
[d2bf768f Document cross-repo git-URL dependency and patch-override policy](.ticket/tickets/d2bf768f-4011-42fa-9149-97d6adb0c322/ticket.toml),
[665a5df8 Document kernel neutrality boundary and extension-trait pattern](.ticket/tickets/665a5df8-eed9-4adb-8022-fe7f07955062/ticket.toml), and
[a74f09cf State CLI binary naming policy as an explicit rule](.ticket/tickets/a74f09cf-2c4b-4c13-9247-cd74519b6b7e/ticket.toml)
can diverge from the architecture already exercised by the repository.

This specification supplies the repository-policy prerequisite for
[69eb4118 Extract workflow tooling into standalone per-tool repositories and reframe context-engine as a consuming example](.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml).

## Dependent expectation

If this specification is implemented, dependents can rely on a workflow domain
using a stable external git source in an extracted repository while the
context-engine development workspace substitutes only the checked-out neutral
kernel and transport harness; domain behavior and transport names remain owned
by the domain's public crate.

## Guards

No `test-api` ValidationSpec currently guards this documentation policy. Review
evidence must include all of the following:

- `cargo metadata --format-version 1 --no-deps` from the repository root shows
	the git source for external `memory-kernel` and `transport-harness`
	dependencies and the root patch override.
- `cargo test -p ticket --all-features` verifies the public `ticket` crate's
	re-export and feature-gated transport binaries.
- `cargo test --manifest-path memory-kernel/Cargo.toml -p transport-harness`
	verifies the neutral shared transport-harness workspace member.
- `../../../target/debug/spec.exe validate-links --workspace . --toon`
	validates every structured ticket reference from this specification.
- Review of each implementing instruction change verifies that it cites this
	specification and preserves requirements R1 through R6 below.

At creation, the forward `TicketRef` records resolve, but `validate-links`
reports a bidirectional inconsistency until each linked ticket records this
specification in `related_specs`. A Ticket Agent must make those reciprocal
ticket updates before the traceability check can pass.

## Positions

- `Cargo.toml`: implemented - the root workspace patches only the
	`https://github.com/mankinskin/memory-kernel` source for `memory-kernel` and
	`transport-harness` to the checked-out `memory-kernel` submodule.
- `memory-kernel/Cargo.toml` and `memory-kernel/README.md`: implemented - the
	standalone neutral kernel owns generic primitives and hosts the sibling
	`transport-harness` crate.
- `memory-api/crates/ticket/Cargo.toml`: implemented - the public `ticket`
	crate defines opt-in `cli`, `mcp`, and `http` features with bare and
	transport-suffixed binaries.
- `memory-api/crates/ticket/src/lib.rs`: implemented - the public crate
	re-exports `ticket-api`.
- `memory-api/crates/ticket/tests/common/mod.rs`: implemented - a
	ticket-owned `TicketCommands` extension trait adds domain-specific helpers
	to generic test sandboxes.
- Repository instruction surfaces for these policies: partial - the four
	linked implementation tickets must make the six requirements explicit in
	durable guidance.

## Governing-rule requirement

The repository ticket/spec workflow in [AGENTS.md](AGENTS.md) and the Spec
Agent contract in [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md)
must introduce this spec according to the documented readiness rule: the
current code positions are implemented, while the consolidated guidance is
partial until the four linked instruction tickets are complete. The general
status-conditioned introduction mechanism is specified by
[51ee3a34 Rule-introduces-spec - status-conditioned spec presentation in session construction](.spec/specs/51ee3a34-110c-45ae-ba73-8a5e7d27f6bb/body.md).

## Scope

This contract covers dependency source identity, local patch limits, the
neutral-kernel boundary, the domain extension-trait ownership rule, public
domain-crate layout, and binary names. It governs durable repository guidance,
not a migration plan for each existing tool.

## Policy requirements

### R1: Cross-repository git-URL resolution

**Requirement.** A workflow domain that consumes an extracted shared crate
must declare the crate with its canonical git URL and `main` branch. The
dependency source identity must remain the same in the consuming repository and
the development workspace so Cargo can resolve the published repository
outside context-engine and the workspace can override that exact source during
development.

**Rationale and evidence.** `ticket`, `ticket-api`, `spec-api`, and sibling
domain crates declare `memory-kernel` from
`https://github.com/mankinskin/memory-kernel` on `main`; `ticket` also declares
the `transport-harness` crate from that source. This preserves an external
dependency contract rather than coupling extracted domains to a local relative
path.

### R2: Patch-override limits

**Requirement.** A context-engine root `[patch."https://github.com/mankinskin/memory-kernel"]`
section may replace only crates published by that source with the matching
checked-out submodule paths. The override remains development-workspace
composition: domain manifests retain their git declarations, and the patch
must not become a replacement mechanism for domain APIs or unrelated sources.

**Rationale and evidence.** The root `Cargo.toml` currently patches exactly
`memory-kernel` and `transport-harness` to `memory-kernel/` paths. The patch
works only because R1 preserves Cargo's source identity.

### R3: memory-kernel neutrality

**Requirement.** `memory-kernel` owns generic filesystem-backed storage,
indexing, search, workspace, board, and cross-store move primitives. The
`transport-harness` sibling may own transport-generic startup mechanics.
Neither crate may absorb a workflow domain's schema, entity semantics, command
dispatch, MCP server handler, HTTP router registration, or domain-specific
interoperability trait.

**Rationale and evidence.** `memory-kernel/README.md` describes the neutral
surface and explicitly assigns domain-specific interoperability traits to each
domain. `memory-kernel/Cargo.toml` hosts `transport-harness`, whose manifest
depends only on transport-generic libraries.

### R4: Domain extension-trait pattern

**Requirement.** Behavior that specializes a generic kernel, harness, or test
fixture for one workflow domain must be expressed by an extension trait owned
by that domain. The extension trait may be implemented for a generic receiver
type, but its method names and returned behavior remain domain-specific and do
not move into `memory-kernel` or `transport-harness`.

**Rationale and evidence.** `TicketCommands` in
`memory-api/crates/ticket/tests/common/mod.rs` is implemented for generic
`Sandbox<S>` values so both ticket test layouts share ticket-specific CLI
helpers without adding ticket concepts to generic fixtures.

### R5: Per-tool domain-crate architecture

**Requirement.** Each extracted workflow tool has an internal `{domain}-api`
crate and one public `{domain}` crate. The public crate re-exports the API and
is the sole package that owns opt-in `cli`, `mcp`, and `http` transport
features plus the corresponding binary targets. Frontends remain separate
packages; they do not become binary targets of the public domain crate.

**Rationale and evidence.** `memory-api/crates/ticket/Cargo.toml` names the
public package `ticket`, depends on `ticket-api`, and declares the three
feature-gated binaries. `memory-api/crates/ticket/src/lib.rs` re-exports
`ticket_api`. The broader extracted-repository target contract is defined by
[5ee7f36a Workflow-tools domain crate contract](.spec/specs/5ee7f36a-2aea-4373-8c67-e6b26ae174bf/body.md).

### R6: Bare CLI binary naming

**Requirement.** The CLI binary name is the bare public domain name
`{domain}`. The same public crate names the MCP and HTTP binaries
`{domain}-mcp` and `{domain}-http`; each binary declares exactly its required
feature so building the library by default does not pull in transport
dependencies.

**Rationale and evidence.** The current `ticket` manifest defines `ticket`
behind `cli`, `ticket-mcp` behind `mcp`, and `ticket-http` behind `http`, with
an empty default feature set. The layout is the concrete precedent established
by completed ticket `07a3eb2d`.

## Traceability

### Implementing tickets

- [9a1bffce Document the {domain}-api plus public {domain} crate architecture](.ticket/tickets/9a1bffce-b825-4f58-a078-2351d9bdaa16/ticket.toml) implements R5.
- [d2bf768f Document cross-repo git-URL dependency and patch-override policy](.ticket/tickets/d2bf768f-4011-42fa-9149-97d6adb0c322/ticket.toml) implements R1 and R2.
- [665a5df8 Document kernel neutrality boundary and extension-trait pattern](.ticket/tickets/665a5df8-eed9-4adb-8022-fe7f07955062/ticket.toml) implements R3 and R4.
- [a74f09cf State CLI binary naming policy as an explicit rule](.ticket/tickets/a74f09cf-2c4b-4c13-9247-cd74519b6b7e/ticket.toml) implements R6.

### Related contracts and exclusions

- [69eb4118 Extract workflow tooling into standalone per-tool repositories and reframe context-engine as a consuming example](.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml) is the parent extraction epic this policy enables.
- [66538d9e Memory kernel standalone extraction](.spec/specs/66538d9e-c8ff-4dd8-b3df-a12dc9984a0e/body.md) defines the standalone-kernel extraction contract that R3 constrains.
- [967c3cf6 Require architectural decisions to be settled before implementation planning](.ticket/tickets/967c3cf6-e73c-4701-9bf0-f51d30914d70/ticket.toml) is intentionally out of scope because the ticket governs planning process rather than any dependency or repository-architecture policy in R1 through R6.

## Acceptance criteria

1. The spec defines R1 through R6, each with a rationale, concrete requirement,
	 and repository evidence. A reviewer can inspect the six headings and the
	 cited manifests, README, and test helper to verify coverage.
2. The spec has structured ticket references for the four implementing tickets
	 and the resolved `69eb4118` epic, all with `workspace = "default"` and
	`store_root = ".ticket"`. `spec validate-links` resolves the forward
	references; the linked tickets must record the reciprocal `related_specs`
	entries before that check is green.
3. The scope, positions, guards, and traceability sections identify how
	 downstream guidance can preserve and verify the policy without recasting
	 the specification as an implementation plan.

## Non-goals

- This spec does not migrate any additional workflow tool or publish a crate.
- This spec does not change Cargo's dependency resolver or alter existing
	remote source URLs.
- This spec does not make `967c3cf6`'s planning-process requirement an
	architecture policy.
- This spec does not require a domain to expose every transport; it standardizes
	naming and feature ownership when a transport is exposed.
