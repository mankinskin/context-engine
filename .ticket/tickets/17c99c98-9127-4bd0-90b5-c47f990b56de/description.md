> Implementation slice completed.
>
> Evidence collected:
> - `cargo test -p crane-cli` passes with both the live transplant fixture and the new dry-run plan test.
> - real dry-run output against `context-engine` -> `../context-stack` now exposes the review metadata explicitly: `source_ref`, `source_commit`, `anchor_commit`, `range_spec`, `target_branch`, `import_branch`, `import_ref`, and all path mappings.
> - crate README now documents the verification flow; the spec record already carries the broader post-import inspection checklist.
