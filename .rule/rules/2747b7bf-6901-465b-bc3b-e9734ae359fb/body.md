## Entry marker

Each composed snippet is preceded by `<!-- <domain>:entry id=<uuid> slug=<path> -->`. The `id` is the canonical entry id in the source store and the `slug` is the hierarchical identifier of that entry. Entry markers let regenerators detect identical inputs, surface diffs, and let editors navigate from generated prose to its canonical source.