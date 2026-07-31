# Generated-file markers

Generated markdown produced by `rule-api`, `spec-api`, `ticket-api`, or any other domain that uses the shared snippet-rendering pipeline carries two kinds of provenance comments. Tooling, humans, and other generators all rely on those comments to detect regenerated files and to map prose back to its canonical source.

## File marker

The first non-empty line of a generated file is `<!-- <domain>:file generated=true -->`, where `<domain>` is the owning API (`rule-api`, `spec-api`, …). The marker signals that the file is owned by the generation pipeline and must not be hand-edited.

## Entry marker

Each composed snippet is preceded by `<!-- <domain>:entry id=<uuid> slug=<path> -->`. The `id` is the canonical entry id in the source store and the `slug` is the hierarchical identifier of that entry. Entry markers let regenerators detect identical inputs, surface diffs, and let editors navigate from generated prose to its canonical source.

## Byte stability

Regeneration must be byte-stable: running the generation command twice with no source changes produces no diff. The shared builder preserves the existing newline convention on rewrite, orders snippets deterministically, and rejects duplicate snippet ids so that re-runs are idempotent.
