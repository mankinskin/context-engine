# Workflow Tools Minimal Consumer Roadmap

## Outcome Summary

workflow-tools will gain a minimal external consumer that proves the public installation story before context-engine becomes the full consumer. A clean environment will bootstrap workflow-skill, resolve a public Cargo dependency without local patches, install and use a workflow transport, and operate a tiny ticket/spec-backed application. GitHub Actions will run the same tutorial continuously, making the fixture the first durable release gate for installation changes.

## Relevant Artifacts

- [69eb4118 extraction epic](../../.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml)
- [182940eb extraction policy](../../.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/spec.toml)
- [b525a7fa umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml)
- [b9a52b79 workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml)
- [92741a14 context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml)
- [2345ba7f cutover validation](../../.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml)
- [0b527d28 installation documentation](../../.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml)
- [01 install contract](01-install-contract-and-skill.md)
- [02 minimal fixture](02-minimal-consumer-fixture.md)
- [03 continuous validation](03-continuous-clean-install.md)
- [04 cutover](04-cutover-and-next-consumers.md)

## Active Blockers

None requiring a requester decision. The dedicated minimal-consumer fixture has no existing ticket; Waypoint 1 creates the required specification and implementation ticket after checking the active ticket board and existing ticket index. The local ticket/session binaries were unavailable during dossier creation, so no ticket mutation was attempted.

## Validation Gates

- `cargo metadata --format-version 1 --no-deps` from the fixture has no local workflow-tools path or `[patch]` override.
- `bash fixtures/minimal-consumer/run-tutorial.sh` bootstraps workflow-skill and the selected workflow transport in a clean temporary directory.
- `cargo build --manifest-path fixtures/minimal-consumer/Cargo.toml` succeeds after bootstrap.
- The tutorial's selected installed transport completes the documented ticket/spec operation.
- The scenario reads back expected ticket/spec store artifacts rather than treating command exit status as evidence of persisted data.
- GitHub Actions executes `bash fixtures/minimal-consumer/run-tutorial.sh` on pull requests and main; add Windows after the installer has a shell-independent path.
- `cargo metadata --format-version 1 --no-deps` in context-engine resolves workflow-tool domains from canonical external sources before Phase E completes.

## Roadmap Waypoints

1. **[Single-session] Create the minimal-consumer specification and implementation ticket.** The ticket defines one fixture, one tutorial scenario, and one GitHub Action release gate; the ticket references [182940eb extraction policy](../../.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/spec.toml), [b525a7fa umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml), and [b9a52b79 workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml). Acceptance: the ticket has an approved owning spec, an explicit clean-environment scenario, and validation commands. Depends on no previous waypoint.

2. **[Ticket-backed] Publish the minimal install contract and workflow-skill bootstrap.** Execute [01 install contract](01-install-contract-and-skill.md) through the umbrella and skill owners. Acceptance: a documented, version-pinned command sequence installs the selected tool set and resolves the fixture's public Cargo dependency without local paths. Depends on Waypoint 1.

3. **[Ticket-backed] Build the tiny external consumer and tutorial.** Execute [02 minimal fixture](02-minimal-consumer-fixture.md). Acceptance: a fresh directory can follow the tutorial to build the app and read back its few ticket/spec records. Depends on Waypoint 2.

4. **[Ticket-backed] Add clean-install continuous integration.** Execute [03 continuous validation](03-continuous-clean-install.md). Acceptance: workflow-tools GitHub Actions runs the exact tutorial scenario on Linux for pull requests and main. Depends on Waypoint 3.

5. **[Ticket-backed] Establish artifact-store ownership and migrate scoped artifacts.** Execute [47a0bcc3 artifact stores](../../.ticket/tickets/47a0bcc3-f42d-475e-b05a-777293c4698e/ticket.toml) followed by [47f2a664 artifact migration](../../.ticket/tickets/47f2a664-7803-4074-b40c-f41d3caf0c54/ticket.toml). Acceptance: workflow-tools and each domain own their stores, and migration batches preserve references. Waypoint 5 may run in parallel with Waypoints 2-4 after the umbrella repository is ready.

6. **[Ticket-backed] Reframe context-engine as the second consumer.** Execute [04 cutover](04-cutover-and-next-consumers.md) through [92741a14 context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml). Acceptance: a clean context-engine checkout uses canonical external workflow-tool sources and installed transports rather than vendored workflow-tool submodules or local domain patches. Depends on Waypoints 4 and 5.

7. **[Ticket-backed] Complete Phase F and add pitch-scripts as the next consumer.** Execute [2345ba7f cutover validation](../../.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml) and [0b527d28 installation documentation](../../.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml), then integrate pitch-scripts using the unchanged tutorial contract. Acceptance: standalone, aggregate, context-engine, and pitch-scripts checks all pass, and the public docs identify context-engine as a worked consumer example. Depends on Waypoint 6.

## Heads-Up Notes

- [workflow-tools contract reference](../../workflow-tools/contract-reference/README.md) is valuable teaching material but is not the external-consumer fixture; retaining the separation prevents local source access from masking install defects.
- The current context-engine root composition intentionally has many local `[patch]` overrides. Those overrides are development composition and cannot be used as evidence of the public installation contract.
- Artifact-store migration is important but should not delay the first minimal installation proof; target-project stores remain in the target project, while workflow-tool self-hosting stores remain with their owning workflow repositories.
- Viewer, browser, and comprehensive multi-domain transport tests belong to the broader Phase F gate, not the initial small consumer scenario.
- The current workflow-tools repository has no `.github` directory and an empty root README; Waypoints 2-4 must establish both a public entry point and a continuous proof before product consumers rely on the path.