# 03 - Migration Pilot Roadmap

## Outcome

Prepare a bounded pilot plan that can later migrate the Presentation System specification only after the contract-ownership decision is settled.

## Priority 1 - Define the Target Artifact Contract (drafted)

Specify the component, criterion, evidence-reference, and directed-contract-edge records, including stable identifiers, allowed roles, cycle behavior, and the ownership decision from package 02.

Drafted in `05-target-artifact-contract.md`, sourced from `transcripts/20-08-2026_specification-architecture-guidelines/merged.clean.md`. The record shapes and cycle behavior are proposed; the contract-ownership question carries a recommended default that still requires requester confirmation before Priority 2 starts.

**Non-goal:** do not commit to a TOML, Markdown, or section-file encoding before the semantics are agreed.

**Validation:** examples covering a one-way relationship and a consumer/provider cycle must be unambiguous and map every referenced criterion to its provider obligation.

## Priority 2 - Map the Presentation System to Components

> Partition the existing Presentation System requirements into a proposed component map. For each component, identify its responsibility, related components, external references, and candidate acceptance criteria.

**Non-goal:** do not edit the current Presentation System spec, its implementation, tickets, or tests.

**Validation:** every active requirement and acceptance criterion from the current body has exactly one proposed owning component or an explicit cross-component contract; deferred material remains marked as deferred.

## Priority 3 - Bind Criteria to Evidence

For each candidate acceptance criterion, identify the required validation specification or test execution and the expected evidence artifact. Existing structured `acceptance_criteria`, `evidence_requirements`, and `fulfillment_summaries` fields are candidates for reuse, but their suitability for component-scoped contracts must be demonstrated.

**Non-goal:** do not create test-api records or execute browser tests during planning.

**Validation:** each proposed criterion is measurable and names at least one evidence source; each contract edge references only provider-owned criteria.

## Priority 4 - Choose the Migration Slice

Decide whether to migrate the Presentation System spec manually as the first pilot, or first deliver generic migration support. Preserve the legacy spec until the migrated result passes health and traceability validation.

**Non-goal:** do not migrate the entire specification store in the pilot.

**Validation:** a dry-run migration plan identifies all input files, all proposed new artifacts, a reversible path, and these post-migration checks:

```bash
./target/debug/spec.exe get 2ccde9ee-85ac-4c87-9601-f6099f5be01c --json
./target/debug/spec.exe health --all
```

## Dependencies

Priority 1 must complete before Priorities 2-4. Priorities 2 and 3 can proceed together once the model is settled. Priority 4 depends on both.