<!-- aligned-structure:v2 -->

# Minimal External Workflow-Tools Consumer

## Target Code Location

- [workflow-tools](workflow-tools) is the owning repository boundary for the future minimal consumer fixture and tutorial.
- [workflow-tools/README.md](workflow-tools/README.md) is the future public entry point for the tutorial.
- [workflow-tools/contract-reference/README.md](workflow-tools/contract-reference/README.md) is an existing local contract reference deliberately excluded from the fixture's source inputs.

## Naming Conventions

The fixture is named `minimal-consumer` and is intended for `workflow-tools/fixtures/minimal-consumer/`. Its executable tutorial is named `run-tutorial.sh`. Criterion identifiers use `MEC-<number>`.

## Requester Input

> workflow-tools gains a minimal external consumer that proves the public installation story: bootstrap workflow-skill in a clean environment, resolve a public Cargo dependency with no local `[patch]` override, install and use a workflow transport, and operate a tiny ticket/spec-backed app.

## Reading Order

1. [182940eb Repository architecture and dependency policies](.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md) - governing dependency-source and development-patch policy.
2. [5ee7f36a Workflow-tools domain crate contract](.spec/specs/5ee7f36a-2aea-4373-8c67-e6b26ae174bf/body.md) - public domain-crate and transport ownership contract.
3. [69eb4118 Extract workflow tooling into standalone per-tool repositories and reframe context-engine as a consuming example](.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml) - extraction epic enabled by the consumer proof.
4. [b9a52b79 Author workflow-skill skills.sh package as the installable entry point](.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml) - provider of the bootstrap step.
5. [2345ba7f End-to-end validation and cutover across split repositories](.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml) - downstream validation owner that consumes the fixture proof.

## Motivation

An extracted workflow tool needs evidence that a project outside `context-engine` can install and use the public contract. The root workspace's local Cargo patches cannot provide that evidence because those patches can mask a missing public dependency or installer path.

## Dependent Expectation

If this specification is implemented, an external consumer can rely on one reproducible tutorial to bootstrap `workflow-skill`, resolve a public workflow-tools dependency, invoke one installed workflow transport, and verify ticket and spec data written by a tiny application without borrowing local workflow tooling.

## Guards

No `test-api` ValidationSpec exists yet for this new fixture. Review evidence must contain successful output from MEC-1 through MEC-4 and the tutorial's record read-back. The future GitHub Actions gate must run the same tutorial command, not a parallel substitute.

## Positions

- [workflow-tools/README.md](workflow-tools/README.md): partial - the public repository exists but has no installation tutorial.
- [workflow-tools/contract-reference/README.md](workflow-tools/contract-reference/README.md): implemented - local contract-reference material exists but is not an external-consumer fixture.
- `workflow-tools/fixtures/minimal-consumer/`: not-implemented - the dedicated patch-free application and tutorial do not yet exist.
- `workflow-tools/.github/`: not-implemented - no continuous clean-install tutorial gate exists.

## Governing-Rule Requirement

[AGENTS.md](AGENTS.md) and [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) must introduce this draft specification as a coming-soon contract until the fixture and evidence are implemented. The repository-policy prerequisite remains [182940eb Repository architecture and dependency policies](.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md).

## Responsibility

This specification owns exactly one minimal external-consumer scenario. The scenario proves the public installation path before `context-engine` becomes a consumer and provides the stable behavior later CI must execute.

## Interfaces And Dependencies

The scenario consumes the `workflow-skill` bootstrap provided by [b9a52b79 Author workflow-skill skills.sh package as the installable entry point](.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml), one public version-pinned workflow domain Cargo dependency, and one installed workflow transport. Dependency source identity and patch limits are governed by [182940eb Repository architecture and dependency policies](.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md), rather than duplicated here.

## Behavior

### Clean-Environment Scenario

A consumer begins in a fresh temporary directory or fresh checkout containing no vendored workflow-tools repository, no copied context-engine submodule, and no local Cargo `[patch]` override for any workflow-tools dependency. The consumer performs the following ordered steps:

1. Bootstrap `workflow-skill` using the documented public command.
2. Create or enter the minimal consumer project and resolve its public, version-pinned Cargo workflow-domain dependency.
3. Install the selected workflow transport into a caller-controlled location and invoke the installed transport.
4. Run the tiny application or documented transport operation to create one ticket record and one spec record in the consumer-owned stores.
5. Read both records back from the stores and compare their identifiers and expected fields with the tutorial's declared expected values.

The scenario succeeds only when every step runs without access to local workflow-tools source paths or context-engine Cargo patches.

## Boundaries And Failure Cases

A tutorial run fails when Cargo metadata identifies a local path or patch override for the workflow-tool dependency, when bootstrap or installed-transport invocation fails, or when the tiny application exits successfully without the expected ticket and spec records being readable. The contract-reference project is not an acceptable substitute because local source access could mask an installation defect.

## Provider/Consumer Contract

The minimal consumer consumes the public domain crate and transport contract from [5ee7f36a Workflow-tools domain crate contract](.spec/specs/5ee7f36a-2aea-4373-8c67-e6b26ae174bf/body.md) and source-identity requirements R1-R2 from [182940eb Repository architecture and dependency policies](.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md). The minimal consumer provides a repeatable clean-install evidence scenario to [2345ba7f End-to-end validation and cutover across split repositories](.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml).

## Examples

From an empty temporary directory, a consumer runs the documented `workflow-skill` bootstrap, executes `bash fixtures/minimal-consumer/run-tutorial.sh`, and observes that the tutorial installs the selected transport, writes a ticket and a spec to consumer-owned stores, and reads both records back by their expected identifiers. The consumer then runs `cargo metadata --format-version 1 --no-deps` and sees only external source identities for workflow-tool dependencies.

## Evidence

| Criterion | Independently checkable outcome | Evidence command or read-back |
| --- | --- | --- |
| MEC-1 external dependency source | The fixture has no local workflow-tools path dependency or `[patch]` override; Cargo reports external source identities for every workflow-tool dependency. | `cargo metadata --format-version 1 --no-deps` from the fixture. |
| MEC-2 clean tutorial | The tutorial completes in a fresh temporary directory or fresh checkout with no vendored workflow tooling and no context-engine patch configuration. | `bash fixtures/minimal-consumer/run-tutorial.sh` succeeds end to end. |
| MEC-3 fixture build | The bootstrapped minimal consumer builds using public dependencies. | `cargo build --manifest-path fixtures/minimal-consumer/Cargo.toml` succeeds. |
| MEC-4 persisted workflow records | The tutorial creates the declared ticket and spec records, then reads both records back from the consumer-owned stores; identifiers and expected fields match the tutorial's declared values. | Tutorial output plus direct transport/CLI read-back of both records. |

## Traceability

- [69eb4118 Extract workflow tooling into standalone per-tool repositories and reframe context-engine as a consuming example](.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml) is the enabling extraction epic.
- [182940eb Repository architecture and dependency policies](.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md) is the governing policy; this spec consumes R1-R2 and does not restate their implementation rules.
- [b9a52b79 Author workflow-skill skills.sh package as the installable entry point](.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml) supplies the bootstrap behavior consumed by the scenario.
- [2345ba7f End-to-end validation and cutover across split repositories](.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml) consumes the scenario as early repeatable proof.
- [92741a14 Reframe context-engine as a consuming example with workflow-tools as an installed dependency](.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml) is explicitly downstream and excluded from the first proof.

## Scope

This is a single `workflow-tools` fixture and tutorial contract, including its public bootstrap, dependency-source proof, installed transport invocation, tiny ticket/spec operation, read-back, and future command reuse by CI.

## Non-Goals

- Viewer or browser tests are not part of the minimal consumer scenario.
- Multi-domain transport coverage is not part of the minimal consumer scenario.
- Artifact-store migration does not block or belong to the minimal consumer scenario.
- Context-engine consumer cutover is downstream work and does not belong to the minimal consumer scenario.
- The fixture is not a replacement for the existing contract-reference project or a second production application.
