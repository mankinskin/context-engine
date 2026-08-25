# Workflow Tools Minimal Consumer Roadmap

## Outcome Summary

The meta-workspace will host one top-level `workflow-tools` source checkout and independent consumer workspaces. `minimal-demo` will be the first top-level consumer: a clean environment will bootstrap workflow-skill, resolve a public Cargo dependency without local patches, install and use a workflow transport, and operate a tiny ticket/spec-backed application. GitHub Actions will run the same tutorial continuously, making the consumer fixture the first durable release gate before context-engine follows as the second consumer.

## Relevant Artifacts

- [69eb4118 extraction epic](../../.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml)
- [182940eb extraction policy](../../.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/spec.toml)
- [b525a7fa umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml)
- [b9a52b79 workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml)
- [92741a14 context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml)
- [2345ba7f cutover validation](../../.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml)
- [0b527d28 installation documentation](../../.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml)
- [72b641b1 consumer topology spec](../../.spec/specs/72b641b1-6620-4043-b956-102d826ce8ea/spec.toml)
- [389f90d9 consumer topology ticket](../../.ticket/tickets/389f90d9-fb06-49b7-948b-0dbd14dcfeca/ticket.toml)
- [01 install contract](01-install-contract-and-skill.md)
- [02 minimal fixture](02-minimal-consumer-fixture.md)
- [03 continuous validation](03-continuous-clean-install.md)
- [04 cutover](04-cutover-and-next-consumers.md)

## Active Blockers

None requiring a requester decision. The requester approved `minimal-demo` as a top-level meta-workspace consumer submodule; [72b641b1 consumer topology](../../.spec/specs/72b641b1-6620-4043-b956-102d826ce8ea/spec.toml) is approved and [389f90d9 topology implementation](../../.ticket/tickets/389f90d9-fb06-49b7-948b-0dbd14dcfeca/ticket.toml) is ready for implementation.

## Validation Gates

- An unqualified workflow operation from the meta-workspace root fails without mutating a consumer store.
- An explicit `minimal-demo` consumer-root selector reads back only `minimal-demo` ticket/spec artifacts.
- `cargo metadata --format-version 1 --no-deps` from `minimal-demo` has no local workflow-tools path or `[patch]` override.
- The Docker fixture fetches a fresh `workflow-minimal-demo` checkout, runs the commit-pinned GitHub `install.sh` through one `curl | bash` entry point, and drives `install-ctl` with `ratatui-testlib` in a fresh image.
- The Docker fixture verifies the configured tools, instructions, hooks, and binaries below `<installation-home>/.workflow-tools/bin/`, while reusing prebuilt delivered binaries instead of compiling workflow-tools from source.
- `cargo build --manifest-path minimal-demo/Cargo.toml` succeeds after bootstrap.
- The tutorial's selected installed transport completes the documented ticket/spec operation.
- The scenario reads back expected ticket/spec store artifacts rather than treating command exit status as evidence of persisted data.
- GitHub Actions executes `bash fixtures/minimal-consumer/run-tutorial.sh` on pull requests and main; add Windows after the installer has a shell-independent path.
- `cargo metadata --format-version 1 --no-deps` in context-engine resolves workflow-tool domains from canonical external sources before Phase E completes.

## Roadmap Waypoints

1. **[Completed 2026-08-25] Approve the consumer contracts and topology plan.** [bc639ab3 minimal consumer spec](../../.spec/specs/bc639ab3-8eda-4268-a7a2-34289bfeba4d/spec.toml) and [72b641b1 consumer topology spec](../../.spec/specs/72b641b1-6620-4043-b956-102d826ce8ea/spec.toml) are approved. [389f90d9 consumer topology ticket](../../.ticket/tickets/389f90d9-fb06-49b7-948b-0dbd14dcfeca/ticket.toml) has clean-environment and negative-path validation criteria. Depends on no previous waypoint.

2. **[Ticket-backed] Establish the top-level consumer topology and explicit workspace selection.** Execute [389f90d9 consumer topology](../../.ticket/tickets/389f90d9-fb06-49b7-948b-0dbd14dcfeca/ticket.toml). Acceptance: `minimal-demo` is a top-level consumer, and tools/hook operations launched from the superproject reject an absent or ambiguous consumer root. Depends on Waypoint 1.

3. **[Ticket-backed] Publish the minimal install contract and workflow-skill bootstrap.** Execute [01 install contract](01-install-contract-and-skill.md) through the umbrella and skill owners. Acceptance: a documented, version-pinned command sequence installs the selected tool set and resolves the `minimal-demo` public Cargo dependency without local paths. Depends on Waypoint 2.

4. **[Ticket-backed] Build the tiny top-level external consumer and Docker tutorial fixture.** Execute [02 minimal fixture](02-minimal-consumer-fixture.md). Acceptance: a fresh Docker image fetches `workflow-minimal-demo`, completes the commit-pinned installer and TUI configuration flow with prebuilt binary reuse, builds `minimal-demo`, and reads back its few ticket/spec records. Depends on Waypoint 3.

5. **[Ticket-backed] Add clean-install continuous integration.** Execute [03 continuous validation](03-continuous-clean-install.md). Acceptance: workflow-tools GitHub Actions runs the exact Docker tutorial scenario on Linux for pull requests and main. Depends on Waypoint 4.

6. **[Ticket-backed] Establish artifact-store ownership and migrate scoped artifacts.** Execute [47a0bcc3 artifact stores](../../.ticket/tickets/47a0bcc3-f42d-475e-b05a-777293c4698e/ticket.toml) followed by [47f2a664 artifact migration](../../.ticket/tickets/47f2a664-7803-4074-b40c-f41d3caf0c54/ticket.toml). Acceptance: workflow-tools and each domain own their stores, and migration batches preserve references. Waypoint 6 may run in parallel with Waypoints 3-5 after the umbrella repository is ready.

7. **[Ticket-backed] Reframe context-engine as the second consumer.** Execute [04 cutover](04-cutover-and-next-consumers.md) through [92741a14 context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml). Acceptance: a clean context-engine checkout uses canonical external workflow-tool sources and installed transports rather than a vendored workflow-tools submodule or local domain patches. Depends on Waypoints 5 and 6.

8. **[Ticket-backed] Complete Phase F and add pitch-scripts as the next consumer.** Execute [2345ba7f cutover validation](../../.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml) and [0b527d28 installation documentation](../../.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml), then integrate pitch-scripts using the unchanged tutorial contract. Acceptance: standalone, aggregate, minimal-demo, context-engine, and pitch-scripts checks all pass. Depends on Waypoint 7.

## Heads-Up Notes

- [workflow-tools contract reference](../../workflow-tools/contract-reference/README.md) is valuable teaching material but is not the external-consumer fixture; retaining the separation prevents local source access from masking install defects.
- `minimal-demo` is a top-level consumer submodule, not `workflow-tools/fixtures/minimal-consumer`; source-tree proximity must not mask installation behavior.
- The meta-workspace root is not a consumer store. A tool operation from the root must declare which sibling consumer it targets.
- The current context-engine root composition intentionally has many local `[patch]` overrides. Those overrides are development composition and cannot be used as evidence of the public installation contract.
- Artifact-store migration is important but should not delay the first minimal installation proof; target-project stores remain in the target project, while workflow-tool self-hosting stores remain with their owning workflow repositories.
- Viewer, browser, and comprehensive multi-domain transport tests belong to the broader Phase F gate, not the initial small consumer scenario.
- The current workflow-tools repository has no `.github` directory and an empty root README; Waypoints 2-4 must establish both a public entry point and a continuous proof before product consumers rely on the path.