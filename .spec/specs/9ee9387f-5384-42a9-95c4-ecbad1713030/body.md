# Problem

Creating new domain stores repeatedly is slow and inconsistent. Teams need one prompt-driven bootstrap flow that generates a minimally functional store while preserving architecture constraints and repository conventions.

## Goals

- Encode domain-store bootstrap guidance in canonical rule entries and generated instruction/prompt surfaces.
- Provide a slash command skill that takes one prompt and scaffolds a minimal domain store.
- Add automated end-to-end prompt regression tests that verify generated outputs and architecture conformance.

## Required behavior

### Rule-managed source of truth
- Instruction and prompt artifacts must be generated from rule-store entries.
- Generated files are not hand-maintained; regeneration is part of validation.
- Rule entries explicitly encode architecture constraints from `target/tmp/architecture-decisions.md` and `architecture/cross-store-workspace-interaction`.

### Generated asset ownership
- Canonical source ownership: rule-store entries.
- Generated targets: slash-skill prompt assets, bootstrap instruction files, and template fragments.
- Manual edits to generated targets are treated as drift and must fail drift checks.

### Slash command scaffolding flow
- Single prompt input yields a minimal store scaffold including crate manifest, base models, storage API shell, CLI/MCP/HTTP stubs where applicable, and baseline tests.
- Output includes contract-crate and composition-root guidance so domain crates remain decoupled and DAG-safe.
- The flow emits actionable next steps for completing domain-specific logic.

### Prompt contract
- Required prompt fields: domain name, primary entities, required interfaces, optional external integrations.
- Prompt parser reports normalized intent and explicit warnings for ambiguous or conflicting instructions.
- Generated scaffold includes machine-readable manifest of created files and architecture checks performed.

### Architecture conformance checks
- Generated scaffold must satisfy DAG and layering constraints:
  - no domain cycles
  - contract crates remain lightweight
  - composition root stays in binaries
- The scaffolder surfaces conformance warnings when prompt intent conflicts with architecture constraints.

### Operational guardrails
- rollout supports feature-flag staged enablement (off, preview, controlled, general).
- failed drift/replay/e2e gates trigger deterministic rollback behavior and preserve diagnostic artifacts.
- operator diagnostics include clear next-step remediation guidance for failed scaffold generations.

### Regression harness
- A dedicated E2E prompt test suite validates representative prompts and asserts:
  - generated files compile
  - baseline tests run
  - architecture checks pass
  - generated instructions/prompt outputs are stable or intentionally versioned

## Validation-track decomposition

- Validation track 1: generation-drift checks ensure rule sources and generated instruction/prompt targets remain synchronized.
- Validation track 2: prompt replay matrix validates scaffold behavior across simple, medium, complex, and edge-case prompt families.
- Validation track 3: end-to-end harness aggregates compile/test/conformance checks across the replay suite and reports regressions with actionable diffs.

## Rollout gates

1. Drift-gate: source and generated artifacts remain synchronized.
2. Replay-gate: prompt matrix remains stable across representative prompt classes.
3. E2E-gate: generated scaffolds compile, tests pass, and architecture checks pass.

Downstream scaffold rollout is blocked if any gate fails.

## Major risks

- prompt ambiguity yielding unusable scaffolds without diagnostics
- drift between rule-source guidance and generated slash-command behavior
- regression suite brittleness from unstable output ordering

## Acceptance criteria

- rule-source and generated-target workflow is explicit and validated
- slash-command scaffold contract is explicit and architecture-constrained
- E2E prompt regression tickets and validation requirements are present
- rollout gating order across drift checks, prompt replay, and full E2E harness is explicit

## Traceability

- [66fae806 [scaffold] Rule-generated store bootstrap instructions and slash command skill](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/66fae806-203d-4235-9151-4272eb0bb603/ticket.toml)
- [23e81ad8 [rule+skill] Rule-store sources for domain-store scaffolding instructions](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/23e81ad8-b67c-49af-97b5-f90f8bb0ae2c/ticket.toml)
- [07d4b1b0 [skill] One-prompt domain-store scaffold slash command flow](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/07d4b1b0-bc20-4ba7-98d4-ed09365f0437/ticket.toml)
- [dedac9f5 [validation] Rule-target generation drift checks for scaffold guidance assets](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/dedac9f5-0d4d-4ad0-8a7e-4acd361c273e/ticket.toml)
- [2ff2c8e8 [validation] Prompt replay matrix for scaffold skill domain coverage](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/2ff2c8e8-eaec-4bd9-9312-ae13cd4b243a/ticket.toml)
- [70222986 [validation] E2E regression harness for domain-store scaffold prompts](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/70222986-3325-4d45-892e-31e7f4d09aa6/ticket.toml)
- [a87dcdf9 [scaffold] Rollout guardrails, feature flags, and rollback protocol](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/a87dcdf9-0638-4c84-a4ed-c8f4d3518e72/ticket.toml)

## Validation

- rule sync-targets check passes for generated instruction/prompt outputs
- focused tests for slash-command prompt parsing and scaffold generation
- drift-check suite validates source-to-generated alignment
- replay matrix suite validates behavior across prompt classes
- E2E prompt suite validates compile/test/architecture checks across prompt fixtures
