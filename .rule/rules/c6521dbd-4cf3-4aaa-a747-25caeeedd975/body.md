## Byte stability

Regeneration must be byte-stable: running the generation command twice with no source changes produces no diff. The shared builder preserves the existing newline convention on rewrite, orders snippets deterministically, and rejects duplicate snippet ids so that re-runs are idempotent.