# Problem

The current `rule-targets` model supports imports and explicit node lists, but it does not provide a reusable README schema that multiple workspaces can inherit. That forces each repo to hand-author structurally similar README targets and makes parent/child navigation rules easy to drift.

## Scope

Define and implement the shared README schema primitives needed for repository-root and first-level child README generation, including required-block validation for parent and child navigation.

## Assumptions To Prove

- `RenderTarget` definitions can gain schema or inheritance metadata without breaking existing target files.
- Inherited README nodes can be overridden or extended by workspace-specific targets.
- Missing required blocks can be surfaced during `explain-target` or `sync-targets --check` rather than only through manual review.
- Existing generated README and AGENTS targets remain byte-stable when they do not opt into the shared schema.
- Shared schema fragments must register once per canonical config file during a single config load, even when reached through both explicit imports and fragment discovery.
- Schema visibility must remain global for the active config load so sibling fragments can reference shared schemas without re-registering them.

## Test-Driven Plan

1. Add focused failing tests for schema inheritance, node override behavior, and missing-block failures.
2. Implement the smallest rule-api changes needed to make those tests pass.
3. Validate the new behavior with representative root and nested workspace targets.

## Acceptance Criteria

- The child implementation tickets in this branch are closed.
- Shared README schema support exists in `rule-api`.
- Required parent/child/installable/command-doc blocks can be validated automatically.
- Representative multi-fragment config loads do not fail from duplicate shared-schema registration.
