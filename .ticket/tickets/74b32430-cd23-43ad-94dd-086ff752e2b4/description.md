# Problem

The repository workflow now expects specs to link tickets, docs, and validation results, but cross-store linking support is still partial.

That leaves important traceability relationships to manual references, which is brittle and makes review summaries harder to trust.

# Scope

Strengthen cross-store linking support for workflow traceability.

The work should support authoritative relationships among at least these artifact types:

- tickets
- specs
- documentation or generated guidance surfaces
- validation results or artifacts

The design should preserve backward compatibility where practical and avoid breaking existing store behavior.

# Acceptance criteria

- There is a supported way to record and retrieve cross-store links among workflow artifacts.
- Tooling surfaces can return authoritative linked targets for review summaries and workflow traceability.
- The improved link support reduces the need for manual path-only references in ticket/spec summaries.
- Backward compatibility or migration expectations are documented.
