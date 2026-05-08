## Operating Notes

- The audit database is local runtime state and should not be committed.
- Coverage degrades gracefully when `cargo llvm-cov` is unavailable; it should produce a structured unavailable result rather than aborting the full audit.
- If you change the audit contract, update tests and this instruction file together.