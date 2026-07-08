<!-- aligned-structure:v1 -->

# Summary

This spec records the decision to replace the old bash extraction flow with `crane-cli` for context-stack tool-history migrations.

## Behavior Story

This spec records the decision to replace the old bash extraction flow with `crane-cli` for context-stack tool-history migrations.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# context-stack/tool-history-transplant

This spec records the decision to replace the old bash extraction flow with `crane-cli` for context-stack tool-history migrations.

## Problem

The earlier `git filter-branch` extractor was solving the wrong migration shape for the tool move:

- it rewrote a much larger history window than necessary;
- it encouraged per-tool branch splitting even though several commits touched multiple tool paths together;
- it made real runs slow enough that failed attempts were expensive to diagnose and rerun.

The desired workflow is narrower: identify the first commit touching the selected tool paths, preserve shared commits across those paths, remap the selected trees, import them into a target repository branch, and only then merge into the target branch.

## Decisions

1. `crane-cli` is the canonical migration tool for this workflow. The old bash extractor is not the source of truth anymore.
2. The unit of history is the combined tool path set, not per-tool branches, because shared commits across `tools/cli/context-cli`, `tools/mcp/context-mcp`, `tools/http/context-http`, and `tools/context-editor` are semantically part of the same migration.
3. History must be narrowed to the first relevant commit touching the selected paths before any rewrite or import step.
4. The migration engine is `git fast-export` -> Rust path transform -> `git fast-import`, imported onto a dedicated branch in the target repository.
5. Verification must happen before any future production migration of a new path set or rewrite shape.

## Current proof

The first `crane-cli` cut has already shown the intended shape on the real context-stack tool set:

- crate validation passed through `cargo test -p crane-cli`, including a temp-repo end-to-end transplant test;
- a real dry run against the combined tool set resolved the expected anchor commit `dbfd786c3764278e3c77f02d13ccfe0f33c5ccea`;
- the first live import into `../context-stack` reported 45 imported commits and merged onto `main`.

That proof is enough to justify using `crane-cli` as the active implementation path, but it does not remove the need for a repeatable preflight protocol for future migrations and follow-up integration work.

## Non-goals in the first cut

The first `crane-cli` implementation does not automatically:

- retarget imported Cargo path dependencies for the destination repository layout;
- decide long-term ownership cleanup in the source repository after import;
- model every possible rewrite shape, including collapsing a filtered subtree directly to branch root.

Those follow-up slices are tracked separately so the migration tool, the verification workflow, and the repository-integration work stay distinguishable.
