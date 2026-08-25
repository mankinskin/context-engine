# Artifact Inventory

| Artifact | Kind and state | Relevance |
| --- | --- | --- |
| [input.clean.md](input.clean.md) | Cleaned requester transcript | States the desired target: a small, green external consumer with a demo, tutorial, and continuous validation. |
| [69eb4118 workflow-tool extraction epic](../../.ticket/tickets/69eb4118-19ec-4b5b-bb12-30e314029cc5/ticket.toml) | Epic, open | Governs the A-F extraction sequence and target split between context-engine and reusable workflow tooling. |
| [182940eb workflow-tool extraction policy](../../.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/spec.toml) | Spec, partial-with-gaps | Requires canonical Git dependencies and limits local Cargo patches to development composition. |
| [b525a7fa workflow-tools umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml) | Phase C task, open | Owns the aggregate repository and its install/build entry point. |
| [47a0bcc3 artifact stores](../../.ticket/tickets/47a0bcc3-f42d-475e-b05a-777293c4698e/ticket.toml) | Phase C task, open | Owns self-referential stores in workflow-tools and the individual tool repositories. |
| [47f2a664 artifact migration](../../.ticket/tickets/47f2a664-7803-4074-b40c-f41d3caf0c54/ticket.toml) | Phase C task, open | Owns classified, reference-preserving artifact relocation. |
| [b9a52b79 workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml) | Phase D task, open | Owns skills.sh publication and the one-command workflow bootstrap. |
| [92741a14 context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml) | Phase E task, open | Owns removal of vendored workflow tooling and context-engine's installed-dependency contract. |
| [2345ba7f end-to-end cutover validation](../../.ticket/tickets/2345ba7f-6d83-449b-bf07-d541c5f8e01e/ticket.toml) | Phase F task, open | Owns final cross-repository validation; the minimal fixture becomes its earliest repeatable proof. |
| [0b527d28 migration and dependency-install documentation](../../.ticket/tickets/0b527d28-9487-4a6c-8c7a-835b4a5d9582/ticket.toml) | Phase F task, open | Owns the general installation guide and context-engine worked example. |
| [72b641b1 consumer topology](../../.spec/specs/72b641b1-6620-4043-b956-102d826ce8ea/spec.toml) | Draft specification | Defines meta-workspace ownership, top-level consumer workspaces, and explicit consumer-root selection. |
| [389f90d9 consumer topology implementation](../../.ticket/tickets/389f90d9-fb06-49b7-948b-0dbd14dcfeca/ticket.toml) | Task, open | Implements `minimal-demo` as the first top-level consumer and blocks wrong-workspace tool operations. |
| [workflow-tools README](../../workflow-tools/README.md) | Present but empty | Confirms there is no current install tutorial or consumer quickstart. |
| [workflow-tools contract reference](../../workflow-tools/contract-reference/README.md) | Present | Demonstrates crate, transport, viewer, and VS Code shapes, but is not an external installed-consumer fixture. |
| [workflow-tools .gitmodules](../../workflow-tools/.gitmodules) | Present | Confirms the umbrella aggregates extracted tool repositories and memory-kernel as submodules. |
| [context-engine Cargo composition](../../Cargo.toml) | Present | Still patches workflow-tool domains to local paths, so it cannot prove external dependency resolution. |
| `workflow-tools/.github/` | Absent | Confirms no repository-local GitHub Action currently verifies a clean consumer setup. |

## Evidence Boundary

Research established the existing extraction plan, current code layout, and the missing product proof. Research did not create or alter tickets, specifications, source code, artifact stores, or CI configuration.