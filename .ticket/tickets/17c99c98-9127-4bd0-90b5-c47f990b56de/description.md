# Implemented edge-case coverage

The verification slice now covers the real context-stack migration shape more directly.

Completed fixtures and checks:

- same-path transplant of `tools/cli/context-cli`, `tools/mcp/context-mcp`, `tools/http/context-http`, and `tools/context-editor`
- sibling leakage checks under `tools/**`
- same-prefix collision checks such as `context-http` vs `context-http-extra`
- delete propagation inside selected paths
- proof that `--dry-run` leaves the target repository clean and does not create the import ref
- real dry-run review against the current `context-engine` -> `../context-stack` mapping set with explicit operator metadata in the CLI output

# Next fixture candidates

- branch-root rewrite fixtures once that capability exists
- rename-heavy synthetic or end-to-end fixtures if a future migration depends on explicit rename/copy semantics
- target precondition fixtures for dirty targets or pre-existing import refs
- overlap fixtures where the destination repository already contains files under the mapped path before merge
