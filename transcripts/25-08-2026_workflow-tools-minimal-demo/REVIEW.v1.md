# First Scope Review

## Verdict

Approved as scoped after the requester selected **Cargo + CLI + skill** as the required first proof.

## Mission Goal

Deliver a minimal external consumer repository that, from a clean environment, uses version-pinned workflow-tools Cargo dependencies, installed workflow-tool transport binaries, and the published workflow-skill to manage a small application with a few tickets and specifications, with the whole tutorial flow continuously verified by GitHub Actions.

## Findings

| Finding | Evidence | Decision |
| --- | --- | --- |
| The target architecture requires external Git dependency identity and development-only patches. | [182940eb workflow-tool extraction policy](../../.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md) requirements R1-R2 | The fixture must contain no local path dependency or `[patch]` override for workflow-tool domains. |
| The umbrella and skill are planned but not yet productized. | [b525a7fa workflow-tools umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/description.md), [b9a52b79 workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/description.md) | The fixture is both their integration contract and the first release gate. |
| Existing contract-reference code teaches a domain shape but does not prove consumer installation. | [workflow-tools contract reference](../../workflow-tools/contract-reference/README.md) | Keep the reference separate; create a consumer fixture with a runnable tutorial. |
| CI is absent from workflow-tools. | `workflow-tools/.github/` absent | Add a clean-environment GitHub Action owned by the fixture/install work. |
| Context-engine remains locally composed with path patches and submodules. | [context-engine Cargo composition](../../Cargo.toml), [92741a14 context-engine consumer cutover](../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/description.md) | Do not use context-engine as the first installation proof; use the smaller fixture, then apply the proven contract to context-engine. |

## Scope Decision

The first consumer proof includes only a tiny Rust application, the ticket and spec stores needed to demonstrate tool use, one public Cargo domain dependency, one installed CLI or MCP transport, and workflow-skill bootstrap. The first proof excludes viewer/browser surfaces, broad artifact migration, and pitch-scripts integration. Those surfaces remain later consumers or separate Phase C/F work.