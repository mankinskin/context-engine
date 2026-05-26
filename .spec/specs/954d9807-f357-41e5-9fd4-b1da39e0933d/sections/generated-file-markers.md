<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=8c492290-44fa-476d-9791-e04f4f6c4614 slug=context-engine/recurring-principles/generated-file-markers/generated-file-markers/l1 -->
# Generated-file markers

Generated markdown produced by `rule-api`, `spec-api`, `ticket-api`, or any other domain that uses the shared snippet-rendering pipeline carries two kinds of provenance comments. Tooling, humans, and other generators all rely on those comments to detect regenerated files and to map prose back to its canonical source.

<!-- spec-api:entry id=abd295a4-1f53-4bd4-adaf-eea3dd1957f3 slug=context-engine/recurring-principles/generated-file-markers/generated-file-markers/file-marker/l5 -->
## File marker

The first non-empty line of a generated file is `<!-- <domain>:file generated=true -->`, where `<domain>` is the owning API (`rule-api`, `spec-api`, …). The marker signals that the file is owned by the generation pipeline and must not be hand-edited.

<!-- spec-api:entry id=2747b7bf-6901-465b-bc3b-e9734ae359fb slug=context-engine/recurring-principles/generated-file-markers/generated-file-markers/entry-marker/l9 -->
## Entry marker

Each composed snippet is preceded by `<!-- <domain>:entry id=<uuid> slug=<path> -->`. The `id` is the canonical entry id in the source store and the `slug` is the hierarchical identifier of that entry. Entry markers let regenerators detect identical inputs, surface diffs, and let editors navigate from generated prose to its canonical source.

<!-- spec-api:entry id=c6521dbd-4cf3-4aaa-a747-25caeeedd975 slug=context-engine/recurring-principles/generated-file-markers/generated-file-markers/byte-stability/l13 -->
## Byte stability

Regeneration must be byte-stable: running the generation command twice with no source changes produces no diff. The shared builder preserves the existing newline convention on rewrite, orders snippets deterministically, and rejects duplicate snippet ids so that re-runs are idempotent.
