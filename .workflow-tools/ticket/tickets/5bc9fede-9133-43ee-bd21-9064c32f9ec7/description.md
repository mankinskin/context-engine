Create a complete, reviewable distribution map for workflow-tools-related instructions currently held under `context-engine/.agents/`, with each file assigned to a destination package or retained as context-engine-specific guidance.

Source: `transcripts/30-08-2026_spec-system-improvement-planning/03-instruction-corpus-modularization.md` (Roadmap Waypoint 4).

## Dependencies
- `01-store-inventory-and-migration-matrix.md` / `migration-matrix.md` (Waypoint 1, completed).
- `context-engine/.agents/`
- Destination package ownership in `workflow-tools/`.

## Work
1. Inventory every instruction, prompt, agent, and skill in the corpus.
2. Classify files by package owner, shared dependency, and installation scope.
3. Define the modular source layout and installer contract before any relocation.
4. Update references in the distribution map so all links resolve after planned moves.
5. Implement an instruction-link validator and retain its machine-readable baseline and exception list with the distribution map.

## Non-Goals
- Do not change instruction semantics while establishing ownership.
- Do not move files or run an installation in this package.

## Acceptance Criteria
- Every inventoried guidance file has exactly one disposition: destination package, shared module, or retained context-engine guidance.
- Validator command `bash workflow-tools/tools/validate-instruction-links.sh --manifest <instruction-distribution.md> --baseline <instruction-link-baseline.txt> --exceptions <instruction-link-exceptions.txt>` is implemented (not just planned) and passes against baseline + exception list, with no unresolved legacy-path references.
- Exception file contains only intentionally retained `context-engine` guidance or time-bounded compatibility redirects with owner and expiry date.
- Validator output, UTC timestamp, and manifest revision recorded in `migration-execution.md`.
- `bash workflow-tools/install.sh --root <temporary-install-root> --dry-run` verified as documented invocation only.

## Exit Artifact
`instruction-distribution.md` with a file-by-file owner and destination map.
