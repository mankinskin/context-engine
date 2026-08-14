# Policy specification for repository architecture and dependencies

Document the repository-level architecture and dependency policies that now govern workflow-tool extraction: cross-repository git dependency resolution, the `memory-kernel` neutrality boundary, the domain extension-trait pattern, per-tool domain-crate architecture, and CLI binary naming.

The repository workflow requires a specification before implementation when requirements or goals change. Tickets `9a1bffce`, B1, B2, and B7 each introduce durable policy, but no single specification presently binds those policies together. Without a shared spec, the policies can drift across instructions and extraction work can lack traceability.

Target artifact: create one new spec-stack record through `spec-api`, with a dedicated `.spec/specs/<allocated-spec-id>/body.md` specification body. The specification must link tickets `9a1bffce`, B1, B2, and B7 as implementing work and reference the existing workflow-tool extraction epic `69eb4118`.

Placement consequence: placing these cross-cutting policies directly in a single implementation instruction file would make the policy difficult to discover and would blur specification from procedure. The spec is the canonical requirements and traceability layer; instruction files remain the implementation-facing guidance derived from the spec.